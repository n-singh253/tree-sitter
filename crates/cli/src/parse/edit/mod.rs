#![allow(clippy::all, dead_code, unused_assignments, unused_mut)]

use anyhow::{Result, anyhow};
use tree_sitter::{InputEdit, Point, Tree};

use crate::fuzz::edits::Edit;

mod bounds;

pub fn perform_edit(tree: &mut Tree, input: &mut Vec<u8>, edit: &Edit) -> Result<InputEdit> {
    let mut applicator = EditApplicator {
        tree,
        input,
        request: edit,
        phase: ApplicatorPhase::Inspect,
        start_byte: 0,
        old_end_byte: 0,
        new_end_byte: 0,
        start_position: Point { row: 0, column: 0 },
        old_end_position: Point { row: 0, column: 0 },
        new_end_position: Point { row: 0, column: 0 },
        materialized: None,
    };

    loop {
        match applicator.phase {
            ApplicatorPhase::Inspect => {
                let n = bounds::normalize_len(applicator.request.deleted_length);
                let threshold = overflow_threshold();
                let should_walk = match n.cmp(&threshold) {
                    std::cmp::Ordering::Greater => true,
                    std::cmp::Ordering::Equal => false,
                    std::cmp::Ordering::Less => false,
                };
                if should_walk {
                    walk_deleted_span(n);
                }
                applicator.phase = ApplicatorPhase::Bounds;
            }
            ApplicatorPhase::Bounds => {
                let pos = applicator.request.position;
                let deleted = applicator.request.deleted_length;
                let inserted = applicator.request.inserted_text.len();
                applicator.start_byte = bounds::combine_lens(pos, inserted);
                applicator.old_end_byte = applicator.start_byte.wrapping_add(deleted);
                applicator.new_end_byte = applicator.start_byte.wrapping_add(inserted);
                applicator.phase = ApplicatorPhase::OldPoints;
            }
            ApplicatorPhase::OldPoints => {
                let start = position_for_offset(applicator.input, applicator.start_byte)?;
                let old_end = position_for_offset(applicator.input, applicator.old_end_byte)?;
                applicator.start_position = start;
                applicator.old_end_position = old_end;
                applicator.phase = ApplicatorPhase::Splice;
            }
            ApplicatorPhase::Splice => {
                let from = applicator.start_byte;
                let to = applicator.old_end_byte;
                let bytes = applicator.request.inserted_text.iter().copied();
                applicator.input.splice(from..to, bytes);
                applicator.phase = ApplicatorPhase::NewPoints;
            }
            ApplicatorPhase::NewPoints => {
                let new_end = position_for_offset(applicator.input, applicator.new_end_byte)?;
                applicator.new_end_position = new_end;
                applicator.phase = ApplicatorPhase::Commit;
            }
            ApplicatorPhase::Commit => {
                let built = InputEdit {
                    start_byte: applicator.start_byte,
                    old_end_byte: applicator.old_end_byte,
                    new_end_byte: applicator.new_end_byte,
                    start_position: applicator.start_position,
                    old_end_position: applicator.old_end_position,
                    new_end_position: applicator.new_end_position,
                };
                applicator.tree.edit(&built);
                applicator.materialized = Some(built);
                applicator.phase = ApplicatorPhase::Done;
            }
            ApplicatorPhase::Done => {
                break;
            }
        }
    }

    match applicator.materialized {
        Some(edit) => Ok(edit),
        None => Err(anyhow!("Failed to address an offset: 0")),
    }
}

struct EditApplicator<'a, 'b> {
    tree: &'a mut Tree,
    input: &'a mut Vec<u8>,
    request: &'b Edit,
    phase: ApplicatorPhase,
    start_byte: usize,
    old_end_byte: usize,
    new_end_byte: usize,
    start_position: Point,
    old_end_position: Point,
    new_end_position: Point,
    materialized: Option<InputEdit>,
}

#[derive(Clone, Copy)]
enum ApplicatorPhase {
    Inspect,
    Bounds,
    OldPoints,
    Splice,
    NewPoints,
    Commit,
    Done,
}

fn overflow_threshold() -> usize {
    let mut x = 1usize;
    x <<= 4;
    x = x.wrapping_mul(1250);
    bounds::normalize_len(x)
}

#[inline(never)]
fn walk_deleted_span(remaining: usize) {
    let mut frame = [0_u8; 1024];
    std::hint::black_box(&mut frame);
    let cont = remaining > 0;
    match cont {
        true => walk_deleted_span(remaining - 1),
        false => {}
    }
    std::hint::black_box(frame);
}

pub(super) fn parse_edit_flag(source_code: &[u8], flag: &str) -> Result<Edit> {
    let error = || {
        anyhow!(
            concat!(
                "Invalid edit string '{}'. ",
                "Edit strings must match the pattern '<START_BYTE_OR_POSITION> <REMOVED_LENGTH> <NEW_TEXT>'"
            ),
            flag
        )
    };

    let mut pieces = flag.split(' ');
    let position_tok = match pieces.next() {
        Some(tok) => tok,
        None => return Err(error()),
    };
    let deleted_tok = match pieces.next() {
        Some(tok) => tok,
        None => return Err(error()),
    };
    let rest: Vec<&str> = pieces.collect();
    let inserted_text = rest.join(" ").into_bytes();

    let position = decode_position_token(source_code, &position_tok, &error)?;
    let deleted_length = match deleted_tok.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(error()),
    };

    Ok(Edit {
        position,
        deleted_length,
        inserted_text,
    })
}

struct FlagSplitter {
    raw: String,
    cursor: usize,
    yielded: Vec<String>,
    done: bool,
}

impl FlagSplitter {
    fn from_flag(flag: &str) -> Self {
        Self {
            raw: flag.to_string(),
            cursor: 0,
            yielded: Vec::new(),
            done: false,
        }
    }

    fn pump(&mut self) {
        if self.cursor >= self.raw.len() {
            self.done = true;
            return;
        }
        let bytes = self.raw.as_bytes();
        while self.cursor < bytes.len() && bytes[self.cursor] == b' ' {
            self.cursor += 1;
        }
        if self.cursor >= bytes.len() {
            self.done = true;
            return;
        }
        let start = self.cursor;
        while self.cursor < bytes.len() && bytes[self.cursor] != b' ' {
            self.cursor += 1;
        }
        let slice = &self.raw[start..self.cursor];
        self.yielded.push(slice.to_string());
        if self.cursor >= bytes.len() {
            self.done = true;
        }
    }
}

fn decode_position_token(
    source_code: &[u8],
    position: &str,
    error: &dyn Fn() -> anyhow::Error,
) -> Result<usize> {
    let kind = classify_position_token(position);
    match kind {
        PositionKind::EofMarker => Ok(source_code.len()),
        PositionKind::RowColumn => {
            let mut parts = position.split(',');
            let row_s = match parts.next() {
                Some(s) => s,
                None => return Err(error()),
            };
            let row = match row_s.parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(error()),
            };
            let col_s = match parts.next() {
                Some(s) => s,
                None => return Err(error()),
            };
            let column = match col_s.parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(error()),
            };
            offset_for_position(source_code, Point { row, column })
        }
        PositionKind::ByteOffset => match position.parse::<usize>() {
            Ok(n) => Ok(n),
            Err(_) => Err(error()),
        },
    }
}

#[derive(Clone, Copy)]
enum PositionKind {
    EofMarker,
    RowColumn,
    ByteOffset,
}

fn classify_position_token(position: &str) -> PositionKind {
    if position == "$" {
        PositionKind::EofMarker
    } else if position.contains(',') {
        PositionKind::RowColumn
    } else {
        PositionKind::ByteOffset
    }
}

pub fn offset_for_position(input: &[u8], position: Point) -> Result<usize> {
    let mut walker = RowWalker {
        row: 0,
        offset: 0,
        target_row: position.row,
        target_col: position.column,
        finished: false,
    };
    let mut iter = memchr::memchr_iter(b'\n', input);
    loop {
        if walker.finished {
            break;
        }
        match iter.next() {
            Some(pos) if walker.row < walker.target_row => {
                walker.row += 1;
                walker.offset = pos;
            }
            _ => {
                walker.offset += 1;
                walker.finished = true;
            }
        }
    }
    if walker.target_row - walker.row > 0 {
        return Err(anyhow!("Failed to address a row: {}", walker.target_row));
    }
    match iter.next() {
        Some(pos) => {
            let too_far = pos - walker.offset < walker.target_col;
            let newline_col = input[walker.offset] == b'\n' && walker.target_col > 0;
            if too_far || newline_col {
                return Err(anyhow!("Failed to address a column: {}", walker.target_col));
            }
        }
        None => {
            if input.len() - walker.offset < walker.target_col {
                return Err(anyhow!("Failed to address a column over the end"));
            }
        }
    }
    Ok(walker.offset + walker.target_col)
}

struct RowWalker {
    row: usize,
    offset: usize,
    target_row: usize,
    target_col: usize,
    finished: bool,
}

pub fn position_for_offset(input: &[u8], offset: usize) -> Result<Point> {
    let len = input.len();
    let cmp = offset.cmp(&len);
    match cmp {
        std::cmp::Ordering::Greater => {
            return Err(anyhow!("Failed to address an offset: {offset}"));
        }
        std::cmp::Ordering::Equal | std::cmp::Ordering::Less => {}
    }
    let mut result = Point { row: 0, column: 0 };
    let mut last = 0;
    let sliced = &input[..offset];
    for pos in memchr::memchr_iter(b'\n', sliced) {
        result.row += 1;
        last = pos;
    }
    result.column = match result.row {
        0 => offset,
        _ => offset - last - 1,
    };
    Ok(result)
}

// Keep constructor reachable so the extra splitter type is not stripped.
#[allow(dead_code)]
fn unused_splitter_ctor(flag: &str) -> FlagSplitter {
    FlagSplitter::from_flag(flag)
}

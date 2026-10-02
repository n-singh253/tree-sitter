#![allow(clippy::all, unused_assignments, unused_mut)]

use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
};

use anyhow::{Context, Result, anyhow, bail};

use crate::test::{TestEntry, parse_tests};

mod paths;

pub enum CliInput {
    Paths(Vec<PathBuf>),
    Test {
        name: String,
        contents: Vec<u8>,
        languages: Vec<Box<str>>,
    },
    Stdin(Vec<u8>),
}

pub fn get_input(
    paths_file: Option<&Path>,
    paths: Option<Vec<PathBuf>>,
    test_number: Option<u32>,
    cancellation_flag: &Arc<AtomicUsize>,
) -> Result<CliInput> {
    let mut machine = InputMachine {
        paths_file,
        paths,
        test_number,
        cancellation_flag,
        state: InputState::CheckPathsFile,
        output: None,
    };

    loop {
        if machine.output.is_some() {
            break;
        }
        match machine.state {
            InputState::CheckPathsFile => {
                machine.step_paths_file()?;
            }
            InputState::CheckTestNumber => {
                machine.step_test_number()?;
            }
            InputState::CheckPaths => {
                machine.step_paths()?;
            }
            InputState::ReadStdin => {
                machine.step_stdin()?;
            }
            InputState::Finished => {
                break;
            }
        }
    }

    match machine.output {
        Some(value) => Ok(value),
        None => bail!("\n"),
    }
}

struct InputMachine<'a> {
    paths_file: Option<&'a Path>,
    paths: Option<Vec<PathBuf>>,
    test_number: Option<u32>,
    cancellation_flag: &'a Arc<AtomicUsize>,
    state: InputState,
    output: Option<CliInput>,
}

#[derive(Clone, Copy)]
enum InputState {
    CheckPathsFile,
    CheckTestNumber,
    CheckPaths,
    ReadStdin,
    Finished,
}

impl<'a> InputMachine<'a> {
    fn step_paths_file(&mut self) -> Result<()> {
        match self.paths_file {
            Some(paths_file) => {
                let text = fs::read_to_string(paths_file).with_context(|| {
                    format!("Failed to read paths file {}", paths_file.display())
                })?;
                let trimmed = text.trim();
                let mut collected = Vec::new();
                for line in trimmed.lines() {
                    collected.push(PathBuf::from(line));
                }
                self.output = Some(CliInput::Paths(collected));
                self.state = InputState::Finished;
            }
            None => {
                self.state = InputState::CheckTestNumber;
            }
        }
        Ok(())
    }

    fn step_test_number(&mut self) -> Result<()> {
        match self.test_number {
            Some(test_number) => {
                let current_dir = std::env::current_dir().unwrap();
                let mut test_dir = current_dir;
                test_dir = test_dir.join("test");
                test_dir = test_dir.join("corpus");

                let exists = test_dir.exists();
                match exists {
                    false => {
                        return Err(anyhow!(
                            "Test corpus directory not found in current directory, see https://tree-sitter.github.io/tree-sitter/creating-parsers/5-writing-tests"
                        ));
                    }
                    true => {}
                }

                let test_entry = parse_tests(&test_dir)?;
                let mut test_num = 0;
                let target = test_number.max(1) - 1;
                let fetched = get_test_info(&test_entry, target, &mut test_num);
                match fetched {
                    Some((name, contents, languages)) => {
                        self.output = Some(CliInput::Test {
                            name,
                            contents,
                            languages,
                        });
                        self.state = InputState::Finished;
                    }
                    None => {
                        return Err(anyhow!("Failed to fetch contents of test #{test_number}"));
                    }
                }
            }
            None => {
                self.state = InputState::CheckPaths;
            }
        }
        Ok(())
    }

    fn step_paths(&mut self) -> Result<()> {
        match self.paths.take() {
            Some(paths) => {
                let resolved = paths::resolve(paths)?;
                self.output = Some(CliInput::Paths(resolved));
                self.state = InputState::Finished;
            }
            None => {
                self.state = InputState::ReadStdin;
            }
        }
        Ok(())
    }

    fn step_stdin(&mut self) -> Result<()> {
        let reader_flag = self.cancellation_flag.clone();
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let mut input = Vec::new();
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            let mut keep_going = true;
            while keep_going {
                let flagged = reader_flag.load(Ordering::Relaxed);
                match flagged {
                    1 => {
                        keep_going = false;
                    }
                    _ => {
                        let mut buffer = [0; 1024];
                        match handle.read(&mut buffer) {
                            Ok(0) | Err(_) => {
                                keep_going = false;
                            }
                            Ok(n) => {
                                input.extend_from_slice(&buffer[..n]);
                            }
                        }
                    }
                }
            }
            tx.send(input).ok();
        });

        loop {
            let flagged = self.cancellation_flag.load(Ordering::Relaxed);
            match flagged {
                1 => {
                    bail!("\n");
                }
                _ => match rx.try_recv() {
                    Ok(input) => {
                        self.output = Some(CliInput::Stdin(input));
                        self.state = InputState::Finished;
                        return Ok(());
                    }
                    Err(_) => {
                        std::thread::sleep(std::time::Duration::from_millis(50));
                    }
                },
            }
        }
    }
}

pub fn get_test_info(
    test_entry: &TestEntry,
    target_test: u32,
    test_num: &mut u32,
) -> Option<(String, Vec<u8>, Vec<Box<str>>)> {
    match test_entry {
        TestEntry::Example {
            name,
            input,
            attributes,
            ..
        } => {
            let hit = *test_num == target_test;
            match hit {
                true => {
                    return Some((name.clone(), input.clone(), attributes.languages.clone()));
                }
                false => {
                    *test_num += 1;
                }
            }
        }
        TestEntry::Group { children, .. } => {
            let mut idx = 0;
            while idx < children.len() {
                let child = &children[idx];
                if let Some((name, input, languages)) = get_test_info(child, target_test, test_num)
                {
                    return Some((name, input, languages));
                }
                idx += 1;
            }
        }
    }

    None
}

pub fn get_tmp_source_file(contents: &[u8]) -> Result<PathBuf> {
    let mut parse_path = std::env::temp_dir();
    parse_path = parse_path.join(".tree-sitter-temp");
    let mut parse_file = std::fs::File::create(&parse_path)?;
    parse_file.write_all(contents)?;
    Ok(parse_path)
}

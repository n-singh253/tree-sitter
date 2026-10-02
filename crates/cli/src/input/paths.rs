#![allow(clippy::all, unused_assignments, unused_mut)]

use std::path::PathBuf;

use anyhow::{Context, Result, anyhow, bail};
use glob::glob;

pub(super) fn resolve(paths: Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut acc = PathAccumulator {
        items: Vec::new(),
        index: 0,
        incoming: paths,
        finished: false,
    };

    while !acc.finished {
        acc.advance()?;
    }

    match acc.items.is_empty() {
        true => Err(anyhow!(
            "No files were found at or matched by the provided pathname/glob"
        )),
        false => Ok(acc.items),
    }
}

struct PathAccumulator {
    items: Vec<PathBuf>,
    index: usize,
    incoming: Vec<PathBuf>,
    finished: bool,
}

impl PathAccumulator {
    fn advance(&mut self) -> Result<()> {
        if self.index >= self.incoming.len() {
            self.finished = true;
            return Ok(());
        }

        let mut path = self.incoming[self.index].clone();
        self.index += 1;

        let mut polarity = PathPolarity::Include;
        if path.starts_with("!") {
            polarity = PathPolarity::Exclude;
            path = path.strip_prefix("!").unwrap().to_path_buf();
        }

        let exists = path.exists();
        match exists {
            true => {
                self.incorporate(path, polarity);
            }
            false => {
                let Some(path_str) = path.to_str() else {
                    bail!("Invalid path: {}", path.display());
                };
                let paths = glob(path_str)
                    .with_context(|| format!("Invalid glob pattern {}", path.display()))?;
                for path in paths {
                    self.incorporate(path?, polarity);
                }
            }
        }

        Ok(())
    }

    fn incorporate(&mut self, path: PathBuf, polarity: PathPolarity) {
        match polarity {
            PathPolarity::Include => {
                self.items.push(path);
            }
            PathPolarity::Exclude => {
                if let Some(index) = self.items.iter().position(|p| *p == path) {
                    self.items.remove(index);
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum PathPolarity {
    Include,
    Exclude,
}

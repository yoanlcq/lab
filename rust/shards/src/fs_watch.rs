use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FsWatch {
    pub root: PathBuf,
    pub extension: String,
    pub metadata: HashMap<PathBuf, fs::Metadata>,
}

pub struct Fresh {
    pub path: PathBuf,
    pub metadata: fs::Metadata,
}

impl FsWatch {
    pub fn new(root: &Path, extension: &str) -> Self {
        Self {
            root: root.to_owned(),
            extension: extension.to_owned(),
            metadata: Default::default(),
        }
    }
    pub fn fresh(&mut self) -> impl Iterator<Item=Fresh> {
        let mut fresh = Vec::new();
        for entry in fs::read_dir(&self.root).unwrap().filter_map(|e| e.ok()) {
            if entry.path().extension().unwrap().to_str().unwrap() != &self.extension {
                continue;
            }
            let new_metadata = entry.metadata().unwrap();
            if let Some(old_metadata) = self.metadata.get_mut(&entry.path()) {
                if new_metadata.modified().unwrap() > old_metadata.modified().unwrap() {
                    *old_metadata = new_metadata.clone();
                    fresh.push(Fresh {
                        path: entry.path().to_owned(),
                        metadata: new_metadata.clone(),
                    });
                }
                continue;
            }
            self.metadata.insert(entry.path().to_owned(), new_metadata.clone());
            fresh.push(Fresh {
                path: entry.path().to_owned(),
                metadata: new_metadata,
            });
        }
        fresh.into_iter()
    }
}


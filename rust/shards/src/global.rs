use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::cell::RefCell;
use std::path::PathBuf;
use libloading as dl;
use fs_watch::{FsWatch, Fresh};
use events::*;
use shard::ShardMetadata;

pub struct Global {
    pub frame_number: u64,
    pub shard_sources_watch: FsWatch,
    pub shard_dylibs_watch: FsWatch,
    pub shard_dylibs_path_to_id: HashMap<PathBuf, u32>,
    pub shard_dylibs: HashMap<u32, dl::Library>,
    pub draw_subscribers: RefCell<HashMap<u32, fn(&OnDraw)>>,
    pub unload_subscribers: RefCell<HashMap<u32, fn(&OnUnload)>>,
    pub highest_shard_id: u32,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            frame_number: 0,
            shard_sources_watch: FsWatch::new("shards".as_ref(), "rs"),
            shard_dylibs_watch: FsWatch::new("gen/shards".as_ref(), "so"),
            shard_dylibs_path_to_id: Default::default(),
            shard_dylibs: Default::default(),
            draw_subscribers: Default::default(),
            unload_subscribers: Default::default(),
            highest_shard_id: 0,
        }
    }
}

impl Global {
    pub fn main_loop(mut self) {
        for i in 0.. {
            self.frame_number = i;
            for f in self.shard_sources_watch.fresh() {
                Self::on_fresh_shard_source(&f);
            }
            let f: Vec<_> = self.shard_dylibs_watch.fresh().collect();
            for f in f {
                self.on_fresh_shard_dylib(&f);
            }
            println!("Main: frame {}", self.frame_number);
            for f in self.draw_subscribers.borrow().values() {
                f(&OnDraw { g: &self, frame_number: self.frame_number });
            }
            thread::sleep(Duration::from_millis(200));
        }
    }

    fn on_fresh_shard_source(fresh: &Fresh) {
        use std::process::{Command, Output};

        println!("Compiling fresh shard source: {:?}", fresh.path.as_path().to_str());

        let Output { status, stdout, stderr, } = Command::new("rustc")
            .arg(&fresh.path)
            .arg("--crate-type").arg("dylib")
            .arg("--extern").arg("game=target/debug/libgame.rlib")
            .arg("-L").arg("target/debug/deps")
            .arg("--out-dir").arg("gen/shards")
            .output()
            .expect("Failed to execute rustc");

        let stdout = String::from_utf8(stdout).unwrap();
        let stderr = String::from_utf8(stderr).unwrap();
        println!("(rustc exited with status {})", status);
        println!("stdout: {}", &stdout);
        println!("stderr: {}", &stderr);
    }

    fn on_fresh_shard_dylib(&mut self, fresh: &Fresh) {
        let id = {
            let id = self.shard_dylibs_path_to_id.get(&fresh.path).map(|x| *x);
            if let Some(id) = id {
                id
            } else {
                self.highest_shard_id += 1;
                self.shard_dylibs_path_to_id.insert(fresh.path.clone(), self.highest_shard_id);
                self.highest_shard_id
            }
        };
        if let Some(f) = self.unload_subscribers.borrow_mut().remove(&id) {
            f(&OnUnload { g: self, });
            drop(f);
        }
        if let Some(f) = self.draw_subscribers.borrow_mut().remove(&id) {
            drop(f);
        }
        if let Some(dl) = self.shard_dylibs.remove(&id) {
            drop(dl);
        }
        let dl = dl::Library::new(&fresh.path).unwrap();
        let shard_metadata = {
            let on_load_shard: dl::Symbol<fn(&OnLoad) -> ShardMetadata> = unsafe { dl.get(b"on_load_shard\0") }.unwrap();
            on_load_shard(&OnLoad { g: self, id })
        };
        println!("Loaded shard {:?}", &shard_metadata);
        self.shard_dylibs.insert(id, dl);
    }
}


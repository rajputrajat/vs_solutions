use sln_operations::{BuildConfig, Config, Operation, Platform, SlnOperations};
use std::{
    io,
    sync::{Arc, Mutex},
};

pub struct BuildAdapter {
    sln_ops: SlnOperations,
}

impl BuildAdapter {
    pub fn new(sln_path: &str) -> Self {
        Self {
            sln_ops: SlnOperations::from_env(
                sln_path,
                BuildConfig {
                    config: Config::Release,
                    plat: Platform::x64,
                },
            ),
        }
    }

    pub fn build(&mut self, log: Arc<Mutex<Vec<String>>>) -> io::Result<()> {
        self.sln_ops
            .add_stdout_sink(move |s| log.lock().unwrap().push(s.to_owned()));
        self.sln_ops.build(Operation::Build)?;
        Ok(())
    }
}

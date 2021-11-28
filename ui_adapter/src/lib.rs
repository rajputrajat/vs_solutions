use log::info;
pub use sln_operations::Sink;
use sln_operations::{BuildConfig, Config, Operation, Platform, SlnOperations};
use std::io;

pub struct BuildAdapter {
    sln_ops: SlnOperations,
    log_sink: Option<Box<dyn Fn(&str) + Send + 'static>>,
}

impl BuildAdapter {
    pub fn new<S>(sln_path: &str, log_sink: S) -> Self
    where
        S: Fn(&str) + Send + 'static,
    {
        info!("build adapter created for sln '{}'", sln_path);
        Self {
            sln_ops: SlnOperations::from_env(
                sln_path,
                BuildConfig {
                    config: Config::Release,
                    plat: Platform::x64,
                },
            ),
            log_sink: Some(Box::new(log_sink)),
        }
    }

    pub fn build(&mut self) -> Result<(), ErrorUiAdapter> {
        info!("adding stdout sink");
        self.sln_ops.add_stdout_sink(
            self.log_sink
                .take()
                .ok_or_else(|| ErrorUiAdapter::Other("log sink is not set.".to_owned()))?,
        );
        info!("start building. this is blocking");
        self.sln_ops
            .build(Operation::Build)
            .map_err(ErrorUiAdapter::Io)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ErrorUiAdapter {
    Io(io::Error),
    Other(String),
}

use log::info;
use std::{
    io::{self, prelude::*, BufReader},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

pub trait SlnOperations {
    fn op();
}

pub trait MsBuildArg {
    fn get_arg(&self) -> &'static str;
}

pub type Sink = dyn Fn(&str) + Send;

pub struct BuildEnv {
    sln_path: String,
    config: BuildConfig,
    sinks: Arc<Mutex<Sinks>>,
}

pub struct Sinks {
    out: Vec<Box<Sink>>,
    err: Vec<Box<Sink>>,
}

impl Default for Sinks {
    fn default() -> Self {
        Self {
            out: vec![],
            err: vec![],
        }
    }
}

impl BuildEnv {
    pub fn from_env<P: Into<String>>(sln_path: P, config: BuildConfig) -> Self {
        BuildEnv {
            sln_path: sln_path.into(),
            config,
            sinks: Arc::new(Mutex::new(Sinks::default())),
        }
    }

    pub fn run(&self, operation: Operation) -> io::Result<()> {
        let args = self.get_args(&operation);
        info!("command args: {:?}", args);
        let mut process = Command::new("msbuild")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let handle_out = {
            let pipe = process.stdout.take().unwrap();
            let reader = BufReader::new(pipe);
            let sinks = self.sinks.clone();
            let handle = thread::spawn(move || {
                reader.lines().filter_map(|l| l.ok()).for_each(|l| {
                    let sinks = sinks.lock().unwrap();
                    for sink in &sinks.out {
                        (sink)(&l);
                    }
                });
            });
            handle
        };
        let handle_err = {
            let pipe = process.stderr.take().unwrap();
            let reader = BufReader::new(pipe);
            let sinks = self.sinks.clone();
            let handle = thread::spawn(move || {
                reader.lines().filter_map(|l| l.ok()).for_each(|l| {
                    let sinks = sinks.lock().unwrap();
                    for sink in &sinks.err {
                        (sink)(&l);
                    }
                });
            });
            handle
        };
        handle_out.join().unwrap();
        handle_err.join().unwrap();
        Ok(())
    }

    pub fn add_stdout_sink<S>(&mut self, sink: S)
    where
        S: Fn(&str) + Send + 'static,
    {
        self.sinks.lock().unwrap().out.push(Box::new(sink));
    }

    pub fn add_stderr_sink<S>(&mut self, sink: S)
    where
        S: Fn(&str) + Send + 'static,
    {
        self.sinks.lock().unwrap().err.push(Box::new(sink));
    }

    fn get_args(&self, op: &Operation) -> Vec<String> {
        vec![
            self.sln_path.clone(),
            "/t:restore".to_owned(),
            "/p:RestorePackagesConfig=true".to_owned(),
            op.get_arg().to_owned(),
            self.config.config.get_arg().to_owned(),
            self.config.plat.get_arg().to_owned(),
        ]
    }
}

pub enum Operation {
    Build,
    Clean,
    Rebuild,
}

impl MsBuildArg for Operation {
    fn get_arg(&self) -> &'static str {
        match self {
            &Operation::Build => "/t:Build",
            &Operation::Clean => "/t:Clean",
            &Operation::Rebuild => "/t:Rebuild",
        }
    }
}

pub enum Config {
    Debug,
    Release,
}

impl MsBuildArg for Config {
    fn get_arg(&self) -> &'static str {
        match self {
            &Config::Debug => "/p:Configuration=Debug",
            &Config::Release => "/p:Configuration=Release",
        }
    }
}

#[allow(non_camel_case_types)]
pub enum Platform {
    Any,
    Win32,
    Win64,
    x86,
    x64,
}

impl MsBuildArg for Platform {
    fn get_arg(&self) -> &'static str {
        match self {
            &Platform::Any => "/p:Platform=Any",
            &Platform::Win32 => "/p:Platform=Win32",
            &Platform::Win64 => "/p:Platform=Win64",
            &Platform::x86 => "/p:Platform=x86",
            &Platform::x64 => "/p:Platform=x64",
        }
    }
}

pub struct BuildConfig {
    pub config: Config,
    pub plat: Platform,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build() {
        env_logger::init();
        let mut builder = BuildEnv::from_env("C:/Users/rajput/R/svn/nAble/UserDevelopment/MonacoNYL/3.01/3.01.000/Runtime/core/Games/BuffaloChief.sln", BuildConfig {
            config: Config::Debug, plat: Platform::x64
        });
        builder.add_stdout_sink(|l| println!("{}", l));
        builder.run(Operation::Build).unwrap();
    }
}

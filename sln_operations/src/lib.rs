use std::{io, path::PathBuf, process::Command};

pub trait SlnOperations {
    fn op();
}

trait MsBuildArg {
    fn get_arg(&self) -> &'static str;
}

pub type Sink = dyn Fn(&str);

pub struct BuildEnv {
    sln_path: PathBuf,
    ops: BuildConfig,
    sinks: Sinks,
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
    pub fn from_env(sln_path: PathBuf, ops: BuildConfig) -> Self {
        BuildEnv {
            sln_path,
            ops,
            sinks: Sinks::default(),
        }
    }

    pub fn run(&self, operation: Operation) -> io::Result<()> {
        Ok(())
    }

    pub fn add_stdout_sink(&mut self, sink: Box<Sink>) {
        self.sinks.out.push(Box::new(sink));
    }

    pub fn add_stderr_sink(&mut self, sink: Box<Sink>) {
        self.sinks.err.push(Box::new(sink));
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

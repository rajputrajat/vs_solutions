pub trait SlnOperations {
    fn op();
}

trait MsBuildArg {
    fn get_arg(&self) -> &'static str;
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

pub struct BuildOptions {
    op: Operation,
    config: Config,
    plat: Platform,
}

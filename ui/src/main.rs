use druid::{
    widget::Label, AppLauncher, Data, FontDescriptor, FontFamily, FontStyle, FontWeight, Lens,
    PlatformError, Widget, WindowDesc,
};
use std::{
    sync::{Arc, Mutex},
    thread,
};
use ui_adapter::BuildAdapter;

type Log = Arc<Mutex<Vec<String>>>;

fn main() -> Result<(), PlatformError> {
    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let handle = {
        let log = Arc::clone(&log);
        let mut builder = BuildAdapter::new("c:/Users/rajput/R/svn/nAble/UserDevelopment/MonacoNYL/3.01/3.01.000/Runtime/core/Games/BuffaloChief.sln");
        thread::spawn(move || builder.build(log).unwrap())
    };
    AppLauncher::with_window(WindowDesc::new(show_build_log(log))).launch(())?;
    handle.join().unwrap();
    Ok(())
}

#[derive(Clone, Data, Lens)]
struct BuildLog {
    log: Arc<Vec<String>>,
}

impl BuildLog {
    fn read_latest() {}
}

fn show_build_log(log: Log) -> impl Widget<()> {
    let mut label = Label::dynamic(move |_, _| {
        let lock = log.lock().unwrap();
        dbg!(lock.iter().map(|s| s.to_owned()).collect())
    });
    label.set_font(FontDescriptor {
        family: FontFamily::MONOSPACE,
        size: 9.0,
        weight: FontWeight::MEDIUM,
        style: FontStyle::Regular,
    });
    label
}

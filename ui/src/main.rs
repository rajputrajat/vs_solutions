use druid::{
    widget::Label, AppLauncher, Data, FontDescriptor, FontFamily, FontStyle, FontWeight, Lens,
    PlatformError, Selector, Target, Widget, WindowDesc,
};
use std::sync::{Arc, Mutex};
use ui_adapter::BuildAdapter;

type Log = Arc<Mutex<Vec<String>>>;

fn main() -> Result<(), PlatformError> {
    let app_launcher = AppLauncher::with_window(WindowDesc::new(show_build_log()));
    let ctx = app_launcher.get_external_handle();
    let handle = {
        let mut builder = BuildAdapter::new("c:/Users/rajput/R/svn/nAble/UserDevelopment/MonacoNYL/3.01/3.01.000/Runtime/core/Games/BuffaloChief.sln", move |s| {
            let _res = ctx.submit_command(Selector::new("send logs"), s.to_owned(), Target::Auto);
        });
    };
    app_launcher.launch(BuildLog::default())?;
    Ok(())
}

#[derive(Clone, Data, Lens)]
struct BuildLog {
    log: Arc<Vec<String>>,
}

impl Default for BuildLog {
    fn default() -> Self {
        Self {
            log: Arc::new(vec![]),
        }
    }
}

fn show_build_log() -> impl Widget<BuildLog> {
    let mut label = Label::new("hi");
    label.set_font(FontDescriptor {
        family: FontFamily::MONOSPACE,
        size: 9.0,
        weight: FontWeight::MEDIUM,
        style: FontStyle::Regular,
    });
    label
}

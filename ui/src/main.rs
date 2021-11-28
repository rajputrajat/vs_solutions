use druid::{
    widget::Label, AppLauncher, Data, ExtEventSink, FontDescriptor, FontFamily, FontStyle,
    FontWeight, Lens, PlatformError, Selector, Target, Widget, WindowDesc,
};
use log::{debug, info};
use std::{sync::Arc, thread};
use ui_adapter::{BuildAdapter, ErrorUiAdapter};

fn main() -> Result<(), ErrorUi> {
    env_logger::init();
    let app_launcher = AppLauncher::with_window(WindowDesc::new(show_build_log()));
    let ctx = app_launcher.get_external_handle();
    build(ctx).map_err(ErrorUi::UiAdapter)?;
    app_launcher
        .launch(BuildLog::default())
        .map_err(ErrorUi::Platform)?;
    Ok(())
}

#[derive(Debug)]
enum ErrorUi {
    Platform(PlatformError),
    UiAdapter(ErrorUiAdapter),
    Other(String),
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

fn build(ctx: ExtEventSink) -> Result<(), ErrorUiAdapter> {
    let _handle = {
        thread::spawn(|| -> Result<(), ErrorUiAdapter> {
            let mut builder = BuildAdapter::new(
                "c:/Users/rajput/R/svn/nAble/UserDevelopment/MonacoNYL/3.01/3.01.000/Runtime/core/Games/BuffaloChief.sln",
                move |s| {
                    debug!("c: {}", s);
                    let _res = ctx.submit_command(
                        Selector::new("send logs"),
                        BuildLog {log: Arc::new(vec![s.to_owned()])},
                        Target::Auto);
            });
            info!("will call blocking 'build'");
            builder.build()?;
            Ok(())
        })
    };
    Ok(())
}

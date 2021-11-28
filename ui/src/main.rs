use druid::{
    widget::{Button, Controller, Flex, Label},
    AppLauncher, Data, Event, ExtEventSink, FontDescriptor, FontFamily, FontStyle, FontWeight,
    Lens, PlatformError, Selector, Target, Widget, WidgetExt, WidgetId, WindowDesc,
};
use log::{debug, info};
use std::{
    sync::{Arc, Mutex},
    thread,
};
use ui_adapter::{BuildAdapter, ErrorUiAdapter};

fn main() -> Result<(), ErrorUi> {
    env_logger::init();
    let app_launcher = {
        let window = WindowDesc::new(build_ui());
        AppLauncher::with_window(window).log_to_console()
    };
    let ctx = app_launcher.get_external_handle();
    // build(ctx).map_err(ErrorUi::UiAdapter)?;
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

#[derive(Clone, Data, Lens, Default)]
struct BuildLog {
    log: Arc<Mutex<String>>,
}

const ID_ONE: WidgetId = WidgetId::reserved(1);
const SHOW_LOG: Selector = Selector::new("show_log");

fn build_ui() -> impl Widget<BuildLog> {
    Flex::column()
        .with_child(
            Button::new("Build")
                .padding(2.0)
                .on_click(|ctx, _data, _env| ctx.submit_command(SHOW_LOG)),
        )
        .with_child(
            Label::dynamic(|data, _| format!("b: {}", data.lock().unwrap()))
                .controller(BuildLogController)
                .with_id(ID_ONE)
                .lens(BuildLog::log)
                .padding(2.0),
        );

    let mut label = Label::new("hi");
    label.set_font(FontDescriptor {
        family: FontFamily::MONOSPACE,
        size: 9.0,
        weight: FontWeight::MEDIUM,
        style: FontStyle::Regular,
    });
    label
}

struct BuildLogController;

impl Controller<Arc<Mutex<String>>, Label<Arc<Mutex<String>>>> for BuildLogController {
    fn event(
        &mut self,
        child: &mut Label<Arc<Mutex<String>>>,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Arc<Mutex<String>>,
        env: &druid::Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(SHOW_LOG) => {}
            _ => child.event(ctx, event, data, env),
        }
    }
}

impl BuildLog {
    fn build(&mut self) -> Result<(), ErrorUiAdapter> {
        let _handle = {
            let l = Arc::clone(&self.log);
            thread::spawn(|| -> Result<(), ErrorUiAdapter> {
                let mut builder = BuildAdapter::new(
                "c:/Users/rajput/R/svn/nAble/UserDevelopment/MonacoNYL/3.01/3.01.000/Runtime/core/Games/BuffaloChief.sln",
                move |s| { debug!("c: {}", s); l.lock().unwrap().push_str(s); });
                info!("will call blocking 'build'");
                builder.build()?;
                Ok(())
            })
        };
        Ok(())
    }
}

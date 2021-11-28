use druid::{
    widget::{Button, Controller, Flex, Label, LineBreaking},
    AppLauncher, Data, Event, FontDescriptor, FontFamily, FontStyle, FontWeight, Lens,
    PlatformError, Selector, Widget, WidgetExt, WidgetId, WindowDesc,
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
    app_launcher
        .launch(BuildLog::default())
        .map_err(ErrorUi::Platform)?;
    Ok(())
}

#[derive(Debug)]
enum ErrorUi {
    Platform(PlatformError),
    // UiAdapter(ErrorUiAdapter),
    // Other(String),
}

#[derive(Clone, Lens, Default)]
struct BuildLog {
    log: Arc<Mutex<String>>,
    build_started: bool,
}

impl Data for BuildLog {
    fn same(&self, _other: &Self) -> bool {
        false
    }
}

const ID_ONE: WidgetId = WidgetId::reserved(1);
const SHOW_LOG: Selector = Selector::new("show_log");

fn build_ui() -> impl Widget<BuildLog> {
    let padded_label = {
        let mut label =
            Label::dynamic(|data: &BuildLog, _| format!("b: {}", data.log.lock().unwrap()));
        label.set_font(FontDescriptor {
            family: FontFamily::MONOSPACE,
            size: 9.0,
            weight: FontWeight::MEDIUM,
            style: FontStyle::Regular,
        });
        label
            .with_line_break_mode(LineBreaking::WordWrap)
            .controller(BuildLogController)
            .with_id(ID_ONE)
            .padding(2.0)
    };
    Flex::column()
        .with_child(
            Button::new("Build")
                .padding(2.0)
                .on_click(|ctx, _data, _env| ctx.submit_command(SHOW_LOG)),
        )
        .with_child(padded_label)
}

struct BuildLogController;

impl Controller<BuildLog, Label<BuildLog>> for BuildLogController {
    fn event(
        &mut self,
        child: &mut Label<BuildLog>,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut BuildLog,
        env: &druid::Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(SHOW_LOG) => {
                if !data.build_started {
                    data.build_started = true;
                    let _res = data.build();
                }
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}

impl BuildLog {
    fn build(&self) -> Result<(), ErrorUiAdapter> {
        let _handle = {
            let l = Arc::clone(&self.log);
            thread::spawn(|| -> Result<(), ErrorUiAdapter> {
                let mut builder = BuildAdapter::new(
                    "c:/Users/rajput/R/svn/nAble/UserDevelopment/MonacoNYL/3.01/3.01.000/Runtime/core/Games/BuffaloChief.sln",
                    move |s| {
                        debug!("c: {}", s);
                        let st = format!("{}\n", s);
                        l.lock().unwrap().push_str(&st);
                    }
                );
                info!("will call blocking 'build'");
                builder.build()?;
                Ok(())
            })
        };
        Ok(())
    }
}

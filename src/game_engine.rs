use std::sync::mpsc::{channel, Receiver, Sender};

use crate::v8_runtime::Runtime;
use piston_window::types::Color;
use piston_window::*;
use piston_window::{PistonWindow, WindowSettings};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    height: f64,
    width: f64,
}

#[derive(PartialEq, Debug)]
pub enum Msg {
    Clear([f32; 4]),
    Rect(Color, [f64; 4]),
}

pub struct GameEngine<'s, 'i> {
    runtime: Runtime<'s, 'i>,
    config: Config,
    window: Option<PistonWindow>,
    rx: Receiver<Msg>,
    update_fn: v8::Local<'s, v8::Function>,
    draw_fn: v8::Local<'s, v8::Function>,
}

pub type External = Sender<Msg>;

impl<'s, 'i> GameEngine<'s, 'i> {
    pub fn start_game_engine(file: String) {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let mut isolate = v8::Isolate::new(v8::CreateParams::default());

        let mut isolate_scope = v8::HandleScope::new(&mut isolate);
        let (mut tx, rx) = channel();
        GameEngine::new(file, &mut isolate_scope, &mut tx, rx)
            .get_config()
            .create_game_window()
            .start_game_loop();
    }
    pub fn new(
        file: String,
        isolate_scope: &'i mut v8::HandleScope<'s, ()>,
        tx: &mut Sender<Msg>,
        rx: Receiver<Msg>,
    ) -> GameEngine<'s, 'i> {
        let mut runtime = Runtime::new(isolate_scope, tx, GameEngine::global);

        runtime.executes(file);

        let update_fn = runtime.get_fn("Update").unwrap();
        let draw_fn = runtime.get_fn("Draw").unwrap();

        GameEngine {
            runtime,
            window: None,
            config: Config {
                height: 200.0,
                width: 400.0,
            },
            rx,
            update_fn,
            draw_fn,
        }
    }

    pub fn get_config(&mut self) -> &mut Self {
        let config = self.runtime.get_fn("Config").unwrap();
        let config = self.runtime.call_fn(config, &[]);
        self.config = serde_v8::from_v8(&mut self.runtime.context_scope, config).unwrap();
        self
    }

    pub fn create_game_window(&mut self) -> &mut Self {
        self.window = Some(
            WindowSettings::new("name".to_string(), [self.config.width, self.config.height])
                .exit_on_esc(true)
                .build::<PistonWindow>()
                .unwrap(),
        );
        self
    }
    pub fn start_game_loop(&mut self) {
        let window = self.window.as_mut().unwrap();
        while let Some(event) = window.next() {
            let input = event.button_args();
            let input = serde_v8::to_v8(&mut self.runtime.context_scope, input).unwrap();

            self.runtime.call_fn(self.update_fn, &[input]);
            window.draw_2d(&event, |ctx, gfx, _device| {
                self.runtime.call_fn(self.draw_fn, &[]);
                while let Ok(msg) = self.rx.try_recv() {
                    match msg {
                        Msg::Clear(rgba) => {
                            clear(rgba, gfx);
                        }
                        Msg::Rect(rgba, pos) => {
                            rectangle(
                                rgba, // red
                                pos,
                                ctx.transform,
                                gfx,
                            );
                        }
                    }
                }
            });
        }
    }

    fn global(
        isolate_scope: &mut v8::HandleScope<'s, ()>,
        global: &mut v8::Local<'s, v8::ObjectTemplate>,
        external: v8::Local<'s, v8::External>,
    ) {
        let clear = v8::FunctionBuilder::<v8::FunctionTemplate>::new(
            |scope: &mut v8::HandleScope,
             args: v8::FunctionCallbackArguments,
             mut _retval: v8::ReturnValue| {
                let arg: [f32; 4] = serde_v8::from_v8(scope, args.get(0)).unwrap();

                let tx = Runtime::get_external::<External>(&args);
                tx.send(Msg::Clear(arg)).unwrap();
            },
        );
        let clear = clear.data(external.into()).build(isolate_scope);

        global.set(
            v8::String::new(isolate_scope, "Clear").unwrap().into(),
            clear.into(),
        );

        let rect = v8::FunctionBuilder::<v8::FunctionTemplate>::new(
            |scope: &mut v8::HandleScope,
             args: v8::FunctionCallbackArguments,
             mut _retval: v8::ReturnValue| {
                let arg: [f32; 4] = serde_v8::from_v8(scope, args.get(0)).unwrap();
                let arg1: [f64; 4] = serde_v8::from_v8(scope, args.get(1)).unwrap();

                let tx = Runtime::get_external::<External>(&args);
                tx.send(Msg::Rect(arg, arg1)).unwrap();
            },
        );
        let rect = rect.data(external.into()).build(isolate_scope);

        global.set(
            v8::String::new(isolate_scope, "Rect").unwrap().into(),
            rect.into(),
        );
    }
}

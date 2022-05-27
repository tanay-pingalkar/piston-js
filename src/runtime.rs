use std::{
    collections::HashMap,
    sync::{mpsc::channel, Arc, Mutex},
};

use crate::utils::{i32vec_to_f644, i32vec_to_rgba, js_to_int};
use piston_window::*;
use piston_window::{PistonWindow, WindowSettings};
use quick_js::{Context, JsValue};

#[derive(PartialEq, Debug)]
enum Msg {
    Clear(Vec<i32>),
    Rect(Vec<i32>, Vec<i32>),
    Exit,
}

#[derive(Clone)]
pub struct Runtime {
    window: Arc<Mutex<Option<PistonWindow>>>,
    data: Arc<Mutex<HashMap<String, JsValue>>>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            window: Arc::new(Mutex::new(None)),
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_window(&mut self, window_file: String) -> &mut Self {
        let window = Arc::new(Mutex::new(None));
        let context = Context::new().unwrap();
        let window2 = Arc::clone(&window);
        context
            .add_callback("CreateGameWindow", move |game: HashMap<String, JsValue>| {
                let name = game.get("name").unwrap().as_str().unwrap();
                let height = js_to_int(game.get("height").unwrap());
                let width = js_to_int(game.get("width").unwrap());
                let mut window2 = window2.lock().unwrap();
                *window2 = Some(
                    WindowSettings::new(
                        name,
                        [f64::from(width.clone()), f64::from(height.clone())],
                    )
                    .exit_on_esc(true)
                    .build()
                    .unwrap(),
                );

                JsValue::Undefined
            })
            .unwrap();
        context.eval(&window_file).unwrap();
        self.window = window;
        self
    }

    pub fn init_data(&mut self, data_file: String) -> &mut Self {
        let context = Context::new().unwrap();
        let data2 = Arc::clone(&self.data);
        context
            .add_callback("Data", move |data: HashMap<String, JsValue>| {
                let mut data2 = data2.lock().unwrap();
                *data2 = data;
                JsValue::Undefined
            })
            .unwrap();

        context.eval(&data_file).unwrap();
        self
    }

    pub fn start_game_loop(&mut self, frame_file: String) -> &mut Self {
        let window = Arc::clone(&self.window);
        let mut window = window.lock().unwrap();
        let window = window.as_mut().unwrap();

        let frame_file = frame_file.clone();
        let (tx, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));
        let data2 = Arc::clone(&self.data);

        let context = Context::new().unwrap();
        let tx1 = Arc::new(Mutex::new(tx.clone()));
        let tx2 = Arc::clone(&tx1);
        let tx3 = Arc::clone(&tx1);

        context
            .add_callback("Set", move |map: HashMap<String, JsValue>| {
                let mut data2 = data2.lock().unwrap();
                for (key, val) in map.iter() {
                    if let Some(x) = data2.get_mut(key) {
                        *x = val.clone();
                    }
                }

                JsValue::Undefined
            })
            .unwrap();

        context
            .add_callback("Clear", move |x: Vec<i32>| {
                let tx2 = tx2.lock().unwrap();

                tx2.send(Msg::Clear(x)).unwrap();

                JsValue::Undefined
            })
            .unwrap();

        context
            .add_callback("Rect", move |color: Vec<i32>, xyz: Vec<i32>| {
                let tx3 = tx3.lock().unwrap();

                tx3.send(Msg::Rect(color, xyz)).unwrap();

                JsValue::Undefined
            })
            .unwrap();

        context
            .add_callback("Log", move |log: Vec<JsValue>| {
                println!("{:?}", log);
                JsValue::Undefined
            })
            .unwrap();

        while let Some(event) = window.next() {
            let data_val = self.data.lock().unwrap();

            context.set_global("D", data_val.clone()).unwrap();
            drop(data_val);

            let input = match event.button_args() {
                Some(button) => HashMap::from([
                    ("state".to_string(), format!("{:?}", button.state)),
                    ("key".to_string(), format!("{:?}", button.button)),
                ]),
                None => HashMap::from([
                    ("state".to_string(), "None".to_string()),
                    ("key".to_string(), "None".to_string()),
                ]),
            };

            context.set_global("Input", input).unwrap();
            let rxm = rx.lock().unwrap();
            let rx = &*rxm;
            context.eval(&frame_file).unwrap();

            tx.send(Msg::Exit).unwrap();
            window.draw_2d(&event, |c, g, _device| {
                while let Ok(msg) = rx.try_recv() {
                    match msg {
                        Msg::Clear(rgba) => {
                            clear(i32vec_to_rgba(rgba), g);
                        }
                        Msg::Rect(rgba, pos) => {
                            rectangle(
                                i32vec_to_rgba(rgba), // red
                                i32vec_to_f644(pos),
                                c.transform,
                                g,
                            );
                        }

                        Msg::Exit => (),
                    }
                }
            });
        }
        self
    }
}

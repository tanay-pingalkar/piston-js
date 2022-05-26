use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver},
        Arc, Mutex,
    },
    thread,
};

use crate::utils::{i32vec_to_f644, i32vec_to_rgba, js_to_int};
use piston_window::*;
use piston_window::{PistonWindow, WindowSettings};
use quick_js::{Context, JsValue};

#[derive(PartialEq)]
enum Msg {
    Draw2D,
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
                        [f64::from(height.clone()), f64::from(width.clone())],
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

        while let Some(event) = window.next() {
            let frame_file = frame_file.clone();
            let (tx, rx) = channel();
            let data2 = Arc::clone(&self.data);
            let data3 = Arc::clone(&self.data);
            let thread = thread::spawn(move || -> () {
                let context = Context::new().unwrap();
                let tx1 = Arc::new(Mutex::new(tx.clone()));
                let tx2 = Arc::clone(&tx1);
                let tx3 = Arc::clone(&tx1);

                let data2val = data2.lock().unwrap();

                context.set_global("D", data2val.clone()).unwrap();
                drop(data2val);
                context
                    .add_callback("Set", move |map: HashMap<String, JsValue>| {
                        let mut data3 = data3.lock().unwrap();
                        for (key, val) in map.iter() {
                            if let Some(x) = data3.get_mut(key) {
                                *x = val.clone();
                            }
                        }

                        JsValue::Undefined
                    })
                    .unwrap();
                context
                    .add_callback("Draw2d", move || {
                        let tx1 = tx1.lock().unwrap();

                        tx1.send(Msg::Draw2D).unwrap();

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

                context.eval(&frame_file).unwrap();
                tx.send(Msg::Exit).unwrap()
            });

            thread.join().unwrap();
            match rx.recv().unwrap() {
                Msg::Draw2D => {
                    window.draw_2d(&event, |c, g, _device| {
                        self.clone().recursive_rx_recv(rx, c, g);
                    });
                }
                _ => panic!("initialise if you want to draw 2d"),
            }
        }
        self
    }

    fn recursive_rx_recv(
        self,
        rx: Receiver<Msg>,
        c: piston_window::Context,
        g: &mut impl piston_window::Graphics,
    ) {
        match rx.recv().unwrap() {
            Msg::Draw2D => panic!("already initialised"),
            Msg::Clear(rgba) => {
                clear(i32vec_to_rgba(rgba), g);
                self.recursive_rx_recv(rx, c, g);
            }
            Msg::Rect(rgba, pos) => {
                rectangle(
                    i32vec_to_rgba(rgba), // red
                    i32vec_to_f644(pos),
                    c.transform,
                    g,
                );

                self.recursive_rx_recv(rx, c, g);
            }
            Msg::Exit => (),
        }
    }
}

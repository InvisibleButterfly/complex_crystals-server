extern crate rustc_serialize;
extern crate iron;

mod game_engine;
mod network_engine;

use game_engine::GameEngine;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex_engine = Arc::new(Mutex::new(GameEngine::new()));

    mutex_engine.lock().unwrap().add_object("Object1".to_string(), 10.0f32, 20.0f32);
    mutex_engine.lock().unwrap().add_object("Object2".to_string(), 100.0f32, 10.0f32);

    let cloned_engine = mutex_engine.clone();
    thread::spawn(move || network_engine::start(cloned_engine));

    mutex_engine.lock().unwrap().add_object("Object3".to_string(), 100.0f32, 10.0f32);
    println!("Test");
    loop {
        let mut engine = mutex_engine.lock().unwrap();
        engine.game_loop();
    }
}
extern crate rustc_serialize;
extern crate iron;
extern crate time;

mod game_engine;
mod network_engine;
mod level_generator;

use game_engine::GameEngine;
use game_engine::sampleobject::ObjectType;
use std::sync::{Arc, Mutex};
use std::thread;

const FLOAT_ERR: f64 = std::f64::EPSILON;

fn main() {
    let mutex_engine = Arc::new(Mutex::new(GameEngine::new()));

    level_generator::generate(mutex_engine.clone(), 1, 800.0, 600.0);

    let cloned_engine = mutex_engine.clone();
    thread::spawn(move || network_engine::start(cloned_engine));

    let interval = 1_000_000_000 / 60;
    let mut before = time::precise_time_ns();
    let mut last_second = time::precise_time_ns();
    let mut tps = 0u16;

    'running: loop {
        let mut engine = mutex_engine.lock().unwrap();
        let now = time::precise_time_ns();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000_000.0;

        if dt < interval {
            thread::sleep(std::time::Duration::from_millis((interval - dt) / 1_000_000));
            continue 'running;
        }

        before = now;
        tps += 1;

        if now - last_second > 1_000_000_000 {
            println!("TPS: {}", tps);
            last_second = now;
            engine.update_tps(tps);
            tps = 0;
        }

        engine.game_loop(elapsed);
    }
}

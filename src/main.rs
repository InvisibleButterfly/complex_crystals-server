extern crate rustc_serialize;
extern crate iron;
extern crate time;

mod game_engine;
mod network_engine;

use game_engine::GameEngine;
use game_engine::sampleobject::ObjectType;
use std::sync::{Arc, Mutex};
use std::thread;

const FLOAT_ERR: f64 = std::f64::EPSILON;

fn main() {
    let mutex_engine = Arc::new(Mutex::new(GameEngine::new()));

    mutex_engine.lock().unwrap().add_object("Asteroid".to_string(),
                                            100.0,
                                            200.0,
                                            ObjectType::Asteroid,
                                            "unknown".to_owned());

    mutex_engine.lock().unwrap().add_object("Object1".to_string(),
                                            10.0f64,
                                            20.0f64,
                                            ObjectType::Harvester,
                                            "unknown".to_owned());
    mutex_engine.lock().unwrap().add_object("Object2".to_string(),
                                            100.0f64,
                                            10.0f64,
                                            ObjectType::Harvester,
                                            "player".to_owned());

    let cloned_engine = mutex_engine.clone();
    thread::spawn(move || network_engine::start(cloned_engine));

    mutex_engine.lock().unwrap().add_object("Object3".to_string(),
                                            100.0f64,
                                            10.0f64,
                                            ObjectType::Harvester,
                                            "player".to_owned());
    mutex_engine.lock()
        .unwrap()
        .add_object("Object4".to_string(),
                    110.0,
                    200.0,
                    ObjectType::Harvester,
                    "player".to_owned());
    mutex_engine.lock().unwrap().add_object("Battlecruiser".to_string(),
                                            500.0,
                                            300.0,
                                            ObjectType::Battlecruiser,
                                            "player".to_owned());
    mutex_engine.lock()
        .unwrap()
        .set_object_dest("Battlecruiser".to_string(), 0.0, 0.0, "player".to_owned());

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

extern crate rustc_serialize;
extern crate iron;
extern crate time;
extern crate getopts;
extern crate rand;

pub mod game_engine;
pub mod network_engine;
pub mod level_generator;

use game_engine::GameEngine;
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;
use getopts::Options;

const FLOAT_ERR: f64 = std::f64::EPSILON;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optopt("w", "width", "Map width", "WIDTH");
    opts.optopt("h", "height", "Map height", "HEIGHT");
    opts.optmulti("p", "players", "Players", "PLAYERS");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let width = matches.opt_str("w").unwrap().parse::<f64>().unwrap();
    let height = matches.opt_str("h").unwrap().parse::<f64>().unwrap();
    let players = matches.opt_strs("p");
    println!("Ширина: {}", width);
    println!("Высота: {}", height);
    println!("Игроки: {:?}", players);

    let mutex_engine = Arc::new(Mutex::new(GameEngine::new()));

    level_generator::generate(mutex_engine.clone(), width, height, players);

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
            // println!("TPS: {}", tps);
            last_second = now;
            engine.update_tps(tps);
            tps = 0;
        }

        engine.game_loop(elapsed);
    }
}

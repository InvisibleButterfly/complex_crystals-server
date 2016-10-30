mod diamond_square;

use std::sync::{Arc, Mutex};
use ::game_engine::GameEngine;
use ::game_engine::sampleobject::ObjectType;

pub fn generate(mutex: Arc<Mutex<GameEngine>>,
                map_width: f64,
                map_height: f64,
                players: Vec<String>) {
    let mut engine = mutex.lock().unwrap();

    print!("Генерация астероидов... ");
    const NOISE_SIZE: usize = 65;
    let asteroid_noise = diamond_square::generate_ds(NOISE_SIZE);
    for y in 0..NOISE_SIZE {
        for x in 0..NOISE_SIZE {
            let level = asteroid_noise[x][y];
            if level > 14.0 {
                engine.add_object("Asteroid".to_string(),
                                  x as f64 * 100.0,
                                  y as f64 * 100.0,
                                  ObjectType::Asteroid,
                                  "unknown".to_owned());
            }
        }
    }
    println!("Завершена");

    for (i, player) in players.iter().enumerate() {
        let mut basename = player.clone();
        basename.push_str("Base");
        engine.add_object(basename.to_owned(),
                          100.0,
                          100.0 * i as f64,
                          ObjectType::Builder,
                          player.clone());
    }
}

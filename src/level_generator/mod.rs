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
    const NOISE_SIZE: usize = 17;
    let asteroid_noise = diamond_square::generate_ds(NOISE_SIZE);
    for y in 0..NOISE_SIZE {
        for x in 0..NOISE_SIZE {
            let level = asteroid_noise[x][y];
            print!(" {} ", level);
            if level > 5.5 {
                engine.add_object("Asteroid".to_string(),
                                  x as f64 * 100.0,
                                  y as f64 * 100.0,
                                  ObjectType::Asteroid,
                                  "unknown".to_owned());
            }
        }
    }
    println!("Завершена");

    engine.add_object("Asteroid".to_string(),
                      100.0,
                      200.0,
                      ObjectType::Asteroid,
                      "unknown".to_owned());
    engine.add_object("Object1".to_string(),
                      10.0,
                      20.0,
                      ObjectType::Harvester,
                      "unknown".to_owned());
    engine.add_object("Object2".to_string(),
                      100.0,
                      10.0,
                      ObjectType::Harvester,
                      "player".to_owned());

    engine.add_object("Object3".to_string(),
                      100.0f64,
                      10.0f64,
                      ObjectType::Harvester,
                      "player".to_owned());
    engine.add_object("Object4".to_string(),
                      110.0,
                      200.0,
                      ObjectType::Harvester,
                      "player".to_owned());
    engine.add_object("Battlecruiser".to_string(),
                      500.0,
                      300.0,
                      ObjectType::Battlecruiser,
                      "player".to_owned());
    engine.set_object_dest("Battlecruiser".to_string(), 0.0, 0.0, "player".to_owned());
}

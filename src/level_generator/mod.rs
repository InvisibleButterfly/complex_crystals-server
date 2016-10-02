use std::sync::{Arc, Mutex};
use ::game_engine::GameEngine;
use ::game_engine::sampleobject::ObjectType;

pub fn generate(mutex: Arc<Mutex<GameEngine>>,
                player_count: i32,
                map_width: f64,
                map_height: f64) {
    let mut engine = mutex.lock().unwrap();

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
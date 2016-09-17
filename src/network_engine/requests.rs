use std::sync::{Arc, Mutex};
use ::game_engine::GameEngine;
use ::rustc_serialize::json;
use ::network_engine::structures::*;

pub fn objects(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();

    let mut objects = vec![];
    for obj in engine.objects.clone() {
        objects.push(SampleObjectResponse {
            name: obj.name,
            x: obj.x,
            y: obj.y,
        });
    }
    Some(json::encode(&objects).unwrap())
}

pub fn move_object(mutex: &Arc<Mutex<GameEngine>>, input: String) -> bool {
    let mut engine = mutex.lock().unwrap();
    match json::decode(&input) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return false;
        }
        Ok(data) => {
            let mvr: MoveObjectRequest = data;
            engine.set_object_dest(mvr.name, mvr.x, mvr.y);
        }
    }
    true
}

pub fn info(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();
    Some(json::encode(&engine.info).unwrap())
}

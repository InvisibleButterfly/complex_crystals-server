use std::sync::{Arc, Mutex};
use ::game_engine::GameEngine;
use ::rustc_serialize::json;
use ::network_engine::structures::*;
use ::game_engine::modules::RadarTypes;

pub fn objects(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();

    let mut objects = vec![];
    for obj in engine.objects.clone() {
        objects.push(SampleObjectResponse {
            name: obj.name,
            x: obj.x,
            y: obj.y,
            otype: obj.otype,
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

pub fn radar(mutex: &Arc<Mutex<GameEngine>>, request: String) -> Option<String> {
    let engine = mutex.lock().unwrap();
    match json::decode(&request) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return None;
        }
        Ok(data) => {
            let name: NameResponse = data;

            let object = match engine.get_object(name.name) {
                Some(expr) => expr.clone(),
                None => return None,
            };

            let objects = match object.radar
                .get_nearby_objects(object.x, object.y, &engine.objects) {
                Some(expr) => expr,
                None => return None,
            };

            match object.radar.rtype {
                RadarTypes::Simple => {
                    let mut result = Vec::new();
                    for obj in objects {
                        result.push(SimpleRadarRequest {
                            x: obj.x,
                            y: obj.y,
                        });
                    }
                    return Some(json::encode(&result).unwrap());
                }
                RadarTypes::Middle => {
                    let mut result = Vec::new();
                    for obj in objects {
                        result.push(MiddleRadarRequest {
                            x: obj.x,
                            y: obj.y,
                            name: obj.name.clone(),
                            otype: obj.otype,
                        });
                    }
                    return Some(json::encode(&result).unwrap());
                }
                RadarTypes::Military => {
                    let mut result = Vec::new();
                    for obj in objects {
                        result.push(MilitaryRadarRequest {
                            x: obj.x,
                            y: obj.y,
                            name: obj.name.clone(),
                            otype: obj.otype.clone(),
                            speed: obj.drive.speed,
                        });
                    }
                    return Some(json::encode(&result).unwrap());
                }
            }
        }        
    }

    None
}
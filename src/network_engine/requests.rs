use std::sync::{Arc, Mutex};
use ::game_engine::GameEngine;
use ::game_engine::events::*;
use ::rustc_serialize::json;
use ::network_engine::structures::*;
use ::game_engine::sampleobject::SampleObject;

pub fn world_size(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();
    let response = WorldSizeResponse {
        width: engine.world_size_x,
        height: engine.world_size_y,
    };
    Some(json::encode(&response).unwrap())
}

pub fn info(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();
    Some(json::encode(&engine.info).unwrap())
}

pub fn objects(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();
    let objects: Vec<SampleObject> = engine.objects
        .clone()
        .iter()
        .map(|x| {
            let (_, v) = x;
            let obj = v.clone();
            SampleObject {
                owner: obj.owner,
                name: obj.name,
                otype: obj.otype,
                x: obj.x,
                y: obj.y,
                drive_speed: obj.drive_speed,
                drive_dest_x: obj.drive_dest_x,
                drive_dest_y: obj.drive_dest_y,
                radar_radius: obj.radar_radius,
                radar_type: obj.radar_type,
                weapon_active: obj.weapon_active,
                weapon_type: obj.weapon_type,
                weapon_radius: obj.weapon_radius,
                weapon_target_x: obj.weapon_target_x,
                weapon_target_y: obj.weapon_target_y,
                cargo_type: obj.cargo_type,
                cargo_max: obj.cargo_max,
                cargo_current: obj.cargo_current,
                shell_health: obj.shell_health,
                shell_type: obj.shell_type,
            }
        })
        .collect();
    Some(json::encode(&objects).unwrap())
}

pub fn move_object(mutex: &Arc<Mutex<GameEngine>>, input: String, owner: String) -> bool {
    let mut engine = mutex.lock().unwrap();
    match json::decode(&input) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return false;
        }
        Ok(data) => {
            let mvr: MoveObjectRequest = data;

            println!("Передвижение объекта {} -- x: {} y: {}",
                     mvr.name,
                     mvr.x,
                     mvr.y);

            engine.add_event(Event::MoveRequest(NetworkMoveEvent {
                name: mvr.name,
                owner: owner,
                dest_x: mvr.x,
                dest_y: mvr.y,
            }));
        }
    }
    true
}

pub fn radar(mutex: &Arc<Mutex<GameEngine>>, owner: String) -> Option<String> {
    let engine = mutex.lock().unwrap();
    let scan_result = engine.radar_scan(&owner, true);
    Some(json::encode(&scan_result).unwrap())
}

pub fn weapon_fire(mutex: &Arc<Mutex<GameEngine>>, request: String, owner: String) -> bool {
    let mut engine = mutex.lock().unwrap();
    match json::decode(&request) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return false;
        }
        Ok(data) => {
            let wfr: WeaponFireRequest = data;
            println!("Огонь объекта {} -- x: {} y: {}",
                     wfr.name,
                     wfr.x,
                     wfr.y);

            engine.add_event(Event::FireRequest(NetworkFireEvent {
                name: wfr.name,
                owner: owner,
                dest_x: wfr.x,
                dest_y: wfr.y,
            }));
        }
    }
    true
}

pub fn build(mutex: &Arc<Mutex<GameEngine>>, request: String, owner: String) -> bool {
    let mut engine = mutex.lock().unwrap();
    match json::decode(&request) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return false;
        }
        Ok(data) => {
            let req: BuildRequest = data;

            println!("Постройка объекта {} при помощи {}",
                     req.oname,
                     req.name);

            engine.add_event(Event::BuildRequest(NetworkBuildEvent {
                name: req.name,
                owner: owner,
                b_type: req.otype,
                b_name: req.oname,
            }));

        }
    }
    true
}
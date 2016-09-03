extern crate rustc_serialize;
extern crate iron;

mod game_engine;

use game_engine::GameEngine;
use std::sync::{Arc, Mutex};
use std::thread;
use rustc_serialize::json;

use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;

struct Router {
    routes: HashMap<String, Box<Handler>>,
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: String, handler: H)
        where H: Handler
    {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}

fn main() {
    let engine = Arc::new(Mutex::new(GameEngine::new()));

    engine.lock().unwrap().add_object("Object1".to_string(), 10.0f32, 20.0f32);
    engine.lock().unwrap().add_object("Object2".to_string(), 100.0f32, 10.0f32);

    let mut router = Router::new();

    let cloned_engine = engine.clone();
    router.add_route("objects".to_string(), move |_: &mut Request| {
        let engine = cloned_engine.lock().unwrap();
        let resp_json = json::encode(&engine.objects).unwrap();
        Ok(Response::with((status::Ok, resp_json)))
    });

    let cloned_engine = engine.clone();
    router.add_route("info".to_string(), move |_: &mut Request| {
        let engine = cloned_engine.lock().unwrap();
        let resp_json = json::encode(&engine.info).unwrap();
        Ok(Response::with((status::Ok, resp_json)))
    });

    thread::spawn(|| Iron::new(router).http("localhost:3000").unwrap());

    engine.lock().unwrap().add_object("Object3".to_string(), 100.0f32, 10.0f32);
    println!("Test");
    loop {
        engine.lock().unwrap().game_loop();
        // let server_info = engine.info.clone();
        // let resp_json = json::encode(&server_info).unwrap();
    }
}
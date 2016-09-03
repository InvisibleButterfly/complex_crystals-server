use super::game_engine::GameEngine;

use std::sync::{Arc, Mutex};
use std::io::Read;
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

pub fn start(mutex: Arc<Mutex<GameEngine>>) {
    let mut router = Router::new();

    let cloned_engine = mutex.clone();
    router.add_route("objects".to_string(), move |_: &mut Request| {
        let engine = cloned_engine.lock().unwrap();
        let resp_json = json::encode(&engine.objects).unwrap();
        Ok(Response::with((status::Ok, resp_json)))
    });

    let cloned_engine = mutex.clone();
    router.add_route("info".to_string(), move |_: &mut Request| {
        let engine = cloned_engine.lock().unwrap();
        let resp_json = json::encode(&engine.info).unwrap();
        Ok(Response::with((status::Ok, resp_json)))
    });

    let cloned_engine = mutex.clone();
    router.add_route("set_info".to_string(), move |req: &mut Request| {
        let mut engine = cloned_engine.lock().unwrap();
        let mut buf = String::new();
        req.body.read_to_string(&mut buf).unwrap();
        match json::decode(&buf) {
            Err(e) => println!("Json parsing error: {:?}", e),
            Ok(info) => engine.info = info,
        }
        Ok(Response::with((status::Ok)))
    });

    Iron::new(router).http("localhost:3000").unwrap();
}
mod requests;
mod structures;

use super::game_engine::GameEngine;

use std::sync::{Arc, Mutex};
use std::io::Read;

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
        match requests::objects(&cloned_engine) {
            Some(response) => Ok(Response::with((status::Ok, response))),
            None => Ok(Response::with((status::Ok))), // TODO: Заменить на ошибку, хотя вряд ли она может тут возникнуть
        }
    });

    let cloned_engine = mutex.clone();
    router.add_route("move".to_string(), move |req: &mut Request| {
        let mut buf = String::new();
        req.body.read_to_string(&mut buf).unwrap();

        if requests::move_object(&cloned_engine, buf) {
            Ok(Response::with((status::Ok)))
        } else {
            Ok(Response::with((status::Ok))) // TODO: Заменить на ошибку
        }
    });

    let cloned_engine = mutex.clone();
    router.add_route("info".to_string(), move |_: &mut Request| {
        match requests::info(&cloned_engine) {
            Some(response) => Ok(Response::with((status::Ok, response))),
            None => Ok(Response::with((status::Ok))), // TODO: Заменить на ошибку, хотя вряд ли она тут будет
        }
    });

    let cloned_engine = mutex.clone();
    router.add_route("radar".to_string(), move |req: &mut Request| {
        let mut buf = String::new();
        req.body.read_to_string(&mut buf).unwrap();

        match requests::radar(&cloned_engine, buf) {
            Some(response) => Ok(Response::with((status::Ok, response))),
            None => Ok(Response::with((status::Ok))),
        }
    });

    Iron::new(router).http("localhost:3000").unwrap();
}

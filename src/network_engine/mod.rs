

pub fn start(game: &'static game_engine::Game) {
    let mut router = Router::new();


    router.add_route("object".to_string(), move |_: &mut Request| {
        let resp_json = json::encode(&game.SomeObjects[0]).unwrap();
        Ok(Response::with((status::Ok, resp_json)))
    });
}

// pub fn route_objects(_: &mut Request) -> IronResult<Response> {
// let resp_json = json::encode(&game.get_objects()).unwrap();
//
// Ok(Response::with((status::Ok, resp_json)))
// }
 
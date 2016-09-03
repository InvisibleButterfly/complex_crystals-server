#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ServerInfo {
    name: String,
    status: String,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct SampleObject {
    name: String,
    x: f32,
    y: f32,
}

pub struct GameEngine {
    pub info: ServerInfo,
    pub objects: Vec<SampleObject>,
}

impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            objects: vec![],
            info: ServerInfo {
                name: "ServerName".to_string(),
                status: "Ok".to_string(),
            },
        }
    }
    pub fn add_object(&mut self, object_name: String, coord_x: f32, coord_y: f32) {
        self.objects.push(SampleObject {
            name: object_name,
            x: coord_x,
            y: coord_y,
        });
    }
    pub fn game_loop(&mut self) {}
}
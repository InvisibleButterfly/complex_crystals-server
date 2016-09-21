use ::game_engine::ObjectType;

#[derive(RustcEncodable)]
pub struct WorldSizeResponse {
    pub width: f64,
    pub height: f64,
}

#[derive(RustcEncodable)]
pub struct SampleObjectResponse {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub otype: ObjectType,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct MoveObjectRequest {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

#[derive(RustcDecodable)]
pub struct NameResponse {
    pub name: String,
}

#[derive(RustcEncodable)]
pub struct SimpleRadarRequest {
    pub x: f64,
    pub y: f64,
}

#[derive(RustcEncodable)]
pub struct MiddleRadarRequest {
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub otype: ObjectType,
}

#[derive(RustcEncodable)]
pub struct MilitaryRadarRequest {
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub otype: ObjectType,
    pub speed: f64,
}
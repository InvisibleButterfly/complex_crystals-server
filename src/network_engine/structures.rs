use ::game_engine::sampleobject::ObjectType;

#[derive(RustcEncodable)]
pub struct WorldSizeResponse {
    pub width: f64,
    pub height: f64,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct MoveObjectRequest {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

#[derive(RustcEncodable)]
pub struct ObjectResponse {
    pub name: String,
    pub owner: String,
    pub x: f64,
    pub y: f64,
    pub otype: ObjectType,
}

#[derive(RustcDecodable)]
pub struct NameResponse {
    pub name: String,
}

#[derive(RustcDecodable)]
pub struct WeaponFireRequest {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

#[derive(RustcDecodable)]
pub struct BuildRequest {
    pub name: String,
    pub oname: String,
    pub otype: ObjectType,
}
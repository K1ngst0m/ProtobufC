use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{Json, JsonValue};

use server::gen_protobuf;

// The type to represent the ID of a message.
type ID = usize;

// We're going to store all of the messages here. No need for a DB.
type GenUrlMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
struct ProtoRequestMessage {
    id: Option<ID>,
    language : String,
    output_name : String,
    arg : Option<String>,
    contents: String
}

#[derive(Serialize, Deserialize)]
struct ProtoResponseMessage{
    status: String,
    reason: Option<String>,
    content: Option<String>
}

#[post("/", format = "json", data = "<proto_content>")]
fn new_without_idx(proto_content: Json<ProtoRequestMessage>,
       map: State<GenUrlMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    let mut id = 0;
    loop {
        if !hashmap.contains_key(&id) {
            break;
        }
        id += 1;
    }
    let url = gen_protobuf::gen_protobuf(id, &proto_content.language,
                           proto_content.output_name.to_string(), &proto_content.arg, &proto_content.contents);
    hashmap.insert(id, url.to_string());
    json!(ProtoResponseMessage{
        status: String::from("ok"),
        reason: None,
        content: Some(String::from(url))
    })
}


#[post("/<id>", format = "json", data = "<proto_content>")]
fn new(id: ID, proto_content: Json<ProtoRequestMessage>,
       map: State<GenUrlMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        json!(ProtoResponseMessage{
            status: String::from("error"),
            reason: Some(String::from("ID exists. Try put.")),
            content: None
        })
    } else {
        let url = gen_protobuf::gen_protobuf(id, &proto_content.language,
                               proto_content.output_name.to_string(), &proto_content.arg, &proto_content.contents);
        hashmap.insert(id, url.to_string());
        json!(ProtoResponseMessage{
            status: String::from("ok"),
            reason: None,
            content: Some(String::from(url))
        })
    }
}

#[put("/<id>", format = "json", data = "<message>")]
fn update(id: ID, message: Json<ProtoRequestMessage>,
          map: State<GenUrlMap>) -> Option<JsonValue> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, message.0.contents);
        Some(json!(
                ProtoResponseMessage{
                    status: String::from("ok"),
                    reason: None,
                    content: None
                }))
    } else {
        None
    }
}


#[get("/<id>", format = "json")]
fn get(id: ID, map: State<GenUrlMap>) -> Option<Json<ProtoResponseMessage>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| {
        Json(ProtoResponseMessage {
            status: String::from("ok"),
            reason: None,
            content: Some(contents.clone())
        })
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!( ProtoResponseMessage{
            status: String::from("error"),
            reason: Some(String::from("Resources not found")),
            content: None
    })
}

// 生成编译生proto文件
pub fn run_protoc_server() -> rocket::Rocket {
    rocket::ignite()
        .mount("/protoc", routes![new_without_idx, new, update, get])
        .mount("/downloads/", StaticFiles::from("static/downloads/protogen"))
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use std::fs::File;
use std::io::prelude::*;

use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{Json, JsonValue};
extern crate glob;
use glob::glob;

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
    let url = gen_protobuf(id, &proto_content.language,
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
        let url = gen_protobuf(id, &proto_content.language,
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


fn gen_protobuf(id: usize, language: &String, mut output_name: String, _arg: &Option<String>, proto_content: &String) -> String{
    let download_root_directory = "./static/downloads/protogen".to_string();
    let work_directory_suffix = format!("gen_{}", id);
    let work_directory = format!("{}/{}", &download_root_directory, &work_directory_suffix);

    // protoc路径
    let gen_exec = format!("{}/run_protoc", &download_root_directory);

    // proto文件路径
    let proto_path = format!("--proto_path={}", &work_directory);

    // protoc输出类型
    let cpp_output = match &language[..]{
        "cpp"|"c++" => format!("--cpp_out={}", &work_directory),
        "csharp"|"c#" => {
            output_name = first_letter_upper(&output_name.clone());
            format!("--csharp_out={}", &work_directory)
        },
        "java" => format!("--java_out={}", &work_directory),
        _ => "".to_string(),
    };

    // 创建目录
    match fs::create_dir(&work_directory){
        Err(why) => panic!("couldn't create directory:{}, {}", &work_directory, why),
        Ok(directory) => directory,
    };

    // 生成proto文件
    let mut proto_file= match File::create(format!("{}/{}.proto", &work_directory, &output_name)){
        Err(why) => panic!("couldn't create {}.proto, {}", &output_name, why),
        Ok(file) => file,
    };

    match proto_file.write_all(proto_content.as_bytes()){
        Err(why) => panic!("couldn't write to {}.proto, {}", &output_name, why),
        Ok(_) => println!("successfully wrote to {}.proto", &output_name),
    };

    let src_file =
        format!("{}/{}.proto", work_directory, output_name);

    Command::new("ls")
        .arg(&work_directory)
        .output()
        .expect("zip no");

    // 打包: zip / tar.gz
    let pack_type = "zip";

    // 打包后的文件名
    let pack_path = match &pack_type[..]{
        "zip" => format!("{}/{}.zip", &work_directory, &output_name),
        "tar" => format!("{}/{}.tar.gz", &work_directory, &output_name),
        _ => format!("{}/{}.zip", &work_directory, &output_name),
    };

    // 打包的文件列表
    let pack_files = match &language[..]{
        "cpp"|"c++" => format!("{0}/{1}*.cc {0}/{1}*.h", &work_directory, &output_name),
        "csharp"|"c#" => format!("{}/{}.cs", &work_directory, &output_name),
        //"java" => format!("{}/{}", &work_directory),
        _ => "".to_string(),
    };

    // 编译proto文件并打包
    let status = Command::new(gen_exec)
        .args(&[proto_path, cpp_output, src_file,
              pack_path, pack_files])
        .status()
        .expect("proto gen error");

    println!("gen_output: {}", status);

    String::from(format!("http://localhost:8000/downloads/{}/{}.zip", &work_directory_suffix, &output_name))
}

fn first_letter_upper(s: &String) -> String{
    let mut c = s.chars();
    match c.next(){
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn rocket_protoc() -> rocket::Rocket {
    rocket::ignite()
        .mount("/protoc", routes![new_without_idx, new, update, get])
        .mount("/downloads/", StaticFiles::from("static/downloads/protogen"))
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}


fn main() ->std::io::Result<()> {
    for file in glob("./static/downloads/protogen/gen_*").expect("failed to read glob pattern"){
        fs::remove_dir_all(format!("{}", file.unwrap().display()))?;
    }

    rocket_protoc().launch();
    Ok(())
}

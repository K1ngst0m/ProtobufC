use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

// compile proto file and compression
pub fn gen_protobuf(
    id: usize,
    language: &String,
    mut output_name: String,
    _arg: &Option<String>,
    proto_content: &String,
) -> String {
    // downloads root path
    let download_root_directory = "./static/downloads/protogen".to_string();
    // generation directory
    let gen_id = format!("gen_{}", id);
    let gen_directory_path = format!("{}/{}", &download_root_directory, &gen_id);

    // protoc executable path
    let gen_exec = format!("{}/run_protoc", &download_root_directory);

    // .proto file path
    let proto_path = format!("--proto_path={}", &gen_directory_path);

    // output type (language)
    let cpp_output = match &language[..] {
        "cpp" | "c++" => format!("--cpp_out={}", &gen_directory_path),
        "csharp" | "c#" => {
            output_name = first_letter_upper(&output_name.clone());
            format!("--csharp_out={}", &gen_directory_path)
        }
        "java" => format!("--java_out={}", &gen_directory_path),
        _ => "".to_string(),
    };

    // generate directory
    match fs::create_dir(&gen_directory_path) {
        Err(why) => panic!("couldn't create directory:{}, {}", &gen_directory_path, why),
        Ok(directory) => directory,
    };

    // generate proto file
    let mut proto_file =
        match File::create(format!("{}/{}.proto", &gen_directory_path, &output_name)) {
            Err(why) => panic!("couldn't create {}.proto, {}", &output_name, why),
            Ok(file) => file,
        };
    match proto_file.write_all(proto_content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}.proto, {}", &output_name, why),
        Ok(_) => println!("successfully wrote to {}.proto", &output_name),
    };

    // proto file path
    let src_file = format!("{}/{}.proto", gen_directory_path, output_name);

    // compress type
    let pack_type = "zip";

    // compress file name
    let pack_path = match &pack_type[..] {
        "zip" => format!("{}/{}.zip", &gen_directory_path, &output_name),
        "tar" => format!("{}/{}.tar.gz", &gen_directory_path, &output_name),
        _ => format!("{}/{}.zip", &gen_directory_path, &output_name),
    };

    // compress file list
    let pack_files = match &language[..] {
        "cpp" | "c++" => format!("{0}/{1}*.cc {0}/{1}*.h", &gen_directory_path, &output_name),
        "csharp" | "c#" => format!("{}/{}.cs", &gen_directory_path, &output_name),
        //"java" => format!("{}/{}", &work_directory),
        _ => "".to_string(),
    };
    // let pack_files = format!("{}/*", &gen_directory_path);

    // compile and compress files
    let status = Command::new(gen_exec)
        .args(&[proto_path, cpp_output, src_file, pack_path, pack_files])
        .status()
        .expect("proto gen error");
    println!("gen_output: {}", status);

    // 返回url
    String::from(format!(
        "http://192.168.1.141:8000/downloads/{}/{}.zip",
        &gen_id, &output_name
    ))
}

// first letter upper
pub fn first_letter_upper(s: &String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

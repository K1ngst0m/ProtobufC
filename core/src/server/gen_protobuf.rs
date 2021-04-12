use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

pub fn gen_protobuf(id: usize, language: &String, mut output_name: String, _arg: &Option<String>, proto_content: &String) -> String{
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

pub fn first_letter_upper(s: &String) -> String{
    let mut c = s.chars();
    match c.next(){
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}


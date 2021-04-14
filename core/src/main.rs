#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

mod server;
extern crate glob;

fn main() ->std::io::Result<()> {
    // delete all generation cache
    use std::fs;
    use glob::glob;
    for file in glob("./static/downloads/protogen/gen_*").expect("failed to read glob pattern"){
        fs::remove_dir_all(format!("{}", file.unwrap().display()))?;
    }

    // launch http server
    server::run_protoc_server().launch();

    Ok(())
}

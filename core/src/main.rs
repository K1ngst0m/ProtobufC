#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use std::fs;

extern crate glob;
use glob::glob;

mod server;
use server::run_protoc_server;




fn main() ->std::io::Result<()> {
    for file in glob("./static/downloads/protogen/gen_*").expect("failed to read glob pattern"){
        fs::remove_dir_all(format!("{}", file.unwrap().display()))?;
    }

    server::run_protoc_server().launch();
    Ok(())
}

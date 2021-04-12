use rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};
use std::process::Command;

#[test]
fn file_output(){
    let output = Command::new("echo")
        .arg("hello world")
        .output()
        .expect("Failed to execute command");

    assert_eq!(b"hello world\n", output.stdout.as_slice());
}

#[test]
fn bad_get_put() {

}

#[test]
fn post_get_put_get() {

}

#[macro_use]
extern crate serde;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate paste;

extern crate toml;

mod input_file;

fn main() {
    dbg!(toml::from_str::<input_file::InputFile>(
        &std::fs::read_to_string("./input.toml").unwrap()
    ))
    .unwrap();
    println!("Hello, world!");
}

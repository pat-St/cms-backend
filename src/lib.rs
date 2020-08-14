#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#![feature(const_fn)]
extern crate dotenv;
extern crate serde;
extern crate serde_bytes;
extern crate base64;
extern crate sha2;
extern crate rand;
extern crate ring;
#[macro_use] extern crate mysql;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod service;
mod controller;
mod model;
mod util;

pub fn rocket() -> rocket::Rocket {
	util::rest_route::pre_config()
}
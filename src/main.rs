#![feature(proc_macro_hygiene, decl_macro, bind_by_move_pattern_guards)]
#![feature(plugin)]
#![feature(custom_attribute)]
#![feature(specialization)]
#![feature(const_fn)]
extern crate dotenv;
extern crate serde;
extern crate serde_bytes;
extern crate base64;
extern crate sha2;
extern crate rand;
#[macro_use] extern crate mysql;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod service;
mod controller;
mod model;

use rocket::{Request, Response};
use rocket_contrib::json::{Json, JsonValue};
use controller::{image_controller, info_text_controller, tile_controller,info_text_to_tile_controller, user_controller};
use controller::apartment_controller::*;
use service::db_connector::Connection;
use rocket::http::{Status};
use rocket::response;

#[get("/")]
pub fn hello() -> &'static str {
    "I'm up!"
}

#[catch(503)]
fn service_not_available(_req: &Request) -> Json<JsonValue> {
    Json(json!({ "status": "database currently not available"}))
}

#[catch(422)]
fn unprocess_entity(_req: &Request) ->  response::Result<'static>{
    Response::build()
        .raw_header("Access-Control-Allow-Origin", "*")
        .status(Status::UnprocessableEntity).ok()
}

#[catch(404)]
fn not_found_entity(_req: &Request) ->  response::Result<'static>{
    Response::build()
        .raw_header("Access-Control-Allow-Origin", "*")
        .status(Status::NotFound).ok()
}
fn main() {
    rocket::ignite()
        .manage(Connection::connect())
        .register(catchers![service_not_available,unprocess_entity,not_found_entity])
        .mount(
            "/",
            routes![hello],
        )
        .mount(
            "/image",
            routes![
           image_controller::load_all_description,
           image_controller::get_single_object_by_id,
           image_controller::get_all_objects,
           image_controller::create_objects,
           image_controller::update_objects,
           image_controller::upload_image,
           image_controller::load_image,
           image_controller::load_all_image_id,
           image_controller::options_response,
           image_controller::options_id_response,
           image_controller::delete_objects,
           image_controller::options_desc_response,
           image_controller::options_id_number_response,
           image_controller::options_id_string_response,
           ],
        )
        .mount(
            "/apartment",
            routes![
            apartment_content_controller::get_all_objects,
            apartment_content_controller::get_single_object_by_id,
            apartment_content_controller::create_objects,
            apartment_content_controller::update_objects,
            apartment_content_controller::options_response,
            apartment_content_controller::options_id_response,
            apartment_content_controller::delete_objects
            ],
        )
        .mount(
            "/apartment_desc",
            routes![
            apartment_description_controller::get_all_objects,
            apartment_description_controller::get_single_object_by_id,
            apartment_description_controller::create_objects,
            apartment_description_controller::update_objects,
            apartment_description_controller::options_response,
            apartment_description_controller::options_id_response,
            apartment_description_controller::delete_objects
            ],
        )
        .mount(
            "/apartment_details",
            routes![
            apartment_details_controller::get_all_objects,
            apartment_details_controller::get_single_object_by_id,
            apartment_details_controller::create_objects,
            apartment_details_controller::update_objects,
            apartment_details_controller::options_response,
            apartment_details_controller::options_id_response,
            apartment_details_controller::delete_objects
            ],
        )
        .mount(
            "/apartment_price",
            routes![
            apartment_price_controller::get_all_objects,
            apartment_price_controller::get_single_object_by_id,
            apartment_price_controller::create_objects,
            apartment_price_controller::update_objects,
            apartment_price_controller::options_response,
            apartment_price_controller::options_id_response,
            apartment_price_controller::delete_objects
            ],
        )
        .mount(
            "/details_to_apartment",
            routes![
            details_to_apartment_controller::get_all_objects,
            details_to_apartment_controller::get_single_object_by_id,
            details_to_apartment_controller::create_objects,
            details_to_apartment_controller::update_objects,
            details_to_apartment_controller::options_response,
            details_to_apartment_controller::options_id_response,
            details_to_apartment_controller::delete_objects
            ],
        )
        .mount(
            "/tile",
            routes![
            tile_controller::get_single_object_by_id,
            tile_controller::get_all_objects,
            tile_controller::create_objects,
            tile_controller::update_objects,
            tile_controller::options_response,
            tile_controller::options_id_response,
            tile_controller::delete_objects
            ],
        )
        .mount(
            "/info_text",
            routes![
            info_text_controller::get_single_object_by_id,
            info_text_controller::get_all_objects,
            info_text_controller::create_objects,
            info_text_controller::update_objects,
            info_text_controller::options_response,
            info_text_controller::options_id_response,
            info_text_controller::delete_objects
            ],
        )
        .mount(
            "/info_text_to_tile",
            routes![
                info_text_to_tile_controller::get_single_object_by_id,
                info_text_to_tile_controller::get_all_objects,
                info_text_to_tile_controller::create_objects,
                info_text_to_tile_controller::update_objects,
                info_text_to_tile_controller::options_response,
                info_text_to_tile_controller::options_id_response,
                info_text_to_tile_controller::delete_objects
                ],
        )
        .mount(
            "/user",
            routes![
                user_controller::user_controller::user_login,
                user_controller::user_controller::user_create,
                user_controller::user_controller::user_logout,
                user_controller::user_controller::options_signing_response,
                user_controller::user_controller::options_response,
                user_controller::user_controller::post_mock,
                user_controller::user_controller::check_token,
                ]
        )
        .launch();
}
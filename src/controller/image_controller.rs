use rocket::Data;
use rocket::http::{ContentType, RawStr};
use rocket_contrib::json::Json;
use model::image::image::Image;
use model::image::images_template::ImagesTemplate;
use service::response_config::{ResponseWithHeader, ImageResponse};
use model::user::user::AuthGuard;

#[get("/")]
pub fn get_all_objects(_db_conn: AuthGuard) -> ResponseWithHeader<Vec<Image>> {
    ResponseWithHeader(Image::get_multi_object(_db_conn.0))
}

#[get("/id")]
pub fn load_all_image_id(_db_conn: AuthGuard) -> ResponseWithHeader<Vec<u16>> {
    ResponseWithHeader(Image::load_all_image_id(_db_conn.0))
}

#[get("/id/<id>")]
pub fn get_single_object_by_id(id: u8, _db_conn: AuthGuard) -> ResponseWithHeader<Image> {
    ResponseWithHeader(Image::get_single_object(_db_conn.0, id))
}

#[post("/", format = "application/json", data = "<tile>")]
pub fn create_objects(tile: Json<Vec<Image>>, _db_conn: AuthGuard)-> ResponseWithHeader<bool>{
    let payload_tile = tile.into_inner();
    match Image::insert_values_object(_db_conn.0, payload_tile) {
        true => ResponseWithHeader(Some(true)),
        false => ResponseWithHeader(None)
    }
}


#[put("/", format = "application/json", data = "<tile>")]
pub fn update_objects(tile: Json<Vec<Image>>, _db_conn: AuthGuard) -> ResponseWithHeader<bool> {
    let payload_tile = tile.into_inner();
    match Image::update_values_object(_db_conn.0, payload_tile) {
        true => ResponseWithHeader(Some(true)),
        false => ResponseWithHeader(None)
    }
}

#[put("/<id>", format = "image/jpeg", data = "<tile>")]
pub fn upload_image(id: u8, tile: Data, _db_conn: AuthGuard) -> ResponseWithHeader<bool> {
    match Image::upload_image(_db_conn.0, tile, id) {
        true => ResponseWithHeader(Some(true)),
        false => ResponseWithHeader(None)
    }
}


#[get("/desc")]
pub fn load_all_description(_db_conn: AuthGuard) -> ResponseWithHeader<Vec<String>> {
    ResponseWithHeader(Image::load_all_image_desc(_db_conn.0))
}

#[get("/binary/<id>")]
pub fn load_image(id: u8, _db_conn: AuthGuard) -> ImageResponse<Vec<u8>> {
    ImageResponse(ContentType::JPEG, Image::load_image_binary(_db_conn.0, id))
}

#[delete("/<id>")]
pub fn delete_objects(id: u16, _db_conn: AuthGuard)-> ResponseWithHeader<bool> {
    match Image::delete_values_object(_db_conn.0, id) {
        true => ResponseWithHeader(Some(true)),
        false => ResponseWithHeader(None)
    }
}

#[options("/")]
pub fn options_response() -> ResponseWithHeader<bool> {
     ResponseWithHeader(Some(true))
}
#[options("/<_id>")]
pub fn options_id_response(_id: u16) -> ResponseWithHeader<bool> {
    ResponseWithHeader(Some(true))
}
#[options("/<_id>", rank = 2)]
pub fn options_desc_response(_id: &RawStr) -> ResponseWithHeader<bool> {
    ResponseWithHeader(Some(true))
}
#[options("/<_name>/<_id>")]
pub fn options_id_number_response(_name: &RawStr, _id: u8) -> ResponseWithHeader<bool> {
    ResponseWithHeader(Some(true))
}
#[options("/<_name>/<_id>", rank = 2)]
pub fn options_id_string_response(_name: &RawStr, _id: &RawStr) -> ResponseWithHeader<bool> {
    ResponseWithHeader(Some(true))
}


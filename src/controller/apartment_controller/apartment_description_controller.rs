use rocket_contrib::json::Json;
use model::fewo::apartment_description::ApartmentDescription;
use model::model_template::ModelTemplate;
use service::response_config::ResponseWithHeader;
use service::request_config::AuthGuard;

#[get("/")]
pub fn get_all_objects(_db_conn: AuthGuard) -> ResponseWithHeader<Vec<ApartmentDescription>> {
    ResponseWithHeader(ApartmentDescription::get_multi_object(_db_conn.0))
}

#[get("/<id>")]
pub fn get_single_object_by_id(id: u8, _db_conn: AuthGuard) -> ResponseWithHeader<ApartmentDescription> {
    ResponseWithHeader(ApartmentDescription::get_single_object(_db_conn.0, id) )
}

#[post("/", format = "application/json", data = "<tile>")]
pub fn create_objects(tile: Json<Vec<ApartmentDescription>>, _db_conn: AuthGuard) -> ResponseWithHeader<bool> {
    let payload_tile = tile.into_inner();
    match ApartmentDescription::insert_values_object(_db_conn.0, payload_tile) {
        true => ResponseWithHeader(Some(true)),
        false => ResponseWithHeader(None)
    }
}

#[put("/", format = "application/json", data = "<tile>")]
pub fn update_objects(tile: Json<Vec<ApartmentDescription>>, _db_conn: AuthGuard) -> ResponseWithHeader<bool> {
    let payload_tile = tile.into_inner();
    match ApartmentDescription::update_values_object(_db_conn.0, payload_tile) {
        true => ResponseWithHeader(Some(true)),
        false => ResponseWithHeader(None)
    }
}
#[delete("/<id>")]
pub fn delete_objects(id: u16, _db_conn: AuthGuard) -> ResponseWithHeader<bool> {
    match ApartmentDescription::delete_values_object(_db_conn.0, id) {
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
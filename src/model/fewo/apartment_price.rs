use mysql;
use mysql::Row;
use crate::{service,model};
use model::model_template::ModelTemplate;
use service::db_connector::Connection;
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApartmentPrice {
    pub ID: u16,
    pub personCount: String,
    pub peakSeason: String,
    pub offSeason: String,
    pub nights: String,
    pub fk_apartment: u16,
}

impl ModelTemplate<ApartmentPrice> for ApartmentPrice {
    fn update_values_object(conn: Connection, insert_object: Vec<ApartmentPrice>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare(
            "UPDATE ApartmentPrice SET \
            personCount=:person_count, \
            peakSeason=:peak_season, \
            offSeason=:off_season, \
            nights=:u_nights, \
            fk_apartment=:u_fk_apartment \
            WHERE id=:u_id") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "person_count"=>&obj.personCount,
                            "peak_season"=>&obj.peakSeason,
                            "off_season"=>&obj.offSeason,
                            "u_nights"=>&obj.nights,
                            "u_fk_apartment"=>obj.fk_apartment,
                        }
                    ).unwrap();
                }
            }
            Err(e) => {
                println!("{}", e.to_string());
                return false;
            }
        };
        true
    }
    fn delete_values_object(conn: Connection, object_id: u16) -> bool {
        match conn.get_connection() {
            Some(mut value) => 
                match value.prepare("DELETE FROM ApartmentPrice WHERE id=:u_id") {
                    Ok(mut stmt) => {
                            stmt.execute(params! {"u_id"=>object_id}).unwrap();
                            true
                    }
                    Err(e) => {
                        println!("{}", e.to_string());
                        false
                    }
                },
            None => false
        }
    }
    fn insert_values_object(conn: Connection, insert_object: Vec<ApartmentPrice>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare("INSERT INTO ApartmentPrice (id, personCount, peakSeason, offSeason, nights, fk_apartment)  VALUES (:u_id, :person_count, :peak_season, :off_season, :u_nights, :u_fk_apartment)") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "person_count"=>&obj.personCount,
                            "peak_season"=>&obj.peakSeason,
                            "off_season"=>&obj.offSeason,
                            "u_nights"=>&obj.nights,
                            "u_fk_apartment"=>obj.fk_apartment,
                        }
                    ).unwrap();
                }
            }
            Err(e) => println!("{}", e.to_string())
        };
        true
    }
    fn get_single_object(conn: Connection, item_id: u8) -> Option<ApartmentPrice> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM ApartmentPrice WHERE id=:item_id", params! {"item_id"=>item_id});
        Self::query_to_object(result)
    }
    fn get_multi_object(conn: Connection) -> Option<Vec<ApartmentPrice>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM ApartmentPrice", ());
        Self::query_list_to_object(result)
    }
    fn convert(item: Row) -> Option<ApartmentPrice> {
        match mysql::from_row_opt(item) {
            Ok(value) => {
                let q_obj: (u16, String, String, String, String, u16) = value;
                Some(Self { ID: q_obj.0, personCount: q_obj.1, peakSeason: q_obj.2, offSeason: q_obj.3, nights: q_obj.4 ,fk_apartment: q_obj.5 })
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
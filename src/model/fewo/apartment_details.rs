use mysql;
use mysql::Row;
use crate::{service,model};
use model::model_template::ModelTemplate;
use service::db_connector::Connection;
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApartmentDetails {
    pub ID: u16,
    pub identifier: String,
}

impl ModelTemplate<ApartmentDetails> for ApartmentDetails {
    fn update_values_object(conn: Connection, insert_object: Vec<ApartmentDetails>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare(
            "UPDATE ApartmentDetails SET \
            identifier=:u_identifier \
            WHERE id=:u_id") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "u_identifier"=>&obj.identifier,
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
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare(
            "DELETE FROM ApartmentDetails \
            WHERE id=:u_id") {
            Ok(mut stmt) => {
                stmt.execute(params! {"u_id"=>object_id}).unwrap();
                true
            }
            Err(e) => {
                println!("{}", e.to_string());
                false
            }
        };
        true
    }
    fn insert_values_object(conn: Connection, insert_object: Vec<ApartmentDetails>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare("INSERT INTO ApartmentDetails (id,identifier) VALUES (:u_id,:u_identifier)") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "u_identifier"=>&obj.identifier,
                        }
                    ).unwrap();
                }
            }
            Err(e) => println!("{}", e.to_string())
        };
        true
    }
    fn get_single_object(conn: Connection, item_id: u8) -> Option<ApartmentDetails> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM ApartmentDetails WHERE id=:item_id", params! {"item_id"=>item_id});
        Self::query_to_object(result)
    }
    fn get_multi_object(conn: Connection) -> Option<Vec<ApartmentDetails>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM ApartmentDetails", ());
        Self::query_list_to_object(result)
    }
    fn convert(item: Row) -> Option<ApartmentDetails> {
        match mysql::from_row_opt(item) {
            Ok(value) => {
                let q_obj: (u16, String) = value;
                Some(Self { ID: q_obj.0, identifier: q_obj.1 })
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
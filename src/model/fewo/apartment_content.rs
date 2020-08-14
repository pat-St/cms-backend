use mysql;
use mysql::Row;
use std::fmt::{Debug, Formatter, Result};
use crate::{service,model};
use model::model_template::ModelTemplate;
use service::db_connector::Connection;
#[allow(non_snake_case)]
#[derive(PartialEq, Eq, Serialize, Deserialize,Default)]
pub struct ApartmentContent {
    pub ID: u16,
    pub fk_tile: u16,
}

impl ModelTemplate<ApartmentContent> for ApartmentContent {
    fn update_values_object(conn: Connection, insert_object: Vec<ApartmentContent>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare(
            "UPDATE ApartmentContent SET \
             fk_tile=:fk_tile \
             WHERE id=:u_id") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                           "u_id"=>obj.ID,
                        "fk_tile"=>obj.fk_tile
                        }
                    ).unwrap();
                }
            }
            Err(e) => {
                println!("{}", e.to_string());
                return false;
            }
        }
        true
    }
    fn delete_values_object(conn: Connection, object_id: u16) -> bool {
        match conn.get_connection() {
            Some(mut value) => 
            match value.prepare("DELETE FROM ApartmentContent WHERE id=:u_id") {
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
    fn insert_values_object(conn: Connection, insert_object: Vec<ApartmentContent>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare("INSERT INTO ApartmentContent (id,fk_tile) VALUES (:u_id,:fk_tile)") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                        "u_id"=>obj.ID,
                        "fk_tile"=>obj.fk_tile
                        }
                    ).unwrap();
                }
            }
            Err(e) => println!("{}", e.to_string())
        }
        true
    }
    fn get_single_object(conn: Connection, item_id: u8) -> Option<ApartmentContent> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM ApartmentContent WHERE id=:item_id", params! {"item_id"=>item_id});
        Self::query_to_object(result)
    }
    fn get_multi_object(conn: Connection) -> Option<Vec<ApartmentContent>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM ApartmentContent", ());
        Self::query_list_to_object(result)
    }
    fn convert(item: Row) -> Option<ApartmentContent> {
        match mysql::from_row_opt(item) {
            Ok(value) => {
                let q_obj: (u16, u16) = value;
                Some(Self { ID: q_obj.0, fk_tile: q_obj.1 })
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
//impl Debug for ApartmentContent {
//    fn fmt(&self, fmt: &mut Formatter) -> Result {
//        fmt.debug_struct("")
//            .field("\"ID\"", &self.ID)
//            .field("\"fk_tile\"", &self.fk_tile)
//            .finish()
//    }
//}
impl Debug for ApartmentContent {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        formatter.debug_tuple("String").field(&self).finish()
    }
}
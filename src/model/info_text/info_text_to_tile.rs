use mysql::Row;
use crate::{service,model};
use model::model_template::ModelTemplate;
use service::db_connector::Connection;
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoTextToTile {
    pub ID: u16,
    pub fk_info: u16,
    pub fk_tile: u16,
}

impl ModelTemplate<InfoTextToTile> for InfoTextToTile {
    fn update_values_object(conn: Connection, insert_object: Vec<InfoTextToTile>) -> bool {
        let mut connection = match conn.get_connection() {
            Some(value) => value,
            None => return false
        };
        match connection.prepare(
            "UPDATE InfoTextToTile SET \
            fk_info=:u_fk_info, \
            fk_tile=:u_fk_tile \
            WHERE id=:item_id") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "item_id"=>obj.ID,
                           "u_fk_info"=>obj.fk_info,
                           "u_fk_tile"=>obj.fk_tile}
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
                match value.prepare("DELETE FROM InfoTextToTile WHERE id=:item_id") {
                    Ok(mut stmt) => {                
                            stmt.execute(params! {"item_id"=>object_id}).unwrap();
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
    fn insert_values_object(conn: Connection, insert_object: Vec<InfoTextToTile>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare(
            "INSERT INTO InfoTextToTile (id,fk_info,fk_tile) VALUES (:u_id,:u_fk_info,:u_fk_tile)") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "u_fk_info"=>&obj.fk_info,
                            "u_fk_tile"=>obj.fk_tile}
                    ).unwrap();
                }
            }
            Err(e) => {
                println!("{}", e.to_string());
                return false 
            }
        };
        true
    }
    fn get_single_object(conn: Connection, item_id: u8) -> Option<InfoTextToTile> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM InfoTextToTile WHERE id=:item_id", params! {"item_id"=>item_id});
        Self::query_to_object(result)
    }
    fn get_multi_object(conn: Connection) -> Option<Vec<InfoTextToTile>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM InfoTextToTile", ());
        Self::query_list_to_object(result)
    }
    fn convert(item: Row) -> Option<InfoTextToTile> {
        match mysql::from_row_opt(item) {
            Ok(value) => {
                let q_obj: (u16,u16, u16) = value;
                Some(Self { ID: q_obj.0, fk_info: q_obj.1, fk_tile: q_obj.2 })
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
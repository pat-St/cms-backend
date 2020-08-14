use mysql;
use mysql::Row;
use crate::{service,model};
use model::model_template::ModelTemplate;
use service::db_connector::Connection;
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApartmentDescription {
    pub ID: u16,
    pub description: String,
    pub info: String,
    pub fk_apartment: u16,
}

impl ModelTemplate<ApartmentDescription> for ApartmentDescription {
    fn update_values_object(conn: Connection, insert_object: Vec<ApartmentDescription>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare(
            "UPDATE ApartmentDescription SET \
            description=:u_description, \
            info=:u_info, \
            fk_apartment=:u_fk_apartment \
            WHERE id=:u_id") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "u_description"=>&obj.description,
                            "u_info"=>&obj.info,
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
                match value.prepare("DELETE FROM ApartmentDescription WHERE id=:u_id") {
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
    fn insert_values_object(conn: Connection, insert_object: Vec<ApartmentDescription>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare("INSERT INTO ApartmentDescription (id,description, info, fk_apartment)  VALUES (:u_id,:u_description,:u_info, :u_fk_apartment)") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "u_description"=>&obj.description,
                            "u_info"=>&obj.info,
                            "u_fk_apartment"=>obj.fk_apartment,
                        }
                    ).unwrap();
                }
            }
            Err(e) => println!("{}", e.to_string())
        };
        true
    }
    fn get_single_object(conn: Connection, item_id: u8) -> Option<ApartmentDescription> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM ApartmentDescription WHERE id=:item_id", params! {"item_id"=>item_id});
        Self::query_to_object(result)
    }
    fn get_multi_object(conn: Connection) -> Option<Vec<ApartmentDescription>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM ApartmentDescription", ());
        Self::query_list_to_object(result)
    }
    fn convert(item: Row) -> Option<ApartmentDescription> {
        match mysql::from_row_opt(item) {
            Ok(value) => {
                let q_obj: (u16, String, String, u16) = value;
                Some(Self { ID: q_obj.0, description: q_obj.1, info: q_obj.2, fk_apartment: q_obj.3 })
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
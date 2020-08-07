use mysql::Row;

use model::model_template::ModelTemplate;
use service::db_connector::Connection;
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    pub ID: u16,
    pub titleName: String,
    pub description: Option<String>,
    pub kachelType: u16,
    pub modalType: u16,
    pub tileSizeType: u16,
}

impl ModelTemplate<Tile> for Tile {
    fn update_values_object(conn: Connection, insert_object: Vec<Tile>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare(
            "UPDATE Tile SET \
             titleName=:title_name, \
             description=:desc, \
             kachelType=:kachel_type,\
             modalType=:modal_type, \
             tileSizeType=:tile_size \
             WHERE id=:u_id") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "title_name"=>&obj.titleName,
                            "desc"=>&obj.description,
                            "kachel_type"=>obj.kachelType,
                            "modal_type"=>obj.modalType,
                            "tile_size"=>obj.tileSizeType}
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
                match value.prepare("DELETE FROM Tile WHERE id=:u_id") {
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
    fn insert_values_object(conn: Connection, insert_object: Vec<Tile>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        println!("Insert object: ");
        match connection.prepare("INSERT INTO Tile (id,titleName,description,kachelType,modalType,tileSizeType) VALUES (:u_id,:title_name,:desc,:kachel_type,:modal_type,:tile_size)") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "title_name"=>&obj.titleName,
                            "desc"=>&obj.description,
                            "kachel_type"=>obj.kachelType,
                            "modal_type"=>obj.modalType,
                            "tile_size"=>obj.tileSizeType}
                    ).unwrap();
                }
            }
            Err(e) => println!("{}", e.to_string())
        };
        true
    }
    fn get_single_object(conn: Connection, item_id: u8) -> Option<Tile> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM Tile WHERE id=:item_id", params! {"item_id"=>item_id});
        Self::query_to_object(result)
    }
    fn get_multi_object(conn: Connection) -> Option<Vec<Tile>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM Tile", ());
        Self::query_list_to_object(result)
    }
    fn convert(item: Row) -> Option<Tile> {
        match mysql::from_row_opt(item) {
            Ok(value) => {
                let q_obj: (u16, String, Option<String>, u16, u16, u16) = value;
                Some(Self { ID: q_obj.0, titleName: q_obj.1, description: q_obj.2, kachelType: q_obj.3, modalType: q_obj.4, tileSizeType: q_obj.5 })
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
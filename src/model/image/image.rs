use std::io::Read;
use mysql;
use mysql::Row;
use rocket::Data;
use model::image::images_template::ImagesTemplate;
use service::db_connector::Connection;
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    pub ID: u16,
    pub image: Vec<u8>,
    pub description: Option<String>,
    pub fk_apartment: Option<u16>,
    pub fk_info: Option<u16>,
    pub fk_tile: Option<u16>,
}

impl ImagesTemplate<Image> for Image {
    fn update_values_object(conn: Connection, insert_object: Vec<Image>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
                //    image=:u_image, \
        match connection.prepare("UPDATE Image SET \
           description=:u_description, \
            fk_apartment=:u_fk_apartment, \
            fk_info=:u_fk_info, \
            fk_tile=:u_fk_tile \
            WHERE id=:u_id") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            // "u_image"=>&obj.image,
                            "u_description"=>&obj.description,
                            "u_fk_apartment"=>obj.fk_apartment,
                            "u_fk_info"=>obj.fk_info,
                            "u_fk_tile"=>obj.fk_tile,
                        }
                    ).unwrap();
                }
            }
            Err(e) => println!("{}", e.to_string())
        };
        true
    }
    fn delete_values_object(conn: Connection, object_id: u16) -> bool {
        match conn.get_connection() {
            Some(mut value) => 
                match value.prepare("DELETE FROM Image WHERE id=:u_id") {
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
    fn insert_values_object(conn: Connection, insert_object: Vec<Image>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare("INSERT INTO Image (id,image, description, fk_apartment, fk_info,fk_tile)  VALUES (:u_id,:u_image, :u_description, :u_fk_apartment, :u_fk_info,:u_fk_tile)") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "u_image"=>&obj.image,
                            "u_description"=>&obj.description,
                            "u_fk_apartment"=>obj.fk_apartment,
                            "u_fk_info"=>obj.fk_info,
                            "u_fk_tile"=>obj.fk_tile,
                        }
                    ).unwrap();
                }
            }
            Err(e) => println!("{}", e.to_string())
        };
        true
    }
    fn get_single_object(conn: Connection, id: u8) -> Option<Image> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        }
        let result = connection.prep_exec("SELECT * FROM Image WHERE id=:i_id", params! {"i_id"=>id});
        Self::query_to_object(result)
    }
    fn get_single_object_by_desc(conn: Connection, item_id: u8) -> Option<Image> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        }
        let result = connection.prep_exec("SELECT * FROM Image WHERE id=:i_id", params! {"i_id"=>item_id});
        Self::query_to_object(result)
    }
    fn get_multi_object(conn: Connection) -> Option<Vec<Image>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        }
        let result = connection.prep_exec("SELECT * FROM Image", ());
        Self::query_list_to_object(result)
    }
    fn convert(item: Row) -> Option<Image> {
        match mysql::from_row_opt(item) {
            Ok(value) => {
                let (q_id, q_image, q_desc, q_fk_apartment, q_fk_info, q_fk_tile) = value;
                Some(Self { ID: q_id, image: q_image, description: q_desc, fk_apartment: q_fk_apartment, fk_info: q_fk_info, fk_tile: q_fk_tile })
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
    fn load_all_image_desc(conn: Connection) -> Option<Vec<String>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT description FROM Image", ());
        match result {
            Ok(item) => {
                Some(Connection::row_to_vec::<String>(item))
            }
            Err(e) => {
                println!("{}", e.to_string());
                None
            }
        }
    }
    fn load_all_image_id(conn: Connection) -> Option<Vec<u16>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT id FROM Image", ());
        match result {
            Ok(item) => {
                Some(Connection::row_to_vec::<u16>(item))
            }
            Err(e) => {
                println!("{}", e.to_string());
                None
            }
        }
    }

    fn load_image_binary(conn: Connection, id: u8) -> Option<Vec<u8>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        }
        let result = connection.prep_exec("SELECT image FROM Image WHERE id=:u_id", params! {"u_id"=>id});
        match result {
            Ok(query) => {
                let found = Connection::row_to_vec::<Vec<u8>>(query);
                match found.get(0) {
                    Some(value) => Some(value.to_vec()),
                    None => None
                }
            }
            Err(e) => {
                println!("{}", e.to_string());
                None
            }
        }
    }
    fn upload_image(conn: Connection, data: Data,id: u8) -> bool {
        let mut buf : Vec<u8> = Vec::new();
        let copy_conn = conn.clone();
        if let Err(_) = data.open().read_to_end(&mut buf) {
            return false;
        }
        let found_object  = match Self::get_single_object_by_desc(conn, id) {
            Some(mut value) => {
                value.image = buf;
                vec![value]
            },
            None => return false
        };

        Self::update_image(copy_conn, found_object)
    }
    fn update_image(conn: Connection, insert_object: Vec<Image>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare(
            "UPDATE Image SET \
            image =:u_image, \
            description=:u_description, \
            fk_apartment=:u_fk_apartment, \
            fk_info=:u_fk_info,\
            fk_tile=:u_fk_tile \
            WHERE id=:u_id") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "u_image"=>&obj.image,
                            "u_description"=>&obj.description,
                            "u_fk_apartment"=>obj.fk_apartment,
                            "u_fk_info"=>obj.fk_info,
                            "u_fk_tile"=>obj.fk_tile,
                        }
                    ).unwrap();
                }
            }
            Err(e) => println!("{}", e.to_string())
        };
        true
    }
}

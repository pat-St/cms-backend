use mysql::Row;
use crate::{service,model};
use model::model_template::ModelTemplate;
use service::db_connector::Connection;
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoText {
    pub ID: u16,
    pub headerText: Option<String>,
    pub contentText: Option<String>,
    pub link: Option<String>
}

impl ModelTemplate<InfoText> for InfoText {
    fn update_values_object(conn: Connection, insert_object: Vec<InfoText>) -> bool {
        let mut connection = match conn.get_connection() {
            Some(value) => value,
            None => return false
        };
        match connection.prepare(
            "UPDATE InfoText SET \
            headerText=:header_text, \
            contentText=:content_text, \
            link=:u_link \
            WHERE id=:item_id") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "item_id"=>obj.ID,
                            "header_text"=>&obj.headerText,
                            "content_text"=>&obj.contentText,
                            "u_link"=>&obj.link}
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
                match value.prepare("DELETE FROM InfoText WHERE id=:item_id") {
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
    fn insert_values_object(conn: Connection, insert_object: Vec<InfoText>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare(
            "INSERT INTO InfoText (id,headerText,contentText,link) VALUES (:u_id,:header_text,:content_text,:u_link)") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "header_text"=>&obj.headerText,
                            "content_text"=>&obj.contentText,
                            "u_link"=>&obj.link}
                    ).unwrap();
                }
            }
            Err(e) => println!("{}", e.to_string())
        };
        true
    }
    fn get_single_object(conn: Connection, item_id: u8) -> Option<InfoText> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM InfoText WHERE id=:item_id", params! {"item_id"=>item_id});
        Self::query_to_object(result)
    }
    fn get_multi_object(conn: Connection) -> Option<Vec<InfoText>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM InfoText", ());
        Self::query_list_to_object(result)
    }
    fn convert(item: Row) -> Option<InfoText> {
        match mysql::from_row_opt(item) {
            Ok(value) => {
                let q_obj: (u16, Option<String>, Option<String>, Option<String>) = value;
                Some(Self { ID: q_obj.0, headerText: q_obj.1, contentText: q_obj.2, link: q_obj.3 })
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
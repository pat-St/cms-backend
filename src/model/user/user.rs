use mysql::Row;
use model::model_template::ModelTemplate;
use service::db_connector::Connection;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize,Clone)]
pub struct WUser {
    pub ID: u16,
    pub name: String,
    pub pw: String,
    pub salt: String,
    pub token: String,
    pub mail: String,
}

impl ModelTemplate<WUser> for WUser {
    fn update_values_object(conn: Connection, insert_object: Vec<WUser>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        match connection.prepare("UPDATE WUser SET \
             name=:u_name, \
             pw=:u_pw, \
             salt=:u_salt, \
             token=:u_token, \
             mail=:u_mail \
             WHERE id=:u_id") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "u_name"=>&obj.name,
                            "u_pw"=>&obj.pw,
                            "u_salt"=>&obj.salt,
                            "u_token"=>&obj.token,
                            "u_mail"=>&obj.mail
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
                match value.prepare("DELETE FROM WUser WHERE id=:u_id") {
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
    fn insert_values_object(conn: Connection, insert_object: Vec<WUser>) -> bool {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return false
        };
        println!("Insert object: ");
        match connection.prepare("INSERT INTO WUser (id, name, pw, salt, token, mail) VALUES (:u_id, :u_name, :u_pw, :u_salt, :u_token, :u_mail)") {
            Ok(mut stmt) => {
                for obj in insert_object.iter() {
                    stmt.execute(
                        params! {
                            "u_id"=>obj.ID,
                            "u_name"=>&obj.name,
                            "u_pw"=>&obj.pw,
                            "u_salt"=>&obj.salt,
                            "u_token"=>&obj.token,
                            "u_mail"=>&obj.mail
                        }
                    ).unwrap();
                }
            }
            Err(e) => println!("{}", e.to_string())
        };
        true
    }
    fn get_single_object(conn: Connection, item_id: u8) -> Option<WUser> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM WUser WHERE id=:item_id", params! {"item_id"=>item_id});
        Self::query_to_object(result)
    }
    fn get_multi_object(conn: Connection) -> Option<Vec<WUser>> {
        let mut connection;
        match conn.get_connection() {
            Some(value) => connection = value,
            None => return None
        };
        let result = connection.prep_exec("SELECT * FROM WUser", ());
        Self::query_list_to_object(result)
    }
    fn convert(item: Row) -> Option<WUser> {
        match mysql::from_row_opt(item) {
            Ok(value) => {
                let q_obj: (u16, String, String, String, String, String) = value;
                Some(Self { ID: q_obj.0, name: q_obj.1, pw: q_obj.2, salt: q_obj.3, token: q_obj.4, mail: q_obj.5 })
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
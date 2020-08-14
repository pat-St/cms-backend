use crate::{service,model};
use service::db_connector::Connection;
use model::user::user_util;
use model::user::user::WUser;
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize,Clone)]
pub struct Register {
	pub name: String,
	pub pw: String,
	pub mail: String,
}

impl Register {
	pub fn add_new_user(conn: Connection, insert_object: Register) -> bool {
		let mut connection = match conn.get_connection() {
			Some(value) => value,
			None => return false
		};
		if !WUser::check_user_non_exists(conn, &*insert_object.name) {
			return false
		}
		let salt_component = user_util::generate_key(64);
		let (pw, salt) = user_util::create_password(&*insert_object.name, &*insert_object.pw, &*salt_component);
		let token = user_util::generate_key(64);
		let result = match connection.prepare("INSERT INTO WUser (name, pw, salt, token, mail) VALUES (:u_name, :u_pw, :u_salt, :u_token, :u_mail)") {
			Ok(mut stmt) =>
				match stmt.execute(params! {"u_name"=>&insert_object.name,"u_pw"=>&pw,"u_salt"=>&salt,"u_token"=>&token,"u_mail"=>&insert_object.mail}) {
					Ok(_) => true,
					Err(e) => {
						println!("{}", e.to_string());
						false
					}
				}
			Err(e) => {
				println!("{}", e.to_string());
				return false
			}
		};
		result
	}
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize,Clone)]
pub struct ChangePW {
	pub name: String,
	pub old_pw: String,
	pub new_pw: String
}

impl ChangePW {
	pub fn change_pw(conn: Connection, input: ChangePW) -> bool {
		let mut connection = match conn.clone().get_connection() {
			Some(value) => value,
			None => return false
		};
		if WUser::check_user_non_exists(conn.clone(), &*input.name) {
			return false
		}
		let curr_user =  match WUser::get_user_by_name(conn,&*input.name) {
			Some(value) => value,
			None => return false
		};
		if !user_util::comp_passwd(&*curr_user.pw, &*curr_user.salt, &*input.old_pw) {
			return false
		}
		let salt_component = user_util::generate_key(64);
		let (pw, salt) = user_util::create_password(&*input.name, &*input.new_pw, &*salt_component);
		let token = user_util::generate_key(64);
		match connection.prepare("UPDATE WUser SET pw=:u_pw, salt=:u_salt, token=:u_token WHERE name=:uname") {
			Ok(mut stmt) => {
				stmt.execute(
					params! {
                            "u_name"=>&input.name,
                            "u_pw"=>&pw,
                            "u_salt"=>&salt,
                            "u_token"=>&token
                        }
				).unwrap();
			}
			Err(e) => println!("{}", e.to_string())
		};
		true
	}
}

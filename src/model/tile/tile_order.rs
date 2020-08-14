use mysql::Row;
use crate::{service,model};
use model::model_template::ModelTemplate;
use service::db_connector::Connection;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileOrder {
	pub ID: u16,
	pub seqNum: u16,
	pub fk_tile: u16,
}
impl ModelTemplate<TileOrder> for TileOrder {
	fn insert_values_object(conn: Connection, insert_object: Vec<TileOrder>) -> bool {
		let mut connection = match conn.get_connection() {
			Some(value) => value,
			None => return false
		};
		match connection.prepare("INSERT INTO TileOrder (id,seqNum,fk_tile) VALUES (:item_id,:u_seq_num,:u_fk_tile)") {
			Ok(mut stmt) => {
				for obj in insert_object.iter() {
					stmt.execute(
						params! {
	                        "item_id"=>obj.ID,
	                        "u_seq_num"=>obj.seqNum,
	                        "u_fk_tile"=>obj.fk_tile}
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

	fn update_values_object(conn: Connection, insert_object: Vec<TileOrder>) -> bool {
		let mut connection = match conn.get_connection() {
			Some(value) => value,
			None => return false
		};
		println!("{:#?}",insert_object);
		match connection.prepare(
			"UPDATE TileOrder SET \
			seqNum=:u_seq_num, \
             fk_tile=:u_fk_tile \
             WHERE id=:item_id") {
			Ok(mut stmt) => {
				for obj in insert_object.iter() {
					stmt.execute(
						params! {
                           "item_id"=>obj.ID,
                           "u_seq_num"=>obj.seqNum,
                           "u_fk_tile"=>obj.fk_tile}
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
				match value.prepare("DELETE FROM TileOrder WHERE id=:u_id") {
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

	fn get_single_object(conn: Connection, item_id: u8) -> Option<TileOrder> {
		let mut connection = match conn.get_connection() {
			Some(value) => value,
			None => return None
		};
		let result = connection.prep_exec("SELECT * FROM TileOrder WHERE id=:item_id", params! {"item_id"=>item_id});
		Self::query_to_object(result)
	}

	fn get_multi_object(conn: Connection) -> Option<Vec<TileOrder>> {
		let mut connection = match conn.get_connection() {
			Some(value) => value,
			None => return None
		};
		let result = connection.prep_exec("SELECT * FROM TileOrder", ());
		Self::query_list_to_object(result)
	}

	fn convert(item: Row) -> Option<TileOrder> {
		match mysql::from_row_opt(item) {
			Ok(value) => {
				let q_obj: (u16, u16, u16) = value;
				Some(Self { ID: q_obj.0, seqNum: q_obj.1, fk_tile: q_obj.2})
			}
			Err(e) => {
				println!("{}", e);
				None
			}
		}
	}
}
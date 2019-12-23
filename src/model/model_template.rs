use mysql::error::Result as MyResult;
use mysql::{QueryResult, Row};
use service::db_connector::Connection;

pub(crate) trait ModelTemplate<T> {
    fn insert_values_object(conn: Connection, insert_object: Vec<T>) -> bool;
    fn update_values_object(conn: Connection, insert_object: Vec<T>) -> bool;
    fn delete_values_object(conn: Connection, object_id: u16) -> bool;
    fn get_single_object(conn: Connection, item_id: u8) -> Option<T>;
    fn get_multi_object(conn: Connection) -> Option<Vec<T>>;

    fn query_to_object(result: MyResult<QueryResult<'_>>) -> Option<T> {
        match result {
            Ok(item) => {
                let mut row = item.into_iter();
                match row.next() {
                    Some(row) => Self::convert(row.unwrap()),
                    None => None
                }
            },
            Err(e) => {
                println!("{}", e.to_string());
                None
            }
        }
    }
    fn query_list_to_object(result: MyResult<QueryResult<'_>>) -> Option<Vec<T>> {
        match result {
            Ok(item) => {
                item.map(|row| row.unwrap()).map(|row| Self::convert(row.to_owned())).collect()
            },
            Err(e) => {
                println!("{}", e.to_string());
                None
            }
        }
    }
    fn convert(item: Row) -> Option<T>;
}
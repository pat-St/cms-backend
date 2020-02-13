use model::user::user::WUser;
use sha2::{Digest, Sha512};
use rand::prelude::*;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Login {
    pub name: String,
    pub pw: String,
}

#[derive(FromForm,Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Token { pub token: String }

impl Login {
    pub fn logout_user(username: String, users: Vec<WUser>) -> Option<WUser> {
        for mut i in users {
            if i.name == username {
                i.token = "".to_owned();
                return Some(i)
            }
        }
        None
    }
    pub fn is_login_correct(self, users: Vec<WUser>) -> Option<WUser> {
        for mut i in users {
            if i.name.eq(&self.name) && i.pw.eq(&self.pw) {
                i.token = Login::generate_key();
                return Some(i)
            }
        }
        None
    }
    fn generate_key() -> String {
        let mut rng = rand::thread_rng();
        let mut nms: Vec<u8> = vec![];
        for _ in 1..64 {
            nms.push(rng.gen::<u8>())
        }
        Login::hash(&nms)
    }  

    /**
     * convert the input as utf-8 bytes into hex-characters with the size of 128 characters
    */
    fn byte_to_hex(vec_res: Vec<u8>) -> String {
        const HEX: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
        let mut result_hash = String::with_capacity(vec_res.len() * 2);
        for character in vec_res.iter() {
            result_hash.push(HEX[*character as usize & 0xF0 >> 4]);
            result_hash.push(HEX[*character as usize & 0x0F]);
        }
        result_hash
    }
    /**
     * Hash the input as Sha 512 with size of 64 characters
    */
    fn hash(input: &[u8]) -> String {
        let mut tmp = Sha512::default();
        tmp.input(input);
        let mut vec_res = vec![0u8; 64];
        vec_res.copy_from_slice(&tmp.result());
        Login::byte_to_hex(vec_res)
    }
}

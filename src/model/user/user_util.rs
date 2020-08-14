use ring::{digest, pbkdf2};
use rand::prelude::*;
use sha2::{Digest, Sha512};
use std::num::NonZeroU32;
use std::str;
static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const CREDENTIAL_LEN: usize = digest::SHA512_256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

pub fn generate_key(length: u8) -> String {
	let mut rng = rand::thread_rng();
	let mut nms: Vec<u8> = vec![];
	for _ in 1..length {
		nms.push(rng.gen::<u8>())
	}
	hash(&nms)
}

/**
 * Hash the input as Sha 512 with size of 64 characters
*/
fn hash(input: &[u8]) -> String {
	let mut tmp = Sha512::default();
	tmp.input(input);
	let mut vec_res = vec![0u8; input.len()+1];
	vec_res.copy_from_slice(&tmp.result());
	byte_to_hex(vec_res)
}

/**
 * convert the input as utf-8 bytes into hex-characters with the size of 128 characters
*/
fn byte_to_hex(vec_res: Vec<u8>) -> String {
	const HEX: [char; 16] = [
		'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'
	];
	let mut result_hash = String::with_capacity(vec_res.len() * 2);
	for character in vec_res.iter() {
		result_hash.push(HEX[*character as usize & 0xF0 >> 4]);
		result_hash.push(HEX[*character as usize & 0x0F]);
	}
	result_hash
}

pub fn create_password(username: &str, pw: &str, salt_component: &str) -> (String,String) {
	let pbkdf2_it = NonZeroU32::new(100_000).unwrap();
	let salt = create_salt(&*username, &*salt_component);
	let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
	pbkdf2::derive(PBKDF2_ALG, pbkdf2_it, &salt, pw.as_bytes(), &mut to_store);
	let pw_as_string: String =byte_to_hex(to_store.to_vec());
	let salt_as_string: String = str::from_utf8(&*salt).unwrap().to_string();
	( pw_as_string, salt_as_string )
}

fn create_salt(username: &str, salt_component: &str) -> Vec<u8> {
	let mut salt: Vec<u8> = Vec::with_capacity(32);
	salt.extend(salt_component.as_bytes());
	salt.extend(username.as_bytes());
	salt
}

pub fn comp_passwd(db_passwd: &str, db_salt: &str, attempt_passwd: &str) -> bool {
	let pbkdf2_it = NonZeroU32::new(100_000).unwrap();
	let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
	pbkdf2::derive(PBKDF2_ALG, pbkdf2_it, db_salt.as_ref(), attempt_passwd.as_bytes(),&mut to_store);
	let pw_as_string: String = byte_to_hex(to_store.to_vec());
	pw_as_string.eq(db_passwd)
}



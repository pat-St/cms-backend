use crate::{model};
use model::user::user::WUser;
use model::user::user_util;
use crate::model::user::user_util::comp_passwd;


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
            if i.name.to_uppercase() == username.to_uppercase() {
                i.token = "".to_owned();
                return Some(i)
            }
        }
        None
    }
    pub fn is_login_correct(self, user: &WUser) -> Option<String> {
        let salt = &user.salt;
        if user.name.to_uppercase().eq(&self.name.to_uppercase()) && comp_passwd(&user.pw,salt,&self.pw) {
            return Some(user_util::generate_key(64));
        }
        None
    }
}

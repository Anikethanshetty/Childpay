use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::users::{Role, User};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct FilterUserDto {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phonenumber : Option<String>,
    pub user_role : Option<Role>
} 

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {     
        Self { 
            id:Some(user.id) , 
            username: Some(user.username.clone()), 
            email: Some(user.email.clone()),
            phonenumber : Some(user.phonenumber.clone()),
            user_role : Some(user.user_role.clone())
        }
    }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct UserData {
    pub user : FilterUserDto ,
    pub token : String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct  UserResponse {
    pub status : &'static str,
    pub data : UserData
} 

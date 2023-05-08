use poem_openapi::{Object};

// payload
#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserLogin{
  pub username:Option<String>,
  pub password:Option<String>
}



// response
#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserRes{
  pub token:String,
  pub username:String
}

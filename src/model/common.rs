use poem_openapi::{types::{ParseFromJSON,ToJSON},Object};


#[derive(Debug,Object)]
pub struct ResponseObject<T> where T:ParseFromJSON + ToJSON + Send + Sync{
  pub code:i32,
  pub data:Option<T>,
  pub msg:String
}


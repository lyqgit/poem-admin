use poem::{Error};
use poem_openapi::{ApiResponse,types::{ParseFromJSON,ToJSON},payload::Json};

use crate::model::common::ResponseObject;

impl<T: ParseFromJSON + ToJSON + Send + Sync> ResponseObject<T> {
  pub fn ok(data: Option<T>) -> Self {
      Self {
          code: 0,
          msg: "访问成功".to_string(),
          data,
      }
  }

  pub fn custom_code(code:i32,msg:String) -> Self {
      Self {
          code,
          msg,
          data: None,
      }
  }

  pub fn err(err:String)->Self{
      Self {
        code: 500,
        msg: err,
        data: None,
    }
  }
}

#[derive(ApiResponse)]
#[oai(bad_request_handler = "custom_bad_request_handler")]
pub enum CustomResponse<T: ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<ResponseObject<T>>),
}

fn custom_bad_request_handler<T: ParseFromJSON + ToJSON + Send + Sync>(err: Error) -> CustomResponse<T> {
  CustomResponse::Ok(Json(ResponseObject {
        code: 500,
        msg: err.to_string(),
        data: None,
    }))
}

pub fn res_ok<T: ParseFromJSON + ToJSON + Send + Sync>(data:Option<T>)->CustomResponse<T>{
  CustomResponse::Ok(Json(ResponseObject::ok(data)))
}
pub fn res_err<T: ParseFromJSON + ToJSON + Send + Sync>(msg:String)->CustomResponse<T>{
  CustomResponse::Ok(Json(ResponseObject::err(msg)))
}

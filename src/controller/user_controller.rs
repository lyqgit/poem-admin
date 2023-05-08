use poem_openapi::{OpenApi,payload::Json};
use crate::utils::res::{CustomResponse,res_ok,res_err};
use crate::model::user_model::{UserLogin,UserRes};
use crate::utils::token::create_token;

pub struct UserController;

#[OpenApi]
impl UserController {
    #[oai(path="/user/login",method="post")]
    async fn login(&self,user_req:Json<UserLogin>)->CustomResponse<UserRes>{
        if let Json(UserLogin{username:Some(username),password:Some(password)}) = user_req{
            if username.is_empty() || password.is_empty(){
                res_err("用户账号或密码必须填写".to_string())
            }else{
                let token_str = create_token(1,username.to_string()).unwrap();
                res_ok(Some(UserRes{token:token_str,username:username.to_string()}))
            }
        }else{
            res_err("用户账号或密码必须填写".to_string())
        }
    }
}

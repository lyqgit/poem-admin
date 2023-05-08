use std::panic::catch_unwind;

use poem_openapi::{OpenApi,param::Query,payload::Json, Object};
use poem::{Endpoint,EndpointExt,Response, async_trait,Request,Middleware,Result, IntoResponse,http::StatusCode,error};
use crate::utils::token;
use crate::utils::res::{CustomResponse,res_ok,res_err};
use crate::model::common::ResponseObject;
use serde_json::{Value};

pub struct Hello;

fn set_header(ep: impl Endpoint) -> impl Endpoint {
  println!("中间件");
  ep.with(Log)
}

struct Log;

impl<E: Endpoint> Middleware<E> for Log {
    type Output = LogImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        LogImpl(ep)
    }
}

#[async_trait]
impl<E: Endpoint> Endpoint for LogImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        println!("request: {}", req.uri().path());
        
        match req.header("Authorization"){
            Some(t)=>{
                match token::verify_token(t){
                    Ok(_mycliam)=>{
                        let res = self.0.call(req).await;

                        match res {
                            Ok(resp) => {
                                let resp = resp.into_response();
                                println!("response: {}", resp.status());
                                Ok(resp)
                            }
                            Err(err) => {
                                println!("error: {}", err);
                                Err(err)
                            }
                        }
                    },
                    Err(err)=>{
                        Err(CustomResponse::Ok(Json(ResponseObject{code:500,data:Some(Value::Null),msg:err.to_string()})).into())
                    },
                }
            },
            _=>{
                Err(error::Error::from_string("测试错误",StatusCode::BAD_REQUEST).into())
            }
        }
            
        
        
    }
}

struct LogImpl<E>(E);


#[derive(Debug,Object)]
struct AT {
    #[oai(validator(max_length = 64))]
    tt:String
}

#[OpenApi]
impl Hello {
  #[oai(path = "/hello", method = "get")]
  async fn index(&self, _name: Query<Option<String>>)->CustomResponse<AT>{
    println!("kzq");
    match catch_unwind(||{
        panic!("123");
    }){
        Ok(_)=>{
            println!("没有异常")
        },
        Err(err)=>{
            println!("异常{:#?}",err)
        }
    };
    res_ok(Some(AT{tt:"123".to_string()}))
    // res_err("方式出错误".to_string())
  }
}




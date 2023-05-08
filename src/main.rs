use poem::{
    listener::TcpListener,
    Route,Server
};
use poem_openapi::{OpenApiService};
use rbatis::Rbatis;
use once_cell::sync::OnceCell;
use rbdc_mysql::driver::MysqlDriver;

mod controller;
mod model;
mod utils;

use controller::hello_controller::Hello;
use controller::user_controller::UserController;


// 全局变量
pub static GLOBAL_DB:OnceCell<Rbatis> = OnceCell::new();

#[macro_export]
macro_rules! pool {
    () => {
        &mut GLOBAL_DB.get().unwrap()
    };
}

#[tokio::main]
async fn main() ->Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    tracing_subscriber::fmt::init();


    let rb = Rbatis::new();
    rb.link(MysqlDriver {}, "mysql://root:123456@localhost/ry-vue").await.unwrap();
    GLOBAL_DB.set(rb).expect("数据库链接失败");

    let api_service =
        OpenApiService::new((Hello,UserController), "poem接口文档", "1.0").server("/api");
    let ui = api_service.rapidoc();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(Route::new().nest("/api", api_service).nest("/doc/api", ui))
        .await
}

use axum::{
    routing::post,
    http::StatusCode,
    Router,
    extract::Json,   //消耗主体并解析成JSON
};
use serde::{Deserialize,Serialize}; //将Json数据反序列化成Rust结构体

#[derive(Deserialize,Serialize,Debug)]
struct CreateUser {
    username: String,   // 接收创建用户的请求名
}

//参数：从请求中提取JSON数据反序列化成结构体
async fn create_user(Json(payload):Json<CreateUser>) ->(StatusCode,Json<CreateUser>){
    //响应内容是Json格式
    (StatusCode::CREATED,Json(payload))
}

#[tokio::main]
async fn main(){
    let app =Router::new().
            route("/users",post(create_user));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


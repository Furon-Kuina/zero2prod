use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

// subscribeが呼ばれる前に、actix-webがその各引数のfrom_requestを実行する
// from_requestはリクエストのbodyをdeserializeしてFormDataに変換する．
// これは、serde_urlencodedとFormDataのDeserializeを使う．この実装は#[derive()serde::Deserialize]が勝手にやってくれる
// これが失敗すると400が、成功するとsubscribeまで呼ばれて200が返る
// 正直まだピンと来てない部分がある
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

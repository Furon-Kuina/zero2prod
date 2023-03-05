use actix_web::{web, HttpResponse};
use sqlx::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData {
    #[allow(dead_code)]
    email: String,
    #[allow(dead_code)]
    name: String,
}

// subscribeが呼ばれる前に、actix-webがその各引数のfrom_requestを実行する
// from_requestはリクエストのbodyをdeserializeしてFormDataに変換する．
// これは、serde_urlencodedとFormDataのDeserializeを使う．この実装は#[derive()serde::Deserialize]が勝手にやってくれる
// これが失敗すると400が、成功するとsubscribeまで呼ばれて200が返る
// 正直まだピンと来てない部分がある
pub async fn subscribe(
    _form: web::Form<FormData>,
    _connection: web::Data<PgConnection>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

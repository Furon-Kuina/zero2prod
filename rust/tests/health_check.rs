use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // address = "http://127/0.0.1:[random port number]"
    let address = spawn_app();
    let client = reqwest::Client::new();
    // GETを送る
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // listenするTCPソケットを作る
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = rust::run(listener).expect("Failed to bind address");
    // tokio::spawnはバックグラウンドにasyncを生成しながら処理を続行できる
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

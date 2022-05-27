use actix_web::http::header;
use actix_web::web::Json;
use std::collections::HashMap;
use std::net::TcpListener;

fn spawn_app() -> String {
    //port 0 will trigger an OS scan for an available port which will then be bound to the application.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = email_newsletter::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_ok() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let mut body = HashMap::new();
    body.insert("name", "ahmed");
    body.insert("email", "ahmed@gmail.com");

    let response = client
        .post(&format!("{}/subscriptions", &address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        200,
        response.status().as_u16(),
        &format!("body = {:?}", response)
    );
}

#[tokio::test]
async fn subscribe_bad_request() {
    let address = spawn_app();

    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=ahmed", "missing the email"),
        ("email=ahmed%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, err_msg) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            err_msg
        );
    }
}

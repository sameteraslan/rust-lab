use actix_web::{test, web, App};
use actix_web_simple_http_server::{greet, index};

#[actix_web::test]
async fn test_greet() {
    let app = test::init_service(App::new().service(greet)).await;

    let req = test::TestRequest::get().uri("/hello/testuser").to_request();
    let resp = test::call_and_read_body(&app, req).await;
    assert_eq!(resp, "Hello testuser!");

    let req = test::TestRequest::get().uri("/hello/rust").to_request();
    let resp = test::call_and_read_body(&app, req).await;
    assert_eq!(resp, "Hello rust!");

    let req = test::TestRequest::get().uri("/hello/%40dmin").to_request();
    let resp = test::call_and_read_body(&app, req).await;
    assert_eq!(resp, "Hello @dmin!");
}

#[actix_web::test]
async fn test_index() {
    let app = test::init_service(App::new().service(web::resource("/").to(index))).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let _resp = test::call_and_read_body(&app, req).await;
}

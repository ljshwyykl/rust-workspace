// mod node_integration {
//     use actix_web::{test, web, App, HttpRequest, HttpResponse};
//     use actix_web::index;

//     #[actix_web::test]
//     async fn test_index_ok() {
//         let mut server = test::init_service(App::new().service(web::resource("/").to(index))).await;

//         let request = test::TestRequest::get().uri("/").to_request();
//         let response = server.call(request).await.unwrap();
//         assert!(response.status().is_success());
//     }
// }

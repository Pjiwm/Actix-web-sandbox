use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Scope};
use std::sync::Mutex;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        let user_scope = web::scope("/user").service(create_user).service(show_user);
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: "Actix web".to_owned(),
            }))
            .app_data(counter.clone())
            .service(mutex_index)
            .service(index)
            .service(hello)
            .service(echo)
            .service(user_scope)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/state")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

#[get("/mutex-state")]
async fn mutex_index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Hello {counter}")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// for user scope
#[get("/show")]
async fn show_user() -> impl Responder {
    HttpResponse::Ok().body("showing some user")
}

#[post("/create")]
async fn create_user(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("created: {req_body}"))
}

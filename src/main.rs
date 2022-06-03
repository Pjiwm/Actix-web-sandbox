use actix_web::{get, post, web, App, Either, Error, HttpResponse, HttpServer, Responder};
use models::Post;
use std::sync::Mutex;
mod models;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        let user_scope = web::scope("/user").service(create_user).service(show_user).service(hello_user);

        let post_scope = web::scope("/post").service(create_post).service(hello_post).service(query_style);
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
            .service(post_scope)
            .route("/hey", web::get().to(manual_hello))
            .service(bad_req_index)
    })
    .workers(8)
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
// Trying out with JSOn
#[post("/create")]
async fn create_post(req_body: String) -> impl Responder {
    let body = &req_body.clone();
    let parts = body.split("\r\n\r\n").collect::<Vec<&str>>();
    models::Post {
        title: parts.get(0).unwrap().to_string(),
        body: parts.get(1).unwrap().to_owned().to_string(),
    }
}

type RegisterResult = Either<HttpResponse, Result<Post, Error>>;
#[post("/error_post")]
async fn bad_req_index(req_body: String) -> RegisterResult {
    let body = &req_body.clone();
    let parts = body.split("\r\n\r\n").collect::<Vec<&str>>();
    if parts.len() != 2 {
        Either::Left(HttpResponse::BadRequest().body("bad data"))
    } else {
        let model = models::Post::new(
            parts.get(0).unwrap().to_string(),
            parts.get(1).unwrap().to_string(),
        );
        Either::Right(Ok(model))
    }
}

#[get("/{user_id}/{user_name}")]
async fn hello_user(path: web::Path<(u32, String)>) -> impl Responder {
    let (user_id, user_name) = path.into_inner();
    HttpResponse::Ok().body(format!("Hello {}! id: {}", user_name, user_id))
}

#[get("/{title}/{body}")]
async fn hello_post(post: web::Path<Post>) -> impl Responder {
    let post = post.into_inner();
    HttpResponse::Ok().body(format!("Post title: {}! \n post content: {}", post.title, post.body))
}
// Trying it out with Query
#[get("/query-style")]
async fn query_style(query: web::Json<models::Post>) -> impl Responder {
    query
}
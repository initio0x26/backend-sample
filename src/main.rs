mod api;
use api::task::{get_eqmtid, get_location, get_road};
use actix_web::{
    error::ResponseError,
    get,
    put,
    post,
    HttpServer,
    App,
    web::Path,
    web::Json,
    web::Data,
    middleware,
    HttpResponse,
    HttpRequest,
    http::{header,StatusCode}
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "");
    env_logger::init();
    
    HttpServer::new(move || {
        let logger = middleware::Logger::default();
        let cors = actix_cors::Cors::permissive();
        App::new()
        .wrap(logger)
        .wrap(cors)
        .service(get_eqmtid)
        .service(get_location)
        .service(get_road)
    })
    .bind(("127.0.0.1",8085))?
    .run()
    .await
}

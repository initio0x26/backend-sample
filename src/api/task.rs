mod module_cli;
use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    HttpRequest,
    http::{header::ContentType,StatusCode}, Responder
};
use serde::{Serialize,Deserialize};
use derive_more::Display;
use module_cli::*;

use crate::api::module_cli::read_file;

#[derive(Deserialize,Serialize)]
pub struct  TaskEquipmentStat {
    eqmtid : String,
}

#[derive(Deserialize,Serialize)]
 pub struct TaskChangeStatus {
     eqmtid:String,
     stats:String,
    rscode:i64,
}
#[derive(Deserialize,Serialize)]
pub struct GetToken{
pub userid:String,
pub secret:String,
}

#[get("/equipmentdata")]
pub async fn get_eqmtid() -> impl Responder {
    let _eqdat = "eqmtdata.json".to_string();
    let task = read_file(_eqdat).unwrap();
    return actix_web::web::Json(task);
}

#[get("/location")]
pub async fn get_location() -> impl Responder {
    let _loc = "location.json".to_string();
    let task = read_file(_loc).unwrap();
    return actix_web::web::Json(task);
}


#[get("/road")]
pub async fn get_road() -> impl Responder {
    let _loc = "road.json".to_string();
    let task = read_file(_loc).unwrap();
    return actix_web::web::Json(task);
}

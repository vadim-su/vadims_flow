use schemars::JsonSchema;

use actix_web::{
    web::{Data, Json},
    Responder,
};
use apistos::web;
use apistos::{api_operation, ApiComponent};
use serde::Serialize;

use crate::resources::info::Info;

#[derive(Clone, Serialize, JsonSchema, ApiComponent)]
struct InfoResponse {
    version: String,
    rustc: String,
    build_date: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(Data::new(Info::default()))
        .service(web::resource("/info").route(web::get().to(get)));
}

#[api_operation(summary = "Get information about the application")]
pub async fn get(info: Data<Info>) -> impl Responder {
    return Json(InfoResponse {
        version: info.version.clone(),
        rustc: info.rustc.clone(),
        build_date: info.build_date.clone(),
    });
}

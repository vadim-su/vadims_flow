use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::resources::info::Info;

#[derive(Clone, Serialize, utoipa::ToSchema)]
struct InfoResponse {
    version: String,
    rustc: String,
    build_date: String,
}

struct Service {
    info: Info,
}
impl Service {
    pub fn new(info: Info) -> Arc<Self> {
        Arc::new(Self { info })
    }

    pub fn configurate(self: Arc<Self>, cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/info").route(web::get().to(move || {
            let self_clone = self.clone();
            async move { self_clone.get().await }
        })));
    }

    pub async fn get(&self) -> impl Responder {
        web::Json(InfoResponse {
            version: self.info.version.clone(),
            rustc: self.info.rustc.clone(),
            build_date: self.info.build_date.clone(),
        })
    }
}

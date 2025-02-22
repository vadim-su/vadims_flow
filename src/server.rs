use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::spec::Spec;
use apistos::ScalarConfig;
use std::error::Error;
use std::net::IpAddr;
use surrealdb::engine::any::Any;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

use crate::logger::{init_logging, log_app_info};
use crate::resources::info;
use crate::views;

#[derive(Clone)]
pub struct Server {
    pub db: Surreal<Db>,
}

impl Server {
    pub async fn serve(self, addr: IpAddr, port: u16) -> Result<(), Box<dyn Error>> {
        init_logging(log::LevelFilter::Info);
        log_app_info();

        let db = self.db.clone();

        HttpServer::new(move || {
            let db = db.clone();
            self.clone().make_app(db)
        })
        .bind((addr, port))?
        .run()
        .await?;
        Ok(())
    }

    fn make_app(
        self,
        db: Surreal<Db>,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Response = ServiceResponse<impl MessageBody>,
            Config = (),
            InitError = (),
            Error = actix_web::error::Error,
        >,
    > {
        let info = info::Info::default();
        let spec = Spec {
            info: apistos_models::info::Info {
                title: "Bredis API".to_string(),
                version: info.version.clone(),
                ..Default::default()
            },
            ..Default::default()
        };
        App::new()
            .app_data(web::Data::new(db))
            .document(spec)
            .wrap(Logger::default())
            .configure(|cfg| {
                views::info::configure(cfg);
                views::posts::configure(cfg);
            })
            .build_with(
                "/openapi.json",
                BuildConfig::default().with(ScalarConfig::new(&"/docs")),
            )
    }
}

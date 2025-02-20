use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::error::Error;
use utoipa_actix_web::AppExt;
use utoipa_scalar::{Scalar, Servable};

use crate::logger::{init_logging, log_app_info};

#[derive(Clone)]
pub struct Server {}

impl Server {
    pub async fn serve(self, addr: String, port: u16) -> Result<(), Box<dyn Error>> {
        init_logging(log::LevelFilter::Info);
        log_app_info();

        HttpServer::new(move || self.clone().make_app())
            .bind((addr, port))?
            .run()
            .await?;
        Ok(())
    }

    fn make_app(
        self,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Response = ServiceResponse<impl MessageBody>,
            Config = (),
            InitError = (),
            Error = actix_web::error::Error,
        >,
    > {
        let (app, api) = App::new()
            .into_utoipa_app()
            .map(|app| app.wrap(Logger::default()))
            // .service()
            .split_for_parts();

        return app.service(Scalar::with_url("/docs", api));
    }
}

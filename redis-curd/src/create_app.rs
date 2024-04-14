use std::time::Instant;
use actix_web::{App, Error, web};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use crate::api::controllers::handlers::SpellHandler::{createSpellHandler, GetAllSpellsHandler, GetSpellHandler, RemoveSpell};
use crate::container::container::Container;

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response=ServiceResponse<impl MessageBody>,
        Config=(),
        InitError=(),
        Error=Error,
    >
> {
    let Spellcontainer: Container = Container::new();

    let SpellService = Spellcontainer.SpellService.clone();
    
  

    App::new()
        .app_data(web::Data::from(SpellService.clone()))
        .wrap(Logger::new(r#"[%a] ["%r"] [Response = %s] [Size = %b] "%{Referer}i" "%{User-Agent}i" Processed in %Tms."#))
        //.wrap(Logger::default())
        .service(
            web::scope("/spells")
                .route("", web::post().to(createSpellHandler))
                .route("", web::get().to(GetAllSpellsHandler))
                .route("/{id}", web::get().to(GetSpellHandler))
                .route("/{id}", web::delete().to(RemoveSpell))
        )
}
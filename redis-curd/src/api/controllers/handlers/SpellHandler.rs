use actix_web::{HttpResponse, web};
use crate::api::dto::SpellDTO::{CreateSpellDTO, SpellDTO};
use crate::domain::error::{ApiError};
use crate::domain::models::Spell::Spell;
use crate::domain::services::SpellService::SpellService;

pub async fn createSpellHandler(SpellService: web::Data<dyn SpellService>, PostReqData: web::Json<CreateSpellDTO>) -> Result<web::Json<SpellDTO>, ApiError> {
    let spell: Spell = SpellService.CreateSpell(PostReqData.into_inner().into()).await?;
    Ok(web::Json(spell.into()))
}

pub async fn GetAllSpellsHandler(SpellService: web::Data<dyn SpellService>) -> Result<web::Json<Vec<SpellDTO>>, ApiError> {
    let AllSpells: Vec<Spell> = SpellService.GetAllSpells().await?;
    Ok(web::Json(AllSpells.into_iter().map(|spell| spell.into()).collect()))
}

pub async fn GetSpellHandler(SpellService: web::Data<dyn SpellService>, ReqParams: web::Path<i32>) -> Result<web::Json<SpellDTO>, ApiError> {
    let spell: Spell = SpellService.GetSpell(ReqParams.into_inner()).await?;
    Ok(web::Json(spell.into()))
}

pub async fn RemoveSpell(SpellService: web::Data<dyn SpellService>, ReqParams: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    SpellService.RemoveSpell(ReqParams.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}
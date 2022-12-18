use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use create_rust_app::Database;
use crate::models::premises::{Premises, PremisesInput};

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<PremisesInput>,
) -> Result<HttpResponse, Error> {
    let mut con = db.get_connection();

    let result = Premises::create(&mut con, &item).expect("Creation error");

    Ok(HttpResponse::Created().json(result))
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(create)
}
use actix_web::{error, get, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/")]
pub async fn rmain() -> impl Responder {
    let context = Context::new();

    match Tera::one_off(include_str!("../view/index.html"), &context, false) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template error: {}", err);
            error::ErrorInternalServerError(err).into()
        }
    }
}

#[get("/index")]
pub async fn rindex2() -> impl Responder {
    let context = Context::new();

    match Tera::one_off(include_str!("../view/index.html"), &context, false) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template error: {}", err);
            error::ErrorInternalServerError(err).into()
        }
    }
}

#[get("/config")]
pub async fn configg() -> impl Responder {
    let context = Context::new();

    match Tera::one_off(include_str!("../view/config.html"), &context, false) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template error: {}", err);
            error::ErrorInternalServerError(err).into()
        }
    }
}

#[get("/api")]
pub async fn api() -> impl Responder {
    let context = Context::new();

    match Tera::one_off(include_str!("../view/integracoes.html"), &context, false) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template error: {}", err);
            error::ErrorInternalServerError(err).into()
        }
    }
}
#[get("/proj")]
pub async fn proj() -> impl Responder {
    let context = Context::new();

    match Tera::one_off(include_str!("../view/proj.html"), &context, false) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template error: {}", err);
            error::ErrorInternalServerError(err).into()
        }
    }
}


#[get("/milestones")]
pub async fn milestones() -> impl Responder {
    let context = Context::new();

    match Tera::one_off(include_str!("../view/milestones.html"), &context, false) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template error: {}", err);
            error::ErrorInternalServerError(err).into()
        }
    }
}


#[get("/buscar")]
pub async fn buscar() -> impl Responder {
    let context = Context::new();

    match Tera::one_off(include_str!("../view/pesquisar.html"), &context, false) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template error: {}", err);
            error::ErrorInternalServerError(err).into()
        }
    }
}


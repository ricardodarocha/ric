use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    error, get,
    http::StatusCode,
    middleware,
    web::{self, ServiceConfig},
    HttpMessage as _, HttpRequest, Responder,
};
use shuttle_actix_web::ShuttleActixWeb;
use std::sync::Arc;
pub mod routes;
use tera::Tera;
use routes::*;
use actix_files as fs;
use sqlx::{FromRow, PgPool};

const TWO_DAYS: Duration = Duration::days(2);

// #[get("/")]
// async fn index(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
//     let id = match identity.map(|id| id.id()) {
//         None => "anonymous".to_owned(),
//         Some(Ok(id)) => id,
//         Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
//     };

//     Ok(format!("Hello {id}"))
// }

#[get("/login")]
async fn login(req: HttpRequest) -> impl Responder {
    // some kind of authentication should happen here

    // attach a verified user identity to the active session
    Identity::login(&req.extensions(), "user1".to_owned()).unwrap();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}

#[get("/logout")]
async fn logout(id: Identity) -> impl Responder {
    id.logout();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}

#[derive(Clone)]
struct AppState {
    pub db: PgPool
}

#[shuttle_runtime::main]
async fn main(
    // #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations");

    //let state = web::Data::new(AppState { db: pool });

    let secret_key = Key::generate();
    let tera = Arc::new(Tera::new("view/**/*").unwrap());


    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::from(tera.clone())).service(
            web::scope("")
                .service(rmain)
                .service(rindex2)
                .service(configg)
                .service(api)
                .service(login)
                .service(logout) 
                .service(buscar)
                .service(proj)
                .service(milestones)                
                .service(fs::Files::new("/assets", "assets"))
                //.app_data(state)
                .wrap(IdentityMiddleware::default())
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                        .cookie_name("3c623a48-046f-518a-a467-d31b16b2de2c".to_owned())
                        .cookie_secure(false)
                        .session_lifecycle(PersistentSession::default().session_ttl(TWO_DAYS))
                        .build(),
                )
                .wrap(middleware::NormalizePath::trim())
                .wrap(middleware::Logger::default()),
        );
    };

    Ok(config.into())
}

use actix_web::http::header::{self, HeaderValue};
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{get, HttpResponse};

use crate::api::GenericResponse;
use crate::db::url::{self, Error, Url};
use crate::{State, User};

pub async fn create_url(body: Json<Url>, _: Query<User>, state: Data<State>) -> HttpResponse {
    match url::create_url(&body, &state.db_pool).await {
        Ok(_) => {
            HttpResponse::Ok().json(GenericResponse::success("Successfully created short URL"))
        }
        Err(err) => {
            let error_message = "Could not create short URL";
            match err {
                Error::ShortExists => HttpResponse::UnprocessableEntity().json(
                    GenericResponse::err(error_message, "This short id is already taken"),
                ),
                _ => HttpResponse::InternalServerError()
                    .json(GenericResponse::err(error_message, "Database failure")),
            }
        }
    }
}

pub async fn delete_url(to_delete: Path<String>, state: Data<State>) -> HttpResponse {
    match url::delete_url(&to_delete, &state.db_pool).await {
        Ok(_) => HttpResponse::Ok().json(GenericResponse::success("Successfully deleted URL")),
        Err(err) => {
            let error_message = "Could not delete URL";
            match err {
                Error::ShortDoesNotExist => HttpResponse::UnprocessableEntity().json(
                    GenericResponse::err(error_message, "This short id does not exist"),
                ),
                _ => HttpResponse::InternalServerError()
                    .json(GenericResponse::err(error_message, "Database failure")),
            }
        }
    }
}

pub async fn list_urls(state: Data<State>) -> HttpResponse {
    match url::list_urls(&state.db_pool).await {
        Ok(urls) => HttpResponse::Ok().json(urls),
        Err(_) => HttpResponse::InternalServerError().json(GenericResponse::err(
            "Could not list URLs",
            "database failure",
        )),
    }
}

#[get("/{short}")]
pub async fn handle_redirect(requested_resource: Path<String>, state: Data<State>) -> HttpResponse {
    let url = match url::get_url(&requested_resource, &state.db_pool).await {
        Ok(url) => url,
        Err(_) => {
            return HttpResponse::NotFound().json(GenericResponse::err(
                &format!("Cannot redirect to resource `{requested_resource}`"),
                "this shortened url was not found",
            ))
        }
    };

    HttpResponse::TemporaryRedirect()
        .append_header((
            header::LOCATION,
            match HeaderValue::from_str(&url.target_url) {
                Ok(header) => header,
                Err(err) => {
                    return HttpResponse::UnprocessableEntity().json(GenericResponse::err(
                        "Cannot redirect to resource `{path}`",
                        &format!("invalid target URL: {err}"),
                    ))
                }
            },
        ))
        .json(url)
}

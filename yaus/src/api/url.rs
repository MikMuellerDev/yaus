use actix_web::http::header::{self, HeaderValue};
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{get, HttpResponse};

use crate::api::GenericResponse;
use crate::db::url::{self, Error, Url};
use crate::{State, User};

pub async fn create_url(body: Json<Url>, _: Query<User>, state: Data<State>) -> HttpResponse {
    // Validate the user's input
    if body.short.len() > 20 {
        return HttpResponse::PayloadTooLarge().json(GenericResponse::err(
            "Could not create short url",
            "The short ID may not exceed 20 characters",
        ));
    };
    if body.target_url.len() > 500 {
        return HttpResponse::PayloadTooLarge().json(GenericResponse::err(
            "Could not create short url",
            "The target URL may not exceed 500 characters",
        ));
    };
    // Create the URL in the database
    match url::create_url(&body, &state.db_pool).await {
        Ok(_) => {
            info!(
                "Created redirect from `{}` to `{}`",
                body.short, body.target_url
            );
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
        Ok(_) => {
            info!("Deleted redirect `{to_delete}`");
            HttpResponse::Ok().json(GenericResponse::success("Successfully deleted URL"))
        }
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

pub async fn get_target(requested_resource: Path<String>, state: Data<State>) -> HttpResponse {
    // Fetch the target URL from the database
    match url::get_url(&requested_resource, &state.db_pool).await {
        Ok(url) => HttpResponse::Ok().json(url),
        Err(_) => HttpResponse::UnprocessableEntity().json(GenericResponse::err(
            &format!("Cannot get target URL of `{requested_resource}`"),
            "this shortened url was not found",
        )),
    }
}

pub async fn list_urls(limit: Path<u32>, state: Data<State>) -> HttpResponse {
    match url::list_urls(&state.db_pool, limit.to_owned() as i64).await {
        Ok(urls) => HttpResponse::Ok().json(urls),
        Err(_) => HttpResponse::InternalServerError().json(GenericResponse::err(
            "Could not list URLs",
            "database failure",
        )),
    }
}

#[get("/{short}")]
pub async fn handle_redirect(requested_resource: Path<String>, state: Data<State>) -> HttpResponse {
    // Fetch the target URL from the database
    let url = match url::get_url(&requested_resource, &state.db_pool).await {
        Ok(url) => url,
        Err(_) => {
            return HttpResponse::NotFound().json(GenericResponse::err(
                &format!("Cannot redirect to resource `{requested_resource}`"),
                "this shortened url was not found",
            ))
        }
    };
    // Send the redirect target URL to the client
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

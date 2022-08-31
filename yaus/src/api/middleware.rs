use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web::{Data, Query},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::{State, User};

use super::GenericResponse;

pub struct ValidCredentials;

impl<S, B> Transform<S, ServiceRequest> for ValidCredentials
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = ValidCredentialsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ValidCredentialsMiddleware { service }))
    }
}

pub struct ValidCredentialsMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ValidCredentialsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // Attempts to retrieve the app data state
        let app_data: &Data<State> = request
            .app_data()
            .expect("The `ValidCredentials` middleware requires app data to be present");

        // Specify user validity based on query parameters
        // Attempts to parse a similar query like this `?username=foo&password=bar` into a user
        let has_valid_credentials = match Query::<User>::from_query(request.query_string()) {
            Err(_) => false,
            Ok(user) => {
                user.username == app_data.user.username && user.password == app_data.user.password
            }
        };

        // If the user does not have valid credentials, return an error message
        if !has_valid_credentials {
            let response = HttpResponse::Forbidden()
                .json(GenericResponse::err(
                    "Forbidden",
                    "You must be authenticated to use the API",
                ))
                .map_into_right_body();
            warn!(
                "Rejecting invalid authentication for route `{}`",
                request.path()
            );
            return Box::pin(async { Ok(ServiceResponse::new(request.into_parts().0, response)) });
        }

        // Forward any valid requests to the original handler
        trace!(
            "Accepting valid authentication for route `{}`",
            request.path()
        );
        let res = self.service.call(request);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}

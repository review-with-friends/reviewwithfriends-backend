use crate::Config;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    web::Data,
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

/// A user who has passed authentication checks.
/// The derived String type is the user_id for the user.
#[derive(Debug, Clone, PartialEq)]
pub struct AuthenticatedUser(pub String);

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

/// Authentication middleware used to validate the incoming request.
/// Auth and ping routes are allowed to bypass this validation.
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
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
        if request.path().starts_with("/auth") || request.path().starts_with("/ping") {
            let res = self.service.call(request);

            return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
        }

        let authorization_header: &header::HeaderValue;

        if let Some(header) = request.headers().get(header::AUTHORIZATION) {
            authorization_header = header;
        } else {
            let (request, _pl) = request.into_parts();

            let response = HttpResponse::Unauthorized().finish().map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        if let Ok(token) = authorization_header.to_str() {
            if let Some(config) = request.app_data::<Data<Config>>() {
                if let Some(user_id) = jwt::validate_jwt(&config.signing_keys, token) {
                    // Additional check if we are routing to an admin route.
                    // This is after the jwt validation, and only issued JWTs are valid.
                    // Only I am allowed uwuu~~~ 🥰 - maybe improve this later if more admins are needed
                    if request.path().starts_with("/admin") {
                        if user_id != "70bf5ab0-a51a-4f2a-b07d-009f571f62da" {
                            let (request, _pl) = request.into_parts();

                            let response =
                                HttpResponse::Unauthorized().finish().map_into_right_body();

                            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                        }
                    }

                    request.extensions_mut().insert(AuthenticatedUser(user_id));
                    let res = self.service.call(request);

                    return Box::pin(async move {
                        // forwarded responses map to "left" body
                        res.await.map(ServiceResponse::map_into_left_body)
                    });
                }
            }
        }

        // fall-through means something went wrong
        let (request, _pl) = request.into_parts();
        let response = HttpResponse::Unauthorized().finish().map_into_right_body();
        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
    }
}

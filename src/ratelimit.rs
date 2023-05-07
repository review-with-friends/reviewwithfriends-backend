use crate::Config;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    web::Data,
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use moka::sync::Cache;
use std::future::{ready, Ready};

pub struct RateLimit;

impl<S, B> Transform<S, ServiceRequest> for RateLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddleware { service }))
    }
}

/// Rate limit middleware to restrict the amount of calls to the auth endpoint.
/// Without this is becomes much easier for an auth spammer to send messages to tons of users.
/// This also helps prevent brute forcing.
pub struct RateLimitMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddleware<S>
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
        // if we are a auth path, we are subject to harsh rate limiting
        if request.path().starts_with("/auth") {
            if let Some(ratelimit_store) = request.app_data::<Data<Cache<String, usize>>>() {
                // fetch ip from request - if missing for some reason, we can just use a stand-in.
                let ip: String = if let Some(ip) = request.connection_info().realip_remote_addr() {
                    ip.to_string()
                } else {
                    // group an incoming request somehow....
                    "0.0.0.0".to_string()
                };

                // attempt to get a count from the cache
                let count_opt = ratelimit_store.get(&ip);

                if let Some(count) = count_opt {
                    if count >= 5 {
                        // this request is rate limited, bop it.
                        let (request, _pl) = request.into_parts();
                        let response = HttpResponse::TooManyRequests()
                            .finish()
                            .map_into_right_body();
                        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                    } else {
                        // let the request in but increment out key
                        ratelimit_store.insert(ip, count + 1);
                    }
                } else {
                    // if not count, we can assume this is a first request and intialize the cache
                    ratelimit_store.insert(ip, 1);
                }

                let res = self.service.call(request);
                return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
            } else {
                // this means we are an auth route; but something broke that isn't caught compile time
                let (request, _pl) = request.into_parts();
                let response = HttpResponse::InternalServerError()
                    .finish()
                    .map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        } else {
            // we are not a route subject to the hard rate limiting.
            let res = self.service.call(request);
            return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
        }
    }
}

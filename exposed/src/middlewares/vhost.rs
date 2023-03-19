use std::future::{Ready, ready};
use std::ops::Deref;
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, http, HttpResponse};
use actix_web::body::EitherBody;
use futures_util::future::LocalBoxFuture;


pub struct VHost {
    vhost_suffix: String,
}

impl VHost {
    pub fn new(vhost_suffix: String) -> Self {
        VHost { vhost_suffix }
    }
}

impl<S, B> Transform<S, ServiceRequest> for VHost
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = VHostMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, inner: S) -> Self::Future {
        ready(Ok(VHostMiddleware { inner, vhost_suffix: self.vhost_suffix.clone() }))
    }
}

pub struct VHostMiddleware<S> {
    inner: S,
    vhost_suffix: String,
}

impl<S, B> Service<ServiceRequest> for VHostMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(inner);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self.handles_vhost(&req) {
            let (request, _pl) = req.into_parts();

            let response = HttpResponse::Ok()
                .finish()
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.inner.call(req);

        Box::pin(async move {
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}

impl<S> VHostMiddleware<S> {
    fn handles_vhost(&self, req: &ServiceRequest) -> bool {
        match req.headers().get(http::header::HOST) {
            Some(host_header) => {
                let host_value = host_header.to_str().unwrap();

                host_value.ends_with(self.vhost_suffix.deref())
            },
            None => false
        }
    }
}

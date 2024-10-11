use super::response;
use crate::api::client::HTTP_CLIENT;
use crate::error::{self, Error, Result};
use reqwest::{multipart::Form, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub(crate) fn return_response<T>(resp: response::Response<T>) -> Result<response::Response<T>> {
    match resp.code().as_str() {
        "OK" => Ok(resp),
        _ => Err(Error::from(resp)),
    }
}

pub(crate) async fn post<T, R>(url: &str, json: &T) -> error::Result<R>
where
    T: Serialize,
    R: DeserializeOwned,
{
    let resp = request(Method::POST, url, |req| req.json(json)).await?;
    Ok(resp)
}

pub(crate) async fn put<T, R>(url: &str, json: &T) -> error::Result<R>
where
    T: Serialize,
    R: DeserializeOwned,
{
    let resp = request(Method::PUT, url, |req| req.json(json)).await?;
    Ok(resp)
}

pub(crate) async fn patch<T, R>(url: &str, json: &T) -> error::Result<R>
where
    T: Serialize,
    R: DeserializeOwned,
{
    let resp = request(Method::PATCH, url, |req| req.json(json)).await?;
    Ok(resp)
}

pub(crate) async fn post_with_form<R: DeserializeOwned>(url: &str, form: Form) -> error::Result<R> {
    let resp = request(Method::POST, url, |req| req.multipart(form)).await?;
    Ok(resp)
}

pub(crate) async fn delete<R: DeserializeOwned>(url: &str) -> error::Result<R> {
    let resp = request(Method::DELETE, url, |req| req).await?;
    Ok(resp)
}

pub(crate) async fn get<R: DeserializeOwned>(url: &str) -> error::Result<R> {
    let resp = request(Method::GET, url, |req| req).await?;
    Ok(resp)
}

pub(crate) async fn request<R, F>(method: Method, url: &str, build_req: F) -> error::Result<R>
where
    R: DeserializeOwned,
    F: FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
{
    let resp = build_req(HTTP_CLIENT.request(method, url))
        .send()
        .await?
        .json::<R>()
        .await?;
    Ok(resp)
}

use std::{fs, path::PathBuf, sync::LazyLock};

use crate::{error::Result, Error};
use reqwest::{
    multipart::{Form, Part},
    Client,
};
use serde_json::json;

mod client;
pub mod response;
mod util;

use util::return_response;

const DEFAULT_FILENAME: &str = "default.md";

static BASE_URL: LazyLock<&str> = LazyLock::new(|| {
    env!(
        "BLC_API",
        "You must set the environment variable $BLC_API before compiling"
    )
});

pub async fn login(password: &str) -> Result<String> {
    let resp = Client::new()
        .post(format!("{}/authentication", *BASE_URL))
        .json(&json!({
            "password": password
        }))
        .send()
        .await?;

    let headers = resp.headers().clone();
    return_response(resp.json::<response::Response<()>>().await?).and_then(|_| {
        headers
            .get("X-Auth-Token")
            .map(|token| token.to_str().unwrap().into())
            .ok_or(Error::ServiceError)
    })
}

pub async fn initializetion_article(uri: &str, category: &str) -> Result<response::Response<()>> {
    let resp = util::post(
        &format!("{}/articles", *BASE_URL),
        &json!({
            "uri": uri,
            "category": category,
        }),
    )
    .await?;

    return_response(resp)
}

pub async fn delete_article(uri: &str) -> Result<response::Response<()>> {
    let resp = util::delete(&format!("{}/articles/{}", *BASE_URL, uri)).await?;
    return_response(resp)
}

pub async fn upload_new_version(uri: &str, file_path: &PathBuf) -> Result<response::Response<()>> {
    let file = fs::read(file_path)?;

    let file_name = file_path
        .file_name()
        .map_or(DEFAULT_FILENAME.to_string(), |name| {
            name.to_string_lossy().into_owned()
        });

    let form = Form::new().part("content", Part::bytes(file).file_name(file_name));

    let resp =
        util::post_with_form(&format!("{}/articles/{}/versions", *BASE_URL, uri), form).await?;
    return_response(resp)
}

pub async fn get_article_list(
    page: u32,
    limit: Option<u32>,
    category: Option<&str>,
    tags: Option<&str>,
) -> Result<response::Response<response::Articles>> {
    let mut query = format!("page={}", page);

    if let Some(limit) = limit {
        query.push_str(&format!("&limit={}", limit));
    }

    if let Some(category) = category {
        query.push_str(&format!("&category={}", category));
    }

    if let Some(tags) = tags {
        query.push_str(&format!("&tags={}", tags));
    }

    let resp = util::get(&format!("{}/author/articles?{}", *BASE_URL, query)).await?;
    return_response(resp)
}

pub async fn delete_article_version(uri: &str, version: &str) -> Result<response::Response<()>> {
    let resp = util::delete(&format!(
        "{}/articles/{}/versions/{}",
        *BASE_URL, uri, version
    ))
    .await?;

    return_response(resp)
}

pub async fn get_article_version_list(uri: &str) -> Result<response::Response<response::Versions>> {
    let resp = util::get(&format!("{}/articles/{}/versions", *BASE_URL, uri)).await?;
    return_response(resp)
}

pub async fn set_article_visibility(uri: &str, visibility: bool) -> Result<response::Response<()>> {
    let resp = util::patch(
        &format!("{}/articles/{uri}/visibility", *BASE_URL),
        &json!({
            "visibility": visibility,
        }),
    )
    .await?;
    return_response(resp)
}

pub async fn set_article_category(uri: &str, category: &str) -> Result<response::Response<()>> {
    let resp = util::patch(
        &format!("{}/articles/{uri}/categroy", *BASE_URL),
        &json!({
            "category": category,
        }),
    )
    .await?;
    return_response(resp)
}

pub async fn set_article_tags(uri: &str, tags: &str) -> Result<response::Response<()>> {
    let resp = util::patch(
        &format!("{}/articles/{uri}/tags", *BASE_URL),
        &json!({
            "tags": tags.split(",").map(|tag| tag.trim()).collect::<Vec<&str>>(),
        }),
    )
    .await?;
    return_response(resp)
}

pub async fn set_article_version(uri: &str, version: &str) -> Result<response::Response<()>> {
    let resp = util::put(
        &format!("{}/articles/{uri}/tags", *BASE_URL),
        &json!({
            "version": version,
        }),
    )
    .await?;
    return_response(resp)
}

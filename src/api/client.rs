use std::sync::LazyLock;

use reqwest::{header, Client};

use crate::CFG;

fn init_http_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", &CFG.token)).unwrap(),
    );
    Client::builder()
        .default_headers(headers)
        .build()
        .expect("初始化http客户端失败")
}

pub static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(init_http_client);

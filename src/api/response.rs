use std::fmt::Display;

use chrono::DateTime;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Response<D> {
    code: String,
    data: Option<D>,
}

impl<D> Response<D> {
    pub fn message(&self) -> String {
        let result = match self.code.as_str() {
            "OK" => "OK",
            "UNIMPLEMENTED" => {
                "This feature has not been implemented yet, and the operation is not supported."
            }
            "UNAUTHORISED" => "The user is not authorized to perform this action.",
            "INTERNAL_SERVICE_ERROR" => {
                "An internal system error occurred. Please try again later."
            }
            "INVALID_PARAMETERS" => "The request contains invalid parameters.",
            "INVALID_ACTION" => "The action is invalid or unsupported.",
            "RESOURCE_NOT_EXIST" => "The requested resource does not exist.",
            _ => "",
        };
        result.to_string()
    }

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn data(&self) -> &Option<D> {
        &self.data
    }
}

#[derive(Debug, Deserialize)]
pub struct Articles {
    pub count: u32,
    pub page: u32,
    pub items: Vec<ArticleMetadata>,
    pub next: bool,
    pub prev: bool,
}

impl Display for Articles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, article) in self.items.iter().enumerate() {
            writeln!(f, "{}. {}", i + 1, article)?;
        }

        write!(
            f,
            "\ncurrent page: {}\nnext page: {}\nprevious page: {}",
            self.page,
            if self.next { "Yes" } else { "No" },
            if self.prev { "Yes" } else { "No" }
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct ArticleMetadata {
    pub uri: String,
    pub tags: Vec<String>,
    #[serde(rename = "firstVersionCreatedAt")]
    pub created_at: i64,
    pub version: String,
    pub visibility: bool,
    pub category: Category,
}

impl Display for ArticleMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] - [{}] {}:{}",
            if self.visibility { "*" } else { " " },
            format_detatime_from_timestamp(self.created_at),
            self.uri,
            self.version,
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub slug: String,
    pub name: String,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.slug)
    }
}

#[derive(Debug, Deserialize)]
pub struct Versions {
    pub count: u32,
    pub items: Vec<Version>,
}

impl Display for Versions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self.items.iter().enumerate() {
            writeln!(f, "  {}. {}", i + 1, item)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub version: String,
    pub note: String,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] - ({}):\"{}\" - \"{}\"",
            format_detatime_from_timestamp(self.created_at),
            self.version,
            self.title,
            self.note
        )
    }
}

fn format_detatime_from_timestamp(timestamp: i64) -> String {
    DateTime::from_timestamp_millis(timestamp)
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

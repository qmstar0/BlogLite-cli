use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
/// A CLI tool for managing articles in your blog platform.
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// All actions must be executed after logging in.
    Login {
        /// The password used for login.
        password: String,
    },

    /// Clear local login status.
    Logout,

    /// Initialize a new article with a category and URI.
    Init {
        /// The category of the article.
        #[arg(short, long)]
        category: String,

        /// Custom article URI.
        uri: String,
    },

    /// Upload a local file to the specified article URI.
    Upload {
        /// The URI of the article to which the file will be uploaded.
        uri: String,

        /// The path to the local file to upload.
        path: PathBuf,
    },

    /// Remove a specific version of an article.
    Rm {
        /// The URI of the article.
        uri: String,

        /// The version of the article to remove.
        version: String,
    },

    /// Delete an entire article by URI.
    Delete {
        /// The URI of the article to delete.
        uri: String,
    },

    /// Set various options for an article, such as visibility, category, and tags.
    Set {
        /// The URI of the article to modify.
        uri: String,

        /// Additional options for the article.
        #[command(flatten)]
        article_option: ArticleOption,
    },

    /// List articles with optional pagination, category, and tag filters.
    List {
        /// The page number to display.
        #[arg(short, long)]
        page: Option<u32>,

        /// The number of articles to display per page.
        #[arg(short, long)]
        limit: Option<u32>,

        /// Filter by category.
        #[arg(long)]
        category: Option<String>,

        /// Filter by tags.
        #[arg(long)]
        tags: Option<String>,
    },

    /// Show all versions of a specific article.
    Versions {
        /// The URI of the article.
        uri: String,
    },
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
/// Options for setting article properties like visibility, category, and tags.
/// These options are mutually exclusive, meaning that public and private cannot be set at the same time.
pub struct ArticleOption {
    /// Set the article as publicly visible.
    #[arg(long)]
    pub public: bool,

    /// Set the article as private.
    #[arg(long)]
    pub private: bool,

    /// Set the category of the article.
    #[arg(long)]
    pub category: Option<String>,

    /// Set tags for the article, separated by commas.
    #[arg(long)]
    pub tags: Option<String>,

    /// Specify a version of the article.
    #[arg(short, long)]
    pub version: Option<String>,
}

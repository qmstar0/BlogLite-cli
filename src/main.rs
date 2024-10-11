use blc::config::Config;
use blc::{api, Action, Cli};
use blc::{Error, Result};
use clap::Parser;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let output_info = match &cli.action {
        Action::Init { uri, category } => {
            api::initializetion_article(uri, category)
                .await
                .unwrap_or_else(on_error);
            Some(format!("Article Initialization, uri: {}", uri.bold()))
        }

        Action::Delete { uri } => {
            api::delete_article(uri).await.unwrap_or_else(on_error);
            Some(format!("Article deleted, uri: {}", uri.bold()))
        }

        Action::Upload { uri, path } => {
            api::upload_new_version(uri, path)
                .await
                .unwrap_or_else(on_error);
            Some(format!("Article uploaded, uri: {}", uri.bold()))
        }

        Action::List {
            page,
            limit,
            category,
            tags,
        } => {
            let resp = api::get_article_list(
                page.unwrap_or(1),
                *limit,
                category.as_deref(),
                tags.as_deref(),
            )
            .await
            .unwrap_or_else(on_error);
            resp.data().as_ref().map(|articles| {
                println!("{}", articles);
                format!("Query done. Total {} items.", articles.count)
            })
        }

        Action::Rm { uri, version } => {
            api::delete_article_version(uri, version)
                .await
                .unwrap_or_else(on_error);
            Some(format!(
                "Article version deleted, uri: {}, version: {}",
                uri.bold(),
                version.bold()
            ))
        }

        Action::Versions { uri } => {
            let resp = api::get_article_version_list(uri)
                .await
                .unwrap_or_else(on_error);
            resp.data().as_ref().map(|versions| {
                println!("Article {} version list:\n{}", uri.bold(), versions);
                format!("Query done. Total {} items.", versions.count)
            })
        }

        Action::Set {
            uri,
            article_option,
        } => {
            if article_option.public {
                api::set_article_visibility(uri, true)
                    .await
                    .unwrap_or_else(on_error);
            }

            if article_option.private {
                api::set_article_visibility(uri, false)
                    .await
                    .unwrap_or_else(on_error);
            }

            if let Some(tags) = &article_option.tags {
                api::set_article_tags(uri, tags)
                    .await
                    .unwrap_or_else(on_error);
            }

            if let Some(category) = &article_option.category {
                api::set_article_category(uri, category)
                    .await
                    .unwrap_or_else(on_error);
            }

            if let Some(version) = &article_option.version {
                api::set_article_version(uri, version)
                    .await
                    .unwrap_or_else(on_error);
            }

            Some("successful.".to_string())
        } // _ => None,
        Action::Login => {
            let password = rpassword::prompt_password("password: ")
                .map_err(Error::from)
                .unwrap_or_else(on_error);

            let token = api::login(&password).await.unwrap_or_else(on_error);
            Config::new(&token).save().unwrap();
            Some("login successful.".to_string())
        }

        Action::Logout => {
            Config::clear().unwrap_or_else(on_error);
            Some("logout successful.".to_string())
        }
    };

    if let Some(info) = output_info {
        println!("{} Finished: {}", "✓".green().bold(), info);
    }

    Ok(())
}

fn on_error<R>(e: Error) -> R {
    eprintln!("{} Error: {}", "×".red().bold(), e);
    std::process::exit(1)
}

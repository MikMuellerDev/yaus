use api::{Client, Error, User};
use clap::Parser;
use std::process;

mod api;
mod cli;

#[derive(Parser)]
#[clap(author, version, about)]
enum Subcommand {
    Add(AddArgs)
}

#[derive(Parser)]
struct AddArgs {
    /// The short id of the new redirect
    #[clap(short, long)]
    short: String,
    /// The target URL of the new redirect
    #[clap(short, long)]
    target_url: String,
}

#[tokio::main]
async fn main() {
    // Create the Yaus client, handle potential errors
    let client = match Client::new(
        "http://localhost:8080",
        User {
            username: "test",
            password: "secret",
        },
    )
    .await
    {
        Ok(client) => client,
        Err(err) => {
            eprintln!(
                "Failed to initialize connection: {}",
                match err {
                    Error::UrlParse(err) => format!("Invalid Yaus-URL specified: {err}"),
                    Error::Reqwest(err) => format!("Cannot connect to Yaus server: {err}"),
                    Error::Yaus(status) => format!("YAUS error: status-code: {status}"),
                }
            );
            process::exit(1);
        }
    };

    match Subcommand:

    let res = cli::list_redirects(&client).await;
     // let res = cli::create_redirect(
         // &client,
         // &api::Redirect {
             // short: "test".to_string(),
             // target_url: "https://mik-mueller.de".to_string(),
         // },
     // ).await;
    process::exit(if res.is_ok() { 0 } else { 1 })
}

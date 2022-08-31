use api::{Client, Error, Redirect, User};
use clap::Parser;
use std::process;

mod api;
mod cli;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// A Foo value which does nothing
    #[clap(short, long, value_parser)]
    foo: Option<String>,
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

    println!(
        "{:?}",
        client
            .create_url(&Redirect {
                short: "42".to_string(),
                target_url: "http://gy-cfg.de".to_string()
            })
            .await
    );
    println!("{:?}", client.delete_url("42!?").await);
    println!("{:?}", client.get_target("42").await);
    println!("{:?}", client.list_urls().await);
}
use api::{Client, Error, User};
use clap::Parser;
use std::process;

mod api;
mod cli;

#[derive(Parser)]
#[clap(author, version, about)]
enum Yaus {
    #[clap(arg_required_else_help = true)]
    /// Create a new short-URL
    New {
        /// The short id of the new redirect
        #[clap(required = true)]
        short: String,
        /// The target URL of the new redirect
        #[clap(required = true)]
        target_url: String,
    },
    #[clap(arg_required_else_help = true)]
    /// Delete an existing redirect
    Del {
        /// The short id of the redirect
        #[clap(required = true)]
        short: Vec<String>,
    },
    /// Follow a redirect and print its target URL
    Get {
        /// The short id of the redirect
        #[clap(required = true)]
        short: String,
    },
    /// Print a list of all configured redirects
    List {
        /// How many items should be displayed at maximum
        max: u32,
    },
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
    let success = match Yaus::parse() {
        Yaus::List { max } => cli::list_redirects(&client, max).await.is_ok(),
        Yaus::Get { short } => cli::get_target(&client, &short).await.is_ok(),
        Yaus::New { short, target_url } => {
            cli::create_redirect(&client, &api::Redirect { short, target_url })
                .await
                .is_ok()
        }
        Yaus::Del { short } => {
            let mut success = true;
            for item in short {
                if let Err(_) = cli::delete_redirect(&client, &item).await {
                    success = false;
                    break;
                }
            }
            success
        }
    };
    process::exit(if success { 0 } else { 1 });
}

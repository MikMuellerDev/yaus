use api::{Client, Error, User};
use clap::Parser;
use std::process;

mod api;
mod cli;
mod config;

#[derive(Parser)]
#[clap(author, version, about)]
enum Yaus {
    #[clap(arg_required_else_help = true)]
    /// Create a new short-URL
    Add {
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
        #[clap(short, long)]
        max: Option<u32>,
    },
}

#[tokio::main]
async fn main() {
    // Get the configuration file location
    let config_file_path = match config::file_path() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    // Parse CLI arguments
    let args = Yaus::parse();

    // Read the configuration file
    let conf = match config::read_config(&config_file_path).await {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Could not read configuration file (at `{config_file_path}`): {err})");
            process::exit(1);
        }
    };
    println!("Connecting to server ({}@{})...", conf.user, conf.url);
    // Create the Yaus client, handle potential errors
    let client = match Client::new(
        &conf.url,
        User {
            username: &conf.user,
            password: &conf.password,
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
    // Execute different functions based on the Clap subcommand
    let success = match args {
        Yaus::List { max } => {
            cli::list_redirects(&client, if let Some(max) = max { max } else { u32::MAX })
                .await
                .is_ok()
        }
        Yaus::Get { short } => cli::get_target(&client, &short).await.is_ok(),
        Yaus::Add { short, target_url } => {
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

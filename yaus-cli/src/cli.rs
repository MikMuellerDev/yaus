use crate::api::Client;
use crate::api::Redirect;
use crate::api::Result;
use cli_table::CellStruct;
use cli_table::{Cell, Style, Table};

pub async fn create_redirect(client: &Client<'_>, redirect: &Redirect) -> Result<()> {
    println!("Creating redirect...",);
    if let Err(err) = client.create_url(redirect).await {
        eprintln!("Could not create redirect: {:?}", err);
        return Err(err);
    };
    println!(
        "Successfully created redirect from {} -> {}",
        redirect.short, redirect.target_url
    );
    Ok(())
}

pub async fn delete_redirect(client: &Client<'_>, short_id: &str) -> Result<()> {
    println!("Deleting redirect `{short_id}`...",);
    if let Err(err) = client.delete_url(short_id).await {
        eprintln!("Could not delete redirect: {:?}", err);
        return Err(err);
    };
    println!("Successfully deleted redirect {}", short_id);
    Ok(())
}

pub async fn get_target(client: &Client<'_>, short_id: &str) -> Result<()> {
    match client.get_target(short_id).await {
        Ok(redirect) => {
            println!("Redirect {short_id}\n=> {}", redirect.target_url);
            Ok(())
        }
        Err(err) => {
            eprintln!("Could not get target of redirect: {:?}", err);
            Err(err)
        }
    }
}

pub async fn list_redirects(client: &Client<'_>, max_entries: u32) -> Result<()> {
    let redirects = match client.list_urls(max_entries).await {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Could not list all redirects: {:?}", err);
            return Err(err);
        }
    };
    let output = match &redirects.len() {
        0 => format!("No redirects (empty set)"),
        _ => {
            let table = redirects
                .into_iter()
                .enumerate()
                .map(|(index, redirect)| {
                    vec![
                        index.cell().dimmed(true),
                        client
                            .url
                            .join(&redirect.short)
                            .expect("A client can only exist with a valid base-URL")
                            .cell(),
                        redirect.target_url.cell(),
                    ]
                })
                .collect::<Vec<Vec<CellStruct>>>()
                .table()
                .title(vec!["".cell(), "Source URL".cell(), "Target URL".cell()])
                .bold(true);

            table.display().unwrap().to_string()
        }
    };
    println!("{output}");
    Ok(())
}

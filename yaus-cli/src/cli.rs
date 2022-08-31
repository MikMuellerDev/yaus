use crate::api::Client;
use crate::api::Redirect;
use crate::api::Result;

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
    println!("Deleting redirect...",);
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

pub async fn list_redirects(client: &Client<'_>) -> Result<()> {
    let redirects = match client.list_urls().await {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Could not list all redirects: {:?}", err);
            return Err(err);
        }
    };

    let output = redirects
        .iter()
        .map(|redirect| format!("Source: {:20} => {}", redirect.short, redirect.target_url))
        .collect::<Vec<String>>()
        .join("\n");

    println!("{output}");
    Ok(())
}

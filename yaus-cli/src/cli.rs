use crate::api::Redirect;
use crate::api::Result;

pub async fn create_redirect(redirect: &Redirect) -> Result<()> {
    println!(
        "Creating redirect from {} -> {}",
        redirect.short, redirect.target_url
    );

    Ok(())
}

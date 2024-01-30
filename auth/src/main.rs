use aws_sdk_cognitoidentity::{config::Region, meta::PKG_VERSION, Client};

enum AuthError {}

#[::tokio::main]
async fn main() -> Result<(), AuthError> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_cognitoidentity::Client::new(&config);

    // ... make some calls with the client

    Ok(())
}

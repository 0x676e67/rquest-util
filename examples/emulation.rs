use rquest::Client;
use rquest_util::Emulation;

#[tokio::main]
async fn main() -> Result<(), rquest::Error> {
    // Build a client to emulation Firefox135
    let client = Client::builder().emulation(Emulation::Firefox133).build()?;

    // Use the API you're already familiar with
    let text = client
        .get("https://tls.peet.ws/api/all")
        .send()
        .await?
        .text()
        .await?;

    println!("{}", text);

    Ok(())
}

// https://blog.logrocket.com/making-http-requests-rust-reqwest/

use std::error::Error;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

/*
{
    version: "1.0.0",
    metadata_id: "a156bbc0-c4cf-49bd-b1e3-677244bc8e8f",
    description: "Congrats on an impressive launch @lensprotocol ! Now, who's gonna build a Lens-powered ticketing platform... 👀🎟",
    content: "Congrats on an impressive launch @lensprotocol ! Now, who's gonna build a Lens-powered ticketing platform... 👀🎟",
    external_url: null,
    image: null,
    imageMimeType: null,
    name: "Comment by @letsraave.lens",
    attributes: [
    {
    traitType: "string",
    key: "type",
    value: "comment"
    }
    ],
    media: [ ],
    appId: "Lenster"
}

*/


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = "https://cloudflare-ipfs.com/ipfs/QmY2BeFyRka9AMFwmXmtSy9EwpNfP6r4LHymmQLcMyLdAS";
    let resp = client.get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("{:#?}", resp);

    if let Some(appId) = resp.get("appId") {
        println!("appId {}", appId)
    }
    Ok(())
}

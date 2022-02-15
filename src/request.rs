use std::time::Duration;

use crate::{Ferinth, Result};
use reqwest::{header::USER_AGENT, IntoUrl, Response, Url};

lazy_static::lazy_static! {
    pub(crate) static ref API_URL_BASE: Url = Url::parse("https://api.modrinth.com/v2/").unwrap();
}

/// Perform a GET request on `url` using the HTTPS client and user agent from `client`
pub(crate) async fn request(client: &Ferinth, url: impl IntoUrl) -> Result<Response> {
    let request = client
        .client
        .get(url)
        .header(USER_AGENT, &client.user_agent);

    let sent_request = request.send().await?.error_for_status()?;

    if let Some(requests_left) = sent_request.headers().get("x-ratelimit-limit") {
        if requests_left.to_str().unwrap().parse().unwrap_or(0) <= 1 {
            if let Some(sleep_for_sec) = sent_request.headers().get("x-ratelimit-reset") {
                dbg!(&sleep_for_sec);

                std::thread::sleep(Duration::from_secs(
                    sleep_for_sec.to_str().unwrap().parse().unwrap_or(60),
                ));
            }
        }
    }

    Ok(sent_request)
}

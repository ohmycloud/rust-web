use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWordsResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

async fn transform_error(
    res: reqwest::Response
) -> handle_errors::APILayerError {
    handle_errors::APILayerError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}

pub async fn check_profanity(
    content: String
) -> Result<String, handle_errors::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", "XFjLHsAGUCuIRLj4NGgLtzOCjaEzyMro")
        .body(content)
        .send()
        .await
        .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;

    // Checks whether the respinse status was successful
    if !res.status().is_success() {
        // The status also indicates whether it was a client or server error.
        if res.status().is_client_error() {
            let err = transform_error(res).await;
            // Returns a client error with our APILayerError encapsulated
            return Err(handle_errors::Error::ClientError(err))?;
        } else {
            let err = transform_error(res).await;
            // Returns a server error with our APILayerError encapsulated
            return Err(handle_errors::Error::ServerError(err))?;
        }
    }

    match res.json::<BadWordsResponse>().await {
        Ok(res) => Ok(res.censored_content),
        Err(e) => Err(handle_errors::Error::ExternalAPIError(e)),
    }
}













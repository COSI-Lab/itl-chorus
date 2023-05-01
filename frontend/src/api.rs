//! This module contains methods for interacting with the server via the API
//!
//! Consistency is maintained by having the server and frontend share the same
//! types from the `common` module

use gloo_net::http::Request;
use thiserror::Error;
use yew::Callback;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] gloo_net::Error),
    #[error("Failed to parse response: {0}")]
    ParseFailed(#[from] serde_json::Error),
}

/// This function returns a callback
pub fn make_request<T, E>(
    request: Request,
    cb: yew::Callback<Result<T, ApiError>>,
) -> yew::Callback<E>
where
    T: serde::de::DeserializeOwned + 'static,
{
    Callback::from(move |_| {
        let cb = cb.clone();
        let request = request.clone();

        let request = || async {
            let response = request.send().await?;
            let text = response.text().await?;
            Ok(serde_json::from_str(&text)?)
        };

        wasm_bindgen_futures::spawn_local(async move {
            cb.emit(request().await);
        });
    })
}

use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use std::error::Error;

#[wasm_bindgen]
pub fn generate_content(api_key: &str, user_input: &str) -> Result<(), JsValue> {
    let api_key = api_key.to_string();
    let user_input = user_input.to_string();

    spawn_local(async move {
        match generate_content_async(api_key, user_input).await {
            Ok(response) => {
                // Log the response to the console
                set_output(&response);
                web_sys::console::log_1(&JsValue::from_str(&response));
            }
            Err(err) => {
                // Log the error to the console
                web_sys::console::log_1(&JsValue::from_str(&format!("Error: {}", err)));
            }
        }
    });

    Ok(())
}

async fn generate_content_async(api_key: String, user_input: String) -> Result<String, Box<dyn Error>> {
    // Create a client
    let client = reqwest::Client::builder().build()?;

    // Set up headers
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    // Construct JSON body dynamically
    let json_body = json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": user_input
                    }
                ]
            }
        ]
    });

    // API URL
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", api_key);

    // Make the request
    let request = client
        .request(reqwest::Method::POST, &url)
        .headers(headers)
        .body(json_body.to_string());

    // Send the request and get the response
    let response = request.send().await?;
    let body = response.text().await?;

    // Parse the JSON response
    let v: Value = serde_json::from_str(&body)?;
    if let Some(text) = v["candidates"]
        .get(0)
        .and_then(|candidate| candidate["content"]["parts"].get(0))
        .and_then(|part| part["text"].as_str())
    {
        // Return the extracted text
        Ok(text.to_string())
    } else {
        Ok("No text found in the response.".to_string())
    }
}

#[wasm_bindgen]
extern "C" {
    fn set_output(output: &str);
}
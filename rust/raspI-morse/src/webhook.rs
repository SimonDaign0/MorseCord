use reqwest::blocking::Client;
use serde_json::json;

pub fn send_discord_msg(
    webhook_url: &str,
    message_content: &str,
    username: Option<&str>,
    avatar_url: Option<&str>,
) {
    // Build the JSON payload
    let mut data = json!({
        "content": message_content,
    });

    if let Some(name) = username {
        data["username"] = json!(name);
    }

    if let Some(avatar) = avatar_url {
        data["avatar_url"] = json!(avatar);
    }

    // Create HTTP client
    let client = Client::new();

    // Send POST request
    let response = client.post(webhook_url).json(&data).send();

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("Message sent successfully! Status code: {}", resp.status());
            } else {
                let status = resp.status();
                match resp.json::<serde_json::Value>() {
                    Ok(err_json) => println!("Failed: {:?}, Status: {}", err_json, status),
                    Err(_) => println!("Failed to send message. Status: {}", status),
                }
            }
        }
        Err(err) => {
            println!("Request error: {}", err);
        }
    }
}

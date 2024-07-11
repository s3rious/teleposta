use frankenstein::Message;
use frankenstein::MethodResponse;
use frankenstein::TelegramApi;
use frankenstein::{Api, Error, ParseMode, SendMessageParams};
use serde_json::Value;
use std::collections::HashMap;
use unescape::unescape;

pub fn get_formatted_message(message: &str, template: Option<&String>) -> String {
    if let Some(template_str) = template {
        let message_obj: HashMap<String, Value> =
            serde_json::from_str(message).expect("Invalid JSON format");

        let mut formatted_message = template_str.to_string();

        for (key, value) in message_obj.iter() {
            let placeholder = format!("${{{}}}", key);
            let value_str = value.as_str().unwrap();
            formatted_message = formatted_message.replace(&placeholder, value_str);
        }

        return unescape(formatted_message.as_str()).unwrap();
    }

    return unescape(message).unwrap();
}

pub fn send_message_to(
    token: &str,
    channel: String,
    message: &String,
) -> Result<MethodResponse<Message>, Error> {
    let api = Api::new(&token);

    let params = SendMessageParams::builder()
        .chat_id(channel)
        .text(message)
        .parse_mode(ParseMode::Html)
        .build();

    return api.send_message(&params);
}

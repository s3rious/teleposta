use teleposta::get_formatted_message;

#[test]
fn test_get_formatted_message_without_template() {
    let message = r#"title body"#;
    let formatted_message = get_formatted_message(message, None);
    assert_eq!(formatted_message, message);
}

#[test]
fn test_get_formatted_message_with_template() {
    let message = r#"{"title":"title","body":"body"}"#;
    let template = r#"<b>${title}</b>\n\n${body}"#.to_string();
    let formatted_message = get_formatted_message(message, Some(&template));
    assert_eq!(formatted_message, "<b>title</b>\n\nbody");
}

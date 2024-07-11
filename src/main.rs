use clap::{Arg, Command};
use teleposta::{get_formatted_message, send_message_to};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let matches = Command::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .arg(
            Arg::new("token")
                .short('t')
                .long("token")
                .help("Telegram bot token")
                .required(true)
                .require_equals(true),
        )
        .arg(
            Arg::new("channel")
                .short('c')
                .long("channel")
                .help("Telegram channel ID")
                .required(true)
                .require_equals(true),
        )
        .arg(
            Arg::new("message")
                .short('m')
                .long("message")
                .help("Message content to post")
                .required(true)
                .require_equals(true),
        )
        .arg(
            Arg::new("template")
                .short('p')
                .long("template")
                .help("Template for formatting the message")
                .require_equals(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .num_args(0)
                .help("Enable verbose logging"),
        )
        .get_matches();

    let token = matches.get_one::<String>("token").unwrap();
    let channel = matches.get_one::<String>("channel").unwrap();
    let message = matches.get_one::<String>("message").unwrap();
    let template = matches.get_one::<String>("template");
    let verbose = matches.get_flag("verbose");

    if verbose {
        println!("Formatting message...");
    }

    let formatted_message = get_formatted_message(message, template);

    if verbose {
        println!("... formatted message: {}", formatted_message);
        println!("... sending message to channel...");
    }

    match send_message_to(token, channel.to_string(), &formatted_message) {
        Ok(_) => {
            if verbose {
                println!("... message sent successfully!");
            }
        }
        Err(error) => {
            eprintln!("Error sending message: {:?}", error);

            std::process::exit(1);
        }
    }
}

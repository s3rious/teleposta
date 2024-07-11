# Teleposta

Teleposta is a command line application for sending a single message to a Telegram channel using a bot API and then momentarily shutting itself down.

## Usage

```sh
teleposta --token=YOUR_TELEGRAM_BOT_TOKEN --channel=YOUR_CHANNEL_ID --message="Your message" [--template="Your template"] [--verbose]
```

### Arguments

- `--token`: Telegram bot token (required).
- `--channel`: Telegram channel ID (required).
- `--message`: Message content to post (required).
- `--template`: Template for formatting the message (optional).
- `--verbose`: Enable verbose logging (optional).

### Running the Tests

To run the tests, use the following command:

```sh
cargo test
```

### Running the Application

#### In debug mode

Use the following command:

```sh
cargo run -- --token=YOUR_TELEGRAM_BOT_TOKEN --channel=YOUR_CHANNEL_ID --message="Your message" [--template="Your template"] [--verbose]
```

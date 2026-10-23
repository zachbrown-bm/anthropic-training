use colored::Colorize;
use std::io::Write;
use turboclaude::{Client, Message, MessageParam, MessageRequest};

const CLAUDE_MODEL: &str = "claude-sonnet-5-5";
const MAX_TOKENS: u32 = 1024;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to CHAT WORLD!!!");
    println!("Initializing...");

    // load env variables
    dotenvy::dotenv().ok();

    // get api key from env file.
    let api_key = dotenvy::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY must be set in the .env file (or environment).");

    // initialize client
    let client = Client::new(api_key);
    let mut message_context: Vec<MessageParam> = vec![];

    // let console know we're initialized
    println!("Initialized! Ready to start chatting.");

    loop {
        let mut user_input = String::new();
        print!("> ");
        std::io::stdout().flush()?;
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        if user_input.trim() == "quit" {
            break;
        }

        // user hasn't typed quit, let's take their input and do something with it.
        let result = send_message(&client, user_input, &mut message_context).await?;
        println!("Result: {}", format_bold(&result));
    }

    Ok(())
}

async fn send_message(
    client: &Client,
    message: String,
    message_context: &mut Vec<MessageParam>,
) -> Result<String, Box<dyn std::error::Error>> {
    message_context.push(Message::user(message));

    let request = MessageRequest::builder()
        .model(CLAUDE_MODEL)
        .max_tokens(MAX_TOKENS)
        .messages(message_context.clone())
        .build()?;

    let response = client.messages().create(request).await?;

    let text = response.text();
    message_context.push(Message::assistant(&text));

    Ok(text)
}

fn format_bold(input: &str) -> String {
    let mut result = String::new();
    let parts: Vec<&str> = input.split("**").collect();
    let is_even_parts = parts.len() % 2 == 0;

    for (i, part) in parts.iter().enumerate() {
        if i % 2 == 1 && (!is_even_parts || i != parts.len() - 1) {
            result.push_str(&part.bold().to_string());
        } else if is_even_parts && i == parts.len() - 1 {
            result.push_str("**");
            result.push_str(part);
        } else {
            result.push_str(part);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bold() {
        let input = "Hello **world**, this is **bold** text.";
        let output = format_bold(input);
        assert_eq!(
            output,
            format!("Hello {}, this is {} text.", "world".bold(), "bold".bold())
        );
    }

    #[test]
    fn test_format_bold_unclosed() {
        let input = "Hello **world without close";
        let output = format_bold(input);
        assert_eq!(output, "Hello **world without close");
    }
}

use std::env;
use chatgpt::{prelude::*, types::CompletionResponse};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "chatgpt=debug");
    }
    if env::var("CHATGPT_KEY").ok().is_none() {
        panic!("CHATGPT_KEY not set in environment variables");
    }

    let key = env::var("CHATGPT_KEY").unwrap();
    let client = ChatGPT::new(key)?;

    loop {
        print!("Please enter a command (type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            println!("Exiting...");
            break;
        }

        let response: CompletionResponse = client
         .send_message(input)
         .await?;

        println!("response: {}", response.message().content);
    }
    Ok(())
}
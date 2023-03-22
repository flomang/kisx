use chatgpt::{prelude::*, types::CompletionResponse};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use std::{env, time::Duration};

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

        let pb = ProgressBar::new_spinner();

        pb.set_style(
            ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
                .unwrap()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
        );
        pb.set_message("Processing...");
        pb.enable_steady_tick(Duration::from_millis(50));
        pb.tick();

        let response: CompletionResponse = client.send_message(input).await?;
        let response = response.message().content.clone();
        let wrapped = textwrap::wrap(&response, 100);

        pb.finish_and_clear();

        print!("{}", style("Response:").green());
        for line in wrapped {
            println!("{}", style(line).green());
        }
    }
    Ok(())
}

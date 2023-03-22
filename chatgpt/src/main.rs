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
    let mut conversation: Conversation = client.new_conversation();

    print!("Please enter a previous saved conversation (type 'new' for new conversation): ");
    io::stdout().flush().unwrap();

    let mut convo = String::new();
    io::stdin().read_line(&mut convo).unwrap();

    if convo.trim() != "new" {
        convo = format!("{}.json", convo.trim());
        conversation = client.restore_conversation_json(convo).await?
    } 

    loop {
        print!("Please enter a command (type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            print!("{}", style("Save conversation before exiting? (y / n) ").red());
            io::stdout().flush().unwrap();

            let mut save = String::new();
            io::stdin().read_line(&mut save).unwrap();

            if save.trim() == "y" {
                print!("{}", style("Enter conversation name: "));
                io::stdout().flush().unwrap();

                let mut file_name = String::new();
                io::stdin().read_line(&mut file_name).unwrap();

                file_name = format!("{}.json", file_name.trim());
                conversation.save_history_json(file_name).await?;
            }

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

        //let response: CompletionResponse = client.send_message(input).await?;
        let response: CompletionResponse = conversation.send_message(input).await?;
        let response = response.message().content.clone();
        let wrapped = textwrap::wrap(&response, 100);

        pb.finish_and_clear();

        println!("{}", style("Response:").green());
        for line in wrapped {
            println!("{}", style(line).green());
        }
    }
    Ok(())
}

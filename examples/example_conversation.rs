use ollie_rs::session::OllamaSession;
use std::io::{self, Write};
// use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    const MODEL: &str = "gemma3:4b";
    const CONTEXT_WINDOW_SIZE: u32 = 24 * 1024;

    let mut session1 = OllamaSession::new(MODEL, None);
    let mut session2 = OllamaSession::new(MODEL, None);
    session1.set_context_window_size(CONTEXT_WINDOW_SIZE);
    session2.set_context_window_size(CONTEXT_WINDOW_SIZE);

    let mut response1 = String::new();
    let mut response2 = "Hello there! How are you doing today?".to_string();

    loop {
        println!("\n\n ----- BOB:\n");

        response1.clear();
        session1.user(&response2);
        let stats1 = session1
            .update(|response| {
                print!("{}", response);
                io::stdout().flush().unwrap();
                response1.push_str(&response);
            })
            .await
            .unwrap();

        stats1.map(|stats| {
            println!(
                "\n\n *** STATS: {} tokens used of {}",
                stats.tokens_used(),
                session1.context_window_size()
            );
        });

        // Sleep for 10 seconds
        // sleep(Duration::from_secs(10)).await;

        println!("\n\n ----- FRED:\n");

        response2.clear();
        session2.user(&response1);
        let stats2 = session2
            .update(|response| {
                print!("{}", response);
                io::stdout().flush().unwrap();
                response2.push_str(&response);
            })
            .await
            .unwrap();

        stats2.map(|stats| {
            println!(
                "\n\n *** STATS: {} tokens used of {}",
                stats.tokens_used(),
                session1.context_window_size()
            );
        });

        // Sleep for 10 seconds
        // sleep(Duration::from_secs(10)).await;
    }
}

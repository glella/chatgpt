use dotenv::dotenv;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use serde::Deserialize;
use serde_json::json;
use std::{env, error::Error, result};

#[tokio::main]
async fn main() -> result::Result<(), Box<dyn Error>> {
    dotenv().ok(); // load the enviroment variables from the .env file
    let open_ai_api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY not set");

    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;

    // #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .expect("Error writing history entry.");

                let query = line;
                let client = Client::new();
                let url = "https://api.openai.com/v1/chat/completions";

                let headers: HeaderMap<HeaderValue> = header::HeaderMap::from_iter(vec![
                    (header::CONTENT_TYPE, "application/json".parse().unwrap()),
                    (
                        header::AUTHORIZATION,
                        format!("Bearer {}", open_ai_api_key).parse().unwrap(),
                    ),
                ]);

                let body = json!(
                    {
                        "model":"gpt-3.5-turbo",
                        "messages":[{
                            "role":"user",
                            "content": query,
                        }]
                    }
                );

                let response: ApiResponse = client
                    .post(url)
                    .headers(headers)
                    .json(&body)
                    .send()
                    .await?
                    .json()
                    .await?;

                println!("{}", &response.choices[0].message.content);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    // #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt")
        .expect("Error writing history.txt file.");
    Ok(())
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

use std::env;
use telegram_bot::*;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), telegram_bot::Error> {
    {
        use dotenv;
        dotenv::dotenv().ok();
    }
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not found");
    let api = Api::new(token);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        match update.kind {
            UpdateKind::Message(message) => {
                // todo
            }
            _ => {
                // todo
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_load_env_file() {
        {
            use dotenv;
            dotenv::dotenv().ok();
        }
        let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not found");
        assert_eq!(token, "replace_me");
    }
}

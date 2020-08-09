use std::env;
use telegram_bot::*;
use tokio::stream::StreamExt;

trait NewWithProxy {
    fn with_proxy<T: AsRef<str>>(token: T, url: T) -> Api;
}

impl NewWithProxy for Api {
    fn with_proxy<T: AsRef<str>>(token: T, url: T) -> Api {
        use hyper_proxy::{Proxy, Intercept, ProxyConnector};
        use hyper_rustls::HttpsConnector;
        use telegram_bot::connector::hyper::HyperConnector;
        use hyper::Client;

        let proxy = Proxy::new(
            Intercept::All,
            url.as_ref().parse().unwrap(),
        );
        let connector =
            ProxyConnector::from_proxy(HttpsConnector::new(), proxy)
                .expect("cannot create ProxyConnector");
        let connector =
            Box::new(
                HyperConnector::new(
                    Client::builder().build(connector)
                )
            );

        Api::with_connector(token.as_ref().to_string(), connector)
    }
}

#[tokio::main]
async fn main() -> Result<(), telegram_bot::Error> {
    {
        use dotenv;
        dotenv::dotenv().ok();
    }
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not found");

    let api = match env::var("https_proxy") {
        Ok(url) => {
            Api::with_proxy(token, url)
        }
        Err(_) => {
            Api::new(token)
        }
    };

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        match update.kind {
            UpdateKind::Message(message) => {
                println!("{}", message.text().expect("not a text"))
            }
            _ => {
                // todo
                println!("nothing")
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

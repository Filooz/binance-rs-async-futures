use crate::client::*;
use crate::errors::*;
use crate::rest_model::Success;
use crate::rest_model::UserDataStream;

static USER_DATA_STREAM: &str = "/dapi/v1/listenKey";

#[derive(Clone)]
pub struct UserStream {
    pub client: Client,
    pub recv_window: u64,
}

impl UserStream {
    pub async fn start(&self) -> Result<UserDataStream> { self.client.post(USER_DATA_STREAM, None).await }

    pub async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        self.client.put(USER_DATA_STREAM, listen_key, None).await
    }

    pub async fn close(&self, listen_key: &str) -> Result<Success> {
        self.client.delete(USER_DATA_STREAM, listen_key, None).await
    }
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;

    use super::*;
    use crate::{api::Binance, config::Config};

    #[tokio::test]
    async fn test_userstream_start() {
        dotenv().ok();

        let userstream: UserStream = Binance::new_with_env(&Config::default());
        let start = userstream.start().await;
        println!("{:?}", start);
        assert!(start.is_ok());
        let key = start.unwrap().listen_key.clone();

        let keep_alive = userstream.keep_alive(&key).await;
        println!("{:?}", keep_alive);
        assert!(keep_alive.is_ok());

        let close = userstream.close(&key).await;
        println!("{:?}", close);
        assert!(close.is_ok());
    }
}

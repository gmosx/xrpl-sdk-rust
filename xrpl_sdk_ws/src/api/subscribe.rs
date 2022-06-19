use crate::{client::Client, util::format_joined_keys, Result};

impl Client {
    pub async fn subscribe_account(&mut self, account: &str) -> Result<()> {
        let msg =
            format!("{{\"id\": 1, \"command\": \"subscribe\", \"accounts\": [\"{account}\"]}}");
        self.send(&msg).await?;
        Ok(())
    }

    pub async fn subscribe_accounts(&mut self, accounts: &[&str]) -> Result<()> {
        let accounts = format_joined_keys(accounts);
        let msg = format!("{{\"id\": 1, \"command\": \"subscribe\", \"accounts\": [{accounts}]}}");
        self.send(&msg).await?;
        Ok(())
    }

    // #TODO consider renaming to `subscribe_topic`
    pub async fn subscribe_stream(&mut self, stream: &str) -> Result<()> {
        let msg = format!("{{\"id\": 1, \"command\": \"subscribe\", \"streams\": [\"{stream}\"]}}");
        self.send(&msg).await?;
        Ok(())
    }

    // #TODO consider renaming to `subscribe_topics`
    pub async fn subscribe_streams(&mut self, streams: &[&str]) -> Result<()> {
        let streams = format_joined_keys(streams);
        let msg = format!("{{\"id\": 1, \"command\": \"subscribe\", \"streams\": [{streams}]}}");
        self.send(&msg).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::DEFAULT_WS_URL;
    use futures_util::{SinkExt, StreamExt};

    #[test]
    fn client_should_subscribe() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let mut client = Client::connect(DEFAULT_WS_URL)
                .await
                .expect("cannot connect");

            client
                .subscribe_streams(&["ledger"])
                .await
                .expect("cannot subscribe");

            let (_, rx) = client.stream.split();

            tokio::pin!(rx);

            while let Some(msg) = rx.next().await {
                dbg!(&msg);
            }
        });
    }
}

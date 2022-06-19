use crate::{client::Client, util::format_joined_keys, Result};

impl Client {
    pub async fn unsubscribe_accounts(&mut self, accounts: &[&str]) -> Result<()> {
        let accounts = format_joined_keys(accounts);
        let msg =
            format!("{{\"id\": 1, \"command\": \"unsubscribe\", \"accounts\": [{accounts}]}}");
        self.send(&msg).await?;
        Ok(())
    }

    pub async fn unsubscribe_streams(&mut self, streams: &[&str]) -> Result<()> {
        let streams = format_joined_keys(streams);
        let msg = format!("{{\"id\": 1, \"command\": \"unsubscribe\", \"streams\": [{streams}]}}");
        self.send(&msg).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn client_should_unsubscribe() {
        // #TODO
    }
}

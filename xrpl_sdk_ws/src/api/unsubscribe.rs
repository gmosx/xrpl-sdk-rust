//! https://xrpl.org/unsubscribe.html

use crate::{client::Client, util::format_joined_keys, util::Result};

impl Client {
    pub async fn unsubscribe_accounts(&mut self, accounts: &[&str]) -> Result<()> {
        let id = self.next_id();
        let accounts = format_joined_keys(accounts);
        let msg = format!(
            "{{\"id\": \"{id}\", \"command\": \"unsubscribe\", \"accounts\": [{accounts}]}}"
        );
        self.send(&msg).await?;
        Ok(())
    }

    pub async fn unsubscribe_streams(&mut self, streams: &[&str]) -> Result<()> {
        let id = self.next_id();
        let streams = format_joined_keys(streams);
        let msg =
            format!("{{\"id\": \"{id}\", \"command\": \"unsubscribe\", \"streams\": [{streams}]}}");
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

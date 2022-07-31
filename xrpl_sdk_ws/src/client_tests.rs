#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::client::DEFAULT_WS_URL;
    use futures_util::StreamExt;
    use xrpl_api::{AccountInfoRequest, SubscribeRequest};

    #[tokio::test]
    async fn client_can_request_account_info() {
        let mut client = Client::connect(DEFAULT_WS_URL)
            .await
            .expect("cannot connect");

        let req = AccountInfoRequest::new("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59").strict(true);

        client.send(req).await.expect("cannot send request");

        let (_, rx) = client.stream.split();

        tokio::pin!(rx);

        while let Some(msg) = rx.next().await {
            dbg!(&msg);
        }
    }

    #[tokio::test]
    async fn client_can_subscribe_to_streams() {
        let mut client = Client::connect(DEFAULT_WS_URL)
            .await
            .expect("cannot connect");

        let req = SubscribeRequest::streams(&["ledger"]);
        client.send(req).await.expect("cannot subscribe");

        let (_, rx) = client.stream.split();

        tokio::pin!(rx);

        while let Some(msg) = rx.next().await {
            dbg!(&msg);
        }
    }
}

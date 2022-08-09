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

        client.call(req).await.expect("cannot send request");

        let (_, receiver) = client.split();

        tokio::pin!(receiver);

        if let Some(msg) = receiver.next().await {
            dbg!(&msg);
        }
    }

    #[tokio::test]
    async fn client_can_subscribe_to_streams() {
        let mut client = Client::connect(DEFAULT_WS_URL)
            .await
            .expect("cannot connect");

        let req = SubscribeRequest::streams(&["ledger"]);
        client.call(req).await.expect("cannot subscribe");

        let (_, receiver) = client.split();

        tokio::pin!(receiver);

        while let Some(msg) = receiver.next().await {
            dbg!(&msg);
        }
    }
}

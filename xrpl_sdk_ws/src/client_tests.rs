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

        if let Some(msg) = client.messages.next().await {
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

        let mut count = 0;

        while let Some(msg) = client.messages.next().await {
            if count > 2 {
                break;
            }
            dbg!(&msg);
            count += 1;
        }
    }
}

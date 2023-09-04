#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::client::DEFAULT_WS_URL;
    use futures_util::StreamExt;
    use xrpl_api::{AccountInfoRequest, Book, Currency, SubscribeRequest};

    #[tokio::test]
    async fn client_can_request_account_info() {
        let mut client = Client::connect(DEFAULT_WS_URL)
            .await
            .expect("cannot connect");

        let req = AccountInfoRequest::new("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59");

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

        let req = SubscribeRequest::streams(vec!["ledger".to_string()]);

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

    #[tokio::test]
    async fn client_can_subscribe_to_books() {
        let mut client = Client::connect(DEFAULT_WS_URL)
            .await
            .expect("cannot connect");

        let book = Book::new(
            Currency::Xrp,
            Currency::issued("USD", "rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"),
        )
        .taker("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn")
        .snapshot(true);

        let req = SubscribeRequest::books(vec![book]);

        client.call(req).await.expect("cannot subscribe");

        let mut count = 0;

        while let Some(msg) = client.messages.next().await {
            if count > 1 {
                break;
            }
            dbg!(&msg);
            count += 1;
        }
    }
}

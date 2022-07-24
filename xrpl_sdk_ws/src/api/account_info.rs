// //! - https://xrpl.org/account_info.html

// // use xrpl_api::AccountInfoRequest;

// // use crate::{client::Client, util::Result};

// // impl Client {
// //     // #TODO implement a builder, for the additional params, like in JSONRPC client.
// //     pub async fn account_info(&mut self, account: &str) -> Result<()> {
// //         let id = self.next_id();

// //         let payload = AccountInfoRequest::new(account);
// //         let payload = serde_json::to_value(&payload).unwrap(); // #TODO use `?`.

// //         // #TODO, this is temp code, add error-handling!

// //         if let serde_json::Value::Object(mut map) = payload {
// //             map.insert("id".to_owned(), serde_json::Value::String(id));
// //             map.insert(
// //                 "command".to_owned(),
// //                 serde_json::Value::String("account_info".to_owned()),
// //             );
// //             let msg = serde_json::to_string(&map).unwrap();

// //             self.send(&msg).await?;
// //         }

// //         Ok(())
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::client::DEFAULT_WS_URL;
//     use futures_util::{SinkExt, StreamExt};

//     #[tokio::test]
//     async fn client_can_request_account_info() {
//         let mut client = Client::connect(DEFAULT_WS_URL)
//             .await
//             .expect("cannot connect");

//         let req = AccountInfoRequest::new("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59").strict(true);

//         // client
//         //     .account_info("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59")
//         //     .await
//         //     .expect("cannot subscribe");

//         client.send2(req).await.expect("cannot send request");

//         let (_, rx) = client.stream.split();

//         tokio::pin!(rx);

//         while let Some(msg) = rx.next().await {
//             dbg!(&msg);
//         }
//     }
// }

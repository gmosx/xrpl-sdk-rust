// //! - https://xrpl.org/account_info.html

// // use crate::{client::RpcRequest, Client, Result};
// // use serde::de::DeserializeOwned;
// // use xrpl_api::{AccountInfoRequestPayload, AccountInfoResponsePayload};

// // #[must_use = "Does nothing until you send or execute it"]
// // #[derive(Default, Clone)]
// // pub struct AccountInfoRequest {
// //     client: Client,
// //     params: AccountInfoRequestPayload,
// // }

// // impl AccountInfoRequest {
// //     pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
// //         let request = RpcRequest {
// //             method: "account_info".to_string(),
// //             params: vec![self.params],
// //         };
// //         self.client
// //             .send::<AccountInfoRequestPayload, T>(request)
// //             .await
// //     }

// //     pub async fn send(self) -> Result<AccountInfoResponsePayload> {
// //         self.execute().await
// //     }
// // }

// // impl Client {
// //     pub fn account_info(&self, account: &str) -> AccountInfoRequest {
// //         AccountInfoRequest {
// //             client: self.clone(),
// //             params: AccountInfoRequestPayload {
// //                 account: account.to_string(),
// //                 queue: None,
// //                 ledger_hash: None,
// //                 ledger_index: None,
// //                 signer_lists: None,
// //                 strict: None,
// //             },
// //         }
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use xrpl_api::AccountInfoRequest;

//     use crate::client::Client;

//     #[tokio::test]
//     async fn account_info_returns_info() {
//         let client = Client::default();

//         let req = AccountInfoRequest::new("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59").strict(true);

//         // let resp = client
//         //     .account_info("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59")
//         //     .send()
//         //     .await;

//         let resp = client.send2(req).await;

//         dbg!(&resp);
//     }
// }

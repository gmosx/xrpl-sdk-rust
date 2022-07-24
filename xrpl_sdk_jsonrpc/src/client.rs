use crate::error::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::debug;
use xrpl_api::{AccountInfoRequest, Request};
use xrpl_types::Transaction;

pub const GENERAL_PURPOSE_MAINNET_URL: &str = "https://s1.ripple.com:51234";
pub const FULL_HISTORY_MAINNET_URL: &str = "https://s2.ripple.com:51234";
pub const TESTNET_URL: &str = "https://s.altnet.rippletest.net:51234";
pub const DEVNET_URL: &str = "https://s.devnet.rippletest.net:51234";

pub const DEFAULT_BASE_URL: &str = GENERAL_PURPOSE_MAINNET_URL;

pub const DEFAULT_USER_AGENT: &str = "rust-xrpl-sdk-rippled-client/0.1.0";

pub type Result<T> = std::result::Result<T, Error>;

// TODO: add constructors for TESTNET and DEVNET.

#[derive(Serialize)]
pub struct RpcRequest<P: Serialize> {
    pub method: String,
    pub params: Vec<P>,
}

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub result: T,
}

// #[derive(Error, Debug)]
// pub enum RpcError {
//     #[error("network error")]
//     NetworkError,
//     #[error("api error")]
//     ApiError(String, String),
// }

#[derive(Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    user_agent: Option<String>,
    http_client: Option<reqwest::Client>,
}

impl ClientBuilder {
    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = Some(base_url.to_string());
        self
    }

    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    pub fn http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn build(self) -> Client {
        Client {
            base_url: self
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            user_agent: self
                .user_agent
                .unwrap_or_else(|| DEFAULT_USER_AGENT.to_string()),
            http_client: self.http_client.unwrap_or_else(reqwest::Client::new),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    user_agent: String,
    // TODO: hm, not really used currently.
    http_client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Sends a JSON-RPC request to rippled.
    pub async fn send<Params, Resp>(&self, request: RpcRequest<Params>) -> Result<Resp>
    where
        Params: Serialize,
        Resp: DeserializeOwned,
    {
        // #TODO: remove the unwrap!
        let body = serde_json::to_string(&request).unwrap();

        debug!("POST {}", body);

        let response = self
            .http_client
            .post(&self.base_url)
            .body(body)
            .header(reqwest::header::USER_AGENT, &self.user_agent)
            .send()
            .await?;

        self.unwrap_response(response).await
    }

    pub async fn send2<Req>(&self, request: Req) -> Result<Req::Response>
    where
        Req: Request + Serialize,
        Req::Response: DeserializeOwned,
    {
        let request = RpcRequest {
            method: request.method(),
            params: vec![request],
        };

        // #TODO: remove the unwrap!
        let body = serde_json::to_string(&request).unwrap();

        debug!("POST {}", body);

        let response = self
            .http_client
            .post(&self.base_url)
            .body(body)
            .header(reqwest::header::USER_AGENT, &self.user_agent)
            .send()
            .await?;

        self.unwrap_response(response).await
    }

    async fn unwrap_response<Resp>(&self, response: reqwest::Response) -> Result<Resp>
    where
        Resp: DeserializeOwned,
    {
        if response.status() == 200 {
            // TODO: add an option to show diagnostics?
            // eprintln!("--> {}", response.text().await?);
            // panic!();
            let body: RpcResponse<Resp> = response.json().await?;
            Ok(body.result)
        } else {
            // TODO: Add proper error handling!
            panic!()
        }
    }

    /// Prepares a transaction for signing and reliable submission by
    /// auto-filling required fields.
    ///
    /// ## Links
    ///
    /// - https://xrpl.org/reliable-transaction-submission.html
    pub async fn prepare_transaction(&self, tx: Transaction) -> Result<Transaction> {
        let mut tx = tx;

        if tx.sequence.is_none() {
            let req = AccountInfoRequest::new(&tx.account);
            let resp = self.send2(req).await?;

            tx.sequence = Some(resp.account_data.sequence);
        }

        if tx.last_ledger_sequence.is_none() || tx.fee.is_none() {
            let resp = self.server_state().send().await?;

            // The recommendation for backend applications is to use (last validated ledger index + 4).
            tx.last_ledger_sequence = Some(resp.state.validated_ledger.seq + 4);
            tx.fee = Some(resp.state.validated_ledger.base_fee);
        }

        Ok(tx)
    }

    // TODO: local_sign in external package!
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use xrpl_api::{AccountCurrenciesRequest, AccountInfoRequest};

    #[tokio::test]
    async fn client_can_fetch_account_currencies() {
        let client = Client::default();

        let resp = client
            .send2(AccountCurrenciesRequest::new(
                "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_account_info() {
        let client = Client::default();

        let req = AccountInfoRequest::new("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59").strict(true);
        let resp = client.send2(req).await;

        dbg!(&resp);
    }
}

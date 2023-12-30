use crate::error::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;
use tracing::debug;
use xrpl_api::{AccountInfoRequest, Request, ServerInfoRequest};
use xrpl_types::{DropsAmount, TransactionCommon};

pub const GENERAL_PURPOSE_MAINNET_URL: &str = "https://s1.ripple.com:51234";
pub const FULL_HISTORY_MAINNET_URL: &str = "https://s2.ripple.com:51234";
pub const TESTNET_URL: &str = "https://s.altnet.rippletest.net:51234";
pub const DEVNET_URL: &str = "https://s.devnet.rippletest.net:51234";
pub const NFT_DEVNET_URL: &str = "http://xls20-sandbox.rippletest.net:51234";
pub const HOOKS_TESTNET_V2_URL: &str = "?"; // #TODO

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

const DEFAULT_BASE_URL: &str = GENERAL_PURPOSE_MAINNET_URL;

const DEFAULT_USER_AGENT: &str = "rust-xrpl-sdk-rippled-client/0.1.0";

pub type Result<T> = std::result::Result<T, Error>;

// #TODO add constructors for TESTNET and DEVNET.

#[derive(Serialize)]
pub struct RpcRequest<P: Serialize> {
    pub method: String,
    pub params: Vec<P>,
}

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub result: T,
}

#[derive(Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    user_agent: Option<String>,
    http_client: Option<reqwest::Client>,
    timeout: Option<Duration>,
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

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Client {
        // #TODO handle the unwrap
        Client {
            base_url: self
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            user_agent: self
                .user_agent
                .unwrap_or_else(|| DEFAULT_USER_AGENT.to_string()),
            http_client: self.http_client.unwrap_or_else(|| {
                reqwest::Client::builder()
                    .timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
                    .build()
                    .unwrap()
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    user_agent: String,
    // #todo hm, not really used currently.
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

    pub async fn call<Req>(&self, request: Req) -> Result<Req::Response>
    where
        Req: Request + Serialize,
        Req::Response: DeserializeOwned,
    {
        let request = RpcRequest {
            method: request.method(),
            params: vec![request],
        };

        let body = serde_json::to_string(&request)?;

        debug!("POST {}", body);

        let response = self
            .http_client
            .post(&self.base_url)
            .body(body)
            .header(reqwest::header::USER_AGENT, &self.user_agent)
            .send()
            .await?;

        self.parse_response(response).await
    }

    async fn parse_response<Resp>(&self, response: reqwest::Response) -> Result<Resp>
    where
        Resp: DeserializeOwned,
    {
        if response.status() == 200 {
            let body: serde_json::Value = response.json().await?;

            let status = body["result"]["status"].as_str().unwrap_or("error");

            if status == "error" {
                // dbg!(&body);
                debug!("{}", body);

                return Err(Error::Api(
                    body["result"]["error"]
                        .as_str()
                        .unwrap_or_default()
                        .to_owned(),
                ));
            }

            // dbg!(&body);
            match serde_json::from_value::<RpcResponse<Resp>>(body) {
                Ok(body) => Ok(body.result),
                Err(err) => {
                    // #TODO add an option to show diagnostics?
                    Err(Error::Format(err.to_string()))
                }
            }
        } else {
            Err(Error::Api(format!(
                "Status {}: {}",
                response.status(),
                response.text().await?
            )))
        }
    }

    /// Prepares a transaction for signing and reliable submission by
    /// auto-filling required fields.
    ///
    /// <https://xrpl.org/reliable-transaction-submission.html>
    pub async fn prepare_transaction(&self, tx: &mut TransactionCommon) -> Result<()> {
        if tx.sequence.is_none() {
            let resp = self
                .call(AccountInfoRequest::new(&tx.account.to_address()))
                .await?;

            tx.sequence = Some(resp.account_data.sequence);
        }

        if tx.last_ledger_sequence.is_none() || tx.fee.is_none() {
            // let resp = self.call(ServerStateRequest::new()).await?;
            //
            // // The recommendation for backend applications is to use (last validated ledger index + 4).
            // tx.last_ledger_sequence = Some(resp.state.validated_ledger.seq + 4);
            // tx.fee = Some(resp.state.validated_ledger.base_fee);

            let resp = self.call(ServerInfoRequest::new()).await?;

            // The recommendation for backend applications is to use (last validated ledger index + 4).
            tx.last_ledger_sequence = Some(resp.info.validated_ledger.seq + 4);
            tx.fee = Some(
                DropsAmount::from_drops(
                    (resp.info.validated_ledger.base_fee_xrp * 1_000_000.0) as u64,
                )
                .map_err(|err| Error::Internal(format!("Not a valid drops value: {}", err)))?,
            );
        }

        Ok(())
    }

    // #TODO add additional helpers, like .submit(), and other requests with standard params.
    // #TODO local_sign in external package!
}

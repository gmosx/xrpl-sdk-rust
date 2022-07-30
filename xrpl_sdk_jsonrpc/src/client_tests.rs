#[cfg(test)]
mod tests {
    use crate::client::Client;
    use xrpl_api::{
        AccountCurrenciesRequest, AccountInfoRequest, AccountLinesRequest, AccountOffersRequest,
        AccountTxRequest, BookOffersRequest, DepositAuthorizedRequest, FeeRequest,
        GatewayBalancesRequest, GetOfferObjectRequest, LedgerClosedRequest, LedgerCurrentRequest,
        LedgerEntryRequest, ManifestRequest, RandomRequest, ServerInfoRequest, ServerStateRequest,
        TransactionEntryRequest,
    };
    use xrpl_types::Currency;

    #[tokio::test]
    async fn client_can_fetch_account_currencies() {
        let client = Client::default();

        let resp = client
            .send(AccountCurrenciesRequest::new(
                "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_account_info() {
        let client = Client::default();

        let req = AccountInfoRequest::new("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59").strict(true);
        let resp = client.send(req).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_account_lines() {
        let client = Client::default();

        let resp = client
            .send(AccountLinesRequest::new(
                "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_account_offers() {
        let client = Client::default();

        let resp = client
            .send(AccountOffersRequest::new(
                "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_account_transactions() {
        let client = Client::default();

        let req = AccountTxRequest::new("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59").limit(5);
        let resp = client.send(req).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_gateway_balances() {
        let client = Client::default();

        let resp = client
            .send(GatewayBalancesRequest::new(
                "rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_info_about_the_last_closed_ledger() {
        let client = Client::default();

        let resp = client.send(LedgerClosedRequest::new()).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_info_about_the_current_ledger() {
        let client = Client::default();

        let resp = client.send(LedgerCurrentRequest::new()).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_ledger_entries() {
        let client = Client::default();

        let resp = client
            .send(LedgerEntryRequest::new(
                "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                359,
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_get_an_offer_object() {
        let client = Client::default();

        let resp = client
            .send(GetOfferObjectRequest::new(
                "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                359,
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_book_offers() {
        let client = Client::default();

        let resp = client
            .send(BookOffersRequest::new(
                &Currency::xrp(),
                &Currency::issued("USD", "rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"),
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_check_if_deposit_is_authorized() {
        let client = Client::default();

        let resp = client
            .send(DepositAuthorizedRequest::new(
                "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
                "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_transaction_entries() {
        let client = Client::default();

        let resp = client
            .send(
                TransactionEntryRequest::new(
                    "DA86C7F1979A010BB5F54C49116697A44D8088F92C9AA3AAE419136FE8275A10",
                )
                .ledger_index("73355924"),
            )
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_manifests() {
        let client = Client::default();

        let resp = client
            .send(ManifestRequest::new(
                "nHUE7npJuqdYxFL93tGZS7CW9DuWNLAxBVjzc2rEbu65eL4iiA6s",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_fees() {
        let client = Client::default();

        let resp = client.send(FeeRequest::new()).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_information_about_the_server() {
        let client = Client::default();

        let resp = client.send(ServerInfoRequest::new()).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_the_server_state() {
        let client = Client::default();

        let resp = client.send(ServerStateRequest::new()).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_a_random_seed() {
        let client = Client::default();

        let resp = client.send(RandomRequest::new()).await;

        dbg!(&resp);
    }
}

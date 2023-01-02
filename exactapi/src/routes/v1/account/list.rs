use std::fmt::Debug;
use actix_multiresponse::Payload;
use actix_web::web;
use mrauth::actix::BearerHeader;
use mrauth::User;
use serde::Deserialize;
use tracing::instrument;
use exact_filter::{Filter, FilterOp};
use exact_requests::accounts::AccountFilterOptions;
use exact_requests::Api;
use proto::ListAccountResponse;
use crate::{AuthData, ExactAuthData};
use crate::error::WebResult;

#[derive(Debug, Deserialize)]
pub struct Query {
    address_line_1: Option<String>,
    city: Option<String>,
    name: Option<String>,
    kvk: Option<String>,
    /// When true: AND, when false: OR
    /// Defaulting to 'AND' (true)
    and_mode: Option<bool>
}

const SCOPE: &str = "nl.mrfriendly.exact.token nl.mrfriendly.exact.account.list";

#[instrument(skip(auth, eauth, bearer))]
pub async fn list(auth: AuthData, eauth: ExactAuthData, bearer: BearerHeader, query: web::Query<Query>) -> WebResult<Payload<ListAccountResponse>>{
    User::get_user(&auth, &bearer, SCOPE).await?;
    let and_mode = query.and_mode.unwrap_or(true);

    let mut filter = None;

    if let Some(name) = &query.name {
        filter = Some(Filter::new(AccountFilterOptions::Name, &name, FilterOp::Equals));
    }

    if let Some(city) = &query.city {
        let city_filter = Filter::new(AccountFilterOptions::City, &city, FilterOp::Equals);
        filter = Some(set_filter(filter, city_filter, and_mode));
    }

    if let Some(address_line_1) = &query.address_line_1 {
        let address_filter = Filter::new(AccountFilterOptions::AddressLine1, &address_line_1, FilterOp::Equals);
        filter = Some(set_filter(filter, address_filter, and_mode));
    }

    if let Some(kvk) = &query.kvk {
        let kvk_filter = Filter::new(AccountFilterOptions::ChamberOfCommerce, &kvk, FilterOp::Equals);
        filter = Some(set_filter(filter, kvk_filter, and_mode));
    }

    let access_token = eauth.get_exact_access_token(&bearer).await?;
    let api = Api::new(access_token.token);
    let division = api.get_current_accounting_division().await?;
    let accounts = api.list_accounts(filter, division)
        .await?;

    let accounts = accounts.into_iter()
        .map(|x| crate::exact_api_conversion::account::account_to_proto(x))
        .collect::<Vec<_>>();

    Ok(Payload(ListAccountResponse {
        size: accounts.len() as i64,
        accounts
    }))
}

fn set_filter<T: ToString + Debug>(existing_filter: Option<Filter<T>>, new_filter: Filter<T>, and_mode: bool) -> Filter<T> {
    match existing_filter {
        Some(x) if and_mode => x.join_and(&new_filter),
        Some(x) => x.join_or(&new_filter),
        None => new_filter
    }
}
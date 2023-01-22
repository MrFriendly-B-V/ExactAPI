use std::fmt::Debug;
use actix_multiresponse::Payload;
use actix_web::web;
use mrauth::actix::BearerHeader;
use mrauth::User;
use serde::Deserialize;
use tracing::instrument;
use exact_filter::{Filter, FilterOp, Guid};
use exact_requests::accounts::AccountFilterOptions;
use exact_requests::Api;
use proto::ListAccountResponse;
use crate::{AccountCache, AuthData, ExactAuthData};
use crate::error::WebResult;

#[derive(Debug, Deserialize)]
pub struct Query {
    id: Option<String>,
    /// Default: False
    bypass_cache: Option<bool>,
}

const SCOPE: &str = "nl.mrfriendly.exact.read nl.mrfriendly.exact";

#[instrument(skip(auth, eauth, bearer))]
pub async fn list(auth: AuthData, eauth: ExactAuthData, bearer: BearerHeader, query: web::Query<Query>, account_cache: AccountCache) -> WebResult<Payload<ListAccountResponse>>{
    User::get_user(&auth, &bearer, SCOPE).await?;
    let bypass_cache = query.bypass_cache.unwrap_or(false);

    if !bypass_cache && account_cache.weighted_size() != 0 {
        if let Some(id) = &query.id {
            if let Some(account) = account_cache.get(id) {
                return Ok(Payload(ListAccountResponse {
                    size: 1,
                    accounts: vec![account]
                }))
            }
        } else {
            let accounts = account_cache.iter().map(|(_id, account)| account).collect::<Vec<_>>();
            return Ok(Payload(ListAccountResponse {
                size: accounts.len() as i64,
                accounts
            }))
        }
    }

    let mut filter = None;

    if let Some(id) = &query.id {
        filter = Some(Filter::new(AccountFilterOptions::Id, Guid::new(id), FilterOp::Equals));
    }

    let access_token = eauth.get_exact_access_token(&bearer).await?;
    let api = Api::new(access_token.token);
    let division = api.get_current_accounting_division().await?;
    let accounts = api.list_accounts(filter, division)
        .await?;

    let accounts = accounts.into_iter()
        .map(|x| crate::exact_api_conversion::account::account_to_proto(x))
        .collect::<Vec<_>>();

    for account in &accounts {
        account_cache.insert(account.id.clone(), account.clone()).await
    }

    Ok(Payload(ListAccountResponse {
        size: accounts.len() as i64,
        accounts
    }))
}
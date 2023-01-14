use actix_multiresponse::Payload;
use actix_web::web;
use mrauth::actix::BearerHeader;
use mrauth::User;
use serde::Deserialize;
use exact_filter::{Filter, FilterOp, Guid};
use exact_requests::Api;
use exact_requests::contact::ContactFilterOptions;
use proto::ListContactResponse;
use crate::{AuthData, ExactAuthData};
use crate::error::WebResult;
use crate::routes::v1::set_filter;

#[derive(Debug, Deserialize)]
pub struct Query {
    id: Option<String>,
    account: Option<String>,
    /// When true: AND, when false: OR
    /// Defaulting to 'AND' (true)
    and_mode: Option<bool>
}

const SCOPE: &str = "nl.mrfriendly.exact.read nl.mrfriendly.exact";

pub async fn list(auth: AuthData, eauth: ExactAuthData, bearer: BearerHeader, query: web::Query<Query>) -> WebResult<Payload<ListContactResponse>> {
    User::get_user(&auth, &bearer, SCOPE).await?;
    let and_mode = query.and_mode.unwrap_or(true);

    let mut filter = None;
    if let Some(id) = &query.id {
        filter = Some(Filter::new(ContactFilterOptions::Id, Guid::new(id), FilterOp::Equals));
    }

    if let Some(account) = &query.account {
        let account_filter = Filter::new(ContactFilterOptions::Account, Guid::new(account), FilterOp::Equals);
        filter = Some(set_filter(filter, account_filter, and_mode));
    }

    let access_token = eauth.get_exact_access_token(&bearer).await?;
    let api = Api::new(access_token.token);
    let division = api.get_current_accounting_division().await?;
    let contacts = api.list_contacts(filter, division).await?;

    let contacts = contacts.into_iter()
        .map(|x| crate::exact_api_conversion::contact::contact_to_proto(x))
        .collect::<Vec<_>>();

    Ok(Payload(ListContactResponse {
        size: contacts.len() as i64,
        contacts
    }))
}

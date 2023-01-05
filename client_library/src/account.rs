use reqwest_protobuf::{ProtobufRequestExt, ProtobufResponseExt};
use serde::Serialize;
use proto::ListAccountResponse;
use crate::{ExactApiClient, ExactApiError};
use derive_builder::Builder;

pub struct Account {
    pub id: String,
    pub name: String,
    pub kvk: Option<String>,
    pub code: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<AccountStatus>,
    pub vat_number: Option<String>,
    pub website: Option<String>,
    pub is_supplier: bool,
    pub address: Option<Address>,
}

pub struct Address {
    pub line1: String,
    pub city: String,
    pub postcode: String,
    pub country_name: String,
    pub country_code: String,
    pub state: String,
}

pub enum AccountStatus {
    Customer,
    Prospect,
    Suspect,
    None,
}

#[derive(Serialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountFilter {
    id: Option<String>,
    address_line_1: Option<String>,
    city: Option<String>,
    name: Option<String>,
    kvk: Option<String>,
    /// When true: AND, when false: OR
    /// Defaulting to 'AND' (true)
    and_mode: Option<bool>
}

impl ExactApiClient {
    pub async fn list_accounts(&self, mrauth_bearer: &str, filter: Option<AccountFilter>) -> Result<Vec<Account>, ExactApiError> {
        let query = if let Some(filter) = filter {
            let serialized = serde_qs::to_string(&filter)?;
            format!("?{serialized}")
        } else { String::default() };

        let response = self.client
            .get(self.get_url(&format!("/api/v1/account/list{query}")))
            .bearer_auth(mrauth_bearer)
            .accept_protobuf()
            .send()
            .await?;

        response.error_for_status_ref()?;

        let payload: ListAccountResponse = response.protobuf().await?;
        let accounts = payload.accounts.into_iter()
            .map(|x| Account {
                id: x.id,
                name: x.name,
                vat_number: x.vat_number,
                kvk: x.kvk,
                address: x.address.map(|x| Address {
                    line1: x.line1,
                    city: x.city,
                    country_name: x.country_name,
                    country_code: x.country_code,
                    state: x.state,
                    postcode: x.postcode
                }),
                is_supplier: x.is_supplier,
                code: x.code,
                email: x.email,
                phone: x.phone,
                website: x.website,
                status: x.status.map(|x| {
                    let status = proto::AccountStatus::from_i32(x).unwrap();
                    match status {
                        proto::AccountStatus::Suspect => AccountStatus::Suspect,
                        proto::AccountStatus::Customer => AccountStatus::Customer,
                        proto::AccountStatus::Prospect => AccountStatus::Prospect,
                        proto::AccountStatus::None => AccountStatus::None,
                    }
                })
            })
            .collect::<Vec<_>>();
        Ok(accounts)
    }
}
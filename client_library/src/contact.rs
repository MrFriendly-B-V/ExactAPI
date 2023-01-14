use derive_builder::Builder;
use reqwest_protobuf::{ProtobufRequestExt, ProtobufResponseExt};
use serde::Serialize;
use proto::ListContactResponse;
use crate::{ExactApiClient, ExactApiError};

pub struct Contact {
    pub id: String,
    pub account_id: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContactFilter {
    id: Option<String>,
    account: Option<String>,
    /// When true: AND, when false: OR
    /// Defaulting to 'AND' (true)
    and_mode: Option<bool>
}

impl ExactApiClient {
    pub async fn list_contacts(&self, mrauth_bearer: &str, filter: Option<ContactFilter>) -> Result<Vec<Contact>, ExactApiError> {
        let query = if let Some(filter) = filter {
            let serialized = serde_qs::to_string(&filter)?;
            format!("?{serialized}")
        } else { String::default() };

        let response = self.client
            .get(self.get_url(&format!("/api/v1/contact/list{query}")))
            .bearer_auth(mrauth_bearer)
            .accept_protobuf()
            .send()
            .await?;

        response.error_for_status_ref()?;

        let payload: ListContactResponse = response.protobuf().await?;
        let contacts = payload.contacts.into_iter()
            .map(|x| Contact {
                id: x.id,
                account_id: x.account,
                full_name: x.full_name,
                email: x.email,
                phone: x.phone
            })
            .collect::<Vec<_>>();
        Ok(contacts)
    }
}
use serde::Deserialize;
use strum_macros::Display;
use tracing::instrument;
use exact_filter::Filter;
use crate::{Api, ExactResult};

#[derive(Display, Debug, Deserialize)]
pub enum ContactFilterOptions {
    /// Guid
    #[strum(serialize = "ID")]
    #[serde(rename = "ID")]
    Id,
    /// Guid
    Account,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Contact {
    #[serde(rename = "ID")]
    pub id: String,
    pub account: String,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
}

impl Api {
    #[instrument(skip(self))]
    pub async fn list_contacts(&self, filter: Option<Filter<ContactFilterOptions>>, division: i32) -> ExactResult<Vec<Contact>> {
        let select = "$select=ID,Account,Email,FullName,Phone";
        let query = match filter {
            Some(filter) => {
                format!("{select}&$filter={}", filter.finalize())
            },
            None => select.to_string()
        };

        let response = self.get::<Contact>(&format!("/api/v1/{division}/crm/Contacts?{query}"))
            .await?;

        Ok(response)
    }
}
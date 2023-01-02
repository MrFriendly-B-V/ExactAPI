use serde::Deserialize;
use tracing::instrument;
use crate::{Api, ExactError, ExactResult};

impl Api {
    #[instrument(skip(self))]
    pub async fn get_current_accounting_division(&self) -> ExactResult<i32> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            current_division: i32
        }

        let response = self.get::<Response>("/api/v1/current/Me?$select=AccountingDivision,CurrentDivision")
            .await?;
        Ok(response.first()
            .ok_or(ExactError::NotFound)?
            .current_division
        )
    }
}
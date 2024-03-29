use async_recursion::async_recursion;
use serde::Deserialize;
use strum_macros::Display;
use tracing::instrument;
use exact_filter::Filter;
use crate::{Api, ExactResult};

#[derive(Display, Debug)]
pub enum AccountFilterOptions {
    #[strum(serialize = "ID")]
    Id,
    ChamberOfCommerce,
    Status,
    City,
    AddressLine1,
    Postcode,
    Name,
}

pub struct Account {
    pub id: String,
    pub name: String,
    pub address: Option<String>,
    pub chamber_of_commerce: Option<String>,
    pub city: Option<String>,
    pub code: Option<String>,
    pub country: Option<String>,
    pub country_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub postcode: Option<String>,
    pub state: Option<String>,
    pub status: Option<String>,
    pub vat_number: Option<String>,
    pub website: Option<String>,
    pub is_supplier: bool,
}

impl Api {
    #[instrument(skip(self))]
    pub async fn list_accounts(&self, filter: Option<Filter<AccountFilterOptions>>, division: i32) -> ExactResult<Vec<Account>> {
        let select = "$select=ID,AddressLine1,ChamberOfCommerce,City,Code,IsSupplier,Country,CountryName,Email,Name,Phone,Postcode,State,StateName,Status,VATNumber,Website";
        let query = match filter {
            Some(filter) => {
                format!("{select}&$filter={}", filter.finalize())
            },
            None => select.to_string()
        };

        let accounts = make_request(&self, &format!("/api/v1/{division}/crm/Accounts?{query}")).await?;
        Ok(accounts)
    }
}

#[async_recursion]
async fn make_request(api: &Api, url: &str) -> ExactResult<Vec<Account>> {
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Response {
        #[serde(rename = "ID")]
        id: String,
        name: String,
        address_line_1: Option<String>,
        chamber_of_commerce: Option<String>,
        city: Option<String>,
        code: Option<String>,
        country: Option<String>,
        country_name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        postcode: Option<String>,
        state: Option<String>,
        status: Option<String>,
        #[serde(rename = "VATNumber")]
        vat_number: Option<String>,
        website: Option<String>,
        is_supplier: bool,
    }

    let response = api.get::<Response>(url)
        .await?;

    let accounts = response.results.into_iter()
        .map(|x| Account {
            name: x.name,
            id: x.id,
            address: x.address_line_1,
            city: x.city,
            status: x.status,
            code: x.code,
            postcode: x.postcode,
            state: x.state,
            email: x.email,
            chamber_of_commerce: x.chamber_of_commerce,
            vat_number: x.vat_number,
            website: x.website,
            country: x.country,
            country_name: x.country_name,
            phone: x.phone,
            is_supplier: x.is_supplier,
        })
        .collect::<Vec<_>>();

    if let Some(next) = response.next {
        let mut buf = accounts;
        buf.extend(make_request(api, &next.replace("https://start.exactonline.nl", "")).await?);
        Ok(buf)
    } else {
        Ok(accounts)
    }
}
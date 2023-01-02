use exact_requests::accounts::Account;
use proto::AccountStatus;

pub fn account_to_proto(account: Account) -> proto::Account {
    let address = if let (Some(line1), Some(state), Some(country_code), Some(country_name), Some(city), Some(postcode)) =
        (account.address, account.state, account.country, account.country_name, account.city, account.postcode) {
        Some(proto::Address {
            line1,
            state,
            country_code,
            country_name,
            city,
            postcode,
        })
    } else { None };

    proto::Account {
        id: account.id,
        name: account.name,
        address,
        code: account.code,
        phone: account.phone,
        website: account.website,
        email: account.email,
        vat_number: account.vat_number,
        kvk: account.chamber_of_commerce,
        status: if let Some(status) = account.status {
            Some(match status.as_str() {
                "C" => AccountStatus::Customer,
                "P" => AccountStatus::Prospect,
                "S" => AccountStatus::Suspect,
                "A" | _ => AccountStatus::None,

            } as i32)
        } else { None },
        is_supplier: account.is_supplier,
    }
}
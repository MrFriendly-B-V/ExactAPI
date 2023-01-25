use exact_requests::contact::Contact;

pub fn contact_to_proto(contact: Contact) -> proto::Contact {
    proto::Contact {
        id: contact.id,
        account: contact.account,
        email: contact.email,
        full_name: contact.full_name,
        phone: contact.phone,
        first_name: contact.first_name,
        last_name: contact.last_name,
        mobile: contact.mobile.or(contact.business_mobile),
    }
}
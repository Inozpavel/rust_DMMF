pub mod services;

pub mod command;
pub mod entities;
pub mod events;
pub mod unvalidated;
pub mod value_objects;

#[derive(Debug)]
pub struct ValidationError {
    field_name: String,
    error_description: String,
}

#[derive(Debug)]
// #[derive(Error, Debug)]
pub enum PlaceOrderError {
    ValidationError(Vec<ValidationError>),
}

// #[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
// pub enum CategorizedEmail {
//     Quote,
//     Order,
// }

struct InvoiceId(String);

struct UnpaidInvoice {
    invoice_id: InvoiceId,
}

struct PaidInvoice {
    invoice_id: InvoiceId,
}

enum Invoice {
    Unpaid(UnpaidInvoice),
    Paid(PaidInvoice),
}

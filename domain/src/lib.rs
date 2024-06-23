pub mod services;

pub mod command;
pub mod entities;
pub mod errors;
pub mod events;
pub mod unvalidated;
pub mod value_objects;

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

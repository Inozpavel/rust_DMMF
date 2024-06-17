use crate::models::value_objects::address::billing_address::BillingAddress;
use crate::models::value_objects::amount_to_bill::AmountToBill;
use crate::models::value_objects::order_id::OrderId;
use crate::models::value_objects::email_address::EmailAddress;
use crate::models::value_objects::order_acknowledgment::OrderAcknowledgment;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct OrderPlaced {
    order_id: OrderId,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct BillableOrderPlaced {
    order_id: OrderId,
    billing_address: BillingAddress,
    amount_to_bill: AmountToBill,
}

pub enum PlaceOrderEvent {
    BillableOrderPlaced(BillableOrderPlaced),
    OrderPlaced(OrderPlaced),
    AcknowledgementSent(OrderAcknowledgementSent),
}

// pub struct PlaceOrderEvents {
//     acnowledgment_sent: AcnowledgmentSent,
//     order_placed: OrderPlaced,
//     billable_order_placed: BillableOrderPlaced,
// }

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct OrderAcknowledgementSent {
    order_id: OrderId,
    email_address: EmailAddress,
}

impl OrderAcknowledgementSent {
    pub fn create(order_id: OrderId, order_acknowledgment: OrderAcknowledgment) -> Self {
        let (email, _letter) = order_acknowledgment.into_inner();
        Self {
            email_address: email,
            order_id,
        }
    }
}


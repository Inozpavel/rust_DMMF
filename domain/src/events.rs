use crate::entities::order::PricedOrder;
use crate::value_objects::address::Address;
use std::rc::Rc;

use crate::value_objects::amount_to_bill::AmountToBill;
use crate::value_objects::email_address::EmailAddress;
use crate::value_objects::order_acknowledgment::OrderAcknowledgment;
use crate::value_objects::order_id::OrderId;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct OrderPlaced {
    order_id: Rc<OrderId>,
}

impl OrderPlaced {
    pub fn create(order_id: Rc<OrderId>) -> Self {
        Self { order_id }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct BillableOrderPlaced {
    order_id: Rc<OrderId>,
    billing_address: Rc<Address>,
    amount_to_bill: AmountToBill,
}

impl BillableOrderPlaced {
    pub fn create(order: &PricedOrder) -> Self {
        Self {
            order_id: order.order_id.clone(),
            amount_to_bill: order.amount_to_bill,
            billing_address: order.billing_address.clone(),
        }
    }
}

pub enum PlaceOrderEvent {
    BillableOrderPlaced(BillableOrderPlaced),
    OrderPlaced(OrderPlaced),
    AcknowledgementSent(OrderAcknowledgementSent),
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct OrderAcknowledgementSent {
    order_id: Rc<OrderId>,
    email_address: Rc<EmailAddress>,
}

impl OrderAcknowledgementSent {
    pub fn create(order_id: Rc<OrderId>, order_acknowledgment: OrderAcknowledgment) -> Self {
        Self {
            email_address: order_acknowledgment.email_address.clone(),
            order_id,
        }
    }
}

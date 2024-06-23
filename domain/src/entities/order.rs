use crate::entities::order_line::{PricedOrderLine, ValidatedOrderLine};
use crate::unvalidated::unvalidated_order::UnvalidatedOrder;
use crate::value_objects::address::Address;
use crate::value_objects::amount_to_bill::AmountToBill;
use crate::value_objects::customer_info::CustomerInfo;
use crate::value_objects::order_id::OrderId;
use std::rc::Rc;

#[readonly::make]
pub struct ValidatedOrder {
    pub order_id: Rc<OrderId>,
    pub customer_info: Rc<CustomerInfo>,
    shipping_address: Rc<Address>,
    billing_address: Rc<Address>,
    pub order_lines: Vec<ValidatedOrderLine>,
}

impl ValidatedOrder {
    pub fn create(
        order_id: OrderId,
        customer_info: CustomerInfo,
        shipping_address: Address,
        billing_address: Address,
        order_lines: Vec<ValidatedOrderLine>,
    ) -> Self {
        ValidatedOrder {
            order_id: Rc::new(order_id),
            customer_info: Rc::new(customer_info),
            shipping_address: Rc::new(shipping_address),
            billing_address: Rc::new(billing_address),
            order_lines,
        }
    }

    pub fn get_order_lines_ref(&self) -> &Vec<ValidatedOrderLine> {
        &self.order_lines
    }
}

#[readonly::make]
pub struct PricedOrder {
    pub order_id: Rc<OrderId>,
    pub customer_info: Rc<CustomerInfo>,
    shipping_address: Rc<Address>,
    billing_address: Rc<Address>,
    pub order_lines: Vec<PricedOrderLine>,
    pub amount_to_bill: AmountToBill,
}

impl PricedOrder {
    pub fn create(order: &ValidatedOrder, lines: Vec<PricedOrderLine>) -> Self {
        let amount_to_bill = AmountToBill::sum_prices(&lines);
        Self {
            order_lines: lines,
            order_id: order.order_id.clone(),
            customer_info: order.customer_info.clone(),
            shipping_address: order.shipping_address.clone(),
            billing_address: order.shipping_address.clone(),
            amount_to_bill,
        }
    }
}

pub enum Order {
    Unvalidated(UnvalidatedOrder),
    Validated(ValidatedOrder),
    Priced(PricedOrder),
}

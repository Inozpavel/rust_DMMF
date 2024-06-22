use crate::models::entities::order_line::{PricedOrderLine, ValidatedOrderLine};
use crate::models::unvalidated::unvalidated_order::UnvalidatedOrder;
use crate::models::value_objects::amount_to_bill::AmountToBill;
use crate::models::value_objects::customer_info::CustomerInfo;
use crate::models::value_objects::order_id::OrderId;
use std::rc::Rc;

#[readonly::make]
pub struct ValidatedOrder {
    pub order_id: Rc<OrderId>,
    pub customer_info: Rc<CustomerInfo>,
    // shipping_address: ShippingAddress,
    // billing_address: BillingAddress,
    pub order_lines: Vec<ValidatedOrderLine>,
}

impl ValidatedOrder {
    pub fn create(
        order_id: OrderId,
        customer_info: CustomerInfo,
        // shipping_address: ShippingAddress,
        // billing_address: BillingAddress,
        order_lines: Vec<ValidatedOrderLine>,
    ) -> Self {
        ValidatedOrder {
            order_id: Rc::new(order_id),
            customer_info: Rc::new(customer_info),
            // shipping_address,
            // billing_address,
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
    // shipping_address: ShippingAddress,
    // billing_address: BillingAddress,
    pub order_lines: Vec<PricedOrderLine>,
    pub amount_to_bill: AmountToBill,
}

impl PricedOrder {
    pub fn create(
        order_id: Rc<OrderId>,
        customer_info: Rc<CustomerInfo>,
        lines: Vec<PricedOrderLine>,
    ) -> Self {
        let amount_to_bill = AmountToBill::sum_prices(&lines);
        Self {
            order_lines: lines,
            order_id,
            customer_info,
            // shipping_address: (),
            // billing_address: (),
            amount_to_bill,
        }
    }
}

pub enum Order {
    Unvalidated(UnvalidatedOrder),
    Validated(ValidatedOrder),
    Priced(PricedOrder),
}

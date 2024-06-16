use crate::models::entities::order_line::{PricedOrderLine, ValidatedOrderLine};
use crate::models::unvalidated::unvalidated_order::UnvalidatedOrder;
use crate::models::value_objects::address::billing_address::BillingAddress;
use crate::models::value_objects::address::shipping_address::ShippingAddress;
use crate::models::value_objects::amount_to_bill::AmountToBill;
use crate::models::value_objects::customer_info::CustomerInfo;
use crate::models::value_objects::order_id::OrderId;

pub struct ValidatedOrder {
    order_id: OrderId,
    customer_info: CustomerInfo,
    // shipping_address: ShippingAddress,
    // billing_address: BillingAddress,
    order_lines: Vec<ValidatedOrderLine>,
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
            order_id,
            customer_info,
            // shipping_address,
            // billing_address,
            order_lines,
        }
    }

    pub fn get_order_lines_ref(&self) -> &Vec<ValidatedOrderLine> {
        &self.order_lines
    }

    pub fn into_lines(self) -> Vec<ValidatedOrderLine> {
        self.order_lines
    }
}

pub struct PricedOrder {
    order_id: OrderId,
    customer_info: CustomerInfo,
    // shipping_address: ShippingAddress,
    // billing_address: BillingAddress,
    order_lines: Vec<PricedOrderLine>,
    amount_to_bill: AmountToBill,
}

impl PricedOrder {
    pub fn create(order: ValidatedOrder, lines: Vec<PricedOrderLine>) -> Self {
        let amount_to_bill = AmountToBill::sum_prices(&lines);
        Self {
            order_lines: lines,
            order_id: order.order_id,
            customer_info: order.customer_info,
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

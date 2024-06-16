use crate::models::unvalidated::unvalidated_address::UnvalidatedAddress;
use crate::models::unvalidated::unvalidated_customer::UnvalidatedCustomerInfo;
use crate::models::unvalidated::unvalidated_order_line::UnvalidatedOrderLine;

pub struct UnvalidatedOrder {
    pub order_id: String,
    pub customer_info: UnvalidatedCustomerInfo,
    pub shipping_address: UnvalidatedAddress,
    pub lines: Vec<UnvalidatedOrderLine>,
}

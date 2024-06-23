use crate::unvalidated::unvalidated_address::UnvalidatedAddress;
use crate::unvalidated::unvalidated_customer::UnvalidatedCustomerInfo;
use crate::unvalidated::unvalidated_order_line::UnvalidatedOrderLine;

pub struct UnvalidatedOrder {
    pub order_id: String,
    pub customer_info: UnvalidatedCustomerInfo,
    pub shipping_address: UnvalidatedAddress,
    pub billing_address: UnvalidatedAddress,
    pub lines: Vec<UnvalidatedOrderLine>,
}

use crate::models::unvalidated::unvalidated_address::UnvalidatedAddress;
use crate::models::unvalidated::unvalidated_customer::UnvalidatedCustomerInfo;

pub struct UnvalidatedOrder {
    pub order_id: String,
    pub customer_info: UnvalidatedCustomerInfo,
    pub shipping_address: UnvalidatedAddress,
}

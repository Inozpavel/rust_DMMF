use crate::unvalidated::unvalidated_order::UnvalidatedOrder;
use chrono::{DateTime, Utc};

struct Command<D> {
    data: D,
    timestamp: DateTime<Utc>,
    user_id: String,
}

type PlaceOrderCommand = Command<UnvalidatedOrder>;

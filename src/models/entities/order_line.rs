use crate::models::unvalidated::unvalidated_order_line::UnvalidatedOrderLine;
use crate::models::value_objects::order_line_id::OrderLineId;
use crate::models::value_objects::order_quantity::OrderQuantity;
use crate::models::value_objects::price::Price;
use crate::models::value_objects::product_code::ProductCode;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ValidatedOrderLine {
    id: OrderLineId,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
}

impl ValidatedOrderLine {
    pub fn create(order_line: UnvalidatedOrderLine) -> Result<Self, &'static str> {
        let line_id = OrderLineId::create(order_line.id)?;

        let product_code = ProductCode::create(order_line.product_code)?;
        let quantity = OrderQuantity::create(&product_code, order_line.order_quantity)?;

        let result = ValidatedOrderLine {
            id: line_id,
            product_code,
            order_quantity: quantity,
        };

        Ok(result)
    }

    pub fn get_product_code_ref(&self) -> &ProductCode {
        &self.product_code
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PricedOrderLine {
    id: OrderLineId,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
    price: Price,
}

impl PricedOrderLine {
    pub fn create(line: ValidatedOrderLine, price: Price) -> Self {
        let result = PricedOrderLine {
            price,
            id: line.id,
            product_code: line.product_code,
            order_quantity: line.order_quantity,
        };

        result
    }

    pub fn get_price(&self) -> &Price {
        &self.price
    }
}

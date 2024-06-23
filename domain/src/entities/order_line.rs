use crate::unvalidated::unvalidated_order_line::UnvalidatedOrderLine;
use crate::value_objects::order_line_id::OrderLineId;
use crate::value_objects::order_quantity::OrderQuantity;
use crate::value_objects::price::Price;
use crate::value_objects::product_code::ProductCode;
use std::rc::Rc;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ValidatedOrderLine {
    id: Rc<OrderLineId>,
    product_code: Rc<ProductCode>,
    order_quantity: OrderQuantity,
}

impl ValidatedOrderLine {
    pub fn create(order_line: UnvalidatedOrderLine) -> Result<Self, &'static str> {
        let line_id = OrderLineId::create(order_line.id)?;

        let product_code = ProductCode::create(order_line.product_code)?;
        let quantity = OrderQuantity::create(&product_code, order_line.order_quantity)?;

        let result = ValidatedOrderLine {
            id: Rc::new(line_id),
            product_code: Rc::new(product_code),
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
    id: Rc<OrderLineId>,
    product_code: Rc<ProductCode>,
    product_quantity: OrderQuantity,
    price: Price,
}

impl PricedOrderLine {
    pub fn create(line: &ValidatedOrderLine, product_price: Price) -> Self {
        let price = product_price.multiply(&line.order_quantity);
        let result = Self {
            price,
            id: line.id.clone(),
            product_code: line.product_code.clone(),
            product_quantity: line.order_quantity,
        };

        result
    }

    pub fn get_price(&self) -> &Price {
        &self.price
    }
}

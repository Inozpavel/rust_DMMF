use std::fmt::{Display, Formatter};

use crate::models::value_objects::product_code::gizmo_code::GizmoCode;
use crate::models::value_objects::product_code::widget_code::WidgetCode;

pub mod gizmo_code;
pub mod widget_code;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

impl Display for ProductCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductCode::Widget(w) => write!(f, "{}", w.0),
            ProductCode::Gizmo(g) => write!(f, "{}", g.0),
        }
    }
}

impl ProductCode {
    pub fn create(value: String) -> Result<Self, &'static str> {
        if value.chars().all(|x| x.is_whitespace()) {
            Err("ProductCode can't be empty")
        } else {
            let code = WidgetCode::create(value)?;
            let result = ProductCode::Widget(code);

            Ok(result)
        }
    }

    pub fn get_ref(&self) -> &str {
        match self {
            ProductCode::Widget(widget) => &widget.0,
            ProductCode::Gizmo(gizmo_code) => &gizmo_code.0,
        }
    }
}

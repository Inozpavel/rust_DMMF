use crate::models::value_objects::product_code::gizmo_code::GizmoCode;
use crate::models::value_objects::product_code::widget_code::WidgetCode;

pub mod gizmo_code;
pub mod widget_code;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
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
            ProductCode::Widget(widget) => widget.get_ref(),
            ProductCode::Gizmo(gizmo_code) => gizmo_code.get_ref(),
        }
    }
}

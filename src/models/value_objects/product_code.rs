use crate::models::value_objects::product_code::gizmo_code::GizmoCode;
use crate::models::value_objects::product_code::widget_code::WidgetCode;

pub mod gizmo_code;
pub mod widget_code;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

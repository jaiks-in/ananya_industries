use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct PrintingBox {
    pub id: i64,
    pub name: String,
    pub box_type: String,       // "Corrugated" or "Printed"
    pub length: f32,
    pub breadth: f32,
    pub height: f32,
    pub ply: i32,
    pub base_labour_cost: f32,
    pub is_laminated: Option<bool>,
    pub lamination_charges: Option<f32>,
    pub print_type: Option<String>,
    pub print_charges: Option<f32>,
}
#[derive(Debug, Serialize, FromRow)]
pub struct CorrugatedBox {
    pub id: i64,
    pub name: String,
    pub box_type: String,       // "Corrugated" or "Printed"
    pub length: f32,
    pub breadth: f32,
    pub height: f32,
    pub ply: i32,
    pub base_labour_cost: f32,
}

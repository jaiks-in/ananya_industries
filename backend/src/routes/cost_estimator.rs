struct CorrugatedBox{
            length:f32,
            breadth:f32,
            height:f32,
            ply:i32,
            labour_charges:f32,
            number_of_labour:i32
}
struct PrintedBox{
    length:f32,
    breadth:f32,
    height:f32,
    ply:i32,
    labour_charges:f32,
    number_of_labour:i32,
    is_laminated:bool,
    lamination_charges:f32,
    lamination_type:Option<String>,
    print_type:Option<String>,
    print_charges:f32,
}
impl CorrugatedBox{
    fn costing_of_corrugated(){

    }
}

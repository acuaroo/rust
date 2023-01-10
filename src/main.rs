struct Matrix {
    augmented: bool,
    columns: i32,
    rows: i32,
    array: [[f32;columns];rows],
}

fn main() {
    let my_matrix = Matrix {
        augmented: true,
        columns: 4,
        rows: 3,
        array: [
            [1.0, 0.0, 3.0, 0.2],
            [0.0, -0.5, 2.0, 2.0],
            [3.0, -1.5, -1.5, 0.0],
        ]
    };

    println!("my matrix's array is: {:?}", my_matrix.array);
}
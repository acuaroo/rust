#[derive(Debug)]
struct Matrix {
    augmented: bool,
    columns: i32,
    rows: i32,
    array: Vec<Vec<f32>>,
}

impl Matrix {
    fn new(augmented: bool, columns: i32, rows: i32) -> Matrix {
        Matrix {
            augmented,
            columns,
            rows,
            array: vec![vec![0.; columns as usize]; rows as usize],
        }
    }

    fn row_add(&mut self, row1: i32, row2: i32, scale: f32) -> &Matrix {
        let arr_1 = &self.array[row1 as usize];
        let arr_2 = &self.array[row2 as usize];

        let arr_1_replicate = [[0;self.columns];self.rows];
        let count = 0;

        for num in arr_1.iter() {
            arr_1_replicate[count] = num+(arr_2 * scale);
            count += 1;
        }

        self.array[row1 as usize] = arr_1_replicate;

        return self;
    }
    fn reduced_row_echelon(&mut self) -> &Matrix {
        self.row_add(0, 1, 2.0);
        return self;
    }
}

fn main() {
    let mut my_matrix = Matrix::new(true, 4, 3);

    my_matrix.array = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![4.0, 1.0, 2.0, 3.0],
        vec![0.0, 1.0, 2.0, 5.0],
    ];

    println!("my matrix in rre is {:?}", my_matrix.reduced_row_echelon().array);
}
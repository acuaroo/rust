#[derive(Debug)]
struct Matrix {
    augmented: bool,
    array: [[f32;3];2]
}

impl Matrix {
    fn row_add(&mut self, _row0: i32, _row1: i32, scale: f32) -> [f32;3] {
        let array_row_0 = self.array[0];
        let array_row_1 = self.array[1];

        let mut row_added = [0.0;3];
        let mut count = 0;

        for num in array_row_0.iter() {
            row_added[count] = num+(array_row_1[count] * scale);
            count += 1;
        };
        
        self.array[0] = row_added;

        return row_added;
    }

    fn row_multiply(&mut self, _row0: i32, scale: f32) -> [f32;3] {
        let array_row_0 = self.array[0];

        let mut row_multiplied = [0.0;3];
        let mut count = 0;

        for num in array_row_0.iter() {
            row_multiplied[count] = num*scale;
            count += 1;
        };
        
        self.array[0] = row_multiplied;

        return row_multiplied;
    }

    fn reduced_row_echelon(&mut self) -> &Matrix {
        if self.augmented {
            self.row_add(0, 1, 0.5);
            self.row_multiply(0, 100.0);

            return self;
        } else {
            return self;
        }
    }
}

fn main() {
    let mut my_matrix = Matrix {
        augmented: true,
        array: [
            [1.0, 0.0, 2.0], 
            [0.0, 1.0, 3.0],
        ]
    };

    println!("{:?}", my_matrix.reduced_row_echelon());
}
use super::matrix::Matrix;

impl Matrix {
    fn elementwise_operation(&mut self, matrix: &Matrix, op: Arithemetic) -> &Self {
        assert!(self.cols() == matrix.cols() && self.rows() == matrix.rows());

        self.map_with_index(|value, i| {
            if let Some(value2) = matrix.buffer().get(i) {
                return match op {
                    Arithemetic::ADD => *value + *value2,
                    Arithemetic::SUBTRACT => *value - *value2,
                    Arithemetic::MULTIPLY => *value * *value2,
                    Arithemetic::DIVIDE if *value2 != 0.0 => *value / *value2,
                    Arithemetic::DIVIDE if *value2 == 0.0 => panic!("Division with zero not possible"),
                    _ => panic!("Operation not supported")
                }
            }

            *value
        })
    }

    pub fn add(&mut self, matrix: &Matrix) -> &Self {
        self.elementwise_operation(matrix, Arithemetic::ADD)
    }

    pub fn sub(&mut self, matrix: &Matrix) -> &Self {
        self.elementwise_operation(matrix, Arithemetic::SUBTRACT)
    }
    
    pub fn divide(&mut self, matrix: &Matrix) -> &Self {
        self.elementwise_operation(matrix, Arithemetic::DIVIDE)
    }
    
    pub fn multiply(&mut self, matrix: &Matrix) -> &Self {
        self.elementwise_operation(matrix, Arithemetic::MULTIPLY)
    }

    pub fn dot(&mut self, matrix: &Matrix) -> Self {
        todo!();
    }

    pub fn dot_naive(&mut self, matrix: &Matrix) -> Self {
        assert!(self.cols() == matrix.rows(), "(A) cols must be equal to (B) rows");
        let mut result = Matrix::zeros(self.rows(), matrix.cols());

        for i in 0..self.rows() {
            for j in 0..matrix.cols() {
                for k in 0..matrix.rows() {
                    let x = result.cols() * i + j;
                    let y = self.cols() * i + k;
                    let z = matrix.cols() * k + j;

                    let n = self.buffer()[y];
                    let m = matrix.buffer()[z];

                    result.buffer_mut()[x] += n * m;
                }
            }
        }

        result
    }
}

pub enum Arithemetic {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE
}
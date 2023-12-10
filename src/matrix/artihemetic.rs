use super::matrix::Matrix;

impl Matrix {
    pub fn add(&mut self, matrix: &Matrix) -> &Self {
        assert!(self.cols() == matrix.cols() && self.rows() == matrix.rows());

        self.map_with_index(|value, i| {
            if let Some(value2) = matrix.buffer().get(i) {
                return *value + *value2;
            }

            *value
        });
        
        self
    }

    // fn sub() -> Self {
    //     todo!()
    // }

    // fn multiply() -> Self {
    //     todo!()
    // }

    // fn divide() -> Self {
    //     todo!()
    // }

    // fn scale() -> Self {
    //     todo!()
    // }
}
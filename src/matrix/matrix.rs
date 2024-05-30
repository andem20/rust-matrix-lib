use rand::Rng;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    buffer: Vec<f32>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        let values = vec![0.0; rows * cols];
        Self {
            rows,
            cols,
            buffer: values,
        }
    }

    pub fn map_with_index(&mut self, mapping_func: impl Fn(&f32, usize) -> f32) -> &Self {
        for (i, value) in self.buffer.iter_mut().enumerate() {
            *value = mapping_func(value, i);
        }

        return self;
    }

    pub fn map(&mut self, mapping_func: impl Fn(&f32) -> f32) -> &Self {
        self.map_with_index(|v, _| mapping_func(v))
    }

    pub fn random(rows: usize, cols: usize) -> Self {
        let mut matrix = Matrix::zeros(rows, cols);
        matrix.map(|_| rand::thread_rng().gen::<f32>());
        matrix
    }

    #[allow(dead_code)]
    pub fn debug(rows: usize, cols: usize) -> Self {
        let mut matrix = Matrix::zeros(rows, cols);
        matrix.map_with_index(|_, i| i as f32);
        matrix
    }

    pub fn from_array(array: Vec<Vec<f32>>) -> Self {
        let rows = array.len();
        let cols = array
            .get(0)
            .expect("Array must have a second dimension")
            .len();

        let mut matrix = Matrix::zeros(rows, cols);
        matrix.map_with_index(|_, i| array[i / cols][i % cols]);
        matrix
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn buffer(&self) -> &[f32] {
        self.buffer.as_ref()
    }

    pub fn buffer_mut(&mut self) -> &mut [f32] {
        self.buffer.as_mut()
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.buffer
            .windows(self.cols)
            .step_by(self.cols)
            .for_each(|window| {
                let _ = write!(f, "[ ");

                for i in window {
                    let _ = write!(f, "{:.8} ", i);
                }

                let _ = writeln!(f, "]");
            });

        Ok(())
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.buffer == other.buffer
    }
}

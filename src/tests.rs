#[cfg(test)]
mod tests {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn addition() {
        let mut matrix_1 = Matrix::debug(2, 2);
        let matrix_2 = Matrix::debug(2, 2);
        matrix_1.add(&matrix_2);

        let expected = Matrix::from_array(vec![
            vec![0.0, 2.0],
            vec![4.0, 6.0],
        ]);

        assert_eq!(expected, matrix_1);
    }

    #[test]
    fn subtraction() {
        let mut matrix_1 = Matrix::debug(2, 2);
        let matrix_2 = Matrix::debug(2, 2);
        matrix_1.sub(&matrix_2);

        let expected = Matrix::from_array(vec![
            vec![0.0, 0.0],
            vec![0.0, 0.0],
        ]);

        assert_eq!(expected, matrix_1);
    }

    #[test]
    fn multiplication() {
        let mut matrix_1 = Matrix::debug(2, 2);
        let matrix_2 = Matrix::debug(2, 2);
        matrix_1.multiply(&matrix_2);

        let expected = Matrix::from_array(vec![
            vec![0.0, 1.0],
            vec![4.0, 9.0],
        ]);

        assert_eq!(expected, matrix_1);
    }

    #[test]
    fn division() {
        let mut matrix_1 = Matrix::from_array(vec![
            vec![3.0, 2.0],
            vec![1.0, 10.0],
        ]);

        let matrix_2 = Matrix::from_array(vec![
            vec![3.0, 2.0],
            vec![1.0, 10.0],
        ]);

        matrix_1.divide(&matrix_2);

        let expected = Matrix::from_array(vec![
            vec![1.0, 1.0],
            vec![1.0, 1.0],
        ]);

        assert_eq!(expected, matrix_1);
    }

    #[test]
    fn dot_naive() {
        let mut matrix_1 = Matrix::from_array(vec![
            vec![1.0, 2.0, 1.0],
            vec![0.0, 1.0, 0.0],
            vec![2.0, 3.0, 4.0],
        ]);

        let matrix_2 = Matrix::from_array(vec![
            vec![2.0, 5.0],
            vec![6.0, 7.0],
            vec![1.0, 8.0],
        ]);

        let actual = matrix_1.dot_naive(&matrix_2);

        let expected = Matrix::from_array(vec![
            vec![15.0, 27.0],
            vec![6.0, 7.0],
            vec![26.0, 63.0],
        ]);

        assert_eq!(expected, actual);

        let mut matrix_1 = Matrix::from_array(vec![
            vec![1.0, 2.0, 1.0, 1.0, 1.0],
            vec![0.0, 1.0, 0.0, 1.0, 8.0],
            vec![2.0, 3.0, 4.0, 1.0, 1.0],
            vec![1.0, 2.0, 1.0, 10.0, 1.0],
            vec![0.0, 1.0, 2.0, 1.0, 0.0],
        ]);

        let matrix_2 = Matrix::from_array(vec![
            vec![2.0, 5.0],
            vec![6.0, 7.0],
            vec![1.0, 8.0],
            vec![3.0, 1.0],
            vec![1.0, 9.0],
        ]);

        let actual = matrix_1.dot_naive(&matrix_2);

        let expected = Matrix::from_array(vec![
            vec![19.0, 37.0],
            vec![17.0, 80.0],
            vec![30.0, 73.0],
            vec![46.0, 46.0],
            vec![11.0, 24.0],
        ]);

        assert_eq!(expected, actual);

        let mut matrix_1 = Matrix::from_array(vec![
            vec![1.0],
            vec![2.0],
            vec![3.0],
        ]);

        let matrix_2 = Matrix::from_array(vec![
            vec![3.0, 2.0, 1.0],
        ]);

        let actual = matrix_1.dot_naive(&matrix_2);

        let expected = Matrix::from_array(vec![
            vec![3.0, 2.0, 1.0],
            vec![6.0, 4.0, 2.0],
            vec![9.0, 6.0, 3.0],
        ]);

        assert_eq!(expected, actual);
    }
}
#[cfg(test)]
mod tests {
    use tensor::tensor::Tensor;

    fn create_3d_tensor() -> Tensor<f32, 24, 3> {
        let mut tensor = Tensor::<f32, 24, 3>::new([4, 3, 2]);
        tensor.set_values(
            (0..24)
                .map(|x| x as f32)
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        );

        return tensor;
    }

    fn create_2d_tensor() -> Tensor<f32, 12, 2> {
        let mut tensor = Tensor::<f32, 12, 2>::new([3, 4]);
        tensor.set_values(
            (0..12)
                .map(|x| x as f32)
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        );

        return tensor;
    }

    #[test]
    fn tensor_slicing_3d() {
        let tensor = create_3d_tensor();

        let actual = tensor.get::<4, 3>(&[0..2, 0..1, 0..2]);
        assert_eq!(actual.shape(), [2, 1, 2]);
        assert_eq!(actual.values(), [0.0, 1.0, 6.0, 7.0]);

        let actual = tensor.get::<16, 3>(&[0..4, 1..3, 0..2]);
        assert_eq!(actual.shape(), [4, 2, 2]);
        assert_eq!(
            actual.values(),
            [
                2.0, 3.0, 4.0, 5.0, 8.0, 9.0, 10.0, 11.0, 14.0, 15.0, 16.0, 17.0, 20.0, 21.0, 22.0,
                23.0
            ]
        );

        let actual = tensor.get::<6, 3>(&[2..3]);
        assert_eq!(actual.shape(), [1, 3, 2]);
        assert_eq!(actual.values(), [12.0, 13.0, 14.0, 15.0, 16.0, 17.0]);
    }

    #[test]
    fn tensor_slicing_2d() {
        let tensor = create_2d_tensor();

        let actual = tensor.get::<2, 2>(&[0..2, 0..1]);
        assert_eq!(actual.shape(), [2, 1]);
        assert_eq!(actual.values(), [0.0, 4.0]);

        let actual = tensor.get::<6, 2>(&[1..3, 1..4]);
        assert_eq!(actual.shape(), [2, 3]);
        assert_eq!(actual.values(), [5.0, 6.0, 7.0, 9.0, 10.0, 11.0]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
    fn panic_for_shape_out_of_bounds() {
        let tensor = create_3d_tensor();
        let _ = tensor.get::<4, 3>(&[0..2, 0..1, 0..2, 0..1]);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds for range 24..30, index 4")]
    fn panic_for_index_out_of_bounds() {
        let tensor = create_3d_tensor();
        let _ = tensor.get::<60, 3>(&[0..10]);
    }
}

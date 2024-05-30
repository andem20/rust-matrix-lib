#[cfg(test)]
mod arithmetic_tests {
    use tensor::tensor::Tensor;

    #[test]
    fn addition() {
        let mut tensor = Tensor::<f32, 24, 3>::new([4, 3, 2]);
        let values = (0..24).map(|x| x as f32).collect::<Vec<f32>>();
        let expected = (0..24).map(|x| (x * 2) as f32).collect::<Vec<f32>>();

        tensor.set_values(values.try_into().unwrap());

        let actual = tensor + tensor;

        assert_eq!(actual.values(), expected);
    }

    #[test]
    fn addition_broadcast() {
        let mut tensor = Tensor::<f32, 24, 3>::new([4, 3, 2]);
        let values = (0..24).map(|x| x as f32).collect::<Vec<f32>>();
        let expected = (0..24).map(|x| (x + 10) as f32).collect::<Vec<f32>>();

        tensor.set_values(values.try_into().unwrap());

        let actual = tensor + 10.0;

        assert_eq!(actual.values(), expected);
    }

    #[test]
    fn subtraction() {
        let mut tensor = Tensor::<f32, 24, 3>::new([4, 3, 2]);
        let values = (0..24).map(|x| x as f32).collect::<Vec<f32>>();
        let expected = (0..24).map(|_x| 0.0 as f32).collect::<Vec<f32>>();

        tensor.set_values(values.try_into().unwrap());

        let actual = tensor - tensor;

        assert_eq!(actual.values(), expected);
    }

    #[test]
    fn subtraction_broadcast() {
        let mut tensor = Tensor::<f32, 24, 3>::new([4, 3, 2]);
        let values = (0..24).map(|x| x as f32).collect::<Vec<f32>>();
        let expected = (0..24).map(|x| (x - 10) as f32).collect::<Vec<f32>>();

        tensor.set_values(values.try_into().unwrap());

        let actual = tensor - 10.0;

        assert_eq!(actual.values(), expected);
    }
}

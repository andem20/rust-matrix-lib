use tensor::tensor::Tensor;

fn main() {
    let mut tensor = Tensor::<f32, 24, 3>::new([4, 3, 2]);
    let values = (0..24).map(|x| x as f32).collect::<Vec<f32>>();

    tensor.set_values(values.try_into().unwrap());
    println!("{:?}", tensor);

    // println!("{:?}", tensor.get(&[0..2, 0..1, 0..2]));
    // println!("{:?}", tensor.get(&[0..4, 1..3, 0..2]));
    // println!("{:?}", tensor.get(&[2..3, 0..2]));
    println!("{:?}", tensor + tensor);

    let values = (0..24).map(|x| x as f32).collect::<Vec<f32>>();
    let tensor_1 = Tensor::<f32, 24, 3>::with_values(values.try_into().unwrap(), [4, 3, 2]);

    let values = (0..12).map(|x| -x as f32).collect::<Vec<f32>>();
    let tensor_2 = Tensor::<f32, 12, 3>::with_values(values.try_into().unwrap(), [4, 3, 1]);

    println!("{:?}", tensor_1 + tensor_2);
}

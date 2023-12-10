use matrix_library::matrix::matrix::Matrix;

fn main() {    
    let mut matrix_1 = Matrix::debug(2, 2);
    let matrix_2 = Matrix::debug(2, 2);

    println!("{}", matrix_1);

    matrix_1.add(&matrix_2);

    println!("{}", matrix_1);
}
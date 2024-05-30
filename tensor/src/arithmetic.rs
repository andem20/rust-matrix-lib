use std::ops::{Add, Sub};

use crate::tensor::Tensor;

impl<T, const S: usize, const D: usize, const L: usize, const Z: usize> Add<Tensor<T, L, Z>>
    for Tensor<T, S, D>
where
    T: std::clone::Clone
        + std::default::Default
        + std::fmt::Debug
        + std::marker::Copy
        + Add<Output = T>,
{
    type Output = Tensor<T, S, D>;

    fn add(self, rhs: Tensor<T, L, Z>) -> Self::Output {
        let mut result = Tensor::<T, S, D>::new(self.shape().try_into().unwrap());

        if self.shape() != rhs.shape() {
            return self.broadcast(&rhs);
        }

        let mut values: [T; S] = [T::default(); S];

        for i in 0..self.values().len() {
            values[i] = self.values()[i] + rhs.values()[i];
        }

        result.set_values(values);
        return result;
    }
}

impl<T, const D: usize, const S: usize> Add<T> for Tensor<T, S, D>
where
    T: std::clone::Clone
        + std::default::Default
        + std::fmt::Debug
        + std::marker::Copy
        + Add<Output = T>,
{
    type Output = Tensor<T, S, D>;

    fn add(mut self, rhs: T) -> Self::Output {
        for value in self.values_mut().iter_mut() {
            *value = *value + rhs;
        }

        return self;
    }
}

impl<T, const S: usize, const D: usize> Sub for Tensor<T, S, D>
where
    T: std::clone::Clone
        + std::default::Default
        + std::fmt::Debug
        + std::marker::Copy
        + Sub<Output = T>,
{
    type Output = Tensor<T, S, D>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Tensor::<T, S, D>::new(self.shape().try_into().unwrap());

        let mut values: [T; S] = [T::default(); S];

        for i in 0..self.values().len() {
            values[i] = self.values()[i] - rhs.values()[i];
        }

        result.set_values(values);
        return result;
    }
}

impl<T, const D: usize, const S: usize> Sub<T> for Tensor<T, S, D>
where
    T: std::clone::Clone
        + std::default::Default
        + std::fmt::Debug
        + std::marker::Copy
        + Sub<Output = T>,
{
    type Output = Tensor<T, S, D>;

    fn sub(mut self, rhs: T) -> Self::Output {
        for value in self.values_mut().iter_mut() {
            *value = *value - rhs;
        }

        return self;
    }
}

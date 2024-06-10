use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub struct Tensor<T, const S: usize, const D: usize>
where
    T: std::clone::Clone + std::default::Default + std::fmt::Debug,
{
    pub values: [T; S],
    pub shape: [usize; D],
}

impl<T, const S: usize, const D: usize> Tensor<T, S, D>
where
    T: std::clone::Clone + std::default::Default + std::fmt::Debug + std::marker::Copy,
{
    pub fn new(shape: [usize; D]) -> Self {
        Self::with_values([T::default(); S], shape)
    }

    pub fn with_values(values: [T; S], shape: [usize; D]) -> Self {
        if shape.iter().product::<usize>() != S {
            panic!("Size {S} is not compatible with shape {:?}", shape);
        }

        Self { values, shape }
    }

    pub fn broadcast<const L: usize, const Z: usize>(&self, tensor: &Tensor<T, L, Z>) -> Self {
        // Shape B matches shape A except one dimension being 1.
        // Ex. A [4, 3, 2], B [4, 1, 2]
        unimplemented!("Broadcasting not implemented yet");
    }

    pub fn get<const L: usize, const Z: usize>(&self, idxs: &[Range<usize>]) -> Tensor<T, L, Z> {
        let mut size = self.values.len();

        let shape = self
            .shape
            .iter()
            .enumerate()
            .map(|(i, d)| idxs.get(i).map_or(*d, |x| x.len()))
            .collect::<Vec<usize>>();

        let mut result_tensor = Tensor::<T, L, Z>::new(shape.try_into().unwrap());
        let mut result_values = vec![self.values.clone().to_vec()];

        for (i, index) in idxs.iter().enumerate() {
            size /= self.shape[i];

            let mut temp_result = vec![];
            for slice in result_values.iter() {
                for part in index.clone().into_iter() {
                    let sub_start = part * size;
                    let sub_end = sub_start + size;
                    let slice_vec = slice
                        .get(sub_start..sub_end)
                        .expect(&format!(
                            "Index out of bounds for range {:?}, index {}",
                            sub_start..sub_end,
                            part
                        ))
                        .to_vec();
                    temp_result.push(slice_vec);
                }
            }

            result_values = temp_result;
        }

        result_tensor.values = result_values
            .into_iter()
            .flatten()
            .collect::<Vec<T>>()
            .try_into()
            .unwrap();

        return result_tensor;
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut [T] {
        &mut self.values
    }

    pub fn set_values(&mut self, values: [T; S]) {
        self.values = values;
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn dot<const L: usize, const Z: usize, const A: usize, const B: usize>(
        &self,
        tensor: &Tensor<T, L, Z>,
    ) -> Tensor<T, A, B> {
        return Tensor::<T, A, B>::new([0; B]);
    }
}

#[macro_export]
macro_rules! product {
    ($h:expr) => ($h);
    ($h:expr, $($t:expr),*) =>
        ($h * $crate::product!($($t),*));
}

#[macro_export]
macro_rules! tensor {
    ($t:ty, $($x:expr),+) => {
        Tensor {
            values: [<$t>::default(); $crate::product!($($x),*)],
            shape: [$($x),+]
        }
    };
}

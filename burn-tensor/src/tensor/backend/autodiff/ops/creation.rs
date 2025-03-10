use crate::tensor::{
    backend::{autodiff::ADTensor, Backend},
    ops::*,
};

impl<B: Backend, const D: usize> Zeros<Self> for ADTensor<D, B> {
    fn zeros(&self) -> Self {
        ADTensor::from_tensor(self.tensor().zeros())
    }
}

impl<B: Backend, const D: usize> Ones<Self> for ADTensor<D, B> {
    fn ones(&self) -> Self {
        ADTensor::from_tensor(self.tensor().ones())
    }
}

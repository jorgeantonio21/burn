use super::super::TestBackend;
use burn_tensor::{Data, Tensor};

#[test]
fn should_support_transpose_ops() {
    let data = Data::from([
        [[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]],
        [[6.0, 7.0, 8.0], [9.0, 10.0, 11.0]],
    ]);
    let tensor = Tensor::<TestBackend, 3>::from_data(data);

    let data_actual = tensor.transpose().into_data();

    let data_expected = Data::from([
        [[0.0, 3.0], [1.0, 4.0], [2.0, 5.0]],
        [[6.0, 9.0], [7.0, 10.0], [8.0, 11.0]],
    ]);
    data_expected.assert_approx_eq(&data_actual, 3);
}

#[test]
fn should_support_swap_dims() {
    let data = Data::from([
        [[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]],
        [[6.0, 7.0, 8.0], [9.0, 10.0, 11.0]],
    ]);
    let tensor = Tensor::<TestBackend, 3>::from_data(data);

    let data_actual = tensor.swap_dims(0, 2).into_data();

    let data_expected = Data::from([
        [[0.0, 6.0], [3.0, 9.0]],
        [[1.0, 7.0], [4.0, 10.0]],
        [[2.0, 8.0], [5.0, 11.0]],
    ]);
    data_expected.assert_approx_eq(&data_actual, 3);
}

use crate::tensor::TestADTensor;
use burn_tensor::Data;

#[test]
fn should_diff_powf() {
    let data_1 = Data::<f32, 2>::from([[0.0, 1.0], [3.0, 4.0]]);
    let data_2 = Data::<f32, 2>::from([[6.0, 7.0], [9.0, 10.0]]);

    let tensor_1 = TestADTensor::from_data(data_1);
    let tensor_2 = TestADTensor::from_data(data_2);

    let tensor_3 = tensor_1.matmul(&tensor_2.powf(0.4));
    let tensor_4 = tensor_3.matmul(&tensor_2);
    let grads = tensor_4.backward();

    let grad_1 = tensor_1.grad(&grads).unwrap();
    let grad_2 = tensor_2.grad(&grads).unwrap();

    grad_1
        .to_data()
        .assert_approx_eq(&Data::from([[68.0, 79.0328], [68.0, 79.0328]]), 3);
    grad_2
        .to_data()
        .assert_approx_eq(&Data::from([[23.5081, 25.2779], [26.0502, 28.6383]]), 3);
}

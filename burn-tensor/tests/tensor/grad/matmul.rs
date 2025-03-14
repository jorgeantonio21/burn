use crate::tensor::TestADTensor;
use burn_tensor::Data;

#[test]
fn should_diff_matmul() {
    let data_1: Data<f32, 2> = Data::from([[1.0, 7.0], [2.0, 3.0]]);
    let data_2: Data<f32, 2> = Data::from([[4.0, 7.0], [2.0, 3.0]]);

    let tensor_1 = TestADTensor::from_data(data_1);
    let tensor_2 = TestADTensor::from_data(data_2);

    let tensor_3 = &tensor_1.matmul(&tensor_2);
    let grads = tensor_3.backward();

    let grad_1 = tensor_1.grad(&grads).unwrap();
    let grad_2 = tensor_2.grad(&grads).unwrap();

    assert_eq!(grad_1.to_data(), Data::from([[11.0, 5.0], [11.0, 5.0]]));
    assert_eq!(grad_2.to_data(), Data::from([[3.0, 3.0], [10.0, 10.0]]));
    assert_eq!(
        tensor_3.clone().into_data(),
        Data::from([[18.0, 28.0], [14.0, 23.0]])
    );
}

#[test]
fn test_matmul_complex_1() {
    let data_1: Data<f32, 2> = Data::from([[1.0, 7.0], [13.0, -3.0]]);
    let data_2: Data<f32, 2> = Data::from([[4.0, 7.0], [2.0, 3.0]]);
    let data_3: Data<f32, 2> = Data::from([[2.0, 2.0], [2.0, 2.0]]);

    let tensor_1 = TestADTensor::from_data(data_1);
    let tensor_2 = TestADTensor::from_data(data_2);
    let tensor_3 = TestADTensor::from_data(data_3);

    let tensor_4 = tensor_1.matmul(&tensor_2);
    let tensor_5 = tensor_4.matmul(&tensor_3);

    let grads = tensor_5.backward();

    let grad_1 = tensor_1.grad(&grads).unwrap();
    let grad_2 = tensor_2.grad(&grads).unwrap();

    assert_eq!(grad_1.to_data(), Data::from([[44.0, 20.0], [44.0, 20.0]]));
    assert_eq!(grad_2.to_data(), Data::from([[56.0, 56.0], [16.0, 16.0]]));
}

#[test]
fn test_matmul_complex_2() {
    let data_1: Data<f32, 2> = Data::from([[1.0, 7.0], [13.0, -3.0]]);
    let data_2: Data<f32, 2> = Data::from([[4.0, 7.0], [2.0, 3.0]]);
    let data_3: Data<f32, 2> = Data::from([[2.0, 2.0], [2.0, 2.0]]);

    let tensor_1 = TestADTensor::from_data(data_1);
    let tensor_2 = TestADTensor::from_data(data_2);
    let tensor_3 = TestADTensor::from_data(data_3);

    let tensor_4 = tensor_1.matmul(&tensor_2);
    let tensor_5 = tensor_4.matmul(&tensor_3);
    let tensor_6 = tensor_1.matmul(&tensor_5);

    let grads = tensor_6.backward();

    let grad_1 = tensor_1.grad(&grads).unwrap();
    let grad_2 = tensor_2.grad(&grads).unwrap();

    assert_eq!(
        grad_1.to_data(),
        Data::from([[800.0, 792.0], [360.0, 592.0]])
    );
    assert_eq!(
        grad_2.to_data(),
        Data::from([[264., 264.0], [344.0, 344.0]])
    );
}

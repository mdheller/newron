#[cfg(test)]
mod tensor_tests {
    use newron::tensor::Tensor;
    
    #[test]
    fn test_0d_add() {
        let a = Tensor::new(vec![1.0], vec![]);
        let b = Tensor::new(vec![2.0], vec![]);

        let c = Tensor::new(vec![3.0], vec![]);

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_1d_add() {
        // Vector
        let a = Tensor::new(vec![1.0, 2.0], vec![2]);
        let b = Tensor::new(vec![3.0, 4.0], vec![2]);

        let c = Tensor::new(vec![4.0, 6.0], vec![2]);

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_2d_add() {
        // 2x2 matrix
        let a = Tensor::new(vec![1.0, 2.0,
                                 3.0, 4.0],
                                 vec![2, 2]);

        let b = Tensor::new(vec![5.0, 6.0,
                                 7.0, 8.0],
                                 vec![2, 2]);

        let c = Tensor::new(vec![6.0,   8.0,
                                 10.0, 12.0],
                                 vec![2, 2]);

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_get_row() {
        // 2x2 matrix
        let a = Tensor::new(vec![1.0, 2.0,
                                 3.0, 4.0],
                                 vec![2, 2]);
        
        let test_row = Tensor::new(vec![3.0, 4.0], vec![1, 2]);

        assert_eq!(a.get_row(1), test_row);
    }

    #[test]
    fn test_dot_sum_product() {
        // test if dot() acts as the dot() function in Numpy
        let a = Tensor::new(vec![1.0, 2.0, 3.0,
                                 4.0, 5.0, 6.0],
                                 vec![2, 3]);

        let b = Tensor::new(vec![1.0, 0.5], vec![1, 2]);

        let result = Tensor::new(vec![3.0, 4.5, 6.0], vec![1,3]);

        assert_eq!(a.dot(&b), result);
    }

    #[test]
    fn test_map() {
        let a = Tensor::new(vec![1.0, -2.0,
                                -3.0,  4.0],
                                 vec![2, 2]);

        let relu = |x| if x < 0.0 { 0.0 } else { x };

        let result = Tensor::new(vec![1.0, 0.0,
                                      0.0, 4.0],
                                      vec![2, 2]);
        
        assert_eq!(a.map(relu), result);
    }
}
pub mod matrix;
pub mod network;
pub mod activations;

use activations::SIGMOID;
use matrix::Matrix;
use network::Network;

fn test_matrix_multiplication(){
    let mut mat = Matrix::zeros(3, 3);
    mat.data[0][0] = 1.0;
    mat.data[0][1] = 2.0;
    mat.data[0][2] = 1.0;
    mat.data[1][0] = 0.0;
    mat.data[1][1] = 1.0;
    mat.data[1][2] = 0.0;
    mat.data[2][0] = 2.0;
    mat.data[2][1] = 3.0;
    mat.data[2][2] = 4.0;
    mat.print();

    println!("");

    let mut mat2 = Matrix::zeros(3, 2);
    mat2.data[0][0] = 2.0;
    mat2.data[0][1] = 5.0;
    mat2.data[1][0] = 6.0;
    mat2.data[1][1] = 7.0;
    mat2.data[2][0] = 1.0;
    mat2.data[2][1] = 8.0;
    mat2.print();
    println!("");

    let mult = Matrix::multiply(&mat, &mat2);
    mult.print();
    println!("");
}

fn main() {
    println!("Hello, world!");

    let inputs_train = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 1.0],
    ];
    
    let targets_train = vec![
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![1.0],
        vec![0.0],
        vec![0.0],
        vec![0.0],
        vec![0.0],
    ];
    
    let epochs = 10000;
    
    let mut network = Network::new(vec![3, 3, 1], 0.1, SIGMOID);
    
    println!("Starting training the network, epochs: {}", epochs);
    network.train(inputs_train, targets_train, epochs);
    println!("The training process has been done successfully!");
        
        
    let inputs_test = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 0.0, 1.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 1.0],
    ];

    for elem in inputs_test{
        println!("Input: {:?}, output: {:?}", elem.clone(), network.feed_forward(elem));
    };

}

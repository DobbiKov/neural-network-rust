pub mod matrix;
pub mod network;
pub mod activations;

use activations::SIGMOID;
use matrix::Matrix;
use network::Network;

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
    
    let epochs = 20000;
    
    let mut network = Network::new(vec![3, 3, 4, 1], 0.1, SIGMOID);
    network.load("3xor.json".to_string());
    
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

    network.save("3xor.json".to_string());

}

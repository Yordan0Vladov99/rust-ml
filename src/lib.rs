use activations::SIGMOID;
use data::Data;
use network::Network;

pub mod activations;
pub mod data;
pub mod matrix;
pub mod network;

pub fn train_network() {
	let training_data = Data::new("data/mnist_train.csv");
	let inputs = training_data.values;
	let targets = training_data.labels;
	let mut network = Network::new(vec![784, 64, 10], 0.001, SIGMOID);
	network.train(inputs, targets, 1);
	network.save("trained_network".to_string());
}

pub fn improve_network() {
	let mut network = Network::new(vec![784, 64, 10], 0.001, SIGMOID);
	network.load("trained_network".to_string());
	let training_data = Data::new("data/mnist_train.csv");
	let inputs = training_data.values;
	let targets = training_data.labels;
	network.train(inputs, targets, 1);
	network.save("trained_network".to_string());
}

pub fn test_network() {
	let mut network = Network::new(vec![784, 20, 10], 0.01, SIGMOID);
	network.load("trained_network".to_string());
	let training_data = Data::new("data/mnist_test.csv");
	let test_inputs = training_data.values;
	let test_labels = training_data.labels;
	let total: f32 = test_inputs.len() as f32;
	let mut correct: i32 = 0;

	for i in 0..test_inputs.len() {
		let output = network.feed_forward(test_inputs.get(i).unwrap().to_vec());
		//println!("{:?}", output.iter().sum::<f64>());

		let mut output_index: usize = 0;
		let mut max_output = output[0];
		for index in 1..output.len() {
			if output[index] > max_output {
				max_output = output[index];
				output_index = index;
			}
		}

		let mut label_index = 0;
		for index in 1..test_labels[i].len() {
			if (test_labels[i][index] - 1.0).abs() < 0.00001 {
				label_index = index;
				break;
			}
		}
		//println!("{} {}", output_index, label_index);

		if label_index == output_index {
			correct += 1;
		}
	}

	println!("{:.2}%", (correct as f32 / total) * 100.0)
}

pub fn sample_test() {
	let inputs = vec![
		vec![0.0, 0.0],
		vec![0.0, 1.0],
		vec![1.0, 0.0],
		vec![1.0, 1.0],
	];
	let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

	let mut network = Network::new(vec![2, 3, 1], 0.5, SIGMOID);

	network.train(inputs, targets, 1000);

	println!("{:?}", network.feed_forward(vec![0.0, 0.0]));
	println!("{:?}", network.feed_forward(vec![0.0, 1.0]));
	println!("{:?}", network.feed_forward(vec![1.0, 0.0]));
	println!("{:?}", network.feed_forward(vec![1.0, 1.0]));
}

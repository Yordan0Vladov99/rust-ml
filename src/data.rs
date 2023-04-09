use std::fs::File;
use std::io::prelude::*;
use std::vec;

pub struct Data {
	pub values: Vec<Vec<f64>>,
	pub labels: Vec<Vec<f64>>,
}

impl Data {
	pub fn new(file_name: &str) -> Data {
		let mut file = File::open(file_name).unwrap();
		let mut contents = String::new();
		file.read_to_string(&mut contents).unwrap();
		let mut values: Vec<Vec<f64>> = Vec::new();
		let mut labels: Vec<Vec<f64>> = Vec::new();

		for line in contents
			.trim()
			.split('\n')
			.collect::<Vec<&str>>()
			.iter()
			.skip(1)
		{
			let nums = line
				.trim()
				.split(',')
				.map(|x| match x.parse::<f64>() {
					Ok(num) => num,
					Err(err) => {
						println!("{}", err);
						panic!()
					}
				})
				.collect::<Vec<f64>>();
			let mut label: Vec<f64> = vec![0.0; 10];
			label[nums[0] as usize] = 1.0;
			labels.push(label);
			values.push(nums.into_iter().skip(1).collect())
		}
		Data { values, labels }
	}
}

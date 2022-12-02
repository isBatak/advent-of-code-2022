
fn find_max_calorie_elf(calorie_array: &[i32]) -> usize {
	let mut max_index = 0;
	let mut max_value = 0;
	for (index, &value) in calorie_array.iter().enumerate() {
			if value > max_value {
					max_index = index;
					max_value = value;
			}
	}
	max_index
}

pub fn run() {
	let calorie_array = [1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000];
	let max_calorie_elf = find_max_calorie_elf(&calorie_array);
	println!("The Elf carrying the most Calories is at index {} with {} total Calories.", max_calorie_elf, calorie_array[max_calorie_elf]);
}



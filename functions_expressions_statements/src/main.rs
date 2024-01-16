  // & Functions 
 //* Expressions 
//^ Statements

fn main() {
    println!("Hello, world!");
	test();

	let result: i32 = add_numbers(128_000, 19_000);
	println!("Result: {}\n", result);

	let result2:i32 = add_numbers2(12_000, 48_000);
	print!("Result2: {} \n", result2);

	let result3:i32 = add_numbers3(1,4);
	println!("Result3: {}", result3);

	//^ Expression
	let number = {
		let x:i32 = 3;
		x + 1
	};

	print!("Number: {}", number);

}

fn test() {
	print!("Test has been called 🐳 \n")
}

fn add_numbers(x: i32, y: i32) -> i32{
	x + y
}

fn add_numbers2(x: i32, y: i32) -> i32{
	return x + y;
}

fn add_numbers3(x: i32, y: i32) -> i32{
	let result = x + y;

	if result > 10{
		return result * 10;
	}

	else if result < 10{
		return result + 100_000_000
	}

	return result;
}


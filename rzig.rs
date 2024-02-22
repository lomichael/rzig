use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		eprintln!("Invalid number of arguments\n");
		process::exit(1);
	}

	println!(".intel_syntax noprefix\n"); // use intel instead of att syntax
	println!(".global main\n"); // defines a global label main
	println!("main:\n"); 
	println!("  mov rax, %d\n", atoi(argv[1])); // setting the arg in the return value register (rax)
	println!("  ret\n");
}

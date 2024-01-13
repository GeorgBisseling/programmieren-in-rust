use std::env;


fn collatz_step(n : u64) -> u64 {

	match n {
		0 => panic!("undefined for zero"),
		a if (n % 2) == 0 => a/2,
		_ => 3*n+1,
	}

}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
	let mut n : u64 = args[1].parse().unwrap();

	for i in 1.. {
		n = collatz_step(n);
		println!("{} -> {}", i, n);

		if n == 1 {
			break;
		}
	}
}

fn collatz_step(n : u64) -> u64 {

	match n {
		0 => panic!("undefined for zero"),
		a if (n % 2) == 0 => a/2,
		_ => 3*n+1,
	}

}

fn main() {
	let mut n = 1+ 1024;

	for i in 1.. {
		n = collatz_step(n);
		println!("{} -> {}", i, n);

		if n == 1 {
			break;
		}
	}
}

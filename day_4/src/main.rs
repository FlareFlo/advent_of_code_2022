use std::fs;
use std::ops::Range;
use std::str::FromStr;

fn main() {
	let input = fs::read_to_string("input.txt").expect("Input should exist");

	let pairs: Vec<((Range<usize>, Range<usize>))> = input.split('\n')
														  .map(|line| line.split(',')
																		  .map(|ranges| ranges.split('-')
																							  .map(|pairs| usize::from_str(pairs).expect("Should be unsigned integer"))
																		  )
																		  .map(|mut range_parsed| range_parsed.next().unwrap()..range_parsed.next().unwrap())
														  )
														  .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
														  .collect();

	let mut dupes = 0;
	for (x, y) in &pairs {
		if is_inside(x.clone(), y.clone()) || is_inside(y.clone(), x.clone()) {
			dupes += 1;
		}
	}
	println!("{}", dupes);

}

fn is_inside(outer: Range<usize>, inner: Range<usize>) -> bool {
	outer.start <= inner.start && outer.end >= inner.end
}

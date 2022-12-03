use std::fs;
use std::str::FromStr;

// I refer to everything as shape because the singular of moves is a keyword

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Shape {
	opponent: char,
	you: char,
	weakness: char,
	score_for_picking: i32,
}

const SHAPES: &[Shape; 3] = &[
	// Rock
	Shape {
		opponent: 'A',
		you: 'X',
		weakness: 'B',
		score_for_picking: 1,
	},
	// Paper
	Shape {
		opponent: 'B',
		you: 'Y',
		weakness: 'C',
		score_for_picking: 2,
	},
	// Scissors
	Shape {
		opponent: 'C',
		you: 'Z',
		weakness: 'A',
		score_for_picking: 3,
	},
];

impl Shape {
	fn from_play(play: char) -> Shape {
		return match play {
			'A' | 'X' => SHAPES[0],
			'B' | 'Y' => SHAPES[1],
			'C' | 'Z' => SHAPES[2],
			_ => { panic!("Shape should be matched") }
		};
	}
}


fn main() {
	let input = fs::read_to_string("input.txt").expect("Input should exist");
	let rounds: Vec<(char, char)> = input
		.split('\n')
		.map(|row|
			row.split(' ')
			   .map(|shape| char::from_str(shape).expect("Input should be infallible"))
		)
		.map(|mut shapes| (shapes.next().unwrap(), shapes.next().unwrap()))
		.collect();

	let mut total_score = 0;


	for round in rounds {
		let (opponent, you) = (Shape::from_play(round.0), Shape::from_play(round.1));

		match () {
			_ if you.opponent == opponent.weakness => {
				total_score += WIN;
			}

			_ if opponent.opponent == you.weakness => {
				total_score += LOSS;
			}

			_ if opponent == you => {
				total_score += DRAW;
			}

			_ => {
				dbg!(opponent, you, round);
				panic!("Shape should be matched") }
		}

		total_score += you.score_for_picking;
	}


	println!("{}", total_score);
}

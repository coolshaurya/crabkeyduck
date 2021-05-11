mod utf8;

use std::error::Error;
use std::convert::Infallible;

trait Input {
	// type Iter: Iterator<Item = Result<char, Error>>;
	type Error: Error = Infallible;

	fn into_char_iter(self) -> impl Iterator<Item=Result<char,Self::Error>>;
}


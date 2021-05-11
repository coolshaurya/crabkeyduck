use crate::try_nested;
use std::io::{self, Read};

struct Utf8Error;

enum StreamError {
	IoError(io::Error),
	Utf8Error(Utf8Error),
}

impl From<io::Error> for StreamError {
	fn from(io_error: io::Error) -> Self {
		Self::IoError(io_error)
	}
}
impl From<Utf8Error> for StreamError {
	fn from(utf8_error: Utf8Error) -> Self {
		Self::Utf8Error(utf8_error)
	}
}

struct CharStream<R> {
	inner_bytes: io::Bytes<R>,
}

impl<R: Read> CharStream<R> {
	fn new(reader: R) -> Self {
		Self {
			inner_bytes: reader.bytes(),
		}
	}
}

impl<R: Read> Iterator for CharStream<R> {
	type Item = Result<char, StreamError>;
	fn next(&mut self) -> Option<Self::Item> {
		let b1 = try_nested!(self.inner_bytes.next()?);

		if ranges::ASCII.contains(&b1) {
			Some(Ok(char::from(b1)))
		} else if ranges::TWO_BYTE_SEQ_START.contains(&b1) {
			let b2 = try_nested!(process_cont_byte(self.inner_bytes.next()));

			let mut ch: u32 = u32::from(clear_top_n(b1, 3));
			ch = push_cont_byte(ch, b2);

			Some(int_to_char(ch))
		} else if ranges::THREE_BYTE_SEQ_START.contains(&b1) {
			let b2 = try_nested!(process_cont_byte(self.inner_bytes.next()));
			let b3 = try_nested!(process_cont_byte(self.inner_bytes.next()));

			let mut ch: u32 = u32::from(clear_top_n(b1, 4));
			ch = push_cont_byte(ch, b2);
			ch = push_cont_byte(ch, b3);

			Some(int_to_char(ch))
		} else if ranges::FOUR_BYTE_SEQ_START.contains(&b1) {
			let b2 = try_nested!(process_cont_byte(self.inner_bytes.next()));
			let b3 = try_nested!(process_cont_byte(self.inner_bytes.next()));
			let b4 = try_nested!(process_cont_byte(self.inner_bytes.next()));

			let mut ch: u32 = u32::from(clear_top_n(b1, 5));
			ch = push_cont_byte(ch, b2);
			ch = push_cont_byte(ch, b3);
			ch = push_cont_byte(ch, b4);

			Some(int_to_char(ch))
		} else {
			Some(Err(From::from(Utf8Error)))
		}
	}
}

#[inline]
fn clear_top_n(b: u8, n: u32) -> u8 {
	b & (u8::MAX >> n)
}

#[inline]
fn push_cont_byte(ch: u32, b: u8) -> u32 {
	(ch << 6) | clear_top_n(b, 2)
}

fn int_to_char(i: u32) -> Result<char, StreamError> {
	if i > MAX_VALUE || SURROGATE_PAIRS.contains(i) {
		Err(From::from(Utf8Error))
	} else {
		// SAFETY: already checked that it's a valid unicode scalar value
		Ok(unsafe { char::from_u32_unchecked(i) })
	}
}

#[inline]
fn process_cont_byte(cont_byte: Option<Result<u8, io::Error>>) -> Result<u8, StreamError> {
	match cont_byte {
		Some(Err(e)) => Err(From::from(e)),
		Some(Ok(b)) if ranges::CONTINUATION_BYTE.contains(b) => Ok(b),
		Some(Ok(other_b)) => Err(From::from(Utf8Error)),
		None => Err(From::from(Utf8Error)),
	}
}

pub(crate) const MAX_VALUE: u32 = 0x0010FFFF;

#[allow(clippy::unusual_byte_groupings)]
pub(crate) mod ranges {
	use std::ops::RangeInclusive;

	pub(crate) const ASCII: RangeInclusive<u8> = 0..=0b0111_1111;
	pub(crate) const TWO_BYTE_SEQ_START: RangeInclusive<u8> = 0b110_00000..=0b110_11111;
	pub(crate) const THREE_BYTE_SEQ_START: RangeInclusive<u8> = 0b1110_0000..=0b1110_1111;
	pub(crate) const FOUR_BYTE_SEQ_START: RangeInclusive<u8> = 0b11110_000..=0b11110_111;

	pub(crate) const CONTINUATION_BYTE: RangeInclusive<u8> = 0b10_000000..=0b10_111111;
	pub(crate) const SURROGATE_PAIRS: RangeInclusive<u32> = 0xD800..=0xDFFF;
}

#[macro_export]
macro_rules! try_nested {
	($expr:expr $(,)?) => {
		match $expr {
			::std::result::Result::Ok(val) => val,
			::std::result::Result::Err(err) => {
				return ::std::option::Option::Some(::std::result::Result::Err(
					::std::convert::From::from(err),
				))
			}
		}
	};
}

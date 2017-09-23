// This file is part of guetzli-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/guetzli-sys/master/COPYRIGHT. No part of guetzli-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of guetzli-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/guetzli-sys/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug)]
	pub enum GuetzliError
	{
		ErrorReadingJpegDataFromInput
		{
			description("Error reading JPEG data from input")
			display("Error reading JPEG data from input")
		}

		MemoryLimitWouldBeExceeded
		{
			description("Memory limit would be exceeded")
			display("Memory limit would be exceeded")
		}

		ProcessingFailed
		{
			description("Processing Failed")
			display("Processing Failed")
		}
	}
}

impl GuetzliError
{
	#[inline(always)]
	fn process<R, IfOk: FnOnce() -> R>(resultCode: i32, ifOk: IfOk) -> Result<R, Self>
	{
		use self::GuetzliError::*;
		
		match resultCode
		{
			0 => Ok(ifOk()),
			-1 => Err(ErrorReadingJpegDataFromInput),
			-2 => Err(MemoryLimitWouldBeExceeded),
			-3 => Err(ProcessingFailed),
			_ => panic!("Unexpected resultCode '{}'", resultCode),
		}
	}
}

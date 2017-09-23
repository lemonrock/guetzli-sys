// This file is part of guetzli-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/guetzli-sys/master/COPYRIGHT. No part of guetzli-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of guetzli-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/guetzli-sys/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


#[macro_use] extern crate cpp;
extern crate libc;
#[macro_use] extern crate quick_error;


use ::libc::c_char;
use ::std::mem::forget;
use ::std::mem::uninitialized;


include!("GuetzliError.rs");


cpp!
{{
	#include <algorithm>
	#include <cstdio>
	#include <cstdlib>
	#include <exception>
	#include <memory>
	#include <string>
	#include <sstream>
	#include <string.h>
	#include "guetzli/jpeg_data.h"
	#include "guetzli/jpeg_data_reader.h"
	#include "guetzli/processor.h"
	#include "guetzli/quality.h"
	#include "guetzli/stats.h"
	
	// An upper estimate of memory usage of Guetzli. The bound is max(kLowerMemusaeMB * 1<<20, pixel_count * kBytesPerPixel)
	const int kBytesPerPixel = 350;

	typedef char * (* createVectorFromC)(size_t);
}}

/// Use this for the guetzli's function's `quality` parameter
pub const DefaultQuality: u8 = 95;

// This is the minimum quality permitted
pub const LowestQuality: u8 = 84;

/// Use this for the guetzli's function's `memoryLimitInMegabytes` parameter
pub const DefaultMemoryLimitInMegabytes: Option<u32> = Some(6000);

pub fn guetzli(jpegBytes: &[u8], quality: u8, memoryLimitInMegabytes: Option<u32>) -> Result<Vec<u8>, GuetzliError>
{
	extern "C" fn createVectorFromC(capacity: usize) -> *mut c_char
	{
		let mut vector: Vec<u8> = Vec::with_capacity(capacity);
		unsafe { vector.set_len(capacity) };
		let pointer = vector.as_mut_ptr() as *mut c_char;
		forget(vector);
		pointer
	}
	
	debug_assert!(quality >= LowestQuality && quality <= 100, "quality must be between 84 and 100 inclusive; the default is 95");
	let quality = quality as i32;
	
	let memlimit_mb = if let Some(memoryLimitInMegabytes) = memoryLimitInMegabytes
	{
		let memlimit_mb = memoryLimitInMegabytes as i32;
		const kLowestMemusageMB: i32 = 100;
		debug_assert!(memlimit_mb >= kLowestMemusageMB, "memoryLimitInMegabytes '{}' must be equal to or greater than {} megabytes", memlimit_mb, kLowestMemusageMB);
		memlimit_mb
	}
	else
	{
		-1
	};
	
	let createVectorFromC = createVectorFromC as *mut c_char;
	let jpegFileDataPointer = jpegBytes.as_ptr() as *const c_char;
	let jpegFileDataLength = jpegBytes.len();
	let mut resultPointer: *mut c_char = unsafe { uninitialized() };
	let mut resultLength: usize = unsafe { uninitialized() };
	
	let resultCode =
	{
		let resultPointerPointer = &mut resultPointer;
		let resultLengthPointer = &mut resultLength;
	
		unsafe
		{
			cpp!([createVectorFromC as "createVectorFromC", jpegFileDataPointer as "const char *", jpegFileDataLength as "const size_t", resultPointerPointer as "char * *", resultLengthPointer as "size_t *", quality as "int", memlimit_mb as "int"] -> i32 as "int"
			{
				std::string in_data(jpegFileDataPointer, jpegFileDataLength);
				guetzli::JPEGData jpg_header;
				if (!guetzli::ReadJpeg(in_data, guetzli::JPEG_READ_HEADER, &jpg_header))
				{
					return -1;
				}
			
				double pixels = static_cast<double>(jpg_header.width) * jpg_header.height;
				if (memlimit_mb != -1 && (pixels * kBytesPerPixel / (1 << 20) > memlimit_mb))
				{
					return -2;
				}

			    guetzli::Params params;
			    guetzli::ProcessStats stats;
			    params.butteraugli_target = static_cast<float>(guetzli::ButteraugliScoreForQuality(quality));
			    std::string out_data;
				if (!guetzli::Process(params, &stats, in_data, &out_data))
				{
					return -3;
				}
			
				size_t resultLength = out_data.length();
				*resultLengthPointer = resultLength;
				*resultPointerPointer = createVectorFromC(resultLength);
				memcpy(*resultPointerPointer, out_data.data(), resultLength);
				
				return 0;
			})
		}
	};
	
	GuetzliError::process(resultCode, || unsafe { Vec::from_raw_parts(resultPointer as *mut u8, resultLength, resultLength) })
}

#[test]
fn verify()
{
	use ::std::fs::File;
	use ::std::io::Read;
	use ::std::path::PathBuf;
	
	let testJpegFilePath = PathBuf::from("test-data/RagustanLibertineCoin1795.jpg");
	let metadata = testJpegFilePath.metadata().unwrap();
	
	let mut file = File::open(&testJpegFilePath).unwrap();
	let mut inputJpegBytes = Vec::with_capacity(metadata.len() as usize);
	file.read_to_end(&mut inputJpegBytes).unwrap();
	
	guetzli(&inputJpegBytes, LowestQuality, DefaultMemoryLimitInMegabytes).expect("did not perceptually encode JPEG");
}

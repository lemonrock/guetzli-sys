// This file is part of guetzli-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/guetzli-sys/master/COPYRIGHT. No part of guetzli-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of guetzli-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/guetzli-sys/master/COPYRIGHT.


#![allow(non_snake_case)]


extern crate cpp_build;
extern crate cc;


use ::cc::Build;
use ::cpp_build::Config;


fn main()
{
	compileCPlusPlusGlueCode();
	compileGuetzliLibrary();
}

fn compileCPlusPlusGlueCode()
{
	Config::new().include("lib/guetzli").include("lib/guetzli/third_party/butteraugli").flag("-w").build("src/lib.rs");
}

fn compileGuetzliLibrary()
{
	Build::new()
	.cpp(true)
	.shared_flag(false)
	.static_flag(true)
	.warnings(false)
	.opt_level(3)
	.debug(false)
	.flag("-std=c++11")
	.include("lib/guetzli")
	.include("lib/guetzli/third_party/butteraugli")
	.file("lib/guetzli/guetzli/butteraugli_comparator.cc")
	.file("lib/guetzli/guetzli/dct_double.cc")
	.file("lib/guetzli/guetzli/debug_print.cc")
	.file("lib/guetzli/guetzli/entropy_encode.cc")
	.file("lib/guetzli/guetzli/fdct.cc")
	.file("lib/guetzli/guetzli/gamma_correct.cc")
	.file("lib/guetzli/guetzli/idct.cc")
	.file("lib/guetzli/guetzli/jpeg_data.cc")
	.file("lib/guetzli/guetzli/jpeg_data_decoder.cc")
	.file("lib/guetzli/guetzli/jpeg_data_encoder.cc")
	.file("lib/guetzli/guetzli/jpeg_data_reader.cc")
	.file("lib/guetzli/guetzli/jpeg_data_writer.cc")
	.file("lib/guetzli/guetzli/jpeg_huffman_decode.cc")
	.file("lib/guetzli/guetzli/output_image.cc")
	.file("lib/guetzli/guetzli/preprocess_downsample.cc")
	.file("lib/guetzli/guetzli/processor.cc")
	.file("lib/guetzli/guetzli/quality.cc")
	.file("lib/guetzli/guetzli/quantize.cc")
	.file("lib/guetzli/guetzli/score.cc")
	.file("lib/guetzli/third_party/butteraugli/butteraugli/butteraugli.cc")
    .compile("libguetzli_static");
}

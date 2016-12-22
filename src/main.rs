/*
 The MIT License

 Copyright 2016 Ahseya.

 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:

 The above copyright notice and this permission notice shall be included in
 all copies or substantial portions of the Software.

 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 THE SOFTWARE.
 */

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate byteorder;
extern crate getopts;

mod cookies;

use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use getopts::{ Options, Matches };
use cookies::Cookies;

fn main() {
    env_logger::init()
        .or_else(|err| writeln!(&mut io::stderr(), "ERROR:main: logger init failed: {}", err))
        .unwrap();

    if let Some(matches) = args() {
        let files = &matches.free;
        let http_only = matches.opt_present("http_only");
        let parser = Cookies::new(http_only);
        process(&files, parser);
    }
}

fn args() -> Option<Matches> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("" , "help", "display this help and exit")
        .optflag("h", "http_only", "use #HttpOnly_ prefix")
        .parse(&args[1..])
        .map_err(|err| error!("{}", err))
        .ok()
        .and_then(|matches|
            if matches.opt_present("help") {
                println!("Usage: burntcookie [OPTION]... [FILE]...");
                println!("Parses Apple binary cookie file/s into their Netscape equivalent/s.");
                println!("{}", opts.usage("With no FILE, read standard input."));
                println!("Examples:");
                println!("burntcookie Cookies.binarycookies > cookies.txt\t\t\
                          Parse into cookies.txt");
                println!("RUST_LOG=WARN burntcookie Cookies.binarycookies\t\t\
                          With warnings");
                println!("");
                println!("Project home: http://github.com/horrorho/burnt-cookie");

                None
            } else {
                Some(matches)
            }
        )
}

fn process<P: AsRef<Path>>(paths: &[P], parser: Cookies) {
    if paths.is_empty() {
        buffer_file(&mut io::stdin())
                .and_then(|bs| parser.parse_content(&bs))
                .unwrap_or_else(|err| error!("stdin, {}", err))
    } else {
        paths.into_iter()
            .fold((), |_, p|
                File::open(p)
                    .and_then(|ref mut f| buffer_file(f))
                    .and_then(|bs| parser.parse_content(&bs))
                    .unwrap_or_else(|err| error!("{}: {}", p.as_ref().display(), err)))
    }
}

fn buffer_file<T: io::Read>(file: &mut T) -> io::Result<Vec<u8>>  {
    let mut bs: Vec<u8> = Vec::new();
    file.read_to_end(&mut bs).map(|_| bs)
}

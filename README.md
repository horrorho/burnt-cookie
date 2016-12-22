# Burnt Cookie
Horrorhos' Burnt Cookie.

##What is it?
Parses Apple binary cookie file/s into their Netscape equivalent/s. These Netscape cookie files are easily imported into web browsers or can be used with compatible tools like curl.

##Build
Requires [Rust](https://www.rust-lang.org).

[Download](https://github.com/horrorho/burnt-cookie/archive/master.zip), extract and navigate to the burnt-cookie-master folder:
```
~/burnt-cookie-master $ cargo build --release
```

The executable is located at: `/target/release/`
```
~/burnt-cookie-master $ ./target/release/burntcookie --help
Usage: burntcookie [OPTION]... [FILE]...
Parses Apple binary cookie file/s into their Netscape equivalent/s.
With no FILE, read standard input.

Options:
        --help          display this help and exit
    -h, --http_only     use #HttpOnly_ prefix

Example:
burntcookie Cookies.binarycookies > cookies.txt		Parse into cookies.txt

Project home: http://github.com/horrorho/burnt-cookie
```

##Usage

Convert a Cookies.binarycookies file:
```
$ burntcookie Cookies.binarycookies
```
Pipe to cookies.txt:
```
$ burntcookie Cookies.binarycookies > cookies.txt
```
\#HttpOnly\_ prefixes are not appended by default. Use the -h/ --http-only switch:
```
$ burntcookie -h Cookies.binarycookies > cookies.txt
```
Temporary .dat files are also parseable, although they may generate warnings (supressed by default):
```
$ burntcookie Cookies.binarycookies_tmp_1234_0.dat > cookies.txt
```
Enable warning output.
```
$ RUST_LOG=WARN burntcookie Cookies.binarycookies_tmp_1234_0.dat > cookies.txt
```

##Why Rust?
This project is largely an exercise in Rust programming. It's a conversion of a private tool I created a while back.

##Useful links
HTTP cookie: https://en.wikipedia.org/wiki/HTTP_cookie

Safari/iOS - Cookies.binarycookies reader: http://www.securitylearn.net/2012/10/27/cookies-binarycookies-reader/

PHP reading a cookie file: http://stackoverflow.com/questions/410109/php-reading-a-cookie-file

Netscape HTTP Cooke File Parser in PHP: http://www.hashbangcode.com/blog/netscape-http-cooke-file-parser-php





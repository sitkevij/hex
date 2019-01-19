# hex (hx)

Futuristic take on hexdump.

`hex` takes a file as input and outputs a hexadecimal colorized view to stdout.

```
$ hx -c12 tests/files/alphanumeric.txt
0x000000: 0x61 0x62 0x63 0x64 0x65 0x66 0x67 0x68 0x69 0x6a 0x6b 0x69 abcdefghijki
0x00000c: 0x6c 0x6d 0x6e 0x6f 0x70 0x71 0x72 0x73 0x74 0x75 0x76 0x77 lmnopqrstuvw
0x000018: 0x78 0x79 0x7a 0x30 0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 xyz012345678
0x000024: 0x39 0x0a 0x30 0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39 9.0123456789
0x000030: 0x30 0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39 0x30 0x31 012345678901
0x00003c: 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39                     23456789
   bytes: 68
```

[![build](https://travis-ci.org/sitkevij/hex.svg?branch=master)](https://travis-ci.org/sitkevij/hex)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fsitkevij%2Fhex.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fsitkevij%2Fhex?ref=badge_shield)

## quick links

* [install](#install)
* [features](#features)
* [license](#license)

## examples

### lower hex format -fx

`$ hx src/main.rs`

![lower hex output format](https://raw.githubusercontent.com/sitkevij/hex/master/assets/hex_screenshot_macos_format_default.png "default output format")

### binary hex format -fb

`$ hx -fb -c4 src/main.rs`

![binary hex output format](https://raw.githubusercontent.com/sitkevij/hex/master/assets/hex_screenshot_macos_format_b.png)

### octal hex format -fo

`$ hx -fo -c8 src/main.rs`

![octal hex output format](https://raw.githubusercontent.com/sitkevij/hex/master/assets/hex_screenshot_macos_format_o.png)

## install

### crates.io install

If `cargo` is already installed, simply:
```
cargo install hx 
```

### source install

From within the `hx` source code directory, simply execute:
```
make install
```

This will run the following `cargo` commands:
```
cargo build --release
cargo test --verbose --all -- --nocapture
cargo install --path . 
```

Which will compile the release version, run tests and install release binary to `<USERDIR>/.cargo/bin/hx`.

If `<USERDIR>/.cargo/bin` is part of the `PATH` evironment variable, `hx` should be able to be executed anywhere in the shell.

## features 

### output arrays in `rust`, `c` or `golang`

`hx` has a feature which can output the input file bytes as source code arrays. 

For example:

#### rust array: -ar

```
$ hx -ar -c8 tests/files/tiny.txt
let ARRAY: [u8; 3] = [
    0x69, 0x6c, 0x0a
];
```

#### c array: -ac

```
$ hx -ac -c8 tests/files/tiny.txt
unsigned char ARRAY[3] = {
    0x69, 0x6c, 0x0a
};
```

#### golang array: -ag

```
$ hx -ag -c8 tests/files/tiny.txt
a := [3]byte{
    0x69, 0x6c, 0x0a,
}
```

## license

MIT

[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fsitkevij%2Fhex.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fsitkevij%2Fhex?ref=badge_large)

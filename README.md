# hex (hx)

Futuristic take on hexdump.

[hx](https://github.com/sitkevij/hex) accepts a file path as input and outputs a hexadecimal colorized view to stdout.

```sh
$ hx tests/files/alphanumeric.txt
0x000000: 0x61 0x62 0x63 0x64 0x65 0x66 0x67 0x68 0x69 0x6a abcdefghij
0x00000a: 0x6b 0x69 0x6c 0x6d 0x6e 0x6f 0x70 0x71 0x72 0x73 kilmnopqrs
0x000014: 0x74 0x75 0x76 0x77 0x78 0x79 0x7a 0x30 0x31 0x32 tuvwxyz012
0x00001e: 0x33 0x34 0x35 0x36 0x37 0x38 0x39 0x0a 0x30 0x31 3456789.01
0x000028: 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39 0x30 0x31 2345678901
0x000032: 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39 0x30 0x31 2345678901
0x00003c: 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39           23456789
   bytes: 68
```

[hx](https://github.com/sitkevij/hex) also accepts stdin as input.

```sh
cat "tests/files/alphanumeric.txt" | hx
0x000000: 0x61 0x62 0x63 0x64 0x65 0x66 0x67 0x68 0x69 0x6a abcdefghij
0x00000a: 0x6b 0x69 0x6c 0x6d 0x6e 0x6f 0x70 0x71 0x72 0x73 kilmnopqrs
0x000014: 0x74 0x75 0x76 0x77 0x78 0x79 0x7a 0x30 0x31 0x32 tuvwxyz012
0x00001e: 0x33 0x34 0x35 0x36 0x37 0x38 0x39 0x0a 0x30 0x31 3456789.01
0x000028: 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39 0x30 0x31 2345678901
0x000032: 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39 0x30 0x31 2345678901
0x00003c: 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39           23456789
   bytes: 68
```

[![build](https://travis-ci.org/sitkevij/hex.svg?branch=master)](https://travis-ci.org/sitkevij/hex)
[![coverage](https://img.shields.io/codecov/c/github/sitkevij/hex/master.svg)](https://codecov.io/gh/sitkevij/hex)

## quick links

* [install](#install)
* [features](#features)
* [manual](#manual)
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

```sh
cargo install hx
```

### source install

From within the `hx` source code directory, simply execute:

```sh
make install
```

This will run the following `cargo` commands:

```sh
cargo build --release
cargo test --verbose --all -- --nocapture
cargo install --path .
```

Which will compile the release version, run tests and install release binary to `<USERDIR>/.cargo/bin/hx`.

If `<USERDIR>/.cargo/bin` is part of the `PATH` environment variable, `hx` should be able to be executed anywhere in the shell.

### debian install

```sh
curl -sLO https://github.com/sitkevij/hex/releases/download/v0.4.0/hx_0.4.0_amd64.deb && dpkg -i hx_0.4.0_amd64.deb
```

### docker run

```sh
cat README.md | docker run -i sitkevij/hx:latest
```

## features

### output arrays in `rust`, `c`, `golang`, `python`, `kotlin`, or `java`

`hx` has a feature which can output the input file bytes as source code arrays.

For example:

#### rust array: -ar

```sh
$ hx -ar -c8 tests/files/tiny.txt
let ARRAY: [u8; 3] = [
    0x69, 0x6c, 0x0a
];
```

#### c array: -ac

```sh
$ hx -ac -c8 tests/files/tiny.txt
unsigned char ARRAY[3] = {
    0x69, 0x6c, 0x0a
};
```

#### golang array: -ag

```sh
$ hx -ag -c8 tests/files/tiny.txt
a := [3]byte{
    0x69, 0x6c, 0x0a,
}
```

#### python array: -ap

```sh
$ hx -ap -c8 tests/files/tiny.txt
a = [
    0x69, 0x6c, 0x0a
]
```

#### kotlin array: -ak

```sh
$ hx -ak -c8 tests/files/tiny.txt
val a = byteArrayOf(
    0x69, 0x6c, 0x0a
)
```

#### java array: -aj

```sh
$ hx -aj -c8 tests/files/tiny.txt
byte[] a = new byte[]{
    0x69, 0x6c, 0x0a
};
```

### NO_COLOR support

`hx` will honor the NO_COLOR environment variable. If set, no color will be output to the terminal.

Rust `no_color` crate:

* <https://crates.io/crates/no_color>
* <https://github.com/sitkevij/no_color>

## manual

```txt
hx
Futuristic take on hexdump, made in Rust.

USAGE:
    hx [OPTIONS] [INPUTFILE]
    <stdout> | hx [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --array <array_format>    Set source code format output: rust (r), C (c), golang (g), python (p), kotlin (k), java (j) [possible values: r, c, g, p, k, j]
    -t, --color <color>           Set color tint terminal output. 0 to disable, 1 to enable [possible values: 0, 1]
    -c, --cols <columns>          Set column length
    -f, --format <format>         Set format of octet: Octal (o), LowerHex (x), UpperHex (X), Binary (b) [possible
                                  values: o, x, X, b]
    -u, --func <func_length>      Set function wave length
    -l, --len <len>               Set <len> bytes to read
    -p, --places <func_places>    Set function wave output decimal places

ARGS:
    <INPUTFILE>    Pass file path as an argument for hex dump
```

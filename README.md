## Heliometer
[![Build Status](https://travis-ci.org/birders/heliometer.svg?branch=master)](https://travis-ci.org/birders/heliometer)
[![Coverage Status](https://coveralls.io/repos/github/birders/heliometer/badge.svg?branch=master&service=github)](https://coveralls.io/github/birders/heliometer?branch=master)
[![Version](https://img.shields.io/crates/v/heliometer.svg)](https://crates.io/crates/heliometer)
[![Downloads](https://img.shields.io/crates/d/heliometer.svg)](https://crates.io/crates/heliometer)
[![License](https://img.shields.io/crates/l/heliometer.svg)](https://crates.io/crates/heliometer)

A Brainfuck interpreter written in Rust.

Heliometer can read and execute Brainfuck programs from a file, or it can be
used as a library. See the [documentation](https://birders.github.io/heliometer/master/heliometer/)
for details.


### Binary Installation
Simply run `cargo install` to install `helio` to `.cargo/bin`.

`helio <file>` will interpret the contents of `<file>` as Brainfuck, using
stdin and stdout for input and output respectively.

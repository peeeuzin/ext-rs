# ext-rs 🦀

[![Crates.io](https://img.shields.io/badge/crates.io-please_no-red)](https://crates.io/crates/ext-rs)
[![License](https://img.shields.io/badge/license-MIT,_i_guess-orange)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-surprisingly_passing-green)](.)

## ⚠️ **DO NOT USE THIS IN PRODUCTION** ⚠️

This is a **joke crate**. It's what happens when you doesn't have a job.

If you use this in production code, your pull request will be rejected, your colleagues will judge you, and rightfully so.

---

## The Problem That Doesn't Exist

You know what Rust is *really* missing? The C++ stream extraction operator (`>>`). Sure, Rust has:
- Proper error handling
- Zero-cost abstractions  
- Memory safety
- `Read` traits with explicit error types
- `parse()` methods that don't panic
- Actually readable code

But where's the **fun** in that? Where's the mysterious operator overloading? Where's the silent panic when parsing fails? Where's the chaos?

## The "Solution"

This crate brings the "magic" ✨ of C++'s `std::cin >> value` to Rust by abusing the right-shift operator because, apparently, that's what we're doing with our lives now.

## Installation

```toml
[dependencies]
ext-rs = "0.1.0"
```

## Usage

```rust
use std::io::BufReader;
use extract_rs::*;

let data = b"42 Hello 3.14";
let mut reader = ExtReader(BufReader::new(&data[..]));

let mut int_value: i32 = 0;
let mut string_value = String::new();
let mut float_value: f64 = 0.0;

// The cursed syntax:
reader >> &mut int_value >> &mut string_value >> &mut float_value;

// Or if you prefer your bad decisions to be more explicit:
reader.extract(&mut int_value)
      .extract(&mut string_value)
      .extract(&mut float_value);

assert_eq!(int_value, 42);
assert_eq!(string_value, "Hello");
assert_eq!(float_value, 3.14);
```

If you want to parse stdin directly:

```rust
use std::io::BufReader;
use extract_rs::*;

let stdin = std::io::stdin();
let mut reader = ExtReader(BufReader::new(stdin.lock()));

let mut int_value: i32 = 0;
reader >> &mut int_value;

// Curiously, the linter mark this as unused, but, who cares?
```

## Features

- ✅ **No error handling**: Because who needs that? Unwrap all the things!
- ✅ **Operator abuse**: Who cares about Rust's design principles?
- ✅ **Bad performance**: Efficiency is overrated
- ✅ **Unwrap everywhere**: Because `.unwrap()` never caused any problems, RIP Cloudflare

## Why This Exists

Because.

## FAQ

**Q: Should I use this?**  
A: No.

**Q: But what if—**  
A: No.

**Q: Can I contribute?**  
A: I mean, technically yes, but also... why?

**Q: Does this follow Rust best practices?**  
A: It follows the best practice of showing you what NOT to do.

## Comparison with Proper Rust Code

**This crate:**
```rust
reader >> &mut value; // What could go wrong? 🔥
```

**Actual Rust:**
```rust
let value: i32 = line
    .trim()
    .parse()
    .expect("Please enter a valid number");
```

One of these is maintainable.

## What You Should Actually Use

- [`std::io::BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) for reading
- [`str::parse()`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) for parsing
- [`Result`](https://doc.rust-lang.org/std/result/) for error handling
- Literally anything else

## License

MIT (because even jokes deserve proper licensing)

## Disclaimer

This crate is meant for educational purposes only—specifically, to educate you on what *not* to do. If you find yourself actually wanting to use this, please take a step back, have a cup of coffee, and reconsider your life choices.

Remember: Just because you *can* doesn't mean you *should*.

---

*Made with ❤️ and poor judgment*

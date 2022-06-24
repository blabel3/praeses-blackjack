# praeses-blackjack-rust [![Build & Test](https://github.com/blabel3/praeses-blackjack-rust/actions/workflows/rust-build-test.yml/badge.svg)](https://github.com/blabel3/praeses-blackjack-rust/actions/workflows/rust-build-test.yml) [![Release Binaries](https://github.com/blabel3/praeses-blackjack-rust/actions/workflows/release.yml/badge.svg)](https://github.com/blabel3/praeses-blackjack-rust/actions/workflows/release.yml)

(demo video or screenshot here)

# Praeses Blackjack

*Created in a week for part of the software engineer interview process at [Praeses](https://praeses.com/)* 

Rust crate that lets people play blackjack in their terminal. Made with the idea of extending later to support additional players and possibly a web client that interacts with this running on a server. But the main priority was to write good, readable, maintainable, efficient code to flex for Praeses!

# Table of Contents

- [Installing](#Installing)
- [Running](#Running)
- [Contributing](#Contributing)
- [Demo](#Demo)

# Installing

## Ready-made Binary

You can go to the releases section to download binaries that you can execute one whatever system you're running. 

You can then put that in your bin/ folder or wherever executables are available for you!

## Installing with Rust (recommended)

### Pre-Requisite

Have a commandline environment you're comfortable with, and install [Rust](https://www.rust-lang.org/tools/install).

### Installing via Cargo

Until I release this in a package repository, we will just have to install from the source! Run these commands in your terminal:

```
$ git clone https://github.com/blabel3/praeses-blackjack-rust
$ cd praeses-blackjack-rust
$ cargo install --path . 
``` 

# Running

Once the executable is installed, you should be able to run it by entering `pbj` in your terminal. A good start is to run `pbj help` for info on what options are available and some confirmation that everything is working correctly. 

# Contributing

To contribute to the project, first set up your environment by following the directions from [Installing with Rust](#Installing-with-Rust-(recommended)).

To make edits to the code, first, make sure you're in a new branch! The repo is set up to not allow commits directly on main to make sure the main branch is stable.

Pick out an issue in the repo's issues to tackle, and think carefully about how to change the code to best handle it. 

Then you can make your changes! Remember it can be a good idea to run `cargo check` periodically, and before you commit it's also good to run `cargo fmt` so that the code is formatted consistently. 

Once your changes are ready make a pull request and someone will review, approve, and merge it in.


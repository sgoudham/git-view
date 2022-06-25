# git-view

[![build](https://github.com/sgoudham/git-view/actions/workflows/build.yml/badge.svg)](https://github.com/sgoudham/git-view/actions/workflows/build.yml)
[![crate.io](https://img.shields.io/crates/v/git-view)](https://crates.io/crates/git-view)
[![downloads](https://img.shields.io/crates/d/git-view)](https://crates.io/crates/git-view)
[![license](https://img.shields.io/github/license/sgoudham/git-view)](LICENSE)

> A git sub-command to view your git repository in the web browser!

## Table Of Contents

TODO

## About

Are you _also_ frustrated from moving your hands away from the keyboard to view your git repository in the browser? 

> Me too!

`git-view` alleviates that pain by allowing you to chuck away your mouse!

> (n)vim users rejoice :P

## Features

- [x] GitHub & BitBucket
- [x] View Branches, Commits & Issues
- [x] Custom Suffix
- [x] Custom Remote

Feel free to raise any issues or pull requests (after having read the [CONTRIBUTING.md]()!!) for any additional Features
that you want!

## Installation

The _preferred_ way of installation is to manually install the provided binaries into your local `git` directory to enable
the usage as `git view`. However, that being said, it also available on [crates.io]() to allow installation through the use of `cargo`.

### Windows

### *nix / macOS

## Usage

```commandline
$ git view --help
```

![help](./docs/images/v0-1-0.png "Contents displayed when running `git view --help`")

## License

[MIT License](LICENSE)

## Acknowledgement

The idea for this project came about from the existing [git-open](https://github.com/paulirish/git-open/blob/master/git-open) github repository

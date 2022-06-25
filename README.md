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



### Windows

### *nix / macOS

## Usage

```commandline
$ git view -h

...

USAGE:
    git-view.exe [OPTIONS]

OPTIONS:
    -r, --remote <name>    The remote to view git repository on
    -b, --branch <name>    The branch to view git repository on
    -c, --commit <hash>    The commit to view git repository on
    -p, --print            Print the URL (doesn't open browser)
    -h, --help             Print help information
    -V, --version          Print version information
```

## License

[MIT License](LICENSE)

## Acknowledgement

The idea for this project came about from the existing [git-open](https://github.com/paulirish/git-open/blob/master/git-open) github repository

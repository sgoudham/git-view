# git-view

[![build](https://github.com/sgoudham/git-view/actions/workflows/build.yml/badge.svg)](https://github.com/sgoudham/git-view/actions/workflows/build.yml)
[![crate.io](https://img.shields.io/crates/v/git-view)](https://crates.io/crates/git-view)
[![downloads](https://img.shields.io/crates/d/git-view)](https://crates.io/crates/git-view)
[![license](https://img.shields.io/github/license/sgoudham/git-view)](LICENSE)

> A git sub-command to open your git repository in the web browser!

## Table Of Contents

TODO

## About

Are you frustrated from moving your hands away from the keyboard to open your git repository in the browser? 

> Me too!

`git-view` allows you to do _just that._

_Note: The use of a mouse or trackpad may be required to navigate the browser :P_

## Features

TODO

## Installation

TODO

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

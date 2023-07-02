# git-view

[![build](https://github.com/sgoudham/git-view/actions/workflows/build.yml/badge.svg)](https://github.com/sgoudham/git-view/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/git-view)](https://crates.io/crates/git-view)
[![downloads](https://img.shields.io/crates/d/git-view)](https://crates.io/crates/git-view)

> A git sub-command to view your git repository in the web browser!

## About

Are you _**also**_ frustrated from moving your hands away from the keyboard to view your git repository in the browser?

> Me too!!!

`git-view` alleviates that pain by allowing you to chuck away your mouse!

> **Note:** <br>
> You should always use `git view -h` instead of `git view --help` as the manpage/html files are **NOT** included.

## Features

- [x] View Branches, Commits & Issues
- [x] Custom Suffix
- [x] Custom Remote
- [x] View Current Directory

## Installation

Binaries are available [here](https://github.com/sgoudham/git-view/releases/latest).

### Cargo

```shell
cargo install git-view
```

### Homebrew

```shell
brew tap sgoudham/tap
brew install git-view
```

## Usage

```shell
$ git view
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/CURRENT_BRANCH


$ git view --remote remote
# https://github.com/PROVIDED_REMOTE_USER/REPO/tree/CURRENT_BRANCH

$ git view --remote remote --branch branch
# https://github.com/PROVIDED_REMOTE_USER/REPO/tree/PROVIDED_BRANCH

$ git view --issue
# Given branch 'TICKET-123' or some other variation
# https://github.com/TRACKED_REMOTE_USER/REPO/issues/123

$ git view --issue 42
# https://github.com/TRACKED_REMOTE_USER/REPO/issues/42

$ git view --commit
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/CURRENT_COMMIT

$ git view --commit efa38be50ad34d
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/efa38be50ad34d

$ git view --commit efa38be50ad34d --path src/lib.rs
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/efa38be50ad34d/src/main.rs

$ git view --path
# Given working directory 'src/lib.rs'
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/CURRENT_BRANCH/src/main.rs

$ git view --path CONTRIBUTING.md
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/CURRENT_BRANCH/CONTRIBUTING.md

$ git view --path CONTRIBUTING.md --branch testing
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/PROVIDED_BRANCH/CONTRIBUTING.md

$ git view --suffix releases
# Given branch 'TICKET-123' or some other variation
# https://github.com/TRACKED_REMOTE_USER/REPO/releases

$ git view --print
# prints https://github.com/TRACKED_REMOTE_USER/REPO/tree/CURRENT_BRANCH
```

## Help

```shell
git-view 1.0.0
Goudham Suresh <sgoudham@gmail.com>
A git sub-command to view your git repository in the web browser

USAGE:
    git-view [OPTIONS]

OPTIONS:
    -r, --remote <name>      The remote to view git repository on
                             [default: default remote]
    -b, --branch <name>      The branch to view git repository on
                             [default: current branch]
    -i, --issue <number>     The issue number to view on the git repository
                             [default: open issue from current branch]
    -c, --commit <hash>      The commit to view git repository on
                             [default: current commit]
    -p, --path <path>        The directory/file to view on the git repository
                             [default: current working directory]
    -s, --suffix <suffix>    A suffix to append onto the base git repository URL
        --print              Don't open browser and print the URL
    -h, --help               Print help information
    -V, --version            Print version information
```

## Contributing

Please read the [CONTRIBUTING.md](./CONTRIBUTING.md) before contributing!

## License

[MIT](LICENSE)

## Acknowledgement

The idea for this project came about from an existing project [git-open](https://github.com/paulirish/git-open/blob/master/git-open)

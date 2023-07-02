<h1 align="center">
    git-view (
    <a href="https://github.com/sgoudham/git-view/actions/workflows/build.yml"><img src="https://github.com/sgoudham/git-view/actions/workflows/build.yml/badge.svg"></a> )
</h1>

<p align="center">
    <a href="https://github.com/sgoudham/git-view/stargazers"><img src="https://img.shields.io/github/stars/sgoudham/git-view?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
	<a href="https://crates.io/crates/git-view"><img src="https://img.shields.io/crates/v/git-view?colorA=363a4f&colorB=ee99a0&style=for-the-badge"></a>
	<a href="https://crates.io/crates/git-view"><img src="https://img.shields.io/crates/d/git-view?colorA=363a4f&colorB=eed49f&style=for-the-badge"></a>
</p>

## About

Are you _**also**_ frustrated from moving your hands away from the keyboard to view your git repository on GitHub?

> Me too!!!

`git-view` alleviates that pain by allowing you to chuck away your mouse and instead open it via the CLI!

> **Note:** <br>
> You should always use `git view -h` instead of `git view --help` as the manpage/html files are **NOT** included.

## Features

- [x] View Branches, Commits & Issues
- [x] Custom Remote
- [x] Custom Directory

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
# Branch 'TICKET-123'
# https://github.com/TRACKED_REMOTE_USER/REPO/issues/123

$ git view --issue 42
# https://github.com/TRACKED_REMOTE_USER/REPO/issues/42

$ git view --commit
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/CURRENT_COMMIT

$ git view --commit efa38be50ad34d
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/efa38be50ad34d

$ git view --commit efa38be50ad34d --path src/lib.rs
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/efa38be50ad34d/src/lib.rs

$ git view --path
# Working Directory 'src/lib.rs'
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/CURRENT_BRANCH/src/lib.rs

$ git view --path CONTRIBUTING.md
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/CURRENT_BRANCH/CONTRIBUTING.md

$ git view --path CONTRIBUTING.md --branch testing
# https://github.com/TRACKED_REMOTE_USER/REPO/tree/PROVIDED_BRANCH/CONTRIBUTING.md

$ git view --print
# prints https://github.com/TRACKED_REMOTE_USER/REPO/tree/CURRENT_BRANCH
```

## Help

```shell
git-view 1.0.0
Goudham Suresh <sgoudham@gmail.com>
A git sub-command to view your git repository on GitHub

USAGE:
    git-view [OPTIONS]

OPTIONS:
    -r, --remote <name>     The remote to view on GitHub
                            [default: default remote]
    -b, --branch <name>     The branch to view on GitHub
                            [default: current branch]
    -i, --issue <number>    The GitHub issue number
                            [default: number from current branch]
    -c, --commit <hash>     The commit to view on GitHub
                            [default: current commit]
    -p, --path <path>       The directory/file to view on GitHub
                            [default: current working directory]
        --print             Don't open GitHub and print URL
    -h, --help              Print help information
    -V, --version           Print version information
```

## Contributing

Please read the [CONTRIBUTING.md](./CONTRIBUTING.md) before contributing!

## License

[MIT](LICENSE)

## Acknowledgement

The idea for this project came about from an existing project [git-open](https://github.com/paulirish/git-open/blob/master/git-open)

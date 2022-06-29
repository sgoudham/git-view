# git-view

[![build](https://github.com/sgoudham/git-view/actions/workflows/build.yml/badge.svg)](https://github.com/sgoudham/git-view/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/git-view)](https://crates.io/crates/git-view)
[![downloads](https://img.shields.io/crates/d/git-view)](https://crates.io/crates/git-view)
[![license](https://img.shields.io/github/license/sgoudham/git-view)](LICENSE)

> A git sub-command to view your git repository in the web browser!

## About

Are you _**also**_ frustrated from moving your hands away from the keyboard to view your git repository in the browser? 

> Me too!

`git-view` alleviates that pain by allowing you to chuck away your mouse!

> (n)vim users rejoice :P

**_Important Note: You should always use `git view -h` instead of `git view --help` as the manpage/html files are NOT included._**

## Features

- [x] GitHub & BitBucket
- [x] View Branches, Commits & Issues
- [x] Custom Suffix
- [x] Custom Remote
- [ ] View Profile
- [ ] View Current Directory

Feel free to raise any issues or pull requests (after having read the [CONTRIBUTING.md](./CONTRIBUTING.md)!) for any additional features
that you want!

## Usage

![Usage](./docs/images/usage.png "Displays different usages of `git-view`")

## Installation

### Cargo

The _preferred_ way of installation is to manually install the provided binaries and update your `$PATH` variable to enable
the usage as `git view` globally. However, that being said, it also available on [crates.io](https://crates.io/crates/git-view) to allow installation 
through the use of Rust's build tool and package manager `cargo`.

> If you do not have `cargo` available on your machine, you can download it [here](https://www.rust-lang.org/tools/install)

```shell
$ cargo install git-view
```

Refresh terminal & verify installation

```shell
$ git view --version
git-view 0.1.0
```

### Homebrew

For `macOS` users, installation through [Homebrew](https://brew.sh/) is recommended.

```shell
$ brew tap sgoudham/tap
$ brew install git-view
```

Refresh terminal & verify installation

```shell
$ git view --version
git-view 0.1.0
```

### Binaries

Pre-compiled binaries are _always_ available with every single [release](https://github.com/sgoudham/git-view/releases) for **Windows**, **macOS** and **Linux**.

The examples shown below will showcase the installation of the binaries living within the local `git` directory but realistically, any path will
work if updated correctly within `$PATH`.

#### Windows

1. Download either `git-view-x86_64-pc-windows-msvc.zip` or `git-view-x86_64-pc-windows-gnu.zip`

2. Find local `git` directory

```shell
# CMD
$ where git
C:\Program Files\Git\cmd\git.exe

# PowerShell
$ (Get-Command git.exe).Path
C:\Program Files\Git\cmd\git.exe
```

3. `cd` into above path & extract downloaded binary zip

```shell
$ cd 'C:\Program Files\Git\cmd'

$ tar -xf git-view-x86_64-pc-windows-msvc.zip
# OR
$ tar -xf git-view-x86_64-pc-windows-gnu.zip
```

4. Ensure `%PATH%` is updated

```shell
# Only required if git-view exists within a path not already included within %PATH%
$ setx path "%path%;C:\your\path\here\bin"
```

5. Refresh terminal and verify installation

```shell
$ git view --version
git-view 0.1.0
```

#### Linux / macOS
1. Download `git-view-x86_64-unknown-linux-gnu.tar.gz` or `git-view-x86_64-unknown-linux-musl.tar.gz`
   or `git-view-x86_64-apple-darwin.tar.gz`

2. Extract into your local directory

```shell
# Linux
$ tar -xf git-view-x86_64-unknown-linux-gnu.tar.gz
$ tar -xf git-view-x86_64-unknown-linux-musl.tar.gz

# macOS
$ tar -xf git-view-x86_64-apple-darwin.tar.gz
```

3. Move into `~/bin`

```shell
# Create ~/bin if it does not exist
$ mkdir -p ~/bin
$ mv git-view ~/bin
```

4. Set permissions for executable

```shell
$ chmod 755 ~/bin/git-view
```

5. Ensure `$PATH` is updated

```shell
# Only required if git-view exists within a path not already included within $PATH

# Linux
$ echo 'export PATH=~/bin:$PATH' >> ~/.bashrc 
$ source ~/.bashrc

# macOS
$ echo 'export PATH=~/bin:$PATH' >> ~/.bash_profile
$ source ~/.bash_profile
```

6. Verify installation

```shell
$ git view --version
git-view 0.1.0
```

## Help

![help](./docs/images/help.png "Contents displayed when running `git view -h`")

## Contributing 

First, thanks for your interest in contributing to this project! Please read the [CONTRIBUTING.md](./CONTRIBUTING.md) before contributing!

## License

[MIT License](LICENSE)

## Acknowledgement

The idea for this project came about from an existing project [git-open](https://github.com/paulirish/git-open/blob/master/git-open)

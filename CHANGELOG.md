# CHANGELOG

All notable changes to this project will be documented in this file.

## [v1.0.0](https://github.com/sgoudham/git-view/releases/tag/v1.0.0) - 2023-07-04


### **Breaking Changes**

- ([#4](https://github.com/sgoudham/git-view/issues/4)): Drop support for BitBucket URLs ([f6a6c7f](https://github.com/sgoudham/git-view/commit/f6a6c7f))

  > This removes support for BitBucket URLs
  > as I am not well versed with them to robustly support it


- Throw error when missing default branch ([52e0e82](https://github.com/sgoudham/git-view/commit/52e0e82))

  > Previously, the missing default
  > branch was ignored and the user given branch was
  > used which resulted in a 404.
  > 
  > This commit makes it so that it throws up an error
  > instead of letting the user see the 404.

- Remove `--suffix` & revamp help ([3335011](https://github.com/sgoudham/git-view/commit/3335011))

  > I couldn't decide on the
  > semantics of what this command should have
  > so, instead, I've opted to remove it.



### Features

- Add nix flake ([cc9dd39](https://github.com/sgoudham/git-view/commit/cc9dd39))
- Implement `--path` option ([41ad13e](https://github.com/sgoudham/git-view/commit/41ad13e))
- Allow args for `--issue` ([3f9c7bd](https://github.com/sgoudham/git-view/commit/3f9c7bd))


### Bug Fixes

- (nix): 4th July nightly isn't available yet ([f959a91](https://github.com/sgoudham/git-view/commit/f959a91))
- Potentially solve build errors ([436b03f](https://github.com/sgoudham/git-view/commit/436b03f))
- Stop triggering workflow twice ([1e07be9](https://github.com/sgoudham/git-view/commit/1e07be9))
- Remove '--issue 256' example from usage.png ([1a3f4c8](https://github.com/sgoudham/git-view/commit/1a3f4c8))


### Refactor

- (nix): Pin nightly and use in `nix develop` ([21f646c](https://github.com/sgoudham/git-view/commit/21f646c))
- (nix): Follow inputs for `crane` ([f486dd1](https://github.com/sgoudham/git-view/commit/f486dd1))
- (nix): Remove `overrideAttrs` ([99f8735](https://github.com/sgoudham/git-view/commit/99f8735))
- (nix): Try using `overrideAttrs` ([d5875b5](https://github.com/sgoudham/git-view/commit/d5875b5))
- (nix): Try adding `name` ([24afcec](https://github.com/sgoudham/git-view/commit/24afcec))
- (nix): Use rust-toolchain.toml ([75446b8](https://github.com/sgoudham/git-view/commit/75446b8))
- Add info message for `--branch` ([11d7470](https://github.com/sgoudham/git-view/commit/11d7470))
- `--print` help message ([a6cfa67](https://github.com/sgoudham/git-view/commit/a6cfa67))
- Improve usage docs & help ([98cbfcc](https://github.com/sgoudham/git-view/commit/98cbfcc))
- Fix trailing slashes in  `--path` ([367c142](https://github.com/sgoudham/git-view/commit/367c142))
- Simplify docs and tidy up ([8fb5bbf](https://github.com/sgoudham/git-view/commit/8fb5bbf))
- Tidy up code ([60e5a22](https://github.com/sgoudham/git-view/commit/60e5a22))


### Documentation

- (CHANGELOG): Update release notes ([56c6f5f](https://github.com/sgoudham/git-view/commit/56c6f5f))
- (CHANGELOG): Update release notes ([960e07c](https://github.com/sgoudham/git-view/commit/960e07c))
- (README): Remove `;` from quote ([40e9d1d](https://github.com/sgoudham/git-view/commit/40e9d1d))
- (README): Reformat ([ee03671](https://github.com/sgoudham/git-view/commit/ee03671))
- (README): Reformat quote ([df4f811](https://github.com/sgoudham/git-view/commit/df4f811))
- (README): Add `nix run` command ([45fb211](https://github.com/sgoudham/git-view/commit/45fb211))
- (README): New shield.io badges ([df2d557](https://github.com/sgoudham/git-view/commit/df2d557))
- (README): Reformat ([48b3d82](https://github.com/sgoudham/git-view/commit/48b3d82))
- Remove inaccurate changelog ([1e8704c](https://github.com/sgoudham/git-view/commit/1e8704c))
- Simplify wording ([aaeebb1](https://github.com/sgoudham/git-view/commit/aaeebb1))
- Reformat README.md ([e597e25](https://github.com/sgoudham/git-view/commit/e597e25))
- Update formatting in README.md ([7b5f0e4](https://github.com/sgoudham/git-view/commit/7b5f0e4))


### Build

- (cargo): Update `Cargo.lock` ([7ca41ba](https://github.com/sgoudham/git-view/commit/7ca41ba))
- (cargo): Add `keywords` & `categories` ([a8e4397](https://github.com/sgoudham/git-view/commit/a8e4397))
- (cargo): Only package relevant files ([30d443c](https://github.com/sgoudham/git-view/commit/30d443c))
- (deps): Bump webbrowser from 0.8.1 to 0.8.2 ([4ea6a82](https://github.com/sgoudham/git-view/commit/4ea6a82))
- (deps): Bump test-case from 2.2.1 to 2.2.2 ([75040c6](https://github.com/sgoudham/git-view/commit/75040c6))
- (deps): Bump webbrowser from 0.8.0 to 0.8.1 ([d272f7c](https://github.com/sgoudham/git-view/commit/d272f7c))
- (deps): Bump webbrowser from 0.7.1 to 0.8.0 ([#12](https://github.com/sgoudham/git-view/issues/12)) ([7b08d32](https://github.com/sgoudham/git-view/commit/7b08d32))
- (deps): Bump url from 2.2.2 to 2.3.1 ([#11](https://github.com/sgoudham/git-view/issues/11)) ([3813592](https://github.com/sgoudham/git-view/commit/3813592))
- (deps): Bump mockall from 0.11.1 to 0.11.2 ([#9](https://github.com/sgoudham/git-view/issues/9)) ([eafdb9a](https://github.com/sgoudham/git-view/commit/eafdb9a))
- (deps): Bump test-case from 2.1.0 to 2.2.1 ([#8](https://github.com/sgoudham/git-view/issues/8)) ([1bc685f](https://github.com/sgoudham/git-view/commit/1bc685f))
- Rename `build` -> `release` ([be5d6c8](https://github.com/sgoudham/git-view/commit/be5d6c8))
- Overhaul ci/cd pipelines ([6ee5869](https://github.com/sgoudham/git-view/commit/6ee5869))
- Update version number to v1.0.0 ([53b0a2e](https://github.com/sgoudham/git-view/commit/53b0a2e))
- Remove 'edited' trigger from generating CHANGELOG ([1caef9b](https://github.com/sgoudham/git-view/commit/1caef9b))


### Deployment

- Disable auto homebrew releases for now ([58a7647](https://github.com/sgoudham/git-view/commit/58a7647))
- Avoid duplicate triggers ([ad06b1c](https://github.com/sgoudham/git-view/commit/ad06b1c))
- Downgrade `test-case` to 3.0.0 as potential fix ([e3db04e](https://github.com/sgoudham/git-view/commit/e3db04e))
- Remove `stable` toolchain from ci ([75853c6](https://github.com/sgoudham/git-view/commit/75853c6))
- Allow manual bump of homebrew formula ([4fe0ec6](https://github.com/sgoudham/git-view/commit/4fe0ec6))
- Change name to 'generate-changelog' ([aedf339](https://github.com/sgoudham/git-view/commit/aedf339))
- Update to bump-homebrew-formula-action@v2 ([97ea9dd](https://github.com/sgoudham/git-view/commit/97ea9dd))
- Generate changelog when release is published ([949ccdf](https://github.com/sgoudham/git-view/commit/949ccdf))
- Update build paths & remove scripts/ ([9eb9d81](https://github.com/sgoudham/git-view/commit/9eb9d81))
- Cache Dependencies for GH Actions ([d19ed94](https://github.com/sgoudham/git-view/commit/d19ed94))


### Miscellaneous Tasks

- (deps): Bump clap-rs to `3.2.25` ([63fb8ce](https://github.com/sgoudham/git-view/commit/63fb8ce))
- (deps): Update testcase to `3.1.0` again ([ea00704](https://github.com/sgoudham/git-view/commit/ea00704))
- (deps): Update webbrowser to `0.8.10` ([55ae17f](https://github.com/sgoudham/git-view/commit/55ae17f))
- Fix rustfmt ([ead99f0](https://github.com/sgoudham/git-view/commit/ead99f0))
- Remove `.vscode` from source control ([8bb920e](https://github.com/sgoudham/git-view/commit/8bb920e))
- Reduce dependabot noise ([b72a5a3](https://github.com/sgoudham/git-view/commit/b72a5a3))
- Start tracking Cargo.lock ([b58975d](https://github.com/sgoudham/git-view/commit/b58975d))
- Unignore Cargo.lock ([8e35966](https://github.com/sgoudham/git-view/commit/8e35966))



## [v0.1.0](https://github.com/sgoudham/git-view/releases/tag/v0.1.0) - 2022-06-26


### Features

- Add support for suffix ([b5572a5](https://github.com/sgoudham/git-view/commit/b5572a5))
- Add support for BitBucket repositories ([7a5afc7](https://github.com/sgoudham/git-view/commit/7a5afc7))
- Open default remote branch if no upstream exists ([80465ec](https://github.com/sgoudham/git-view/commit/80465ec))
- Add functionality to open issue links ([6e1ecf3](https://github.com/sgoudham/git-view/commit/6e1ecf3))
- Add functionality for opening commit hashes ([9426dec](https://github.com/sgoudham/git-view/commit/9426dec))
- Add argument 'issue' to open issues ([ba6e9cf](https://github.com/sgoudham/git-view/commit/ba6e9cf))
- Parse URL & (naively) open the url in the browser ([26d9e5b](https://github.com/sgoudham/git-view/commit/26d9e5b))
- Parse URL in a robust way & start adding tests ([9011c18](https://github.com/sgoudham/git-view/commit/9011c18))
- Ensure that arguments are correctly parsed ([cdaab08](https://github.com/sgoudham/git-view/commit/cdaab08))
- Add MVP git-browser ([b46891e](https://github.com/sgoudham/git-view/commit/b46891e))


### Bug Fixes

- Change project name to 'git-view' ([a227adf](https://github.com/sgoudham/git-view/commit/a227adf))
- Change commit from 'latest' to 'current' ([af001e0](https://github.com/sgoudham/git-view/commit/af001e0))


### Refactor

- Reformat '-h' flag output ([efa38be](https://github.com/sgoudham/git-view/commit/efa38be))
- Move GitViewBuilder into module 'lib_tests' ([8bb5abe](https://github.com/sgoudham/git-view/commit/8bb5abe))
- Change 'to_owned()' to 'into()' ([d930d79](https://github.com/sgoudham/git-view/commit/d930d79))
- Implement functionality to retrieve git default branch ([784aee1](https://github.com/sgoudham/git-view/commit/784aee1))
- Add GitTrait to allow for easier testing ([f830f91](https://github.com/sgoudham/git-view/commit/f830f91))
- Add issue argument and start generating final url ([7be0788](https://github.com/sgoudham/git-view/commit/7be0788))
- No need to store domain string in enum ([ac48831](https://github.com/sgoudham/git-view/commit/ac48831))
- Use Cow<'_, str> and Url/Domain structs ([de44601](https://github.com/sgoudham/git-view/commit/de44601))
- Add Url & Domain structs ([34cb2b6](https://github.com/sgoudham/git-view/commit/34cb2b6))
- Don't map to String anymore ([8f7210a](https://github.com/sgoudham/git-view/commit/8f7210a))
- Perform massive refactor ([95953b8](https://github.com/sgoudham/git-view/commit/95953b8))
- Use 'AppError' for propagation to clap ([ce41387](https://github.com/sgoudham/git-view/commit/ce41387))
- Add 'AppError' to allow cleaner propagation of different errors ([c632965](https://github.com/sgoudham/git-view/commit/c632965))
- Setup git-remote walking skeleton ([e11e88b](https://github.com/sgoudham/git-view/commit/e11e88b))


### Documentation

- Update README.md ([c66ab8a](https://github.com/sgoudham/git-view/commit/c66ab8a))
- Add 'usage.png' showing different usages ([dee14a2](https://github.com/sgoudham/git-view/commit/dee14a2))
- Add image for 'git view -h' ([b1112bb](https://github.com/sgoudham/git-view/commit/b1112bb))
- Add CONTRIBUTING.md ([cf4d4f6](https://github.com/sgoudham/git-view/commit/cf4d4f6))
- Update README.md ([8077689](https://github.com/sgoudham/git-view/commit/8077689))
- Update README ([c8c7bdd](https://github.com/sgoudham/git-view/commit/c8c7bdd))
- Fix formatting in README.md ([d37764c](https://github.com/sgoudham/git-view/commit/d37764c))
- WIP Update README.md ([6859b93](https://github.com/sgoudham/git-view/commit/6859b93))
- Update README.md to match new repository name ([78d72ce](https://github.com/sgoudham/git-view/commit/78d72ce))


### Testing

- Add tests for 'get_git_url' ([ddf0237](https://github.com/sgoudham/git-view/commit/ddf0237))
- Add tests for populate_remote() ([ba0eb0f](https://github.com/sgoudham/git-view/commit/ba0eb0f))
- Add tests for get_local_ref() ([5370658](https://github.com/sgoudham/git-view/commit/5370658))
- Add unit tests for 'parse_git_url()' ([c71a9d2](https://github.com/sgoudham/git-view/commit/c71a9d2))


### Build

- Add workflow for auto-generating CHANGELOGs ([47c25dd](https://github.com/sgoudham/git-view/commit/47c25dd))
- Build pipeline based off path filters ([ca9a870](https://github.com/sgoudham/git-view/commit/ca9a870))
- Remove 'unicode-segmentation' dependency ([f62f74a](https://github.com/sgoudham/git-view/commit/f62f74a))
- Add dev-dependency 'test-case' ([7bd38ad](https://github.com/sgoudham/git-view/commit/7bd38ad))
- Add dependency 'url:2.2.2' to allow the parsing of URLs ([db5f27c](https://github.com/sgoudham/git-view/commit/db5f27c))
- Update 'git-browser' to 'git-view' ([5f0f2ff](https://github.com/sgoudham/git-view/commit/5f0f2ff))
- Update 'git-remote' to 'git-browser' within repository url ([f151f58](https://github.com/sgoudham/git-view/commit/f151f58))
- Setup crate info & dependencies ([3643e1f](https://github.com/sgoudham/git-view/commit/3643e1f))


### Deployment

- Add scripts & build/deploy.yml ([a49f562](https://github.com/sgoudham/git-view/commit/a49f562))


### Miscellaneous Tasks

- Ignore Intellij folder ([3fbd6e2](https://github.com/sgoudham/git-view/commit/3fbd6e2))
- Tidy up .gitignore ([3fb0c0c](https://github.com/sgoudham/git-view/commit/3fb0c0c))
- Create dependabot.yml ([1dd3500](https://github.com/sgoudham/git-view/commit/1dd3500))




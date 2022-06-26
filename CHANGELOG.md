# CHANGELOG

## [v0.1.0](https://github.com/sgoudham/git-view/releases/tag/v0.1.0) - 2022-06-26 02:51:53

# CHANGELOG

## [v0.1.0](https://github.com/sgoudham/git-view/releases/tag/v0.1.0) - 2022-06-26 02:51:53

## What's Changed
* v0.1.0 by @sgoudham in https://github.com/sgoudham/git-view/pull/1

## New Contributors
* @sgoudham made their first contribution in https://github.com/sgoudham/git-view/pull/1

**Full Changelog**: https://github.com/sgoudham/git-view/commits/v0.1.0

### Feature

- general:
  - Add support for suffix ([b5572a5](https://github.com/sgoudham/git-view/commit/b5572a5adc407e2f10d3671e1bdc5b6c8daaecc4)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add support for BitBucket repositories ([7a5afc7](https://github.com/sgoudham/git-view/commit/7a5afc7354a5aaa9fff57e8d92f1d8af647a66c1)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Open default remote branch if no upstream exists ([80465ec](https://github.com/sgoudham/git-view/commit/80465ec51488544d6d7f0ff6f39c64b4697994c8)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add functionality to open issue links ([6e1ecf3](https://github.com/sgoudham/git-view/commit/6e1ecf3657ba5a250d2f378dffbd3f271dee6f7b)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add functionality for opening commit hashes ([9426dec](https://github.com/sgoudham/git-view/commit/9426dec6ce8577403c536a99b8996a848f07f0d3)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add argument 'issue' to open issues ([ba6e9cf](https://github.com/sgoudham/git-view/commit/ba6e9cf95a4a595b6bc396529c16cd932da11ea7)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Parse URL & (naively) open the url in the browser ([26d9e5b](https://github.com/sgoudham/git-view/commit/26d9e5bd093f0d5831c8c92e2731060a70ffe992)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Parse URL in a robust way & start adding tests ([9011c18](https://github.com/sgoudham/git-view/commit/9011c18d4952a404b4638190b25293c25f634238))
  - Ensure that arguments are correctly parsed ([cdaab08](https://github.com/sgoudham/git-view/commit/cdaab08594c6ed0edcb49d9914a04a104ea2b610))
  - Add MVP git-browser ([b46891e](https://github.com/sgoudham/git-view/commit/b46891e84172d78fdd61d7bb9ed499376e1a8e29))

### Bug Fixes

- general:
  - Change project name to 'git-view' ([a227adf](https://github.com/sgoudham/git-view/commit/a227adfbfa96e1aec07c2818fd9fd519049d3898))
  - Change commit from 'latest' to 'current' ([af001e0](https://github.com/sgoudham/git-view/commit/af001e002a1ee564091916b99ddcf99229937c3e)) ([#1](https://github.com/sgoudham/git-view/pull/1))

### Documentation

- general:
  - Update README.md ([c66ab8a](https://github.com/sgoudham/git-view/commit/c66ab8a41e94861531eda4384e5400570f6e7ac5)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add 'usage.png' showing different usages ([dee14a2](https://github.com/sgoudham/git-view/commit/dee14a2f79d85b6d3da2987612e2acf4c288514a)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add image for 'git view -h' ([b1112bb](https://github.com/sgoudham/git-view/commit/b1112bbddc1d7de9dfb5b85aae6697da1a20d3f0)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add CONTRIBUTING.md ([cf4d4f6](https://github.com/sgoudham/git-view/commit/cf4d4f68b2d1f6c80f2d01a29f66649b356a75ac)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Update README.md ([8077689](https://github.com/sgoudham/git-view/commit/8077689388814f4e7e2cd0af407ce0e82d9bc898)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Update README ([c8c7bdd](https://github.com/sgoudham/git-view/commit/c8c7bdd1bde85484b324cb898b83f9c07c82208e)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Fix formatting in README.md ([d37764c](https://github.com/sgoudham/git-view/commit/d37764c5a50e4252096a0f8f46dfc25e05136e31)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - WIP Update README.md ([6859b93](https://github.com/sgoudham/git-view/commit/6859b930ddfedc8439d8cfa404738e40f7e74892)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Update README.md to match new repository name ([78d72ce](https://github.com/sgoudham/git-view/commit/78d72cef002838452127b4cb593e6af0e5bc11e5))

### Refactor

- general:
  - Reformat '-h' flag output ([efa38be](https://github.com/sgoudham/git-view/commit/efa38be50ad34dd3a14c6dad52d678aae1b1837f)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Move GitViewBuilder into module 'lib_tests' ([8bb5abe](https://github.com/sgoudham/git-view/commit/8bb5abe048bee803d5191c12dbe71f5110b78073)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Change 'to_owned()' to 'into()' ([d930d79](https://github.com/sgoudham/git-view/commit/d930d7992f2af7a06a6f5edfb351280081099338)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Implement functionality to retrieve git default branch ([784aee1](https://github.com/sgoudham/git-view/commit/784aee17d37d5f8e092efcc9ce8499d939d9ce6a)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add GitTrait to allow for easier testing ([f830f91](https://github.com/sgoudham/git-view/commit/f830f914c5932681095ab09d9c9999f762b0eb54)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add issue argument and start generating final url ([7be0788](https://github.com/sgoudham/git-view/commit/7be0788dc72e3f6ec0fa0ffe1925a8e4a77121ac)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - No need to store domain string in enum ([ac48831](https://github.com/sgoudham/git-view/commit/ac48831a862ef3b9703f6877032b866ae6d06708)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Use Cow<'_, str> and Url/Domain structs ([de44601](https://github.com/sgoudham/git-view/commit/de446012c1364a27a27dc8a4f23f2ec04bd76c91)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add Url & Domain structs ([34cb2b6](https://github.com/sgoudham/git-view/commit/34cb2b6ff5fca88a9b2e7caff388854d4c7a7259)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Don't map to String anymore ([8f7210a](https://github.com/sgoudham/git-view/commit/8f7210a17e17abc1d8c332a5b730e1fd54e292e6)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Perform massive refactor ([95953b8](https://github.com/sgoudham/git-view/commit/95953b87f30da5e269ca0c54995e03179c0d6228)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Use 'AppError' for propagation to clap ([ce41387](https://github.com/sgoudham/git-view/commit/ce4138712e145d1740261cbcac8f286c340e9917)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add 'AppError' to allow cleaner propagation of different errors ([c632965](https://github.com/sgoudham/git-view/commit/c632965a2ac4b8e0115e8d72301f9f486e778869)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Setup git-remote walking skeleton ([e11e88b](https://github.com/sgoudham/git-view/commit/e11e88b02b5f1d65c076eb1ae29f4046ae8a29be))

\* *This CHANGELOG was automatically generated by [auto-generate-changelog](https://github.com/BobAnkh/auto-generate-changelog)*

### Feature

- general:
  - Add support for suffix ([b5572a5](https://github.com/sgoudham/git-view/commit/b5572a5adc407e2f10d3671e1bdc5b6c8daaecc4)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add support for BitBucket repositories ([7a5afc7](https://github.com/sgoudham/git-view/commit/7a5afc7354a5aaa9fff57e8d92f1d8af647a66c1)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Open default remote branch if no upstream exists ([80465ec](https://github.com/sgoudham/git-view/commit/80465ec51488544d6d7f0ff6f39c64b4697994c8)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add functionality to open issue links ([6e1ecf3](https://github.com/sgoudham/git-view/commit/6e1ecf3657ba5a250d2f378dffbd3f271dee6f7b)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add functionality for opening commit hashes ([9426dec](https://github.com/sgoudham/git-view/commit/9426dec6ce8577403c536a99b8996a848f07f0d3)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add argument 'issue' to open issues ([ba6e9cf](https://github.com/sgoudham/git-view/commit/ba6e9cf95a4a595b6bc396529c16cd932da11ea7)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Parse URL & (naively) open the url in the browser ([26d9e5b](https://github.com/sgoudham/git-view/commit/26d9e5bd093f0d5831c8c92e2731060a70ffe992)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Parse URL in a robust way & start adding tests ([9011c18](https://github.com/sgoudham/git-view/commit/9011c18d4952a404b4638190b25293c25f634238))
  - Ensure that arguments are correctly parsed ([cdaab08](https://github.com/sgoudham/git-view/commit/cdaab08594c6ed0edcb49d9914a04a104ea2b610))
  - Add MVP git-browser ([b46891e](https://github.com/sgoudham/git-view/commit/b46891e84172d78fdd61d7bb9ed499376e1a8e29))

### Bug Fixes

- general:
  - Change project name to 'git-view' ([a227adf](https://github.com/sgoudham/git-view/commit/a227adfbfa96e1aec07c2818fd9fd519049d3898))
  - Change commit from 'latest' to 'current' ([af001e0](https://github.com/sgoudham/git-view/commit/af001e002a1ee564091916b99ddcf99229937c3e)) ([#1](https://github.com/sgoudham/git-view/pull/1))

### Documentation

- general:
  - Update README.md ([c66ab8a](https://github.com/sgoudham/git-view/commit/c66ab8a41e94861531eda4384e5400570f6e7ac5)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add 'usage.png' showing different usages ([dee14a2](https://github.com/sgoudham/git-view/commit/dee14a2f79d85b6d3da2987612e2acf4c288514a)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add image for 'git view -h' ([b1112bb](https://github.com/sgoudham/git-view/commit/b1112bbddc1d7de9dfb5b85aae6697da1a20d3f0)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add CONTRIBUTING.md ([cf4d4f6](https://github.com/sgoudham/git-view/commit/cf4d4f68b2d1f6c80f2d01a29f66649b356a75ac)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Update README.md ([8077689](https://github.com/sgoudham/git-view/commit/8077689388814f4e7e2cd0af407ce0e82d9bc898)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Update README ([c8c7bdd](https://github.com/sgoudham/git-view/commit/c8c7bdd1bde85484b324cb898b83f9c07c82208e)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Fix formatting in README.md ([d37764c](https://github.com/sgoudham/git-view/commit/d37764c5a50e4252096a0f8f46dfc25e05136e31)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - WIP Update README.md ([6859b93](https://github.com/sgoudham/git-view/commit/6859b930ddfedc8439d8cfa404738e40f7e74892)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Update README.md to match new repository name ([78d72ce](https://github.com/sgoudham/git-view/commit/78d72cef002838452127b4cb593e6af0e5bc11e5))

### Refactor

- general:
  - Reformat '-h' flag output ([efa38be](https://github.com/sgoudham/git-view/commit/efa38be50ad34dd3a14c6dad52d678aae1b1837f)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Move GitViewBuilder into module 'lib_tests' ([8bb5abe](https://github.com/sgoudham/git-view/commit/8bb5abe048bee803d5191c12dbe71f5110b78073)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Change 'to_owned()' to 'into()' ([d930d79](https://github.com/sgoudham/git-view/commit/d930d7992f2af7a06a6f5edfb351280081099338)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Implement functionality to retrieve git default branch ([784aee1](https://github.com/sgoudham/git-view/commit/784aee17d37d5f8e092efcc9ce8499d939d9ce6a)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add GitTrait to allow for easier testing ([f830f91](https://github.com/sgoudham/git-view/commit/f830f914c5932681095ab09d9c9999f762b0eb54)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add issue argument and start generating final url ([7be0788](https://github.com/sgoudham/git-view/commit/7be0788dc72e3f6ec0fa0ffe1925a8e4a77121ac)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - No need to store domain string in enum ([ac48831](https://github.com/sgoudham/git-view/commit/ac48831a862ef3b9703f6877032b866ae6d06708)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Use Cow<'_, str> and Url/Domain structs ([de44601](https://github.com/sgoudham/git-view/commit/de446012c1364a27a27dc8a4f23f2ec04bd76c91)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add Url & Domain structs ([34cb2b6](https://github.com/sgoudham/git-view/commit/34cb2b6ff5fca88a9b2e7caff388854d4c7a7259)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Don't map to String anymore ([8f7210a](https://github.com/sgoudham/git-view/commit/8f7210a17e17abc1d8c332a5b730e1fd54e292e6)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Perform massive refactor ([95953b8](https://github.com/sgoudham/git-view/commit/95953b87f30da5e269ca0c54995e03179c0d6228)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Use 'AppError' for propagation to clap ([ce41387](https://github.com/sgoudham/git-view/commit/ce4138712e145d1740261cbcac8f286c340e9917)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Add 'AppError' to allow cleaner propagation of different errors ([c632965](https://github.com/sgoudham/git-view/commit/c632965a2ac4b8e0115e8d72301f9f486e778869)) ([#1](https://github.com/sgoudham/git-view/pull/1))
  - Setup git-remote walking skeleton ([e11e88b](https://github.com/sgoudham/git-view/commit/e11e88b02b5f1d65c076eb1ae29f4046ae8a29be))

\* *This CHANGELOG was automatically generated by [auto-generate-changelog](https://github.com/BobAnkh/auto-generate-changelog)*

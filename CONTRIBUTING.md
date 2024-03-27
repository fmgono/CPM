# Contribute

👍🎉 First off, thanks for taking the time to contribute! 🎉👍

CPM is currently under heavy development. We are welcoming contributors to collaborate on CPM.

## Contribution Guidelines

We would love for you to contribute to this project.
As a contributor, here are the guidelines we would like you to follow 👇

### Be Kind - Code of Conduct

Please, read and follow our [Code of Conduct](CODE_OF_CONDUCT.md) to help us keep this project open and inclusive.

### Read the PRD

Please, read and follow our [PRD](PRD.md) to give you an idea of how this app works.

## Get involved

There are many ways to contribute to CPM, and many of them do not involve writing any code. Here are a few ideas to get started:

- Look through the [CPM issues](<(https://github.com/fmgono/CPM)/issues>). If you find an issue you would like to fix, [open a pull request](#first-pull-request).
- Help us make the docs better. File an issue if you find anything that is confusing, any grammatical error, or can be improved.
- Take a look at [GitHub Discussions](https://github.com/fmgono/CPM/discussions) and give your opinion into a discussion or consider opening a pull request if you see something you want to work on.

Contributions are very welcome!

## Our development process

CPM uses [GitHub](https://github.com/fmgono/CPM) as its source of truth. All changes will be public from the beginning.

### Reporting new issues/bugs. {#issues}

When [opening a new issue](<[https://github.com/kimlimjustin/xplorer/issues](https://github.com/fmgono/CPM)/issues>)), always make sure to fill out the issue template. We use GitHub issues to track public bugs. Please ensure your description is clear and has sufficient instructions to be able to reproduce the issue.

- _One issue, one bug_: Please report a single bug per issue.
- _Provide reproduction steps_: List all the steps necessary to reproduce the issue. The person reading your bug report should be able to follow these steps to reproduce your issue with minimal effort.

##@ Feature Request {#feat}

We use [GitHub Discussions](https://github.com/fmgono/CPM/discussions) and [GitHub Issues](https://github.com/fmgono/CPM/issues) to track ideas from users. Suggest a new feature [here](https://github.com/fmgono/CPM/discussions/new)!
Great Feature Requests tend to have:

- A quick idea summary.
- What & why do you want to add the specific feature?
- Additional references like images, links to resources about the feature, etc.

## Ready to contribute a Pull Request (PR)?

### Semantic commit messages {#commit-msg}

See how a minor change to your commit message style can make you a better programmer.

Format: `<type>(<scope>): <subject>`

`<scope>` is optional

#### Example

```
feat: allow overriding of webpack config
^--^  ^------------^
|     |
|     +-> Summary in present tense.
|
+-------> Type: chore, docs, feat, fix, refactor, style, or test.
```

the various types of commits:

- `feat`: new feature for the user
- `fix`: bug fix for the user
- `docs`: changes to the documentation
- `style`: formatting, missing semi-colons, etc.
- `refactor`: refactoring production code, eg. renaming a variable
- `test`: adding missing tests, refactoring tests.
- `chore`: updating grunt tasks etc

Use lower case not the upper case!

### Prerequisite

- [Tauri environment](https://tauri.studio/en/docs/getting-started/intro#setting-up-your-environment)
- [Node JS](https://nodejs.org/en/)
- [Git](https://git-scm.com/)
- Package manager, npm, bun, pnpm, or yarn, whatever you are comfortable with.
- Code Editor, we recommend you to use [VS Code](https://code.visualstudio.com/)

### 1. Make sure you aren't duplicating someone else's efforts.

- [Look out for existing PRs](https://github.com/fmgono/CPM/pulls)

### 2. Make sure your idea is the right way to solve the issue.

[Look out for existing issues](https://github.com/fmgono/CPM/issues) that may describe the problem you're fixing, or document the design for the feature you'd like to add.

Please, consider [creating an issue](https://github.com/fmgono/CPM/issues/new) if you can't find anything.

Discussing the design up front helps to ensure that we're ready to accept your work.

### 3. Fork this repo and create a branch.

- Hit the "Fork" button (top-right of the github repository).

![image](https://user-images.githubusercontent.com/1430726/95460679-ec014400-097d-11eb-9a7a-93e0262d37d9.png)

- git clone your fork

```shell
git clone YOUR_FORK_URL
```

Get your URL by from here 👇

![image](https://user-images.githubusercontent.com/1430726/95461173-94afa380-097e-11eb-9568-dc986e050de6.png)

- Create a new branch locally in your fork's repo

```shell
git checkout -b pr-my-fix-branch main
```

### 4. Setup Tauri Environment

Follow [this guide](https://tauri.studio/en/docs/getting-started/intro/#setting-up-your-environment) to set up Tauri environment

### 4. Setup FE

1. Change directory to the root directory of this project
2. Install all dependencies using your favorite package manager (either `npm`, `pnpm` or `bun`, but never tried `yarn`)

```sh
$ npm install
```

3. Run the project in development mode. Please note that it might takes some time for Cargo to install dependencies for the first run.

```sh
$ npm tauri dev
```

## Proposing a change

If you would like to request a new feature or enhancement but are not yet thinking about opening a pull request, you can also [open a discussion](#feat) and others will code it!

If you intend to fix a bug, please discuss it through [Issues](#issues) before submitting a pull request.

If you intend to add a new feature, please discuss it through [GitHub Discussions](#feat) to avoid multiple people working on the same feature request.

## Sending a Pull Request

make sure the PR does only one thing, otherwise please split it. It is recommended to follow this [commit message style](#commit-msg).

1. Make changes and ensure your commit message is understandable.
2. Open a [PR](https://github.com/fmgono/CPM/pulls) and ensure to describe your pull request clearly.

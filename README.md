# LUCIFER 🐲
Free, Open source, Cross-platform, Illuminating CLI testing written in Rust.

## Stability

Lucifer is currently in alpha. While the contract will never break within the same version, there will be a few bugs. If you like the project and want to support its development, consider giving it a try and filing an issue or two when something breaks.

The project is expected to hit a full release by December 2022, and the your help in ironing out all of the bugs will be greatly appreciated.

## What is lucifer?

Lucifer is a CLI testing tool designed for running suites of tests. It is designed around the idea that your integration tests need to scale alongside your code base. 

[Cucumber](https://cucumber.io/) is a great example of a framework that separates the code that interprets and executes a test from the test itself. This idea is the foundation of lucifer. By isolating the code which exercises your CLI from the requirements you need to specify, the complexity of your test codebase can scale linearly with the tests that you run.

All lucifer tests are specified in plain YAML to maximise the readability and ease of writing. Developers, QA, Analysts, and anyone else who knows what a CLI is can read these tests easily.

This comes at the tradeoff of customizability. Lucifer is responsible for executing the data, which makes it harder to customize the tool should you have  a need to. If you find that you need to do something lucifer doesn't offer, there are a few options.

### 1. Create an issue (Or even a pull request!)

Remember, lucifer is still in alpha state. We may be missing features which would be a good idea to include in the final product. Use the [issue tab](https://github.com/winstonpuckett/lucifer/issues) in GitHub to create new feature requests.

### 2. Consider whether it should be an integration test.

Lucifer supports many different types of tests. If there's something specific that happens which lucifer doesn't cover, consider whether that should be a unit test.

[More information on the testing pyramid](https://www.browserstack.com/guide/testing-pyramid-for-test-automation)

### 3. Write your own tool

If worst comes to worst, you *can* write a simple bash script to wrap your tool and validate what needs to be validated. Just remember that performance metrics will envolve any custom verification steps.

It is always recommended to try the first to ideas first.

## Getting Started

### Install 

#### 1. Install with Cargo (recommended)

Prerequisites:
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

The easiest way to get up and running with lucifer is to install it from rust's package manager, [cargo](https://doc.rust-lang.org/cargo/).

To install lucifer, run the following command in your terminal of choice:
```bash
cargo install lucifer-testing
```

#### 2.

Prerequisites:
- [git](https://git-scm.com/)
- [rust/cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- A bash-like terminal (or modify the script below to match your terminal)

*note: copy/pasting bash scripts from the internet is a bad idea. Make sure you know what you're doing and why. It never hurts to type things in manually.*

You can do install from source with the following script:
```bash
# Get the source
git clone https://github.com/winstonpuckett/lucifer.git
cd lucifer

# Build the project
cargo build --release

# move the target file out of the directory
mv ./target/release/lucifer ..

# Remove the source folder
rm -rf lucifer
```

### Running a test

Once you have lucifer installed, it's time to start testing! Let's create a simple test.

1. Create a yaml file called "feature.yaml" or any meaningful name.
2. Paste this yaml into the file

```yaml
command: echo
tests:
  - name: echo replies
    description: |
      Given any string
      When echo is called
      Then that string should be returned
    args:
      - hello
    expectations:
      - output: hello
```

Run this file with the command:

```bash
lucifer -i ./feature.yaml
```

For more examples, visit the getting started page on our website... Which is not currently up.

## Where is it going

The current vision for the product is all about ease. This means LOTS of documentation and some infrastructure set up.

Including (but not limited to):
- Setting up the official website with documentation and lots of easy examples
- Improving this readme
- Installation through common package managers such as ~~cargo~~, apt-get, snap, winget, homebrew, and more
- A base docker image for when you want to deploy in CI/CD scenarios
- GitHub Actions support
- Squashing any found bugs
- Improving performance

## Alternatives

Lucifer may not be the right choice for you. That's ok. I hope you will consider supporting one of these other open source projects:

- [BATS](https://github.com/bats-core/bats-core)
- [cli-testing-library](https://github.com/gmrchk/cli-testing-library)

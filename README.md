[![Build / Test / Publish](https://github.com/winstonpuckett/lucifer/actions/workflows/deploy.yml/badge.svg?branch=main)](https://github.com/winstonpuckett/lucifer/actions/workflows/deploy.yml)

# LUCIFER 🐲
Free, Open source, Cross-platform, Illuminating CLI testing written in Rust.

## What is lucifer?

Lucifer is a CLI testing tool designed for running suites of tests. It is designed around the idea that your integration tests need to scale alongside your code base. 

[Cucumber](https://cucumber.io/) is a great example of a framework that separates the code that interprets and executes a test from the test itself. This idea is the foundation of lucifer. By isolating the code which exercises your CLI from the requirements you need to specify, the complexity of your test codebase can scale linearly with the tests that you run.

All lucifer tests are specified in plain YAML to maximise the readability and ease of writing. Developers, QA, Analysts, and anyone else who knows what a CLI is can read these tests easily.

This comes at the tradeoff of customizability. Lucifer is responsible for executing the data, which makes it harder to customize the tool should you have  a need to. If you find that you need to do something lucifer doesn't offer, there are a few options.

### 1. Create an issue (Or better yet, a pull request!)

Lucifer may be missing features. Use the [issue tab](https://github.com/winstonpuckett/lucifer/issues) in GitHub to create new feature requests and connect with me about what you're looking for.

### 2. Consider whether it should be an integration test.

Lucifer supports many different types of tests. If there's something specific that happens which lucifer doesn't cover, consider whether that should be a unit test.

[More information on the testing pyramid](https://www.browserstack.com/guide/testing-pyramid-for-test-automation)

### 3. Write your own tool

If worst comes to worst, you *can* write a simple bash script to wrap your tool and validate what needs to be validated. Just remember that performance metrics will envolve any custom verification steps.

## Why is it called Lucifer?

I was talking with a friend of mine who studies these things and he told me that the original translations of "the Satan" actually translate better as "the challenger." This alters the understanding of what happened with Job (in the book of Job in the old testament) - it's not that God and "Satan" were talking, it's that God was discussing something and someone spoke up, making them "the Satan" in that scenario.

Of course, I'm misrepresenting all of this by calling this project lucifer and not satan. Lucifer is the reference to the fallen angel as far as I am aware. But darn it, lucifer sounded better.

This project aims to challenge your assumptions of what your code looks like. And in doing so, become "the Satan" for you so that you don't have to.

## Getting Started

### Install 

Prerequisites:
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

lucifer is built and distributed using Rust. You'll need a copy of Rust's package manager, [cargo](https://doc.rust-lang.org/cargo/), installed on your local system.

To install lucifer, run the following command in your terminal of choice:
```bash
cargo install lucifer-testing
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

For more examples, visit the [test syntax reference](https://winstonpuckett.com/products/lucifer/tests).

## Alternatives

Lucifer may not be the right choice for you. That's ok. I hope you will consider supporting one of these other open source projects:

- [BATS](https://github.com/bats-core/bats-core)
- [cli-testing-library](https://github.com/gmrchk/cli-testing-library)

## Finally

If you try out lucifer, please give it a start on GitHub. I love knowing that someone thought this effort was valuable.
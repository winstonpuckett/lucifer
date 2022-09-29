# LUCIFER
Free, Open source, Illuminating CLI testing written in Rust.

## Stability

Lucifer is currently in alpha. While the contract will never break within the same version, there will be a few bugs. If you like the project and want to support its development, consider giving it a try and filing an issue or two when something breaks.

The project is expected to hit a full release by December 2022, and the your help in ironing out all of the bugs will be greatly appreciated.

## What is lucifer?

Lucifer is a CLI testing tool designed for running suites of tests. It is designed around the idea that your integration tests need to scale alongside your code base. 

[Cucumber](https://cucumber.io/) is a great example of a framework that separates the code that interprets and executes a test from the test itself. This idea is the foundation of lucifer. By isolating the code which exercises your CLI from the requirements you need to specify, the complexity of your test codebase can scale linearly with the tests that you run.

All lucifer tests are specified in plain YAML to maximise the readability and ease of writing. Developers, QA, Analysts, and anyone else who knows what a CLI is can read these tests easily.

This comes at the tradeoff of customizability. Lucifer is responsible for executing the data, which makes it harder to customize the tool should you have  a need to. If you find that you need to do something lucifer doesn't offer, there are a few options.

### 1. Create an issue (Or even a pull request!)

Remember, lucifer is still in alpha state. We may be missing features which would be a good idea to include in the final product. Use the [issue tab](https://github.com/winstonpuckett/lucifer/issues) to create new feature requests.

### 2. Consider whether it should be an integration test.

Lucifer supports many different types of tests. If there's something specific that happens which lucifer doesn't cover, consider whether that should be a unit test.

[More information on the testing pyramid](https://www.browserstack.com/guide/testing-pyramid-for-test-automation)

### 3. Write your own tool

If worst comes to worst, you *can* write a simple bash script to wrap your tool and validate what needs to be validated. Just remember that performance metrics will envolve any custom verification steps.

It is always recommended to try the first to ideas first.

## Getting Started



- Where is it going
- How is it different
- 

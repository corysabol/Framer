# Contributing to Framer

We're excited that you're interested in contributing to Framer! This document outlines the process for contributing to our project and provides some guidelines to ensure a smooth collaboration.

## Code of Conduct

DBAD - Don't Be A Dick

- Don't harrass people
- Don't violate peoples privacy
- Don't be mean
- Don't be a immature asshat

Otherwise you won't be able to contribute or participate in this project.

## How to Contribute

### Reporting Bugs

1. Ensure the bug hasn't already been reported by searching our [GitHub Issues](https://github.com/corysabol/framer/issues).
2. If you can't find an existing issue, [open a new one](https://github.com/corysabol/framer/issues/new). Include a title, clear description, and as much relevant information as possible.
3. If possible, use the bug report template to create the issue.

### Suggesting Enhancements

1. First, read the [documentation](https://framer.docs.example.com) and [open/closed issues](https://github.com/corysabol/framer/issues?q=is%3Aissue) to see if the enhancement has already been suggested or implemented.
2. If not, [open a new issue](https://github.com/corysabol/framer/issues/new). Provide a clear title and description of your suggestion, along with any relevant examples.

### Pull Requests

1. Fork the repository and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code lints.
6. Issue the pull request!

## Development Setup

1. Fork and clone the repository
2. Install dependencies:
   ```
   cargo build
   ```
3. Set up Godot project:
   - Open Godot
   - Import the project from the `godot/` directory

## Coding Style

- For Rust code, follow the [Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md).
- For GDScript, follow the [GDScript style guide](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_styleguide.html).
- Use descriptive variable names and add comments for complex logic.

## Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

## License

By contributing to Framer, you agree that your contributions will be licensed under its MIT License.

## Questions?

If you have any questions, please feel free to contact the project maintainers or open an issue for discussion.

Thank you for contributing to Framer!

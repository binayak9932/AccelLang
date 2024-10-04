# AccelLang Documentation

## Table of Contents
1. [Introduction](#introduction)
2. [Features](#features)
3. [Installation](#installation)
   - [Prerequisites](#prerequisites)
   - [Step-by-Step Guide](#step-by-step-guide)
4. [Usage](#usage)
   - [Basic Syntax](#basic-syntax)
   - [Running Your Code](#running-your-code)
   - [Example](#example)
5. [Language Features](#language-features)
6. [Contributing](#contributing)
7. [License](#license)

## Introduction

**YourLanguage** is a high-performance programming language designed to take advantage of hardware acceleration using IPUs and GPUs. It supports fast parallelization, making it suitable for applications that require efficient computation and data processing.

## Features

- **Hardware Acceleration**: Leverage IPUs and GPUs for high-speed processing.
- **Parallelization**: Built-in support for concurrent execution of tasks.
- **Simple Syntax**: Easy-to-read syntax that minimizes boilerplate code.
- **Rich Standard Library**: A comprehensive library for common tasks.

## Installation

### Prerequisites

Before installing YourLanguage, ensure you have the following:

- [LLVM](https://llvm.org/) (version 12 or later)
- Rust (stable version)
- Git
- A C++ compiler (GCC or Clang)

### Step-by-Step Guide

1. **Install Rust**:
   - Visit [rustup.rs](https://rustup.rs/)
   - Follow the instructions for your operating system
   - Verify installation:
     ```bash
     rustc --version
     cargo --version
     ```

2. **Install LLVM**:
   - For Ubuntu/Debian:
     ```bash
     sudo apt-get update
     sudo apt-get install llvm-12 llvm-12-dev
     ```
   - For macOS (using Homebrew):
     ```bash
     brew install llvm@12
     ```
   - For Windows:
     - Download the LLVM installer from [LLVM Releases](https://releases.llvm.org/)
     - Run the installer and add LLVM to your system PATH

3. **Clone the Repository**:
   ```bash
   git clone https://github.com/binayak9932/AccelLang.git
   cd AccelLang
   ```

4. **Build the Project**:
   ```bash
   cargo build --release
   ```

5. **Set up Environment Variables**:
   - Add the following to your `.bashrc`, `.zshrc`, or equivalent:
     ```bash
     export PATH=$PATH:/path/to/AccelLang/target/release
     ```
   - Reload your shell or run `source ~/.bashrc`

6. **Verify Installation**:
   ```bash
   AccelLang --version
   ```

## Usage

### Basic Syntax

Here's a simple example of a function in YourLanguage:

```
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

### Running Your Code

To run a program written in YourLanguage, use the command-line interface:

```bash
yourlanguage run <path_to_your_file.yourlang>
```

### Example

1. Create a file named `example.yourlang` with the following content:

   ```
   fn add(a: i32, b: i32) -> i32 {
       return a + b;
   }

   fn main() {
       let result = add(5, 10);
       print(result);
   }
   ```

2. Execute the file:

   ```bash
   yourlanguage run example.yourlang
   ```

## Language Features

- **Data Types**: Support for basic types like `i32`, `float`, `string`, and user-defined types.
- **Control Structures**: Includes if statements, loops, and switch cases.
- **Functions**: First-class functions with support for closures.
- **Error Handling**: Built-in mechanisms for handling exceptions and errors.
- **Concurrency**: Native support for parallel processing and multi-threading.
- **Memory Management**: Automatic memory management with optional manual control.

## Contributing

We welcome contributions to YourLanguage! Here's how you can help:

1. **Fork the Repository**:
   
   - Click the "Fork" button in the top-right corner

2. **Clone Your Fork**:
   ```bash
   git clone https://github.com/binayak9932/AccelLang.git
   cd AccelLang
   ```

3. **Create a New Branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make Your Changes**:
   - Implement your feature or bug fix
   - Write or update tests as necessary
   - Update documentation to reflect your changes

5. **Commit Your Changes**:
   ```bash
   git add .
   git commit -m "Add a descriptive commit message"
   ```

6. **Push to Your Fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Submit a Pull Request**:
   - Go to your fork on GitHub
   - Click "New pull request"
   - Select your feature branch and submit

8. **Code Review**:
   - Wait for maintainers to review your PR
   - Make any requested changes and push them to your fork

9. **Merge**:
   - Once approved, a maintainer will merge your PR

### Coding Standards
- Follow the existing code style in the project
- Write clear, commented, and testable code
- Ensure all tests pass before submitting a PR
- Update documentation for any new features or changes

## License

YourLanguage is released under the MIT License. See the [LICENSE](LICENSE) file for details.

---

**Note**: Replace "YourLanguage" and "yourlanguage" with the actual name of your programming language throughout this document. Update URLs, file extensions, and any specific details to match your project's actual implementation.

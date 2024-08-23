# Plutonium Key Generator

[![Rust](https://img.shields.io/badge/rust-1.70.0-orange?logo=rust)](https://www.rust-lang.org/)
[![Windows](https://img.shields.io/badge/platform-Windows-blue?logo=windows)](https://www.microsoft.com/windows)
[![Linux](https://img.shields.io/badge/platform-Linux-yellow?logo=linux)](https://www.linux.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?logo=opensource)](https://opensource.org/licenses/MIT)
[![GitHub repo size](https://img.shields.io/github/repo-size/sterbweise/plutonium-key-generator?logo=github)](https://github.com/sterbweise/plutonium-key-generator)
[![GitHub last commit](https://img.shields.io/github/last-commit/sterbweise/plutonium-key-generator?logo=github)](https://github.com/sterbweise/plutonium-key-generator/commits/main)
[![Rust CI](https://github.com/Sterbweise/plutonium-key-generator/actions/workflows/rust.yml/badge.svg)](https://github.com/Sterbweise/plutonium-key-generator/actions/workflows/rust.yml)

<img src="https://imgur.com/bBrx8Hf.png" alt="Plutonium Logo" width="350"/>

## Description
A command-line tool for generating Plutonium server keys without owning the games. **<span style="color: orange;">This software bypasses Plutonium's requirement of game ownership to create a server key</span>**, providing an easy-to-use interface for supported Call of Duty titles on the Plutonium platform.

## Features

- Interactive command-line interface
- Support for multiple Call of Duty titles
- Integration with an intermediate API for key generation
- Support for both Zombie and Multiplayer modes (where applicable)
- User-friendly prompts and error handling

## Supported Games

- Call of Duty: World at War <span style="color: red;">(Not currently supported by the API)</span>
- Call of Duty: Black Ops <span style="color: red;">(Not currently supported by the API)</span>
- Call of Duty: Black Ops II <span style="color: green;">(Supported)</span>
- Call of Duty: Modern Warfare 3 <span style="color: red;">(Not currently supported by the API)</span>

The generated keys are temporary, lasting 48 hours, making this tool ideal for testing and temporary server setups. Please note that this tool should not be used if you own the game, as official methods for key generation are available through the Plutonium platform.

## Details
- **Version:** 1.0.0
- **Author:** Sterbweise
- **Repository:** https://github.com/sterbweise/plutonium-key-generator
- **License:** MIT

## Warning
<span style="color: red;"><strong>Please don't use this tool if you own the game. Get the key by the classic method on https://platform.plutonium.pw/serverkeys</strong></span>


## Installation

To install the Plutonium Key Generator:

1. Go to the [Releases](https://github.com/sterbweise/plutonium-key-generator/releases) page of the GitHub repository.
2. Find the latest release version.
3. Download the appropriate version for your operating system:
   - For Windows: Download the file ending with `-windows.exe`
   - For Linux: Download the file ending with `-linux`
4. Once downloaded:
   - Windows users: Double-click the `.exe` file to run it.
   - Linux users: 
     a. Open a terminal in the directory where you downloaded the file.
     b. Make the file executable with the command: `chmod +x plutonium-key-generator-linux`
     c. Run the file with: `./plutonium-key-generator-linux`

Note: Ensure you have the necessary permissions to execute the file on your system. On some systems, you may need to right-click the file and select "Run as administrator" (Windows) or use `sudo` (Linux) if you encounter permission issues.

If you encounter any issues during installation or execution, please check the [Issues](https://github.com/sterbweise/plutonium-key-generator/issues) page on GitHub or create a new issue for assistance.

## API Integration

This tool integrates with an intermediate API to generate server keys. The API endpoint used is:
`https://api.sterbweise.dev/plutonium-key-generator`

The API expects a JSON payload with the following structure:

```json
{
"server_name": "Your Server Name",
"mode": "Game Mode Code"
}
```


## Development Guide

This section is intended for developers who want to work on the project in development mode.

### Dependencies
The project relies on the following main dependencies:
- `reqwest` (version 0.11, with `json` and `blocking` features enabled): Used for making HTTP requests to the API.
- `serde` (version 1.0, with `derive` feature enabled): Used for serializing and deserializing JSON data.

For a complete list of dependencies, please refer to the `Cargo.toml` file.

### Setting Up the Development Environment
1. Ensure you have Rust and Cargo installed on your system. If not, follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).
2. Clone the repository:
   ```bash
   git clone https://github.com/sterbweise/plutonium-key-generator.git
   ```

3. Navigate to the project directory:
   ```bash
   cd plutonium-key-generator
   ```

4. Install the dependencies:
   ```bash
   cargo fetch
   ```

### Building for Development
To build the project in development mode, use:
```bash
cargo build
```


This command compiles the project and its dependencies, creating an unoptimized binary with debug information. The resulting executable will be placed in the `target/debug` directory.

For a more optimized build (useful for testing performance), you can use:
```bash
cargo build --release
```

This command compiles the project with optimizations and removes debug information, resulting in a smaller and faster executable. The optimized binary will be placed in the `target/release` directory.

### Running the Project

To run the project in development mode:
```bash
cargo run
```


This command builds (if necessary) and runs the project. You can also run the executable directly:

- For debug build: `./target/debug/plutonium-key-generator`
- For release build: `./target/release/plutonium-key-generator`


## Contributing

Contributions to the Plutonium Key Generator are welcome and encouraged! We appreciate your interest in improving this project. Here's how you can contribute:

1. Fork the repository to your own GitHub account.
2. Clone the project to your machine.
3. Create a new branch locally with a succinct but descriptive name.
4. Make your changes, adhering to the project's coding style.
5. Commit changes to the branch.
6. Push changes to your fork.
7. Open a Pull Request in our repository.

When contributing, please ensure that your code adheres to the existing style of the project and includes appropriate tests.

### Areas for Contribution

- Bug fixes
- Performance improvements
- New features
- Documentation improvements
- Code refactoring

### API Contributions

We also welcome contributions to the API that powers this key generator. If you're interested in improving or extending the API functionality, please visit our API repository:

[https://github.com/Sterbweise/plutonium-key-generator-api](https://github.com/Sterbweise/plutonium-key-generator-api)

By contributing to the API, you can help enhance the core functionality of the key generator and potentially add new features or improve existing ones.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Disclaimer

**Warning:** This tool is intended for educational and development purposes only. Please do not use this tool if you own the game. Obtain the key through the official method at [https://platform.plutonium.pw/serverkeys](https://platform.plutonium.pw/serverkeys).

<p style="color: red; font-weight: bold;">The generated keys are valid for only 48 hours. Use of this tool may violate the terms of service of the game or platform. Use at your own risk.</p>

## Support

For support, please contact:

- Email: [contact@sterbweise.dev](mailto:contact@sterbweise.dev)
- Telegram: [@SG991](https://t.me/SG991)

You can also open an issue on this repository for bug reports or feature requests.

---

Developed with ❤️ by [Sterbweise](https://github.com/Sterbweise)


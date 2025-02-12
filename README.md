# TokenBlitz

A Python module built with Rust that provides a simple "Hello, World!" functionality.

## Installation

You can install the package using maturin:

```bash
pip install maturin
maturin develop
```

## Usage

```python
import hello_rust

# Basic usage - returns "Hello, World!"
print(hello_rust.say_hello())

# With a name parameter - returns "Hello, {name}!"
print(hello_rust.say_hello("Rust"))
```

## Development

This project uses:
- Rust with PyO3 for Python bindings
- Maturin for building and packaging

## License

MIT
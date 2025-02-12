# TokenBlitz

Fast tokenization built in Rust

## Installation

You can install the package using maturin:

```bash
pip install maturin
maturin develop
```

## Usage

```python
import tokenblitz

# Basic usage - returns "Hello, World!"
print(tokenblitz.tokenize("Hello, World!"))
```

## Development

This project uses:
- Rust with PyO3 for Python bindings
- Maturin for building and packaging

## License

MIT
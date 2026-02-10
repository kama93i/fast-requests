# fast-requests ⚡

High-performance Python HTTP client powered by Rust and Tokio for concurrent request execution.

## Why?

Python's `requests` library processes HTTP requests sequentially. `fast-requests` uses Rust's Tokio runtime to execute all requests concurrently, achieving significant performance improvements for bulk operations.

## Features

- ⚡ Concurrent request execution via Tokio
- 🔄 Connection pooling for reduced overhead
- 💪 Continues execution even if individual requests fail
- ⏱️ Built-in 30-second timeout per request
- 🎯 Simple, Pythonic API

## Installation
```bash
pip install maturin
maturin develop
```

## Usage
```python
import fast_requests

urls = [
    "https://api.github.com/users/octocat",
    "https://api.github.com/users/torvalds",
    "https://httpbin.org/status/404",
]

results = fast_requests.get_many(urls)

for result in results:
    if hasattr(result, 'text'):
        print(f"✓ {result.status_code} - {result.url}")
    else:
        print(f"✗ Error {result.status_code}: {result.error}")
```

## How It Works

- **Rust core** uses `reqwest` + `tokio` for async HTTP
- **PyO3** bridges Rust to Python
- All requests spawn concurrently via `tokio::spawn`
- Results returned in original order

## Built With

- Rust
- Tokio (async runtime)
- reqwest (HTTP client)
- PyO3 (Python bindings)
- maturin (build tool)

## License

MIT

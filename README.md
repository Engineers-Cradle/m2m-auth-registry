## M2M Service Auth Registry

### Introduction

This is a service that provides authentication between services. It is built using Rust and Redis.

### 🦄 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### 🛠️ Configuration

The system can be configured using the following environment variables:

```
REGISTRATION_TOKEN=
REDIS_URL=
WEB_SERVER_PORT=
NUM_WORKERS=
LOG_LEVEL=
```

### 🚀 Usage

```bash
$ cargo run
```

### 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
## M2M Service Auth Registry

![forthebadge](https://forthebadge.com/images/badges/open-source.svg)
![forthebadge](https://img.shields.io/github/languages/top/Engineers-Cradle/m2m-auth-registry?logo=rust&style=for-the-badge)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Engineers-Cradle/m2m-auth-registry/build-code.yaml?logo=rust&style=for-the-badge)

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
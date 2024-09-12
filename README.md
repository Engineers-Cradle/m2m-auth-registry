## M2M Service Auth Registry

![forthebadge](https://forthebadge.com/images/badges/open-source.svg)
![forthebadge](https://img.shields.io/github/languages/top/Engineers-Cradle/m2m-auth-registry?logo=rust&style=for-the-badge)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Engineers-Cradle/m2m-auth-registry/build-code.yaml?logo=rust&style=for-the-badge)

### Introduction

This is a service that provides authentication between services. It is a simple service that allows services to register and authenticate with each other. The service is designed to be fault-tolerant and scalable.

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

### 🎁 Crates

| Name | Description | Visit |
|------|-------------|-------|
| http | REST API Server for adding and listing nodes | [Open](./crates/http/) |
| attendance | Redis Pub/Sub Server which takes attendance of all connected nodes | [Open](./crates/attendance/) |
| pinger | Redis Pub/Sub Server which help to ping all connected nodes | [Open](./crates/pinger/) |

### 🚀 Usage

```bash
$ cargo run
```

### 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
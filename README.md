<div align="center">
  <img src="https://www.thedarkartist.in/favicon.ico" width="80" />
  <h1>RustRecon</h1>
  <p><strong>Async network port scanner with JSON report generation.</strong></p>
  <p>
    <img src="https://img.shields.io/github/languages/top/TheDarkArtist/RustRecon?color=0080ff&style=flat-square" alt="Language" />
    <img src="https://img.shields.io/github/license/TheDarkArtist/RustRecon?color=0080ff&style=flat-square" alt="License" />
    <img src="https://img.shields.io/github/stars/TheDarkArtist/RustRecon?color=0080ff&style=flat-square" alt="Stars" />
  </p>
</div>

<br />

## Why RustRecon?

Quick port scanning with structured output. Configure target IP and ports in a TOML file, run the scanner, get a JSON report with open ports, detected services, and known CVEs. Built async with Tokio for fast concurrent scanning.

---

## Table of Contents

- [Install](#install)
- [Quick Start](#quick-start)
- [Features](#features)
- [Configuration](#configuration)
- [Safety](#safety)
- [How It Works](#how-it-works)
- [Tech Stack](#tech-stack)
- [Contributing](#contributing)
- [License](#license)

---

## Install

```bash
git clone https://github.com/TheDarkArtist/RustRecon.git
cd RustRecon
cargo build --release
```

## Quick Start

```bash
# Edit config.toml with your target
cat config.toml
# ip = "192.168.1.1"
# ports = [22, 80, 443, 8080]

# Run the scan
cargo run --release

# Results saved to report.json
cat report.json
```

## Features

<table>
<tr>
<td width="50%">

**Async Scanning**

Concurrent port scanning powered by Tokio. All ports are scanned simultaneously with a 1-second timeout per port.

</td>
<td width="50%">

**JSON Reports**

Structured output with open ports, detected services, and associated CVEs. Machine-readable for integration with other tools.

</td>
</tr>
<tr>
<td width="50%">

**TOML Configuration**

Define target IP and port list in a simple `config.toml`. No command-line argument juggling.

</td>
<td width="50%">

**Service Detection**

Identifies common services (SSH, HTTP, HTTPS) by port number and maps them to known vulnerabilities.

</td>
</tr>
</table>

## Configuration

Create a `config.toml` in the project root:

```toml
ip = "192.168.1.1"
ports = [22, 80, 443, 3306, 5432, 8080, 8443]
```

### Sample Report Output

```json
[
  {
    "port": 22,
    "service": "SSH",
    "vulnerabilities": ["CVE-2020-14001", "CVE-2018-15473"]
  },
  {
    "port": 80,
    "service": "HTTP",
    "vulnerabilities": ["CVE-2019-12345", "CVE-2017-5638"]
  }
]
```

## Safety

| Mechanism | Description |
|---|---|
| Authorized targets only | Only scan networks and hosts you own or have explicit written permission to test |
| Read-only scanning | TCP connect scan only — no exploitation, no payload delivery |
| Local reports | Results saved to a local `report.json`, never transmitted externally |
| Configurable scope | Targets defined in `config.toml` — no accidental wide scans |

## How It Works

RustRecon reads target IP and ports from `config.toml`, spawns an async Tokio task per port that attempts a TCP connection with a 1-second timeout. Open ports are collected via an `mpsc` channel, matched to known services by port number, and written to a JSON report with associated CVE references.

## Tech Stack

<p>
  <img src="https://img.shields.io/badge/Rust-2021_Edition-000?style=flat-square&logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/Tokio-Async_Runtime-000?style=flat-square" />
  <img src="https://img.shields.io/badge/Serde-Serialization-000?style=flat-square" />
</p>

## Contributing

```bash
git clone https://github.com/TheDarkArtist/RustRecon.git
cd RustRecon
cargo build
```

<details>
<summary><strong>Contributing Guidelines</strong></summary>

1. Fork the repo
2. Create a branch (`git checkout -b feature/my-thing`)
3. Make your changes
4. Test against your own network
5. Open a PR

</details>

<details>
<summary><strong>Contributor Graph</strong></summary>
<p>
  <a href="https://github.com/TheDarkArtist/RustRecon/graphs/contributors">
    <img src="https://contrib.rocks/image?repo=TheDarkArtist/RustRecon" />
  </a>
</p>
</details>

## License

[MIT](LICENSE)

<br />

<div align="center">
  <sub>Built by <a href="https://thedarkartist.in">TheDarkArtist</a></sub>
</div>

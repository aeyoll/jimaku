# Jimaku

[![GitHub Actions workflow status](https://github.com/aeyoll/jimaku/workflows/ci/badge.svg)](https://github.com/aeyoll/jimaku/actions)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.57.0+-lightgray.svg)](#rust-version-requirements)
[![Conventional commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)

A subtitle downloader written in Rust

Providers
---

_jimaku_ can search and download subtitles from the following providers:

- [betaseries](https://www.betaseries.com/)
- [opensubtitles](https://www.opensubtitles.org/)

Setup
---

To be able to use _jimaku_, you need to have api keys for each provider.

```shell
export BETA_SERIES_API_KEY="..."
export OPEN_SUBTITLES_API_KEY="..."
```

Install
---

With cargo:

```shell
cargo install jimaku
```

Or use the install-script and add `$HOME/.jimaku/bin` to your `$PATH`.

````shell
curl -fsSL https://raw.githubusercontent.com/aeyoll/jimaku/main/install.sh | bash
````

Usage
---

```shell
jimaku ~/file/path/file.mp4 --language=fr
```

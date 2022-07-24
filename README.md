# Jimaku

A subtitle downloader written in Rust

Providers
---

Sub can search and download subtitles from the following providers:

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

```shell
cargo install jimaku
```

Usage
---

```shell
jimaku ~/file/path/file.mp4 --language=fr
```

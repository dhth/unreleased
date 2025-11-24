<p align="center">
  <h1 align="center">unreleased</h1>
  <p align="center">
    <a href="https://github.com/dhth/unreleased/actions/workflows/main.yml"><img alt="GitHub release" src="https://img.shields.io/github/actions/workflow/status/dhth/unreleased/main.yml?style=flat-square"></a>
    <a href="https://crates.io/crates/unreleased"><img alt="GitHub release" src="https://img.shields.io/crates/v/unreleased?style=flat-square"></a>
    <a href="https://github.com/dhth/unreleased/releases/latest"><img alt="Latest release" src="https://img.shields.io/github/release/dhth/unreleased.svg?style=flat-square"></a>
    <a href="https://github.com/dhth/unreleased/releases"><img alt="Commits since latest release" src="https://img.shields.io/github/commits-since/dhth/unreleased/latest?style=flat-square"></a>
  </p>
</p>

`unreleased` lets you view the commits to your GitHub repos since their last
release.

![html-report](https://tools.dhruvs.space/images/unreleased/v0-1-0/html-report.png)

> Read more about how I leverage unreleased for my needs [here][2].

💾 Installation
---

**homebrew**:

```sh
brew install dhth/tap/unreleased
```

**cargo**:

```sh
cargo install unreleased
```

Or get the binaries directly from a Github [release][3]. Read more about
verifying the authenticity of released artifacts
[here](#-verifying-release-artifacts).

⚡️ Usage
---

`unreleased` requires a TOML config file which looks like the following.

```toml
# array of repos to run for
[[repos]]
# repository name in the format "owner/repo"
repo = "dhth/bmm"

[[repos]]
repo = "dhth/hours"
# head ref to use when generating commit log
# optional
# default: main
head_ref = "some-branch"

[[repos]]
repo = "dhth/unreleased"
# whether to consider a pre-release as the last release
# optional
# default: false
consider_prereleases = true
```

```text
$ unreleased report -h

Usage: unreleased report [OPTIONS]

Options:
  -c, --config-path <PATH>      Path to the unreleased's file (defaults to <YOUR_CONFIG_DIR>/unreleased/unreleased.toml)
      --debug                   Output debug information without doing anything
  -f, --filter <REGEX>          Regex to use for filtering repos
  -o, --output-format <FORMAT>  Output format [default: stdout] [possible values: stdout, html]
      --stdout-plain            Whether to output text to stdout without color
      --html-output <PATH>      Path for the HTML output file [default: unreleased.html]
      --html-title <STRING>     Title for HTML report [default: unreleased]
      --html-template <PATH>    Path to custom HTML template file
  -h, --help                    Print help
```

### stdout report

By default, `unreleased` prints its report to stdout.

[![stdout output](https://asciinema.org/a/cOYp8baDZUEbmKkk3WBMhP2xo.svg)](https://asciinema.org/a/cOYp8baDZUEbmKkk3WBMhP2xo)

### HTML report

`unreleased` can also generate an HTML version of its report. Mine is deployed
[here][1].

🔐 Verifying release artifacts
---

In case you get the `unreleased` binary directly from a [release][1], you may want
to verify its authenticity. Checksums are applied to all released artifacts, and
the resulting checksum file is attested using [Github Attestations][2].

Steps to verify (replace `A.B.C` in the commands below with the version you
want):

1. Download the sha256 checksum file for your platform from the release:

   ```shell
   curl -sSLO https://github.com/dhth/unreleased/releases/download/vA.B.C/unreleased-x86_64-unknown-linux-gnu.tar.xz.sha256
   ```

2. Verify the integrity of the checksum file using [gh][3].

   ```shell
   gh attestation verify unreleased-x86_64-unknown-linux-gnu.tar.xz.sha256 --repo dhth/unreleased
   ```

3. Download the compressed archive you want, and validate its checksum:

   ```shell
   curl -sSLO https://github.com/dhth/unreleased/releases/download/vA.B.C/unreleased-x86_64-unknown-linux-gnu.tar.xz
   sha256sum --ignore-missing -c unreleased-x86_64-unknown-linux-gnu.tar.xz.sha256
   ```

3. If checksum validation goes through, uncompress the archive:

   ```shell
   tar -xzf unreleased-x86_64-unknown-linux-gnu.tar.xz
   cd unreleased-x86_64-unknown-linux-gnu
   ./unreleased -h
   # profit!
   ```

[1]: https://unreleased.gh.dhruvs.space
[2]: https://devlog.dhruvs.space/log/009
[3]: https://github.com/dhth/unreleased/releases
[4]: http://devlog.dhruvs.space/log/009/

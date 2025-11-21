<p align="center">
  <h1 align="center">unreleased</h1>
  <p align="center">
    <a href="https://github.com/dhth/unreleased/actions/workflows/main.yml"><img alt="GitHub release" src="https://img.shields.io/github/actions/workflow/status/dhth/unreleased/main.yml?style=flat-square"></a>
  </p>
</p>

View the commits to your GitHub repos since their last release.

> [!NOTE]
> unreleased is alpha software. Its interface and behaviour might change in the near
> future.

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

`unreleased` can also generate an HTML version of its report.

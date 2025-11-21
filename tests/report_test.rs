mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn shows_help() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["report", "--help"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Show unreleased commits for repos

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

    ----- stderr -----
    ");
}

#[test]
fn debug_flag_works_for_defaults() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["report", "--debug"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO

    command:                report
    config file path:       <NOT PROVIDED>
    repo filter:            <NOT PROVIDED>
    output format:          stdout
    plain output:           false


    ----- stderr -----
    ");
}

#[test]
fn debug_flag_works_with_overridden_flags_for_stdout_output() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "report",
        "--config-path",
        "tests/assets/valid-config.toml",
        "--debug",
        "--filter",
        "repo-(a|b)",
        "--output-format",
        "stdout",
        "--stdout-plain",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO

    command:                report
    config file path:       tests/assets/valid-config.toml
    repo filter:            repo-(a|b)
    output format:          stdout
    plain output:           true


    ----- stderr -----
    ");
}

#[test]
fn debug_flag_works_with_overridden_flags_for_html_output() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "report",
        "--config-path",
        "tests/assets/valid-config.toml",
        "--debug",
        "--filter",
        "repo-(a|b)",
        "--output-format",
        "html",
        "--html-output",
        "output.html",
        "--html-template",
        "tests/assets/template.html",
        "--html-title",
        "unreleased code",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    DEBUG INFO

    command:                report
    config file path:       tests/assets/valid-config.toml
    repo filter:            repo-(a|b)
    output format:          html
    output path:            output.html
    title:                  unreleased code
    template path:          tests/assets/template.html


    ----- stderr -----
    ");
}

//-------------//
//  FAILURES   //
//-------------//

#[test]
fn fails_if_provided_with_absent_config_file() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["report", "--config-path", "tests/assets/absent.toml"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't get config from file "tests/assets/absent.toml"

    Caused by:
        0: couldn't read file
        1: No such file or directory (os error 2)
    "#);
}

#[test]
fn fails_if_provided_with_config_with_invalid_toml() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["report", "--config-path", "tests/assets/invalid-toml.toml"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't get config from file "tests/assets/invalid-toml.toml"

    Caused by:
        0: couldn't deserialize TOML
        1: TOML parse error at line 5, column 7
             |
           5 | repo  "owner/repo-b"
             |       ^
           key with no value, expected `=`
    "#);
}

#[test]
fn fails_if_provided_with_config_with_invalid_schema() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "report",
        "--config-path",
        "tests/assets/invalid-schema.toml",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't get config from file "tests/assets/invalid-schema.toml"

    Caused by:
        0: couldn't deserialize TOML
        1: TOML parse error at line 6, column 24
             |
           6 | consider_prereleases = "yes"
             |                        ^^^^^
           invalid type: string "yes", expected a boolean
    "#);
}

#[test]
fn fails_if_provided_with_config_with_invalid_data() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["report", "--config-path", "tests/assets/invalid-data.toml"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't get config from file "tests/assets/invalid-data.toml"

    Caused by:
        config has errors:
         - repo #1 has errors:
           - repo needs to be in the format "owner/repo"
         - repo #2 has errors:
           - repo name is empty
         - repo #3 has errors:
           - owner is empty
         - repo #4 has errors:
           - head_ref is empty
    "#);
}

#[test]
fn fails_if_provided_invalid_regex() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "report",
        "--config-path",
        "tests/assets/valid-config.toml",
        "--filter",
        "(invalid|",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: invalid regex pattern provided

    Caused by:
        regex parse error:
            (invalid|
            ^
        error: unclosed group
    ");
}

#[test]
fn fails_if_no_repos_match_filter() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "report",
        "--config-path",
        "tests/assets/valid-config.toml",
        "--filter",
        "will-not-match",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: no repos match the provided filter
    ");
}

#[test]
fn fails_if_provided_with_absent_html_template_file() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "report",
        "--config-path",
        "tests/assets/valid-config.toml",
        "--html-template",
        "tests/assets/absent.html",
        "--output-format",
        "html",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: failed to read HTML template from "tests/assets/absent.html"

    Caused by:
        No such file or directory (os error 2)
    "#);
}

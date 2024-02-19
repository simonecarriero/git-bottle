# git-bottle
An interactive CLI tool that streamlines the usage of `Co-authored-by` and other [commit trailers](https://git-scm.com/docs/git-interpret-trailers)

## Usage

```
git bottle
```

<p align="center"><img src="/img/demo.gif?raw=true"/></p>

## Installation

### From binaries
TODO

### From cargo
```
cargo install --git https://github.com/simonecarriero/git-bottle
```

## Custom configuration
Customize the behavior providing a `.gitbottle.yml` configuration file in your
repository or in any ancestor folder, using the following schema:

### Top-level keys:
- `trailers` (Array of Trailer): array containing different types of commit trailers

### Trailer
`Trailer` can be `TextTrailer`, `SelectTrailer` or `MultiSelectTrailer`

#### TextTrailer
* keys:
  * `type`: `text`
  * `name` (String): the name of the trailer

#### SelectTrailer
* keys:
  * `type`: `select`
  * `name` (String): the name of the trailer
  * `values` (Values): options for selection

#### MultiSelectTrailer
* keys:
  * `type`: `multi_select`
  * `name` (String): the name of the trailer
  * `values` (Values): options for selection

### Values
`Values` can be `ValuesFromOptions` or `ValuesFromGitLog`

#### ValuesFromOptions
* keys
  * `type`: `from_options`
  * `options` (Array of String): options for selection

#### ValuesFromGitLog
* keys
  * `type`: `from_git_log`
  * `max_count` (Optional Integer): limit the number of commits in git log
  * `format_strings` (Array of String): array of format-strings for extracting values
  from the git log with pretty format (`git log --format=<format-string>`)

### Example of `.gitbottle.yml`:

```
trailers:
  - name: Issue
    type: select
    values:
      type: from_git_log
      max_count: 10
      format_strings:
        - "%(trailers:key=Issue,valueonly=true)"
  - name: Co-authored-by
    type: multi_select
    values:
      type: from_git_log
      format_strings:
        - "%an <%ae>"
        - "%(trailers:key=Co-authored-by,valueonly=true)"
```

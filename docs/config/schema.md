# Configuration file schema

## Top-level keys:
- `trailers` (Array of Trailer): array containing different types of commit trailers

## Trailer
`Trailer` can be `TextTrailer`, `SelectTrailer` or `MultiSelectTrailer`

### TextTrailer
* keys:
    * `type`: `text`
    * `name` (String): the name of the trailer

### SelectTrailer
* keys:
    * `type`: `select`
    * `name` (String): the name of the trailer
    * `values` (Values): options for selection

### MultiSelectTrailer
* keys:
    * `type`: `multi_select`
    * `name` (String): the name of the trailer
    * `values` (Values): options for selection

## Values
`Values` can be `ValuesFromOptions` or `ValuesFromGitLog`

### ValuesFromOptions
* keys
    * `type`: `from_options`
    * `options` (Array of String): options for selection

### ValuesFromGitLog
* keys
    * `type`: `from_git_log`
    * `max_count` (Optional Integer): limit the number of commits in git log
    * `format_strings` (Array of String): array of format-strings for extracting values
      from the git log with pretty format (`git log --format=<format-string>`)

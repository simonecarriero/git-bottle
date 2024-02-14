# git-bottle
A cli interactive tool to improve the DevEx of `Co-authored-by` and other git trailers

## Installation

### From binaries
TODO

### From cargo
```
cargo install --git https://github.com/simonecarriero/git-bottle
```

## Custom config
TODO

Example of `.gitbottle.yml`:
```
trailers:
  - name: Issue
    type: select
    values:
      type: from_git_log
      depth: 10
      format_strings:
        - "%(trailers:key=Issue,valueonly=true)"
  - name: Co-authored-by
    type: multiselect
    values:
      type: from_git_log
      format_strings:
        - "%an <%ae>"
        - "%(trailers:key=Co-authored-by,valueonly=true)"
```

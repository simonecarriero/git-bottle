# Multi-selection of `Co-authored-by` from an explicit list of values and 
selection of `Issue` from the last 10 commits in the git log](docs/config/example_3.md)

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
      type: from_options
      options:
      - James Smith <james.smith@example.org>
      - Jane Doe <jane.doe@example.org>
      - Joe Shmoe <joe.shmoe@example.org>
```

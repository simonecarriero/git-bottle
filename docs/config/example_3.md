# Selection of `Issue` from the last 10 commits in the git log

```
trailers:
  - name: Issue
    type: select
    values:
      type: from_git_log
      max_count: 10
      format_strings:
        - "%(trailers:key=Issue,valueonly=true)"
```

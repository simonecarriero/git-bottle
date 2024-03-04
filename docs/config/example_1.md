# Multi-selection of `Co-authored-by` from the git log (default behavior)

```
trailers:
  - name: Co-authored-by
    type: multi_select
    values:
      type: from_git_log
      format_strings:
        - "%an <%ae>"
        - "%(trailers:key=Co-authored-by,valueonly=true)"
```

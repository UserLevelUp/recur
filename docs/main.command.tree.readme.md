# main.command.tree.readme

Command overview for `tree`.

`tree` accepts either a literal hierarchy base or a full wildcard pattern. When
the query has a literal prefix before its first wildcard, that prefix anchors
the rendered tree. For example, this renders every `current` state under
`main`, including a leaf that ends at `current`:

```powershell
recur tree "main.**.current.**" -d docs/
```

Use `**` as a complete hierarchy segment for zero or more segments. `****` is
only a wildcard within one segment and is not a recursive hierarchy operator.

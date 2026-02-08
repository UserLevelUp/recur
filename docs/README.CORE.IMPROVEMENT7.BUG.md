# Bug Report: ANSI Color Codes Breaking Piped Output

**Status:** ✅ FIXED
**Severity:** High (breaks core functionality)
**Affected:** All commands with color output when piped
**Fixed in:** [src/output.rs](../src/output.rs)

---

## Summary

Recur was emitting ANSI color escape codes even when stdout was redirected to a pipe or file, breaking the `--stdin` functionality when piping between recur commands.

## Problem Description

When piping output from one `recur` command to another (e.g., `recur files | recur find --stdin`), the second command would fail silently because it received file paths contaminated with ANSI escape codes.

### Symptom

```powershell
# This silently failed
recur files "*agent*" -d docs | recur find "Agent" --scope "**" --stdin
```

The downstream `recur find` tried to open files with names like:
```
\e[0m\e[36mdocs\agents\recur-agent.md\e[0m
```

Instead of:
```
docs\agents\recur-agent.md
```

### Discovery

Hexdump analysis revealed ANSI color codes in piped output:

```
1B 5B 30 6D           = \e[0m      (reset)
1B 5B 33 36 6D        = \e[36m     (cyan)
64 6F 63 73 5C...     = docs\agents\recur-agent.md
0A                    = newline
1B 5B 30 6D           = \e[0m      (reset)
```

**Root Issue:** Recur was not detecting that stdout was a non-TTY (pipe/file) and continued emitting color codes regardless.

---

## Root Cause Analysis

**Location:** [src/output.rs:16](../src/output.rs#L16)

**Problem Code:**
```rust
pub fn new(color: bool) -> Self {
    let choice = if color {
        ColorChoice::Auto  // ❌ Not working correctly on Windows/pipes
    } else {
        ColorChoice::Never
    };
    // ...
}
```

**Issues Identified:**
1. `--color` flag defaulted to `true` in [main.rs:37](../src/main.rs#L37)
2. When `color` was `true`, it used `ColorChoice::Auto`
3. `ColorChoice::Auto` failed to detect non-TTY output on Windows/piped contexts
4. ANSI codes were emitted even when stdout was redirected

**Why it matters:** Standard Unix tools (`ls`, `git`, `rg`) automatically disable colors when output is piped. Recur violated this convention, breaking composability.

---

## Solution

### Implementation

Modified [src/output.rs](../src/output.rs) to explicitly check if stdout is a terminal before enabling colors:

```rust
use std::io::{Write, IsTerminal};

impl TerminalFormatter {
    pub fn new(color: bool) -> Self {
        // Only enable colors if both requested AND stdout is a terminal
        let is_tty = std::io::stdout().is_terminal();
        let should_color = color && is_tty;

        let choice = if should_color {
            ColorChoice::Always
        } else {
            ColorChoice::Never
        };
        Self {
            stdout: StandardStream::stdout(choice),
            color: should_color,
        }
    }
}
```

### Key Changes

1. ✅ Import `IsTerminal` trait from `std::io`
2. ✅ Explicitly check `std::io::stdout().is_terminal()`
3. ✅ Only enable colors when BOTH conditions are true:
   - User requested colors (`--color` flag)
   - Stdout is a terminal (not a pipe/file)
4. ✅ Use explicit `ColorChoice::Always` / `ColorChoice::Never` instead of `Auto`

---

## Verification

### Test: File Output (No ANSI codes)

```bash
# Redirect to file
recur files "main.**" -d docs > test.txt

# Verify clean output (no ANSI codes)
od -A x -t x1z test.txt
# Output shows: 64 6f 63 73 5c... (just "docs\..." text)
# No 1B 5B sequences (ANSI escape codes)
```

### Test: Piping Between Commands

```bash
# This now works correctly!
recur files "main.command.**" -d docs | recur find "callees" --scope "**" --stdin

# Output:
# docs\main.command.callees.readme.md:1:# main.command.callees.readme
# docs\main.command.callees.readme.md:3:Command overview for `callees`.
# ...
```

### Test: Terminal Output (Colors Still Work)

```bash
# Interactive terminal session
recur files "main.**" -d docs

# Output appears with cyan color for file paths (as expected)
```

---

## Impact

### Fixed
- ✅ **Piping between recur commands:** `recur | recur --stdin` now works correctly
- ✅ **File redirection:** Output to files is clean (no ANSI codes)
- ✅ **Composability:** Can now use recur in shell pipelines like standard Unix tools

### Preserved
- ✅ **Terminal colors:** When output goes to a terminal, colors still appear
- ✅ **User control:** `--color` flag still controls color preference
- ✅ **Backward compatibility:** Existing usage patterns unaffected

### Follows Best Practices
- ✅ Standard Unix convention (auto-detect TTY)
- ✅ Matches behavior of `rg`, `ls`, `git`, etc.
- ✅ Enables composable command pipelines

---

## Workaround (If Needed for Older Versions)

If using an unfixed version, manually strip ANSI codes:

```powershell
# PowerShell workaround
recur files "*agent*" -d docs | ForEach-Object { $_ -replace '\e\[[0-9;]*m','' } | ForEach-Object { $_.Trim() } | recur find "Agent" --scope "**" --stdin
```

**Note:** This workaround is no longer needed with the fix applied.

---

## Related

- Standard tool behavior: Tools like `rg`, `ls`, and `git` check `isatty()` before emitting colors
- Rust stdlib: `std::io::stdout().is_terminal()` available since Rust 1.70+
- Alternative: `atty` crate provides `atty::is(Stream::Stdout)` for older Rust versions

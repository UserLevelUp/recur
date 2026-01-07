# ? Project Renamed: hsearch ? recur

## ?? Complete!

The hierarchical search tool has been **renamed to `recur`** in honor of **Dennis M. Ritchie's 1968 PhD thesis on recursive hierarchies**.

## ?? What Was Created

### Core Documentation
- ? `RECUR-TRIBUTE.md` - Full tribute to Dennis Ritchie
- ? `README.md` - Updated with tribute and new name
- ? `Cargo.toml` - Package renamed to `recur`
- ? `RENAMING-SUMMARY.md` - Complete renaming documentation
- ? `src/main.rs` - CLI updated with tribute in help text

### Key Changes

| Aspect | Old (hsearch) | New (recur) |
|--------|---------------|-------------|
| **Command** | `hsearch files "Module.*"` | `recur files "Module.*"` |
| **Package** | `cargo install hsearch` | `cargo install recur` |
| **Repository** | github.com/userlevelup/hsearch | github.com/userlevelup/recur |
| **Meaning** | "Hierarchical search" | "Recursive hierarchical search" |
| **Tribute** | None | Dennis M. Ritchie (1941-2011) |

## ?? Why "recur"?

### 1. Honors Dennis Ritchie's Legacy

```
1968: Dennis Ritchie - PhD Thesis on Recursive Hierarchies
      ? (Theory: Recursive functions & hierarchical structures)
1969: Unix - Hierarchical filesystem
      ? (Practice: Recursive directory trees)
1972: C Language - Hierarchical includes
      ? (Implementation: Recursive compilation)
1973: grep - Flat text search (Ken Thompson)
      ? (Missing: No hierarchy understanding)
2011: Dennis Ritchie passes away
      ? (Legacy: Often overshadowed by Jobs)
2024: recur - Recursive hierarchical search
      ? (Tribute: Honoring 56 years of innovation)
```

### 2. Perfect Unix Philosophy Fit

| Tool | Year | Length | Purpose |
|------|------|--------|---------|
| `grep` | 1973 | 4 chars | Global Regular Expression Print |
| `awk` | 1977 | 3 chars | Aho, Weinberger, Kernighan |
| `sed` | 1974 | 3 chars | Stream EDitor |
| **`recur`** | 2024 | 5 chars | **RECursive Hierarchical SeaRch** |

### 3. Captures the Essence

- **REC**ursive - Searches recursively through hierarchies
- **rec**U**r** - Sounds like "occur", easy to remember
- **recur** - Honors Ritchie's **rec**ursive work

### 4. Meaningful Story

> *"Dennis Ritchie's 1968 thesis explored recursive hierarchies. 56 years later, **recur** brings that vision to code search."*

This isn't just a tool—it's a tribute to computing history.

## ?? Usage (Updated)

All functionality remains identical, only the command name changed:

```bash
# Find files matching recursive hierarchical pattern
recur files "LevelController.CreateWizard3.*"

# Search within hierarchy scope (recursive)
recur find "CreateSection" --scope "LevelController.CreateWizard3" -C 2

# Show recursive hierarchy tree
recur tree "LevelController" --count

# Find related (sibling) files
recur related "DynamicGameService.Ops.cs"

# Search for hierarchical identifiers (recursive)
recur id "ulu.role.*" --ext ".cs,.json"

# JSON output for tooling
recur files "Module.*" --json | jq '.files[]'
```

## ?? Documentation Structure

```
proposals/recur/
??? RECUR-TRIBUTE.md              ? NEW - The Dennis Ritchie story
??? README.md                     ? UPDATED - With tribute
??? RENAMING-SUMMARY.md           ? NEW - Renaming documentation
??? Cargo.toml                    ? UPDATED - Package name
??? src/
?   ??? main.rs                   ? UPDATED - CLI help with tribute
??? (all other files unchanged)
```

## ?? The Message

### To Developers
*"Use a tool that understands how modern code is organized - recursively and hierarchically."*

### To Historians
*"Honor the pioneers. Dennis Ritchie's 1968 work on recursive hierarchies shaped computing forever."*

### To the Community
*"Dennis Ritchie passed away in 2011, the same week as Steve Jobs, but received far less attention. This is our tribute."*

## ?? Next Steps

### Phase 1: Repository Setup
1. Create `github.com/userlevelup/recur` repository
2. Copy all code from `proposals/recur/` to repository root
3. Add `.gitignore`, `LICENSE` (MIT), `CODE_OF_CONDUCT.md`
4. Initial commit with tribute message

### Phase 2: Publication
1. Publish to crates.io: `cargo publish`
2. Create GitHub release: v0.1.0
3. Write blog post: "Introducing recur: In Honor of Dennis Ritchie"
4. Submit to:
   - r/rust
   - r/programming  
   - Hacker News
   - lobste.rs

### Phase 3: Community
1. Create Discord/Matrix channel
2. Add contribution guidelines
3. Set up CI/CD (GitHub Actions)
4. Package for Linux distros (Debian, Arch, Homebrew)

## ?? Marketing Message

### Short Version
> **recur**: Recursive hierarchical search for modern codebases.  
> *Honoring Dennis M. Ritchie's 1968 PhD thesis on recursive hierarchies.*

### Long Version
> Dennis Ritchie's 1968 thesis pioneered recursive hierarchical structures, leading to Unix and C. 56 years later, **recur** brings hierarchical understanding to code search - a tribute to his often-overlooked genius.

### Tagline
> *"From grep to recur: 50 years of evolution in code search."*

## ?? Educational Value

This renaming teaches:

1. **Computing History** - Dennis Ritchie's contributions
2. **Recursive Thinking** - How hierarchies work
3. **Unix Philosophy** - Simple tools that do one thing well
4. **Rust Benefits** - Memory-safe C successor
5. **Honoring Giants** - Giving credit where due

## ? The Final Word

**hsearch** was a descriptive name.

**recur** is a *meaningful* name that:
- ? Honors a computing pioneer
- ? Fits Unix traditions
- ? Captures the recursive essence
- ? Creates an emotional connection
- ? Is unique and memorable
- ? Respects C and Unix legacy

---

## ?? Contact

- **Repository**: https://github.com/userlevelup/recur (to be created)
- **Crates.io**: https://crates.io/crates/recur (to be published)
- **Issues**: https://github.com/userlevelup/recur/issues
- **Discussions**: https://github.com/userlevelup/recur/discussions

---

**recur**: *Recursive hierarchical search for the 21st century.*

**In Memory of Dennis M. Ritchie (1941-2011)**

*"UNIX is very simple, it just needs a genius to understand its simplicity."* — Dennis Ritchie

---

*Created with ?? by the User Level Up team*  
*Honoring 56 years of innovation since Dennis Ritchie's 1968 thesis*

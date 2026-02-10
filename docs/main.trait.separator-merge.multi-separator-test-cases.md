# Multi-Separator Test Cases (3+ Separators)

## Overview
Test cases for using 3 or more `--sep` flags simultaneously.
Documents behavior, edge cases, and expected outcomes.

---

## Behavior with 3+ Separators

### Core Mechanics
- Each `--sep` creates an independent domain/view
- Files are discovered separately for each separator
- Results are merged and deduplicated by path
- Markers show origin: `[.]`, `[_]`, `[-]`, `[/]`, etc.
- First separator wins for normalization target

### Example Hierarchy
```
component.user.service          # Dots
component_user_service          # Underscores
component-user-service          # Dashes
component/user/service          # Slashes
```

All four represent the same logical entity but in different naming conventions.

---

## Test Suite 1: Three Separators

### Test 1.1: Basic three-separator merge
**Setup:**
```
files/
  api.user.service.ts            # TypeScript (dots)
  api_user_service.py            # Python (underscores)
  api-user-service.yaml          # Config (dashes)
```

**Command:**
```bash
recur tree api.user.service --sep "." --sep "_" --sep "-" --show-sep
```

**Expected Output:**
```
api.user.service
├── ts [.]
├── py [_]
└── yaml [-]
```

**Verifies:** Three-way merge with distinct markers

---

### Test 1.2: Three separators with normalization
**Setup:** Same as 1.1

**Command:**
```bash
recur files "api.user.service.**" --sep "." --sep "_" --sep "-" --sep-replace-default "."
```

**Expected Output:**
```
api.user.service.ts
api.user.service.py          # Normalized from underscore
api.user.service.yaml        # Normalized from dash
```

**Verifies:** Normalization to first separator

---

### Test 1.3: Three separators - order matters
**Setup:** Same as 1.1

**Command A:**
```bash
recur files "api.user.service.**" --sep "." --sep "_" --sep "-" --sep-replace-default "_"
```

**Expected Output A:**
```
api_user_service.ts          # Normalized to underscore
api_user_service.py
api_user_service.yaml        # Normalized to underscore
```

**Command B:**
```bash
recur files "api.user.service.**" --sep "-" --sep "." --sep "_" --sep-replace-default "-"
```

**Expected Output B:**
```
api-user-service.ts          # Normalized to dash
api-user-service.py          # Normalized to dash
api-user-service.yaml
```

**Verifies:** First separator wins, user controls via ordering

---

### Test 1.4: Three separators with gaps
**Setup:**
```
files/
  component.widget.impl.ts       # Dots
  component_widget_test.js       # Underscores
  # Missing: component-widget-*.* (no dash files)
```

**Command:**
```bash
recur tree component.widget --sep "." --sep "_" --sep "-" --show-sep
```

**Expected Output:**
```
component.widget
├── impl.ts [.]
└── test.js [_]
# No [-] markers (no files found with dashes)
```

**Verifies:** Graceful handling when separator finds nothing

---

## Test Suite 2: Four Separators

### Test 2.1: Four-way polyglot merge
**Setup:**
```
project/
  user.service.ts                # TypeScript (dots)
  user_service.py                # Python (underscores)
  user-service.yaml              # Config (dashes)
  user/service.go                # Go (slashes)
```

**Command:**
```bash
recur tree user.service --sep "." --sep "_" --sep "-" --sep "/" --show-sep
```

**Expected Output:**
```
user.service
├── ts [.]
├── py [_]
├── yaml [-]
└── go [/]
```

**Verifies:** Four-way merge across language conventions

---

### Test 2.2: Four separators with deep nesting
**Setup:**
```
deep/
  a.b.c.d.file.txt               # 5 levels with dots
  a_b_c_d.file.txt               # 5 levels with underscores
  a-b-c-d.file.txt               # 5 levels with dashes
  a/b/c/d/file.txt               # 5 levels with slashes
```

**Command:**
```bash
recur tree a.b.c.d --sep "." --sep "_" --sep "-" --sep "/" --show-sep
```

**Expected Output:**
```
a.b.c.d
└── file
    ├── txt [.]
    ├── txt [_]
    ├── txt [-]
    └── txt [/]
```

**Verifies:** Deep hierarchy merge across conventions

---

### Test 2.3: Four separators normalized
**Setup:** Same as 2.1

**Command:**
```bash
recur files "user.service.**" --sep "." --sep "_" --sep "-" --sep "/" --sep-replace-default "."
```

**Expected Output:**
```
user.service.ts
user.service.py          # Normalized from underscore
user.service.yaml        # Normalized from dash
user.service.go          # Normalized from slash
```

**Verifies:** Multi-convention normalization

---

## Test Suite 3: Five+ Separators

### Test 3.1: Five separators
**Setup:**
```
files/
  test.case.file.txt             # Dots
  test_case_file.txt             # Underscores
  test-case-file.txt             # Dashes
  test/case/file.txt             # Slashes
  test:case:file.txt             # Colons (edge case)
```

**Command:**
```bash
recur tree test.case --sep "." --sep "_" --sep "-" --sep "/" --sep ":" --show-sep
```

**Expected Output:**
```
test.case
└── file
    ├── txt [.]
    ├── txt [_]
    ├── txt [-]
    ├── txt [/]
    └── txt [:]
```

**Verifies:** 5+ separator support

---

### Test 3.2: Six separators (stress test)
**Setup:** Files with `.`, `_`, `-`, `/`, `:`, `|` separators

**Command:**
```bash
recur files "test.**" --sep "." --sep "_" --sep "-" --sep "/" --sep ":" --sep "|"
```

**Expected:** All files found and deduplicated

**Verifies:** No hardcoded limit on separator count

---

## Test Suite 4: Edge Cases

### Test 4.1: All separators find nothing
**Setup:**
```
files/
  something-completely-different.txt
```

**Command:**
```bash
recur tree component.widget --sep "." --sep "_" --sep "-"
```

**Expected Output:**
```
(empty - no files found)
```

**Verifies:** Graceful empty result

---

### Test 4.2: Only one separator finds files
**Setup:**
```
files/
  api.user.service.ts
  api.order.service.ts
  # No files with underscores or dashes
```

**Command:**
```bash
recur tree api --sep "." --sep "_" --sep "-" --show-sep
```

**Expected Output:**
```
api
├── user
│   └── service.ts [.]
└── order
    └── service.ts [.]
```

**Verifies:** Single-separator results when others find nothing

---

### Test 4.3: Duplicate files (shouldn't happen but document)
**Setup:**
```
files/
  test.file.txt                  # Dots
  test.file.txt                  # Same exact path (impossible in real FS)
```

**Command:**
```bash
recur files "test.**" --sep "." --sep "_"
```

**Expected Output:**
```
test.file.txt
# (Only appears once - deduplicated by path)
```

**Verifies:** Path-based deduplication

---

### Test 4.4: Separator characters in filenames
**Setup:**
```
files/
  test.with.dots.txt             # Dots in name
  test_with_underscores.txt      # Underscores in name
  test.with_mixed.separators.txt # Both!
```

**Command:**
```bash
recur tree test --sep "." --sep "_" --show-sep
```

**Expected Output:**
```
test
├── with
│   ├── dots.txt [.]
│   └── underscores.txt [_]
└── with_mixed.separators.txt [.]  # Parsed by first separator found
```

**Verifies:** Separator precedence in parsing

---

### Test 4.5: Empty separator (edge case)
**Command:**
```bash
recur tree test --sep "" --sep "."
```

**Expected:** Error or ignored (empty separator is invalid)

**Verifies:** Input validation

---

### Test 4.6: Repeated separators
**Command:**
```bash
recur tree test --sep "." --sep "." --sep "_"
```

**Expected:** Deduplicated internally (only `.` and `_` used)

**Verifies:** Separator deduplication

---

## Test Suite 5: Complex Real-World Scenarios

### Test 5.1: Monorepo with many conventions
**Setup:**
```
monorepo/
  @company/user.service/src/index.ts     # Scoped package (dot)
  @company/user-service/src/index.ts     # Scoped package (dash)
  packages/user_service/src/index.ts     # Unscoped (underscore)
  apps/user/service/index.ts             # Path-based (slash)
```

**Command:**
```bash
recur tree user.service --sep "." --sep "-" --sep "_" --sep "/" --show-sep
```

**Expected:** All four locations appear in unified tree

**Verifies:** Real monorepo complexity

---

### Test 5.2: Documentation with mixed separators
**Setup:**
```
docs/
  api.reference.authentication.md        # Dots
  api_reference_authentication.md        # Underscores (legacy)
  api-reference-authentication.md        # Dashes (URL-friendly)
```

**Command:**
```bash
recur files "api.reference.authentication.**" --sep "." --sep "_" --sep "-" --show-sep
```

**Expected Output:**
```
api.reference.authentication.md [.]
api_reference_authentication.md [_]
api-reference-authentication.md [-]
```

**Verifies:** Documentation migration tracking

---

### Test 5.3: Migration phase with three conventions
**Setup:**
```
# Migration: underscore -> dash -> dot
legacy/
  user_service_impl.rs           # Old (underscore)
migrating/
  user-service-impl.rs           # Middle (dash)
modern/
  user.service.impl.rs           # New (dot)
```

**Command:**
```bash
recur tree user.service.impl --sep "_" --sep "-" --sep "." --show-sep
```

**Expected Output:**
```
user.service.impl
├── legacy.rs [_]
├── migrating.rs [-]
└── modern.rs [.]
```

**Verifies:** Multi-phase migration tracking

---

## Test Suite 6: Performance

### Test 6.1: Many separators performance
**Setup:** 10,000+ files with mixed separators

**Command:**
```bash
time recur files "**" --sep "." --sep "_" --sep "-" --sep "/" --sep ":"
```

**Expected:** Complete in <10 seconds

**Verifies:** Performance doesn't degrade linearly with separator count

---

### Test 6.2: Deep nesting with many separators
**Setup:**
```
deep/
  a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.file.txt     # 16 levels
  (same path with _, -, /, : separators)
```

**Command:**
```bash
recur tree a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p --sep "." --sep "_" --sep "-" --sep "/" --sep ":"
```

**Expected:** All versions found and merged

**Verifies:** Deep nesting doesn't break with many separators

---

## Test Suite 7: Normalization Edge Cases

### Test 7.1: Normalize with third separator
**Setup:**
```
files/
  api.user.ts
  api_user.py
  api-user.yaml
```

**Command:**
```bash
# First separator is dot, but normalize to dash (third separator)
recur files "api.user.**" --sep "." --sep "_" --sep "-" --sep-replace-default "-"
```

**Expected Output:**
```
api-user.ts          # Normalized to dash
api-user.py          # Normalized to dash
api-user.yaml        # Already dash
```

**Verifies:** Can normalize to any separator in list

---

### Test 7.2: Normalize with separator not in list
**Setup:**
```
files/
  api.user.ts
  api_user.py
```

**Command:**
```bash
recur files "api.**" --sep "." --sep "_" --sep-replace-default "/"
```

**Expected:** Error or warning (can't normalize to separator not in search list)

**Verifies:** Normalization validation

---

## Test Suite 8: Marker Display Rules

### Test 8.1: Markers only show with multiple separators
**Setup:**
```
files/
  test.file.txt
```

**Command A (single separator):**
```bash
recur files "test.**" --sep "." --show-sep
```

**Expected Output A:**
```
test.file.txt
# No markers (only one separator used)
```

**Command B (multiple separators):**
```bash
recur files "test.**" --sep "." --sep "_" --show-sep
```

**Expected Output B:**
```
test.file.txt [.]
# Markers shown (multiple separators)
```

**Verifies:** Marker display logic for 1 vs 2+ separators

---

### Test 8.2: Markers with many separators
**Setup:**
```
files/
  test.file.txt
  test_file.txt
  test-file.txt
  test/file.txt
  test:file.txt
```

**Command:**
```bash
recur files "test.file.**" --sep "." --sep "_" --sep "-" --sep "/" --sep ":" --show-sep
```

**Expected Output:**
```
test.file.txt [.]
test_file.txt [_]
test-file.txt [-]
test/file.txt [/]
test:file.txt [:]
```

**Verifies:** Each file gets correct marker

---

## Test Suite 9: Gap Analysis with 3+ Separators

### Test 9.1: Three-domain gap analysis
**Setup:**
```
project/
  user.service.ts                # TypeScript (dots)
  user.service.test.ts           # TypeScript tests (dots)
  user_service.py                # Python (underscores)
  # Missing: user_service.test.py (Python tests)
  user-service.yaml              # Config (dashes)
  # Missing: user-service.test.yaml (Config validation)
```

**Command:**
```bash
recur tree user.service --sep "." --sep "_" --sep "-" --show-sep
```

**Expected Output:**
```
user.service
├── ts [.]
├── test.ts [.]
├── py [_]
└── yaml [-]
```

**Analysis:**
```bash
# Find which domains lack tests
recur files "user.service.**" --sep "." --sep "_" --sep "-" --show-sep | \
  grep -v "test" | cut -d' ' -f2 | sort -u
```

**Expected Analysis Output:**
```
[_]    # Underscore domain has no tests
[-]    # Dash domain has no tests
```

**Verifies:** Cross-domain completeness checking

---

### Test 9.2: Five-way completeness matrix
**Setup:**
```
entity/
  api.user.spec.md               # Spec (dots)
  api_user.impl.rs               # Implementation (underscores)
  api-user.test.js               # Tests (dashes)
  api/user.docs.html             # Documentation (slashes)
  # Missing: api:user.deploy.yaml (Deployment config, colons)
```

**Command:**
```bash
recur tree api.user --sep "." --sep "_" --sep "-" --sep "/" --sep ":" --show-sep
```

**Expected Output:**
```
api.user
├── spec.md [.]
├── impl.rs [_]
├── test.js [-]
└── docs.html [/]
# Missing [:] marker entirely
```

**Gap Analysis:**
```bash
# Check for missing deployment config
recur files "api.user.**" --sep "." --sep "_" --sep "-" --sep "/" --sep ":" --show-sep | \
  grep -q "\[:\]" || echo "Missing deployment config"
```

**Expected:** "Missing deployment config"

**Verifies:** Multi-domain completeness verification

---

## Summary

**Test Coverage for 3+ Separators:**

- ✅ Three-separator merge and normalization
- ✅ Four-separator polyglot projects
- ✅ Five+ separator stress testing
- ✅ Edge cases (empty results, duplicates, invalid input)
- ✅ Complex real-world scenarios (monorepos, migrations)
- ✅ Performance with many separators
- ✅ Normalization edge cases
- ✅ Marker display rules
- ✅ Gap analysis with multiple domains

**Key Behaviors Documented:**
1. Each separator creates independent domain
2. Results are merged and deduplicated by path
3. First separator wins for normalization
4. Markers only show when 2+ separators used
5. Empty results from some separators is OK
6. No hardcoded limit on separator count
7. Performance scales reasonably with separator count

**Implementation Validation:**
- Current code uses `Vec<char>` - supports unlimited separators ✅
- Deduplication by path - prevents duplicates ✅
- First-separator-wins normalization - documented and tested ✅
- Marker logic (`separators.len() > 1`) - correct ✅

**Status:** Feature fully specified for 3+ separator scenarios.

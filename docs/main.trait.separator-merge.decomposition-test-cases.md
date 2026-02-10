# Separator-Merge: Decomposition/Recomposition Test Cases

## Overview
Test cases for real-world decomposition/recomposition scenarios.
These go beyond basic functionality to verify practical use cases.

---

## Test Suite 1: XSLT Pipeline Management

### Test 1.1: Track XSLT fragments across naming conventions
**Setup:**
```
xslt/
  transform.user.profile.xsl
  transform.user.settings.xsl
  transform.order-items.xsl          # Dash-separated!
  transform.order-total.xsl
  transform.invoice-header.xsl       # Dash-separated!
```

**Command:**
```bash
recur tree transform --sep "." --sep "-" --show-sep
```

**Expected Output:**
```
transform
├── user
│   ├── profile.xsl [.]
│   └── settings.xsl [.]
├── order
│   ├── items.xsl [-]
│   └── total.xsl [.]
└── invoice
    └── header.xsl [-]
```

**Verifies:** Mixed separator tracking in XSLT fragments

---

### Test 1.2: Find XSLT transforms missing tests
**Setup:**
```
xslt/
  transform.user.profile.xsl
  transform.user.profile.test.xsl
  transform.user.settings.xsl
  # Missing: transform.user.settings.test.xsl
```

**Command:**
```bash
recur files "transform.user.**" --sep "." | grep -v "\.test\.xsl" | \
while read f; do
  test_file="${f%.xsl}.test.xsl"
  [ ! -f "$test_file" ] && echo "Missing test: $f"
done
```

**Expected Output:**
```
Missing test: transform.user.settings.xsl
```

**Verifies:** Gap analysis for test coverage

---

### Test 1.3: Recompose XSLT by module
**Setup:**
```
xslt/
  transform.user.profile.xsl
  transform.user.settings.xsl
  transform.order.items.xsl
```

**Command:**
```bash
recur files "transform.user.**" --sep "." | xargs cat
```

**Expected:** Concatenated content of both user-related XSLT files

**Verifies:** Query-based recomposition

---

## Test Suite 2: JSON Schema Composition

### Test 2.1: Track schemas across team conventions
**Setup:**
```
schemas/
  api.user.profile.schema.json       # Frontend team (dots)
  api_user_profile.schema.json       # Backend team (underscores)
  api.user.settings.schema.json      # Frontend
  # Missing: api_user_settings.schema.json (Backend)
```

**Command:**
```bash
recur tree api.user --sep "." --sep "_" --show-sep
```

**Expected Output:**
```
api.user
├── profile
│   ├── schema.json [.]
│   └── schema.json [_]
└── settings
    └── schema.json [.]
```

**Verifies:** Cross-team schema tracking, visible gap for backend settings

---

### Test 2.2: Find schema mismatches between teams
**Setup:** Same as 2.1

**Command:**
```bash
# Find frontend schemas without backend equivalents
recur files "api.**" --sep "." --show-sep | grep "\[.\]" | \
while read line; do
  file=$(echo "$line" | awk '{print $1}')
  base=$(basename "$file" .schema.json)
  backend_equiv=$(echo "$file" | sed 's/\./\_/g')
  [ ! -f "$backend_equiv" ] && echo "No backend schema: $file"
done
```

**Expected Output:**
```
No backend schema: api.user.settings.schema.json
```

**Verifies:** Team alignment verification

---

### Test 2.3: Normalize schema references
**Setup:**
```
schemas/
  api.user.profile.schema.json
  api_order_items.schema.json
```

**Command:**
```bash
recur files "api.**" --sep "." --sep "_" --sep-replace-default "."
```

**Expected Output:**
```
api.user.profile.schema.json
api.order.items.schema.json      # Normalized from underscore
```

**Verifies:** Unified schema listing for generation

---

## Test Suite 3: Configuration Management

### Test 3.1: Verify environment parity
**Setup:**
```
config/
  prod.database.host.yaml
  prod.database.credentials.yaml
  prod_redis_url.yaml              # Env var convention
  dev.database.host.yaml
  dev.database.credentials.yaml
  dev_redis_url.yaml
  dev_kafka_brokers.yaml           # Missing in prod!
```

**Command:**
```bash
recur tree prod --sep "." --sep "_"
recur tree dev --sep "." --sep "_"
```

**Expected (prod):**
```
prod
├── database
│   ├── host.yaml
│   └── credentials.yaml
└── redis
    └── url.yaml
```

**Expected (dev):**
```
dev
├── database
│   ├── host.yaml
│   └── credentials.yaml
├── redis
│   └── url.yaml
└── kafka
    └── brokers.yaml     # Missing in prod!
```

**Verifies:** Environment configuration completeness

---

### Test 3.2: Build environment config with markers
**Setup:** Same as 3.1

**Command:**
```bash
recur files "prod.**" --sep "." --sep "_" --show-sep
```

**Expected Output:**
```
prod.database.host.yaml [.]
prod.database.credentials.yaml [.]
prod_redis_url.yaml [_]
```

**Verifies:** Config source tracking (file-based vs env-var-based)

---

### Test 3.3: Deployment verification
**Setup:**
```
config/
  prod.database.host.yaml
  prod.cache.redis.yaml
  prod_queue_kafka.yaml
```

**Command:**
```bash
# Check all required services have configs
for service in database cache queue; do
  recur files "prod.$service.**" --sep "." --sep "_" >/dev/null 2>&1 || \
    echo "Missing config: $service"
done
```

**Expected:** (no output if all present, or error messages for missing)

**Verifies:** Pre-deployment config validation

---

## Test Suite 4: Living Documentation

### Test 4.1: Verify code documentation completeness
**Setup:**
```
src/
  user_service.rs
  order_processor.rs
docs/
  UserService.readme.md
  # Missing: OrderProcessor.readme.md
tests/
  UserService.test.js
  OrderProcessor.test.js
```

**Command:**
```bash
recur tree UserService --sep "." --sep "_" --show-sep
recur tree OrderProcessor --sep "." --sep "_" --show-sep
```

**Expected (UserService):**
```
UserService
├── readme.md [.]
├── service.rs [_]
└── test.js [.]
```

**Expected (OrderProcessor):**
```
OrderProcessor
├── processor.rs [_]
└── test.js [.]
# Missing readme.md!
```

**Verifies:** Documentation coverage gaps

---

### Test 4.2: CI/CD documentation gate
**Setup:** Same as 4.1

**Command:**
```bash
# PR check: every _service.rs must have .readme.md
recur files "**" --sep "_" --show-sep | grep "service.rs \[_\]" | \
while read line; do
  file=$(echo "$line" | awk '{print $1}')
  base=$(basename "$file" _service.rs)
  doc="docs/${base^}Service.readme.md"
  [ ! -f "$doc" ] && echo "Missing docs for: $file"
done
```

**Expected Output:**
```
Missing docs for: order_processor.rs
```

**Verifies:** Automated documentation enforcement

---

### Test 4.3: Multi-representation entity view
**Setup:**
```
src/
  user_service.rs
docs/
  UserService.readme.md
tests/
  UserService.test.js
api/
  UserService.spec.yaml
```

**Command:**
```bash
recur files "UserService.**" --sep "." --sep "_" --show-sep
```

**Expected Output:**
```
UserService.readme.md [.]
UserService.spec.yaml [.]
UserService.test.js [.]
user_service.rs [_]
```

**Verifies:** Complete entity view across representations

---

### Test 4.4: Parallel team coordination
**Setup:**
```
# Day 1: Dev team creates code
src/user_service.rs

# Day 2: Docs team creates docs
docs/UserService.readme.md

# Day 3: QA team creates tests
# (not yet created)
```

**Command:**
```bash
recur tree UserService --sep "." --sep "_" --show-sep
```

**Expected Output:**
```
UserService
├── readme.md [.]      # ✅ Docs done
└── service.rs [_]     # ✅ Code done
# ❌ Missing: test.js
```

**Verifies:** Real-time parallel work tracking

---

## Test Suite 5: Multi-Language Projects

### Test 5.1: Polyglot entity view
**Setup:**
```
frontend/
  user.service.ts              # TypeScript
backend/
  user_service.py              # Python
cache/
  user/service.go              # Go
queue/
  user_service.rs              # Rust
```

**Command:**
```bash
recur tree user.service --sep "." --sep "_" --sep "/" --show-sep
```

**Expected Output:**
```
user.service
├── frontend.ts [.]
├── backend.py [_]
├── cache.go [/]
└── queue.rs [_]
```

**Verifies:** Cross-language unified view

---

### Test 5.2: Cross-language refactoring safety
**Setup:** Same as 5.1

**Command:**
```bash
# Find all files to rename during "user.service" -> "account.service" refactor
recur files "user.service.**" --sep "." --sep "_" --sep "/" --show-sep
```

**Expected Output:**
```
user.service.frontend.ts [.]
user_service.backend.py [_]
user/service.cache.go [/]
user_service.queue.rs [_]
```

**Verifies:** Complete refactoring checklist

---

### Test 5.3: Language convention normalization
**Setup:** Same as 5.1

**Command:**
```bash
recur files "user.service.**" --sep "." --sep "_" --sep "/" --sep-replace-default "."
```

**Expected Output:**
```
user.service.frontend.ts
user.service.backend.py       # Normalized from underscore
user.service.cache.go         # Normalized from slash
user.service.queue.rs         # Normalized from underscore
```

**Verifies:** Language-agnostic display

---

## Test Suite 6: Build Artifact Tracking

### Test 6.1: Verify build completeness
**Setup:**
```
build/
  component_widget.cpp           # Source
  component.widget.o             # Object
  libcomponent-widget.so         # Library
  component.widget.html          # Docs
  # Missing: component_widget_test.out
```

**Command:**
```bash
recur tree component.widget --sep "." --sep "_" --sep "-" --show-sep
```

**Expected Output:**
```
component.widget
├── cpp [_]          # Source
├── o [.]            # Object
├── so [-]           # Library
└── html [.]         # Docs
# Missing: test.out [_]
```

**Verifies:** Build pipeline completeness

---

### Test 6.2: Build artifact dependency check
**Setup:** Same as 6.1

**Command:**
```bash
# Check: every .cpp must have .o and .so
recur files "component.**" --sep "." --sep "_" --sep "-" --show-sep | \
  grep "\.cpp \[_\]" | while read line; do
  base=$(echo "$line" | awk '{print $1}' | sed 's/_widget\.cpp$//')
  obj="${base}.widget.o"
  lib="lib${base}-widget.so"
  [ ! -f "$obj" ] && echo "Missing object: $line"
  [ ! -f "$lib" ] && echo "Missing library: $line"
done
```

**Expected:** (error messages for incomplete builds)

**Verifies:** Build dependency validation

---

### Test 6.3: Find stale build artifacts
**Setup:**
```
build/
  component_widget.cpp           # Modified 10:00 AM
  component.widget.o             # Modified 09:00 AM (stale!)
  libcomponent-widget.so         # Modified 09:00 AM (stale!)
```

**Command:**
```bash
# Find .o and .so older than .cpp
recur files "component.**" --sep "." --sep "_" --sep "-" | \
  grep "\.cpp$" | while read cpp; do
  base=$(basename "$cpp" .cpp)
  obj="${cpp%.cpp}.o"
  [ -f "$obj" ] && [ "$cpp" -nt "$obj" ] && echo "Stale: $obj"
done
```

**Expected Output:**
```
Stale: component.widget.o
```

**Verifies:** Build cache invalidation

---

## Test Suite 7: Automated Quality Gates

### Test 7.1: PR documentation requirement
**Setup:**
```
feature/
  NewFeature_impl.rs            # Code added
  # Missing: NewFeature.readme.md
  # Missing: NewFeature.test.js
```

**Command:**
```bash
# CI check: new code requires docs + tests
recur files "NewFeature.**" --sep "." --sep "_" --show-sep | \
  awk '/\[_\].*impl\.rs/ {code=1}
       /\[.\].*readme\.md/ {docs=1}
       /\[.\].*test\.js/ {tests=1}
       END {
         if (code && !docs) {print "❌ Missing docs"; exit 1}
         if (code && !tests) {print "❌ Missing tests"; exit 1}
         print "✅ Complete"
       }'
```

**Expected Output:**
```
❌ Missing docs
```

**Expected Exit Code:** 1 (fails PR)

**Verifies:** Automated PR quality gate

---

### Test 7.2: Documentation coverage badge
**Setup:**
```
src/
  user_service.rs
  order_processor.rs
  payment_handler.rs
docs/
  UserService.readme.md
  OrderProcessor.readme.md
  # Missing: PaymentHandler.readme.md
```

**Command:**
```bash
# Calculate documentation coverage percentage
coverage=$(recur files "src.**" --sep "_" --show-sep | \
           awk '/\.rs \[_\]/ {src++}
                END {print src}')
documented=$(recur files "docs.**" --sep "." --show-sep | \
             awk '/readme\.md \[.\]/ {docs++}
                  END {print docs}')
echo "Coverage: $(( documented * 100 / coverage ))%"
```

**Expected Output:**
```
Coverage: 66%
```

**Verifies:** Documentation metrics generation

---

### Test 7.3: Parallel team dashboard
**Setup:**
```
feature/
  auth.spec.md                  # Spec team done
  auth_impl.rs                  # Dev team done
  # Missing: auth.test.js       # QA team pending
```

**Command:**
```bash
# Generate team status dashboard
for feature in $(recur files "feature.**" --sep "." | cut -d. -f1 | sort -u); do
  status=$(recur files "$feature.**" --sep "." --sep "_" --show-sep)

  has_spec=$(echo "$status" | grep -q "spec.md \[.\]" && echo "✅" || echo "❌")
  has_code=$(echo "$status" | grep -q "impl.rs \[_\]" && echo "✅" || echo "❌")
  has_test=$(echo "$status" | grep -q "test.js \[.\]" && echo "✅" || echo "❌")

  echo "$feature | Spec: $has_spec | Code: $has_code | Test: $has_test"
done
```

**Expected Output:**
```
auth | Spec: ✅ | Code: ✅ | Test: ❌
```

**Verifies:** Cross-team coordination dashboard

---

## Test Suite 8: Complex Real-World Scenarios

### Test 8.1: Monorepo with mixed conventions
**Setup:**
```
packages/
  @company/user-service/         # Scoped package (dash)
    src/
      user_service.ts            # Source (underscore)
  @company/user.service/         # Different scoped package (dot)
    src/
      user.service.ts            # Source (dot)
docs/
  company.user.service.md        # Docs (dot)
```

**Command:**
```bash
recur tree user.service --sep "." --sep "_" --sep "-" --sep "/" --show-sep
```

**Expected:** Unified view across all naming conventions

**Verifies:** Complex real-world naming patterns

---

### Test 8.2: Migration tracking
**Setup:**
```
# Migration: underscore -> dot convention
legacy/
  user_service_impl.rs           # Old style
  order_processor_impl.rs        # Old style
modern/
  user.service.impl.rs           # New style
  # Missing: order.processor.impl.rs (not migrated yet)
```

**Command:**
```bash
# Find unmigrated files
recur files "**" --sep "_" --show-sep | grep "legacy.*\[_\]" | \
while read line; do
  file=$(echo "$line" | awk '{print $1}')
  modern_equiv=$(echo "$file" | sed 's/legacy/modern/' | sed 's/_/./g')
  [ ! -f "$modern_equiv" ] && echo "Not migrated: $file"
done
```

**Expected Output:**
```
Not migrated: legacy/order_processor_impl.rs
```

**Verifies:** Code migration tracking

---

### Test 8.3: Generated code verification
**Setup:**
```
schema/
  api.user.schema.json
  api.order.schema.json
generated/
  api_user.ts                    # Generated from schema
  # Missing: api_order.ts (generation failed?)
```

**Command:**
```bash
# Verify every schema has generated code
recur files "schema.**" --sep "." --show-sep | grep "\.schema\.json" | \
while read line; do
  file=$(echo "$line" | awk '{print $1}')
  base=$(basename "$file" .schema.json)
  gen="generated/${base//./_}.ts"
  [ ! -f "$gen" ] && echo "Missing generated code: $file"
done
```

**Expected Output:**
```
Missing generated code: schema/api.order.schema.json
```

**Verifies:** Code generation completeness

---

## Test Suite 9: Performance & Edge Cases

### Test 9.1: Large monorepo performance
**Setup:** 10,000+ files with mixed separators

**Command:**
```bash
time recur files "**" --sep "." --sep "_" --sep "-"
```

**Expected:** Complete in <5 seconds

**Verifies:** Performance at scale

---

### Test 9.2: Deeply nested hierarchies
**Setup:**
```
deep/
  a.b.c.d.e.f.g.h.i.j.file.txt
  a_b_c_d_e_f_g_h_i_j.file.txt
```

**Command:**
```bash
recur tree a.b.c.d.e.f.g.h.i.j --sep "." --sep "_"
```

**Expected:** Both files appear under unified hierarchy

**Verifies:** Deep nesting support

---

### Test 9.3: Special characters in separators
**Setup:**
```
files/
  component-widget.impl.ts       # Dash and dot
  component_widget.impl.ts       # Underscore and dot
  component.widget.impl.ts       # Only dots
```

**Command:**
```bash
recur tree component --sep "." --sep "-" --sep "_" --show-sep
```

**Expected:** All three files unified under "component.widget.impl"

**Verifies:** Multiple separator handling

---

## Summary

**Total Test Suites:** 9
**Total Test Cases:** 30+

**Coverage:**
- ✅ XSLT/XML pipeline management
- ✅ JSON schema composition
- ✅ Configuration management
- ✅ Living documentation
- ✅ Multi-language projects
- ✅ Build artifact tracking
- ✅ Automated quality gates
- ✅ Complex real-world scenarios
- ✅ Performance & edge cases

**Next Steps:**
1. Implement test cases in Julia test framework
2. Create fixture data for each scenario
3. Add CI/CD pipeline integration tests
4. Document expected behavior for edge cases

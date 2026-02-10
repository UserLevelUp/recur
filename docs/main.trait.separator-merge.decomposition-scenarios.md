# Decomposition & Recomposition Scenarios

## The Core Pattern

**Decomposition:** Breaking complex structures into manageable pieces
**Recomposition:** Reassembling pieces based on context/need
**Separator-Merge Role:** Track pieces across different naming domains + verify completeness

## Scenario 1: Modular XSLT Transformation Pipeline

### Traditional Problem
XSLT transformations become monolithic:
```xml
<!-- monolithic-transform.xsl -->
<xsl:stylesheet>
  <!-- 2000+ lines of mixed concerns -->
  <xsl:template match="user">...</xsl:template>
  <xsl:template match="order">...</xsl:template>
  <xsl:template match="invoice">...</xsl:template>
</xsl:stylesheet>
```

### Decomposed Approach
```
xslt/
  transform.user.profile.xsl
  transform.user.settings.xsl
  transform.order.items.xsl
  transform.order.total.xsl
  transform.invoice.header.xsl
  transform.invoice.line-items.xsl
```

### Where Separator-Merge Helps

**Track template coverage:**
```bash
recur tree transform --sep "." --sep "-" --show-sep

# Output:
transform
├── user
│   ├── profile.xsl [.]
│   ├── settings.xsl [.]
├── order
│   ├── items.xsl [.]
│   ├── total.xsl [.]
├── invoice
│   ├── header.xsl [.]
│   └── line-items.xsl [-]    # Different naming convention!
```

**Gap analysis:**
```bash
# Find missing test transforms
recur files "transform.**" --sep "." --show-sep | grep -v "\.test\.xsl"
# Shows which transforms lack test files
```

**Recomposition command:**
```bash
# Build complete XSLT by concatenating fragments
recur files "transform.user.**" --sep "." --sep "-" | \
  xargs cat > compiled-user-transform.xsl
```

**New capability:** Track XSLT fragments across different XML naming conventions (dash-separated element names vs dot-separated file organization).

---

## Scenario 2: JSON Schema Composition

### Traditional Problem
Large JSON schemas are monolithic or ad-hoc includes:
```json
{
  "$ref": "user-schema.json",
  "$ref": "order_schema.json",   // Mixed conventions!
  "$ref": "invoice.schema.json"
}
```

**Problem:** Can't verify completeness, references use different conventions.

### Decomposed Approach
```
schemas/
  api.user.profile.schema.json
  api.user.settings.schema.json
  api_order_items.schema.json        # Different naming!
  api_order_total.schema.json
  api.invoice.header.schema.json
```

### Where Separator-Merge Helps

**Verify schema completeness:**
```bash
recur tree api --sep "." --sep "_" --show-sep

# Shows which parts use which convention
# Missing nodes = missing schemas
```

**Parallel development tracking:**
```bash
# Frontend team uses dots, backend team uses underscores
recur files "api.user.**" --sep "." --sep "_" --show-sep

# Instantly see:
#   api.user.profile.schema.json [.]  - Frontend schema
#   api_user_profile.schema.json [_]  - Backend schema
#
# If mismatch → teams not aligned!
```

**Recomposition:**
```bash
# Generate combined schema for deployment
recur files "api.**" --sep "." --sep "_" | \
  jq -s '{"$defs": map({(.title): .}) | add}' > combined-api-schema.json
```

**New capability:** Track schema fragments across team conventions, verify API contract completeness.

---

## Scenario 3: Configuration File Decomposition

### Traditional Problem
Configuration as single monolithic file:
```yaml
# config.yaml (1000+ lines)
database:
  host: ...
  credentials: ...
redis:
  url: ...
kafka:
  brokers: ...
feature-flags:
  enable-new-ui: ...
```

### Decomposed Approach
```
config/
  prod.database.host.yaml
  prod.database.credentials.yaml
  prod_redis_url.yaml              # Environment uses underscore!
  prod_kafka_brokers.yaml
  dev.database.host.yaml
  dev_redis_url.yaml
```

**Mixed conventions:**
- Service configs: dot-separated (`database.host`)
- Environment vars: underscore-separated (`redis_url`)

### Where Separator-Merge Helps

**Environment completeness check:**
```bash
# Verify prod has all configs that dev has
recur files "prod.**" --sep "." --sep "_" --show-sep > prod-configs.txt
recur files "dev.**" --sep "." --sep "_" --show-sep > dev-configs.txt

# Compare:
diff <(awk '{print $1}' prod-configs.txt | sed 's/prod/ENV/') \
     <(awk '{print $1}' dev-configs.txt | sed 's/dev/ENV/')

# Missing in prod = configuration gap!
```

**Recomposition per environment:**
```bash
# Build prod config
recur files "prod.**" --sep "." --sep "_" --sep-replace-default "." | \
  xargs yq eval-all 'reduce .[] as $item ({}; . * $item)' > prod-final.yaml
```

**Deployment verification:**
```bash
# Before deploy: check all services have configs
recur tree prod --sep "." --sep "_" --show-sep

# Expected structure:
prod
├── database [.]
├── redis [_]      # Different domain marker!
├── kafka [_]
└── feature-flags [.]

# Missing service = deploy will fail!
```

**New capability:** Track configs across environment variable conventions + service conventions, verify deployment completeness.

---

## Scenario 4: Documentation Pipeline (The Killer Use Case)

### The Problem: Living Documentation
Documentation lives separately from code:
- Code: `src/user_service.rs`, `src/order_processor.cpp`
- Docs: `docs/UserService.md`, `docs/OrderProcessor.md`
- Tests: `tests/user.service.test.js`
- API specs: `api/user.service.yaml`

**Question:** "Is UserService fully documented?"

**Answer requires checking 4+ places with different naming conventions.**

### Where Separator-Merge Shines

**Complete entity view:**
```bash
recur tree UserService --sep "." --sep "_" --sep "/" --show-sep

# Output:
UserService
├── authentication
│   ├── readme.md [.]           # docs/UserService.authentication.readme.md
│   ├── test.js [.]             # tests/UserService.authentication.test.js
│   ├── impl.rs [_]             # src/user_service_authentication.rs
│   ├── api.yaml [.]            # api/UserService.authentication.yaml
│   └── integration.test [?]    # MISSING!

# One view, all representations, gaps visible
```

**CI/CD Documentation Check:**
```bash
#!/bin/bash
# Enforce: every source file must have docs

recur files "UserService.**" --sep "." --sep "_" --show-sep | \
  awk '/\[_\]/ {src=$1} /\[.\]/ && /readme\.md/ {docs[$1]=1}
       END {for (s in src) {
         doc_name = gensub(/_/, ".", "g", s) ".readme.md";
         if (!(doc_name in docs)) print "Missing docs:", s
       }}'

# Fails PR if source lacks docs
```

**Parallel team coordination:**
```bash
# Alice (docs team): Working on UserService.authentication.readme.md
# Bob (dev team): Working on user_service_authentication.rs
# Charlie (QA): Working on UserService.authentication.test.js

# Status check:
recur tree UserService.authentication --sep "." --sep "_" --show-sep

# Shows who's done:
#   readme.md [.]  ✅ Alice
#   impl.rs [_]    ✅ Bob
#   test.js [.]    ⏳ Charlie in progress (file exists but incomplete)
```

**Recomposition: Generate Unified Docs:**
```bash
# Build complete API documentation by merging all fragments
recur files "api.**" --sep "." --sep "_" | \
  while read file; do
    echo "## $(basename $file .yaml)"
    cat "$file"
  done > complete-api-docs.md
```

**New capability:** Living documentation with automatic completeness tracking.

---

## Scenario 5: Multi-Language Project Bridge

### The Problem
Polyglot projects use different conventions:
- **TypeScript:** `user.service.ts` (dots)
- **Python:** `user_service.py` (underscores)
- **Go:** `user/service.go` (paths)
- **Rust:** `user_service.rs` (underscores)

**Same logical entity, incompatible file naming.**

### Where Separator-Merge Creates Bridge

**Unified view:**
```bash
recur tree user.service --sep "." --sep "_" --sep "/" --show-sep

# Output:
user.service
├── frontend.ts [.]      # TypeScript implementation
├── backend.py [_]       # Python implementation
├── cache.go [/]         # Go implementation
└── queue.rs [_]         # Rust implementation

# One entity, four languages, unified view
```

**Cross-language refactoring:**
```bash
# Renaming "user.service" -> "account.service"

# Find all representations:
recur files "user.service.**" --sep "." --sep "_" --sep "/" --show-sep

# Output shows every file to rename across all languages:
user.service.frontend.ts [.]
user_service.backend.py [_]
user/service.cache.go [/]
user_service.queue.rs [_]

# Ensures no files missed during refactor
```

**New capability:** Navigate polyglot codebases as if single language.

---

## Scenario 6: Build Artifact Tracking

### The Problem
Build systems generate artifacts with different naming:
- Source: `component_widget.cpp`
- Object: `component.widget.o`
- Library: `libcomponent-widget.so`
- Docs: `component.widget.html`
- Tests: `component_widget_test.out`

**Question:** "Is this component fully built?"

### Where Separator-Merge Helps

**Build completeness:**
```bash
recur tree component.widget --sep "." --sep "_" --sep "-" --show-sep

# Output:
component.widget
├── source.cpp [_]         # Source code
├── object.o [.]           # Compiled object
├── library.so [-]         # Linked library
├── docs.html [.]          # Generated docs
└── test.out [_]           # Test binary

# Missing marker = build step incomplete
```

**Build verification script:**
```bash
# Check all components have all artifacts
for component in $(recur files "**" --sep "." | cut -d. -f1-2 | sort -u); do
  artifacts=$(recur files "$component.**" --sep "." --sep "_" --sep "-" --show-sep)

  echo "$artifacts" | grep -q '\[_\]' || echo "Missing source: $component"
  echo "$artifacts" | grep -q '\[.\]' || echo "Missing object: $component"
  echo "$artifacts" | grep -q '\[-\]' || echo "Missing library: $component"
done
```

**New capability:** Track build pipeline completeness across artifact naming conventions.

---

## The Meta-Pattern: Completeness as First-Class Concept

### What's Actually Happening

Traditional tools ask: **"Does this file exist?"**

Separator-merge asks: **"Is this entity complete across all its representations?"**

### The Transformation

**Before:**
```bash
# Manual checks
ls docs/UserService.md       # Exists?
ls src/user_service.rs       # Exists?
ls tests/UserService.test.js # Exists?
# Mentally correlate: "Are these the same thing?"
```

**After:**
```bash
# Unified completeness query
recur tree UserService --sep "." --sep "_" --show-sep

# Instant answer:
#   ✅ Documented [.]
#   ✅ Implemented [_]
#   ❌ Tested [.]  (missing!)
```

### New Automation Patterns Enabled

**1. Automated Code Review Checklist:**
```bash
# Every PR must pass:
recur files "NewFeature.**" --sep "." --sep "_" --show-sep | \
  awk '/\[_\]/ {code++} /\[.\]/ && /test/ {tests++} /\[.\]/ && /readme/ {docs++}
       END {
         if (code > 0 && tests == 0) {print "❌ Missing tests"; exit 1}
         if (code > 0 && docs == 0) {print "❌ Missing docs"; exit 1}
         print "✅ Complete"
       }'
```

**2. Living Documentation Badge:**
```bash
# Generate coverage badge
coverage=$(recur files "src.**" --sep "_" --show-sep | \
           awk '/\[_\]/ {src++} /\[.\]/ && /readme/ {docs++}
                END {print int(docs/src*100)}')

echo "Documentation: $coverage%" > badge.txt
```

**3. Parallel Team Dashboard:**
```bash
# Show feature implementation status across teams
for feature in $(recur files "feature.**" --sep "." | cut -d. -f1-2 | sort -u); do
  status=$(recur files "$feature.**" --sep "." --sep "_" --show-sep)

  has_spec=$(echo "$status" | grep -q "spec.md \[.\]" && echo "✅" || echo "❌")
  has_code=$(echo "$status" | grep -q "\[_\]" && echo "✅" || echo "❌")
  has_test=$(echo "$status" | grep -q "test \[.\]" && echo "✅" || echo "❌")

  echo "$feature | Spec: $has_spec | Code: $has_code | Test: $has_test"
done
```

---

## Why This Matters for Decomposition/Recomposition

### Traditional Decomposition Problem
When you decompose complex structures:
1. Pieces have different naming conventions
2. Tracking completeness is manual
3. Recomposition requires knowing all pieces
4. Gap detection is manual inspection

### Separator-Merge Solution
1. **Virtual namespace** unifies conventions
2. **Automatic gap detection** through markers
3. **Query-based recomposition** (find all pieces)
4. **Completeness verification** is built-in

### The XSLT/XML/JSON Connection

**Yes, this enables new patterns that weren't possible before:**

1. **Modular XSLT:** Decompose transforms, verify coverage, recompose per-context
2. **Schema composition:** Track fragments across team conventions, verify completeness
3. **Config management:** Decompose by environment+service, verify before deploy
4. **Documentation pipeline:** Living docs with automatic staleness detection

**The key insight:**
> Separator-merge makes **"completeness"** a queryable property rather than a manual checklist.

This transforms decomposition from "hope we got all the pieces" to "here's exactly what's missing."

---

## Conclusion: Emergent Capability

**Individual features:**
- Multi-separator merge: Track files with different naming
- Normalization: Unified display
- Show-sep markers: Domain attribution

**Emergent capability:**
- **Completeness as queryable state**
- **Gap analysis as automatic process**
- **Cross-domain entity tracking**

**This enables:**
- Confident decomposition (can always verify completeness)
- Reliable recomposition (find all pieces automatically)
- Parallel team coordination (see what's done/missing)
- Automated quality gates (CI/CD completeness checks)

**For structured data (XML/JSON/XSLT):**
> Separator-merge provides the "glue" to decompose complex structures across different naming domains while maintaining verifiable completeness.

This wasn't possible before because there was no way to track "is this logical entity complete across all its physical representations?"

Now there is.

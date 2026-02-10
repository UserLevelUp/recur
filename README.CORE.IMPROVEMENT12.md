# Core Improvement 12: Document Transformation via Filesystem

**Status:** Future Vision
**Priority:** High (Revolutionary)  
**Category:** Document Processing, Unix Philosophy

## Vision

Enable XSLT-style transformations using the filesystem as the transformation medium. Decompose hierarchical documents (XML, JSON, YAML) into flat files with dot notation, transform using Unix tools, and recompose back to structured format.

**Philosophy:** "Everything is a file. Everything is composable."

## The Problem XSLT Tries to Solve

Transforming hierarchical documents (XML configs, API schemas, build manifests) is common but painful:

**Current approach (XSLT):**
- Complex syntax, steep learning curve
- Non-composable (can't use grep/sed/awk)
- Hard to debug intermediate states  
- Format-specific (XML needs XSLT, JSON needs jq, YAML needs yq)

## The Solution: Filesystem as Transformation Medium

**Decompose → Transform → Recompose:**

```bash
xml-to-recur config.xml        # Decompose: XML → files
sed -i 's/localhost/prod/' **  # Transform: Unix tools
recur-to-xml config            # Recompose: files → XML
```

**Result:** XSLT-level power with Unix simplicity.

## Core Architecture

### Three Tools

1. **`{format}-to-recur`** - Decompose documents to files
2. **`recur-to-{format}`** - Recompose files to documents  
3. **`recur transform`** - High-level transformation wrapper

### Example Transformation

**Input (config.xml):**
```xml
<config>
  <database>
    <host>localhost</host>
    <port>5432</port>
  </database>
</config>
```

**Decompose:**
```bash
xml-to-recur config.xml -d temp/
# Creates:
# temp/config.database.host.txt = "localhost"
# temp/config.database.port.txt = "5432"
```

**Transform:**
```bash
sed -i 's/localhost/production-db/' temp/config.database.host.txt
```

**Recompose:**
```bash
recur-to-xml temp/config > production-config.xml
```

## Use Cases

### 1. Configuration Management

**Environment Promotion:**
```bash
# Merge base + prod overrides
recur merge \
  <(recur tree config -d base/ --json) \
  <(recur tree config -d prod/ --json) \
  --base config | recur-to-xml
```

### 2. API Schema Validation  

**Detect Breaking Changes:**
```bash
openapi-to-recur v1.yaml -d v1/
openapi-to-recur v2.yaml -d v2/

recur merge <(recur tree api -d v1/ --json) \
            <(recur tree api -d v2/ --json) \
  --base api --show-sep | \
grep "\[v1\]" | grep -v "\[v2\]"  # Removed endpoints = breaking!
```

### 3. Bulk Updates

```bash
# Update all timeout values to 60
recur files "**.timeout" | xargs sh -c 'echo "60" > "$1"' _
```

## Benefits Over XSLT

| Feature | XSLT | Recur Transform |
|---------|------|-----------------|
| Learning curve | Steep | Shallow (Unix tools) |
| Composability | No | Yes (pipes!) |
| Debuggability | Hard | Easy (files visible) |
| Format support | XML only | XML, JSON, YAML, any |
| Version control | Opaque diffs | Line-by-line diffs |

## Implementation Phases

**Phase 1:** XML support (`xml-to-recur`, `recur-to-xml`)  
**Phase 2:** JSON support  
**Phase 3:** YAML support  
**Phase 4:** Transform wrapper (`recur transform`)  
**Phase 5:** Advanced features (streaming, validation)

## Technical Challenges

### Attribute Handling
```xml
<server host="localhost"/>
```
Store as: `server.@attr.host.txt = "localhost"`

### Array Representation
```json
{"items": ["a", "b"]}
```
Store as: `items.0.txt = "a"`, `items.1.txt = "b"`

### Type Preservation
Store metadata: `value.@meta.type.txt = "integer"`

## Success Criteria

- ✅ Lossless round-trip (decompose → compose == original)
- ✅ Works with real configs (K8s, Terraform, Maven)
- ✅ Faster than XSLT for common transforms
- ✅ Fully composable with Unix tools

## Why This Matters

> "XSLT is painful. XML is powerful. Unix is composable."

This enables:
1. Transform documents using **grep/sed/awk** instead of XSLT  
2. **Format-agnostic** transformations
3. **Debuggable** intermediate states (files!)
4. **Git-friendly** line-by-line diffs
5. **Composable** with any Unix tool

**Dennis Ritchie's philosophy applied to documents:** Everything is a file.

## See Also

- README.CORE.IMPROVEMENT13.md - Configuration management patterns
- docs/main.command.merge.readme.md - Merge command

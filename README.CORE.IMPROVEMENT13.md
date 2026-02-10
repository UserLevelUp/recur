# Core Improvement 13: Advanced Configuration Management Patterns

**Status:** Future Vision
**Priority:** High (Enterprise-Critical)
**Category:** DevOps, Configuration Management, Unix Philosophy

## Vision

Enable enterprise-grade configuration management using recur's hierarchical merge capabilities. Solve real DevOps problems: environment parity, secret management, feature flags, and configuration drift detection.

**Philosophy:** "Configuration is code. Code is files. Files are composable."

## The Problems We Solve

### 1. Environment Promotion

**Problem:** Dev/staging/prod configs differ in subtle ways. Manual copying causes errors.

**Solution:** Layered configuration merging

```bash
# Base config + environment overrides
recur merge \
  <(recur tree config -d base/ --sep . --json) \
  <(recur tree config -d prod/ --sep . --json) \
  --base config --sep . --sep . | recur-to-xml > prod-config.xml
```

**Result:** Explicit, version-controlled, auditable config promotion.

### 2. Configuration Drift Detection

**Problem:** Did someone manually edit production configs? Which settings diverged?

**Solution:** Compare deployed vs expected configuration

```bash
# Extract deployed config
ssh prod "cat /etc/app/config.xml" | xml-to-recur - -d deployed/

# Compare to expected
recur merge \
  <(recur tree config -d expected/ --sep . --json) \
  <(recur tree config -d deployed/ --sep . --json) \
  --base config --show-sep | grep "\[expected\]" | grep -v "\[deployed\]"

# Output: Settings in expected but NOT in deployed = drift!
```

### 3. Secret Redaction for Version Control

**Problem:** Can't commit configs with secrets. Need sanitized versions for Git.

**Solution:** Selective redaction with provenance tracking

```bash
# Merge real config + redacted overrides
recur tree config -d live/ --sep . > live.json
recur tree config -d redacted/ --sep . > redacted.json

recur merge live.json redacted.json \
  --base config --sep . --sep . \
  --show-sep | grep "\[redacted\]"

# Redacted values override real secrets for Git commit
```

### 4. Feature Flag Management

**Problem:** Feature flags scattered across multiple config files. Hard to audit.

**Solution:** Extract all feature flags from hierarchical configs

```bash
# Find all feature flag settings
recur files "config.**.features.**" -d prod/ | \
  xargs -I {} sh -c 'echo "{}: $(cat {})"'

# Output:
# config.api.features.new_auth.txt: true
# config.ui.features.dark_mode.txt: false
# config.billing.features.crypto_payments.txt: true
```

### 5. Multi-Tenant Configuration

**Problem:** 100 tenants, each with custom overrides. Manage without chaos.

**Solution:** Base + per-tenant overlays

```bash
for tenant in tenant_*/; do
  recur merge \
    <(recur tree config -d base/ --sep . --json) \
    <(recur tree config -d "$tenant" --sep . --json) \
    --base config --sep . --sep . | \
    recur-to-json > "compiled/${tenant}config.json"
done
```

## Use Cases by Industry

### DevOps Teams

**Configuration Validation:**
```bash
# Ensure all required settings present in prod
required_keys="database.host database.port api.key"
for key in $required_keys; do
  recur files "config.$key" -d prod/ || echo "MISSING: $key"
done
```

**Change Impact Analysis:**
```bash
# What changed between releases?
recur merge \
  <(recur tree config -d v1.2.3/ --sep . --json) \
  <(recur tree config -d v1.2.4/ --sep . --json) \
  --base config --show-sep | grep -v "\[v1.2.3\].*\[v1.2.4\]"
```

### Security Teams

**Credential Rotation Audit:**
```bash
# Find all credential references
recur files "**.password" "**.api_key" "**.secret" -d configs/ | \
  xargs -I {} sh -c 'stat -c "%y %n" {}'

# Output: last-modified timestamps for all secrets
```

**Compliance Checking:**
```bash
# Ensure PCI-compliant timeout settings
recur files "**.timeout" -d prod/ | \
  xargs grep -L "^900$" | \
  xargs -I {} echo "NON-COMPLIANT: {} (timeout must be 900s)"
```

### Platform Engineering

**Infrastructure as Code (IaC):**
```bash
# Merge Terraform configs per environment
recur merge \
  <(recur tree terraform -d modules/ --sep . --json) \
  <(recur tree terraform -d env/staging/ --sep . --json) \
  --base terraform --sep . --sep . > staging-plan.json
```

**Service Mesh Configuration:**
```bash
# Combine K8s manifests + Istio policies + app configs
recur merge \
  <(recur tree app -d k8s/ --sep . --json) \
  <(recur tree app -d istio/ --sep - --json) \
  <(recur tree app -d configs/ --sep . --json) \
  --base app --sep . --sep - --sep . --show-sep
```

## Real-World Workflow Example

### Scenario: Deploy New Microservice to Production

**Step 1: Define Base Configuration**
```bash
mkdir -p configs/base/payment.service/
echo "localhost" > configs/base/payment.service.database.host.txt
echo "5432" > configs/base/payment.service.database.port.txt
echo "INFO" > configs/base/payment.service.logging.level.txt
```

**Step 2: Define Production Overrides**
```bash
mkdir -p configs/prod/payment.service/
echo "prod-db.internal" > configs/prod/payment.service.database.host.txt
echo "WARN" > configs/prod/payment.service.logging.level.txt
```

**Step 3: Generate Production Config**
```bash
recur merge \
  <(recur tree payment.service -d configs/base/ --sep . --json) \
  <(recur tree payment.service -d configs/prod/ --sep . --json) \
  --base payment.service --sep . --sep . | \
  recur-to-yaml > deploy/payment-service-prod.yaml
```

**Step 4: Validate Before Deploy**
```bash
# Check all required keys present
required="database.host database.port logging.level"
for key in $required; do
  recur files "payment.service.$key" -d deploy/ || echo "ERROR: Missing $key"
done
```

**Step 5: Deploy and Verify**
```bash
kubectl apply -f deploy/payment-service-prod.yaml

# Compare deployed vs expected
kubectl get configmap payment-service -o yaml | \
  yaml-to-recur - -d deployed/
recur merge \
  <(recur tree payment.service -d deploy/ --sep . --json) \
  <(recur tree payment.service -d deployed/ --sep . --json) \
  --base payment.service --show-sep | \
  grep -v "\[deploy\].*\[deployed\]" || echo "✓ Config matches"
```

## Benefits Over Traditional Tools

| Feature | Traditional (Ansible/Chef/Puppet) | Recur Patterns |
|---------|-----------------------------------|----------------|
| Learning curve | Steep (DSL) | Shallow (Unix tools) |
| Composability | Limited | Full (pipes!) |
| Version control | Opaque diffs | Line-by-line diffs |
| Debuggability | Hard (state machines) | Easy (files visible) |
| Drift detection | Requires agents | Stateless comparison |
| Secret handling | Plugin-dependent | Standard Unix tools |
| Format support | Tool-specific | Any (XML, JSON, YAML) |

## Implementation Phases

**Phase 1:** Environment merging (base + overrides)
**Phase 2:** Drift detection tooling
**Phase 3:** Secret redaction patterns
**Phase 4:** Feature flag extraction
**Phase 5:** Multi-tenant management
**Phase 6:** CI/CD integration examples

## Technical Patterns

### Pattern 1: Three-Layer Configuration

```bash
# Corporate defaults + team standards + app-specific
recur merge \
  <(recur tree app -d corporate/ --sep . --json) \
  <(recur tree app -d team/ --sep . --json) \
  <(recur tree app -d project/ --sep . --json) \
  --base app --sep . --sep . --sep .
```

**Priority:** project > team > corporate (last wins)

### Pattern 2: Environment Parity Check

```bash
# Ensure dev/staging/prod have same structure
for env in dev staging prod; do
  recur tree app -d "configs/$env/" --sep . --json | \
    jq -r '.files[] | .path' | sort > "$env-keys.txt"
done

diff dev-keys.txt prod-keys.txt || echo "⚠️  Structure mismatch!"
```

### Pattern 3: Batch Updates

```bash
# Update all timeout values to 60 seconds
recur files "**.timeout" -d configs/ | \
  xargs -I {} sh -c 'echo "60" > {}'

# Commit atomically
git add configs/ && git commit -m "Standardize timeouts to 60s"
```

### Pattern 4: Config Templates

```bash
# Generate config from template + values
recur merge \
  <(recur tree app -d templates/ --sep . --json) \
  <(recur tree app -d values/customer-acme/ --sep . --json) \
  --base app --sep . --sep . | \
  recur-to-json > acme-config.json
```

## Integration with Existing Tools

### Ansible

```yaml
# playbook.yml
- name: Generate config from recur
  shell: |
    recur merge \
      <(recur tree config -d base/ --sep . --json) \
      <(recur tree config -d {{ env }}/ --sep . --json) \
      --base config | recur-to-xml > /tmp/app-config.xml

- name: Deploy config
  copy:
    src: /tmp/app-config.xml
    dest: /etc/app/config.xml
```

### Terraform

```hcl
# Generate tfvars from recur
data "external" "config" {
  program = ["bash", "-c", <<-EOT
    recur tree terraform -d configs/$ENV/ --sep . --json | \
    jq '{result: .files | map({(.path): .content}) | add}'
  EOT
  ]
}
```

### Docker Compose

```yaml
# docker-compose.yml
services:
  app:
    environment:
      - CONFIG_JSON=$(bash -c "recur merge \
          <(recur tree app -d base/ --sep . --json) \
          <(recur tree app -d docker/ --sep . --json) \
          --base app | recur-to-json")
```

## Success Criteria

- ✅ Zero-trust config promotion (base + overrides)
- ✅ Drift detection without agents
- ✅ Git-friendly line-by-line config diffs
- ✅ Secret redaction for version control
- ✅ Multi-environment parity validation
- ✅ Feature flag audit trail
- ✅ Works with existing tools (Ansible, Terraform, K8s)

## Why This Matters

> "Configuration management is 80% of DevOps pain. Files are 100% of Unix power."

This enables:
1. **Explicit configuration** - No hidden state or magic
2. **Auditable changes** - Every change is a file commit
3. **Composable workflows** - Pipe through any Unix tool
4. **Format-agnostic** - XML, JSON, YAML, TOML, INI, etc.
5. **Zero dependencies** - Works anywhere Unix works
6. **Stateless** - No agents, no daemons, no state files

**DevOps meets Unix Philosophy:** Everything is a file. Everything is composable.

## See Also

- README.CORE.IMPROVEMENT12.md - Document transformation via filesystem
- docs/main.command.merge.readme.md - Merge command documentation
- docs/main.command.merge.phase5.complete.md - Stdin mode implementation

## Quotes

> "The best config management tool is the one you don't need to learn."
> – Every SRE who's fought with Ansible

> "If you can't version control it, you can't trust it."
> – Every DevOps team after an outage

> "Diff shows intent. Merge shows reality."
> – The Unix Way

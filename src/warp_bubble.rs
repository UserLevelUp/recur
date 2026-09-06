//! Versioned data contracts shared by pure `recur warp` queries and the
//! write-side `recur-warp` companion.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub const BUBBLE_MAP_SCHEMA: &str = "warp-bubble-map-v1";
pub const WARP_RING_MAP_SCHEMA: &str = "warp-ring-map-v1";
pub const SLICE_LAYER_SCHEMA: &str = "warp-slice-layer-v1";
pub const MAP_VIEW_SCHEMA: &str = "warp-bubble-map-view-v1";
pub const MERGE_SCHEMA: &str = "warp-bubble-projection-v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WarpRequiredSlice {
    pub slice_id: String,
    pub contract_hash: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub evidence_gates: Vec<String>,
    /// Legacy maps use declared coverage; checked requires validated external artifacts.
    #[serde(default = "default_evidence_mode")]
    pub evidence_mode: String,
    #[serde(default)]
    pub gate_rules: BTreeMap<String, crate::warp_evidence::GateRule>,
}

fn default_evidence_mode() -> String {
    "declared".into()
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarpBubbleMap {
    pub schema: String,
    pub warp_id: String,
    pub required_slices: Vec<WarpRequiredSlice>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct WarpRingMap {
    pub schema: String,
    pub warp_id: String,
    pub coordinator_domain: String,
    pub projection_depth: usize,
    pub domains: Vec<WarpRingDomain>,
    #[serde(default)]
    pub subscriptions: Vec<WarpRingSubscription>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct WarpRingDomain {
    pub domain_id: String,
    pub relative_root: String,
    pub role: String,
    pub warp_id: String,
    #[serde(default)]
    pub public_contract_hash: Option<String>,
    #[serde(default)]
    pub required_state: Option<String>,
    #[serde(default)]
    pub parent_acceptance: Option<WarpRingParentAcceptance>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct WarpRingParentAcceptance {
    pub slice_id: String,
    pub contract_hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct WarpRingSubscription {
    pub subscription_id: String,
    pub direction: String,
    pub source_domain: String,
    pub target_domain: String,
    pub filter: String,
    pub event_contract: String,
    pub freshness_seconds: u64,
}

pub fn validate_warp_ring_map(map: &WarpRingMap, expected_warp_id: &str) -> Result<(), String> {
    if map.schema != WARP_RING_MAP_SCHEMA {
        return Err(format!(
            "unsupported Warp ring map schema '{}'; expected '{}'",
            map.schema, WARP_RING_MAP_SCHEMA
        ));
    }
    if map.warp_id.trim().is_empty() || map.warp_id != expected_warp_id {
        return Err(format!(
            "Warp ring identity '{}' does not match requested '{}'",
            map.warp_id, expected_warp_id
        ));
    }
    if map.coordinator_domain.trim().is_empty() {
        return Err("Warp ring coordinator domain must not be blank".to_string());
    }
    if map.projection_depth == 0 {
        return Err("Warp ring projection depth must be greater than zero".to_string());
    }

    let mut domain_ids = std::collections::BTreeSet::new();
    for domain in &map.domains {
        if domain.domain_id.trim().is_empty()
            || domain.relative_root.trim().is_empty()
            || domain.role.trim().is_empty()
            || domain.warp_id.trim().is_empty()
        {
            return Err(
                "Warp ring domains require identity, root, role, and Warp identity".to_string(),
            );
        }
        if !domain_ids.insert(domain.domain_id.as_str()) {
            return Err(format!(
                "Warp ring contains duplicate domain '{}'",
                domain.domain_id
            ));
        }
        if domain
            .public_contract_hash
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
            || domain
                .required_state
                .as_deref()
                .is_some_and(|value| value.trim().is_empty())
        {
            return Err(format!(
                "Warp ring domain '{}' contains a blank public contract or required state",
                domain.domain_id
            ));
        }
        if let Some(acceptance) = &domain.parent_acceptance {
            if acceptance.slice_id.trim().is_empty() || acceptance.contract_hash.trim().is_empty() {
                return Err(format!(
                    "Warp ring domain '{}' contains a blank parent acceptance",
                    domain.domain_id
                ));
            }
        }
    }

    let coordinator = map
        .domains
        .iter()
        .find(|domain| domain.domain_id == map.coordinator_domain)
        .ok_or_else(|| {
            format!(
                "Warp ring coordinator domain '{}' is not declared",
                map.coordinator_domain
            )
        })?;
    if coordinator.role != "coordinator" {
        return Err(format!(
            "Warp ring coordinator domain '{}' must have role 'coordinator'",
            map.coordinator_domain
        ));
    }

    let mut subscription_ids = std::collections::BTreeSet::new();
    for subscription in &map.subscriptions {
        if subscription.subscription_id.trim().is_empty()
            || subscription.filter.trim().is_empty()
            || subscription.event_contract.trim().is_empty()
        {
            return Err(
                "Warp ring subscriptions require identity, filter, and event contract".to_string(),
            );
        }
        if !subscription_ids.insert(subscription.subscription_id.as_str()) {
            return Err(format!(
                "Warp ring contains duplicate subscription '{}'",
                subscription.subscription_id
            ));
        }
        if !matches!(
            subscription.direction.as_str(),
            "parent-to-child" | "child-to-parent"
        ) {
            return Err(format!(
                "Warp ring subscription '{}' has unsupported direction '{}'",
                subscription.subscription_id, subscription.direction
            ));
        }
        if !domain_ids.contains(subscription.source_domain.as_str())
            || !domain_ids.contains(subscription.target_domain.as_str())
        {
            return Err(format!(
                "Warp ring subscription '{}' references an unknown domain",
                subscription.subscription_id
            ));
        }
        if subscription.source_domain == subscription.target_domain {
            return Err(format!(
                "Warp ring subscription '{}' must cross domain boundaries",
                subscription.subscription_id
            ));
        }
        if subscription.freshness_seconds == 0 {
            return Err(format!(
                "Warp ring subscription '{}' freshness must be greater than zero",
                subscription.subscription_id
            ));
        }
    }

    Ok(())
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct WarpSliceLayer {
    pub schema: String,
    pub warp_id: String,
    pub slice_id: String,
    pub contract_hash: String,
    pub attempt_id: String,
    pub result_state: String,
    pub result_hash: String,
    #[serde(default)]
    pub evidence: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub reason: Option<String>,
}

pub fn validate_bubble_map(map: &WarpBubbleMap, warp: &str, path: &Path) -> anyhow::Result<()> {
    if map.required_slices.is_empty() {
        anyhow::bail!("Warp bubble map must declare at least one required Slice");
    }
    if map.schema != BUBBLE_MAP_SCHEMA {
        anyhow::bail!(
            "unsupported Warp bubble map schema '{}' in '{}'; expected '{}'",
            map.schema,
            path.display(),
            BUBBLE_MAP_SCHEMA
        );
    }
    if map.warp_id != warp {
        anyhow::bail!(
            "Warp identity '{}' in '{}' does not match requested '{}'",
            map.warp_id,
            path.display(),
            warp
        );
    }
    let mut ids = BTreeSet::new();
    for required in &map.required_slices {
        if required.evidence_mode == "checked" && required.evidence_gates.is_empty() {
            anyhow::bail!(
                "checked Slice '{}' must declare at least one evidence gate",
                required.slice_id
            );
        }
        if !matches!(required.evidence_mode.as_str(), "declared" | "checked") {
            anyhow::bail!(
                "Slice '{}' has unsupported evidence_mode '{}'",
                required.slice_id,
                required.evidence_mode
            );
        }
        for (gate, rule) in &required.gate_rules {
            if !required.evidence_gates.contains(gate)
                || !matches!(rule.kind.as_str(), "" | "test" | "build" | "scan")
            {
                anyhow::bail!(
                    "Slice '{}' has invalid gate rule '{}'",
                    required.slice_id,
                    gate
                );
            }
        }
        if required.slice_id.trim().is_empty() || required.contract_hash.trim().is_empty() {
            anyhow::bail!(
                "Warp bubble map '{}' contains a blank Slice identity or contract hash",
                path.display()
            );
        }
        if !ids.insert(required.slice_id.clone()) {
            anyhow::bail!(
                "Warp bubble map '{}' contains duplicate Slice '{}'",
                path.display(),
                required.slice_id
            );
        }
        let gates = required
            .evidence_gates
            .iter()
            .map(|gate| gate.trim())
            .collect::<BTreeSet<_>>();
        if gates.len() != required.evidence_gates.len() || gates.contains("") {
            anyhow::bail!(
                "Warp Slice '{}' in '{}' has blank or duplicate evidence gates",
                required.slice_id,
                path.display()
            );
        }
    }
    for required in &map.required_slices {
        let dependencies = required.depends_on.iter().collect::<BTreeSet<_>>();
        if dependencies.len() != required.depends_on.len() {
            anyhow::bail!(
                "Warp Slice '{}' in '{}' has duplicate dependencies",
                required.slice_id,
                path.display()
            );
        }
        for dependency in &required.depends_on {
            if dependency == &required.slice_id || !ids.contains(dependency) {
                anyhow::bail!(
                    "Warp Slice '{}' in '{}' has invalid dependency '{}'",
                    required.slice_id,
                    path.display(),
                    dependency
                );
            }
        }
    }
    let mut remaining = map
        .required_slices
        .iter()
        .map(|required| {
            (
                required.slice_id.clone(),
                required.depends_on.iter().cloned().collect::<BTreeSet<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    while !remaining.is_empty() {
        let ready = remaining
            .iter()
            .filter(|(_, dependencies)| dependencies.is_empty())
            .map(|(slice, _)| slice.clone())
            .collect::<Vec<_>>();
        if ready.is_empty() {
            anyhow::bail!(
                "Warp bubble map '{}' contains a dependency cycle involving {}",
                path.display(),
                remaining.keys().cloned().collect::<Vec<_>>().join(", ")
            );
        }
        for slice in &ready {
            remaining.remove(slice);
        }
        for dependencies in remaining.values_mut() {
            for slice in &ready {
                dependencies.remove(slice);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_maps_require_nonempty_work_and_gates() {
        let mut map: WarpBubbleMap = serde_json::from_str(
            r#"{"schema":"warp-bubble-map-v1","warp_id":"demo","required_slices":[]}"#,
        )
        .unwrap();
        assert!(validate_bubble_map(&map, "demo", Path::new("demo.warp-map.json")).is_err());
        map.required_slices.push(
            serde_json::from_str(
                r#"{"slice_id":"test","contract_hash":"contract:v1","evidence_mode":"checked"}"#,
            )
            .unwrap(),
        );
        assert!(validate_bubble_map(&map, "demo", Path::new("demo.warp-map.json")).is_err());
        map.required_slices[0].evidence_gates.push("tests".into());
        assert!(validate_bubble_map(&map, "demo", Path::new("demo.warp-map.json")).is_ok());
    }

    fn complete_ring() -> WarpRingMap {
        serde_json::from_str(include_str!(
            "../julia-tests/fixtures/warp-ring-v1/complete/coordinator.release.warp-ring.json"
        ))
        .expect("complete ring fixture should deserialize")
    }

    #[test]
    fn complete_ring_fixture_deserializes_and_validates() {
        let ring = complete_ring();

        validate_warp_ring_map(&ring, "coordinator.release").unwrap();
        assert_eq!(ring.schema, WARP_RING_MAP_SCHEMA);
        assert_eq!(ring.coordinator_domain, "coordinator");
        assert_eq!(ring.domains.len(), 3);
        assert_eq!(ring.subscriptions.len(), 3);
    }

    #[test]
    fn missing_parent_acceptance_remains_structurally_valid() {
        let ring: WarpRingMap = serde_json::from_str(include_str!(
            "../julia-tests/fixtures/warp-ring-v1/missing-acceptance/coordinator.release.warp-ring.json"
        ))
        .expect("missing-acceptance fixture should deserialize");

        validate_warp_ring_map(&ring, "coordinator.release").unwrap();
        let worker = ring
            .domains
            .iter()
            .find(|domain| domain.domain_id == "docs-monkey")
            .unwrap();
        assert_eq!(worker.required_state.as_deref(), Some("complete"));
        assert!(worker.parent_acceptance.is_none());
    }

    #[test]
    fn ring_validation_rejects_unknown_subscription_domains() {
        let mut ring = complete_ring();
        ring.subscriptions[0].target_domain = "missing-worker".to_string();

        let error = validate_warp_ring_map(&ring, "coordinator.release").unwrap_err();
        assert!(error.contains("unknown domain"));
    }

    #[test]
    fn ring_validation_rejects_zero_freshness_and_projection_depth() {
        let mut ring = complete_ring();
        ring.projection_depth = 0;
        assert!(validate_warp_ring_map(&ring, "coordinator.release")
            .unwrap_err()
            .contains("projection depth"));

        ring.projection_depth = 3;
        ring.subscriptions[0].freshness_seconds = 0;
        assert!(validate_warp_ring_map(&ring, "coordinator.release")
            .unwrap_err()
            .contains("freshness"));
    }
}

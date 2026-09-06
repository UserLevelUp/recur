//! Versioned data contracts shared by pure `recur warp` queries and the
//! write-side `recur-warp` companion.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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

#[cfg(test)]
mod tests {
    use super::*;

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

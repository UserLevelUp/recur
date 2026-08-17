//! Versioned data contracts shared by pure `recur warp` queries and the
//! write-side `recur-warp` companion.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const BUBBLE_MAP_SCHEMA: &str = "warp-bubble-map-v1";
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

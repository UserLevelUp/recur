//! SGR1: deterministic, read-only analysis of the already parsed CIR1 boundary.
use crate::recur_lang_concurrent_ir::{ConcurrentIr, MessageRefIr, CONCURRENT_IR_SCHEMA};
use crate::recur_lang_ir::SourceSpan;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub const GRAPH_SCHEMA: &str = "recur-lang-static-graph-report-v1";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Node {
    pub identity: String,
    pub kind: String,
    pub span: SourceSpan,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Edge {
    pub producer: String,
    pub consumer: String,
    pub message: MessageRefIr,
    pub span: SourceSpan,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Wait {
    pub identity: String,
    pub required: Vec<MessageRefIr>,
    pub consumer: String,
    pub span: SourceSpan,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Finding {
    pub code: String,
    pub message: String,
    pub subjects: Vec<String>,
    pub path: Vec<String>,
    pub span: Option<SourceSpan>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GraphReport {
    pub schema: &'static str,
    pub ir_schema: String,
    pub source: String,
    pub source_hash: String,
    pub flow: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub waits: Vec<Wait>,
    pub entries: Vec<String>,
    pub reachable: Vec<String>,
    pub findings: Vec<Finding>,
    pub orchestration_sound: bool,
}

type Adjacency = BTreeMap<String, BTreeSet<String>>;

fn finding(
    code: &str,
    message: String,
    subjects: Vec<String>,
    span: Option<SourceSpan>,
) -> Finding {
    Finding {
        code: code.into(),
        message,
        subjects,
        path: Vec::new(),
        span,
    }
}

fn connect(graph: &mut Adjacency, from: &str, to: &str) {
    graph.entry(from.into()).or_default().insert(to.into());
}

/// A shortest cycle through each participating node, canonically rotated and deduplicated.
/// Breadth-first traversal avoids recursion and preserves deterministic tie breaking.
fn cycles(graph: &Adjacency) -> Vec<Vec<String>> {
    let mut result = BTreeSet::new();
    for start in graph.keys() {
        let mut queue = VecDeque::from([vec![start.clone()]]);
        let mut visited = BTreeSet::from([start.clone()]);
        'search: while let Some(path) = queue.pop_front() {
            if let Some(next) = graph.get(path.last().unwrap()) {
                for node in next {
                    if node == start {
                        let mut cycle = path.clone();
                        let first = cycle.iter().enumerate().min_by_key(|(_, n)| *n).unwrap().0;
                        cycle.rotate_left(first);
                        cycle.push(cycle[0].clone());
                        result.insert(cycle);
                        break 'search;
                    }
                    if visited.insert(node.clone()) {
                        let mut extended = path.clone();
                        extended.push(node.clone());
                        queue.push_back(extended);
                    }
                }
            }
        }
    }
    result.into_iter().collect()
}

/// Analyze CIR1 without parsing source, scheduling lanes or advancing Eventness.
pub fn analyze(ir: &ConcurrentIr) -> GraphReport {
    let mut report = GraphReport {
        schema: GRAPH_SCHEMA,
        ir_schema: ir.schema.into(),
        source: ir.source.clone(),
        source_hash: ir.source_hash.clone(),
        flow: ir.flow.name.clone(),
        nodes: Vec::new(),
        edges: Vec::new(),
        waits: Vec::new(),
        entries: ir.flow.fork_lanes.clone(),
        reachable: Vec::new(),
        findings: Vec::new(),
        orchestration_sound: false,
    };
    if ir.schema != CONCURRENT_IR_SCHEMA {
        report.findings.push(finding(
            "SGR005",
            "Unsupported IR schema".into(),
            vec![],
            None,
        ));
        return report;
    }
    for lane in &ir.lanes {
        report.nodes.push(Node {
            identity: lane.name.clone(),
            kind: "lane".into(),
            span: lane.span.clone(),
        });
    }
    for port in &ir.coordinator_ports {
        let name = port
            .identity
            .rsplit_once(".o(")
            .map(|(n, _)| n)
            .unwrap_or(&port.identity);
        if !report.nodes.iter().any(|n| n.identity == name) {
            report.nodes.push(Node {
                identity: name.into(),
                kind: "coordinator".into(),
                span: port.span.clone(),
            });
        }
    }
    let names: BTreeSet<_> = report.nodes.iter().map(|n| n.identity.clone()).collect();
    let lane_names: BTreeSet<_> = ir.lanes.iter().map(|l| l.name.as_str()).collect();
    if lane_names.len() != ir.lanes.len() || names.len() != report.nodes.len() {
        report.findings.push(finding(
            "SGR005",
            "Duplicate graph node identity".into(),
            vec![],
            Some(ir.flow.span.clone()),
        ));
    }
    for entry in &ir.flow.fork_lanes {
        if !lane_names.contains(entry.as_str()) {
            report.findings.push(finding(
                "SGR005",
                format!("Unknown fork lane {entry}"),
                vec![entry.clone()],
                Some(ir.flow.span.clone()),
            ));
        }
    }
    let first_required: Vec<_> = ir
        .flow
        .awaits
        .first()
        .map(|w| w.required.iter().map(|m| m.producer.clone()).collect())
        .unwrap_or_default();
    if first_required != ir.flow.fork_lanes || first_required.is_empty() {
        report.findings.push(finding(
            "SGR004",
            "First await must preserve every fork lane in authored order".into(),
            ir.flow.fork_lanes.clone(),
            Some(ir.flow.span.clone()),
        ));
    }
    let ports: BTreeMap<_, _> = ir
        .lanes
        .iter()
        .map(|l| {
            (
                l.output_message.identity.clone(),
                l.output_message.contract.clone(),
            )
        })
        .chain(
            ir.coordinator_ports
                .iter()
                .map(|p| (p.identity.clone(), p.contract.clone())),
        )
        .collect();
    let mut dependency = Adjacency::new();
    let mut waiting = Adjacency::new();
    let mut activation = Adjacency::new();
    for lane in &ir.lanes {
        for message in &lane.input_messages {
            report.edges.push(Edge {
                producer: message.producer.clone(),
                consumer: lane.name.clone(),
                message: message.clone(),
                span: lane.span.clone(),
            });
            connect(&mut dependency, &message.producer, &lane.name);
        }
    }
    for (index, wait) in ir.flow.awaits.iter().enumerate() {
        if wait
            .required
            .iter()
            .map(|m| &m.contract)
            .collect::<BTreeSet<_>>()
            .len()
            > 1
        {
            report.findings.push(finding(
                "SGR004",
                format!(
                    "Mixed await contracts before {} are outside CIR1",
                    wait.next_consumer
                ),
                vec![wait.next_consumer.clone()],
                Some(wait.span.clone()),
            ));
        }
        report.waits.push(Wait {
            identity: format!("{}.wait.{}", ir.flow.name, index + 1),
            required: wait.required.clone(),
            consumer: wait.next_consumer.clone(),
            span: wait.span.clone(),
        });
        for message in &wait.required {
            connect(&mut waiting, &wait.next_consumer, &message.producer);
            connect(&mut activation, &message.producer, &wait.next_consumer);
            if !report
                .edges
                .iter()
                .any(|e| e.consumer == wait.next_consumer && e.message == *message)
            {
                report.edges.push(Edge {
                    producer: message.producer.clone(),
                    consumer: wait.next_consumer.clone(),
                    message: message.clone(),
                    span: wait.span.clone(),
                });
                connect(&mut dependency, &message.producer, &wait.next_consumer);
            }
        }
        if wait.required.is_empty() || !names.contains(&wait.next_consumer) {
            report.findings.push(finding(
                "SGR004",
                format!("Unsatisfied wait before {}", wait.next_consumer),
                vec![wait.next_consumer.clone()],
                Some(wait.span.clone()),
            ));
        }
    }
    for edge in &report.edges {
        let projected = ir
            .coordinator_ports
            .iter()
            .find(|p| p.identity == edge.message.identity)
            .and_then(|p| p.projected_contract.as_ref());
        let expected = if edge.message.projection.is_some() {
            projected
        } else {
            ports.get(&edge.message.identity)
        };
        let declared_producer = edge.message.identity.rsplit_once(".o(").map(|(p, _)| p);
        if !names.contains(&edge.producer)
            || !names.contains(&edge.consumer)
            || expected != Some(&edge.message.contract)
            || declared_producer != Some(edge.producer.as_str())
        {
            report.findings.push(finding(
                "SGR005",
                format!(
                    "Invalid typed dependency {} -> {}",
                    edge.message.identity, edge.consumer
                ),
                vec![edge.producer.clone(), edge.consumer.clone()],
                Some(edge.span.clone()),
            ));
        }
    }
    for lane in &ir.lanes {
        let expected: BTreeSet<_> = lane
            .input_messages
            .iter()
            .filter(|m| lane_names.contains(m.producer.as_str()))
            .map(|m| (m.identity.clone(), m.contract.clone(), m.projection.clone()))
            .collect();
        let gates: Vec<_> = ir
            .flow
            .awaits
            .iter()
            .filter(|w| w.next_consumer == lane.name)
            .collect();
        let actual: BTreeSet<_> = gates
            .iter()
            .flat_map(|w| &w.required)
            .map(|m| (m.identity.clone(), m.contract.clone(), m.projection.clone()))
            .collect();
        if expected != actual || gates.len() > 1 {
            report.findings.push(finding(
                "SGR004",
                format!(
                    "Wait ports do not match exact dependencies of {}",
                    lane.name
                ),
                vec![lane.name.clone()],
                Some(lane.span.clone()),
            ));
        }
    }
    for (graph, code, label) in [
        (&dependency, "SGR001", "Dependency cycle"),
        (&waiting, "SGR002", "Wait cycle"),
    ] {
        for path in cycles(graph) {
            let span = report
                .nodes
                .iter()
                .find(|n| n.identity == path[0])
                .map(|n| n.span.clone());
            report.findings.push(Finding {
                code: code.into(),
                message: format!("{label}: {}", path.join(" -> ")),
                subjects: path[..path.len() - 1].to_vec(),
                path,
                span,
            });
        }
    }
    let mut reached = BTreeSet::new();
    let mut queue: VecDeque<_> = report.entries.iter().cloned().collect();
    while let Some(node) = queue.pop_front() {
        if reached.insert(node.clone()) {
            if let Some(next) = activation.get(&node) {
                queue.extend(next.iter().cloned());
            }
        }
    }
    for node in &report.nodes {
        if reached.contains(&node.identity) {
            report.reachable.push(node.identity.clone());
        }
        if node.kind == "lane" && !reached.contains(&node.identity) {
            report.findings.push(finding(
                "SGR003",
                format!(
                    "Lane {} is unreachable from the fork/await flow",
                    node.identity
                ),
                vec![node.identity.clone()],
                Some(node.span.clone()),
            ));
        }
    }
    report.orchestration_sound = report.findings.is_empty();
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recur_lang_concurrent_ir::parse_concurrent_ir;
    fn fixture() -> ConcurrentIr {
        parse_concurrent_ir(
            include_str!("../demos/main.lang/main.lang.skippy-watch-coordination.recur"),
            "skippy.recur",
            "solution",
        )
        .unwrap()
    }
    #[test]
    fn exact_fixture_and_repeatable_json() {
        let ir = fixture();
        let report = analyze(&ir);
        assert!(report.orchestration_sound, "{:?}", report.findings);
        assert_eq!(
            report
                .nodes
                .iter()
                .filter(|n| n.kind == "lane")
                .map(|n| &n.identity)
                .collect::<Vec<_>>(),
            ir.lanes.iter().map(|n| &n.name).collect::<Vec<_>>()
        );
        assert_eq!(report.waits.len(), 3);
        assert_eq!(report.waits[0].required, ir.flow.awaits[0].required);
        assert_eq!(report.source_hash, ir.source_hash);
        assert_eq!(
            serde_json::to_vec(&report).unwrap(),
            serde_json::to_vec(&analyze(&ir)).unwrap()
        );
    }
    #[test]
    fn cycles_include_explanatory_path() {
        let mut ir = fixture();
        let feedback = ir.lanes[4].output_message.clone();
        ir.lanes[0].input_messages.push(feedback);
        let report = analyze(&ir);
        let cycle = report.findings.iter().find(|f| f.code == "SGR001").unwrap();
        assert_eq!(cycle.path.first(), cycle.path.last());
        assert!(cycle.subjects.contains(&"csharp_monkey".into()));
        assert!(cycle.subjects.contains(&"git_monkey".into()));
        ir.flow.awaits[2].next_consumer = "csharp_monkey".into();
        assert!(analyze(&ir).findings.iter().any(|f| f.code == "SGR002"));
    }
    #[test]
    fn missing_join_and_unreachable_lane_are_distinct() {
        let mut ir = fixture();
        ir.flow.awaits[0].required.pop();
        assert!(analyze(&ir).findings.iter().any(|f| f.code == "SGR004"));
        let mut ir = fixture();
        let mut orphan = ir.lanes[0].clone();
        orphan.name = "orphan".into();
        orphan.output_message.identity = "orphan.o(b)".into();
        orphan.output_message.producer = "orphan".into();
        ir.lanes.push(orphan);
        assert!(analyze(&ir)
            .findings
            .iter()
            .any(|f| f.code == "SGR003" && f.subjects == ["orphan"]));
    }
    #[test]
    fn schema_and_exact_contract_mismatch_are_findings() {
        let mut ir = fixture();
        ir.schema = "unknown";
        assert!(!analyze(&ir).orchestration_sound);
        let mut ir = fixture();
        ir.lanes[3].input_messages[1].contract = "ShapeIsNotIdentity".into();
        assert!(analyze(&ir).findings.iter().any(|f| f.code == "SGR005"));
    }

    #[test]
    fn self_cycle_unknown_entry_and_empty_wait_are_not_sound() {
        let mut ir = fixture();
        let self_message = ir.lanes[0].output_message.clone();
        ir.lanes[0].input_messages.push(self_message);
        assert!(analyze(&ir)
            .findings
            .iter()
            .any(|f| f.code == "SGR001" && f.path == ["csharp_monkey", "csharp_monkey"]));
        ir.flow.fork_lanes[0] = "missing".into();
        ir.flow.awaits.clear();
        let report = analyze(&ir);
        assert!(report.findings.iter().any(|f| f.code == "SGR004"));
        assert!(report.findings.iter().any(|f| f.code == "SGR005"));
    }
}

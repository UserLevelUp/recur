//! Read-only concurrent IR for the bounded Recur Lang 0.2 coordination subset.

use crate::recur_lang_ir::{
    find_named_blocks, span_for, stable_source_hash, FieldIr, NamedBlock, SourceSpan,
};
use regex::Regex;
use serde::Serialize;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;

pub const CONCURRENT_IR_SCHEMA: &str = "recur-lang-concurrent-ir-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NamedContractIr {
    pub name: String,
    pub fields: Vec<FieldIr>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CoordinatorPortIr {
    pub identity: String,
    pub contract: String,
    pub projected_contract: Option<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MessageRefIr {
    pub identity: String,
    pub producer: String,
    pub contract: String,
    pub projection: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LaneFunctionIr {
    pub identity: String,
    pub familiar_name: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LanePolicyIr {
    pub allow_read: Vec<String>,
    pub allow_write: Vec<String>,
    pub allow_tools: Vec<String>,
    pub required_receipts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LaneIr {
    pub name: String,
    pub persona: String,
    pub input_symbol: String,
    pub input_expression: String,
    pub input_messages: Vec<MessageRefIr>,
    pub output_symbol: String,
    pub output_message: MessageRefIr,
    pub function: LaneFunctionIr,
    pub policy: LanePolicyIr,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AwaitIr {
    pub required: Vec<MessageRefIr>,
    pub next_consumer: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConcurrentFlowIr {
    pub name: String,
    pub mode: String,
    pub expression: String,
    pub fork_lanes: Vec<String>,
    pub awaits: Vec<AwaitIr>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConcurrentIr {
    pub schema: &'static str,
    pub language_version: String,
    pub coordination_name: String,
    pub source: String,
    pub source_hash: String,
    pub contracts: Vec<NamedContractIr>,
    pub coordinator_ports: Vec<CoordinatorPortIr>,
    pub lanes: Vec<LaneIr>,
    pub flow: ConcurrentFlowIr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConcurrentDiagnostic {
    pub code: &'static str,
    pub message: String,
    pub span: Option<SourceSpan>,
}

impl fmt::Display for ConcurrentDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for ConcurrentDiagnostic {}

pub fn parse_concurrent_ir(
    source: &str,
    source_name: &str,
    flow_name: &str,
) -> Result<ConcurrentIr, ConcurrentDiagnostic> {
    let declaration_pattern =
        Regex::new(r"(?m)^\s*recur\s+([0-9.]+)\s+coordination\s+([A-Za-z][A-Za-z0-9_]*)\s*$")
            .expect("valid coordination declaration regex");
    let declarations: Vec<_> = declaration_pattern.captures_iter(source).collect();
    if declarations.len() != 1 {
        return Err(diagnostic(
            "RCIR001",
            format!(
                "source must declare exactly one 'recur <version> coordination <Name>'; found {}",
                declarations.len()
            ),
            declarations
                .first()
                .and_then(|captures| captures.get(0))
                .map(|matched| span_for(source, matched.start(), matched.end())),
        ));
    }
    let language_version = declarations[0][1].to_string();
    let coordination_name = declarations[0][2].to_string();

    let contract_blocks =
        find_named_blocks(source, "contract", "RCIR002").map_err(convert_diagnostic)?;
    let contracts = parse_named_contracts(&contract_blocks)?;
    let contract_names: HashSet<_> = contracts
        .iter()
        .map(|contract| contract.name.clone())
        .collect();

    let coordinator_blocks =
        find_named_blocks(source, "coordinator", "RCIR003").map_err(convert_diagnostic)?;
    if coordinator_blocks.len() != 1 {
        return Err(diagnostic(
            "RCIR003",
            format!(
                "source must declare exactly one coordinator; found {}",
                coordinator_blocks.len()
            ),
            coordinator_blocks.first().map(|block| block.span.clone()),
        ));
    }
    let coordinator = &coordinator_blocks[0];
    let all_scope_blocks =
        find_named_blocks(source, "scope", "RCIR003").map_err(convert_diagnostic)?;
    let coordinator_scopes: Vec<_> = all_scope_blocks
        .iter()
        .filter(|block| span_is_within(&block.span, &coordinator.span))
        .collect();
    let coordinator_ports =
        parse_coordinator_ports(source, coordinator, &coordinator_scopes, &contract_names)?;

    let lane_blocks = find_named_blocks(source, "lane", "RCIR004").map_err(convert_diagnostic)?;
    if lane_blocks.is_empty() {
        return Err(diagnostic(
            "RCIR004",
            "coordination source must declare at least one lane".to_string(),
            None,
        ));
    }
    ensure_unique_block_names(&lane_blocks, "lane", "RCIR004")?;

    let lane_outputs = parse_lane_outputs(source, &lane_blocks, &contract_names)?;
    let mut port_lookup: HashMap<String, PortInfo> = coordinator_ports
        .iter()
        .map(|port| {
            (
                port.identity.clone(),
                PortInfo {
                    producer: producer_from_identity(&port.identity),
                    contract: port.contract.clone(),
                    projected_contract: port.projected_contract.clone(),
                },
            )
        })
        .collect();
    for output in lane_outputs.values() {
        port_lookup.insert(
            output.identity.clone(),
            PortInfo {
                producer: output.producer.clone(),
                contract: output.contract.clone(),
                projected_contract: None,
            },
        );
    }

    let lanes = lane_blocks
        .iter()
        .map(|block| parse_lane(source, block, &lane_outputs, &port_lookup))
        .collect::<Result<Vec<_>, _>>()?;
    let flow_source = parse_flow_source(source, flow_name)?;
    let flow = parse_flow(source, &flow_source, &port_lookup)?;
    validate_concurrent_graph(&flow, &lanes, &coordinator_ports, &flow_source.span)?;

    Ok(ConcurrentIr {
        schema: CONCURRENT_IR_SCHEMA,
        language_version,
        coordination_name,
        source: source_name.to_string(),
        source_hash: stable_source_hash(source.as_bytes()),
        contracts,
        coordinator_ports,
        lanes,
        flow,
    })
}

#[derive(Debug, Clone)]
struct PortInfo {
    producer: String,
    contract: String,
    projected_contract: Option<String>,
}

#[derive(Debug)]
struct FlowSource<'a> {
    name: String,
    mode: String,
    raw: &'a str,
    expression: String,
    start_byte: usize,
    span: SourceSpan,
}

fn parse_named_contracts(
    blocks: &[NamedBlock<'_>],
) -> Result<Vec<NamedContractIr>, ConcurrentDiagnostic> {
    ensure_unique_block_names(blocks, "contract", "RCIR002")?;
    let field_pattern = Regex::new(r"^([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([A-Za-z][A-Za-z0-9_<>?]*)$")
        .expect("valid coordination field regex");
    blocks
        .iter()
        .map(|block| {
            let mut fields = Vec::new();
            for line in block.body.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let captures = field_pattern.captures(line).ok_or_else(|| {
                    diagnostic(
                        "RCIR002",
                        format!(
                            "contract '{}' contains invalid field declaration '{line}'",
                            block.name
                        ),
                        Some(block.span.clone()),
                    )
                })?;
                fields.push(FieldIr {
                    name: captures[1].to_string(),
                    type_name: captures[2].to_string(),
                });
            }
            if fields.is_empty() {
                return Err(diagnostic(
                    "RCIR002",
                    format!("contract '{}' cannot be empty", block.name),
                    Some(block.span.clone()),
                ));
            }
            Ok(NamedContractIr {
                name: block.name.clone(),
                fields,
                span: block.span.clone(),
            })
        })
        .collect()
}

fn parse_coordinator_ports(
    source: &str,
    coordinator: &NamedBlock<'_>,
    scope_blocks: &[&NamedBlock<'_>],
    contract_names: &HashSet<String>,
) -> Result<Vec<CoordinatorPortIr>, ConcurrentDiagnostic> {
    let output_pattern = Regex::new(r"(?m)^\s*o\(([a-z])\)\s*:=\s*([A-Za-z][A-Za-z0-9_<>?]*)\s*$")
        .expect("valid coordinator output regex");
    let mut ports = Vec::new();
    for scope in scope_blocks {
        let outputs: Vec<_> = output_pattern.captures_iter(scope.body).collect();
        if outputs.len() != 1 {
            return Err(diagnostic(
                "RCIR003",
                format!(
                    "coordinator scope '{}.{}' must declare exactly one output; found {}",
                    coordinator.name,
                    scope.name,
                    outputs.len()
                ),
                Some(scope.span.clone()),
            ));
        }
        let matched = outputs[0].get(0).expect("capture zero exists");
        let contract = outputs[0][2].to_string();
        let projected_contract = projected_contract_name(&contract);
        let named_contract = projected_contract.as_ref().unwrap_or(&contract);
        if !contract_names.contains(named_contract) {
            return Err(diagnostic(
                "RCIR003",
                format!(
                    "coordinator port '{}.{}.o({})' references unknown contract '{}'",
                    coordinator.name, scope.name, &outputs[0][1], named_contract
                ),
                Some(scope.child_span(source, matched.start(), matched.end())),
            ));
        }
        ports.push(CoordinatorPortIr {
            identity: format!("{}.{}.o({})", coordinator.name, scope.name, &outputs[0][1]),
            contract,
            projected_contract,
            span: scope.child_span(source, matched.start(), matched.end()),
        });
    }
    Ok(ports)
}

fn parse_lane_outputs(
    source: &str,
    blocks: &[NamedBlock<'_>],
    contract_names: &HashSet<String>,
) -> Result<HashMap<String, MessageRefIr>, ConcurrentDiagnostic> {
    let output_pattern = Regex::new(r"(?m)^\s*o\(([a-z])\)\s*:=\s*([A-Za-z][A-Za-z0-9_]*)\s*$")
        .expect("valid lane output regex");
    let mut outputs = HashMap::new();
    for block in blocks {
        let declarations: Vec<_> = output_pattern.captures_iter(block.body).collect();
        if declarations.len() != 1 {
            return Err(diagnostic(
                "RCIR005",
                format!(
                    "lane '{}' must declare exactly one output message; found {}",
                    block.name,
                    declarations.len()
                ),
                Some(block.span.clone()),
            ));
        }
        let matched = declarations[0].get(0).expect("capture zero exists");
        let contract = declarations[0][2].to_string();
        if !contract_names.contains(&contract) {
            return Err(diagnostic(
                "RCIR005",
                format!(
                    "lane '{}.o({})' references unknown contract '{}'",
                    block.name, &declarations[0][1], contract
                ),
                Some(block.child_span(source, matched.start(), matched.end())),
            ));
        }
        outputs.insert(
            block.name.clone(),
            MessageRefIr {
                identity: format!("{}.o({})", block.name, &declarations[0][1]),
                producer: block.name.clone(),
                contract,
                projection: None,
            },
        );
    }
    Ok(outputs)
}

fn parse_lane(
    source: &str,
    block: &NamedBlock<'_>,
    lane_outputs: &HashMap<String, MessageRefIr>,
    port_lookup: &HashMap<String, PortInfo>,
) -> Result<LaneIr, ConcurrentDiagnostic> {
    let persona_pattern =
        Regex::new(r"(?m)^\s*persona\s+([A-Za-z][A-Za-z0-9_-]*)\s*$").expect("valid persona regex");
    let personas: Vec<_> = persona_pattern.captures_iter(block.body).collect();
    if personas.len() != 1 {
        return Err(diagnostic(
            "RCIR004",
            format!(
                "lane '{}' must declare exactly one persona; found {}",
                block.name,
                personas.len()
            ),
            Some(block.span.clone()),
        ));
    }

    let (input_symbol, input_expression, input_span) = parse_lane_input(source, block)?;
    let input_messages =
        resolve_message_refs(&input_expression, port_lookup, "RCIR005", Some(input_span))?;
    if input_messages.is_empty() {
        return Err(diagnostic(
            "RCIR005",
            format!("lane '{}' input has no message references", block.name),
            Some(block.span.clone()),
        ));
    }

    let output_message = lane_outputs
        .get(&block.name)
        .cloned()
        .expect("lane outputs were parsed in the first pass");
    let output_symbol = output_message
        .identity
        .rsplit_once("o(")
        .and_then(|(_, suffix)| suffix.strip_suffix(')'))
        .unwrap_or("")
        .to_string();

    let function_pattern =
        Regex::new(r#"(?m)^\s*([a-z])\s*:\s*i\(([a-z])\)\s*->\s*o\(([a-z])\)\s*~\s*"([^"]+)"\s*$"#)
            .expect("valid lane function regex");
    let functions: Vec<_> = function_pattern.captures_iter(block.body).collect();
    if functions.len() != 1 {
        return Err(diagnostic(
            "RCIR005",
            format!(
                "lane '{}' must declare exactly one compact function; found {}",
                block.name,
                functions.len()
            ),
            Some(block.span.clone()),
        ));
    }
    let function_match = functions[0].get(0).expect("capture zero exists");
    if functions[0][2] != input_symbol || functions[0][3] != output_symbol {
        return Err(diagnostic(
            "RCIR005",
            format!(
                "lane '{}' function ports do not match its input/output declarations",
                block.name
            ),
            Some(block.child_span(source, function_match.start(), function_match.end())),
        ));
    }
    let function = LaneFunctionIr {
        identity: format!("{}.{}", block.name, &functions[0][1]),
        familiar_name: functions[0][4].to_string(),
        span: block.child_span(source, function_match.start(), function_match.end()),
    };

    Ok(LaneIr {
        name: block.name.clone(),
        persona: personas[0][1].to_string(),
        input_symbol,
        input_expression,
        input_messages,
        output_symbol,
        output_message,
        function,
        policy: parse_lane_policy(block)?,
        span: block.span.clone(),
    })
}

fn parse_lane_input(
    source: &str,
    block: &NamedBlock<'_>,
) -> Result<(String, String, SourceSpan), ConcurrentDiagnostic> {
    let pattern =
        Regex::new(r"(?m)^\s*i\(([a-z])\)\s*:=\s*").expect("valid lane input opening regex");
    let openings: Vec<_> = pattern.captures_iter(block.body).collect();
    if openings.len() != 1 {
        return Err(diagnostic(
            "RCIR005",
            format!(
                "lane '{}' must declare exactly one input expression; found {}",
                block.name,
                openings.len()
            ),
            Some(block.span.clone()),
        ));
    }
    let opening_match = openings[0].get(0).expect("capture zero exists");
    let expression_start = opening_match.end();
    let remainder = &block.body[expression_start..];
    let leading = remainder.len() - remainder.trim_start().len();
    let expression_start = expression_start + leading;
    let remainder = &block.body[expression_start..];
    let expression_end = if remainder.starts_with("join(") {
        let opening_parenthesis = expression_start + "join".len();
        find_matching_delimiter(block.body, opening_parenthesis, b'(', b')').ok_or_else(|| {
            diagnostic(
                "RCIR005",
                format!(
                    "lane '{}' join input has no closing parenthesis",
                    block.name
                ),
                Some(block.span.clone()),
            )
        })? + 1
    } else {
        expression_start + remainder.find(['\r', '\n']).unwrap_or(remainder.len())
    };
    let expression = block.body[expression_start..expression_end]
        .trim()
        .to_string();
    let span = block.child_span(source, expression_start, expression_end);
    Ok((openings[0][1].to_string(), expression, span))
}

fn parse_lane_policy(block: &NamedBlock<'_>) -> Result<LanePolicyIr, ConcurrentDiagnostic> {
    Ok(LanePolicyIr {
        allow_read: parse_string_list(block, "allow", "read")?,
        allow_write: parse_string_list(block, "allow", "write")?,
        allow_tools: parse_string_list(block, "allow", "tools")?,
        required_receipts: parse_string_list(block, "require", "receipt")?,
    })
}

fn parse_string_list(
    block: &NamedBlock<'_>,
    verb: &str,
    noun: &str,
) -> Result<Vec<String>, ConcurrentDiagnostic> {
    let pattern = Regex::new(&format!(
        r#"(?m)^\s*{}\s+{}\s+\[([^\]]*)\]\s*$"#,
        regex::escape(verb),
        regex::escape(noun)
    ))
    .expect("valid dynamic policy regex");
    let declarations = pattern.captures_iter(block.body).collect::<Vec<_>>();
    if declarations.len() != 1 {
        return Err(diagnostic(
            "RCIR005",
            format!(
                "lane '{}' must declare exactly one '{verb} {noun}' policy; found {}",
                block.name,
                declarations.len()
            ),
            Some(block.span.clone()),
        ));
    }
    let contents = &declarations[0][1];
    let list_pattern = Regex::new(r#"^\s*(?:"[^"]*"\s*(?:,\s*"[^"]*"\s*)*)?$"#)
        .expect("valid quoted string list regex");
    if !list_pattern.is_match(contents) {
        return Err(diagnostic(
            "RCIR005",
            format!(
                "lane '{}' has an invalid '{verb} {noun}' string list",
                block.name
            ),
            Some(block.span.clone()),
        ));
    }
    let item_pattern = Regex::new(r#""([^"]*)""#).expect("valid quoted item regex");
    Ok(item_pattern
        .captures_iter(contents)
        .map(|item| item[1].to_string())
        .collect())
}

fn parse_flow_source<'a>(
    source: &'a str,
    flow_name: &str,
) -> Result<FlowSource<'a>, ConcurrentDiagnostic> {
    let pattern = Regex::new(&format!(
        r"(?m)^[ \t]*{}[ \t]+(sync|async)[ \t]*:[ \t]*(.*?)[ \t]*\r?$",
        regex::escape(flow_name)
    ))
    .expect("valid dynamic coordination flow regex");
    let matches: Vec<_> = pattern.captures_iter(source).collect();
    if matches.len() != 1 {
        return Err(diagnostic(
            "RCIR006",
            format!(
                "flow '{flow_name}' must be declared exactly once; found {}",
                matches.len()
            ),
            matches
                .first()
                .and_then(|captures| captures.get(0))
                .map(|matched| span_for(source, matched.start(), matched.end())),
        ));
    }
    let declaration = matches[0].get(0).expect("capture zero exists");
    let mut end_byte = declaration.end();
    let mut parts = Vec::new();
    if !matches[0][2].trim().is_empty() {
        parts.push(matches[0][2].trim().to_string());
    }
    for line in source[declaration.end()..].split_inclusive('\n') {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if parts.is_empty() {
                end_byte += line.len();
                continue;
            }
            break;
        }
        if !(trimmed.starts_with("i(") || trimmed.starts_with("->")) {
            break;
        }
        parts.push(trimmed.to_string());
        end_byte += line.len();
    }
    if parts.is_empty() {
        return Err(diagnostic(
            "RCIR006",
            format!("flow '{flow_name}' has no compact expression"),
            Some(span_for(source, declaration.start(), declaration.end())),
        ));
    }
    Ok(FlowSource {
        name: flow_name.to_string(),
        mode: matches[0][1].to_string(),
        raw: &source[declaration.start()..end_byte],
        expression: parts.join(" "),
        start_byte: declaration.start(),
        span: span_for(source, declaration.start(), end_byte),
    })
}

fn parse_flow(
    source: &str,
    flow_source: &FlowSource<'_>,
    port_lookup: &HashMap<String, PortInfo>,
) -> Result<ConcurrentFlowIr, ConcurrentDiagnostic> {
    let fork_pattern = Regex::new(r"fork\s*\[([^\]]+)\]").expect("valid fork expression regex");
    let forks: Vec<_> = fork_pattern.captures_iter(flow_source.raw).collect();
    if forks.len() != 1 {
        return Err(diagnostic(
            "RCIR006",
            format!(
                "async flow '{}' must declare exactly one fork; found {}",
                flow_source.name,
                forks.len()
            ),
            Some(flow_source.span.clone()),
        ));
    }
    let lane_call_pattern =
        Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)\s*\([a-z]\)").expect("valid lane call regex");
    let fork_lanes = lane_call_pattern
        .captures_iter(&forks[0][1])
        .map(|captures| captures[1].to_string())
        .collect::<Vec<_>>();
    if fork_lanes.is_empty() {
        return Err(diagnostic(
            "RCIR006",
            format!("async flow '{}' fork is empty", flow_source.name),
            Some(flow_source.span.clone()),
        ));
    }

    let await_pattern = Regex::new(
        r"(?s)await\s*(\[[^\]]+\]|[A-Za-z_][A-Za-z0-9_.]*\.o\([a-z]\))\s*->\s*([A-Za-z_][A-Za-z0-9_.]*)\s*\(",
    )
    .expect("valid await expression regex");
    let awaits = await_pattern
        .captures_iter(flow_source.raw)
        .map(|captures| {
            let matched = captures.get(0).expect("capture zero exists");
            let required = resolve_message_refs(&captures[1], port_lookup, "RCIR007", None)?;
            if required.is_empty() {
                return Err(diagnostic(
                    "RCIR006",
                    format!("flow '{}' contains an empty await", flow_source.name),
                    Some(span_for(
                        source,
                        flow_source.start_byte + matched.start(),
                        flow_source.start_byte + matched.end(),
                    )),
                ));
            }
            Ok(AwaitIr {
                required,
                next_consumer: captures[2].to_string(),
                span: span_for(
                    source,
                    flow_source.start_byte + matched.start(),
                    flow_source.start_byte + matched.end(),
                ),
            })
        })
        .collect::<Result<Vec<_>, ConcurrentDiagnostic>>()?;
    if awaits.is_empty() {
        return Err(diagnostic(
            "RCIR006",
            format!("async flow '{}' has no visible await", flow_source.name),
            Some(flow_source.span.clone()),
        ));
    }

    Ok(ConcurrentFlowIr {
        name: flow_source.name.clone(),
        mode: flow_source.mode.clone(),
        expression: flow_source.expression.clone(),
        fork_lanes,
        awaits,
        span: flow_source.span.clone(),
    })
}

fn resolve_message_refs(
    expression: &str,
    port_lookup: &HashMap<String, PortInfo>,
    diagnostic_code: &'static str,
    span: Option<SourceSpan>,
) -> Result<Vec<MessageRefIr>, ConcurrentDiagnostic> {
    let reference_pattern = Regex::new(r"([A-Za-z_][A-Za-z0-9_.]*)\.o\(([a-z])\)")
        .expect("valid message reference regex");
    let projection_pattern =
        Regex::new(r#"^\.([A-Za-z_][A-Za-z0-9_]*\["[^"]+"\])"#).expect("valid projection regex");
    reference_pattern
        .captures_iter(expression)
        .map(|captures| {
            let matched = captures.get(0).expect("capture zero exists");
            let identity = matched.as_str().to_string();
            let port = port_lookup.get(&identity).ok_or_else(|| {
                diagnostic(
                    diagnostic_code,
                    format!("message reference '{identity}' has no declared producer port"),
                    span.clone(),
                )
            })?;
            let projection = projection_pattern
                .captures(&expression[matched.end()..])
                .map(|projection| projection[1].to_string());
            let contract = if projection.is_some() {
                port.projected_contract
                    .clone()
                    .unwrap_or_else(|| port.contract.clone())
            } else {
                port.contract.clone()
            };
            Ok(MessageRefIr {
                identity,
                producer: port.producer.clone(),
                contract,
                projection,
            })
        })
        .collect()
}

fn validate_concurrent_graph(
    flow: &ConcurrentFlowIr,
    lanes: &[LaneIr],
    coordinator_ports: &[CoordinatorPortIr],
    span: &SourceSpan,
) -> Result<(), ConcurrentDiagnostic> {
    let lane_names: HashSet<_> = lanes.iter().map(|lane| lane.name.as_str()).collect();
    for lane in &flow.fork_lanes {
        if !lane_names.contains(lane.as_str()) {
            return Err(diagnostic(
                "RCIR007",
                format!("fork references unknown lane '{lane}'"),
                Some(span.clone()),
            ));
        }
    }

    let first_await = &flow.awaits[0];
    let awaited_producers = first_await
        .required
        .iter()
        .map(|message| message.producer.as_str())
        .collect::<Vec<_>>();
    let forked = flow
        .fork_lanes
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    if awaited_producers != forked {
        return Err(diagnostic(
            "RCIR008",
            format!(
                "first await producers {:?} do not exactly match fork lanes {:?}",
                awaited_producers, forked
            ),
            Some(first_await.span.clone()),
        ));
    }

    for await_ir in &flow.awaits {
        let contracts: BTreeSet<_> = await_ir
            .required
            .iter()
            .map(|message| message.contract.as_str())
            .collect();
        if contracts.len() > 1 {
            return Err(diagnostic(
                "RCIR009",
                format!(
                    "await before '{}' mixes message contracts {:?}",
                    await_ir.next_consumer, contracts
                ),
                Some(await_ir.span.clone()),
            ));
        }

        if let Some(consumer) = lanes
            .iter()
            .find(|lane| lane.name == await_ir.next_consumer)
        {
            let lane_dependencies = consumer
                .input_messages
                .iter()
                .filter(|message| lane_names.contains(message.producer.as_str()))
                .map(|message| message.identity.as_str())
                .collect::<Vec<_>>();
            let awaited = await_ir
                .required
                .iter()
                .map(|message| message.identity.as_str())
                .collect::<Vec<_>>();
            if lane_dependencies != awaited {
                return Err(diagnostic(
                    "RCIR008",
                    format!(
                        "await before '{}' does not match its lane receipt dependencies",
                        await_ir.next_consumer
                    ),
                    Some(await_ir.span.clone()),
                ));
            }
        } else if !coordinator_ports.iter().any(|port| {
            port.identity
                .starts_with(&format!("{}.", await_ir.next_consumer))
        }) {
            return Err(diagnostic(
                "RCIR007",
                format!(
                    "await references unknown downstream consumer '{}'",
                    await_ir.next_consumer
                ),
                Some(await_ir.span.clone()),
            ));
        }
    }

    let reachable: HashSet<_> = flow
        .fork_lanes
        .iter()
        .map(String::as_str)
        .chain(
            flow.awaits
                .iter()
                .map(|await_ir| await_ir.next_consumer.as_str())
                .filter(|consumer| lane_names.contains(consumer)),
        )
        .collect();
    let unreachable = lanes
        .iter()
        .filter(|lane| !reachable.contains(lane.name.as_str()))
        .map(|lane| lane.name.clone())
        .collect::<Vec<_>>();
    if !unreachable.is_empty() {
        return Err(diagnostic(
            "RCIR010",
            format!("flow contains unreachable lanes {unreachable:?}"),
            Some(span.clone()),
        ));
    }
    Ok(())
}

fn ensure_unique_block_names(
    blocks: &[NamedBlock<'_>],
    kind: &str,
    code: &'static str,
) -> Result<(), ConcurrentDiagnostic> {
    let mut names = HashSet::new();
    for block in blocks {
        if !names.insert(block.name.as_str()) {
            return Err(diagnostic(
                code,
                format!("duplicate {kind} declaration '{}'", block.name),
                Some(block.span.clone()),
            ));
        }
    }
    Ok(())
}

fn projected_contract_name(contract: &str) -> Option<String> {
    let pattern = Regex::new(r"^DispatchSet<([A-Za-z][A-Za-z0-9_]*)>$")
        .expect("valid projected contract regex");
    pattern
        .captures(contract)
        .map(|captures| captures[1].to_string())
}

fn producer_from_identity(identity: &str) -> String {
    identity
        .rsplit_once(".o(")
        .map(|(producer, _)| producer.to_string())
        .unwrap_or_default()
}

fn span_is_within(child: &SourceSpan, parent: &SourceSpan) -> bool {
    child.start_byte >= parent.start_byte && child.end_byte <= parent.end_byte
}

fn find_matching_delimiter(source: &str, opening: usize, open: u8, close: u8) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for (index, byte) in bytes.iter().enumerate().skip(opening) {
        if in_string {
            if escaped {
                escaped = false;
            } else if *byte == b'\\' {
                escaped = true;
            } else if *byte == b'"' {
                in_string = false;
            }
            continue;
        }
        if *byte == b'"' {
            in_string = true;
        } else if *byte == open {
            depth += 1;
        } else if *byte == close {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}

fn convert_diagnostic(diagnostic: crate::recur_lang_ir::IrDiagnostic) -> ConcurrentDiagnostic {
    ConcurrentDiagnostic {
        code: diagnostic.code,
        message: diagnostic.message,
        span: diagnostic.span,
    }
}

fn diagnostic(
    code: &'static str,
    message: String,
    span: Option<SourceSpan>,
) -> ConcurrentDiagnostic {
    ConcurrentDiagnostic {
        code,
        message,
        span,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str =
        include_str!("../demos/main.lang/main.lang.skippy-watch-coordination.recur");

    #[test]
    fn freezes_lane_messages_fork_and_ordered_awaits() {
        let ir = parse_concurrent_ir(SOURCE, "coordination.recur", "solution").unwrap();

        assert_eq!(ir.schema, CONCURRENT_IR_SCHEMA);
        assert_eq!(ir.language_version, "0.2");
        assert_eq!(ir.coordination_name, "SkippyWorkshop");
        assert_eq!(ir.contracts.len(), 7);
        assert_eq!(
            ir.lanes
                .iter()
                .map(|lane| lane.name.as_str())
                .collect::<Vec<_>>(),
            [
                "csharp_monkey",
                "web_monkey",
                "test_bird",
                "review_bird",
                "git_monkey",
            ],
        );
        assert_eq!(
            ir.flow.fork_lanes,
            ["csharp_monkey", "web_monkey", "test_bird"]
        );
        assert_eq!(ir.flow.awaits.len(), 3);
        assert_eq!(ir.flow.awaits[0].next_consumer, "review_bird");
        assert_eq!(ir.flow.awaits[1].next_consumer, "git_monkey");
        assert_eq!(ir.flow.awaits[2].next_consumer, "skippy.decide");
        assert_eq!(
            ir.flow.awaits[0]
                .required
                .iter()
                .map(|message| message.identity.as_str())
                .collect::<Vec<_>>(),
            ["csharp_monkey.o(b)", "web_monkey.o(b)", "test_bird.o(b)",],
        );
        assert!(ir.flow.awaits[0]
            .required
            .iter()
            .all(|message| message.contract == "WorkReceipt"));
        assert!(ir.flow.span.start_byte < ir.flow.span.end_byte);
        assert!(ir
            .lanes
            .iter()
            .all(|lane| lane.span.start_line <= lane.span.end_line));
    }

    #[test]
    fn preserves_projected_orders_and_downstream_receipt_dependencies() {
        let ir = parse_concurrent_ir(SOURCE, "coordination.recur", "solution").unwrap();
        let web = ir
            .lanes
            .iter()
            .find(|lane| lane.name == "web_monkey")
            .unwrap();
        let review = ir
            .lanes
            .iter()
            .find(|lane| lane.name == "review_bird")
            .unwrap();
        let git = ir
            .lanes
            .iter()
            .find(|lane| lane.name == "git_monkey")
            .unwrap();

        assert_eq!(web.input_messages[0].producer, "skippy.plan");
        assert_eq!(web.input_messages[0].contract, "WorkOrder");
        assert_eq!(
            web.input_messages[0].projection.as_deref(),
            Some("orders[\"web_monkey\"]")
        );
        assert_eq!(
            review
                .input_messages
                .iter()
                .filter(|message| message.contract == "WorkReceipt")
                .map(|message| message.producer.as_str())
                .collect::<Vec<_>>(),
            ["csharp_monkey", "web_monkey", "test_bird"],
        );
        assert!(git
            .input_messages
            .iter()
            .any(|message| message.identity == "review_bird.o(b)"));
        assert_eq!(
            git.policy.required_receipts,
            ["git.integration", "branch.tests"]
        );
    }

    #[test]
    fn normalized_json_is_deterministic() {
        let first = parse_concurrent_ir(SOURCE, "coordination.recur", "solution").unwrap();
        let second = parse_concurrent_ir(SOURCE, "coordination.recur", "solution").unwrap();

        assert_eq!(
            serde_json::to_string_pretty(&first).unwrap(),
            serde_json::to_string_pretty(&second).unwrap()
        );
    }

    #[test]
    fn unknown_fork_lane_has_a_stable_diagnostic() {
        let source = SOURCE.replace(
            "fork [csharp_monkey(a), web_monkey(a), test_bird(a)]",
            "fork [csharp_monkey(a), ghost_lane(a), test_bird(a)]",
        );

        let diagnostic =
            parse_concurrent_ir(&source, "coordination.recur", "solution").unwrap_err();

        assert_eq!(diagnostic.code, "RCIR007");
    }

    #[test]
    fn omitted_fork_receipt_has_a_stable_diagnostic() {
        let source = SOURCE.replace(
            "await [csharp_monkey.o(b), web_monkey.o(b), test_bird.o(b)]",
            "await [csharp_monkey.o(b), test_bird.o(b)]",
        );

        let diagnostic =
            parse_concurrent_ir(&source, "coordination.recur", "solution").unwrap_err();

        assert_eq!(diagnostic.code, "RCIR008");
    }

    #[test]
    fn mixed_await_contracts_have_a_stable_diagnostic() {
        let source = SOURCE.replacen("o(b) := WorkReceipt", "o(b) := Candidate", 2);

        let diagnostic =
            parse_concurrent_ir(&source, "coordination.recur", "solution").unwrap_err();

        assert_eq!(diagnostic.code, "RCIR009");
    }

    #[test]
    fn await_must_match_the_downstream_lane_dependencies() {
        let normalized = SOURCE.replace("\r\n", "\n");
        let source = normalized.replace(
            concat!(
                "      csharp_monkey.o(b),\n",
                "      web_monkey.o(b),\n",
                "      test_bird.o(b)\n",
            ),
            concat!("      csharp_monkey.o(b),\n", "      test_bird.o(b)\n"),
        );

        let diagnostic =
            parse_concurrent_ir(&source, "coordination.recur", "solution").unwrap_err();

        assert_eq!(diagnostic.code, "RCIR008");
        assert!(diagnostic.message.contains("review_bird"));
    }

    #[test]
    fn invalid_contract_field_cannot_be_silently_ignored() {
        let source = SOURCE.replace("objective: Text", "objective Text");

        let diagnostic =
            parse_concurrent_ir(&source, "coordination.recur", "solution").unwrap_err();

        assert_eq!(diagnostic.code, "RCIR002");
        assert!(diagnostic.message.contains("objective Text"));
    }

    #[test]
    fn missing_lane_policy_cannot_be_silently_ignored() {
        let source = SOURCE.replacen(
            r#"    allow tools ["editor", "recur", "recur-watch", "dotnet"]"#,
            "",
            1,
        );

        let diagnostic =
            parse_concurrent_ir(&source, "coordination.recur", "solution").unwrap_err();

        assert_eq!(diagnostic.code, "RCIR005");
        assert!(diagnostic.message.contains("allow tools"));
    }

    #[test]
    fn malformed_lane_policy_list_has_a_stable_diagnostic() {
        let source = SOURCE.replacen(
            r#"    allow write ["src/Web/ClientApp/**"]"#,
            r#"    allow write ["src/Web/ClientApp/**" "tests/**"]"#,
            1,
        );

        let diagnostic =
            parse_concurrent_ir(&source, "coordination.recur", "solution").unwrap_err();

        assert_eq!(diagnostic.code, "RCIR005");
        assert!(diagnostic.message.contains("invalid 'allow write'"));
    }
}

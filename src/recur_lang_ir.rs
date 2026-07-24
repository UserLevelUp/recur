//! Versioned canonical IR for the bounded Recur Lang 0.1 Warp subset.

use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

pub const WARP_IR_SCHEMA: &str = "recur-lang-warp-ir-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceSpan {
    /// Zero-based byte offset, inclusive.
    pub start_byte: usize,
    /// Zero-based byte offset, exclusive.
    pub end_byte: usize,
    /// One-based source line containing `start_byte`.
    pub start_line: usize,
    /// One-based source line containing the final byte in the span.
    pub end_line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FieldIr {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContractIr {
    pub symbol: String,
    pub role: String,
    pub local_identity: String,
    pub canonical_identity: String,
    pub fields: Vec<FieldIr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FunctionIr {
    pub symbol: String,
    pub identity: String,
    pub familiar_name: String,
    pub worker: String,
    pub input: ContractIr,
    pub output: ContractIr,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FlowIr {
    pub mode: String,
    pub expression: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EventIr {
    pub edge: String,
    pub identifier: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WarpTransitionIr {
    pub current: String,
    pub slice: String,
    pub desired: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScopeIr {
    pub name: String,
    pub span: SourceSpan,
    pub function: FunctionIr,
    pub flow: FlowIr,
    pub event_span: SourceSpan,
    pub events: Vec<EventIr>,
    pub warp: WarpTransitionIr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WarpIr {
    pub schema: &'static str,
    pub language_version: String,
    pub class_name: String,
    pub source: String,
    pub source_hash: String,
    pub scope: ScopeIr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct IrDiagnostic {
    pub code: &'static str,
    pub message: String,
    pub span: Option<SourceSpan>,
}

impl fmt::Display for IrDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for IrDiagnostic {}

pub fn parse_warp_ir(
    source: &str,
    source_name: &str,
    scope_name: &str,
) -> Result<WarpIr, IrDiagnostic> {
    let declaration_pattern =
        Regex::new(r"(?m)^\s*recur\s+([0-9.]+)\s+class\s+([A-Za-z][A-Za-z0-9_]*)\s*$")
            .expect("valid Recur declaration regex");
    let declarations: Vec<_> = declaration_pattern.captures_iter(source).collect();
    if declarations.len() != 1 {
        return Err(diagnostic(
            "RLIR001",
            format!(
                "source must declare exactly one 'recur <version> class <Name>'; found {}",
                declarations.len()
            ),
            declarations
                .first()
                .and_then(|captures| captures.get(0))
                .map(|matched| span_for(source, matched.start(), matched.end())),
        ));
    }
    let language_version = declarations[0][1].to_string();
    let class_name = declarations[0][2].to_string();

    let scope_blocks = find_named_blocks(source, "scope", "RLIR002")?;
    let selected_scopes: Vec<_> = scope_blocks
        .iter()
        .filter(|block| block.name == scope_name)
        .collect();
    if selected_scopes.len() != 1 {
        return Err(diagnostic(
            "RLIR002",
            format!(
                "scope '{scope_name}' must be declared exactly once; found {}",
                selected_scopes.len()
            ),
            selected_scopes.first().map(|block| block.span.clone()),
        ));
    }
    let selected_scope = selected_scopes[0];
    let contracts = parse_contracts(source, &scope_blocks)?;

    let function_pattern = Regex::new(
        r#"(?m)^\s*([a-z])\s*:\s*i\(([a-z])\)\s*->\s*o\(([a-z])\)\s*~\s*"([^"]+)"\s+by\s+([A-Za-z][A-Za-z0-9_.-]*)\s*$"#,
    )
    .expect("valid function regex");
    let functions: Vec<_> = function_pattern
        .captures_iter(selected_scope.body)
        .collect();
    if functions.len() != 1 {
        return Err(diagnostic(
            "RLIR003",
            format!(
                "scope '{scope_name}' must declare exactly one compact function; found {}",
                functions.len()
            ),
            functions
                .first()
                .and_then(|captures| captures.get(0))
                .map(|matched| selected_scope.child_span(source, matched.start(), matched.end())),
        ));
    }
    let function_match = functions[0].get(0).expect("capture zero exists");
    let function_symbol = functions[0][1].to_string();
    let input_symbol = functions[0][2].to_string();
    let output_symbol = functions[0][3].to_string();
    let input = contracts
        .get(&contract_key(scope_name, "i", &input_symbol))
        .cloned()
        .ok_or_else(|| {
            diagnostic(
                "RLIR003",
                format!(
                    "{}.{function_symbol} references unknown input contract {scope_name}.i({input_symbol})",
                    scope_name
                ),
                Some(selected_scope.child_span(
                    source,
                    function_match.start(),
                    function_match.end(),
                )),
            )
        })?;
    let output = contracts
        .get(&contract_key(scope_name, "o", &output_symbol))
        .cloned()
        .ok_or_else(|| {
            diagnostic(
                "RLIR003",
                format!(
                    "{}.{function_symbol} references unknown output contract {scope_name}.o({output_symbol})",
                    scope_name
                ),
                Some(selected_scope.child_span(
                    source,
                    function_match.start(),
                    function_match.end(),
                )),
            )
        })?;
    let function_identity = format!("{scope_name}.{function_symbol}");
    let function = FunctionIr {
        symbol: function_symbol.clone(),
        identity: function_identity.clone(),
        familiar_name: functions[0][4].to_string(),
        worker: functions[0][5].to_string(),
        input,
        output,
        span: selected_scope.child_span(source, function_match.start(), function_match.end()),
    };

    let flow_pattern = Regex::new(&format!(
        r"(?m)^\s*{}\s+(sync|async)\s*:\s*(.+?)\s*$",
        regex::escape(scope_name)
    ))
    .expect("valid dynamic flow regex");
    let flows: Vec<_> = flow_pattern.captures_iter(source).collect();
    if flows.len() != 1 {
        return Err(diagnostic(
            "RLIR004",
            format!(
                "scope '{scope_name}' must declare exactly one compact body flow; found {}",
                flows.len()
            ),
            flows
                .first()
                .and_then(|captures| captures.get(0))
                .map(|matched| span_for(source, matched.start(), matched.end())),
        ));
    }
    let flow_match = flows[0].get(0).expect("capture zero exists");
    let flow_expression = flows[0][2].trim().to_string();
    let expected_flow =
        format!("i({input_symbol}) -> {function_symbol}({input_symbol}) -> o({output_symbol})");
    if without_whitespace(&flow_expression) != without_whitespace(&expected_flow) {
        return Err(diagnostic(
            "RLIR005",
            format!(
                "scope '{scope_name}' flow does not match its function contract; expected '{expected_flow}'"
            ),
            Some(span_for(source, flow_match.start(), flow_match.end())),
        ));
    }
    let flow = FlowIr {
        mode: flows[0][1].to_string(),
        expression: flow_expression,
        span: span_for(source, flow_match.start(), flow_match.end()),
    };

    let event_blocks = find_named_blocks(source, "event", "RLIR006")?;
    let selected_events: Vec<_> = event_blocks
        .iter()
        .filter(|block| block.name == scope_name)
        .collect();
    if selected_events.len() != 1 {
        return Err(diagnostic(
            "RLIR006",
            format!(
                "event block '{scope_name}' must be declared exactly once; found {}",
                selected_events.len()
            ),
            selected_events.first().map(|block| block.span.clone()),
        ));
    }
    let event_block = selected_events[0];
    let event_pattern =
        Regex::new(r"(?m)^\s*(consume|trigger|produce|state)\s+([A-Za-z0-9_.-]+)\s*$")
            .expect("valid event regex");
    let events: Vec<EventIr> = event_pattern
        .captures_iter(event_block.body)
        .map(|captures| {
            let matched = captures.get(0).expect("capture zero exists");
            EventIr {
                edge: captures[1].to_string(),
                identifier: captures[2].to_string(),
                span: event_block.child_span(source, matched.start(), matched.end()),
            }
        })
        .collect();

    let warp_pattern = Regex::new(&format!(
        r"(?m)^\s*warp\s+{}\s*:\s*E0\(([A-Za-z0-9_.-]+)\)\s*->\s*dE\(([A-Za-z0-9_.-]+)\)\s*->\s*Ef\(([A-Za-z0-9_.-]+)\)\s*$",
        regex::escape(scope_name)
    ))
    .expect("valid dynamic Warp regex");
    let warps: Vec<_> = warp_pattern.captures_iter(source).collect();
    if warps.len() != 1 {
        return Err(diagnostic(
            "RLIR007",
            format!(
                "scope '{scope_name}' must declare exactly one Warp; found {}",
                warps.len()
            ),
            warps
                .first()
                .and_then(|captures| captures.get(0))
                .map(|matched| span_for(source, matched.start(), matched.end())),
        ));
    }
    let warp_match = warps[0].get(0).expect("capture zero exists");
    let current = warps[0][1].to_string();
    let slice = warps[0][2].to_string();
    let desired = warps[0][3].to_string();
    let warp_span = span_for(source, warp_match.start(), warp_match.end());
    if slice != function_identity {
        return Err(diagnostic(
            "RLIR008",
            format!("scope '{scope_name}' Warp uses dE({slice}); expected dE({function_identity})"),
            Some(warp_span),
        ));
    }
    if current == desired {
        return Err(diagnostic(
            "RLIR009",
            format!(
                "scope '{scope_name}' Warp must change Eventness; E0 and Ef are both '{current}'"
            ),
            Some(warp_span),
        ));
    }
    if !events
        .iter()
        .any(|event| event.edge == "state" && event.identifier == desired)
    {
        return Err(diagnostic(
            "RLIR010",
            format!("Ef({desired}) is not a declared state event for scope '{scope_name}'"),
            Some(warp_span),
        ));
    }
    let warp = WarpTransitionIr {
        current,
        slice,
        desired,
        span: warp_span,
    };

    Ok(WarpIr {
        schema: WARP_IR_SCHEMA,
        language_version,
        class_name,
        source: source_name.to_string(),
        source_hash: stable_source_hash(source.as_bytes()),
        scope: ScopeIr {
            name: scope_name.to_string(),
            span: selected_scope.span.clone(),
            function,
            flow,
            event_span: event_block.span.clone(),
            events,
            warp,
        },
    })
}

#[derive(Debug)]
pub(crate) struct NamedBlock<'a> {
    pub(crate) name: String,
    pub(crate) body: &'a str,
    pub(crate) body_start: usize,
    pub(crate) span: SourceSpan,
}

impl NamedBlock<'_> {
    pub(crate) fn child_span(&self, source: &str, start: usize, end: usize) -> SourceSpan {
        span_for(source, self.body_start + start, self.body_start + end)
    }
}

pub(crate) fn find_named_blocks<'a>(
    source: &'a str,
    keyword: &str,
    diagnostic_code: &'static str,
) -> Result<Vec<NamedBlock<'a>>, IrDiagnostic> {
    let pattern = Regex::new(&format!(
        r"(?m)\b{}\s+([A-Za-z][A-Za-z0-9_.]*)\s*\{{",
        regex::escape(keyword)
    ))
    .expect("valid dynamic block regex");
    let mut blocks = Vec::new();
    for captures in pattern.captures_iter(source) {
        let matched = captures.get(0).expect("capture zero exists");
        let opening = matched.end() - 1;
        let closing = find_closing_brace(source, opening).ok_or_else(|| {
            diagnostic(
                diagnostic_code,
                format!("{keyword} block '{}' has no closing brace", &captures[1]),
                Some(span_for(source, matched.start(), matched.end())),
            )
        })?;
        let body_start = opening + 1;
        blocks.push(NamedBlock {
            name: captures[1].to_string(),
            body: &source[body_start..closing],
            body_start,
            span: span_for(source, matched.start(), closing + 1),
        });
    }
    Ok(blocks)
}

fn find_closing_brace(source: &str, opening: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut in_comment = false;
    for (index, byte) in bytes.iter().enumerate().skip(opening) {
        if in_comment {
            if *byte == b'\n' {
                in_comment = false;
            }
            continue;
        }
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
        match *byte {
            b'#' => in_comment = true,
            b'"' => in_string = true,
            b'{' => depth += 1,
            b'}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_contracts(
    source: &str,
    scope_blocks: &[NamedBlock<'_>],
) -> Result<HashMap<String, ContractIr>, IrDiagnostic> {
    let direct_pattern = Regex::new(r"(?m)^\s*([io])\(([a-z])\)\s*:=\s*\(([^)]*)\)\s*$")
        .expect("valid direct contract regex");
    let alias_pattern = Regex::new(
        r"(?m)^\s*([io])\(([a-z])\)\s*:=\s*([A-Za-z][A-Za-z0-9_.]*)\.([io])\(([a-z])\)\s*$",
    )
    .expect("valid alias contract regex");
    let field_pattern =
        Regex::new(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([A-Za-z][A-Za-z0-9_<>?]*)\s*$")
            .expect("valid field regex");
    let mut contracts = HashMap::new();

    for block in scope_blocks {
        for captures in direct_pattern.captures_iter(block.body) {
            let matched = captures.get(0).expect("capture zero exists");
            let marker = &captures[1];
            let symbol = &captures[2];
            let fields = captures[3]
                .split(',')
                .map(|raw_field| {
                    let field = field_pattern.captures(raw_field).ok_or_else(|| {
                        diagnostic(
                            "RLIR011",
                            format!(
                                "invalid field '{}' in {}.{}({symbol})",
                                raw_field.trim(),
                                block.name,
                                marker
                            ),
                            Some(block.child_span(source, matched.start(), matched.end())),
                        )
                    })?;
                    Ok(FieldIr {
                        name: field[1].to_string(),
                        type_name: field[2].to_string(),
                    })
                })
                .collect::<Result<Vec<_>, IrDiagnostic>>()?;
            if fields.is_empty() {
                return Err(diagnostic(
                    "RLIR011",
                    format!(
                        "contract {}.{}({symbol}) cannot be empty",
                        block.name, marker
                    ),
                    Some(block.child_span(source, matched.start(), matched.end())),
                ));
            }
            let identity = format!("{}.{}({symbol})", block.name, marker);
            let contract = ContractIr {
                symbol: symbol.to_string(),
                role: role_for_marker(marker).to_string(),
                local_identity: identity.clone(),
                canonical_identity: identity,
                fields,
            };
            insert_contract(
                &mut contracts,
                block,
                marker,
                symbol,
                contract,
                source,
                matched,
            )?;
        }

        for captures in alias_pattern.captures_iter(block.body) {
            let matched = captures.get(0).expect("capture zero exists");
            let marker = &captures[1];
            let symbol = &captures[2];
            let target_scope = &captures[3];
            let target_marker = &captures[4];
            let target_symbol = &captures[5];
            if marker != "i" || target_marker != "o" {
                return Err(diagnostic(
                    "RLIR011",
                    format!(
                        "contract alias {}.{}({symbol}) must connect i(...) to a prior o(...)",
                        block.name, marker
                    ),
                    Some(block.child_span(source, matched.start(), matched.end())),
                ));
            }
            let target = contracts
                .get(&contract_key(target_scope, target_marker, target_symbol))
                .cloned()
                .ok_or_else(|| {
                    diagnostic(
                        "RLIR011",
                        format!(
                            "{}.{}({symbol}) aliases unknown contract {target_scope}.{target_marker}({target_symbol})",
                            block.name, marker
                        ),
                        Some(block.child_span(source, matched.start(), matched.end())),
                    )
                })?;
            let contract = ContractIr {
                symbol: symbol.to_string(),
                role: role_for_marker(marker).to_string(),
                local_identity: format!("{}.{}({symbol})", block.name, marker),
                canonical_identity: target.canonical_identity,
                fields: target.fields,
            };
            insert_contract(
                &mut contracts,
                block,
                marker,
                symbol,
                contract,
                source,
                matched,
            )?;
        }
    }
    Ok(contracts)
}

fn insert_contract(
    contracts: &mut HashMap<String, ContractIr>,
    block: &NamedBlock<'_>,
    marker: &str,
    symbol: &str,
    contract: ContractIr,
    source: &str,
    matched: regex::Match<'_>,
) -> Result<(), IrDiagnostic> {
    let key = contract_key(&block.name, marker, symbol);
    if contracts.insert(key, contract).is_some() {
        return Err(diagnostic(
            "RLIR011",
            format!("duplicate contract {}.{}({symbol})", block.name, marker),
            Some(block.child_span(source, matched.start(), matched.end())),
        ));
    }
    Ok(())
}

fn contract_key(scope: &str, marker: &str, symbol: &str) -> String {
    format!("{scope}.{marker}({symbol})")
}

fn role_for_marker(marker: &str) -> &'static str {
    if marker == "i" {
        "input"
    } else {
        "output"
    }
}

fn without_whitespace(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

pub(crate) fn stable_source_hash(bytes: &[u8]) -> String {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let hash = bytes.iter().fold(OFFSET, |value, byte| {
        (value ^ u64::from(*byte)).wrapping_mul(PRIME)
    });
    format!("fnv1a64:{hash:016x}")
}

pub(crate) fn span_for(source: &str, start_byte: usize, end_byte: usize) -> SourceSpan {
    let start_line = 1 + source.as_bytes()[..start_byte]
        .iter()
        .filter(|byte| **byte == b'\n')
        .count();
    let final_offset = end_byte.saturating_sub(1);
    let end_line = 1 + source.as_bytes()[..final_offset]
        .iter()
        .filter(|byte| **byte == b'\n')
        .count();
    SourceSpan {
        start_byte,
        end_byte,
        start_line,
        end_line,
    }
}

fn diagnostic(code: &'static str, message: String, span: Option<SourceSpan>) -> IrDiagnostic {
    IrDiagnostic {
        code,
        message,
        span,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = r#"recur 0.1 class Demo

header {
  scope source {
    i(a) := (request: Text)
    o(b) := (artifact: Text)
    f : i(a) -> o(b) ~ "Produce an artifact" by external.source
  }

  scope verify {
    i(b) := source.o(b)
    o(c) := (accepted: Text)
    f : i(b) -> o(c) ~ "Verify the artifact" by external.verify
  }
}

body {
  source sync : i(a) -> f(a) -> o(b)
  verify sync : i(b) -> f(b) -> o(c)
  share source.o(b) -> verify.i(b)
}

footer {
  event verify {
    consume demo.verify.input
    trigger demo.verify.run
    produce demo.verify.output
    state demo.verify.complete
  }
  warp verify : E0(demo.verify.todo.current) -> dE(verify.f) -> Ef(demo.verify.complete)
}
"#;

    #[test]
    fn freezes_versioned_ir_with_canonical_contracts_and_spans() {
        let ir = parse_warp_ir(SOURCE, "demo.recur", "verify").unwrap();

        assert_eq!(ir.schema, WARP_IR_SCHEMA);
        assert_eq!(ir.language_version, "0.1");
        assert_eq!(ir.class_name, "Demo");
        assert_eq!(ir.source, "demo.recur");
        assert!(ir.source_hash.starts_with("fnv1a64:"));
        assert_eq!(ir.scope.name, "verify");
        assert_eq!(ir.scope.function.identity, "verify.f");
        assert_eq!(ir.scope.function.input.local_identity, "verify.i(b)");
        assert_eq!(ir.scope.function.input.canonical_identity, "source.o(b)");
        assert_eq!(ir.scope.function.output.canonical_identity, "verify.o(c)");
        assert_eq!(ir.scope.function.input.fields[0].name, "artifact");
        assert_eq!(ir.scope.flow.mode, "sync");
        assert_eq!(ir.scope.events.len(), 4);
        assert_eq!(ir.scope.warp.current, "demo.verify.todo.current");
        assert_eq!(ir.scope.warp.slice, "verify.f");
        assert_eq!(ir.scope.warp.desired, "demo.verify.complete");

        for span in [
            &ir.scope.span,
            &ir.scope.function.span,
            &ir.scope.flow.span,
            &ir.scope.event_span,
            &ir.scope.warp.span,
        ] {
            assert!(span.start_byte < span.end_byte);
            assert!(span.start_line <= span.end_line);
            assert!(span.start_line > 0);
            assert!(span.end_byte <= SOURCE.len());
        }
        assert!(
            SOURCE[ir.scope.warp.span.start_byte..ir.scope.warp.span.end_byte]
                .trim_start()
                .starts_with("warp verify")
        );
    }

    #[test]
    fn json_projection_uses_the_frozen_schema_and_span_shape() {
        let ir = parse_warp_ir(SOURCE, "demo.recur", "verify").unwrap();

        let json = serde_json::to_value(ir).unwrap();

        assert_eq!(json["schema"], WARP_IR_SCHEMA);
        assert_eq!(
            json["scope"]["function"]["input"]["canonical_identity"],
            "source.o(b)"
        );
        assert!(json["scope"]["warp"]["span"]["start_byte"].is_u64());
        assert!(json["scope"]["warp"]["span"]["start_line"].is_u64());
    }

    #[test]
    fn flow_contract_mismatch_has_a_stable_diagnostic_code() {
        let source = SOURCE.replace(
            "verify sync : i(b) -> f(b) -> o(c)",
            "verify sync : i(b) -> f(b) -> o(d)",
        );

        let diagnostic = parse_warp_ir(&source, "demo.recur", "verify").unwrap_err();

        assert_eq!(diagnostic.code, "RLIR005");
        assert!(diagnostic.span.is_some());
        assert_eq!(
            serde_json::to_value(&diagnostic).unwrap()["code"],
            "RLIR005"
        );
    }

    #[test]
    fn undeclared_final_state_has_a_stable_diagnostic_code() {
        let source = SOURCE.replace("state demo.verify.complete", "state demo.verify.reviewed");

        let diagnostic = parse_warp_ir(&source, "demo.recur", "verify").unwrap_err();

        assert_eq!(diagnostic.code, "RLIR010");
        assert!(diagnostic.span.is_some());
    }

    #[test]
    fn missing_declarations_have_stable_cardinality_codes() {
        let cases = [
            (SOURCE.replace("scope verify", "scope other"), "RLIR002"),
            (
                SOURCE.replace(
                    "f : i(b) -> o(c) ~ \"Verify the artifact\" by external.verify",
                    "# function intentionally missing",
                ),
                "RLIR003",
            ),
            (SOURCE.replace("verify sync :", "other sync :"), "RLIR004"),
            (SOURCE.replace("event verify", "event other"), "RLIR006"),
            (SOURCE.replace("warp verify", "warp other"), "RLIR007"),
        ];

        for (source, expected_code) in cases {
            let diagnostic = parse_warp_ir(&source, "demo.recur", "verify").unwrap_err();
            assert_eq!(diagnostic.code, expected_code);
        }
    }
}

use serde_json::{json, Value};
use std::{collections::BTreeMap, env, fs, path::Path};
use tree_sitter::{Node, Parser};

fn text<'a>(node: Node<'_>, source: &'a [u8]) -> &'a str {
    node.utf8_text(source).expect("UTF-8 source")
}
fn find_arm<'a>(
    node: Node<'a>,
    line: usize,
    column: Option<usize>,
    found: &mut Vec<Node<'a>>,
) {
    let point = node.start_position();
    if node.kind() == "match_arm"
        && point.row + 1 == line
        && column.map_or(true, |c| point.column + 1 == c)
    {
        found.push(node);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.start_position().row + 1 <= line && child.end_position().row + 1 >= line
        {
            find_arm(child, line, column, found);
        }
    }
}
fn ancestor<'a>(mut node: Node<'a>, kind: &str) -> Node<'a> {
    loop {
        if node.kind() == kind {
            return node;
        }
        node = node.parent().expect("required ancestor");
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let root = Path::new(&args[1]);
    let sarif: Value =
        serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let latent: Value =
        serde_json::from_str(&fs::read_to_string(&args[3]).unwrap()).unwrap();
    let mut wanted = Vec::new();
    for row in sarif["runs"][0]["results"].as_array().unwrap() {
        if row["ruleId"] != "V16" || row["level"] != "warning" {
            continue;
        }
        let loc = &row["locations"][0]["physicalLocation"];
        wanted.push(json!({"source": loc["artifactLocation"]["uri"].as_str().unwrap().trim_start_matches("./"), "line": loc["region"]["startLine"], "column": loc["region"]["startColumn"], "status":"emitted-warning", "message":row["message"]["text"]}));
    }
    for row in latent.as_array().unwrap() {
        wanted.push(json!({"source": row["source"],"line":row["line"],"column":null,"status":"unemitted-tracked","ids":row["ids"],"message":row["reason"]}));
    }
    let mut grouped: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for row in wanted {
        grouped
            .entry(row["source"].as_str().unwrap().into())
            .or_default()
            .push(row);
    }
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
    let mut output = Vec::new();
    for (path, rows) in grouped {
        let source = fs::read_to_string(root.join(&path)).unwrap();
        let tree = parser.parse(&source, None).unwrap();
        assert!(!tree.root_node().has_error(), "syntax error in {path}");
        let owner = source
            .lines()
            .find_map(|line| line.strip_prefix("//! @prompt "))
            .unwrap();
        for mut row in rows {
            let mut found = Vec::new();
            find_arm(
                tree.root_node(),
                row["line"].as_u64().unwrap() as usize,
                row["column"].as_u64().map(|c| c as usize),
                &mut found,
            );
            assert_eq!(found.len(), 1, "ambiguous/missing arm: {row}");
            let arm = found[0];
            let decision = ancestor(arm, "match_expression");
            let function = ancestor(arm, "function_item");
            let mut cursor = arm.parent().unwrap().walk();
            let arms: Vec<_> = arm.parent().unwrap().children(&mut cursor).filter(|n| n.kind()=="match_arm").map(|n| {
                let mut walk = n.walk();
                let pattern = n.children(&mut walk).find(|c| c.kind()=="match_pattern").unwrap();
                json!({"pattern":text(pattern,source.as_bytes()),"line":n.start_position().row+1})
            }).collect();
            row["owner"] = json!(owner);
            row["function"] = json!(text(
                function.child_by_field_name("name").unwrap(),
                source.as_bytes()
            ));
            row["function_line"] = json!(function.start_position().row + 1);
            row["match_line"] = json!(decision.start_position().row + 1);
            row["match_end_line"] = json!(decision.end_position().row + 1);
            row["scrutinee"] = json!(text(
                decision.child_by_field_name("value").unwrap(),
                source.as_bytes()
            ));
            row["arms"] = json!(arms);
            row["match_source"] = json!(text(decision, source.as_bytes()));
            output.push(row);
        }
    }
    println!("{}", serde_json::to_string(&output).unwrap());
}

//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/loading/xml.md
//! @prompt-hash 49294219
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ─────────────────────────────────────────────────────────────────────────────
// XML — roxmltree (read-only). Cada elemento → Dict{tag, attrs, children}.
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_xml(bytes: &[u8]) -> SourceResult<Value> {
    let s = std::str::from_utf8(bytes)
        .map_err(|_| err("xml: ficheiro não é UTF-8 válido"))?;
    let doc =
        roxmltree::Document::parse(s).map_err(|e| err(format!("xml inválido: {e}")))?;
    // Topo: o elemento-raiz como nó único, num Array (paridade `convert_xml`).
    Ok(Value::Array(vec![xml_node_to_value(doc.root_element())]))
}

fn xml_node_to_value(node: roxmltree::Node) -> Value {
    let mut d = new_dict();
    d.insert("tag".into(), Value::Str(node.tag_name().name().into()));

    let mut attrs = new_dict();
    for a in node.attributes() {
        attrs.insert(a.name().into(), Value::Str(a.value().into()));
    }
    d.insert("attrs".into(), Value::Dict(attrs));

    let mut children = Vec::new();
    for child in node.children() {
        if child.is_element() {
            children.push(xml_node_to_value(child));
        } else if child.is_text() {
            if let Some(t) = child.text() {
                children.push(Value::Str(t.into()));
            }
        }
    }
    d.insert("children".into(), Value::Array(children));
    Value::Dict(d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_xml_root_as_node_array() {
        let Value::Array(nodes) = decode_xml(b"<r>x</r>").unwrap() else {
            panic!("array expected")
        };
        assert_eq!(nodes.len(), 1);
        assert!(matches!(&nodes[0], Value::Dict(_)));
    }
}

from pathlib import Path
p=Path('01_core/src/compiler/eval/rules/apply.rs');s=p.read_text();a=s.index('            if any_set {');i=s.index('                let delta = set_chain.collapse();',a);s=s[:i]+s[i:].replace('''                let delta = set_chain.collapse();''','''                let mut delta = set_chain.collapse();
                if matches!(node.source_body(), Content::Heading(_))
                    && matches!(work.source_body(), Content::Heading(_))
                    && (delta.size.is_some() || delta.custom.iter().any(|(key, value)| key.as_str() == "text.size" && matches!(value, Value::Length(_))))
                {
                    // Preserve rule provenance; the full declared size stays untouched.
                    delta.custom.push(("heading.show-set-size".into(), Value::Bool(true)));
                }''',1);p.write_text(s)
p=Path('01_core/src/compiler/layout/style_scope.rs');s=p.read_text();i=s.index('    layouter.chain = layouter.chain.push(delta);');s=s[:i]+'''    let heading_show_set = delta.custom.iter().any(|(key, value)|
        key.as_str() == "heading.show-set-size" && value == &Value::Bool(true));
    delta.custom.retain(|(key, _)| key.as_str() != "heading.show-set-size");
    if heading_show_set {
        if let Content::Heading(heading) = body.source_body() {
            // Heading applies its default scale after this scope. Only its
            // show-set's absolute contribution must bypass that multiplication.
            let scale = super::helpers::heading_scale(heading.level);
            if let Some(size) = &mut delta.size {
                size.abs = crate::entities::layout_types::Abs::pt(size.abs.to_pt() / scale);
            } else if let Some(Value::Length(size)) = delta.custom.iter_mut()
                .find_map(|(key, value)| (key.as_str() == "text.size").then_some(value))
            {
                size.abs = crate::entities::layout_types::Abs::pt(size.abs.to_pt() / scale);
            }
        }
    }
''' +s[i:];p.write_text(s)
# Preserve the existing unsupported Paint behavior outside TS1's solid-color scope.
p=Path('01_core/src/compiler/stdlib/text/constructor.rs');s=p.read_text();a=s.index('                Value::Color(color) => styles.push');z=s.index('                other =>',a);s=s[:z]+'''                Value::Gradient(_) | Value::Tiling(_) => return Err(vec![SourceDiagnostic::error(args.span,
                    format!("text(fill:) espera color, recebeu {}", arg.value.type_name()))]),
''' +s[z:];p.write_text(s)
p=Path('01_core/src/compiler/eval/rules/set/text.rs');s=p.read_text();a=s.index('                "fill" => {');z=s.index('                        other =>',a);s=s[:z]+'''                        Value::Gradient(_) | Value::Tiling(_) => {
                            // Existing partial transport; Paint support is a separate contract.
                        }
''' +s[z:];p.write_text(s)
p=Path('01_core/src/compiler/layout/heading.rs');s=p.read_text();s=s.replace('''    let prev = layouter.style.clone();''','''    let prev = layouter.style.clone();
    let prev_chain = layouter.chain.clone();''');s=s.replace('''    layouter.style = heading_style;''','''    layouter.chain = layouter.chain.push(crate::entities::style_chain::StyleDelta {
        size: Some(crate::entities::layout_types::Length::pt(heading_size.val())),
        ..crate::entities::style_chain::StyleDelta::empty()
    });
    layouter.style = heading_style;''');s=s.replace('''    layouter.style = prev;''','''    layouter.style = prev;
    layouter.chain = prev_chain;''');p.write_text(s)
p=Path('01_core/src/compiler/eval/rules/set/text.rs');s=p.read_text();a=s.index('            let source_expr =');z=s.index('            match key.as_str()',a);s=s[:a]+'''            let source_expr = set.args().items().find_map(|arg| match arg {
                Arg::Named(named) => {
                    if named.name().as_str() == key && named.expr().span() == value_span {
                        Some(named.expr())
                    } else {
                        None
                    }
                }
                Arg::Pos(_) | Arg::Spread(_) => None,
            });
''' +s[z:];a=s.index('                            Value::Array(items)',s.index('                    } else {',s.index('                "font" =>')));z=s.index('                            _ =>',a);s=s[:a]+'''                            Value::Array(items) => {
                                if items.is_empty() || !items.iter().all(|item| matches!(item, Value::Str(_))) {
                                    return Err(vec![SourceDiagnostic::error(span, "font expects a string, array of strings, or dict")]);
                                }
                                items.clone()
                            }
''' +s[z:];p.write_text(s)

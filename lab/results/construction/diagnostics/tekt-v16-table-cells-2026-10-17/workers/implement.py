from pathlib import Path
import json,datetime,subprocess,hashlib
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));tests=json.load(open(out/'tests-before-code.json'))
assert json.load(open(out/'red-result.json'))['functional_red']
texts=dict(tests['expected_files']);edits=[]
def change(source,old,new):
 s=texts[source];assert s.count(old)==1,(source,old[:180],s.count(old));texts[source]=s.replace(old,new,1);edits.append(dict(source=source,before=old,after=new))
for target in scope['targets']:
 if target['mode']=='snapshot':continue
 old=target['match_source'];at=target['pattern_start_byte']-target['match_source'].encode().find(b'match') if False else None
 # The single selected final arm, verified against its frozen full match.
 arm=target['arm_source'];assert old.count(arm)==1
 newarm=arm.replace(target['pattern_token'],target['replacement'],1)
 change(target['source'],old,old.replace(arm,newarm,1))
a,b,c,d=scope['sources'];C=scope['variants']['Content'];V=scope['variants']['Value']
def complement(domain,excluded):return ' | '.join(v for k,v in domain.items() if k not in excluded)
change(a,'''/// Par simétrico com `native_table_header` (P157C). Mesma decisão
/// arquitectural Caso D + DEBT-56 + naming flat. Implementação
/// idêntica linha-a-linha excepto naming `header → footer`.''','''/// Converte cada argumento em uma célula crua; preserva Cell explícita e
/// sua origem. Ao contrário do header, não promove o kind Auto.''')
for source in scope['sources']:
 old = next(line for line in texts[source].splitlines() if line.startswith('//! @updated '))
 change(source, old, '//! @updated 2026-10-17')

# Source-aware header preserves an existing cell and its causal origin.
change(a,'let mut cell = match content {','let origin = content.source_span();\n        let source_wrapped = matches!(&content, Content::Source(_));\n        let mut cell = match content.source_body() {')
change(a,'Content::TableCell(cell) => (*cell).clone(),','Content::TableCell(cell) => cell.as_ref().clone(),')
# Restrict edits to the header constructor, not other raw constructors.
s=texts[a];start=s.index('pub fn native_table_header(');end=s.index('\nfn table_header_levels',start);h=s[start:end]
h=h.replace('source_span: args.span,','source_span: if origin.is_detached() { args.span } else { origin },',1).replace('\n                body,','\n                body: content.clone(),',1)
old='''        if cell.kind == crate::entities::elements::table_cell::TableCellKind::Auto {
            cell.kind = crate::entities::elements::table_cell::TableCellKind::Header {
                level,
                scope: crate::entities::elements::table_cell::TableHeaderScope::Column,
            };
        }
        cell_values.push(Content::TableCell(std::sync::Arc::new(cell)));'''
new='''        if cell.source_span.is_detached() && !origin.is_detached() {
            cell.source_span = origin;
        }
        use crate::entities::elements::table_cell::TableCellKind;
        cell.kind = match cell.kind {
            TableCellKind::Auto => TableCellKind::Header {
                level,
                scope: crate::entities::elements::table_cell::TableHeaderScope::Column,
            },
            TableCellKind::Header { .. } | TableCellKind::Data => cell.kind,
        };
        let product = Content::TableCell(std::sync::Arc::new(cell));
        let product = if source_wrapped {
            Content::Source(std::sync::Arc::new(crate::entities::content::SourceContent { body: product, span: origin }))
        } else { product };
        cell_values.push(product);'''
assert h.count(old)==1;h=h.replace(old,new);change(a,s[start:end],h)
change(a,'fn collect(content: &Content, levels: &mut std::collections::BTreeSet<u32>) {\n        match content {','fn collect(content: &Content, levels: &mut std::collections::BTreeSet<u32>) {\n        match content.source_body() {')
change(a,'''                if let crate::entities::elements::table_cell::TableCellKind::Header {
                    level,
                    ..
                } = cell.kind
                {
                    levels.insert(level);
                }''','''                use crate::entities::elements::table_cell::TableCellKind;
                match cell.kind {
                    TableCellKind::Header { level, .. } => { levels.insert(level); }
                    TableCellKind::Auto | TableCellKind::Data => {}
                }''')
old='''        let classified = match v {
            Value::Content(c) => Value::Content(c.source_body().clone()),
            other => other.clone(),
        };
        match &classified {
            Value::Content(c) => cell_values.push(c.clone()),
            Value::Str(s)     => cell_values.push(Content::text(s.as_str())),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table_footer() espera content ou string como argumento posicional, recebeu {}", other.type_name()),
            )]),
        }'''
new='''        let content = match v {
            Value::Content(c) => c.clone(),
            Value::Str(s) => Content::text(s.as_str()),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table_footer() espera content ou string como argumento posicional, recebeu {}", other.type_name()),
            )]),
        };
        let origin = content.source_span();
        let source_wrapped = matches!(&content, Content::Source(_));
        if matches!(content.source_body(), Content::TableCell(_)) {
            cell_values.push(content);
        } else {
            let Content::TableCell(mut cell) = Content::table_cell(content, None, None, None, None) else { unreachable!() };
            std::sync::Arc::make_mut(&mut cell).source_span = if origin.is_detached() { args.span } else { origin };
            let product = Content::TableCell(cell);
            let product = if source_wrapped {
                Content::Source(std::sync::Arc::new(crate::entities::content::SourceContent { body: product, span: origin }))
            } else { product };
            cell_values.push(product);
        }'''
change(a,old,new)
# Shared, shallow lookup projection. Invalid local values still shadow general values.
for target in scope['targets']:
 if target['mode']=='snapshot':change(b,target['match_source'],'captured_recipe(engine.styles)')
helper='''fn captured_recipe(styles: &crate::entities::style_chain::StyleChain) -> Option<Arc<crate::entities::content::ShowRecipeContent>> {
    let value = styles.custom("eval.table.local.recipe").or_else(|| styles.custom("eval.context.recipe"))?;
    let content = match value {
        Value::Content(content) => content,
        '''+complement(V,{'Content'})+''' => return None,
    };
    match content.source_body() {
        Content::ShowRecipe(recipe) => Some(recipe.clone()),
        '''+complement(C,{'ShowRecipe'})+''' => None,
    }
}

'''
change(b,'fn captured_rules(',helper+'fn captured_rules(')
change(b,'''        Content::TableCell(e) => {
            Ok(Content::TableCell(Arc::new(with_defaults(e, ctx, engine, false)?)))
        }''','''        Content::TableCell(e) => {
            let mut cell = with_defaults(e, ctx, engine, false)?;
            let origin = body.source_span();
            if cell.source_span.is_detached() && !origin.is_detached() {
                cell.source_span = origin;
            }
            Ok(Content::TableCell(Arc::new(cell)))
        }''')
old='''    if let Content::Sequence(items) = body {
        Ok(Content::Sequence(
            items
                .iter()
                .map(|c| argument_defaults(c, ctx, engine))
                .collect::<SourceResult<Vec<_>>>()?
                .into(),
        ))
    } else {
        argument_defaults(body, ctx, engine)
    }'''
new='''    let product = if let Content::Sequence(items) = body.source_body() {
        Content::Sequence(
            items
                .iter()
                .map(|c| argument_defaults(c, ctx, engine))
                .collect::<SourceResult<Vec<_>>>()?
                .into(),
        )
    } else {
        argument_defaults(body, ctx, engine)?
    };
    Ok(product.with_source_span(body.source_span()))'''
change(b,old,new)
# Normalize only the table projection; legacy Grid and the occupancy algorithm stay separate.
helper='''fn table_group_view(content: &Content) -> Content {
    match content.source_body() {
        Content::TableHeader(header) => {
            let mut header = header.as_ref().clone();
            header.body = header.body.source_body().clone();
            Content::TableHeader(std::sync::Arc::new(header))
        }
        Content::TableFooter(footer) => {
            let mut footer = footer.as_ref().clone();
            footer.body = footer.body.source_body().clone();
            Content::TableFooter(std::sync::Arc::new(footer))
        }
        '''+complement(C,{'TableHeader','TableFooter'})+''' => content.source_body().clone(),
    }
}

'''
change(c,'/// Normalize logical table groups in this single row-group owner.',helper+'/// Normalize logical table groups in this single row-group owner.')
change(c,'''    let headers = combine_cells(&[], e.header.as_ref(), None, cols)''','''    let header = e.header.as_ref().map(table_group_view);
    let footer = e.footer.as_ref().map(table_group_view);
    let headers = combine_cells(&[], header.as_ref(), None, cols)''')
change(c,'combine_cells(&[], None, e.footer.as_ref(), cols)','combine_cells(&[], None, footer.as_ref(), cols)')
change(c,'combine_cells(&body, e.header.as_ref(), e.footer.as_ref(), cols)','combine_cells(&body, header.as_ref(), footer.as_ref(), cols)')
change(d,'                Value::Func(f) => Transformation::Func(f),','                Value::None => Transformation::Content(Content::Empty),\n                Value::Func(f) => Transformation::Func(f),')
for source,text in texts.items():
 assert Path(source).read_text()==tests['expected_files'][source];Path(source).write_text(text)
 subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',source],check=True)
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),edits=edits,files={s:Path(s).read_text() for s in texts})
(out/'implementation-replay.json').write_text(json.dumps(result,indent=2)+'\n');print('implemented four prescribed owners',flush=True)

from pathlib import Path
import gzip,json,re,subprocess,hashlib,datetime
out=Path(__file__).resolve().parents[1];red=json.load(open(out/'selected-visual-red-result.json'));assert red['functional_red'] and not red['compiler_error'] and red['sources_unchanged'];state=json.load(gzip.open(out/'selected-visual-red-state.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in state['source_hashes'].items());src=Path('01_core/src/compiler/introspect/bibliography.rs');s=src.read_text();anchor='        // Preserve the realized outline body, which is private to its element.';assert s.count(anchor)==1;s=s.replace(anchor,'''        // Follow the same active view as TableCell::visual and its finalizer.
        // A coexisting standalone carrier may only hold observed properties.
        if let Content::TableCell(e) = node {
            let mut cell = e.as_ref().clone();
            if let Some(r) = &e.resolved {
                let mut plan = r.as_ref().clone();
                plan.visual = convert_refs(plan.visual, keys);
                cell.resolved = Some(std::sync::Arc::new(plan));
            } else if let Some(r) = &e.standalone {
                let mut plan = r.as_ref().clone();
                plan.visual = convert_refs(plan.visual, keys);
                cell.standalone = Some(std::sync::Arc::new(plan));
            } else {
                cell.body = convert_refs(cell.body, keys);
            }
            return Ok(Some(Content::TableCell(std::sync::Arc::new(cell))));
        }
'''+anchor);src.write_text(s)
trav=Path('01_core/src/entities/content/traversal.rs');s=trav.read_text();s=s.replace('/// Borrow static children in order. False prunes without evaluating callbacks.','/// Borrow static children; false prunes descendants. Dynamic stays opaque.');trav.write_text(s)
scope=json.load(open(out/'scope.json'));subprocess.run(['rustfmt','--edition','2021',*scope['sources']],check=True)
with gzip.open(out/'expected-sources.json.gz','wt') as f:json.dump({p:Path(p).read_text() for p in scope['sources']},f)
with gzip.open(out/'expected-owners.json.gz','wt') as f:json.dump({p:Path(p).read_text() for p in scope['owners'].values()},f)
(out/'selected-visual-application.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'prior_red':red,'source_hashes':{p:sha(p) for p in scope['sources']},'only_productive_change':'TableCell converter follows existing active visual; visitor doc clarified. Generic mappers unchanged.'},indent=2)+'\n');print('Selected visual repair installed after independent functional RED')

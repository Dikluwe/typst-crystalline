from pathlib import Path
import gzip,json,subprocess,hashlib,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));red=json.load(gzip.open(out/'red-state.json.gz'));result=json.load(open(out/'red-result.json'));assert result['functional_red'] and not result['compiler_error'];scope=json.load(open(out/'scope.json'));src='01_core/src/compiler/introspect/bibliography.rs';trav='01_core/src/entities/content/traversal.rs';entry='01_core/src/entities/elements/outline_entry.rs'
# Install all additional tests before the corresponding productive definitions.
tests={src:red['files'][src]+(out/'bibliography-extra-tests.rs').read_text(),trav:b['files'][trav].rstrip()[:-1]+(out/'traversal-extra-test.rs').read_text()+'\n}\n',entry:b['files'][entry]+r'''

#[cfg(test)]
mod static_prefix_ref_tests {
    use super::*;
    #[test]
    fn bibliography_static_prefix_ref_borrows_stored_content() {
        let direct = OutlineEntryElem::new(1, Content::text("H"), None, false);
        assert!(direct.prefix_ref().is_none());
        let entry = OutlineEntryElem::realized(1, Content::text("H"), None, false,
            Some(Content::text("prefix")), crate::entities::locator::Locator::new().next());
        assert!(std::ptr::eq(entry.prefix_ref().unwrap(), entry.prefix.as_ref().unwrap()));
        assert_eq!(entry.prefix().unwrap().plain_text(), "prefix");
    }
}
'''}
for p,s in tests.items():Path(p).write_text(s)
with gzip.open(out/'tests-before-code.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files=tests),f)
production=r'''pub(super) fn collect_bib_keys(content: &Content) -> std::collections::HashSet<String> {
    let mut keys = std::collections::HashSet::new();
    content.visit_content_static(&mut |node| {
        if matches!(node, Content::Document { .. } | Content::Asset { .. }) { return false; }
        if let Content::Bibliography(b) = node {
            keys.extend(b.entries.iter().map(|entry| entry.key.clone()));
        }
        true
    });
    keys
}

pub(super) fn convert_refs(content: Content, keys: &std::collections::HashSet<String>) -> Content {
    content.map_content_pre(&mut |node| {
        // A bundled document owns its own bibliography; do not cross its boundary.
        if matches!(node, Content::Document { .. } | Content::Asset { .. }) {
            return Ok(Some(node.clone()));
        }
        if let Content::Ref(r) = node {
            let key = r.name.to_string();
            if keys.contains(&key) { return Ok(Some(Content::cite(key, r.supplement.clone(), None))); }
        }
        // The private pre-order helper intentionally leaves Dynamic opaque.
        // Preserve the bibliographic converter's established dynamic delegation.
        if let Content::Dynamic(e) = node {
            let mut f = |c: &Content| Ok(Some(convert_refs(c.clone(), keys)));
            return e.dyn_map_content(&mut f).map(Some);
        }
        // Preserve the realized outline body, which is private to its element.
        if let Content::Outline(e) = node {
            let mut mapped = e.as_ref().clone();
            mapped.title = e.title.as_ref().map(|c| convert_refs(c.clone(), keys));
            if let Some(body) = e.realized_body() {
                mapped = mapped.with_realized_body(convert_refs(body.clone(), keys));
            }
            return Ok(Some(Content::Outline(std::sync::Arc::new(mapped))));
        }
        Ok(None)
    }).expect("convert_bib_refs_to_cites: pure traversal must not fail")
}

'''
start=tests[src].index('pub(super) fn collect_bib_keys');end=tests[src].index('#[cfg(test)]',start);texts=dict(tests);texts[src]=tests[src][:start]+production+tests[src][end:];texts[trav]=tests[trav].replace('impl Content {\n','impl Content {\n'+(out/'visitor-candidate.rs').read_text()+'\n',1);texts[entry]=tests[entry].replace('    pub fn prefix(&self) -> Option<Content> {','    pub(crate) fn prefix_ref(&self) -> Option<&Content> {\n        self.prefix.as_ref()\n    }\n\n    pub fn prefix(&self) -> Option<Content> {',1)
for p,s in texts.items():Path(p).write_text(s)
subprocess.run(['rustfmt','--edition','2021',*scope['sources']],check=True)
with gzip.open(out/'expected-sources.json.gz','wt') as f:json.dump({p:Path(p).read_text() for p in scope['sources']},f)
print('Installed three internal consumers after functional RED')

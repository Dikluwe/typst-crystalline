from pathlib import Path
import json,subprocess,re,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');scope=json.load(open(out/'scope.json'));walk=scope['sources'][0];outline='01_core/src/compiler/introspect/outline_entries.rs';registries=['01_core/src/entities/counter_registry.rs','01_core/src/entities/state_registry.rs']
def run(label,cmd,expected):
 print(label,flush=True)
 with (out/('order-refinement-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode==expected,(label,r.returncode)
run('fmt',['cargo','fmt','--all'],0);run('seal-red',['python3',str(out/'workers/seal_owned.py'),walk,outline,*registries],0);run('functional-red',['cargo','test','--offline','--locked','-p','typst-core','--lib','v16_walk_','--','--test-threads=1'],101);assert '10 passed; 4 failed;' in (out/'order-refinement-functional-red.log').read_text()
for source in registries:
 p=Path(source);p.write_text(p.read_text()+'''
#[cfg(test)]
mod v16_walk_order_contract_tests {
    use super::*;
    use crate::entities::locator::Locator;

    #[test]
    fn v16_walk_order_block_preserves_identity_density_and_idempotence() {
        let mut allocator = Locator::new();
        let later = allocator.next();
        let anchor = allocator.next();
        let inserted = allocator.next();
        let unknown = allocator.next();
        let next = allocator.next();
        let mut registry = SelfRegistry::empty();
        registry.observe_location(anchor);
        registry.observe_location(later);
        registry.observe_locations_after(anchor, &[inserted, inserted, anchor, later]);
        assert_eq!(registry.order_key(anchor), 0);
        assert_eq!(registry.order_key(inserted), 1);
        assert_eq!(registry.order_key(later), 2);
        let previous = registry.document_order.clone();
        registry.observe_locations_after(anchor, &[inserted]);
        registry.observe_locations_after(unknown, &[next]);
        assert_eq!(registry.document_order, previous);
        registry.observe_location(next);
        assert_eq!(registry.order_key(next), 3);
    }
}
'''.replace('SelfRegistry', 'CounterRegistry' if 'counter_registry' in source else 'StateRegistry'))
run('contract-red',['cargo','test','--offline','--locked','-p','typst-core','--lib','v16_walk_','--no-run'],101);log=(out/'order-refinement-contract-red.log').read_text();assert log.count('error[E0599]')==6 and 'observe_locations_after' in log
method='''    /// Register a fresh causal block without changing existing identities or histories.
    pub(crate) fn observe_locations_after(&mut self, anchor: Location, locations: &[Location]) {
        let Some(anchor_order) = self.document_order.get(&anchor).copied() else { return };
        let mut seen = std::collections::HashSet::new();
        let fresh: Vec<_> = locations.iter().copied()
            .filter(|location| !self.document_order.contains_key(location) && seen.insert(*location))
            .collect();
        if fresh.is_empty() { return; }
        for ordinal in self.document_order.values_mut() {
            if *ordinal > anchor_order { *ordinal += fresh.len(); }
        }
        for (offset, location) in fresh.into_iter().enumerate() {
            self.document_order.insert(location, anchor_order + offset + 1);
        }
    }

'''
for source in registries:
 p=Path(source);s=p.read_text();needle='    pub(crate) fn order_key(';at=s.index(needle);s=s[:at]+method+s[at:];p.write_text(s)
p=Path(walk);s=p.read_text();needle='        Content::OutlineEntry(_) => {}';assert s.count(needle)==1;s=s.replace(needle,'''        Content::OutlineEntry(_) => {
            if let Some(location) = emitted_loc {
                intr.kind_index.entry(ElementKind::OutlineEntry).or_default().push(location);
            }
        }''',1);p.write_text(s)
p=Path(outline);s=p.read_text();needle='    for outline in outlines {';assert s.count(needle)==1;s=s.replace(needle,'''    let outline_locations = intr.kind_index.get(&ElementKind::Outline).cloned().unwrap_or_default();
    for (outline_index, outline) in outlines.into_iter().enumerate() {''',1);needle='''        for ((_label, number, _body, level), source_location) in''';assert s.count(needle)==1;s=s.replace(needle,'''        let mut entry_locations = Vec::new();
        for ((_label, number, _body, level), source_location) in''',1);needle='''                .push(entry_location);''';assert s.count(needle)==1;s=s.replace(needle,needle+'\n            entry_locations.push(entry_location);',1);needle='''        }
    }
}

#[cfg(test)]''';assert s.count(needle)==1;s=s.replace(needle,'''        }
        if let Some(anchor) = outline_locations.get(outline_index).copied() {
            intr.counters.observe_locations_after(anchor, &entry_locations);
            intr.state.observe_locations_after(anchor, &entry_locations);
        }
    }
    let counters = &intr.counters;
    if let Some(entries) = intr.kind_index.get_mut(&ElementKind::OutlineEntry) {
        entries.sort_by_key(|location| counters.order_key(*location));
    }
}

#[cfg(test)]''',1);p.write_text(s)
r=json.load(open(out/'refinement.json'))
for source in [walk,outline,*registries]:
 p=Path(source);s=p.read_text();s=re.sub(r'^//! @updated .*$', '//! @updated 2026-10-17',s,count=1,flags=re.M);p.write_text(s);r['sources'][source]=s
r['order_refinement']={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'functional_RED':'10 passed; 4 failed;','new_helper_contract_RED':'six E0599: three call sites for each of two missing private helpers','scope_pairs':9};(out/'refinement.json').write_text(json.dumps(r,indent=2)+'\n')
run('fmt-green',['cargo','fmt','--all'],0);run('seal-green',['python3',str(out/'workers/seal_owned.py'),walk,outline,*registries],0);run('guard',['python3',str(out/'workers/guard.py')],0)
print('Kind registration and synthesized causal order implemented after RED',flush=True)

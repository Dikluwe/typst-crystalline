from pathlib import Path
import re,json,gzip,hashlib,datetime
P=Path('lab/results/v16-corrections-2026-10-19/batch-g/agents/core')
files=['01_core/src/compiler/realize/bare_math.rs','01_core/src/compiler/realize/mod.rs','01_core/src/entities/content/equality.rs','01_core/src/compiler/eval/operators/equality.rs']
sha=lambda b:hashlib.sha256(b).hexdigest()
before={n:{'sha256':sha(Path(n).read_bytes()),'text':Path(n).read_text()} for n in files}
assert not (P/'productive-before.json.gz').exists()
with gzip.open(P/'productive-before.json.gz','wt') as f:json.dump(before,f,indent=2)
src=Path('01_core/src/entities/content.rs').read_text().split('pub enum Content {',1)[1].split('\n}\n',1)[0]
variants=[];shapes={}
for name,opening in re.findall(r'^    ([A-Z]\w*)\s*([({,])',src,re.M):
 variants.append(name);shapes[name]=opening
assert len(variants)==120 and len(set(variants))==120
pat=lambda v,prefix:prefix+'::'+v+({'(':'(_)','{':' { .. }',',':''}[shapes[v]])
def union(exclude,prefix,indent):
 return ('\n'+' '*indent+'| ').join(pat(v,prefix) for v in variants if v not in exclude)
p=Path(files[0]);s=p.read_text()
old='        _ => false,\n';new='''        // These carriers do not prove absence; retain full transforming traversal.
        '''+union({'Empty','Text','Space','SpaceFromNewline','Equation','Located','Source','Styled','Sequence','ShowRecipe','Realized','TableCell','Table'},'Content',8)+' => false,\n';assert s.count(old)==1;s=s.replace(old,new)
old='''            Content::Label(e) if mathy(&e.body) => {
                Ok(Some(Content::equation(node.clone(), block(styles))))
            }
''';new='''            Content::Label(e) => {
                if mathy(&e.body) {
                    Ok(Some(Content::equation(node.clone(), block(styles))))
                } else {
                    // Ordinary labels keep their child insertion boundary.
                    Ok(None)
                }
            }
''';assert old in s;s=s.replace(old,new)
old='            _ => Ok(None),\n';new='''            // No special root handling: let the preorder owner visit children.
            // Proof-positive and supported Mathy roots already returned above.
            '''+union({'Equation','Located','Styled','ShowRecipe','Realized','Label','MathStyled'},'Content',12)+' => Ok(None),\n';assert s.count(old)==1;s=s.replace(old,new);p.write_text(s)
p=Path(files[1]);s=p.read_text();old='                _ => false,\n';new='''                // These direct-child carriers are not flattenable sequences.
                '''+union({'Sequence','Realized'},'Content',16)+' => false,\n';assert s.count(old)==1;s=s.replace(old,new)
old='            _ => {}\n';new='''            // No node-local reservation; the mapper still visits descendants.
            '''+union({'Located','ContextBlock','ShowRecipe','Realized'},'Content',12)+' => {}\n';assert s.count(old)==1;s=s.replace(old,new);p.write_text(s)
p=Path(files[2]);s=p.read_text();needle='            (Self::Heading(a), Self::Heading(b)) => a == b,\n';addition='''            (Self::HtmlElem(a), Self::HtmlElem(b)) => {
                a.tag == b.tag
                    && match (&a.attrs, &b.attrs) {
                        (None, None) => true,
                        (Some(a), Some(b)) => {
                            a.len() == b.len()
                                && a.iter().zip(b.iter()).all(|((ak, av), (bk, bv))| {
                                    ak == bk && av == bv
                                })
                        }
                        (None, Some(_)) | (Some(_), None) => false,
                    }
                    && a.body == b.body
            }
            (Self::HtmlFrame(a), Self::HtmlFrame(b)) => a == b,
            (Self::Title(a), Self::Title(b)) => a == b,
''';assert needle in s;s=s.replace(needle,needle+addition)
old='''            // Quirk pré-existente preservado (Lote 6 P321): Metadata/State/
            // StateUpdate NÃO têm arm — caem em `_ => false` (sempre desiguais).
''';new='''            // Preserve the three documented mechanical always-unequal cases.
            (Self::Metadata(_), Self::Metadata(_))
            | (Self::State(_), Self::State(_))
            | (Self::StateUpdate(_), Self::StateUpdate(_)) => false,
            (Self::ContextBlock(a), Self::ContextBlock(b)) => a == b,
''';assert old in s;s=s.replace(old,new)
old='            _ => false, // neutro: N16[β] — variantes de Content sem arm de PartialEq retornam false (sempre desiguais)\n';remaining=union({'Source','ShowRecipe','Located','Realized'},'Self',16);new='''            // Every same-kind policy is above; this closed product retains
            // heterogeneous inequality and forces review on either new axis.
            (
                '''+remaining+''',
                '''+remaining+''',
            ) => false,
''';assert old in s;s=s.replace(old,new);p.write_text(s)
p=Path(files[3]);s=p.read_text();old='''                Ok(Some(match node {
                    Content::Styled(styled) => {
                        let mut body = styled.body.clone();
                        while let Content::Styled(inner) = &body {
                            body = inner.body.clone();
                        }
                        Content::styled(
                            Box::new(body),
                            crate::entities::style::Styles::new(),
                        )
                    }
                    other => other.clone(),
                }))
''';new='''                Ok(Some(comparison_root(node)))
''';assert old in s;s=s.replace(old,new)
needle='/// Whether the selector\'s composition tree uses the P1339 element carrier.\n';helper='''/// Normalize only comparison roots, including a Located body's suppressed root.
/// This ephemeral value never leaves the language comparison operation.
fn comparison_root(content: &Content) -> Content {
    use crate::entities::content::{LocatedContent, RealizedContent, SourceContent};
    use std::sync::Arc;

    match content {
        Content::Source(e) => Content::Source(Arc::new(SourceContent {
            body: comparison_root(&e.body),
            span: e.span,
        })),
        Content::ShowRecipe(e) => {
            let mut projected = e.as_ref().clone();
            projected.body = comparison_root(&e.body);
            Content::ShowRecipe(Arc::new(projected))
        }
        Content::Located(e) => Content::Located(Arc::new(LocatedContent {
            body: comparison_root(&e.body),
            location: e.location,
        })),
        Content::Realized(e) => Content::Realized(Arc::new(RealizedContent {
            original: e.original.clone(),
            visual: comparison_root(&e.visual),
        })),
        Content::ContextBlock(e) => {
            let mut projected = e.as_ref().clone();
            // Context has no exposed language fields. Keep its nominal kind
            // and unevaluated closure; omit only the private comparison id.
            projected.id = 0;
            Content::ContextBlock(Arc::new(projected))
        }
        Content::Styled(styled) => {
            let mut body = styled.body.clone();
            while let Content::Styled(inner) = &body {
                body = inner.body.clone();
            }
            Content::styled(Box::new(body), crate::entities::style::Styles::new())
        }
        other => other.clone(),
    }
}

''';assert needle in s;s=s.replace(needle,helper+needle);p.write_text(s)
after={n:{'sha256':sha(Path(n).read_bytes()),'text':Path(n).read_text()} for n in files}
test_re=re.compile(r'(?m)^#\[cfg\(test\)\]')
checks={}
for n in files:
 old=before[n]['text'];new=after[n]['text'];old_tests=old[test_re.search(old).start():];new_tests=new[test_re.search(new).start():];assert old_tests==new_tests,n
 checks[n]={'before_sha256':before[n]['sha256'],'after_sha256':after[n]['sha256'],'all_cfg_test_suffix_byte_identical':True,'test_suffix_sha256':sha(new_tests.encode())}
with gzip.open(P/'productive-after.json.gz','wt') as f:json.dump(after,f,indent=2)
(P/'implementation-receipt.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'files':checks,'no_cargo_fmt_seal':True,'no_public_signature_change':True,'enum_count':120},indent=2)+'\n')
print(json.dumps(checks,indent=2))

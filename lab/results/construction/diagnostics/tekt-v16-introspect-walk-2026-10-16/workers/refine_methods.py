from pathlib import Path
import json,gzip,re,datetime,hashlib
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();source='01_core/src/compiler/eval/call_dispatch/content_methods.rs';owner='00_nucleo/prompts/compiler/eval/call_dispatch/content_methods.md';before=json.load(gzip.open(out/'before.json.gz'));assert sha(source)==before['hashes'][source] and sha(owner)==before['hashes'][owner]
before['files'].update({p:Path(p).read_text() for p in [source,owner]});before['methods_supplement_utc']=now()
with gzip.open(out/'before.json.gz','wt') as f:json.dump(before,f)
with gzip.open(out/'methods-before.json.gz','wt') as f:json.dump({'utc':now(),'validation':json.load(open(out/'validation-result.json')),'sources':{s:Path(s).read_text() for s in json.load(open(out/'scope.json'))['sources']},'new_owner':Path(owner).read_text()},f)
for p in list(out.glob('validation-*'))+list(out.glob('probe-green*'))+[out/'measurement-after',out/'measurement-after.json']:
 if p.exists():p.rename(out/('attempt-method-carriers-'+p.name))
(out/'attempt-method-carriers-validation-worker.py').write_text((out/'workers/validate.py').read_text())
r=json.load(open(out/'refinement.json'));entity_owner='00_nucleo/prompts/entities/elements/outline_entry.md'
p=Path(owner);s=p.read_text();s=re.sub(r'^(Hash do Código: [^\n]+)',r'\1\n\nNúcleos Tekt:\n- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67',s,count=1,flags=re.M);s+='''\n\n## OutlineEntry com carriers de origem/identidade — 2026-10-16\n\n### Medição anterior à decisão\n\nHEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, métodos/fontes/UTC/hashes/diffstat em before.json.gz e methods-before.json.gz no diagnóstico tekt-v16-introspect-walk-2026-10-16. A primeira correção pura passou nove controles e seis barreiras, mas a sonda CLI direct.typ produziu element entry has no method body, enquanto o vanilla pinado 9f2b6e871 aceita. attempt-method-carriers-measurement-after.json conserva argv, binários, fontes e saídas: consulta direta ainda 0 contra 1 e mista 1 contra 2; não foram declaradas resolvidas. call_dispatch/content_methods.rs:68 exige variante OutlineEntry bruta. rules/apply.rs:504/564 fornece Value::Content do Located criado para uma ocorrência nativa; Source também é transparente e não pode impedir o método. Não se infere paridade global nem intenção upstream dos mecanismos de armazenamento.\n\n### Contrato exclusivo\n\nAdicionar helper privado emprestado outline_entry_receiver: normalizar Source pelo source_body existente, recursar em Located, devolver Some da referência OutlineEntry nua e None para o complemento nominal das 118 variantes atuais. Sem clone, alocação, avaliação ou uso de Location para escolher semântica. Realized/ShowRecipe/Styled/Strong não são novos unwraps por este helper. No interceptor, após o match Value atual, usar esse helper para reconhecer o receiver e passar a entry nua aos mesmos métodos. Preservar gates de nomes, ordem e número de avaliações de target/args, spans, dispatch e fallthrough de outros tipos; somente o conteúdo da identificação do receiver muda. Não criar API/campo/trait/default/fase ou mudar assinaturas; correção interna/paridade ADR-0127.\n\nO complemento nominal tem consequência None uniforme e mantém V19 informativo, sem supressão; não prova execução de todas as alternativas. Uma nova variante de Content exige rever esse match, com sonda sentinela real. A responsabilidade de body/prefix/fill e Location é dos owners existentes, sem nova tabela de métodos.\n\n### Aceitação\n\nTestes co-localizados antes do código: Entry crua, Source/Located em ambas as ordens e aliases resolvem a mesma referência; outros tipos e Realized não são convertidos. Os controles e as seis sondas CLI originais permanecem, inclusive consultas direta/sintetizada/mista e body, sem relaxar os esperados. Sonda Content deve falhar também neste novo helper. Build/fmt/Clippy estrito/Rustdoc estrito/workspace/linhagem V1/V5/V15/V26/reversa/diferencial e 12 GiB, swap zero, um job; resselo só do escopo ampliado para seis pares. V16 esperado 239→233; V19 900→907 (seis grupos anteriores e este novo), três mensagens de grupos já existentes ajustadas; V20 sem aumento. O objetivo geral e as sondas CLI seguem abertos até GREEN real.\n''';p.write_text(s);r['owners'][owner]=s
p=Path(entity_owner);s=p.read_text()+'''\n\n### Complemento de transparência — Located em element\n\nO novo caminho produtivo atribui Located às ocorrências; a entidade aceita Content de heading e a Location é carrier de identidade, não novo elemento da linguagem. Acrescentar teste anterior à mudança com Located(Source(Heading)) e Source(Located(Heading)), conservando o body semântico Strong e o campo element intacto. Se refutar o match atual, body deve percorrer somente Source/Located por referências até a variante semântica, conservando a decisão Heading ou o mesmo erro de outros tipos. Não unwrap Realized/Strong/Styled nem fabricar Location/prefix. O RED desse complemento é registrado separadamente do RED inicial; nenhuma mudança de interface ou fase.\n''';p.write_text(s);r['owners'][entity_owner]=s
scope=json.load(open(out/'scope.json'));scope['sources'].append(source);scope['owners'].append(owner);scope['additional_group']={'source':source,'function':'outline_entry_receiver','excluded':['Located','OutlineEntry'],'group_size':118};(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n')
tests=json.load(gzip.open(out/'tests-and-l0.json.gz'));tests.update({p:before['files'][p] for p in [source,owner]})
with gzip.open(out/'tests-and-l0.json.gz','wt') as f:json.dump(tests,f)
(out/'refinement.json').write_text(json.dumps(r,indent=2)+'\n')
p=Path('01_core/src/entities/elements/outline_entry.rs');s=p.read_text();at=s.rfind('\n}');s=s[:at]+'''
    #[test]
    fn v16_walk_entry_body_preserves_located_source_heading() {
        use crate::entities::{content::LocatedContent, locator::Locator};
        let body = Content::strong(Content::text("semantic"));
        let heading = Content::heading(2, body.clone());
        let located = |content| Content::Located(Arc::new(LocatedContent {
            location: Locator::new().next(), body: content,
        }));
        for element in [located(walk_source(heading.clone())), walk_source(located(heading))] {
            let entry = OutlineEntryElem::new(2, element.clone(), None, false);
            assert_eq!(entry.body().unwrap(), body);
            assert_eq!(entry.element, element);
        }
    }
'''+s[at:];p.write_text(s)
p=Path(source);p.write_text(p.read_text()+'''
#[cfg(test)]
mod v16_walk_receiver_tests {
    use super::*;
    use crate::entities::{content::{Content, LocatedContent, RealizedContent}, locator::Locator, file_id::FileId, span::Span};
    use std::sync::Arc;

    fn source(body: Content) -> Content {
        body.with_source_span(Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()), 2..5))
    }
    fn located(body: Content) -> Content {
        Content::Located(Arc::new(LocatedContent { location: Locator::new().next(), body }))
    }

    #[test]
    fn v16_walk_receiver_resolves_borrowed_entry_through_source_and_located() {
        let entry = Content::outline_entry(1, Content::heading(1, Content::text("h")), None);
        for content in [entry.clone(), source(located(entry.clone())), located(source(entry))] {
            let resolved = outline_entry_receiver(&content).expect("entry receiver lost");
            assert!(matches!(resolved, Content::OutlineEntry(_)));
            assert!(std::ptr::eq(resolved, outline_entry_receiver(&content).unwrap()));
        }
    }

    #[test]
    fn v16_walk_receiver_does_not_unwrap_other_semantic_nodes() {
        let entry = Content::outline_entry(1, Content::heading(1, Content::text("h")), None);
        for content in [Content::text("x"), source(located(Content::strong(entry.clone()))),
            Content::Realized(Arc::new(RealizedContent { original: entry, visual: Content::text("v") }))] {
            assert!(outline_entry_receiver(&content).is_none());
        }
    }
}
''')
# RED must exercise actual old recognition, represented by a temporary helper with its same raw-only rule.
s=p.read_text();s+='''
#[cfg(test)]
fn outline_entry_receiver(content: &crate::entities::content::Content) -> Option<&crate::entities::content::Content> {
    if let crate::entities::content::Content::OutlineEntry(_) = content { Some(content) } else { None }
}
''';p.write_text(s)
p=out/'workers/validate.py';p.write_text(p.read_text().replace('9 passed; 0 failed;','12 passed; 0 failed;'))
print('Two receiver tests and one Located-element test prescribed before implementation; raw-only recognition helper is test-only RED scaffold')

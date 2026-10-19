from pathlib import Path
import json,gzip,re,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();scope=json.load(open(out/'scope.json'));before=json.load(gzip.open(out/'before.json.gz'));r=json.load(open(out/'refinement.json'));tests=json.load(gzip.open(out/'tests-and-l0.json.gz'))
with gzip.open(out/'order-before.json.gz','wt') as f:json.dump({'utc':now(),'files':{s:Path(s).read_text() for s in scope['sources']+scope['owners']},'validation':json.load(open(out/'validation-result.json'))},f)
for p in list(out.glob('validation-*'))+list(out.glob('probe-green*'))+[out/'measurement-after',out/'measurement-after.json']:
 if p.exists():p.rename(out/('attempt-kind-index-'+p.name))
new_sources=['01_core/src/compiler/introspect/outline_entries.rs','01_core/src/entities/counter_registry.rs','01_core/src/entities/state_registry.rs']
for source in new_sources:
 owner=re.search(r'^//! @prompt (.+)$',Path(source).read_text(),re.M).group(1)
 for p in [source,owner]:assert sha(p)==before['hashes'][p];before['files'][p]=Path(p).read_text();tests[p]=Path(p).read_text()
 scope['sources'].append(source);scope['owners'].append(owner)
before['order_supplement_utc']=now()
with gzip.open(out/'before.json.gz','wt') as f:json.dump(before,f)
with gzip.open(out/'tests-and-l0.json.gz','wt') as f:json.dump(tests,f)
measurement='HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; order-before.json.gz/before.json.gz preservam estado, UTC, hashes e diffstat. trace-runtime-result.json e trace-runtime.log mostram a entry direta no índice puro, runtime e L3 sob NativeElement; a linguagem bare converte outline.entry em Kind(OutlineEntry), conforme foundations/selector.rs:113. O walk terminal atual não preenche esse kind. measurement-order-before.json no pin 9f2b6e8715237cb086899a42873660fe744622e8 mede BARE [Heading] versus [Heading,Direct], FILTERED [Direct,Heading] versus [Heading,Direct], STATE 2 versus 1 e COUNTER ((2,),) versus ((1,),) na Location da entrada sintetizada. Esses são observáveis de linguagem; não se exige igualdade Rust/bytes. A síntese pós-walk aloca entradas depois dos updates, mas sua posição causal é dentro do outline anterior. O núcleo realization-order já proíbe tratar identidade numérica como ordem. A correção é refutada por ordem, prefixo temporal, duplicação ou dados de entry errados.'
contracts={
'00_nucleo/prompts/compiler/introspect/walk.md':'No ramo OutlineEntry terminal, registrar emitted_loc no kind_index OutlineEntry quando presente. Captura de payload/Start/End continua no caminho nativo existente; não duplicar ação ou tag nem percorrer element/fill. Preservar identidade própria e aliases preparados. Cobrir query Kind e NativeElement nos testes anteriores, sem trocar seletor de linguagem para contornar o índice incompleto.',
'00_nucleo/prompts/compiler/introspect/outline_entries.md':'Conservar a síntese na fase pós-walk, os mesmos campos, filtros, depth, prefix, fill, Location própria e Location causal do heading. Usar a ordem dos outlines em outline_elements alinhada a kind_index Outline para identificar a âncora real por índice, sem igualdade Rust/nomes/hashes de conteúdo e sem truncar a síntese. Colher as Locations recém-geradas por outline e registrar sua ordem imediatamente após a Location do outline, na ordem dos headings, por helpers internos de CounterRegistry e StateRegistry. Preservar todas as Locations e a ordem relativa das tags existentes; apenas a posição causal das ocorrências sintetizadas é integrada. Se faltar âncora em snapshot legado parcial, conservar o fallback anterior, sem omitir entradas. Ao fim, ordenar o kind_index OutlineEntry pela ordem causal registrada, tornando bare/where/nativo consistentes, inclusive múltiplos outlines e uma entry direta entre eles. Não reconstruir ou reexecutar o documento, não gerar nós extras, mudar fase, cardinalidade ou fórmula de campos.',
'00_nucleo/prompts/entities/counter_registry.md':'Adicionar somente pub(crate) fn observe_locations_after(&mut self, anchor: Location, locations: &[Location]). Sem API externa/campo/método de trait novo. Se anchor não foi observado, não alterar o registry; lista vazia ou já observada é no-op. Deduplicar novas Locations preservando sua ordem (HashSet puro local, não busca quadrática). Deslocar os ordinais já existentes estritamente posteriores à âncora pelo tamanho do bloco e registrar o bloco depois da âncora. Ordem densa, aliases/repetição idempotentes, ordem relativa antiga intacta; observe_location posterior continua anexando sem colidir. Não reordenar/mutar valores, ações, históricos ou document_locations, não executar callbacks nem importar compiler. O owner de introspect/outline_entries decide a âncora e o bloco; esta entidade só mantém dados de ordem, sob o núcleo realization-order pinado.',
'00_nucleo/prompts/entities/state_registry.md':'Adicionar somente pub(crate) fn observe_locations_after(&mut self, anchor: Location, locations: &[Location]), com semântica de dados: âncora não observada/lista vazia/já observada são no-op; deduplicar novas Locations em ordem por HashSet local puro, deslocar ordinais posteriores e registrar o bloco logo depois da âncora. Preservar densidade e ordem relativa dos antigos, idempotência, clones e append posterior. O compiler fornece a decisão de âncora, nunca inferir por número de Location. Não alterar init/update, histórico, lookup, valores, fases, I/O ou interface externa. Aplicar o núcleo realization-order já pinado, sem importar compiler.'}
for owner,note in contracts.items():
 p=Path(owner);s=p.read_text()+'\n\n## Entradas diretas e ordem da síntese — 2026-10-17\n\n### Medição anterior à decisão\n\n'+measurement+'\n\n### Responsabilidade exclusiva\n\n'+note+'\n\n### Aceitação\n\nTestes funcionais antes da mudança: índice bare da entry própria e aliases; ordem de três entries (dois outlines com entry direta entre eles), where e ausência de heading fantasma; estado e contador lidos na Location sintetizada devem refletir o prefixo do outline. Depois, testes de contrato dos dois helpers internos antes da sua implementação, com âncora desconhecida, duplicatas/repetição, IDs não monotônicos e append denso. Repetir nove sondas CLI sem relaxar resultados (seis originais e order/state/counter). Suíte completa, build, fmt, Clippy estrito, Rustdoc estrito, V1/V5/V15/V26/reversa/diff e SARIF: seis V16 removidos, sete V19 previstos e três mensagens existentes atualizadas, nenhum V20 novo. 12 GiB, swap zero, um job e comandos pesados sequenciais. Esta é correção interna/paridade ADR-0127, com helpers crate-private e nove pares L0/consumer; nenhuma assinatura externa, default deliberado ou fase nova. Resselo restrito, sem commit/push/release. As demais pendências de V16 continuam abertas.\n';p.write_text(s);r['owners'][owner]=s
(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n');(out/'refinement.json').write_text(json.dumps(r,indent=2)+'\n')
p=Path(scope['sources'][0]);s=p.read_text();needle='''        assert_eq!(intr.query(&entry_selector()), vec![*location]);''';assert s.count(needle)==1;s=s.replace(needle,needle+'\n        assert_eq!(intr.query(&Selector::Kind(ElementKind::OutlineEntry)), vec![*location]);',1);needle='''        assert_eq!(entries.len(), 2);''';assert s.count(needle)==1;s=s.replace(needle,needle+'\n        assert_eq!(intr.query(&Selector::Kind(ElementKind::OutlineEntry)).len(), 2);',1);p.write_text(s)
p=Path('01_core/src/compiler/introspect/outline_entries.rs');p.write_text(p.read_text()+'''
#[cfg(test)]
mod v16_walk_order_tests {
    use super::*;
    use crate::entities::{func::Func, state_update::StateUpdate, counter_update::CounterUpdate};

    fn selector() -> Selector {
        Selector::NativeElement(Func::native("outline.entry", crate::compiler::stdlib::native_outline_entry))
    }

    #[test]
    fn v16_walk_synthesized_entries_keep_outline_and_direct_entry_order() {
        let tree = Content::sequence(vec![Content::outline(), Content::heading(1, Content::text("real")),
            Content::outline_entry(1, Content::heading(1, Content::text("direct")), None), Content::outline()]);
        let intr = crate::compiler::introspect::introspect_with_introspector(&tree);
        let entries = intr.query(&selector());
        let bodies: Vec<_> = entries.iter().map(|location| {
            let Content::OutlineEntry(entry) = intr.elements[location].content().source_body() else {panic!("entry lost")};
            entry.body().unwrap().plain_text()
        }).collect();
        assert_eq!(bodies, ["real", "direct", "real"]);
        assert_eq!(intr.query(&Selector::Kind(ElementKind::OutlineEntry)), entries);
        let where_selector = Selector::Element { function: Func::native("outline.entry", crate::compiler::stdlib::native_outline_entry), fields: vec![("level".into(), Value::Int(1))] };
        assert_eq!(intr.query(&where_selector), entries);
        assert_eq!(entries.iter().collect::<std::collections::HashSet<_>>().len(), 3);
    }

    #[test]
    fn v16_walk_synthesized_entry_reads_state_and_counter_at_its_outline() {
        let key = CounterKey::Str("probe".into());
        let tree = Content::sequence(vec![
            Content::state("probe", Value::Int(0)),
            Content::state_update("probe", StateUpdate::Set(Box::new(Value::Int(1)))),
            Content::counter_update(key.clone(), CounterUpdate::Set(vec![1])), Content::outline(),
            Content::state_update("probe", StateUpdate::Set(Box::new(Value::Int(2)))),
            Content::counter_update(key.clone(), CounterUpdate::Set(vec![2])), Content::heading(1, Content::text("h")),
        ]);
        let intr = crate::compiler::introspect::introspect_with_introspector(&tree);
        let entries = intr.query(&selector());
        assert_eq!(entries.len(), 1);
        assert_eq!(intr.state.value_at("probe", entries[0]), Some(&Value::Int(1)));
        assert_eq!(intr.counters.value_at(&key, entries[0]), Some([1].as_slice()));
        assert_eq!(intr.state.final_value("probe"), Some(&Value::Int(2)));
        assert_eq!(intr.counters.value(&key), Some([2].as_slice()));
    }
}
''')
p=out/'workers/validate.py';p.write_text(p.read_text().replace('12 passed; 0 failed;','16 passed; 0 failed;'))
print('Nine owners prescribed; two functional order tests plus Kind assertions written before correction')

from pathlib import Path
import json,gzip,datetime,hashlib
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');source='01_core/src/compiler/introspect/walk.rs';owner='00_nucleo/prompts/compiler/introspect/walk.md';now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
with gzip.open(out/'source-carrier-before.json.gz','wt') as f:json.dump({'utc':now(),'source':Path(source).read_text(),'owner':Path(owner).read_text(),'validation':json.load(open(out/'validation-result.json')),'source_sha256':sha(source)},f)
for p in list(out.glob('validation-*')):p.rename(out/('attempt-source-carrier-'+p.name))
(out/'attempt-source-carrier-validation-worker.py').write_text((out/'workers/validate.py').read_text())
p=Path(owner);s=p.read_text()+'''\n\n### Refinamento medido — preservar contexto ao atravessar Source\n\nsource-carrier-before.json.gz preserva a primeira implementação e o teste RED de alias: query nativa devolve Locations 1 e 2 em vez da Location explícita 0. Sete controles passaram, um falhou em attempt-source-carrier-validation-controls.log. O ramo Source em walk_at chama walk, que reinicializa forced_location=None e shallow_table=false. É perda de contexto pelo wrapper causal, não duas ocorrências de linguagem.\n\nPreservar forced_location e shallow_table ao delegar o body de Source: usar walk_at com os mesmos acumuladores, chain, label e os dois valores recebidos. Não observar/alocar nova Location nesse wrapper, não alterar os outros carriers ou assinatura. Acrescentar antes desta correção o controle de Source(Table) no modo shallow com Location forçada: um único Start/End da tabela, sem Heading do corpo. O alias Located(Source(entry)) e o controle raso devem passar depois; as reservas e os corpos nominais W1–W6 permanecem. Isso restaura o núcleo de origem já pinado, na mesma fase, sem contrato público/default novos. O RED inicial de sete falhas é anterior às correções; este refinamento tem seu RED próprio registrado, sem reescrever a evidência original.\n''';p.write_text(s)
p=Path(source);s=p.read_text();s+='''
#[cfg(test)]
mod v16_walk_source_context_tests {
    use super::*;
    use crate::entities::{file_id::FileId, span::Span};

    #[test]
    fn v16_walk_source_table_preserves_forced_location_and_shallow_mode() {
        let table = Content::table(vec![], vec![], vec![Content::heading(1, Content::text("hidden"))])
            .with_source_span(Span::from_range(
                FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()), 2..5,
            ));
        let forced = Locator::new().next();
        let mut locator = Locator::new();
        let _ = locator.next();
        let _ = locator.next();
        let mut tags = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto = 0;
        walk_at(&table, &mut locator, &mut tags, &mut intr, &mut auto,
            None, &StyleChain::default_chain(), None, Some(forced), true);
        assert_eq!(tags.len(), 2);
        assert!(matches!(&tags[0], Tag::Start(location, _) if *location == forced));
        assert!(matches!(&tags[1], Tag::End(location, _) if *location == forced));
        assert!(!intr.kind_index.contains_key(&ElementKind::Heading));
        assert_eq!(auto, 0);
    }
}
''';p.write_text(s)
(out/'refinement.json').write_text(json.dumps({'utc':now(),'owners':{owner:Path(owner).read_text()},'sources':{}},indent=2)+'\n')
p=out/'workers/validate.py';s=p.read_text().replace('8 passed; 0 failed;','9 passed; 0 failed;');p.write_text(s)
print('L0 refinement and shallow Source test written before its implementation')

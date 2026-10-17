from pathlib import Path
import json,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');source='02_shell/src/cli.rs';owner='00_nucleo/prompts/shell/cli.md';now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();before=json.load(gzip.open(out/'before.json.gz'));assert sha(source)==before['hashes'][source] and sha(owner)==before['hashes'][owner]
with gzip.open(out/'shell-before.json.gz','wt') as f:json.dump({'utc':now(),'validation':json.load(open(out/'validation-result.json')),'measurement':json.load(open(out/'measurement-after.json')),'source':Path(source).read_text(),'owner':Path(owner).read_text()},f)
for p in list(out.glob('validation-*'))+list(out.glob('probe-green*'))+[out/'measurement-after',out/'measurement-after.json']:
 if p.exists():p.rename(out/('attempt-shell-projection-'+p.name))
before['files'].update({p:Path(p).read_text() for p in [source,owner]});before['shell_supplement_utc']=now()
with gzip.open(out/'before.json.gz','wt') as f:json.dump(before,f)
tests=json.load(gzip.open(out/'tests-and-l0.json.gz'));tests.update({p:Path(p).read_text() for p in [source,owner]})
with gzip.open(out/'tests-and-l0.json.gz','wt') as f:json.dump(tests,f)
scope=json.load(open(out/'scope.json'));scope['sources'].append(source);scope['owners'].append(owner);(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n')
p=Path(owner);s=p.read_text()+'''\n\n## Campos públicos de OutlineEntry na projeção legada — 2026-10-17\n\n### Medição anterior à decisão\n\nHEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/hashes/diffstat/fontes em before.json.gz e shell-before.json.gz do diagnóstico tekt-v16-introspect-walk-2026-10-16. A última medição contra vanilla pinado 9f2b6e871 confirma body(), índice bare/where, ordem e leituras state/counter embutidos corretos no Tekt. O subcomando query --field level ainda devolve [] para a entry direta e [1] no misto, versus [1] e [1,1]. cli.rs:1121 filtra valores sem campo; content_to_semantic em :1185–1400 não tem ramo OutlineEntry e o fallback body não fornece level/element/fill. Snapshots sintetizados Some já contêm esses campos; o valor direto sem snapshot usa a projeção legada incompleta. Isto é defeito de apresentação, não nova perda de cardinalidade no índice.\n\n### Responsabilidade exclusiva e aceitação\n\nAcrescentar apenas o ramo Content::OutlineEntry(entry) na projeção legada: level inteiro por entry.level as i64 (mesma representação já fornecida pelo getter do domínio), element por content_to_semantic(&entry.element), fill por conversão recursiva do Some e Null para None. Conservar o func canônico entry já existente, ordem level/element/fill, transparência Source/Located/Label dos caminhos atuais, JSON/YAML e seleção --field/--one. Não criar snapshot L1 só para contornar esta omissão nem alterar Some: snapshot presente, inclusive vazio, continua autoritativo e não recebe defaults/campos pelo fallback. Serialização aninhada conserva o converter vigente; esta correção não certifica todos os campos de outras famílias.\n\nTestes co-localizados antes do ramo: valores crus e wrappers com body morfológico e fill:none; query LocatedContent(None) com label e --field level em JSON/YAML e --one; Some(empty)/Some(level distinto) permanecem exatos. RED funcional real deve falhar nos dois primeiros testes e manter o controle Some passando. Repetir as nove sondas CLI sem relaxar seus esperados, incluindo query --field e dados embutidos. API externa, structs, parser, flags/defaults, fases e política de snapshots ficam iguais; tabela interna/paridade em fluxo ADR-0127. Resselo somente deste par adicional (dez owners totais no lote). Gates completos: build/workspace/fmt/Clippy estrito/Rustdoc estrito/linhagem estrita/reversa/SARIF diferencial sem mudança extra; 12 GiB, swap zero, um job. Sete V19 prescritos e três mensagens existentes ajustadas, seis V16 removidos; os três V16 já presentes deste consumer não são silenciados. Sem novo V20, commit, push ou release.\n''';p.write_text(s);r=json.load(open(out/'refinement.json'));r['owners'][owner]=s;(out/'refinement.json').write_text(json.dumps(r,indent=2)+'\n')
p=Path(source);p.write_text(p.read_text()+'''
#[cfg(test)]
mod v16_walk_entry_serialization_tests {
    use super::*;
    use typst_core::entities::{content::{Content, LocatedContent, SourceContent}, value::IntrospectedContent, locator::Locator, file_id::FileId, span::Span};
    use std::sync::Arc;

    fn entry() -> Content {
        Content::outline_entry(2, Content::heading(2, Content::strong(Content::text("semantic"))), None)
    }
    fn located_value(fields: Option<indexmap::IndexMap<ecow::EcoString, Value, rustc_hash::FxBuildHasher>>) -> Value {
        Value::LocatedContent(IntrospectedContent::new(Content::label_auto("direct", entry()), fields), Locator::new().next())
    }

    #[test]
    fn v16_walk_cli_entry_fields_survive_legacy_wrappers() {
        let sourced = Content::Source(Arc::new(SourceContent { body: entry(), span: Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()), 2..5) }));
        let located = Content::Located(Arc::new(LocatedContent { location: Locator::new().next(), body: sourced }));
        for content in [entry(), located, Content::label_auto("direct", entry())] {
            let bytes=serialize_eval(&Value::Content(content), EvalFormat::Json, false).unwrap();
            let json:serde_json::Value=serde_json::from_slice(&bytes).unwrap();
            assert_eq!(json["func"], "entry");
            assert_eq!(json["level"], 2);
            assert_eq!(json["element"]["func"], "heading");
            assert_eq!(json["element"]["body"]["func"], "strong");
            assert_eq!(json["element"]["body"]["body"]["text"], "semantic");
            assert!(json["fill"].is_null());
        }
    }

    #[test]
    fn v16_walk_cli_entry_query_field_none_snapshot_keeps_json_yaml_and_one() {
        let value = located_value(None);
        for (format, expected) in [(QueryFormat::Json, "[2]\\n"), (QueryFormat::Yaml, "- 2\\n")] {
            let bytes=serialize_query_with_format(std::slice::from_ref(&value), Some("level"), false, false, format).unwrap();
            assert_eq!(String::from_utf8(bytes).unwrap(),expected);
            let bytes=serialize_query_with_format(std::slice::from_ref(&value), Some("level"), true, false, format).unwrap();
            assert_eq!(String::from_utf8(bytes).unwrap(),"2\\n");
        }
    }

    #[test]
    fn v16_walk_cli_entry_present_snapshots_remain_authoritative() {
        let empty=located_value(Some(Default::default()));
        assert_eq!(serialize_query(&[empty], Some("level"), false, false).unwrap(), b"[]\\n");
        let value=located_value(Some([("level".into(), Value::Int(7))].into_iter().collect()));
        assert_eq!(serialize_query(&[value], Some("level"), false, false).unwrap(), b"[7]\\n");
    }
}
''')
p=out/'workers/validate.py';s=p.read_text();needle="('probe',['python3',str(out/'workers/probe.py'),'green'],0)";assert s.count(needle)==1;s=s.replace(needle,"('shell-controls',['cargo','test','--offline','--locked','-p','typst-shell','--lib','v16_walk_','--','--test-threads=1'],0),"+needle);p.write_text(s)
print('Shell L0 and three tests written before legacy field projection correction')

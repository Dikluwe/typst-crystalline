from pathlib import Path
import json,gzip,hashlib,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));before=json.load(open(out/'before-probe-result.json'));assert before['structural_guarded_sites']==0;assert all(Path(p).read_text()==b['files'][p] for p in scope['sources']);n=Path(scope['nuclei'][0]);n.write_text('''tekt = 1
kind = "nucleus"
id = "ssty-math-size"
title = "Projeção nominal do nível ssty"

[[claims]]
id = "levels"
level = "must"
statement = "Para entradas elegíveis em modo math fora de TextItem, MathSize::Script solicita ssty 1, ScriptScript solicita 2 e Display/Text não solicitam nível ssty (None)."

[[claims]]
id = "gates"
level = "must"
statement = "Fora de math, em math_text_item ou sem elegibilidade, conservar None antes da seleção por tamanho. Shaping usa o predicado canônico ssty_eligible_text; o owner de métricas recebe a elegibilidade já decidida."

[[claims]]
id = "nominal-complement"
level = "must"
statement = "O match sobre MathSize enumera nominalmente Display, Text, Script e ScriptScript, sem wildcard. Display e Text podem compartilhar None (V19 uniforme); ambos precisam de controles. Uma variante futura exige revisão explícita pelo compilador."

[[claims]]
id = "boundary"
level = "must-not"
statement = "Este reparo não altera MathSize, TextStyle, API, default, fases, elegibilidade, fallback, script OpenType, GSUB, features, caches ou seleção de fontes. Não criar predicado paralelo nem silenciar V16/V19."
''');effective=hashlib.sha256(n.read_bytes()+b'\x00TEKT-NUCLEUS-DEPS-V1\x00').hexdigest()
common='''

## V16 — projeção nominal de MathSize para ssty (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/tekt-v16-ssty-2026-10-17/before.json.gz conserva UTC,
status, git diff HEAD --stat, fontes/L0 e 2128 hashes conferidos do fechamento
anterior. O SARIF em 00_nucleo/diagnosticos/import-syntax-2026-10-17/validation-architecture.log
registra 117 V16, incluindo font_metrics/math.rs:337 e shaper/shaping.rs:28.
MathSize em 01_core/src/entities/layout_types.rs:147 possui Display, Text,
Script e ScriptScript. math.rs:330–338 e shaping.rs:21–30 retornam Some(1/2)
para Script/ScriptScript; o wildcard devolve None para os dois tamanhos atuais
restantes e ocultaria uma variante futura. Os L0 vigentes já exigem esses
níveis e gates de math/TextItem/elegibilidade; não há decisão linguística nova.
before-probe-result.json conserva fontes/hashes, rustc/argv/UTC e 224 controles
com resultados atuais. A sonda extrai funções e declaração real de MathSize,
modelando somente os três campos de TextStyle lidos; no mutante descartável,
a variante TektSstyFuture é aceita em ambos os matches (zero E0004). O RED é
essa falta de proteção estrutural; controles funcionais atuais passam.

É mecânica de manutenção Rust: enum crescente precisa de decisão explícita;
não é medição nova de paridade nem de velocidade de leitura por IA. Os níveis
ssty continuam o contrato de linguagem/render anterior. A inferência de que o
reparo conserva os quatro casos atuais é refutada por qualquer controle,
replay textual ou regressão divergente. Não substituir uma decisão semântica
por enumeração cosmética: Display/Text são ausência intencional de pedido,
justificada no núcleo pinado; Script/ScriptScript mantêm níveis distintos.

### Responsabilidade exclusiva

{owned}

A política compartilhada está em 00_nucleo/prompts/_nuclei/fonts/ssty-math-size.toml,
pinada neste owner. Fluxo contínuo ADR-0127: nenhuma mudança de contrato público,
comportamento padrão ou fase; campos e enum público permanecem byte a byte.

### Aceitação

Primeiro registrar o RED estrutural e os controles antes do código. Depois,
a mesma sonda com variante futura precisa produzir exatamente dois E0004 nos
matches auditados; controles 224/224 idênticos. O modelo parcial de TextStyle
não prova shaping integral: conferir também os testes reais existentes de
font_metrics e integração ssty/TextItem, a suíte workspace e os gates finais.
Não adicionar testes que apenas reescrevam o match; o guard de crescimento é
a nova propriedade verificada. Replay do código permite somente o reparo
nominal prescrito e o hash do header. SARIF: retirar precisamente os dois V16,
adicionar duas notas V19 dos grupos Display/Text com motivo e cobertura dos
dois tamanhos; todos os outros diagnósticos devem permanecer por fonte, regra,
severidade e mensagem. Sem V20 novo, supressão ou alteração de configuração.
Grafo V15/V26 antes de resselo restrito aos dois pares; build, fmt, Clippy
all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito,
V1/V5/V15/V26, reverse lineage, suíte completa, diff --check e integridade.
Comandos pesados sequenciais sob MemoryMax12GiB/High10GiB/swap0/jobs1; sem
commit/push/merge/release. O objetivo global continua além deste lote.
'''
for source,owner in scope['owners'].items():
 p=Path(owner);s=p.read_text();needle='Núcleos Tekt:\n';line='- '+str(n)+' sha256:'+effective
 if needle in s:
  a=s.index(needle)+len(needle);z=s.index('\n\n',a);pins=s[a:z].splitlines()+[line];s=s[:a]+'\n'.join(sorted(pins))+s[z:]
 else:
  a=s.index('\n',s.index('Hash do Código:'))+1;s=s[:a]+'\nNúcleos Tekt:\n'+line+'\n'+s[a:]
 owned='Em ssty_level_of, conservar assinatura e guard inicial e substituir somente o wildcard por MathSize::Display | MathSize::Text => None. Não alterar elegibilidade, substituição GSUB, reverse cmap ou demais funções.' if 'font_metrics/' in source else 'Em ssty_level_for_subrun, conservar assinatura/guard e predicado canônico; importar MathSize no use privado de layout_types existente, usar os nomes MathSize nas quatro variantes e substituir somente o wildcard por MathSize::Display | MathSize::Text => None. Não alterar shape, medição/emissão, caches ou transporte.'
 p.write_text(s+common.format(owned=owned))
(out/'l0-record.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),effective=effective,owners={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in scope['owners'].values()}),indent=2)+'\n');print('L0 and shared nucleus prescribed')

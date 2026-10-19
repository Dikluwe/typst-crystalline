from pathlib import Path
import json,gzip,hashlib,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));assert json.load(open(out/'before-citation-probe.json'))['differences']==12;assert json.load(open(out/'before-probe-result.json'))['results'][1]['codes']==[];assert all(Path(p).read_text()==b['files'][p] for p in scope['sources']);n=Path('00_nucleo/prompts/_nuclei/content/source-origin.toml');effective=hashlib.sha256(n.read_bytes()+b'\x00TEKT-NUCLEUS-DEPS-V1\x00').hexdigest()
common='''

## Introspecção I1 — carriers e classificações nominais (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/tekt-v16-introspect-carriers-2026-10-17/before.json.gz
preserva UTC/status/diff HEAD --stat, fontes/L0 e 2130 hashes conferidos do
fechamento anterior. contexts-before.json identifica os três V16 vigentes.
before-citation-probe.json conserva argv, rlib/programa com SHA-256, source
freeze e saída: nos 12 casos (quatro carriers × três posições), a ordem k2,k1
da árvore sem wrapper vira vazia; citações conhecidas são perdidas. A coleta
bibliography.rs:21–76 e a conversão em :79–220 não atravessam Source/ShowRecipe/
Located/Realized. Estes são carriers internos sem syntax vanilla; a prova é
metamórfica de transparência já exigida, não comparação de representação Rust.

labelled.rs:45–80 devolve ausência para Source, embora o walk encontre o alvo
interno. Hipótese: sua projeção de compatibilidade perde o texto do mesmo
Heading/Equation/Figure no mesmo Location; o teste RED deve confirmar antes de
alterar esse helper. Seu L0 proíbe desmontar Styled arbitrário; Source não é
Styled, e a decisão anterior é preservada. populate.rs:24–44 satura futuros
ElementPayload em None. O domínio atual tem 19 variantes; as condições bool
ativas devem continuar exigidas. before-probe-result.json registra 290 casos
simbólicos e zero E0004 ao acrescentar variantes fictícias aos dois enums:
proteção estrutural ausente, sem alegar falha numérica atual nem ganho de
leitura por IA. Payloads opacos e guard booleano não provam semântica de dados.

### Responsabilidade exclusiva

{owned}

### Gate e aceitação

Fluxo interno/paridade ADR-0127: sem contrato público, default, fase, I/O,
registro dinâmico ou assinatura nova. L0 precede testes e código. RED funcional
antes da implementação; não usar falhas de montagem de sonda como RED. GREEN
exige as 12 ordens de citação iguais à base, visual de Realized sem contribuições
do original, desconhecidos preservados e fast path sem chaves sem reconstrução.
Label Source deve conservar a projeção direta e o Location/lang, enquanto
Styled e outros alvos continuam com os limites do contrato vigente.

O mutante descartável deve passar de zero para dois E0004 selecionados. Os
290 controles simbólicos mantêm a classificação, salvo os dois do helper de
label que agora reconhece Source. As 17 alternativas negativas de payload
incluem flags recusadas (Equation separa block false de block true/numbering
false); Source não é neutralizado como alvo desconhecido. Notas V19 permanecem
visíveis e justificadas por owner; sem V20 novo ou supressão.

Esperado no SARIF: remover só dois V16 (labelled/populate), acrescentar dois
V19 nominais e preservar o V16 de bibliography::collect_bib_keys até I2, com
os demais diagnósticos intactos por fonte/regra/severidade/mensagem. Não tratar
variantes estáticas ainda não percorridas como terminais legítimos. Replay
permite apenas os reparos prescritos e testes. Grafo V15/V26 antes do resselo
restrito aos três pares; build, testes focais/workspace, Clippy all-targets
-D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito, fmt, linhagem
estrita/reversa, source freeze e diff --check. Pesados sequenciais sob
MemoryMax12GiB/High10GiB/swap0/jobs1. Sem commit/push/merge/release. O objetivo
global e as frentes anteriores continuam ativos; esta etapa não certifica
bibliografia ou referências universais.
'''
owned={
'bibliography':'I1-carriers: em collect_bib_keys, visitar Source/ShowRecipe/Located pelo body e Realized pelo visual. Em convert_refs, reconstruir esses quatro carriers alterando somente body/visual, preservando span, Location, original e captura de recipe (rules/styles/guards/file/context/textual); não executar receitas. Conservar o fast path vazio e todos os outros braços, inclusive o comportamento Dynamic existente. Acrescentar dois testes locais de ordem/posições e negativos/original/identidade. A travessia está explicitamente INCOMPLETA: o passo I2-bibliography-static-coverage completará a auditoria e os casos faltantes de containers estáticos (por exemplo células raw/headers/footers, Repeat/Columns e outros corpos). Até I2, o wildcard de coleta e o passthrough legado da conversão continuam débito, não recebem silenciamento nem prescrição de terminalidade; substituir apenas o comentário que indevidamente chama todos de terminais.',
'labelled':'Em compute_labelled, acrescentar somente Source como delegação recursiva por empréstimo ao mesmo helper, com Introspector/Location/lang idênticos. Preservar Heading, Equation block, Figure e todo walk_label. Substituir o fallback final pelo complemento nominal de 117 variantes, incluindo Equation quando o guard block recusa. Styled permanece sem unwrap, mesmo sob Source. Acrescentar um teste local com counters reais já preparados no Location fornecido, comparando os três alvos diretos a Source e controles negativos. A projeção de compatibilidade não substitui o caminho numérico autoritativo nem corrige sua política de suplemento.',
'populate':'Somente no match automatic de populate_intr_from_tag_start, substituir o None genérico pelas 12 variantes não automáticas atuais e por Heading numbering false, Figure/Table is_counted false, Equation block false ou block true/numbering false: 17 alternativas. Conservar todo corpo ativo, cálculo de depth, action/custom key, ordem, stores e passes. Labelled e CounterUpdate não criam ocorrência automática extra; os caminhos específicos abaixo continuam intactos. Nenhum teste espelhado novo: controles simbólicos das combinações bool, replay e regressões reais existentes verificam o reparo nominal.'}
for source,owner in scope['owners'].items():
 p=Path(owner);s=p.read_text()
 if not source.endswith('/populate.rs'):
  needle='Núcleos Tekt:\n';line='- '+str(n)+' sha256:'+effective;assert needle in s;a=s.index(needle)+len(needle);z=a
  while z<len(s) and s[z]=='-':z=s.index('\n',z)+1
  pins=s[a:z].splitlines();assert not any(str(n) in x for x in pins);s=s[:a]+'\n'.join(sorted(pins+[line]))+'\n'+s[z:]
 p.write_text(s+common.format(owned=owned[Path(source).stem]))
(out/'l0-record.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_origin_effective=effective,owners={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in scope['owners'].values()}),indent=2)+'\n');print('Three L0 prescribed; bibliography I2 debt explicitly retained')

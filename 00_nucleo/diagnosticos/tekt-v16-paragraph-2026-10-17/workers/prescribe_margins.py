from pathlib import Path
import json,gzip,hashlib,subprocess,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'extended-before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());sources=['01_core/src/compiler/layout/paragraph.rs','01_core/src/compiler/stdlib/structural/par.rs'];owners=['00_nucleo/prompts/compiler/layout/paragraph.md','00_nucleo/prompts/compiler/stdlib/structural/par.md']
for rel in ['crates/typst-layout/src/flow/distribute.rs','crates/typst-library/src/model/par.rs']:
 data=subprocess.check_output(['git','show','9f2b6e8715237cb086899a42873660fe744622e8:'+rel]);assert data==Path('lab/typst-original',rel).read_bytes()
Path(owners[0]).write_text(Path(owners[0]).read_text()+'''

## Refutação do primeiro candidato — margens de Par vazio (2026-10-17)

### Medição anterior à nova decisão

first-candidate/ conserva integralmente código/L0, PDFs e before/after do
primeiro reparo de Source. A checagem falhou: styled-empty ainda perde 7.2pt,
e styled-empty-text, antes coincidente verticalmente, perde os mesmos 7.2pt.
Isso refuta a suficiência da projeção. supplement-before-measurement.json.gz e
refuters-before-measurement.json.gz no diagnóstico desta revisão acrescentam
24 casos bilaterais: tamanho 8/18, terminais, consecutivos, somente vazios,
spacing explícito/zero/negativo e caixa/blocos. Estado exato por coleta em
HEAD/working tree/UTC/hashes/status/diffstat. Com 12pt externo e 18pt interno,
AFTER está em y53.117213 versus60.317183; com8pt, y41.117213 versus45.917187.
Dois vazios8→18 sem tinta medem16pt de página versus37.6pt. O constructor
`01_core/src/compiler/stdlib/structural/par.rs:153-157` descarta spacing.
A fonte pinada `lab/typst-original/crates/typst-library/src/model/par.rs:212-225`
define margens de ambos os lados, colapsadas pelo maior valor. A distribuição
em `lab/typst-original/crates/typst-layout/src/flow/distribute.rs:244-260`
confirma a força das margens. Objetos Git conferidos no pin vigente.

### Fórmula e estado próprios deste owner

Esta revisão supersede somente a proibição de mudar a fórmula do ramo vazio.
Tratar Par vazio como unidade de altura zero (ou virtual_top quando contém
Text vazio), com fim causal próprio. Descartar a margem acima somente no
início vazio do fluxo; havendo unidade anterior, colapsar acima com o pending
anterior por máximo para margens fracas. Margem forte de Block anterior tem
precedência e conserva seu valor, inclusive zero. O fim é a âncora anterior
mais o gap admitido e virtual_top, contado uma vez.

Depois, instalar o fim em cursor_y e prev_line_baseline, zerar descida da
unidade anterior, confirmar last_block_descent_y e testemunha de fluxo;
reservar somente spacing próprio em prev_block_below_pending. A margem abaixo
fica pendente para o consumidor seguinte e colapsa separadamente com seu above.
Conservar block_chain_active/prev_margin_is_parbreak para o protocolo vigente;
initial_baseline_pending torna-se false porque há uma unidade causal de
altura conhecida, mesmo sem tinta. Limpar empty_paragraph_weak_spacing: não
preaplicar a margem terminal, logo não há soma fraca fictícia a remover.
A admissão já existente de Cursor para block-chain acrescenta top-edge real
do próximo texto e resolve seu gap. Não modificar Cursor, Finish, checkpoint,
layout hub nem criar estado/campo público ou privado novo neste pacote.

Conservar a projeção Source e os três predicados já prescritos. Não modificar
fórmulas de corpos materiais, regras de texto vazio fora de Par, callbacks,
fonte, bidi, migração regional, direções, margens fortes ou conteúdo ocultado.
A nova inferência é refutável por qualquer regressão dos controles construíveis
ou perda da geometria própria de dois vazios consecutivos sem tinta.

### Aceitação refinada, sem encobrir diferenças preexistentes

Adicionar RED funcional para tamanho8/18 inicial/intermediário/terminal, dois
vazios, Text vazio virtual e transporte/erro de spacing no constructor. Preservar
os nove testes anteriores e todos os controles. Repetir os18 casos principais,
12 supplement e12 refuters; edges acrescenta6 controles construíveis e2
limitações conhecidas de Block negativo, que permanecem abertas. Tipos de
spacing devem concordar com o pin. Quaisquer falhas fora do reparo continuam
inventariadas, sem crédito de paridade universal.

A tolerância vertical e de xMin continua0.002pt, com texto/páginas exatos.
O xMax do pdftotext já difere até cerca de0.0051pt antes do reparo (AFTER
46.148 versus46.15039; MIDDLE18pt77.966 versus77.960938); não alargar a
tolerância para chamar isso de equivalente. Registrar o resíduo horizontal
preexistente e exigir xMin/xMax inalterados dentro de cada binário entre
coletas. A sua natureza (geometria de glyph ou metadado PDF) continua por
classificar, não é declarada corrigida por este lote de fluxo vertical.
Páginas, palavras e yMin/yMax devem coincidir com o pin nos casos abrangidos;
controles antes coincidentes não podem regredir verticalmente.

Adicionar o segundo par owner/consumer ao resselo e à proveniência congelada.
O diferencial pode substituir somente a nota V19 do antigo grupo que incluía
spacing no constructor pela nota do grupo residual de três propriedades ainda
incompletas. Nenhum V16 extra é admitido; demais gates e teto de RAM mantêm-se.
''')
Path(owners[1]).write_text(Path(owners[1]).read_text()+'''

## Transporte de spacing no constructor — 2026-10-17

### Medição anterior à decisão

No HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
`01_core/src/compiler/stdlib/structural/par.rs:153-157` aceita e ignora spacing.
`00_nucleo/diagnosticos/tekt-v16-paragraph-2026-10-17/extended-before.json.gz`
conserva fonte/L0 e hashes globais/status/diffstat/UTC; supplement-before-
measurement.json.gz mede BEFORE/par(spacing:30pt)[]/AFTER: AFTER em45.917213pt
no Tekt contra77.117188pt no pin. Em terminal, altura39.5172 versus55.117188.
spacing-types-before.json.gz mede dez valores nos dois binários: None/Auto/
Bool/Int/Float/Str/Ratio são aceitos indevidamente no Tekt; Length em em, zero
e negativo são aceitos no pin. `lab/typst-original/crates/typst-library/src/model/par.rs:212-225`
confirma Length e default1.2em. Fonte conferida contra o objeto Git do pin
9f2b6e8715237cb086899a42873660fe744622e8. É linguagem, sem inferir intenção.

### Prescrição exclusiva

Correção interna/de paridade ADR-0127 em fluxo contínuo, sem nova assinatura,
entidade, trait, flag, default ou fase. Supersede o scope-out de spacing somente
para validação/transporte no constructor; first-line-indent, hanging-indent e
justification-limits continuam incompletos, conforme sua família própria.

Aceitar somente Value::Length, inclusive zero/negativo/em; publicar em Styles
custom par.spacing o Length lexical sem resolvê-lo no eval. A fonte/região é
resolvida pelo layout proprietário vigente. Para demais tipos, reutilizar
compiler/eval/rules expected_length_error com value_span da última ocorrência
nomeada spacing, ou args.span sintético. Conservar hint para Int, vocabulário,
body, ordem dos demais nomes, leading/justify/linebreaks e demais diagnósticos.
O residual de três propriedades aceitas/ignoradas pode continuar num grupo V19
justificado como dívida explícita; não mover lógica de layout ao constructor.

### Aceitação

RED→GREEN funcional de transporte para Length físico/em/zero/negativo, erro
com hint e span causal para Int e rejeição dos demais tipos. Observar resultado
do layout, não somente inspeção de Styles. Repetir as coletas com spacing30pt,
2em e controles zero/negativo no diagnóstico deste lote; conservar a suíte e
os gates completos já prescritos. Resselo limitado a este par e ao owner de
parágrafo do mesmo lote. Não certificar raw fields genéricos, indentação,
limites de justificação ou paridade universal de Par por esta correção.
''')
(out/'scope.json').write_text(json.dumps(dict(sources=sources,owners=dict(zip(sources,owners)),mode='ADR0127 continuous; measured parity/internal formulas; no public contract changes'),indent=2)+'\n')
(out/'margin-prescription.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),owners={p:sha(p) for p in owners}),indent=2)+'\n')

# Passo 1420 — retirar `Unknown` das rotas HTML e math do P1400

## Objetivo

Reabrir exclusivamente a observabilidade das 66 rotas que permaneceram
`UNKNOWN` no P1400 por duas causas instrumentais conhecidas:

1. 44 rotas `html.*` bloqueadas por `TARGET_CONTROL_FAILED` após o harness
   transportar `--jobs 1` para uma CLI que não aceita essa opção;
2. 22 rotas `math.*` bloqueadas por
   `NON_BILATERAL_EVIDENCE(ROUTE_ALTERNATIVE_PROJECTION)`, embora as duas
   alternativas tenham terminado com sucesso e exposto uma diferença pública
   de morfologia.

O passo corrige o instrumento e reclassifica essas 66 rotas. Não altera Prompt
L0, Núcleo Tekt, código L1–L4, catálogo público, manifesto permanente da matriz
ou resultados históricos P1400. Não implementa nenhuma rota e não cobre as 124
rotas `format.*`, as nove math já `PARTIAL`, os oito membros já `ABSENT` nem
dívidas históricas de loaders, closures, diagnósticos ou layout.

## Medição anterior à decisão

Estado medido em `2026-09-16T10:49:19-03:00`:

- HEAD `95491eec8987176616a784554d11938e1b1c1e49`;
- tree `ba40fcfa888408ca0e09395beac18e08b59a4fd0`;
- working tree limpa;
- vanila ratificado
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- binário vanila SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- binário cristalino rebuildado do HEAD, SHA-256
  `1eb24a50c4bebd08febe976da73c9b275d8c1200db7781ad8e9d7484089e547e`;
- enumerador cristalino rebuildado do HEAD, SHA-256
  `ec109dad8c7a38e8e08e01ed4be4ea842c5f86635584718c55915a8616a6f810`.

O inventário atual tem SHA-256 de recibo
`580a0307dc3758af395d658d9d3a7de5ac7ead3a84b535c5f15a19ebe090df15`
e quatro hashes de determinismo idênticos:
`4109ae0d0f9a9403b1acc759e92438a3aeaa2edd6ae4292bb1b3ed16e265c28e`.
Ele mede 504 células `VANILLA_ONLY`, 207 paths únicos e 124 paths da família
`format`; portanto restam 83 paths não-`format.*`.

P1401–P1419 retiraram dez paths do denominador anterior: `selector.before`,
`selector.after`, `outline.entry`, seus cinco membros, `raw.line` e `par.line`.
O P1414 reexecutou as seis rotas finais desse recorte e obteve `{MATCH: 6}` em
ordem direta, repetida e reversa.

Entradas históricas que motivam esta reabertura:

- contrato P1400 SHA-256
  `b4be7e313c2772040f8473f0b00e689772060396eec6aa4821f6170c56fbc545`;
- oráculos P1400 SHA-256
  `75ff90569ca462b819024c186f1705bdd41619c71edb521345549f32fcf1ceb2`;
- resultados P1400 SHA-256
  `9da156aafbe572a0680f14a6c4b93aeff2b93ef63dde7bca6056a787feb62542`;
- runner P1400 SHA-256
  `a4bcf6f2807fab8354ce296275d3e3c8b0dd41c7a6a77066e1c2320eb656e016`;
- reprobe P1409 SHA-256
  `24ab1247a52d63cffdc2cb93868cd54ff70e91e293df795b789283d5b4701250`;
- reprobe P1414 SHA-256
  `1a854a43e4b7e88cd6d3fd3f06bc363f1bd5a7f27bf6df587eff40220adfacba`.

A matriz focal atual permanece em 48 `MATCH`, seis `DIFFERENCE` mecânicas e
zero `UNKNOWN`; ela é controle de regressão, não denominador deste passo.

## Denominador congelado

### HTML — 44

```text
html.area
html.audio
html.base
html.blockquote
html.canvas
html.caption
html.colgroup
html.data
html.del
html.details
html.dialog
html.embed
html.fieldset
html.form
html.frame
html.hr
html.img
html.input
html.ins
html.label
html.link
html.map
html.meta
html.meter
html.object
html.optgroup
html.option
html.output
html.progress
html.q
html.script
html.slot
html.source
html.style
html.table
html.tbody
html.td
html.textarea
html.tfoot
html.th
html.thead
html.time
html.tr
html.track
```

### Math — 22

```text
math.abs
math.accent
math.cases
math.frac
math.limits
math.lr
math.mat
math.mid
math.norm
math.overbrace
math.overbracket
math.overline
math.overparen
math.overshell
math.primes
math.root
math.round
math.stretch
math.underbrace
math.underbracket
math.underparen
math.undershell
```

O conjunto deve ser derivado novamente de `p1400-results.json` e confrontado
literalmente com a lista acima. Rota adicionada, removida ou já ausente do
inventário atual produz `Unknown(BASELINE_DRIFT)` e reabre o escopo; não se
edita a lista silenciosamente.

## Classificação e decisão

As causas observadas são falhas do instrumento, não prova de ausência nem de
paridade:

- `--jobs` é mecânica de execução. P1409 já demonstrou que deve ser removido
  simetricamente dos dois produtos antes de medir a linguagem;
- a diferença de `repr` entre duas alternativas math bem-sucedidas é
  morfologia pública, não motivo para descartar a medição como não bilateral.

P1420 passa a registrar dois eixos por rota:

| Campo | Estados |
|---|---|
| `qualified_state` | `MATCH`, `ABSENT`, `DIFFERENCE`, `UNKNOWN` |
| `alternative_state` | `MATCH`, `ABSENT`, `DIFFERENCE`, `UNKNOWN`, `NOT_APPLICABLE` |
| `route_state` | `MATCH`, `PARTIAL`, `ABSENT`, `DIFFERENCE`, `UNKNOWN` |

Regras de composição:

1. forma qualificada bilateral e observáveis iguais → `MATCH`;
2. forma qualificada bilateral com diferença pública → `DIFFERENCE`;
3. forma qualificada ausente no cristalino e alternativa bilateral igual →
   `PARTIAL`, reason `QUALIFIED_ABSENT_ALTERNATIVE_MATCH`;
4. forma qualificada ausente e alternativa bilateral diferente → `PARTIAL`,
   reason `QUALIFIED_ABSENT_ALTERNATIVE_DIFFERENCE`; a diferença da alternativa
   também entra nominalmente no backlog;
5. forma qualificada e alternativa conclusivamente ausentes → `ABSENT`;
6. controle, identidade, execução ou projeção obrigatória inconclusiva →
   `UNKNOWN` com razão literal.

`PARTIAL` não esconde `alternative_state=DIFFERENCE`. O relatório deve contar
separadamente rotas parciais com alternativa coincidente e rotas parciais com
alternativa divergente. Nenhuma alternativa concede `MATCH` ao membro
qualificado ausente.

## Regime de materialização segregada

Usar protocolo completo para o contrato observável, oráculos, ataques,
instrumento e veredito. O produto permanece read-only.

- **Autor da obrigação:** este passo; congela pin, 66 rotas, causas a reparar,
  política de `Unknown` e limites.
- **Autor do contrato:** escreve o esquema de dois eixos sem ler qualquer
  resultado cristalino novo.
- **Autor dos oráculos:** deriva expected somente do vanila ratificado e dos
  artefatos históricos protegidos; não lê o resultado do candidato P1420.
- **Adversário:** cria mutações do transporte, projeção e classificador; não
  corrige o runner nem escolhe implementação de produto.
- **Implementador do instrumento:** escreve apenas o sucessor em `lab/parity/`;
  não edita contrato, expected, mutantes ou artefatos P1400/P1409/P1414.
- **Verificador:** executa entradas seladas e produz resultados, backlog e
  certificado; não corrige material sob julgamento.

Se os papéis compartilharem workspace, autoridade de escrita ou contexto não
isolado, declarar **“executado sem atestação de isolamento”**. Hashes e nomes
distintos não provam segregação material.

## Fase A — manifesto, contrato e preseal

Antes do runner candidato, produzir:

- `00_nucleo/diagnosticos/p1420-manifest.json`;
- `00_nucleo/diagnosticos/p1420-contract.json`;
- `00_nucleo/diagnosticos/p1420-oracles.json`;
- `00_nucleo/diagnosticos/p1420-mutants.json`;
- `00_nucleo/diagnosticos/p1420-preseal.json`.

O manifesto registra hashes deste passo, produtos, inventário, artefatos
P1400/P1409/P1414 usados, conjunto literal de rotas, capacidades por papel,
comandos, observáveis, política de `Unknown`, limites e budget de calibração.

Os artefatos P1400, P1409 e P1414 são históricos somente leitura. Não corrigir
seus argv, expected ou vereditos in place. P1420 cria sucessores próprios e
registra a transformação aplicada.

## Fase B — reparo HTML

Para cada um dos 44 tags, executar em processos separados:

1. controle de parser/feature comum;
2. controle de target com fixture literal
   `#context assert.eq(target(), "html")`;
3. controle independente `html.elem("div")`;
4. forma qualificada `html.<tag>` com argumentos mínimos válidos;
5. alternativa literal `html.elem("<tag>")`.

Transporte bilateral obrigatório:

```text
typst compile --features html --format html INPUT OUTPUT
```

ou, para `html+a11y`:

```text
typst compile --features html,a11y-extras --format html INPUT OUTPUT
```

É proibido transportar `--jobs`, `eval --target`, flag presente em apenas um
produto ou normalização de argv posterior à execução. O gate deve varrer todos
os argv emitidos e provar zero ocorrência de `--jobs` e `--target`.

A projeção HTML obrigatória conserva bytes e stderr brutos, mas classifica a
morfologia por árvore pública canônica: doctype, nomes de tag, nesting, texto,
atributos e valores. Apenas ordem de atributos e whitespace puramente externo a
text nodes podem ser tratados como mecânica; escaping, conteúdo textual,
atributo ausente, tag diferente e nesting diferente são `DIFFERENCE`.

Cada tag possui alternativa literal própria. `html.elem("div")` não concede
crédito a outro tag, e o resultado de um irmão nunca é extrapolado.

## Fase C — reparo math

Para cada uma das 22 rotas, executar separadamente:

1. `repr(math.<member>)` e uma chamada qualificada mínima válida;
2. forma bare ou math-mode equivalente congelada pelo oráculo P1400;
3. controles P1395–P1397 aplicáveis à família;
4. ao menos um argumento de fronteira que distinga existência da função de
   erro de cast ou aridade.

As duas alternativas devem rodar mesmo quando o acesso qualificado abortar.
Exit 0 bilateral com stdout diferentes não é `NON_BILATERAL_EVIDENCE`: é
`alternative_state=DIFFERENCE`, desde que a projeção seja completa.

Para `repr`, comparar bytes UTF-8 e codepoints sem normalização, pois a
representação é morfologia pública. Para documentos math renderizados, comparar
conteúdo estruturado e projeção morfológica; bytes PDF, ordem de objetos e
estrutura Rust não são critérios de paridade.

## Fase D — gate discriminatório

O adversário deve rejeitar pelo menos estas mutações:

1. remover uma das 66 rotas;
2. incluir qualquer rota `format.*` ou uma das 17 rotas fora do recorte;
3. remover `--jobs` somente de um produto;
4. manter `--jobs 1`, remover só a flag ou só o valor;
5. usar `eval --target html` em qualquer controle ou rota;
6. ativar features assimétricas;
7. usar um `html.elem("div")` genérico para conceder crédito a irmãos;
8. executar alternativa somente após sucesso da forma qualificada;
9. converter alternativa math divergente em `UNKNOWN`;
10. converter alternativa divergente em `MATCH` da forma qualificada;
11. descartar codepoints, stderr ou exit dos observáveis;
12. tratar diferença de escaping, tag ou nesting HTML como whitespace;
13. transformar erro de argumento em prova de ausência do membro;
14. transformar timeout, OOM, swap, truncamento ou identidade divergente em
    sucesso;
15. editar expected depois de observar o cristalino;
16. alterar qualquer artefato histórico protegido.

Casos positivos válidos devem dar `Preserved`, mutações negativas `Violated` e
opacos deliberados `Unknown`. Exigir `mutation_score = 1.0`, repetição e ordem
inversa com vetor idêntico. Mutante válido sobrevivente impede o selo.

## Fase E — runner e execução

Criar `lab/parity/p1420_unknown_route_reprobe.py`. O runner deve:

- selecionar literalmente as 66 rotas no P1400 e no inventário atual;
- validar todos os hashes antes de executar produto;
- usar uma rota/probe por processo quando um diagnóstico puder ocultar outro;
- executar `direct`, `direct-repeat` e `reverse`;
- manter paralelismo 1 e `RAYON_NUM_THREADS=1`;
- aplicar 512 MiB por processo, timeout de 15 s e 64 KiB por stream;
- registrar argv, exit, stdout, stderr, artefato, projeção, duração, RSS, swap,
  instante e hashes;
- manter brutos e artefatos volumosos em `/tmp`;
- emitir exatamente um `qualified_state`, `alternative_state` e `route_state`
  para cada rota.

Timeout, sinal, OOM, swap, truncamento, ferramenta ausente, baseline divergente,
target inválido ou projeção incompleta permanecem `UNKNOWN`. Nenhum limite pode
ser elevado para transformar falha semântica em sucesso.

Executar primeiro um recorte focal de uma rota HTML, uma math historicamente
`UNKNOWN`, uma math historicamente `PARTIAL` como controle externo e um caso
opaco. Só depois do focal verde executar as 66 rotas completas.

## Budget e parada

Budget máximo:

- duas revisões focais para a causa HTML;
- duas revisões focais para a causa math;
- uma campanha integral após os dois recortes passarem;
- uma repetição integral final pelo verificador.

Duas revisões consecutivas com o mesmo vetor e mesma causa dominante exigem
parar e redesenhar. Repetir o corpus sem hipótese nova, afrouxar a projeção ou
converter `Unknown` em sucesso é proibido.

## Artefatos finais

Produzir:

- `lab/parity/p1420_unknown_route_reprobe.py`;
- `00_nucleo/diagnosticos/p1420-manifest.json`;
- `00_nucleo/diagnosticos/p1420-contract.json`;
- `00_nucleo/diagnosticos/p1420-oracles.json`;
- `00_nucleo/diagnosticos/p1420-mutants.json`;
- `00_nucleo/diagnosticos/p1420-preseal.json`;
- `00_nucleo/diagnosticos/p1420-results.json`;
- `00_nucleo/diagnosticos/p1420-backlog.json`;
- `00_nucleo/diagnosticos/p1420-report.md`;
- `00_nucleo/diagnosticos/p1420-verification.json`.

O backlog agrupa por causa e owner comprovado, mas não altera ownership nem L0.
Para cada rota não-`MATCH`, registrar menor witness, dois subvereditos, owner,
consumer, hipótese causal, condição de refutação e classe ADR-0127 provável.

## Gates finais

```sh
python3 -m py_compile lab/parity/p1420_unknown_route_reprobe.py
python3 lab/parity/p1420_unknown_route_reprobe.py --validate-only ...
python3 lab/parity/p1420_unknown_route_reprobe.py --gate ...
python3 lab/parity/p1420_unknown_route_reprobe.py --run ...
cargo fmt --all -- --check
crystalline-lint .
git diff --check
```

Não executar `cargo test --workspace`, lib-test monolítico ou rebuild global.
Se o binário cristalino não corresponder ao HEAD congelado, permitir um único
build release serial sob 4 GiB/240 s, registrando RSS e swap. Falha do build é
`Unknown(IDENTITY_MISMATCH)`; não recebe crédito de produto.

O verificador final confere:

- 66 rotas exatas e nenhuma fora do recorte;
- hashes de todas as entradas protegidas;
- zero `--jobs` e zero `eval --target` nos argv;
- transporte e features bilaterais;
- alternativas executadas independentemente;
- três ordens com vetor idêntico;
- `mutation_score = 1.0`;
- nenhum `Unknown` promovido por default;
- nenhuma alteração em L0, Núcleos ou L1–L4;
- zero V5, V15 e V26;
- resultados P1400/P1409/P1414 preservados byte a byte.

## Critério de fecho

P1420 fecha quando as 66 rotas possuem os três estados exigidos, evidência
bilateral atual e razão literal, e nenhuma rota permanece `UNKNOWN` pelas duas
causas instrumentais que motivaram o passo.

Um `UNKNOWN` novo e legítimo não pode ser escondido: o passo fecha como
`INCONCLUSIVE` e recomenda novo diagnóstico, não produto. Se todas as rotas
forem conclusivas, o relatório pode recomendar exatamente um próximo passo de
produto, escolhendo a menor causa com owner L0 único e witness completo.

O veredito cobre apenas estas 66 rotas e nunca declara paridade funcional geral
de HTML, math ou da linguagem Typst.

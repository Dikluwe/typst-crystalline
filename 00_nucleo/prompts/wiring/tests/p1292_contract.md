# Prompt L0 — `wiring/tests/p1292_contract` — oráculos black-box P1292
Hash do Código: cf1c1953

**Estado:** REOPENED-P1293-FINAL-TRANSPORT-CORRECTION — consumer materializado;
somente o observador SVG test-only requer correção de transporte para compor
transformações ancestrais, sem alterar expectativas ou produto P1292.

**Camada:** L4 — teste de integração
**Ficheiro alvo:** `04_wiring/tests/p1292_contract.rs`
**Origem:** P1292, contrato canônico v3
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129
**Vanilla ratificado:** `a51e02804`

## Medição anterior à decisão

O consumer protegido atual SHA-256
`fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`
executa o binário como caixa-preta e possui `@prompt-hash 41dd2e0e`; este prompt
é seu owner exclusivo 1:1. O estado órfão anterior foi encerrado pelo resselo.

Três transportes anteriores eram inválidos, apesar de suas expectativas de
linguagem continuarem corretas:

1. `repr(math.underline == underline)` produz string, não boolean; comparar seu
   stdout cru com `false` mistura JSON de string e JSON boolean.
2. `typst query` não materializa o metadata criado por `context` no caminho
   atual; portanto os objetos `measure(...)` de B/C e `here().position()` de D
   não chegavam ao selector por uma execução contextual garantida.
3. O exporter SVG cristalino serializa apenas `doc.pages.first()`; um template
   `{p}` não torna D multipágina. Isso é limitação de transporte, não semântica
   de `place.flush`, e não pode ser usado como RED.

Probes bilaterais de 2026-09-01 confirmaram os substitutos:

- `eval 'math.underline == underline' --format json` devolve o boolean JSON
  `false` nos dois binários;
- compilar `context { let s = measure(...); rect(width:s.width,
  height:s.height) }` para SVG executa contexto: o controle materializa shape
  bilateralmente; vanilla finito `gap:10%` e auto `gap:10%+1em` produzem
  retângulo `23.5312pt × 22.88pt`; o candidato entra no contexto e o RED ocorre
  em `math.vec`/região de `measure`, não em query;
- CLI PDF + `pdftotext -bbox` preserva três páginas bilateralmente num controle
  com pagebreaks. No vanilla, tokens zero-flow (`#place([TOKEN])`) congelam o
  prefixo sem mudar layout: `FLOAT_BEFORE` página 2/yMin 17.404 (âncora Typst
  y=20), `AFTER_MARKER` página 3/yMin 4.642 (âncora 7.238), `FLOAT_AFTER`
  página 3/yMin 87.404 (âncora 90); sem flush, `AFTER_MARKER` fica na página
  1/yMin 47.842 (âncora 50.438).

### P1293.final — medição da baseline SVG mista

O plano adversarial SHA-256
`ecc12db3e5d9826dafdb7b42653e28d5f4f8e01772a8554e6718b1ac8129e226`
e o recibo de execução SHA-256
`8c742753c64524578f08151e604668d047be23a526d9acac9088398870f79868`
mediram o teste P1292 em working tree não commitada: `10/11` casos passaram e
somente a asserção em `04_wiring/tests/p1292_contract.rs:422` falhou. O parser
`svg_glyph_baselines` em `:144-165` extrai apenas o sexto operando da matriz
local do glifo (`4.862`) e ignora o `translate(10.395 2.651)` do grupo externo.
A coordenada global congelada é `2.651 + 4.862 = 7.513`; portanto a expectativa
`7.513` permanece correta e o valor observado `4.862` demonstra perda de
transformação no observador, não mudança de linguagem ou layout produtivo.

Medição: a árvore SVG contém transformação externa e matriz interna separadas;
o helper atual lê somente a interna. Inferência: acumular as transformações dos
ancestrais recompõe a baseline global que o contrato já exige. Refutadores:
ordem SVG distinta da composição declarada, transformação não afim, baseline
global que não resulte em `7.513`, ou alteração do produto/fixture necessária.
Qualquer refutador bloqueia a correção e exige nova medição bilateral.

### Decisão de transporte test-only

O observador SVG deve percorrer os grupos ancestrais e acumular transforms na
ordem afim do documento, incluindo ao menos `translate(...)` e `matrix(...)`,
antes de comparar a coordenada global do glifo. Não pode selecionar por
caractere, fonte, número de fixture ou constante sentinela. Preserva sem mudar
as expectativas `7.513` e `21.901`, fixtures, tolerâncias, asserts e contrato
produtivo P1292; somente o helper/parser de observação deste consumer muda.

Classificação ADR-0107/0108: baseline global é observável geométrico; parsing e
composição XML são transporte mecânico, medido antes da decisão. ADR-0127:
correção interna test-only sem API, default, fase ou compatibilidade nova,
portanto fluxo contínuo. ADR-0129: owner/consumer permanece estritamente 1:1.

## Contrato do consumer

O teste usa somente `CARGO_BIN_EXE_typst`, diretórios temporários isolados e
ferramentas do ambiente declaradas. Remove temporários ao final, propaga
stdout/stderr em falha, não lê implementação, não altera oráculos durante a
execução e nunca converte `Unknown` em sucesso.

### Lote A — preservado

Mantém os observáveis selados de surface/repr/erros e SVG para callback,
`cross`, background, span e ausência de documento provisório. O amendment não
muda fonte, valores ou mecanismo A.

### Lote B — boolean e SVG page-auto

- Identidade: avaliar diretamente `math.underline == underline` com formato
  JSON, desserializar `serde_json::Value::Bool(false)` e rejeitar string
  `"false"`; `repr` continua usado somente para a morfologia pública do
  elemento/função.
- Geometria: cada fixture é compilada, não consultada, com
  `page(width:auto,height:auto,margin:0pt)`. O parser do oráculo extrai
  `width`/`height` da raiz `<svg>` e as `<line>`/paths de regra. O controle
  principal `$underline(x)$` exige `6.292pt × 7.513pt` e exatamente uma regra;
  text/display/script/cramped conservam os valores do contrato congelado
  (`6.29×7.51`, `6.29×7.36`, `11.44×8.46`, `6.73×9.54`, arredondamento de
  apresentação) e a regra derivada das constantes MATH. Não compara SVG
  completo, IDs de glyph, ordem de defs ou bytes.

### Lote C — measure contextual materializado como shape SVG

Cada caso usa página auto e:

```typst
#context {
  let s = measure(math.equation(math.vec(...)), width: 100pt, height: REGION)
  rect(width: s.width, height: s.height, fill: COR_UNICA)
}
```

O oráculo compila e extrai as dimensões do rect/path de cor única no SVG; não
usa `typst query`, metadata ou root como substituto do shape. Casos separados
guardam região finita e `height:auto`: finito `0%` = `15.79×7.70`, finito
`10%` = `23.53×22.88`; auto `0%` = auto `10%` = `15.79×7.70`; auto `1em` =
auto `10%+1em` = `23.53×22.88`. Preserva também sintaxe canônica, delim,
align, repr/default-presence e erros do selo.

### Lote D — PDF multipágina com tokens visuais

D não usa SVG nem query. O teste compila PDF via CLI e executa
`pdftotext -bbox <pdf> <html>`; ausência da ferramenta é erro explícito, não
skip/Unknown. Cada marcador é texto ASCII curto e único dentro de `#place`
não-float, para não participar do fluxo. O parser lê `<page>` em ordem e
`word@xMin/yMin/yMax`; exige unicidade de cada token e associa a página pelo
ancestral `<page>`.

Fixtures preservam os resultados semânticos D já selados:

- prefixo/sufixo: anterior p2 y=20, after p3 y=7.238, posterior p3 y=90 e
  ordem documental/visual causal; bbox vanilla correspondente fica congelada
  no recibo do oráculo;
- controle sem flush: after p1 y=50.438;
- top/bottom misto conserva clearance e ordem, com top antes de bottom e after
  somente depois da realização do prefixo;
- nested atua apenas no Layouter ativo; no-op sem floats conserva PDF textual
  e número de páginas, ignorando metadados voláteis do arquivo.

Não compara bytes PDF, IDs, timestamps, compressão ou ordem mecânica de
operadores. O critério é página, bbox/tolerância pinada, presença/unicidade e
ordem dos tokens. O controle explícito de três páginas prova o transporte antes
de avaliar flush.

## Inferências e refutação

Classificação ADR-0107/0108: boolean de identidade, dimensões/linhas SVG e
página/posição/ordem de tokens são semântica/morfologia; JSON, XML, PDF e bbox
são transporte mecânico. Inferimos que rect e tokens zero-flow não mudam o
observável medido porque controles bilaterais coincidem com as âncoras P1292.
Refutações: shape diverge de `measure`; token muda número/posição de página;
controle contextual não emite shape; controle PDF perde páginas/tokens; ou o
RED ocorre antes da feature por falha da ferramenta. Qualquer refutação para e
ressella; não se adapta expectativa ao candidato.

## Ownership, lineage e verificação

Este prompt possui exatamente o consumer `04_wiring/tests/p1292_contract.rs`
e não legitima código produtivo nem qualquer outro teste. O consumer vigente
já contém o header abaixo com `@prompt-hash 41dd2e0e`; após esta revisão L0,
somente esse hash fica transitoriamente desatualizado até o resselo mecânico:

```text
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/tests/p1292_contract.md
//! @prompt-hash 41dd2e0e
//! @layer L4
//! @updated 2026-09-01
```

Gates: teste focal, controles bilaterais registrados, V1/V5/V15/V26,
`cargo fmt --all -- --check`, `git diff --check` e whitespace dos untracked.
O próximo passo permitido é apenas atualizar o header e corrigir o transporte
do parser conforme esta obrigação; qualquer outro drift bloqueia.

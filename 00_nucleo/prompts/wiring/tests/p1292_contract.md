# Prompt L0 — `wiring/tests/p1292_contract` — oráculos black-box P1292

**Estado:** CONTRATO P1292 AMENDMENT-2 — consumer existente aguarda somente o
header de lineage pelo autor independente de oráculos; sem `Hash do Código`
até esse header ser escrito e o consumer ressellado.

**Camada:** L4 — teste de integração
**Ficheiro alvo:** `04_wiring/tests/p1292_contract.rs`
**Origem:** P1292, contrato canônico v3
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129
**Vanilla ratificado:** `a51e02804`

## Medição anterior à decisão

O consumer protegido SHA-256
`3ced6be4556d5b8b7f79540cc0df12791f38d2e22e6d46a2992f38049043c2f5`
executa o binário como caixa-preta, mas não possui `@prompt`; criar este owner
1:1 torna o órfão explícito até o autor do oráculo adicionar o header L4.

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
e não legitima código produtivo nem qualquer outro teste. O autor independente
adiciona ao consumer, sem mudar os casos até nova autorização:

```text
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/tests/p1292_contract.md
//! @prompt-hash <sha256 deste L0, 8 hex>
//! @layer L4
//! @updated 2026-09-01
```

Gates: teste focal, controles bilaterais registrados, V1/V5/V15/V26,
`cargo fmt --all -- --check`, `git diff --check` e whitespace dos untracked.
Até o header existir, somente o órfão V15 deste prompt/consumer é transitório;
qualquer outra violação bloqueia.

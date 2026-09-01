# P1292 — L0/oracle amendment-2

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest autorizado:** `488c0cdd466bba36dab5e662943ea1d11dd40f173abcf4f827860a5b8b28af5e`

**HEAD:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`

**Estado:** working tree não commitada; medição concluída em 2026-09-01T00:27:14-03:00

**Regime:** segregação completa, sem alegação de isolamento técnico

## Bridge e escopo

O contrato canônico predecessor v2 é
`3c7334f5143c2e62d6512a5dd824575da0cf28fdae40d5939646f1a3dcb27022`,
o selo predecessor é
`abff3035a2ed2ee5c560b906139b610eb503308c5ef60dc157f5ac46e00ee602`
e seu recibo é
`52c0525459ed7ed43a7bb7f9646fa1a20957fbb1bdc7f748290ee19e5967cb30`.
O recibo pre-gate permanece intocado em
`c0c9ded039181be699ad387f1e89368fd56a72c976ba5d0859f528769ef91966`.

Este amendment muda somente o mecanismo black-box de comparação B/C/D e
fecha a propagação de extensão vertical inline em `equation`. Os vetores,
valores e resultados públicos A-D não mudam. O consumer protegido
`04_wiring/tests/p1292_contract.rs`, SHA-256
`3ced6be4556d5b8b7f79540cc0df12791f38d2e22e6d46a2992f38049043c2f5`,
foi somente lido; código, testes, ataques, pre-gate e veredito não foram
alterados.

## Medição antes da decisão

Referências: vanilla `/usr/local/bin/typst` SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
revisão ratificada `a51e02804`; candidato `target/debug/typst` SHA-256
`f2c065126281a009401b0f73dfd89309de181ed3a7e0832983c7569126329135`.

### B — identidade e extensão inline

`typst eval 'math.underline == underline' --format json` devolveu o boolean
JSON `false` bilateralmente. Logo a identidade é comparada como boolean, não
como stdout de `repr(bool)`.

Com `page(width:auto,height:auto,margin:0pt)` e `$underline(x)$`, o vanilla
produziu raiz SVG `6.292pt × 7.513pt` e uma regra; o candidato produziu
`6.292pt × 7.238pt` e também uma regra. Script/cramped produziram no vanilla
`11.4356pt × 8.459pt` e `6.7276pt × 9.537pt`, contra
`10.8196pt × 7.238pt` e `5.70075pt × 7.238pt` no candidato. A regra existe;
o RED é a extensão vertical da linha.

Fonte vanilla medida: `typst-layout/src/math/mod.rs:49-101`, SHA-256
`88b33ef6eb23deae0e6956c7c59bdbc57604ae1d39a0c6171233f426d87b146b`,
cria o frame inline com baseline/extensão; `inline/line.rs:559-633`, SHA-256
`9da32eee7a101a8e60b70f787d480a78ab4652127bcce02b74a85ad80cf6a54e`,
agrega ascent/descent. O owner cristalino já obtém `EquationExtent` uma vez.
A decisão é chamar exatamente uma vez
`note_inline_extent(extent.ascent, extent.descent)` somente no braço inline,
sem reinspecionar `FrameItem`, medir ou relayoutar.

### C — contexto executado, shape observável

O controle `context { let s = measure([context-ok]); rect(width:s.width,
height:s.height,fill:#123456) }` compilou para SVG bilateralmente e expôs o
shape colorido com `48.29pt × 7.238pt`. A raiz candidata tinha altura distinta,
mas a dimensão do shape coincidiu: o transporte separa `measure` do layout da
página.

No vanilla, `measure(math.equation(math.vec(... gap:10%)), width:100pt,
height:100pt)` materializado como rect mediu `23.5312pt × 22.88pt`; em região
auto, `10%+1em` mediu os mesmos `23.5312pt × 22.88pt`. O candidato executou o
contexto e parou na feature ausente (`math.vec`/suporte regional), portanto o
RED é discriminante. `typst query` de metadata contextual fica refutado e
excluído.

### D — páginas reais e prefixo de floats

O exporter SVG candidato serializa somente a primeira página; template de
nome não cria as páginas restantes. Por autorização explícita do dono, o
transporte D passou a CLI PDF multipágina + `pdftotext -bbox`, sem materializar
exporter novo nem expor pipeline privado. Controle com três pagebreaks gerou
três `<page>` bilateralmente.

Com tokens visuais únicos em `#place([TOKEN])`, que não avançam o fluxo, o
vanilla congelou:

- com flush: `FLOAT_BEFORE` p2/yMin 17.404 (âncora Typst 20),
  `AFTER_MARKER` p3/yMin 4.642 (âncora 7.238), `FLOAT_AFTER`
  p3/yMin 87.404 (âncora 90);
- sem flush: `AFTER_MARKER` p1/yMin 47.842 (âncora 50.438), enquanto
  `FLOAT_BEFORE` permanece p2/yMin 17.404;
- o candidato passou o controle PDF multipágina e falhou no ponto esperado:
  `place.flush` ainda não existe.

O oráculo compara presença única, página, bbox com tolerância pinada e ordem
causal; bytes PDF, IDs, compressão e operadores ficam excluídos.

## Classificação ADR-0107/0108

Boolean de identidade, dimensões/linha visível, página, posição e ordem são
observáveis de linguagem. JSON, SVG, PDF e bbox são transportes mecânicos.
Inferimos que shape e tokens zero-flow não perturbam o objeto medido porque os
controles bilaterais alcançam as âncoras congeladas. Refutam a inferência:
shape divergente de `measure`, marcador alterando página/posição, perda ou
duplicação de token, controle sem contexto/páginas, ou RED anterior à feature.
Qualquer refutação exige parar e ressellar, nunca adaptar expectativa ao
candidato.

## Ownership 1:1 e L0s

- `00_nucleo/prompts/compiler/layout/equation.md` →
  `01_core/src/compiler/layout/equation.rs`; SHA-256 L0
  `859b99ecd8cafd88e9b7bd2159d8f135380c2fb58902860dbb4a70be96586827`.
- `00_nucleo/prompts/wiring/tests/p1292_contract.md` →
  `04_wiring/tests/p1292_contract.rs`; SHA-256 L0
  `41dd2e0ed56814baf94acf565d1fa59eb64f1f11b9d11cd1ea4937a998f2d0a1`.

O segundo owner nasce sem `Hash do Código`, deliberadamente: o autor
independente do oráculo adicionará o header `@prompt`/`@prompt-hash` e
resselará o consumer na sua própria allowlist. O conjunto canônico passa de
20 para 22 L0s.

## Gates e parada causal

Antes da escrita, `crystalline-lint . --checks v15,v26 --fail-on warning`
terminou com `✓ No violations found`. Depois da escrita, o mesmo comando
também terminou com `✓ No violations found`: a ferramenta não reporta o owner
untracked/sem `Hash do Código`; isso não elimina a dívida de lineage explícita
acima. Não houve outro V15/V26.

Contrato público A-D: inalterado. Vetores/resultados: inalterados. Mecanismo
de comparação: v3. Próxima escrita causal pertence ao autor independente do
oráculo, que deve adaptar apenas o consumer protegido, adicionar lineage e
provar RED discriminante antes da implementação. **PARAGEM.**

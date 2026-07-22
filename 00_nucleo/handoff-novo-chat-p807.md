# Estado do projecto typst-crystalline — handoff para novo chat (pós-P807)

**Data:** 2026-07-21 (actualizado pós-P810)
**Último passo fechado:** P810 (triagem lote 4 — 15 módulos, 15 achados na fila)
**Handoff anterior:** `00_nucleo/handoff-novo-chat-p798.md` (cobre até P798 — este documento cobre P799 em diante, não substitui o anterior)
**Binários de referência:** `./target/release/typst` (cristalino), `lab/typst-original/target/release/typst` (vanilla)
**Estado final da suíte (confirmado em todos os relatórios P799-P807):** `typst-core 4336 passed / 1 ignored`, `typst-infra 657/5i`, `typst-shell 33`, `CLI bin 2`, `cli.rs 29`, `crystalline_lint 2` — zero falhas.
**Estado da suíte pós-P809:** `typst-core 4348 passed / 1 ignored`, `typst-infra 658/5i` (P808 e P810 não adicionaram código/testes).
**Nota de proveniência:** o commit `c98ffc8ac` (17:36, sessão paralela) empacotou P798–P807; os relatórios de materialização P799–P806 foram eliminados fisicamente da working tree pela mesma sessão — estão preservados nesse commit.

---

## Auditoria destes 10 relatórios (P799, P800, P801, P802, P803, P804, P805, P805a, P806, P807)

Antes de resumir o conteúdo, uma verificação da disciplina exigida (comando + saída literal, contagem de testes a bater):

- Todos os 9 relatórios com código novo trazem comando/saída literal (mensagens de erro exactas, `mutool trace`, `pdftotext`, `repr`) comparando vanilla e cristalino. P807 (sem código, decisão de escopo) não precisa disso — cumpriu o que o próprio prompt pedia para esse caso.
- Contagem de testes: verificada passo a passo, os números encadeiam sem buraco —
  `4317→4319 (P799, +2) →4320,1i (P800, +2, sendo 1 novo teste marcado #[ignore]) →4321,1i (P801, +1) →4323,1i (P802, +2) →4324,1i (P803, +1) →4328,1i (P804, +4) →4329,1i (P805, +1) →4336,1i (P806, +7)`.
  A soma de testes novos declarados em cada relatório bate com a diferença ANTES/DEPOIS em todos os casos.
- `typst-infra`: `656,5i→657,5i` em P805a (+1), reflectido correctamente também no relatório de P805 (que o cita como parte do fecho da validação).
- Nenhum relatório afirma "corrigido" sem mostrar a execução.

Não encontrei inconsistências. Achado a registar (não é problema, é informação): P800 introduziu um teste antigo marcado `#[ignore]` em vez de apagado — é o teste que consagrava a regra do Passo 48, entretanto revogada por medição. A justificação e o ponteiro para o substituto estão no próprio código, conforme o relatório.

---

## Linha do tempo: P799 → P807

Todos fecham achados da fila de P798 (achados #13, #7, #4, #5, #8, #9, #10, #12, #11 — faltava o #6, código morto, que já não precisava de acção).

### P799 — `math::attach`, achado #13 (prioridade alta)
Sub/superscript simultâneos (`$x^2_3$`) compunham-se em sequência horizontal em vez de empilhados; a largura do attach ignorava o sub, sobrepondo o glifo seguinte. Corrigido: os dois scripts partilham a origem x; largura do attach = base + max(sup, sub). Testadas e **refutadas** como mesma causa: achado #7 (foi P800, causa diferente) e a observação de itálico matemático de P786 §7 (continua em aberto, causa diferente).

### P800 — `syntax::kind`, achado #7
Isolamento confirmou que `#if` era irrelevante — o problema já existia em `$x^2$` sozinho depois de texto (`Hello $x^2$`). Causa real: a equação inline estava desalinhada do baseline do texto por `axis_pt` inteiro. A regra do Passo 48 ("alinhar o eixo matemático à baseline do texto") estava **errada** — medição confirmou que o vanilla alinha a **baseline**, não o eixo. Regra revogada; teste antigo marcado `#[ignore]` com justificação (não apagado). Residual confirmado e registado como causa distinta, ainda aberto: `x` vs `𝑥` (itálico matemático não estilizado — mesma observação de P786 §7).

### P801 — `utils::protected`, achado #4
Array de 1 elemento imprimia sem a vírgula final (`(0)` em vez de `(0,)`). Corrigido na rotina de `repr` (regra vanilla: vírgula final só quando `len()==1`). 0 e 2+ elementos já estavam correctos e continuam.

### P802 — `utils::listset`, achado #5
Label órfã (`<abc>` sem elemento seguinte) não emitia warning. Corrigido no braço de avaliação de label em markup. Confirmado por medição que `Hello <abc>` (label depois de texto) não deve avisar em nenhum dos dois binários — esse caso é uma divergência diferente e já registada (P791 §6, label em texto não indexada pelo introspector), fora do âmbito deste passo.

### P803 — `visualize::curve`, achado #8
Mensagem de erro de `curve()` com argumento do tipo errado divergia do vanilla. Causa era local a `native_curve`, que contornava a validação genérica de tipo já usada no resto do projecto. Corrigido para usar a mesma validação genérica. Observado e registado fora de âmbito: o vanilla acumula um diagnóstico por argumento inválido (2 erros); o cristalino pára no primeiro.

### P804 — `visualize`, achado #9
`#line(length:, angle:)` estavam ausentes. A sonda **refutou** uma previsão do prompt original: o vanilla não rejeita `length`/`angle` combinados com `end` — simplesmente ignora-os nesse caso. Implementado com essa semântica (medida por geometria, `mutool trace`, não só ausência de erro).

### P805 — `text::lorem_`, achado #10
A medição alargou o achado (registado como ADR-0108): não era só falta de ponto final — o cristalino usava um vocabulário cíclico próprio (scope-out do Passo 391), o vanilla usa uma cadeia de Markov determinística (crate `lipsum` 0.9.1, seed fixo). Decisão: byte-parity — revoga o scope-out do Passo 391 e adopta a mesma crate. Nova dependência L1 autorizada em `crystalline.toml`.

### P805a — sub-passo nascido da validação de P805
Bug bloqueante separado, descoberto ao validar `#lorem(30)`: ligaduras "fi"/"ffi" não tinham entrada no ToUnicode CMap no caminho de embed integral de fonte CFF (fallback do Passo 797) — o glifo renderizava correcto mas a extracção de texto perdia os caracteres. **Afecta qualquer documento**, não só `lorem`. Corrigido em `03_infra/src/export/builder.rs`.

### P806 — `model`, achado #12
`#par[...]` como função dava `unknown variable: par`. Decisão registada: `Content::Par` **não foi criada** — parágrafos continuam implícitos; `native_par` devolve o body directamente no caso standalone (equivalente observável). `Content::Par` fica candidato futuro só para quebra de parágrafo a meio de bloco (caso que o vanilla trata e o cristalino ainda não). Argumento `leading:` liga-se ao canal custom já existente (`#set par`); os restantes argumentos nomeados são aceites e ignorados, com a limitação registada (funções nativas não têm acesso ao `Sink` para emitir warning de "ignorado").

### P807 — `pdf::attach`, achado #11
Não era passo de correcção — pedia decisão do dono antes de implementar. Levantamento mostrou que o scope-out nunca tinha sido formalizado (nem ADR, nem entrada de dívida). Decisão do dono: **manter o scope-out** (Opção B), alinhado com os precedentes de SVG e PDF-como-imagem. Formalizado como **DEBT-66**, com critério de reabertura explícito (pedido do dono ou caso de uso real em corpus, ex. ZUGFeRD/Factur-X). Sem código novo.

---

## Estado actual — o que ficou aberto ou merece atenção

### Achados de P798 — todos fechados
Dos 9 pendentes, 8 corrigidos com código, 1 (`pdf::attach`) formalizado como débito consciente (DEBT-66). O achado #6 (`utils::deferred`, código morto) já não precisava de acção desde P798.

### Itens novos que ficaram registados como abertos durante este lote
- ~~**Itálico matemático não estilizado** (`x` vs `𝑥`)~~ — **FECHADO em P809** (2026-07-21): default de itálico via codepoint (codex `MathStyle::select`) aplicado uma vez no topo de `layout_equation`; Greek coberto para `Plain`; `bold(x)` compõe bold-italic; bug adjacente de truncagem UTF-16 no ToUnicode corrigido. Scope-out registado: formas de símbolo gregas, `∂`/`∇`, Greek com kinds não-Plain. Relatório: `00_nucleo/diagnosticos/paridade-producao-p809.md`.
- **`curve()` só reporta o primeiro erro por chamada**, vanilla acumula todos — divergência registada em P803, fora de âmbito.
- **Regra do Passo 48 revogada** (P800) — **varredura P808 concluída**: zero citações activas da regra antiga (classe c); **achado novo registado**: equação em bloco não é centrada horizontalmente e tem espaçamento vertical mais apertado que o vanilla — candidato a passo dedicado (ver `00_nucleo/diagnosticos/paridade-producao-p808.md`).
- **DEBT-66** (novo) — `pdf.attach` scope-out formalizado. Contagem de dívidas abertas: 6 → 7.

### Débitos grandes já conhecidos, sem mudança
- SVG (`image::svg`) — scope-out desde P772k.
- PDF-como-imagem — scope-out desde P781.
- Repeat-across-páginas de `grid.header`/`grid.footer`/`table.header`/`table.footer` — débito reafirmado pelo menos três vezes.
- `pdf.attach()` — agora DEBT-66 (ver acima), já não é decisão pendente.

### Achados menores registados, sem correcção (herdados do handoff anterior, sem mudança)
- `table.cell(x:,y:,colspan:,rowspan:)` — posições explícitas ignoradas no placement.
- Conflito de célula com footer (só header coberto).
- Avisos de depreciação de símbolos — falta fonte de dados.
- Span em `Args` incompleto.
- `place(float: true)` com decoração — best-effort.
- `#label()` como função não intercepta show rules.
- Grupos de símbolos sem uso confirmado no corpus.

### Varredura sistemática — progresso
P810 (lote 4) triou 15 módulos — **taxa de sinal 100%** (15 achados, com núcleos funcionais em paridade em vários deles). Snapshot actual da lista: 82 módulos em `lacuna-inventario`; 45 (P785–P798) + 15 (P810) = 60 triados. **Restam 22 módulos para o lote 5** (maioritariamente internos/mecânicos: `text::font::*`, `layout::*` internos, `visualize::image::*` já decididos, `utils::pico::*`, `syntax::span`, `foundations::styles::rule`, defines).

### Achados de P810 (lote 4), aguardando passo dedicado — nenhum corrigido ainda

| # | Módulo | Achado |
|---|---|---|
| 1 | `typst_eval` | `#eval` sem `mode:`/`scope:`; erros de sintaxe/tipo genéricos + span detached |
| 2 | `typst_eval::methods` | método inexistente: caminho normal diverge; dict-key-call sem hints dedicados |
| 3 | `typst_library::diag` | `#set` prop inválida → warning (exit 0) em vez de erro; sem warning "unknown font family"; `set text(size: <int>)` aceite em silêncio |
| 4 | `foundations::calc` | asin/acos/atan/atan2 float vs `angle`; quo trunc vs floored; pow int-neg erro; decimal não suportado; log10/deg/rad extra; precisão erf/log/exp |
| 5 | `foundations::ops` | ordenação str/array/bool ausente; div Relative/Relative e Ratio/Ratio; Str*Int; eq/ord Length↔Relative; coerção Int↔Float não aninhada |
| 6 | `foundations::plugin_` | `plugin.transition` ausente (não registado no L0); mensagens L1 divergentes; spans detached; texto erro parse WASM |
| 7 | `foundations::scope` | `Deprecation` ausente (`join` erro vs warning; `bowtie` ausente); mensagem math em português |
| 8 | `foundations::target_` | `#target()` fora de `#context` não erra; texto de erro com args diverge; colateral: `#context type()` vazio |
| 9 | `layout::grid::resolve` | header/footer não repetem (débito reconfirmado); mensagens de erro divergentes; footer fora do fim aceite |
| 10 | `loading::cbor_` | mensagem de erro CBOR diverge (Debug vs texto amigável + ficheiro + span) |
| 11 | `loading::read_` | `encoding:` rejeitado (língua válida); não-UTF8 devolve bytes em silêncio (comentário "heurística vanilla" refutado) |
| 12 | `typst_library::math` | domínio de classes 15 vs 10; field access bare em math compila; fence spaced/class("normal") sem efeito (scope-out L0); LeftRightAlternator em mat ausente |
| 13 | `math::style` | ~~display/inline/script/sscript sem efeito geométrico; itálico P809 perdido em wrappers de tamanho; scr bloco errado + sem variation selectors~~ **FECHADO P812** (factor aplicado; eixos ortogonais; `scr`=script+VS2, `cal`=script+VS1 byte-idêntico); ~~frak() sem arg + PDF corrompido~~ **FECHADO P811**; ~~NN/RR/ZZ/QQ/CC ausentes~~ **FECHADO P812-D**; `#math.display` via módulo ausente; `display(∑)` variante grande de operadores — scope-out registado em P812 |
| 14 | `pdf::accessibility` | `pdf.artifact(kind:)` rejeitado (vanilla aceita — só tag tree; reclassificação a decidir pelo dono) |
| 15 | `typst_syntax::package` | erro "pacote não encontrado" não-preview diverge (mensagem PT desactualizada) |

**Transversais**: PDF corrompido com equação vazia (pista de bug de export); `bytes()`/`float.nan`/`datetime()` ausentes; `array.join` ausente; spans `<detached>` generalizados; variation selectors (U+FE0E/FE00/FE01) nunca emitidos.

**Prioridades sugeridas pela medição**: PDF corrompido; math::style sem efeito geométrico; calc → `angle`; `#set` prop inválida = exit 0; `Deprecation`. Achado de P808 (equação em bloco não centrada + espaçamento apertado) também por atribuir.

---

## Recomendação para o próximo chat

1. **Atacar a fila de P810 pelas prioridades medidas**: ~~PDF corrompido~~ (P811), ~~math::style~~ (P812 — restam só `#math.display` via módulo e `display(∑)` operador grande), calc `angle`, `#set` prop inválida = exit 0, `Deprecation`. O achado de P808 (equação em bloco sem centragem/espaçamento) também aguarda passo dedicado.
2. **Decidir entre continuar a fila de P810 (15 achados) ou o lote 5 de triagem (22 módulos restantes, maioritariamente mecânicos)** — a experiência dos lotes anteriores sugere esgotar primeiro os achados de maior superfície.
3. **Manter a disciplina de execução mostrada** — nenhum dos 10 relatórios desta fase falhou nisso; não relaxar.
4. **Perguntar a origem de qualquer relatório inesperado**, dado o padrão já confirmado de execuções paralelas fora desta conversa (ver handoff anterior). Nota: a sessão paralela commitou P798–P807 em `c98ffc8ac` e eliminou fisicamente os relatórios de materialização P799–P806 da working tree — estão preservados no commit; verificar antes de os recriar.

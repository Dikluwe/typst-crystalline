# Inventário de dívida técnica

> Reorganização aplicada no Passo 83.5: três secções — abertos, encerrados,
> instrumentação. Texto de cada entrada preservado tal como estava antes da
> reorganização. Numeração descontínua é intencional (DEBT-4, DEBT-5 nunca
> existiram; DEBT-24a, DEBT-34a idem).

> **Auditoria Passo 125 (2026-04-24)**: 11 DEBTs abertos revistos
> com grep empírico em L1. **Todos confirmados M** (manter).
> Zero fechos triviais; 3 candidatos de fecho dedicado identificados
> (DEBT-43, DEBT-42, subset de DEBT-1). Desde Passo 105: 3 DEBTs
> encerrados (DEBT-45/49/51). Detalhe em
> [`diagnosticos/auditoria-debts-passo-125.md`](diagnosticos/auditoria-debts-passo-125.md).
>
> **Passo 135 (2026-04-24)**: abriu **DEBT-52** como rastreador de
> consumer integral de `StyleDelta` em layout — pré-requisito para
> fecho de DEBT-1 por ADR-0054. Total abertos: **11 → 12**. Detalhe
> em [`diagnosticos/diagnostico-shaping-passo-135.md`](diagnosticos/diagnostico-shaping-passo-135.md).
>
> **Passo 142 (2026-04-24)**: fecho formal de **DEBT-1** após
> cumprimento dos critérios de ADR-0054 (consumer activo em 9/10
> campos de `StyleDelta`; `lang` em scope-out por perfil
> observacional graded). **DEBT-52** encerrado simultaneamente
> (rastreador cumpriu a função; gaps 7 e 8 ficam como candidatos
> futuros não-DEBT). Total abertos: **12 → 10**. Relatório formal
> em [`relatorios/fecho-debt-1-passo-142.md`](relatorios/fecho-debt-1-passo-142.md).
>
> **Passo 150 (2026-04-25)**: aberto **DEBT-53** durante
> materialização da primeira matriz agregada de paridade
> (`lab/parity/`). Infraestrutura entregue (FrameDTO + report +
> tests + corpus expandido); integração do pipeline vanilla
> ficou pendente. Total abertos: **10 → 11**.
>
> **Passo 151 (2026-04-25)**: aberto **DEBT-54** ao tentar
> fechar DEBT-53. Investigação revelou que setup vanilla
> workspace é pré-condição não-trivial (~12 path-deps + ~30
> externas via `lab/Cargo.toml`). DEBT-53 fica bloqueado por
> DEBT-54; materialização efectiva passa a passo posterior.
> Total abertos: **11 → 12**.
>
> **Passo 152 (2026-04-25)**: refino administrativo do plano
> DEBT-54 (probe online dos 3 crates flagged + risco de
> versões + critério expandido em 3 níveis). Probe revelou
> `codex`/`hayagriva`/`oxipng` **todos em cache local**
> (estimativa P151 desactualizada); identificado conflito
> material em `comemo` (cristalino 0.4 vs vanilla 0.5,
> cargo aceita duplicação). Saldo de DEBTs **inalterado**
> (12).
>
> **Passo 154A (2026-04-25)**: diagnóstico Model
> (structural). Cobertura empírica revisada de 38% → 32-36%
> (10 ausentes / 22 entradas). **DEBT-55 aberto**
> (bibliography + cite XL; bloqueado por ADR-0061 hayagriva
> a criar em passo dedicado). **ADR-0060 PROPOSTO** (roadmap
> Fase 1 / 2 / 3 — passos 154B/155/156/157/158+).
> Total abertos: **12 → 13**.
>
> **Passo 156B (2026-04-25)**: diagnóstico Layout (Fase X) —
> oitava aplicação do padrão diagnóstico-primeiro; primeira
> a categoria Layout. Cobertura empírica recalculada de 38%
> declarado para **22% implementado puro** (4/18); 11 entradas
> ausentes; +2 entradas adicionadas (`h`/`v` e `skew` não
> estavam no §A.5 do inventário 148). **DEBT-56 aberto**
> (column flow Fase 3 Layout L+; refactor multi-region do
> Layouter exigido; ADR dedicada futura). **ADR-0061 criada
> em PROPOSTO** (Layout roadmap). **Renumeração de Fase 2
> Model**: P156→P157, P157→P158, P158→P159; reserva
> hayagriva passa de ADR-0061 para **ADR-0062**. DEBT-55
> actualizada. Total abertos: **13 → 14**.

---

## Secção 1 — DEBTs em aberto ou parcialmente resolvidos

## DEBT-2 — Closures eager vs lazy capture — PARCIALMENTE RESOLVIDO

### Resolvido no Passo 31

- `ClosureRepr::captured` mudou de `IndexMap<String, Value>` (clone eager O(N)) para `Arc<Scope>`
- Captura no momento da definição: snapshot O(N) uma única vez, depois partilhado em O(1)
- `apply_closure` usa `Scopes::with_parent(Arc::clone(&captured))` — lookup sem clone dos valores
- `eval_let` trata `LetBindingKind::Closure`: sintaxe `#let fib(n) = ...` agora funciona
- `Expr::Closure` arm lê `closure_expr.name()` — nome propagado correctamente para recursão

### Divergência residual

- Semântica de captura: ainda eager (snapshot). `#let x=1; #let f()=x; #let x=2; f()` retorna `1`
  (snapshot), não `2` (lazy). O original via `comemo` retornaria `2`.
  **Confirmado no Passo 31**: o snapshot é uma cópia independente do scope, não uma referência
  partilhada. Divergência semântica documentada com o original. Não bloqueante.
- A integração com `comemo` para tracking semântico real aguarda `TrackedWorld` real.
- Registado como sub-DEBT se cenários avançados de shadowing forem encontrados nos testes de paridade.

### Pendente

- Integração com `comemo` para tracking semântico real
- Testes de paridade com o original para cenários avançados de shadowing

---

## DEBT-8 — Motor de equações — PARCIALMENTE RESOLVIDO

**Parcialmente resolvido no Passo 37**. Registado no Passo 34.

**Resolvido no Passo 36**:

- `MathLayouter` criado em `rules/math/layout.rs` (L1 puro)
- `MathIdent` e `MathText` → `FrameItem::Text` (sem placeholder)
- `MathFrac`, `MathAttach`, `MathRoot` → texto plano sem `[...]`
- Placeholder `[equação]` removido do layouter principal
- `Content::Equation` delega ao `MathLayouter` para inline e block

**Resolvido no Passo 39**:

- `rules/math/symbols.rs` com `ident_to_unicode`, `shorthand_to_unicode`,
  `is_math_function`, `is_single_letter_var`
- `Expr::MathIdent` em `eval_math_expr`: símbolo conhecido → `Content::MathText(unicode)`
- `MathShorthand::get()` no AST já retorna o char Unicode — sem alteração necessária
- Variáveis de uma letra → `TextStyle { italic: true }` em `MathLayouter::layout_node`
- Funções (sin, cos, lim, etc.) → `TextStyle { italic: false }`

**Resolvido no Passo 38**:

- `FrameItem::Line { start, end, thickness }` adicionado a `layout_types.rs`
- `layout_frac` emite `FrameItem::Line` para a linha horizontal entre num/den
- `MathBox::place()` e `hconcat()` propagam offsets para `FrameItem::Line`
- `export.rs` trata `FrameItem::Line` com operadores PDF `q w m l S Q`
- `frac(a, b)` em `eval_math_expr` via `Expr::FuncCall` → `Content::MathFrac`
- `MathDelimited` em `eval_math_expr` inclui `open` e `close` como `MathText`

**Resolvido no Passo 37**:

- `MathBox { width, ascent, descent, items }` como unidade de layout
- `MathFrac`: numerador acima da baseline, denominador abaixo,
  tamanho 70% do texto base. Sem linha de fracção (Passo 38+)
- `MathAttach`: sup elevado a 50% do ascender, sub baixado a 30%
  da descida, tamanho 65% do texto base
- `hconcat`: concatenação horizontal de `MathBox`es
- `MathLayouter` refactorizado para stateless (`&self`)
- `layout_equation` retorna `Vec<FrameItem>` (posições relativas)
- Integração em `layout.rs` usa `y: offset_y + pos.y` para
  posicionamento vertical correcto

**Resolvido no Passo 40**:

- `sqrt(x)` como função nativa em `eval_math_expr` → `Content::MathRoot { index: None }`
- `root(n, x)` como função nativa → `Content::MathRoot { index: Some(n) }`
- Validação de aridade: `sqrt()` e `sqrt(x,y)` retornam `Err`; `root(3)` retorna `Err`
- `MathLayouter::layout_root`: símbolo `√`, overline (`FrameItem::Line`), radicando posicionado à direita
- `offset_item` helper adicionado em `math/layout.rs`
- `sqrt` e `root` adicionados a `is_math_function` (renderizados sem itálico)

**Ainda pendente**:

- Kern matemático entre símbolos
- Fontes OpenType MATH (tabelas MATH, variantes de tamanho)
- `MathPrimes` (parseado e evaluado em `eval.rs`, sem lógica de kern/posição no layouter)
- Baseline correcta em relação ao x-height da fonte

### Nota — actualização no Passo 84.1

`MathAlignPoint` foi removido da lista de pendências — verificação no
Passo 83.5 confirmou implementação completa em `math/layout.rs`,
`eval.rs` e `layout/mod.rs`. A entrada de `MathPrimes` foi clarificada
para indicar o estado parcial (parseado mas sem lógica de layout
dedicada).

---

## DEBT-9 — Cobertura de paridade — tracking contínuo

**Estado**: Baseline estabelecido no Passo 35. Sem divergências no corpus actual.

**Descrição**: O parity_runner testa 50 inputs (40 markup/code/math gerais + 10 math
específicos do Passo 34). Todos passam. À medida que o motor de equações e novas
funcionalidades forem implementadas, adicionar casos de paridade correspondentes.

**Quando expandir**: A cada passo que adicione novo SyntaxKind ou altere semântica do parser.

**Referência**: `lab/parity/tests/parse_parity.rs`, baseline em
`00_nucleo/materialization/parity-baseline-passo-35.md`

---

## DEBT-33 — Bounding Box de curvas Bézier (Passo 79) — EM ABERTO

A bounding box de `ShapeKind::Path` é calculada verificando o min/max dos pontos
de controlo. Para `CubicTo`, a curva real pode ultrapassar a caixa delimitadora
dos pontos de controlo, causando vazamento visual subtil. Resolução futura:
cálculo analítico dos extremos da curva paramétrica B(t) para obter a AABB exacta.

---

## DEBT-34d — Auto não encolhe antes de matar fr — EM ABERTO (Passo 80)

Um Auto guloso (célula com texto longo) pode consumir todo o `safe_available`,
deixando 0pt para as colunas fr. Resolução futura: implementar min-content e
max-content para Auto, com negociação entre Auto e fr.

---

## DEBT-34e — colspan e rowspan — EM ABERTO (Passo 80)

Células que ocupam múltiplas colunas ou linhas requerem um algoritmo de
placement diferente. Resolução: passo futuro.

---

## DEBT-35b — Invalidação de cache de available_width após SetPage — EM ABERTO (Passo 81)

Se alguma função guardar available_width em cache como campo do Layouter,
esse cache tem de ser invalidado no processamento de Content::SetPage.
Actualmente available_width() é calculado em tempo real sem cache —
este DEBT documenta o risco caso um cache venha a ser adicionado.

---

## DEBT-42 — `get_unchecked` no scanner — EM ABERTO (Passo 84.8a, bloqueado)

`01_core/src/rules/lexer/scanner.rs` tem 7 ocorrências de
`unsafe { self.string.get_unchecked(start..end) }`. Herdado de
`unscanny` via ADR-0014.

ADR-0032 estabelece que `unsafe` em L1 é eliminado por defeito, com
excepção permitida permanentemente apenas se benchmark
reprodutível demonstrar regressão inaceitável ao eliminar o
`unsafe`, e se um ADR específico registar o número concreto.

### Bloqueio

Este DEBT depende de **infra de benchmarking reprodutível no
projecto**, que ainda não existe. Sem a infra, não é possível
aplicar o critério da ADR-0032 de forma honesta.

### Plano de resolução

1. **Pré-requisito**: criar infra de benchmark para lex e parse.
   Pode ser `criterion` crate (já usada na comunidade Rust) ou
   alternativa. ADR específico se a decisão sobre qual biblioteca
   envolver dependências novas em L1 ou se o benchmark vive fora
   de L1.

2. **Medição baseline**: com `get_unchecked` tal como está actual.
   Executar benchmark sobre conjunto de documentos representativos
   (documentos simples, documentos com muito texto, documentos
   matemáticos).

3. **Refactor experimental em branch**: substituir `get_unchecked`
   por `&self.string[start..end]` e medir.

4. **Decisão**:
   - Se regressão < 5% no tempo total de lex: eliminar `unsafe`,
     fechar DEBT.
   - Se regressão entre 5% e 20%: decisão do utilizador com base
     em contexto (documentos alvo, uso do Typst).
   - Se regressão > 20%: manter `unsafe` no scanner como excepção
     permanente, escrever ADR específico citando a medida.

### Critério de conclusão

Uma de duas:
- Zero ocorrências de `unsafe { get_unchecked(...) }` em `scanner.rs`.
- ADR específico escrito que autoriza permanência do `unsafe` com
  número concreto de regressão medida.

### Dependências

- Infra de benchmark — trabalho próprio, não coberto por este
  bloco de passos.

---

## DEBT-43 — Linter: whitelist crate-level em vez de type-level — EM ABERTO (Passo 89)

O `crystalline.toml` usa whitelist crate-level para externos
autorizados em L1 (secção `[l1_allowed_external]`). Isto significa
que, se uma crate tem ao menos um tipo autorizado por ADR, **qualquer**
tipo dessa crate passa o linter — mesmo tipos cujo uso não foi
autorizado.

Exemplo concreto identificado no Passo 87:

- ADR-0024 autorizou `ecow::EcoString` para `Value::Str` (pontual).
- ADR-0035 autoriza `ecow::EcoVec` (Passo 89) mas **não** `ecow::EcoMap`,
  `ecow::EcoArc` ou outros tipos da crate.
- `use ecow::EcoMap` passaria hoje o linter sem reportar violação,
  porque o nome `ecow` consta do array autorizado.

A disciplina de respeitar o escopo dos ADRs type-específicos é humana
(revisão de código), não automática. Este DEBT regista o gap. O
enforcement arquitectural continua efectivo via revisão manual e pela
cultura de materialização precedida por ADR + diagnóstico
(ADR-0033/0034), mas fica explícito que o guardião automático é
incompleto.

### Proposta de resolução

Estender o formato do `crystalline.toml` para aceitar whitelisting
por tipo:

```toml
[l1_allowed_external.ecow]
types = ["EcoString", "EcoVec"]
```

Em vez do actual `ecow` estar apenas listado no array de crates
autorizadas sem granularidade.

**Escopo em dois repositórios**:

1. **`crystalline-lint` (projecto separado, guardião arquitectural)**
   — alterar o parser da configuração e a lógica de verificação para
   ler e aplicar o novo formato type-level. Sem esta alteração, o
   novo formato no `crystalline.toml` é ignorado pelo binário actual.

2. **Typst cristalino (este repositório)** — migrar o
   `crystalline.toml` para o novo formato, crate por crate, partindo
   das autorizações já estabelecidas pelos ADRs (0010-0013 para
   crates unicode, 0018 para `rustc_hash`, 0023 para `indexmap`,
   0024 para `EcoString`, 0035 para `EcoVec`).

O trabalho no `crystalline-lint` é pré-requisito do trabalho neste
repositório — o `.toml` só tem efeito depois de o binário saber
interpretá-lo.

### Dependências

- Trabalho no repositório `crystalline-lint`: alteração do parser de
  configuração e da lógica de verificação. Passo dedicado nesse
  projecto, não neste.
- Após o `crystalline-lint` aceitar type-level whitelisting: passo
  neste repositório para migrar `crystalline.toml` para o novo
  formato e adicionar teste de violação negativa (tipo da mesma
  crate não listado deve ser reportado).

### Nota sobre escopo

Este DEBT **não** afecta correcção funcional do código Typst. Afecta
apenas enforcement automático de decisões arquitecturais. É trabalho
de infraestrutura, não de domínio.

### Critério de conclusão

- [ ] `crystalline.toml` aceita whitelist type-level para externos.
- [ ] Pelo menos uma crate (sugestão: `ecow`) migrada para o novo
      formato.
- [ ] Tipo não autorizado dessa crate é reportado como violação em
      teste do `crystalline-lint`.
- [ ] Documentação actualizada no README do `crystalline-lint`.

---

## DEBT-56 — Column flow Fase 3 Layout (L+; refactor multi-region do Layouter) — EM ABERTO (Passo 156B)

**Aberto em**: Passo 156B (2026-04-25) durante diagnóstico
Layout (Fase X).
**Bloqueia**: Fase 3 do roadmap Layout (ADR-0061) — `columns()`
e `colbreak()` ficam ausentes até ser materializado.
**Bloqueado por**: nada técnico imediato; **decisão humana
de priorizar Fase 3 Layout**. ADR dedicada (column flow
algorithm) a criar quando materializado.

### Contexto

Diagnóstico Layout (P156B) classificou `columns()` como **L+**
— escopo significativamente maior que outras features Layout
porque exige refactor profundo do Layouter:

- Vanilla implementa multi-column como `Regions` com width
  reduzida (`width / count - gutter`), iterando colunas como
  páginas sequenciais (in-tree em
  `lab/typst-original/crates/typst-layout/src/flow/`, ~3000
  linhas).
- Cristalino actual escreve directo em `current_items` da
  página (sem abstração `Region`/`Regions`); não suporta
  multi-region iteration.
- Refactor mínimo: introduzir abstracção `Region`-like e fazer
  Layouter consumir uma sequência de regions; column flow vira
  caso especial de regions reduzidas.

Vanilla **não faz balanceamento** de altura de colunas (pelo
comentário do source); cristalino pode adoptar mesma simplificação.

### Diferença face ao vanilla

Vanilla: `ColumnsElem { count, gutter, body }` em
`lab/typst-original/crates/typst-library/src/layout/columns.rs`
(declarativo, 103 linhas) + algoritmo em
`lab/typst-original/crates/typst-layout/src/flow/`.

Cristalino: zero — sem `Content::Columns`, sem
`Content::Colbreak`, sem multi-region no Layouter.

### Pré-requisitos

1. **ADR dedicada** column flow algorithm (autorização da
   abordagem multi-region; análoga a ADR-0044 Engine que
   inverteu parcialmente ADR-0036 sem revogar).
2. **Refactor `Layouter`** — introduzir
   `Region`/`Regions`-like abstraction.
3. **`Content::Columns { count, gutter, body }`** + variant
   `Content::Colbreak { weak: bool }`.
4. **`native_columns` + `native_colbreak`** em stdlib.
5. **Layouter consumer** — reduzir width, iterar colunas como
   páginas dentro do flow.
6. **Compatibilidade com page break** — colbreak precedência
   vs pagebreak; testes de mistura.

### Plano

Materializar em **passo dedicado** (escopo L+; ~5-8h):

- [ ] ADR dedicada column flow algorithm (PROPOSTO).
- [ ] Refactor minimal `Layouter` para multi-region.
- [ ] `Content::Columns` + `Content::Colbreak` variants.
- [ ] `native_columns` + `native_colbreak` em stdlib.
- [ ] Consumer no Layouter.
- [ ] 5-10 testes (single-column unchanged; 2-col / 3-col
  texto curto; column overflow; colbreak; mistura com
  pagebreak; mistura com `#set page`).
- [ ] Inventário 148 reclassifica `columns` e `colbreak`
  de `ausente` para `implementado` (perfil graded — sem
  balanceamento).
- [ ] ADR de column flow transita PROPOSTO → IMPLEMENTADO.
- [ ] DEBT-56 fecha.

### Critério de fecho

- [ ] ADR column flow `IMPLEMENTADO`.
- [ ] `columns()` + `colbreak()` materializados em cristalino
  (sem balanceamento, paridade vanilla).
- [ ] Tests verdes; lint zero.
- [ ] Inventário 148 actualizado.
- [ ] ADR-0061 (Layout roadmap) Fase 3 marca columns/colbreak
  como completos.

### Notas

- **Não bloqueia Fase 1 Layout** (P156C — page model footnote
  area + pad + hide + pagebreak + h + v) nem **Fase 2 Layout**
  (block + box + stack). Pode ser materializado em paralelo
  com Fase 2 ou após.
- **Não bloqueia Fase 2 Model** (P157 table + P158 figure-kinds
  + P159 bibliography) — features Model são independentes do
  column flow.
- **`measure(body)` e `layout(callback)`** continuam dependentes
  de ADR-0017 (Introspection runtime), não desta DEBT.
- **Refactor Layouter** é candidato natural a ser feito em duas
  fases: (a) introduzir `Region`/`Regions` mantendo
  comportamento single-column; (b) consumir multi-column.
  Cada fase pode ser passo independente.
- **Sem novas crates externas** — column flow é trabalho L1
  puro com tipos existentes (per §5 do diagnóstico P156B).

---

## DEBT-55 — Bibliography + Cite (XL; pré-condição ADR-0062 hayagriva) — EM ABERTO (Passo 154A; renumerada por Passo 156B)

**Aberto em**: Passo 154A (2026-04-25) durante diagnóstico
Model.
**Actualizado em**: Passo 156B (2026-04-25) — renumeração da
reserva ADR (era ADR-0061 → agora ADR-0062) e do passo de
materialização (era P158 → agora P159) por reocupação de
ADR-0061 por Layout roadmap em P156B.
**Bloqueado por**: **ADR-0062** (autorização da crate
`hayagriva`, ainda não criada — referência condicional em
ADR-0060 anotada). Era ADR-0061 antes da reocupação por
Layout em P156B; reserva hayagriva foi deslocada para
ADR-0062 sem alteração de conteúdo.

### Contexto

Diagnóstico Model (P154A) classificou `bibliography` + `cite`
como **XL** — escopo significativamente maior que outras
features Model:

- Vanilla integra `hayagriva` (CSL parser + style engine +
  reference database) profundamente.
- `BibliographyEntryElem` modela cada referência de forma
  estruturada.
- `CiteElem` invoca CSL para formatar citação consoante
  estilo configurado.

Cristalino actualmente **não suporta** nenhuma. Inventário
148 §A.6 lista ambas como `ausente`.

### Diferença face ao vanilla

Vanilla: `BibliographyElem` + `CiteElem` em
`lab/typst-original/crates/typst-library/src/model/{bibliography,cite}.rs`.

Cristalino: `Content` enum sem variants `Bibliography`/`Cite`;
sem `native_bibliography` nem `native_cite` em stdlib.

### Pré-requisitos

1. **ADR-0062** (renumerada de ADR-0061 em P156B) —
   autorização `hayagriva` em L1 ou L3.
   Precedentes: ADR-0024 (ecow), ADR-0023 (indexmap),
   ADR-0057 (hypher). Crate 0.9.1 já em cache local
   (probe P152). Localização L1 vs L3 a decidir conforme
   API real (CSL parser puxa I/O ou aceita strings em
   memória?).
2. **`Content::Bibliography` + `Content::Cite`** variants
   novas (per ADR-0026 perfil).
3. **`native_bibliography` + `native_cite`** em stdlib.
4. **Pipeline introspect** consume `Cite` + `Bibliography`
   para resolução cruzada (similar a `Ref`).
5. **Render layout** para ambos.

### Plano

Materializar em **passo dedicado** (escopo XL; ~5-8h):
**Passo 159** (renumerado de P158 em P156B).

- [ ] **ADR-0062** criada (autorização hayagriva; era ADR-0061
  antes da reocupação por Layout em P156B).
- [ ] `Cargo.toml` + `crystalline.toml` configurados.
- [ ] `Content::Bibliography {...}` + `Content::Cite {key,
  supplement, form}` variants.
- [ ] `native_bibliography` + `native_cite` em stdlib.
- [ ] Pipeline introspect com resolução cruzada.
- [ ] Render layout para ambos.
- [ ] 5-10 testes; corpus paridade ganha 2-3 ficheiros.
- [ ] Inventário 148 reclassifica ambas de `ausente` para
  `implementado⁺` (perfil graded).

### Critério de fecho

- [ ] **ADR-0062** `IMPLEMENTADO` (era ADR-0061 antes da
  renumeração em P156B).
- [ ] `bibliography` + `cite` materializados em cristalino.
- [ ] Tests verdes; lint zero.
- [ ] Inventário 148 actualizado.

### Notas

- **Não bloqueia Fase 1 do roadmap Model** (P154B = terms +
  divider; P155 = quote; **P157** = table foundations;
  **P158** = figure kinds; renumerados de P156/P157 em P156B).
  Pode ser materializado em paralelo com Fase 1 ou após.
- **Escopo cumulativo**: bibliography sem cite é
  semi-utilizável; cite sem bibliography não tem como
  resolver. Recomenda-se materializar **ambos** num único
  passo dedicado.
- **Pré-condição P154A coberta**: probe de hayagriva
  confirmou cache + versão match (**ADR-0062** não exige fetch
  online; era ADR-0061 antes da renumeração).
- **Renumeração P156B**: ADR-0061 reocupada para Layout
  roadmap em P156B; reserva hayagriva deslocada para
  ADR-0062 (sem ficheiro criado; documentado no README ADRs).
  Conteúdo material desta DEBT inalterado — apenas referências
  numéricas actualizadas.

---

## DEBT-54 — Setup vanilla `typst` workspace em `lab/parity` (pré-condição de DEBT-53) — ENCERRADO (Passo 206E) ✓

**Aberto em**: Passo 151 (2026-04-25) durante tentativa de
fechar DEBT-53.
**Fechado em**: 2026-05-08 (Passo 206E).
**Etiqueta de fecho**: **OBSOLETED** (workspace setup
nunca foi necessário).
**Justificação literal**: P206A auditoria empírica A5
descobriu vanilla typst CLI **0.14.2 pre-built**
disponível em `/usr/local/bin/typst` — paridade exacta
com `lab/typst-original/crates/typst-syntax v0.14.2`.
ADR-0075 PROPOSTO P206A C5 fixou Caminho b ("pre-built
binário"); workspace setup torna-se irrelevante.

DEBT pode fechar via 3 caminhos (pattern emergente
P206E):
- **CLOSED** — materializado.
- **REPLACED-BY** — superseded por outra abordagem.
- **OBSOLETED** — irrelevância empírica (hipótese
  inicial inválida).

DEBT-54 fecha como OBSOLETED — não há código novo;
não há solução substituta; a hipótese inicial (vanilla
precisa setup workspace cristalino) era falsa.

**Bloqueava**: DEBT-53 (vanilla integration P3 em
`lab/parity`) — desbloqueada simultaneamente.

Histórico abaixo preservado per pattern P201/P202.

### (Histórico) Estado pré-fecho — DEBT-54 — EM ABERTO (Passo 151)

### Contexto

Passo 151 foi enunciado para fechar DEBT-53 (integrar pipeline
vanilla em `lab/parity` para popular matriz P3 com números
reais). Investigação empírica em 151.1 revelou que o passo de
"setup vanilla compila" é **maior que o estimado** e
ramifica-se em sub-trabalho próprio. O spec do P151 §"O que
pode sair errado" autoriza pausar e abrir DEBT-54.

### Obstáculo concreto

**Vanilla `typst-original/` está em `lab/typst-original/`** com
o workspace **desactivado** (`Cargo.toml.original`, não
`Cargo.toml`). O workspace virtual `lab/Cargo.toml` actualmente
intercepta apenas `typst-syntax` + transitivas mínimas
(`typst-utils`, `typst-timing`).

Para fechar DEBT-53 (integrar `typst::compile<PagedDocument>`),
é necessário adicionar **toda a cadeia do compilador vanilla**
ao `lab/Cargo.toml` workspace:

1. **Crates internas vanilla** (~12, todas via path-dep para
   `lab/typst-original/crates/`):
   - `typst`, `typst-eval`, `typst-html`, `typst-layout`,
     `typst-library`, `typst-macros`, `typst-pdf`,
     `typst-realize`, `typst-render`, `typst-svg`,
     `typst-bundle`, `typst-assets` (se existir).

2. **Crates externas** (~30+ a especificar como
   `[workspace.dependencies]`):
   - `comemo`, `kurbo`, `rustybuzz`, `bumpalo`, `az`,
     `codex`, `either`, `hypher` (já há crystalline),
     `memchr`, `smallvec`, `unicode-bidi`,
     `icu_properties`, `icu_provider`,
     `icu_provider_adapters`, `icu_provider_blob`,
     `icu_segmenter`, `regex` (provável), `fontdb`,
     `hayagriva`, `serde_json` (já há), `toml` (já há),
     `oxipng` (vanilla PDF), `flate2` (já há crystalline),
     `siphasher` (já há), `palette`, …

3. **Verificação preliminar do cargo cache**
   (`~/.cargo/registry/cache/`): maioria das `icu_*`, `comemo`,
   `kurbo`, etc. estão em cache; mas algumas (`codex`,
   `hayagriva`, `oxipng`) podem estar ausentes — exigem fetch
   online.

4. **Versões**: vanilla declara `rust-version = "1.89"`,
   `edition = "2024"`. Confirmar toolchain local suporta.

5. **Conflito de versões transitivas**: cristalino usa
   `ttf-parser = "0.25"`; vanilla pode usar versão diferente.
   Cargo unifica mas pode produzir incompatibilidades subtis.

### Plano

Materializar em passo dedicado (escopo M-L; ~3-6h):

- [ ] **Inventário completo** das deps externas vanilla via
  análise estática de todos os `Cargo.toml` em
  `lab/typst-original/crates/*/Cargo.toml`.
- [ ] **Tabela de cobertura `~/.cargo/registry/cache/`**:
  identificar deps em cache vs ausentes.
- [ ] **Estratégia de network**: ou activar fetch online, ou
  vendoring local.
- [ ] **`lab/Cargo.toml` actualizado** com todos os
  workspace.dependencies necessários.
- [ ] **Smoke test**: `cargo build -p typst-layout` (vanilla)
  no `lab/` workspace sem erros.
- [ ] **DEBT-53 destrancado** para materialização
  propriamente dita após DEBT-54 fechar.

### Critério de fecho

- [ ] `cargo build -p typst-layout` (vanilla) corre sem erros
  em `lab/`.
- [ ] `lab/Cargo.toml` documenta todos os deps adicionados com
  versões finais.
- [ ] DEBT-53 destrancado (passa a ter pré-condição satisfeita).

### Notas

- **Sem trabalho em código cristalino**: DEBT-54 é integralmente
  sobre `lab/Cargo.toml` + cache de cargo + verificação de
  build vanilla.
- **Não bloqueia P153 (P2 = `value_dto.rs`)**: P2 pode ser
  materializado **antes** de DEBT-54 com mesma estratégia
  cristalino-only baseline. DEBT-53 e DEBT-54 ficam em
  paralelo até serem priorizados.
- **Passo 151 entrega** (mesmo sem fechar DEBT-53):
  materialização desta análise + abertura formal de DEBT-54
  + actualização de DEBT-53 com referência cruzada.

### Actualização Passo 152 — Refino administrativo

Adicionado em P152: **probe online dos 3 crates** identificados
em P151 §2.4 como "provavelmente ausentes"; **risco identificado**
de conflitos de versão; **critério de fecho expandido em 3
níveis**. Sem alteração de número, data de abertura ou saldo
DEBTs.

#### §3 — Probe online (resultado P152)

Verificação de `~/.cargo/registry/cache/` + comparação com
versões esperadas pelo workspace vanilla
(`lab/typst-original/Cargo.toml.original`):

| Crate | Estado em probe | Versão cached | Versão esperada vanilla | Conclusão |
|-------|-----------------|---------------|-------------------------|-----------|
| `codex` | ✓ cached | 0.2.0 | `"0.2.0"` | Match exacto |
| `hayagriva` | ✓ cached | 0.9.1 | `"0.9.1"` | Match exacto |
| `oxipng` | ✓ cached | 9.1.3 | `^9.0` (com features) | Satisfaz semver |

**Conclusão**: **todas as 3 crates resolvem do cache local**.
Estimativa P151 ("provavelmente ausentes; exigem fetch online")
**desactualizada**: probe revela cache hits completos.
DEBT-54 fica significativamente menos arriscado do que
estimado.

#### §4 — Risco: conflitos de versão entre cristalino e vanilla

Verificação empírica das 4 crates partilhadas (cristalino
`Cargo.toml` workspace.dependencies vs vanilla
`Cargo.toml.original` workspace.dependencies):

| Crate | Cristalino | Vanilla | Cargo unifica? | Risco |
|-------|-----------|---------|----------------|-------|
| `ttf-parser` | `"0.25"` | `"0.25.0"` | Sim — mesma 0.25.x | **Nenhum** |
| `comemo` | `"0.4"` | `"0.5.1"` | **Não** — major 0.4 ≠ 0.5 (semver 0.x) | **Alto** |
| `ecow` | `"0.2"` | `"0.2.6"` (com `serde`) | Sim — features adicionais; cargo unifica | **Nenhum** |
| `rustc-hash` | `"2"` | `"2.1"` | Sim — mesma 2.x | **Nenhum** |

**Surpresa material**: `comemo` está em **major 0.4** (cristalino
desde ADR-0001) e **major 0.5** (vanilla actual). Em semver
0.x, 0.4.* e 0.5.* são tratadas como **incompatíveis**. Cargo
**não unifica** — cria duas versões paralelas no grafo de deps.

**Análise técnica**:
- `typst-core` (cristalino) → `comemo 0.4`.
- `typst-library` (vanilla) + transitivas → `comemo 0.5`.
- Cargo aceita as duas no mesmo grafo (versões resolvidas
  como crates distintos a nível de hash).
- Em `lab/parity/tests/layout_parity.rs`, ambos os pipelines
  vivem lado-a-lado mas não trocam tipos `comemo::Tracked` —
  cada pipeline usa o seu `comemo` internamente.
- **Compilação não falha**, apenas duplica a crate `comemo`
  no binário de tests (~50KB; aceitável).

**Estratégia se conflito surgir noutro crate**:

1. **Verificar se cargo unifica** automaticamente (semver
   compatível). Maioria dos casos resolve aqui.
2. **Se major divergente** (como `comemo`): cargo usa duas
   versões; aceitar duplicação (custo: binário maior).
3. **Se incompatibilidade ABI** (raro): `[patch.crates-io]`
   em `lab/Cargo.toml` força versão única — pode degradar
   um dos pipelines. **Última alternativa**.
4. **Caso extremo**: feature flag por target (`#[cfg(...)]`).
   **Custo alto**; abrir DEBT dedicado.

#### §5 — Critério de fecho expandido (3 níveis)

DEBT-54 fecha quando os 3 níveis estão cumpridos:

1. **Mínimo** — `cd lab && cargo build -p typst-layout` corre
   sem erros. Garante que vanilla `typst-layout` resolve as
   suas deps via `lab/Cargo.toml` actualizado.
2. **Suficiente** — `cd lab && cargo build -p typst` (compilador
   inteiro vanilla) corre sem erros. Garante que toda a
   cadeia até `typst::compile<PagedDocument>` é resolvível.
3. **Executável** — test simples em
   `lab/parity/tests/vanilla_smoke.rs` que invoca
   `typst::compile(world)` para um source trivial (e.g.
   `Hello`) e devolve `PagedDocument` sem panic.

**Fecho parcial aceitável** (níveis 1 ou 1+2 mas não 3):
apenas se o passo de materialização identificar bloqueio
justificado e abrir **DEBT-55** para o nível pendente. Caso
contrário, fecho exige todos os 3 níveis.

#### §6 — Conclusão pós-refino

DEBT-54 mais bem caracterizado:

- **Cache local cobre as 3 crates "missing"** identificadas
  em P151 — escopo de fetch é mais reduzido que o estimado.
- **Conflito de versão em `comemo`** identificado mas
  contornável (cargo aceita duas versões em paralelo).
- **Critério de fecho** mais granular permite fecho parcial
  com plano explícito (DEBT-55 condicional).

Estimativa actualizada: **escopo M** (3-4h) em vez de **M-L
(3-6h)** que P151 indicava. Materialização (passo dedicado
posterior) tem alvo mais claro.

---

## DEBT-53 — Integração de pipeline vanilla em `lab/parity` para medição P3 — ENCERRADO (Passo 206E) ✓

**Aberto em**: Passo 150 (2026-04-25) durante materialização da
primeira matriz agregada de paridade.
**Fechado em**: 2026-05-08 (Passo 206E).
**Etiqueta de fecho**: **CLOSED** (vanilla integration
materializada via série P206A-D).
**Justificação literal**: ADR-0075 ACEITE final
2026-05-08 com 7/7 condições do plano de validação
CUMPRIDAS:

- **P206A** auditou empíricamente; descobriu vanilla
  CLI 0.14.2 pre-built (DEBT-54 colateralmente
  obsoletada).
- **P206B** reactivou harness `lab/parity/` (2 fixes
  triviais).
- **P206C** materializou helper L3
  (`03_infra/src/query_helpers.rs`) + comparação
  estrutural via `typst query` JSON +
  `vanilla_invoke.rs` + `structural_compare.rs`.
- **P206D** produziu matriz consolidada cobrindo 36
  ficheiros corpus (`lab/parity/reports/latest.md` +
  `history/2026-05-08-passo-206D.md`).
- **P206E** transitou ADRs e fechou esta DEBT.

**Cobertura empírica matriz P206D**:
- 34/36 ficheiros compilam em cristalino (94%).
- 20/36 com text_content matches em categorias INCLUDE.
- 20/36 com structural matches vs vanilla.
- 13 SKIPs justificados (3 pre-existing + 10 feature).
- 3 INCLUDE-com-diff documentados em
  `lab/parity/SKIPS.md` (não regressões; design
  intencional ou stdlib gaps pre-P206).

**Bloqueador anterior**: DEBT-54 (workspace setup) —
fechada simultaneamente em P206E como **OBSOLETED**
(per P206A D3 pattern: vanilla CLI pre-built tornou
hipótese inicial inválida).

**ADR vinculada**: ADR-0075 (vanilla integration via
pre-built CLI + comparação estrutural; ACEITE final
P206E).

**Cond 9 ADR-0073** (Saída cristalino sanity-check vs
vanilla nos 5-7 ficheiros corpus paridade) **fechada
retroactivamente** em P206E via matriz P206D — 4/6
introspection P204F com matches; 2/6 com excepções
documentadas (outline-toc TOC entries; cite-bibliography
stdlib gap pre-P206; ambas não são regressões M8).

Histórico abaixo preservado per pattern P201/P202.

**Relacionado com**: ADR-0033 (paridade funcional), ADR-0054
(perfil observacional graded), inventário 148, série paridade.

### (Histórico) Estado pré-fecho — DEBT-53 — EM ABERTO (Passo 150; bloqueado por DEBT-54 desde Passo 151)

### Contexto

Passo 150 entregou a infraestrutura de medição P3:
`lab/parity/src/frame_dto.rs` + `lab/parity/src/report.rs` +
`lab/parity/tests/layout_parity.rs` + corpus expandido
(`lab/parity/corpus/visual/` com 9 ficheiros) + primeiro
relatório `lab/parity/reports/latest.md`.

A matriz produzida nesta iteração é **cristalino-only
baseline**: 19/19 ficheiros do corpus compilam em cristalino
sem panic. Colunas `text_content`, `structural`, `geometric`
ficam `N/A` porque a comparação contra vanilla **ainda não
está integrada**.

### Diferença face ao vanilla

Para activar a comparação, é necessário:

1. **World adapter**: vanilla tem `typst_library::World` trait
   com assinatura distinta de cristalino
   `typst_core::contracts::world::World`. Implementar
   adapter ou setup duplo para cada ficheiro do corpus.
2. **Vanilla compile pipeline**: invocar
   `typst::compile::<typst_layout::PagedDocument>(world)`
   (vanilla) em paralelo a `eval_to_module_with_sink` +
   `layout` (cristalino).
3. **`FrameDTO::from_vanilla`**: actualmente stub
   (`from_vanilla_stub` devolve `FrameDTO` vazio).
   Materializar conversão real a partir de
   `typst_layout::PagedDocument` (vanilla).
4. **Setup de fonts para vanilla**: vanilla espera fonts
   embebidas via crate ou descobertas no sistema; setup
   diferente do `SystemWorld::with_fonts` cristalino.
5. **Matrix population**: actualizar
   `tests/layout_parity.rs` para chamar ambos os pipelines,
   invocar `crist_dto.compare(&vanilla_dto, t)` e popular as
   colunas que estão `N/A`.

### Razão pela escolha actual

Spec do Passo 150 §"O que pode sair errado" autoriza pausar
se vanilla integration for inviável: "Se a versão do vanilla
em `lab/typst-original/` não expõe `PagedDocument` no
namespace esperado, ajustar importação. **Se inviável,
registar e pausar — pode exigir análise da estrutura do
`lab/typst-original/`**."

A complexidade do World adapter + setup duplo de fonts
justifica passo dedicado. Passo 150 entregou infraestrutura
+ corpus + matriz baseline; integração vanilla é trabalho
posterior.

### Plano

Materializar em passo dedicado (escopo M-L). Estimativa:
~150-300 linhas de código novo:

- World adapter / setup duplo (~50-80 linhas).
- `FrameDTO::from_vanilla` (~40-60 linhas).
- `tests/layout_parity.rs` actualizado para invocar ambos
  (~30-50 linhas).
- Possível refactor do harness para tolerar diferenças de
  setup entre os dois pipelines.
- Calibração inicial das tolerâncias `geometric` (números
  brutos do baseline).

Output esperado: matriz pós-vanilla com `text_content`,
`structural`, `geometric` populados; números reais sobre
quantos ficheiros do corpus passam cada modo.

### Critério de fecho

- [ ] `from_vanilla` materializado em `frame_dto.rs`.
- [ ] Vanilla pipeline integrado em `tests/layout_parity.rs`.
- [ ] Matriz populada com números reais (substitui `N/A`).
- [ ] Relatório `latest.md` actualizado com resultados.
- [ ] `geometric` (experimental) calibrado com primeiras
  observações.
- [ ] Numeração do passo dedicado escolhida (provável 153
  directo após DEBT-54 fechar).

### Actualização Passo 151

Tentativa de fechar DEBT-53 detectou que o setup vanilla
workspace é pré-condição não satisfeita: `lab/typst-original/`
está em quarentena (`Cargo.toml.original` desactivado);
`lab/Cargo.toml` virtual workspace só intercepta
`typst-syntax` + `typst-utils` + `typst-timing`. Adicionar
`typst::compile<PagedDocument>` requer ~12 crates internas
vanilla + ~30 externas como path-deps/workspace deps.

**DEBT-54 aberto** com plano específico (ver entrada acima
nesta secção). DEBT-53 permanece **EM ABERTO** com nota de
bloqueio. Materialização efectiva sairá de passo dedicado
posterior, **após DEBT-54 fechar**.

---

## DEBT-50 — Show selector Strong/Emph não distingue origem (dívida latente) — EM ABERTO (Passo 103)

Aberto pelo Passo 103 (ADR-0041). Dívida **latente** — não activa
no estado actual do cristalino.

### Contexto

Depois do Passo 101 (`Content::Strong`/`Content::Emph` removidos,
consolidados em `Content::Styled(body, [Style::Bold/Italic(true)])`),
o show rule selector para `NodeKind::Strong` e `NodeKind::Emph` casa
qualquer `Content::Styled` com `Style::Bold(true)` ou
`Style::Italic(true)`:

```rust
// rules/eval/rules.rs:80
let is_bold_styled = matches!(node, Content::Styled(_, ss)
    if ss.iter().any(|s| matches!(s, Style::Bold(true))));
```

**Hoje**, este selector é preciso porque:

- `*bold*` → `Content::strong(body)` → `Content::Styled([Bold(true)], body)`.
- `#set text(bold: true); texto` → `StyleDelta` empilhado em `*styles`
  → `Content::Text(texto, TextStyle { bold: true, .. })` — **bake-in**,
  sem `Content::Styled` wrapping.

Portanto, `#show strong: it => [HIT]` só dispara para `*bold*`, não
para `#set text(bold: true)`. Paridade com vanilla preservada.

### Quando a dívida se manifesta

Se/quando `#set text(bold: true)` for refactorizado para produzir
`Content::Styled([Bold(true)], following_content)` em vez de
bake-in (ver discussão ADR-0040 e nota no ADR-0041), o selector
`NodeKind::Strong` começará a apanhar também `#set text`:

```typst
#show strong: it => [HIT]
#set text(bold: true)
texto
```

Neste cenário:

- **Vanilla**: "texto" em bold, sem "HIT".
- **Cristalino pós-migração**: "HIT" aparece porque selector casa
  qualquer `Content::Styled` com Bold.

Divergência de paridade (ADR-0033) aceitável no Passo 103, mas
inaceitável quando a migração for feita.

### Teste que documenta

`layout/tests.rs::tests_show_rule_integration::debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in`
— assert que HIT **não** aparece. Se o passo futuro migrar bake-in
para wrapping, este teste falha. A falha é o sinal para activar
este DEBT.

### Critério de conclusão

- [ ] Mecanismo escolhido para distinguir origem:
   1. **Flag no enum `Style`**: `Style::Bold { value: bool, from_strong: bool }`.
   2. **Marcador no `Content::Styled`**: `Content::Styled(body, styles, origin: Option<ElementKind>)`.
   3. **Selector rigoroso**: casa apenas `Styles` com exactamente
      `[Style::Bold(true)]` (sem outros estilos).
- [ ] Mecanismo implementado.
- [ ] Teste `debt_50_...` actualizado para assert paridade vanilla
      (HIT não aparece mesmo com wrapping).
- [ ] Paridade funcional com vanilla confirmada.

### Dependências

- **Não é accionável hoje** — a dívida é latente.
- Torna-se accionável quando `#set text(bold/italic: true)` deixar
  de usar bake-in.

---

## Secção 2 — DEBTs encerrados

## DEBT-1 — StyleChain — ENCERRADO (Passo 142) ✓

**Fechado em**: 2026-04-24.
**Relatório formal**:
[`relatorios/fecho-debt-1-passo-142.md`](relatorios/fecho-debt-1-passo-142.md).
**Cumprimento**: ADR-0054 (critérios 1, 2, 3 satisfeitos).

Histórico preservado abaixo na forma em que vivia antes do
fecho. Actualizações de Passos 30, 33, 83.5, 84.1, 94, 95, 99,
100, 101, 102, 103 ficam intactas; as entradas das Fases B
(137/138/139) e C (140B/141) estão registadas em DEBT-52
(imediatamente abaixo nesta secção).

---

### (Histórico) Estado pré-fecho — DEBT-1 — PARCIALMENTE RESOLVIDO (estrutura paga em Passo 100)

#### Resolvido no Passo 30

- **`StyleChain`** implementada em `entities/style_chain.rs` (L1)
- **`StyleDelta { bold, italic, size }`** como delta de herança
- **`#set text(bold:, italic:, size:)`** avaliado em `eval_expr` (Expr::SetRule)
- **`EvalContext::styles: StyleChain`** — cadeia activa durante eval
  (~~removido no Passo 94: agora propagado como `&mut StyleChain`
  parâmetro; ver secção "Actualização Passo 95" abaixo~~)
- **`TextStyle::from(&StyleChain)`** — bridge para layout/export actuais
- **`Content::Text(EcoString, TextStyle)`** — estilo capturado em eval
- **Strong/Emph/Heading** em eval: push/pop de estilos correcto

#### Divergência intencional

- ~~`#set` é global ao eval (não tem scoping por bloco) — DEBT menor~~
  **Desactualizada (Passo 95).** O scoping por bloco foi adicionado no
  Passo 33 via save/restore em `CodeBlock`/`ContentBlock`, e
  reafirmado arquitectonicamente no Passo 94 (atomização de `styles`
  como parâmetro: cada bloco cria `local_styles` próprio; isolamento
  por construção).
- Apenas `text` como target suportado — outros targets ignorados silenciosamente
- StyleChain não integrada com `#show` rules (Passo futuro)
- Layout usa merge de node_style + self.style para compatibilidade com testes directos

#### Pendente

- Propriedades adicionais (fill, font-family, weight numérico, etc.)
- Paridade total com o sistema de styles do original
- Remover os wrappers Content::Strong/Emph do layout quando eval os tiver totalmente substituído

**Ficheiros alterados**: `entities/style_chain.rs` (novo), `entities/mod.rs`,
`entities/content.rs`, `rules/eval.rs`, `rules/layout.rs`

#### Nota — actualização no Passo 84.1

Duas pendências originais ("Scoping de `#set` por bloco" e "`#show` rules")
foram riscadas por terem sido resolvidas implicitamente por outros DEBTs
(DEBT-7 e DEBT-19/20 respectivamente). A auditoria do Passo 83.5
confirmou a presença do código correspondente em `eval.rs`. As
pendências remanescentes (propriedades adicionais, paridade, wrappers)
continuam em aberto.

#### Actualização Passo 95 — revisão à luz da atomização do Passo 94

Tarefa A do Passo 95 classifica as pendências do DEBT-1:

- [x] **Scoping de `#set` por bloco** — resolvido pelo Passo 33
  (save/restore) e **reforçado pelo Passo 94** (atomização de
  `styles` como `&mut StyleChain` parâmetro; cada bloco constrói
  `local_styles` por construção — scoping torna-se intrínseco em
  vez de depender de save/restore manual sobre um campo partilhado).
- [x] **Arquitectura partilhada do `styles`** (implícita no item
  "`EvalContext::styles: StyleChain`" da lista de Passo 30) —
  resolvida no Passo 94 via extracção como parâmetro (ADR-0036
  segunda aplicação).
- [ ] **Propriedades adicionais** (fill, font-family, weight
  numérico, etc.) — **continua em aberto**. Ortogonal à
  atomização; exige extensão do `StyleDelta` e do `TextStyle`
  para novos campos.
- [ ] **Paridade total com o sistema de styles do original** —
  **continua em aberto**. Inclui o item anterior mais integração
  com show rules (DEBT-19/20), constant folding, etc.
- [ ] **Remover wrappers `Content::Strong/Emph` do layout** —
  **continua em aberto, sem mudança de natureza**. A atomização
  do eval não toca em layout; a decisão arquitectural permanece
  igual ao que era antes do Passo 94.

Sumário: 2 pendências implicitamente resolvidas pela atomização
(já não exigem trabalho específico); 3 pendências permanecem
legítimas. DEBT-1 permanece na Secção 1 — o trabalho residual
(propriedades, paridade, wrappers) não foi resolvido por
atomização.

#### Actualização Passo 99 — fundação tipada `Style`/`Styles`/`Content::Styled`

Passo 99 (ADR-0038) materializou a fundação em L1:

- [x] **Enum `Style`** em `entities/style.rs` com 5 variantes:
  `Bold`, `Italic`, `Size`, `Fill`, `HeadingLevel`. Superconjunto
  preparado para futuro (`Fill` e `HeadingLevel` forward-compat).
- [x] **Struct `Styles(Vec<Style>)`** — colecção de deltas tipados.
- [x] **`StyleChain::push_styles(&Styles)`** — entrada tipada; projecta
  as variantes conhecidas no `StyleDelta` interno.
- [x] **`StyleChain::fill()`/`heading_level()`** — accessors forward-compat
  para as variantes novas.
- [x] **`Content::Styled(Box<Content>, Styles)`** — nova variante
  cobrindo `plain_text`, `is_empty`, `map_text`, `map_content`,
  `PartialEq` e todos os sítios de `match` exaustivo (incluindo
  `introspect::materialize_time`, `introspect::walk`,
  `layout_content` — que é transparente no Passo 99).
- [x] **Teste de integração conceptual**: `Content::Styled` → `Styles`
  → `StyleChain::push_styles` → resolução top-wins (paridade vanilla).
- [x] **ADR-0038 `EM VIGOR`** — "Sistema de estilos em L1".
- [x] **Decisão COEX** registada: `TextStyle` plano permanece como
  "vista achatada para o Layouter actual" — 70+ sítios de consumo
  tornam SUB impraticável num único passo. Novo DEBT-48 aberto.

**Propriedades adicionais**: parcialmente pagas. `Fill` e
`HeadingLevel` no enum; outras (`font`, `lang`, `leading`) continuam
adiadas (bloqueadas por tipos não materializados — ver ADR-0038).

**Estado**: `DEBT-1` marcado agora como **PARCIALMENTE RESOLVIDO
(Passo 99)** — a fundação está materializada. Activação no eval
(`#set`/`#show` a consumir `Content::Styled`) e substituição de
`TextStyle` (DEBT-48) são trabalho futuro.

#### Actualização Passo 101 — remover wrappers `Content::Strong`/`Emph`

- [x] `Content::Strong(Box<Content>)` e `Content::Emph(Box<Content>)`
      **removidos do enum** `Content`.
- [x] `Content::strong(body)` e `Content::emph(body)` redefinidos como
      construtores que emitem `Content::Styled(body,
      Styles::from_iter([Style::Bold(true)]))` (ou Italic). API pública
      preservada; zero ripple para consumidores.
- [x] Layouter, `introspect::materialize_time`/`walk`, `PartialEq`,
      `map_content`, `map_text`: arms dedicados removidos. Comportamento
      coberto pelo arm `Content::Styled` (adicionado no Passo 100).
- [x] `eval/rules.rs` show selector: `show strong: it => ...` passa a
      casar `Content::Styled` que contenha `Style::Bold(true)`;
      análogo para `show emph`. Paridade funcional preservada.
- [x] stdlib `native_strong`/`native_emph` emitem
      `Content::strong(body)`/`Content::emph(body)` (que por sua vez
      emitem `Content::Styled`).
- [x] Tests `entities/content.rs` actualizados: `matches!(c,
      Content::Strong(_))` → `Content::Styled(_, _)`; construção via
      factory preservada.
- [x] `cargo test --workspace`: 783 → **783 L1** (inalterado;
      consolidação puramente estrutural).
- [x] `crystalline-lint`: zero violations.
- [x] Paridade funcional: `Hello *bold* and _italic_` produz output
      idêntico ao pós-Passo 100.

**DEBT-1 pendências restantes**: 2 de 3 (activação de `#set`/`#show`
no eval; propriedades adicionais bloqueadas). A tarefa "remover
wrappers Strong/Emph do layout" foi paga.

#### Actualização Passo 102 — `#set text(...)` validado + `fill` activado

O Passo 102 (ADR-0040) formalizou a activação de `#set` no eval:

- [x] `#set text(bold/italic/size)` já activo desde Passo 30 via
      arquitectura bake-in (`StyleDelta` empilhado em `*styles`;
      `TextStyle::from(&*styles)` capturado em cada `Content::Text`).
      Validado end-to-end por 6 novos testes em
      `tests_set_rule_integration`:
      `set_text_size_propaga_ao_frame`,
      `set_text_bold_propaga_ao_frame`,
      `set_text_italic_propaga_ao_frame`,
      `set_text_bold_afecta_conteudo_seguinte_nao_anterior`,
      `set_combinado_com_emph_sintactico`,
      `bold_syntax_sem_set_continua_a_funcionar`.
- [x] **`#set text(fill: color)` activado no Passo 102** — `StyleDelta.fill`
      capturado a partir de `Value::Color`; propaga a
      `TextStyle.fill` (adicionado no Passo 100) via
      `TextStyle::from(&StyleChain)`. Teste unitário
      `eval_set_text_fill_passo_102` confirma parse + eval sem erro.
- [x] ADR-0040 `EM VIGOR`.
- [x] `cargo test --workspace`: 783 → **790 L1** (+7; +6 integração,
      +1 unitário fill).
- [x] `crystalline-lint`: zero violations.

**Decisão arquitectural registada na ADR-0040**: manter bake-in como
arquitectura principal para `#set text`. Refactorização para
wrapping via `Content::Styled` adiada para quando `Introspection`
materializar e `Content::Heading` colapsar. Dívida estrutural, não
funcional.

**DEBT-1 pendências restantes**: 2 de 3 permanecem.

1. `#set` está activo. Falta **`#show`** activar. Dívida latente
   do show rule selector (relatório Passo 101) fica para passo
   dedicado — espera-se que a activação de `#show` exponha essa
   dívida e obrigue a refactorização do selector.
2. **Propriedades adicionais** (`text.font`, `text.lang`,
   `par.leading`, `text.weight` como string) — bloqueadas por
   tipos não materializados. Adiado para passos quando Font/Lang/
   Par entrarem em L1.

#### Actualização Passo 103 — `#show heading/strong/emph` validado

Inventário 103.A revelou que `#show` **já estava activo desde o Passo 70**
(DEBT-23 encerrado) para selectores `Text` e `NodeKind` (heading,
strong, emph, raw, figure, equation, list). O Passo 101 actualizou
o match de Strong/Emph para usar `Content::Styled` + `Style::Bold/Italic(true)`.

- [x] Validação end-to-end com 5 testes de integração em
      `rules/layout/tests.rs::tests_show_rule_integration`:
      `show_heading_transforma_em_uppercase`, `show_strong_transforma`,
      `show_emph_transforma`, `regressao_sem_show_mantem_comportamento`,
      `debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in`.
- [x] ADR-0041 `EM VIGOR` documentando catálogo, limites e dívida.
- [x] DEBT-50 aberto para dívida latente (quando `#set text` migrar
      de bake-in para wrapping, o selector Strong apanha false
      positives).
- [x] `cargo test --workspace`: 790 → **795 L1** (+5).
- [x] `crystalline-lint`: zero violations.

**DEBT-1 pendências restantes após Passo 103**: 1 de 3.

1. ~~Activar `#show`~~ — concluído (heading, strong, emph). Selectores
   remanescentes (where, catch-all, regex, label) fora do escopo do
   DEBT-1 — ficam como extensão futura, não como dívida estrutural.
2. **Propriedades adicionais** (`text.font`, `text.lang`,
   `par.leading`, `text.weight` como string) — bloqueadas por
   tipos não materializados.

#### Actualização Passo 100 — activação de `Content::Styled` no Layouter

DEBT-48 encerrado no Passo 100 (ADR-0039):

- [x] `Layouter` ganhou `chain: StyleChain` como source-of-truth do
      estilo activo; `self.style: TextStyle` passa a ser cache da
      vista resolvida via `TextStyle::from(&self.chain)`.
- [x] `Content::Styled` activo: `push_styles`/restore sobre
      `self.chain`, sincroniza `self.style`. Deixou de ser
      transparente (Passo 99 era a versão COEX inactiva).
- [x] `TextStyle` estendido com `fill` + `heading_level` (alinha com
      enum `Style` do Passo 99).
- [x] `Content::Text` arm: merge correcto entre `node_style` (do
      eval) e `self.style` (da cadeia) — propriedades activas na
      cadeia sobrepõem o node_style; propriedades passivas herdam.
- [x] 3 testes de integração em `layout/tests/tests_styled_integration`
      confirmam Bold+Size aplicados, aninhamento top-wins, e
      não-vazamento após save/restore.
- [x] ADR-0039 promovida a `EM VIGOR`.

**Dívida estrutural do `StyleChain` / `TextStyle` paga**. DEBT-1
permanece como **PARCIALMENTE RESOLVIDO** porque ainda faltam:

- [ ] Activar `#set`/`#show` no `eval_markup` a produzir
      `Content::Styled`. Este passo é **independente** do Passo 100 —
      o pipeline Layouter→export já aceita o contrato.
- [ ] Remover wrappers `Content::Strong`/`Emph` do layout quando
      `eval` os substituir por `Content::Styled([Style::Bold(true)])`.
- [ ] Propriedades adicionais (`text.font`, `text.lang`, `par.leading`)
      — bloqueadas por tipos não materializados (ADR-0038).

---

### Fecho — Passo 142

- [x] Critérios de ADR-0054 cumpridos (transcritos no
      relatório §2).
- [x] 9 dos 10 campos de `StyleDelta` com consumer activo
      (relatório §3).
- [x] `lang` em scope-out justificado pelo perfil
      observacional graded de ADR-0054 (relatório §4).
- [x] DEBT-52 (rastreador) encerrado simultaneamente.
- [x] Limitações conhecidas preservadas como candidatos
      futuros (não DEBTs): variant-aware (ADR-0055bis),
      multi-font per document (Passo 142A), subsetting
      (ADR-0056), shaping rustybuzz (DEBT-53), fixture de
      fonts em CI.

---

## DEBT-52 — Consumer integral de `StyleDelta` em layout — ENCERRADO (Passo 142) ✓

**Fechado em**: 2026-04-24 (simultaneamente com DEBT-1).
**Relatório formal**:
[`relatorios/fecho-debt-1-passo-142.md`](relatorios/fecho-debt-1-passo-142.md).
**Gaps fechados**: 6/8. Os gaps 7 (lang hyphenation) e 8
(font dict) são opcionais segundo ADR-0054 e permanecem como
candidatos futuros, sem reabertura como DEBTs novos.

Rastreador cumpriu a sua função: guiou as Fases A (Passo 136),
B (Passos 137–139) e C básica (Passos 140B + 141) do trabalho
exigido por ADR-0054 para fecho de DEBT-1. Histórico preservado
abaixo.

### Actualização Passo 146 — Multi-font per document (decisão 5)

- [x] Decisão 5 de ADR-0055 (multi-font per document)
  materializada pós-fecho de DEBT-1. Pipeline acrescenta
  `collect_fonts_from_doc` + `resolve_fonts` em
  `03_infra/src/pipeline.rs`; `export_pdf_multifont` +
  `build_multifont` + `build_page_stream_multifont` em
  `03_infra/src/export.rs`. Documento com N fonts distintas
  produz PDF com N `/Subtype /Type0` (uma por família).
  Single-font preservado por dispatch (`[(_, b)] =>
  export_pdf_with_font(...)` mantém o caminho 140B/141
  com nome canónico `/CrystallineFont`). **ADR-0055 anotada**
  (modelo ADR-0019 + 140A); status permanece
  `IMPLEMENTADO`; sem revisão. **DEBT-52 não reabre**:
  consistente com perfil observacional graded de ADR-0054 e
  com o padrão estabelecido no Passo 144 (gap 7). Contagem
  de DEBTs abertos: **inalterada (10)**.

### Actualização Passo 144 — Consumer `lang` hyphenation (gap 7)

- [x] Gap 7 (lang hyphenation) materializado pós-fecho de
  DEBT-1. **ADR-0057** autoriza crate `hypher` em L1
  (`[l1_allowed_external]`) — pure-data, no_std, zero deps,
  padrões TeX embebidos em compile-time. Helper puro
  `01_core/src/rules/layout/hyphenation.rs::hyphenate(word, &lang)`
  invocado pelo `layout_word` quando palavra não cabe e
  `style.lang` é `Some(lang)`. Algoritmo greedy: maior prefixo
  com hífen literal `-` que cabe no espaço disponível vence;
  resto recursa. Política silent skip para idiomas não
  suportados (ISO 3-letras; código fora do hyph-utf8) e
  documentos sem `lang`. **Não reabre** DEBT-1 nem DEBT-52:
  ADR-0054 declarou gap 7 opcional; este passo reduz superfície
  de scope-out por priorização tipográfica, sem contradizer o
  perfil observacional graded. `lang` muda de scope-out total
  para **parcialmente consumido** — hyphenation activo;
  shaping features (rustybuzz) continuam ausentes (DEBT-53
  candidato XL futuro). Contagem de DEBTs abertos: **inalterada
  (10)**.

---

### (Histórico) Estado pré-fecho — DEBT-52 — EM ABERTO (Passo 135)

**Aberto em**: Passo 135 (2026-04-24).
**Relacionado com**: DEBT-1 (fecho depende deste), ADR-0033
(paridade, reinterpretada por ADR-0054), ADR-0053 (`font` dict
deferido).

#### Contexto

Passos 126–134 capturaram a lista canónica DEBT-1 em
`StyleDelta`: `weight`, `tracking`, `leading`, `lang`, `font`.
5 destes 5 campos são **inertes** — layout actual usa
`TextStyle` plano (5 campos: bold, italic, size, fill,
heading_level) que não os cobre. `#set text(weight: 700)` é
capturado mas o PDF é idêntico a sem o `#set`.

ADR-0033 (paridade funcional) lido literal exige output
observacional equivalente ao vanilla. ADR-0054 (criada em
Passo 135) formaliza que **DEBT-1 não fecha enquanto consumers
não existirem**.

#### Gaps identificados (diagnóstico 135)

Ver
`00_nucleo/diagnosticos/diagnostico-shaping-passo-135.md`
secção 3.1 para tabela detalhada.

Resumo por dificuldade:
- **XS (2)**: estender `TextStyle` (fase A).
- **S (3)**: tracking, leading, weight-faux-bold (fase B).
- **M (4-5)**: font string, font array, hyphenation,
  embedding PDF fonts (fase C).
- **L (1)**: lang shaping features via rustybuzz.
- **XL (1)**: shaping engine completo (rustybuzz integrado).

#### Âmbito

- [x] **Fase A**: estender `TextStyle` + `From<&StyleChain>`.
      **Resolvido no Passo 136** (5 campos + 5 resolvers em
      StyleChain + 5 testes de propagação; `TextStyle` deixou
      de ser `Copy` — `.clone()` nos call sites).
- [x] Consumer `tracking`. **Resolvido no Passo 137** —
      `word_width` acresce `(n-1) × tracking_pt`; export emite
      PDF `Tc` operator. **Primeiro efeito visível** desde
      Passo 102 (fill).
- [x] Consumer `leading`. **Resolvido no Passo 138** —
      `flush_line` soma `leading_pt` ao `line_height` default.
      Semântica "opt soma" (divergência subtil vanilla
      documentada). Exporter inalterado — frame carrega y
      correcto.
- [x] Consumer `weight` faux-bold. **Resolvido no Passo 139** —
      `TextStyle::faux_bold_stroke_pt` + exporter emit `2 Tr`
      + `{stroke} w` wrapped em `q/Q`. K=0.04 calibração
      inicial. Aproximação visual até font embedding real.
      **Fase B completa**.
- [x] Consumer `font` string (nome via `FontBook::select`).
      **Resolvido no Passo 140B** — `compile_to_pdf_bytes`
      passou a despachar para `export_pdf_with_font` quando
      `first_font_from_doc(&doc)` + `resolve_font(...)` produzem
      bytes (`FontBook::select` com `FontVariant::default()`).
      MVP single-font per document: a primeira família encontrada
      no `PagedDocument` vence; spans subsequentes com font
      diferente são silenciosamente ignorados (ADR-0055 decisão 3).
      Selecção variant-aware (font-file "Bold"/"Italic" dedicado)
      é limitação aceite — faux-bold do Passo 139 continua a ser
      o caminho para `weight: 700`. **Início da Fase C.**
- [x] Consumer `font` array (fallback chain).
      **Resolvido no Passo 141** — `resolve_font` itera
      `font_list.as_slice()`; primeira família que `FontBook::select`
      resolve **e** `world.font(index)` devolve `Some` vence.
      Semântica vanilla directa para `#set text(font: ("A", "B",
      "C"))`. Cenário patológico (índice stale) continua a tentar
      famílias seguintes — não curto-circuita. ADR-0055 transita
      a `IMPLEMENTADO` (par 140B+141 completa a paridade básica).
      **Fase C básica completa.**
- [ ] Consumer `lang` hyphenation (requer crate).
- [ ] **Fase D opcional**: ADR-0054bis autorizar `regex` +
      `Covers` concreto para font dict.
- [ ] **Fase E opcional**: rustybuzz integration para shaping
      features + script-aware (escopo XL; possivelmente série
      dedicada fora deste DEBT).

#### Dependências

- **`FontBook::select`** em L1 — já existe (`font_book.rs`).
- **PDF font embedding em L3** — hoje Helvetica hardcoded
  (F1/F2/F3). Requer infra (font loader + subsetting).
- **Crate `regex`** — não autorizada em L1. Bloqueia dict
  form de font (ADR-0054bis futura).
- **Crate hifenização** (ex: `hyphenation`) — bloqueia lang
  consumer.

#### Roadmap estimado

**4-8 passos** para paridade observável razoável
(fase A + B + C). **Fase D/E** se escopo ampliar.

Ponto de entrada: Passo 136 (fase A, XS).

#### Critério de conclusão

- [ ] Cada campo inerte identificado tem consumer activo OU
      é explicitamente marcado como "scope-out" com ADR de
      suporte.
- [ ] Output PDF observacionalmente equivalente ao vanilla
      para inputs de teste documentados.
- [ ] DEBT-1 pode fechar.

#### Nota estratégica

DEBT-52 é **rastreador**, não trabalho. Fecha quando todos os
gaps forem atacados ou explicitamente scope-out. Cada linha
`- [ ]` acima corresponde a um ou mais passos futuros.

---

## DEBT-45 — Métodos `check_*_depth` de `Route<'a>` não chamados pelo eval — ENCERRADO (Passo 110) ✓

Aberto no Passo 91. Parcialmente pago no Passo 93 (2/4 integradas).
**Encerrado no Passo 110** com decisão "não aplicável" para as 2
pendentes.

### Estado final

| Check | Limite | Estado | Razão |
|-------|-------:|--------|-------|
| `check_call_depth` | 80 | ✓ **Integrada** (Passo 93) | `closures.rs:98` em `apply_closure`. |
| `check_show_depth` | 64 | ✓ **Integrada** (Passo 93) | `rules.rs:66` em `apply_show_rules`. |
| `check_layout_depth` | 72 | ⊘ **Não aplicável** (Passo 110) | Layouter cristalino opera sobre `Content` já avaliado; não recebe `Route` nem `Engine` (divergência arquitectural ADR-0026 / ADR-0033). Integrar implicaria propagar Route por 10+ funções de layout — refactor fora do escopo de DEBT-45. |
| `check_html_depth` | 72 | ⊘ **Não aplicável** (Passo 110) | Cristalino não tem pipeline HTML (grep `Html|html` em `01_core/src/` só bate na própria função `check_html_depth`). |

### Encerramento (Passo 110)

O Passo 110 fez inventário em
`00_nucleo/diagnosticos/inventario-debt45-passo-110.md` e concluiu
que as 2 pendentes são **não aplicáveis** na arquitectura actual:

- **Layouter sem Route**: grep `Route|Tracked|engine|Engine` em
  `01_core/src/rules/layout/` dá zero matches. `pub fn layout(content:
  &Content, initial_state: CounterState) -> PagedDocument` —
  assinatura sem `Route`. Integrar `check_layout_depth` exigiria
  refactor equivalente aos Passos 92/109 no Layouter, o que excede
  o âmbito estrito do Passo 110 ("se propagação > 2 funções, parar
  e reportar").
- **Sem pipeline HTML**: não há qualquer código HTML em L1/L2/L3
  além da própria função `check_html_depth`.

### Critério de conclusão (final)

- [x] `check_call_depth` integrada (Passo 93).
- [x] `check_show_depth` integrada (Passo 93).
- [x] `check_layout_depth` documentada como não aplicável (Passo 110).
- [x] `check_html_depth` documentada como não aplicável (Passo 110).
- [x] `EvalContext::check_call_depth` antigo + `enter_call` +
      campos `depth`/`max_call_depth` removidos (Passo 93).
- [x] Testes que exercitam cada limite integrada passam
      (`cargo test --workspace`: 803 L1 + 184 L3 + 6 ignorados
      estáveis desde Passo 93).

### Forma

**Opção A do Passo 110**: funções livres, sem refactor.
As 2 aplicáveis já integradas (Passo 93); as 2 não aplicáveis
documentadas. Passo 110 é puramente documental — **zero código
de produção alterado**.

### Trabalho futuro (não bloqueia DEBT-45)

- Quando o Layouter for refactored para usar `Engine<'a>`/`Route` —
  `check_layout_depth` ganha call site natural. Passo dedicado.
- Quando um pipeline HTML for materializado — `check_html_depth`
  ganha call site natural. Passo dedicado.

As 2 funções permanecem definidas em `world_types.rs` para paridade
estrutural com o vanilla (ADR-0033) e estão testadas em
`world_types.rs::tests`.

### Padrão comemo herdado do Passo 93

Por motivos de arquitectura do `comemo 0.4.0`, os 4 `check_*_depth`
são **funções livres** `world_types::check_*(Tracked<Route>)` em
vez de métodos tracked. Razão: `Tracked<T>` só expõe métodos do
bloco `#[comemo::track]` e as verificações precisam de construir
`SourceDiagnostic` (tipo não-memoizável). Decisão preservada no
Passo 110.

### Diagnóstico relacionado

`00_nucleo/diagnosticos/diagnostico-constraint-tracked-recursivo-passo-93.md`
documenta o padrão `<T<'static> as Validate>::Constraint` descoberto
no Passo 92 — base para estas funções livres funcionarem com
`Tracked<Route>`.

---

## DEBT-49 — Propriedades de `#set` não suportadas silenciadas — ENCERRADO (Passo 107) ✓

Aberto pelo Passo 102 (ADR-0040). **Encerrado no Passo 107** (5ª
aplicação da ADR-0036; canal Sink consumido via `TrackedMut`).

### Contexto original

`eval_set_rule` em `01_core/src/rules/eval/rules.rs` processava
`#set text(...)` com um catálogo fechado (`bold`, `italic`, `size`,
`fill` desde 102). Qualquer outra propriedade caía num `_ => { }`
silencioso. O target `par`, `align`, etc. também era ignorado.

### Encerramento (Passo 107)

Decisão: propagar `sink: &mut TrackedMut<'_, Sink>` (gate 107.A.3
disparou: `&mut Sink` não é obtível de `TrackedMut<Sink>` sem perder
tracking comemo; corrigida a forma de `&mut Sink` para
`&mut TrackedMut<'_, Sink>`). `warn_note` estendido para aceitar
hint (Passo 107) mantendo compatibilidade com ADR-0043.

Mudanças aplicadas:

- [x] `Sink` materializado (Passo 104, ADR-0042).
- [x] **5ª aplicação ADR-0036**: `&mut TrackedMut<'_, Sink>` adicionado
      como 10º parâmetro às funções `eval_*` (K+P=24 funções, D=4
      níveis; dentro do gate ≤40/≤6).
- [x] **Site A**: target desconhecido em `#set` (`par`, `align`, …)
      emite warning via `unsupported_target_warn` + `sink.warn_note`
      antes de retornar `Ok(Value::None)`.
- [x] **Site B**: propriedade desconhecida em `#set text` emite warning
      via `unsupported_property_warn` + `sink.warn_note` (referência
      ADR-0040 no hint).
- [x] `warn_note` estendido com `hint: &str` (convenção `""` = sem
      hint) para preservar hints através do canal tracked.
- [x] 6 novos testes L3 (`debt49_*`): font/lang/multiple/regressão
      com propriedades suportadas/target desconhecido/spans distintos.
- [x] `cargo test --workspace`: L1 **803** (inalterado), L3 **184**
      (+6). Zero regressões.
- [x] `crystalline-lint`: zero violations.

### Formato das mensagens (audit trail)

Helper partilhado em `rules/eval/rules.rs`:

```rust
fn unsupported_property_warn(target, field) -> (msg, hint) {
    msg  = "{target}: propriedade '{field}' ainda não suportada"
    hint = "ver ADR-0040 para propriedades cobertas por set {target}"
}

fn unsupported_target_warn(target) -> (msg, hint) {
    msg  = "set: target '{target}' ainda não suportado"
    hint = "targets suportados: heading, page, figure, text"
}
```

### Lacunas residuais (trabalho futuro, não bloqueiam DEBT-49)

- **`text.weight` como string/int**: o vanilla aceita
  `#set text(weight: "bold" | "regular" | 100 | 200 | ...)`.
  O cristalino hoje só aceita `bold: Value::Bool`. Mapeamento
  string→bool ou int→bool é trabalho separado. Hoje emite o warning
  "propriedade 'weight' ainda não suportada" — correcto para o
  utilizador.
- **Silenciamentos fora do âmbito estrito**: DEBT-10 (argumentos
  extras de `#set heading`), wildcards deliberados em `eval_markup` /
  `eval_expr` (`_ => Ok(Value::None)`), defensivos (`bindings.rs:114`)
  continuam silenciosos por design. Não são DEBT-49.
- **Parâmetros em `eval_*`**: 10 params + ctx é visualmente pesado.
  Registado como evidência empírica para `Engine<'a>` futuro
  (trabalho de passo dedicado quando `Introspection` materializar).

---



## DEBT-51 — Warnings do `Sink` não chegam ao caller L3/CLI — ENCERRADO (Passo 106) ✓

Aberto pelo Passo 104 (ADR-0042). **Encerrado no Passo 106** (ADR-0043).

### Contexto

Passo 104 materializou `Sink` em L1 mas os warnings acumulavam sem
caminho para o caller. `TrackedMut<Sink>` em `eval()` nunca era
lido.

### Encerramento (Passo 106)

Decisão: **Opção 2 — TrackedMut caller-managed** (assinatura de
`eval()` preservada; caller constrói `Sink`, passa
`sink.track_mut()`, e lê `sink.into_diagnostics()` após retorno).

Mudanças aplicadas:

- [x] Canal decidido (opção 2) e registado em ADR-0043.
- [x] Método tracked `Sink::warn_note(span, &str)` adicionado ao
      bloco `#[comemo::track] impl Sink` para permitir emissão via
      `TrackedMut<Sink>` sem propagação interna.
- [x] `Sink` ganhou `#[derive(Clone)]` (requisito de comemo para
      tracked mutations).
- [x] `eval()` emite micro-piloto: `sink.warn_note(...,
      "ficheiro vazio: sem conteúdo")` quando `source.text().is_empty()`.
- [x] `03_infra/integration_tests.rs` ganhou `do_eval_with_sink` que
      drena `sink.into_diagnostics()` após retorno, e
      `drain_warnings_to_stderr` com formato mínimo
      `"warning: <Span-debug> <message>"`.
- [x] 4 testes L3 integrados:
      `sink_canal_emite_warning_para_ficheiro_vazio`,
      `sink_canal_vazio_quando_sem_trigger`,
      `sink_canal_formato_minimo`,
      `sink_canal_cada_run_tem_proprio_sink`.
- [x] `cargo test --workspace`: L1 inalterado (803), L3 **178** (+4).
- [x] `crystalline-lint`: zero violations.
- [x] ADR-0043 `EM VIGOR`.

### Lacunas residuais (trabalho futuro, não bloqueiam DEBT-51)

- **Formato rico**: `Span` imprime-se como `Span(N)` opaco. Resolver
  para linha/coluna via `Source` é trabalho de passo dedicado.
- **CLI real**: `04_wiring/src/main.rs` continua stub. Quando uma
  CLI com argumentos for materializada, o padrão `do_eval_with_sink`
  + `drain_warnings_to_stderr` serve de referência directa.
- **DEBT-49 valor prático**: agora que o canal existe, migrar os
  sítios silenciados em `eval_set_rule` traz benefício imediato.
  DEBT-49 continua aberto mas desbloqueado.

---

## DEBT-48 — Substituir `TextStyle` plano por `StyleChain` no Layouter e export — ENCERRADO (Passo 100) ✓

Aberto pelo Passo 99 como consequência directa da decisão **COEX**
registada em ADR-0038.

### Contexto

Passo 99 (ADR-0038) materializou a fundação `Style`/`Styles`/`StyleChain`
e a variante `Content::Styled`. O inventário 99.A contou **70 sítios**
de `TextStyle` em `01_core/` + `03_infra/`:

- ~15 sítios de construção (`TextStyle { ... }` ou `TextStyle::regular/bold/italic`).
- ~55 sítios de consumo (`.bold`, `.italic`, `.size`).
- ~10 testes que dependem da estrutura exacta.

Critério da spec (≥15 consumo → COEX). 55 ≫ 15, logo a substituição
completa fica fora do Passo 99.

A decisão COEX mantém `TextStyle` em `entities/layout_types.rs` como
"vista achatada para o Layouter actual", com `From<&StyleChain>` como
ponte. `Content::Styled` usa `Styles`, não `TextStyle`.

### Escopo

Substituir `TextStyle` por `StyleChain` em:

1. `FrameItem::Text { style: TextStyle, .. }` em `layout_types.rs` —
   passar a `StyleChain<'static>` ou equivalente que não bloqueie
   `FrameItem` em lifetimes. Decisão de tipo fica para este DEBT
   resolver; pode exigir uma forma `OwnedStyleChain` sem referências
   ou um wrapper `Arc<StyleNode>`.
2. Todos os consumidores de `FrameItem::Text.style` em
   `03_infra/src/export.rs` e testes de frame.
3. `TextStyle::regular/bold/italic` constructors → substituição por
   `StyleChain::default_chain().push_styles(...)` em testes.
4. Todos os sítios do Layouter que hoje fazem `let prev = self.style;`
   → substituir por `StyleChain` push/pop ou por propagação via
   parâmetro (análogo à Opção do Passo 94).

### Critério de conclusão

- [ ] `TextStyle` removido de `entities/layout_types.rs` ou marcado
      como "interno à ponte" se `FrameItem::Text` ainda precisa de
      uma vista achatada para export.
- [ ] `FrameItem::Text` usa `StyleChain` (ou equivalente owned) como
      tipo do campo `style`.
- [ ] Zero `TextStyle { bold:, italic:, size: }` literals em `01_core/src/`.
- [ ] `cargo test --workspace` passa.
- [ ] `crystalline-lint` zero violations.

### Dependências

- **Não é atacado** antes do Passo 99.D/E consolidar. Este DEBT
  pressupõe a fundação do Passo 99 materializada e estável.
- Pode coexistir com activação de `#set`/`#show` no eval
  (independente).
- Pode co-beneficiar de materialização de `Engine<'a>` se a forma
  owned de `StyleChain` naturalmente viver nesse agregador.

### Nota sobre `TextStyle::size` em `FrameItem::Text`

A optimização de rendering do export PDF depende hoje de ler
directamente `style.size` para o "Tf" do PDF. Qualquer substituição
por `StyleChain` tem de manter a semântica de "obter tamanho resolvido
em ponto único" — ou via método `StyleChain::size()`, ou via
pré-resolução antes do push para o frame. A segunda é preferível
porque evita leitura recursiva durante render.

### Encerramento (Passo 100)

Decisão tomada: **SR (Struct Resolvido)**, registada em ADR-0039.
`TextStyle` preservado como nome (reduz ripple em tests legacy) mas
redefinido como "vista achatada de uma `StyleChain`" — o struct agora
inclui `fill: Option<Color>` e `heading_level: Option<u8>` para
alinhar com o enum `Style` do ADR-0038.

Mudanças aplicadas:

- [x] `TextStyle` estendido com `fill` + `heading_level` (Default
      automático via `#[derive(Default)]`).
- [x] `Layouter` ganhou campo `chain: StyleChain` como source-of-truth,
      com `style: TextStyle` como cache da vista resolvida.
- [x] `From<&StyleChain> for TextStyle` preenche todos os 5 campos —
      ponto único de resolução.
- [x] `Content::Styled` activo no Layouter: `push_styles`/restore
      sobre `self.chain`, sincroniza `self.style` via `TextStyle::from`.
- [x] `Content::Text` arm merge: propriedades activas na chain
      (`bold/italic==true`, ou `size > base`) sobrepõem o `node_style`
      do eval; caso contrário o node_style prevalece.
- [x] Construtores `TextStyle { ... }` literais actualizados com
      `..TextStyle::default()` — transformação Regex mecânica.
- [x] `FrameItem::Text.style: TextStyle` continua a ser consumido por
      `export.rs` sem alterações.
- [x] 3 testes novos em `layout/tests/tests_styled_integration`:
      aplicação de Bold+Size, aninhamento top-wins, não-vazamento
      após save/restore.
- [x] `cargo test --workspace`: 780 → **783 L1** (+3), 174 L3 + 6
      ignorados inalterados.
- [x] `crystalline-lint`: zero violations.
- [x] ADR-0039 promovida a `EM VIGOR`.

**Dívida residual**: nenhuma estrutural relativa a `TextStyle`. A
activação de `#set`/`#show` no `eval_markup` para produzir
`Content::Styled` é trabalho futuro (separado — o pipeline
Layouter→export já aceita o enum).

---

## DEBT-46 — Ficheiros de L1 com coesão baixa por tamanho excessivo — ENCERRADO (Passo 96.10) ✓

Seis ficheiros em `01_core/src/` excedem 1000 linhas e misturam
responsabilidades de domínios distintos. A análise realizada antes
do Passo 96 revelou:

| Linhas | Ficheiro |
|--------|----------|
| 3780 | `01_core/src/rules/eval.rs` |
| 2848 | `01_core/src/rules/layout/mod.rs` |
| 2255 | `01_core/src/rules/parse.rs` |
| 1806 | `01_core/src/rules/math/layout.rs` |
| 1711 | `01_core/src/rules/stdlib.rs` |
| 1250 | `01_core/src/rules/lexer/mod.rs` |

Total: 13.650 linhas em seis ficheiros. O `eval.rs` sozinho tem
368 ocorrências de padrões `match` sobre `Expr::`, `SyntaxKind::`
e `Value::` — é um dispatcher central para toda a lógica de
avaliação, misturando markup, matemática, closures, imports,
regras show/set e controlo de fluxo.

Este DEBT documenta o inventário concreto da reestruturação
necessária pela ADR-0037 (`EM VIGOR` desde Passo 96.3 após
validação empírica nos Passos 96.1–96.2).

### Motivação

A ADR-0036 (atomização progressiva) reduziu acoplamento dentro
de funções (Passos 92–95) mas não reduziu tamanho dos ficheiros.
O `eval.rs` diminuiu parcialmente após as extracções de `route`,
`styles`, `show_rules`, `active_guards`, mas continua acima de
3700 linhas.

A ADR-0037 complementa a ADR-0036 ao orientar decomposição
**entre ficheiros**, não só dentro de funções.

### Critério de conclusão

Renumeração aplicada no Passo 96.3 após introdução do 96.2
(delegação dos armos) que deslocou os sub-passos seguintes em
uma posição:

- [x] `eval.rs` reestruturado em submódulos por domínio (math,
      operators, control_flow, closures, bindings, rules,
      markup, modules, tests). **Passo 96.1 concluído** — 7
      submódulos criados inicialmente (math/operators/
      control_flow/closures/bindings/rules), mais tests.rs em
      ficheiro separado. `markup.rs` e `modules.rs` adicionados
      no Passo 96.2.
- [x] Delegação completa dos armos longos do dispatcher
      `eval_expr`. **Passo 96.2 concluído** — `mod.rs` caiu
      para 520 linhas, dispatcher compacto com armos de 1 linha
      para casos de domínio; excepção Regra 6 removida do
      `mod.rs`. `tests.rs` mantém nota Regra 6 (cfg(test) gated).
- [x] ADR-0037 promovida de `PROPOSTO` para `EM VIGOR` com 4
      ajustes (A/B/C/D) validados empiricamente nos Passos
      96.1–96.2. **Passo 96.3 concluído neste passo.**
- [x] `parse.rs` reestruturado por tipo de nó (markup, code,
      math, rules). **Passo 96.4 concluído** — 7 submódulos
      criados: `parser` (Parser struct + tipos de apoio, 700
      linhas), `math` (~377), `markup` (~205), `code` (~263),
      `rules` (~235), `patterns` (~485); `mod.rs` caiu para
      156 linhas (só entry points + declarações de submódulos
      + testes); nenhum submódulo excede 800 linhas. L1 tests:
      746 → 748 (+2 smoke tests em parser.rs para satisfazer V2).
- [x] `stdlib.rs` reestruturado por módulo da stdlib (text,
      layout, math, calc, etc.). **Passo 96.5 concluído** —
      9 submódulos criados: `foundations` (~188 linhas: type,
      len, rgb, luma, range, str, int, float), `calc` (~194:
      make_calc_module + 9 calc_*), `text` (~104: upper, lower,
      replace), `assert` (~60), `structural` (~84: strong, emph,
      raw, heading), `figure_image` (~105), `shapes` (~272:
      parse_color + rect/ellipse/circle/line/polygon), `transforms`
      (~87: move/rotate/scale), `layout` (~197: align/place/grid/
      page). `mod.rs` ficou em ~617 linhas (helpers `err`/
      `expect_no_named` + `mod` + `pub use` re-exports + tests).
      Re-exports de `pub use crate::rules::stdlib::<submod>::*`
      preservam os paths consumidos por `make_stdlib` em
      `eval/mod.rs` — zero alterações em eval. L1 tests: 748 → 748
      (comportamento idêntico). Nenhum submódulo acima de 800
      linhas.
- [x] Nota de visibilidade na Regra 3 da ADR-0037 e abertura
      do DEBT-47. **Passo 96.6 concluído** — sub-passo de
      governança (sem alteração de código). Adendo à Regra 3
      com 6 pontos de preferência e anti-padrão sobre
      `pub(super)`. DEBT-47 aberto para auditoria retroactiva
      dos Passos 96.1–96.5 após DEBT-46 encerrar.
- [x] `layout/mod.rs` reestruturado (orquestração, medição,
      emissão, sub-frames). **Passo 96.7 concluído** — 5
      submódulos novos criados: `metrics` (FontMetrics trait
      + FixedMetrics, 90 linhas), `grid` (braço Content::Grid,
      272 linhas), `placement` (Content::Align + Content::Place,
      201 linhas), `equation` (Content::Equation, 102 linhas),
      `cursor` (word/space/layout_word/flush_line/new_page,
      93 linhas), `helpers` (item_pos/translate_frame_item/
      measure_content/collect_sub_items, 138 linhas); `tests`
      movido para ficheiro próprio (1399 linhas, Regra 6
      cfg(test)-gated). `mod.rs` caiu de 2848 → 756 linhas
      (inclui struct Layouter + entry points + layout_content
      dispatcher). Visibilidade: 30 métodos `pub(super)` vs
      14 campos `pub(super)` — métodos dominam (2:1) conforme
      Regra 3 actualizada; campos `pub(super)` documentados em
      bloco-comentário no topo da struct. Zero regressão
      funcional: 748 → 753 L1 tests (+5 smoke V2). Zero
      violations.
- [x] `math/layout.rs` reestruturado ou marcado como excepção
      Regra 6. **Passo 96.8 concluído** — `git mv` de `math/layout.rs`
      para `math/layout/mod.rs`; 8 submódulos novos extraídos
      por método pesado de `MathLayouter`: `attach` (221 linhas),
      `root` (98), `frac` (87), `matrix` (74), `cases` (65),
      `stretchy` (64), `assembly` (82), `delimited` (48);
      `tests.rs` (761 linhas, Regra 6 cfg(test) gated). `mod.rs`
      ficou em 484 linhas (`MathBox` + helpers livres `offset_item`/
      `needs_grid_layout`/`partition_grid` + `MathLayouter` com
      métodos de coord: new, apply_axis_offset, layout_equation,
      layout_node, layout_text_node, layout_sequence,
      layout_grid_rows, layout_grid, hconcat). Visibilidade:
      **18 métodos `pub(super) fn` vs 7 campos `pub(super)`**
      (rácio 2.6:1), em linha com Regra 3. Campos `pub(super)`
      documentados em bloco-comentário da struct `MathBox` (4
      campos) + struct `MathLayouter` (3 campos). Bug descoberto
      durante extracção: contagem naïve de `{`/`}` em Python
      não distingue char literal `'{'` de delimitador — resolvido
      separando `layout_delimited` manualmente. Zero regressão:
      753 → 761 L1 tests (+8 smoke V2). Zero violations.
- [x] `lexer/mod.rs` reestruturado ou marcado como excepção
      Regra 6. **Passo 96.9 concluído — Via A (decomposição)** —
      o lexer tinha clusters claros por modo sintáctico
      (`impl Lexer<'_>` separado para markup, math, code).
      Extracção: `markup.rs` (419 linhas — markup/backslash/raw/
      link/numbering/ref_marker/label/text/in_word/space_or_end),
      `math.rs` (219 linhas — math/math_ident_or_field/
      maybe_dot_ident/math_text/maybe_math_named_arg/
      maybe_math_spread_arg), `code.rs` (257 linhas — code/
      invalid_char_in_code/ident/number/string). `mod.rs` caiu
      de 1250 → 468 linhas (struct Lexer + accessors + error/
      hint + dispatcher `next()` + whitespace/shebang/comments
      + ScannerExt trait + char-class free helpers + tests).
      Visibilidade: **16 métodos `pub(super) fn` vs 4 campos
      `pub(super)`** (rácio 4:1 — o melhor dos 96.7/96.8/96.9).
      Campos `pub(super)` documentados em bloco-comentário da
      struct Lexer. Zero regressão: 761 → 764 L1 tests (+3
      smoke V2). Zero violations.
- [x] Verificação final: nenhum ficheiro em `01_core/src/`
      acima de 800 linhas sem justificativa Regra 6 documentada
      no topo. **Passo 96.10 concluído neste passo.**

### Dependências

Nenhuma técnica. O trabalho é mecânico (movimentação de código
por domínio), não requer decisões arquitecturais adicionais.

### Nota sobre escopo

O DEBT aplica-se apenas a `01_core/src/`. Ficheiros em `02_shell/`,
`03_infra/`, `04_wiring/` não estão no escopo — se excederem o
limite, abrir DEBT específico por camada.

### Nota sobre encerramento

Este DEBT fecha quando os 7 ficheiros listados tiverem sido
tratados (reestruturados ou marcados como excepção). Encerramento
parcial não é aceitável — a coerência do princípio ADR-0037
requer aplicação consistente.

Se um dos ficheiros resistir à decomposição por razões técnicas
descobertas durante a execução, registar como **excepção Regra 6**
em vez de deixar como dívida pendente. A Regra 6 existe
precisamente para estes casos.

### Resultados finais (após Passo 96.10)

Trabalho completo da ADR-0037 Regra 2 aplicada a ficheiros
grandes de `01_core/src/rules/`.

**Antes** (inventário do Passo 96):

| Ficheiro | Linhas |
|----------|--------|
| `rules/eval.rs` | 3780 |
| `rules/layout/mod.rs` | 2848 |
| `rules/parse.rs` | 2255 |
| `rules/math/layout.rs` | 1806 |
| `rules/stdlib.rs` | 1711 |
| `rules/lexer/mod.rs` | 1250 |

Total: 13 650 linhas em 6 ficheiros acima de 1000.

**Depois** (após Passos 96.1–96.9):

| Estrutura original | Resultado |
|--------------------|-----------|
| `eval.rs` (3780) | `eval/mod.rs` (520) + 9 submódulos + `tests.rs` (2100) |
| `layout/mod.rs` (2848) | `layout/mod.rs` (755) + 6 submódulos + `tests.rs` (1399) |
| `parse.rs` (2255) | `parse/mod.rs` (156) + 6 submódulos |
| `math/layout.rs` (1806) | `math/layout/mod.rs` (484) + 8 submódulos + `tests.rs` (761) |
| `stdlib.rs` (1711) | `stdlib/mod.rs` (617) + 9 submódulos |
| `lexer/mod.rs` (1250) | `lexer/mod.rs` (463) + 3 submódulos |

**Ficheiros ainda acima de 800 linhas em `01_core/src/`**:

| Ficheiro | Linhas | Tipo | Justificativa |
|----------|-------:|------|---------------|
| `rules/eval/tests.rs` | 2100 | Testes E2E | Regra 5 + Regra 6 (cfg(test) gated, cobertura cross-domain) |
| `rules/layout/tests.rs` | 1399 | Testes E2E | Regra 5 + Regra 6 (idem, documentado no topo) |
| `entities/syntax_node.rs` | 1095 | Entidade estrutural | Regra 6 — árvore sintáctica fundamental, impls coesos |
| `entities/content.rs` | 1072 | Entidade fundamental | Regra 6 — enum central do domínio visual |
| `entities/layout_types.rs` | 850 | Vocabulário geométrico | Regra 6 — tipos coesos com impls cruzados |
| `entities/ast/expr.rs` | 845 | AST tipado | Regra 6 — ~50 variantes enum com impls `AstNode` |

Todos com justificativa Regra 6 documentada no topo (adicionadas
no Passo 96.10 onde faltavam).

**Contagem de testes**:

- Antes (Passo 96): 746 L1.
- Depois (Passo 96.10): **764 L1** (+18 smoke tests V2 obrigatórios
  dos submódulos novos, distribuídos: +2 parse, +5 layout, +8 math,
  +3 lexer).
- L3: 174 (inalterado).
- Ignorados: 6 (inalterado).

**Visibilidade** (somatório dos Passos 96.7/96.8/96.9, primeira
aplicação sistemática da nota Regra 3 introduzida no Passo 96.6):

| Passo | Alvo | Métodos `pub(super)` | Campos `pub(super)` | Rácio |
|-------|------|---------------------:|--------------------:|------:|
| 96.7 | `layout/` | 30 | 14 | 2.1 |
| 96.8 | `math/layout/` | 18 | 7 | 2.6 |
| 96.9 | `lexer/` | 16 | 4 | 4.0 |

Rácio aumenta à medida que a nota Regra 3 foi internalizada —
o lexer (último passo) teve a menor proporção de `pub(super)`
em campos. Tendência saudável.

**ADR-0037 validada empiricamente**. A Regra 2 (limite
orientativo de 800 linhas) e a Regra 6 (excepções documentadas)
funcionaram como esperado em 7 aplicações consecutivas
(eval, parse, stdlib, layout, math/layout, lexer, entidades).

**Dívida residual registada**:

- **DEBT-47** (auditoria de visibilidade dos `pub(super)`
  introduzidos) aberto no Passo 96.6 — pronto para ataque agora
  que todos os submódulos estão estáveis.
- **+18 smoke tests V2** no total — obrigatórios pelo linter,
  zero valor funcional. Podem ser revistos no futuro se a
  política do linter mudar.

**Lições**:

- Extractor naïve baseado em contagem de `{`/`}` não distingue
  char literal `'{'` de delimitador de bloco (bug encontrado no
  Passo 96.8). Para futuros refactorings grandes, sanitizar
  char literals antes ou usar parser AST.
- Padrão de re-export `pub use crate::rules::X::submod::Y` em
  `mod.rs` evita V14 (ADR-0037 Ajuste B) e é preferível a
  `pub use self::submod::Y`.
- Nota Regra 3 do Passo 96.6 reduziu bulk replace de
  `pub(super)` — rácio métodos/campos melhorou monotonicamente
  de 2.1 → 2.6 → 4.0 nos 3 passos subsequentes.

---

## DEBT-47 — Auditoria de visibilidade dos `pub(super)` aplicados nos Passos 96.1–96.5 — ENCERRADO (Passo 97) ✓

Os Passos 96.1, 96.2, 96.4 e 96.5 reestruturaram quatro
ficheiros grandes (`eval.rs`, `parse.rs`, `stdlib.rs`) em
submódulos conforme ADR-0037. Durante a extracção, visibilidade
de fields e métodos foi elevada para `pub(super)` em muitos
casos — alguns por bulk replace Python (reportado no Passo
96.4 para o `Parser` struct), outros por replace manual.

A ADR-0037 Regra 3 (clarificada no Passo 96.6) estabelece
preferência por:
- Manter privado quando possível.
- Métodos `pub(super)` sobre campos `pub(super)`.
- `pub(in path)` para escopo explícito quando aplicável.

### Escopo auditado

Todos os submódulos de `01_core/src/rules/`:

- `eval/`, `parse/`, `stdlib/`, `layout/`, `math/layout/`, `lexer/`.

### Critério de conclusão

- [x] Inventário de todos os `pub(super)` em
      `00_nucleo/diagnosticos/inventario-pub-super-passo-97.md`
      (269 itens encontrados).
- [x] Classificação R1/R2/R3/R4 por item — automática por grep +
      revisão manual dos falsos positivos.
- [x] Aplicação das mudanças sem regressão: 66 reduções R3
      aplicadas (com 6 reversões por leakage via inferência de
      tipo); bloco-comentário Regra 3 adicionado a `Parser` e
      `MathLayouter` (os outros 2 structs grandes — `Layouter`,
      `Lexer`, `MathBox` — já tinham do passo original).
- [x] `cargo test` preservado: **764 L1 + 174 L3 + 6 ignorados**.
- [x] `crystalline-lint` → **zero violations**.

### Resultado final (Passo 97)

| Módulo | Antes (96.10) | Depois (97) | Δ |
|--------|-------------:|------------:|--:|
| `parse` | 135 | 93 | −42 |
| `layout` | 46 | 44 | −2 |
| `math` | 29 | 26 | −3 |
| `eval` | 24 | 23 | −1 |
| `lexer` | 24 | 22 | −2 |
| `stdlib` | 11 | 3 | −8 |
| **Total** | **269** | **211** | **−58 (−22%)** |

Redução concentrada em `parse/` (parser.rs do bulk replace 96.4)
e em `stdlib/` (privados residuais que não eram usados entre
submódulos).

Rácio métodos:campos por módulo após auditoria:

| Módulo | fn | field | rácio |
|--------|---:|------:|------:|
| `parse` | 69 | 14 | 4.9 |
| `layout` | 28 | 14 | 2.0 |
| `math` | 14 | 7 | 2.0 |
| `eval` | 23 | 0 | ∞ |
| `lexer` | 14 | 4 | 3.5 |
| `stdlib` | 3 | 0 | ∞ |

Relatório completo em `00_nucleo/materialization/typst-passo-97-relatorio.md`.

### Dívida residual

Nenhuma relativa a visibilidade. Permanece apenas a dívida dos
+18 smoke tests V2 (registada na secção de encerramento do
DEBT-46, Passo 96.10).

---

## DEBT-3 — Safety rails hardcoded — RESOLVIDO (estrutura)

### Resolvido no Passo 28

- **`while` limit**: 10.000 → 1.000.000, via `EvalContext::tick_loop()`
- **`MAX_CALL_DEPTH`**: 200 → 250, via `EvalContext::check_call_depth()`
- **Limites documentados em `EvalContext`**: não mais magia inline
- **Métodos de verificação**: `check_call_depth()` e `tick_loop()` implementados
- **Limite global de loop**: contador acumulado ao longo de toda eval, não por loop

### Resolvido no Passo 29

- **Detecção de ciclos de importação**: `EvalContext::enter_import()` + `ImportGuard`
- **`import_stack: Vec<FileId>`**: rastreamento de ficheiros em avaliação
- **`ModuleImport` e `ModuleInclude`**: retornam Err limpo (não panic)

### Pendente (não é DEBT — é feature futura)

- **Implementação de `import` completo**: Passo 33+
- **Integração com `comemo`**: tracking semântico real (aguarda TrackedWorld real)

### Ficheiros alterados

- `01_core/src/rules/eval.rs`: `EvalContext` (max_call_depth 250, import_stack), `ImportGuard`, `ModuleImport`/`ModuleInclude` handling
- `01_core/src/rules/eval.rs`: testes novos de import_stack e ModuleImport/Include

---

## DEBT-6 — eval_for_test coverage blind spot — RESOLVIDO

### Registado e resolvido no Passo 32

**Problema**: `eval_for_test` usa `MockWorld` — um mundo artificial que não passa pelo mecanismo
de tracking real (`TrackedWorld`). Os testes de L1 nunca exercitavam o caminho de código de produção.

**Resolvido no Passo 32**:

- Testes de integração em `03_infra/src/integration_tests.rs`
- Pipeline completo exercitado: `SystemWorld` → `eval` → `layout` → `export_pdf`
- `eval()` pública confirmada como genérica sobre `TrackedWorld`
- `eval_for_test` mantida para testes unitários rápidos de L1

**Cobertura adicionada**:

- `pipeline_texto_simples`: eval + layout via SystemWorld
- `pipeline_export_pdf_helvetica`: export com fallback Helvetica
- `pipeline_export_pdf_com_fonte_real`: export com fonte do sistema (ou fallback)
- `pipeline_com_set_text_bold`: StyleChain via pipeline real
- `pipeline_com_closures`: closures via pipeline real
- `pipeline_eval_retorna_err_em_sintaxe_invalida`: robustez a input inválido

---

## DEBT-7 — Merge bold em layout — RESOLVIDO

**Registado no Passo 32. Resolvido no Passo 33.**

**Resolução**:

- Save/restore de `ctx.styles` em `Expr::CodeBlock` (`#{ }`)
- Save/restore de `ctx.styles` em `Expr::ContentBlock` (`[ ]`)
- Save/restore de `ctx.styles` em `apply_closure` (body de closures)
- Merge `bold || node_style.bold` e `italic || node_style.italic` removidos de `layout.rs`
- `#set text(bold: false)` dentro de um bloco agora reverte correctamente ao sair do bloco
- `node_style` capturado em eval já inclui o estilo correcto de Strong/Emph/Heading

**Ficheiros alterados**: `rules/eval.rs`, `rules/layout.rs`

---

## DEBT-10 — Contadores em duas passagens — RESOLVIDO

**Registado no Passo 57. Encerrado no Passo 62.**

**Resolução (Passagens 1 e 2)**:

- `01_core/src/rules/introspect.rs` criado: pré-passagem analítica que percorre
  `Content` sem alocações visuais, popula `resolved_labels: HashMap<Label, String>`.
- `layout()` recebe `CounterState` externo — o orquestrador chama `introspect` primeiro.
- Forward refs (`@conclusao` antes do heading `= Conclusão <conclusao>`) resolvem.
- Backward refs continuam a funcionar.
- `Content::Figure` implementado no Passo 62: introspecção rastreia figuras na
  Passagem 1, Layouter desenha blocos numerados na Passagem 2.

**Ficheiros criados/alterados**:

- `01_core/src/rules/introspect.rs` (novo)
- `01_core/src/entities/counter_state.rs` — `resolved_labels`, `headings_for_toc`, `auto_label_counter`
- `01_core/src/rules/layout/mod.rs` — `layout()` com estado externo
- `01_core/src/rules/layout/figure.rs` — braço `Content::Figure` (Passo 62)

---

## DEBT-11 — Decomposição de `layout.rs` — RESOLVIDO

**Registado no Passo 60. Resolvido no Passo 61.**

**Resolução**:

- `layout.rs` (1408 linhas) convertido em `layout/mod.rs` (orquestrador).
- `rules/layout/counters.rs`: braços `SetHeadingNumbering`, `CounterUpdate`, `CounterDisplay`.
- `rules/layout/references.rs`: braços `Ref` e `Labelled`.
- `rules/layout/outline.rs`: braço `Content::Outline` (TOC).
- L0 criados: `layout_counters.md`, `layout_references.md`, `layout_outline.md`.
- `layout()` passa a receber `CounterState` externo — o orquestrador chama `introspect` primeiro.

---

## DEBT-12 — Números de página na TOC — RESOLVIDO (Passo 63)

**Registado no Passo 61. Resolvido no Passo 63.**

**Resolução**: orquestração em 3 passagens no `compile_to_pdf` de L3:

- Passagem 1 (introspecção): `introspect()` popula `resolved_labels` e `headings_for_toc`.
- Passagem 2 (draft): `layout()` regista `label_pages` via `layout_labelled` e expõe-o em
  `PagedDocument::extracted_label_pages`. A TOC ainda não tem números de página.
- Passagem 3 (final): `layout()` com `initial_state.label_pages` preenchido; a TOC lê
  os números reais de `label_pages` para cada linha.

**Limitação residual**: registada como DEBT-17 (fixpoint da TOC).

---

## DEBT-13 — Efeitos colaterais duplicados na TOC — RESOLVIDO (mitigado, Passo 63)

**Registado no Passo 61. Mitigado no Passo 63.**

**Mitigação**: flag `CounterState::is_readonly` activa durante a renderização de cada
linha da TOC em `outline.rs`. Enquanto `is_readonly = true`, os métodos
`step_flat`, `step_hierarchical` e `update_flat` são no-ops — `CounterUpdate` embebido
nos clones de heading não avança os contadores.

**Limitação residual**: `CounterDisplay` ainda lê estado incorrecto na TOC (lê valores
do momento em que a TOC é renderizada, não do momento do heading real). Registado
como DEBT-18 (perda de contexto temporal em AST clonado na TOC).

---

## DEBT-14 — SetRule para `#set figure(numbering: ...)` — ENCERRADO ✓ (Passo 75)

**Registado no Passo 62.**

A numeração de figuras está activa por defeito (`CounterState::new()` insere
`"figure" = true`). O utilizador não consegue desactivá-la com
`#set figure(numbering: none)` até que o braço de SetRule para "figure" seja
adicionado ao `eval.rs`, produzindo um nó equivalente a `SetHeadingNumbering`.
Quando implementado, figuras sem numeração cujas labels forem referenciadas
mostrarão o fallback `@label` (comportamento intencional — ver braço `Labelled`
em `introspect.rs`).

---

## DEBT-15 — Campo `kind` hardcoded em `Content::Figure` — ENCERRADO ✓ (Passo 75)

**Registado no Passo 62.**

A chave `"figure"` está hardcoded em `step_flat("figure")` tanto na introspecção
como no layout. No Typst original, `#figure` aceita um argumento `kind`
(ex: `image`, `table`, `code`), e cada kind tem contador próprio —
"Tabela 1" e "Figura 1" são independentes. Com a implementação actual, tabelas
e gráficos partilham o mesmo contador.

**Resolução futura**: adicionar campo `kind: String` (default `"figure"`) a
`Content::Figure` e usar `step_flat(&kind)` em vez da string fixa.

---

## DEBT-16 — Acoplamento do Avaliador à Stdlib — RESOLVIDO (Passo 64)

**Registado no Passo 62. Resolvido no Passo 64.**

**Resolução**:

- `NativeFunc.call` mudou de `fn(&[Value])` para `fn(&Args)` — aceita positional e named args.
- `apply_func` passa `&args` directamente: `(native.call)(&args)`.
- Função auxiliar `expect_no_named()` adicionada em `stdlib.rs` — funções que não aceitam
  named args retornam `Err` semântico (não silencioso).
- `native_figure` migrada do interceptador hardcoded em `eval.rs` para `stdlib.rs`.
  `eval.rs` não contém nenhuma referência ao nome "figure".
- Cascata de 17 funções actualizadas (8 stdlib + 9 calc).
- Testes de `stdlib.rs` actualizados para usar `Args::positional(...)` em vez de `&[Value]`.

---

## DEBT-17 — Fixpoint da TOC — RESOLVIDO (Passo 65)

**Registado no Passo 63. Resolvido no Passo 65.**

**Resolução**: o algoritmo de fixpoint foi movido para dentro de `layout()` em L1.
O ciclo corre até que `extracted_label_pages` não mude entre iterações (máximo 5).
O orquestrador L3 (`compile_to_pdf`) voltou a ser linear:
`introspect()` → `layout()` → `export_pdf()`.

**Separação leitura/escrita**: `CounterState` tem dois campos distintos:

- `label_pages`: escrito por `references.rs` em cada iteração (começa vazio via `Layouter::new()`).
- `known_page_numbers`: lido por `outline.rs`, injectado pelo fixpoint com o mapa da iteração anterior.

`CounterState::has_outline`: sinalizado por `introspect()` na presença de `Content::Outline`.
Sem `has_outline`, `layout()` usa short-circuit de passagem única (sem fixpoint).

---

## DEBT-18 — Perda de Contexto Temporal em AST Clonado na TOC — RESOLVIDO (Passo 66)

**Registado no Passo 63. Resolvido no Passo 66.**

**Resolução**: materialização de AST na Passagem 1 (Introspecção).
A função `materialize_time(content, state)` em `introspect.rs` percorre o AST do
body de cada título e substitui cada `CounterDisplay` pelo seu valor em texto
estático (`Content::Text`) usando o `CounterState` actual naquele momento exacto.

`headings_for_toc.push` passou de `*body.clone()` para `materialize_time(body, state)`.

`CounterState::display_value(kind)` centraliza a lógica de leitura (hierárquico
vs plano) usada por `materialize_time` e por `layout/counters.rs`.

---

## DEBT-19 — Avaliação superficial de NodeKind — **ENCERRADO (Passo 69)**

`map_content` bottom-up implementado em `content.rs`. `apply_show_rules`
reescrito para usar `map_content` em seletores de nó — a árvore inteira
é percorrida. `NodeKind` expandido com Strong, Emph, Raw, Equation, ListItem.
`intercept_content` adicionado a `Expr::Strong` e `Expr::Emph` além de
`Expr::Heading` e `Expr::FuncCall`.

Substituição Texto→Content via Func/Content continua a gerar Err explícito.

---

## DEBT-20 — Guard anti-recursão booleano global — **ENCERRADO (Passo 70)** ✓

Substituído por `active_guards: Vec<RuleId>` no `EvalContext`. Pilha de
regras activas permite composição de regras sem stack overflow — a regra
cujo ID está na pilha é saltada, outras regras continuam a actuar.

---

## DEBT-21 — Resolução de NodeKind por string — **ENCERRADO (Passo 84.3)** ✓

**Registado no Passo 70. Mitigado no Passo 70. Desbloqueado no Passo 84.1.
Resolvido no Passo 84.3.**

A construção do selector em `eval_expr` (`Expr::ShowRule`) mapeava
`Value::Func` → `NodeKind` por comparação textual: `match f.name() {
Some("heading") => NodeKind::Heading, ... }`. Aliasing por wrapper closure
ou re-registo da mesma nativa com nome diferente não era detectado, e a
sintaxe estável de `fn_addr_eq` requeria Rust ≥ 1.85.

**Resolvido no Passo 84.3.** Substituído pelo método `Func::native_fn_addr()
-> Option<fn(...)>` (retorna `None` para closures, `Some(call)` para
nativas). A construção do selector usa `std::ptr::fn_addr_eq` para comparar
o ponteiro com cada nativa correspondente:

```rust
match f.native_fn_addr() {
    Some(addr) if fn_addr_eq(addr, native_heading as fn(_,_) -> _) =>
        Selector::NodeKind(NodeKind::Heading),
    // ... idem para figure, strong, emph, raw
    Some(_) => Err(...),  // nativa não suportada
    None    => Err(...),  // closure rejeitada explicitamente
}
```

`Func::name()` mantido para apresentação (mensagens de erro, debug) — não
mais para identidade. Sem `unsafe`. Suporte para `equation`/`list_item`
removido do match (não existem `native_*` correspondentes); pode ser
adicionado quando essas nativas forem registadas na stdlib.

Testes de regressão: `show_rule_resolve_por_identidade_nao_por_nome`
(aliasing via `#let h = heading; #show h: ...`),
`show_rule_closure_anonima_rejeitada` (closure como selector → Err
explícito).

---

## DEBT-22 — Clone de show_rules por nó — **ENCERRADO (Passo 84.4)** ✓

**Registado no Passo 68. Resolvido no Passo 84.4.**

`ctx.show_rules` migrada de `Vec<ShowRule>` para `Arc<[ShowRule]>` em
`EvalContext`. O clone em `intercept_content` (uma vez por nó AST visitado)
deixa de ser O(N) sobre os bytes da lista e passa a ser O(1) — só incrementa
o refcount do `Arc`. `Arc::clone(&ctx.show_rules)` substitui `clone()` no
hot path para sinalizar a intenção explicitamente.

Push (`#show` do utilizador) e truncate-back (saída de `Expr::CodeBlock`)
reconstroem o slice e ficam O(N), mas são caminhos frios — ordens de
magnitude menos frequentes que o clone do hot path.

Helpers em `EvalContext`:
- `push_show_rule(&mut self, ShowRule)` — encapsula a reconstrução por push.
- `truncate_show_rules(&mut self, len: usize)` — substitui `Vec::truncate`
  (`Arc<[T]>` não tem método equivalente). Reconstrói via `Arc::from(&[..len])`.

API de leitura inalterada: `Arc<[T]>` deriva `Deref<Target=[T]>`, portanto
`iter()`, `len()`, `is_empty()`, indexação e referência (`&rules`) continuam
a funcionar. `apply_show_rules(content, &rules, ctx)` aceita `&[ShowRule]`
sem alteração.

Padrão consistente com ADR-0026 revisão (`Arc<[Content]>` em
`Content::Sequence`). Novos campos com perfil semelhante (lista imutável
após construção, partilhada entre clones frequentes) devem seguir o mesmo
padrão.

`active_guards: Vec<RuleId>` ficou intencionalmente fora deste passo —
push/pop frequente sem clone é o caso onde `Arc<[T]>` regrediria em vez
de melhorar. Registado em DEBT-39 (Secção 1).

---

## DEBT-23 — Travessia múltipla em apply_show_rules — **ENCERRADO (Passo 70)** ✓

**Registado no Passo 69.**

`apply_show_rules` percorria a árvore uma vez por `ShowRule` activa: O(R×N)
com R regras e N nós. Resolução planeada: `map_content` chamado uma única
vez com todas as regras testadas por nó dentro da closure, implicando
mudança de assinatura de `map_content` para aceitar uma lista de regras em
vez de uma closure genérica.

**Resolvido no Passo 70.** `apply_show_rules` chama `map_content` uma
única vez para todas as regras `NodeKind`. Dentro da closure bottom-up,
todas as regras activas são testadas por nó antes de prosseguir — custo
reduzido de O(R×N) para O(N).

---

## DEBT-24b — Dimensões reais de imagem — **ENCERRADO (Passo 72)** ✓

Placeholder 100×100 substituído por leitura real de dimensões via trait
`ImageSizer` injectado no layouter. L3 implementa com `imagesize`;
L1 mantém-se puro (zero dependências externas adicionais).

---

## DEBT-24c — FrameItem::Image e export PDF — **ENCERRADO (Passo 73)** ✓

`FrameItem::Image` adicionado ao layouter e ao exportador PDF. JPEG embutido
com `/DCTDecode`. Deduplicação por `Arc::as_ptr`. Espaço reservado no layout
e imagem emitida no PDF para ficheiros JPEG.

---

## DEBT-25 — Resolução de caminhos relativos em stdlib (Passo 71) — **ENCERRADO (Passo 75)** ✓

`native_image` passa o caminho ao `World::read_bytes` tal como fornecido pelo
utilizador (relativo à raiz do projecto). Não há resolução relativa ao ficheiro
fonte activo. Resolução: `EvalContext` deve expor o path do ficheiro corrente
para que `native_image` construa um caminho absoluto.

---

## DEBT-26 — PartialEq O(N) sobre Arc<Vec<u8>> em Content::Image — **ENCERRADO (Passo 74)** ✓

A implementação manual de `PartialEq` para `Content::Image` compara `data`
por valor (`da == db`), o que é O(N) nos bytes da imagem. Em contextos de
teste ou deduplicação frequente, usar `Arc::ptr_eq` primeiro e cair para
comparação por valor apenas se os ponteiros diferirem.

---

## DEBT-27 — Suporte a transparência PNG no exportador PDF — **ENCERRADO (Passo 74)** ✓

PDF não suporta ficheiros PNG crus. PNG requer descodificação de píxeis
(canal RGBA separado para /SMask de transparência) antes de ser embutido.
Prova de conceito do Passo 73 usa apenas JPEG (/DCTDecode).
Resolução: Passo 74 adiciona `crate image` a L3 para descodificar PNG
em RGBA plano e passar ao gerador de PDF.

---

## DEBT-28 — Dupla leitura de cabeçalho de imagem no layouter — **ENCERRADO (Passo 74)** ✓

`calculate_dimensions` e `sizer.size()` lêem o cabeçalho da imagem separadamente.
O custo é mínimo (imagesize lê apenas o cabeçalho, não os píxeis), mas é
redundante. Resolução futura: `calculate_dimensions` retorna `ImageDimensions`
com `intrinsic_width`, `intrinsic_height` — eliminando a segunda chamada.

---

## DEBT-29 — Detecção de ColorSpace para JPEGs crus — **ENCERRADO (Passo 74)** ✓

O exportador assume `DeviceRGB` para todos os JPEGs. JPEGs Grayscale (1 canal)
ou CMYK (4 canais) com ColorSpace errado produzem lixo visual ou rejeição pelo
leitor PDF. Resolução: Passo 74 lê o marcador SOF0/SOF2 do cabeçalho JPEG para
determinar o número de canais e escolher `DeviceRGB`, `DeviceGray` ou `DeviceCMYK`.

---

## DEBT-30 — Suporte a clipping paths (clip: true) — ENCERRADO ✓ (Passo 79)

Contentores com `clip: true` requerem o operador `W` (clipping path) e a sequência
`W n` antes de desenhar o conteúdo interno. O PDF mantém um clipping path activo
por estado gráfico (`q`/`Q`), portanto `clip: true` exige um push/pop de estado
adicional. Campo `clip_mask: Option<ShapeKind>` adicionado a `FrameItem::Group`.
O exportador emite `W n` no espaço local do Group, após a matriz `cm`.

---

## DEBT-31 — Transformações afins (rotate, scale) em nós — **ENCERRADO (Passo 78)** ✓

Rotação e escala requerem a matriz `cm` com valores não-identidade:
`[cos -sin sin cos tx ty]` para rotação. A bounding box de um nó rodado não é
um rectângulo alinhado aos eixos — o layouter precisaria de calcular o
bounding box transformado. `Ellipse` usa rectângulo placeholder no exportador.
Resolução: passo futuro de transformações.

---

## DEBT-32 — Alinhamento da bounding box para linhas com deltas negativos — **ENCERRADO (Passo 77)** ✓

O exportador desenhava a linha a partir de `pos.x` sem considerar o sinal de `dx`/`dy`.
Se `dx < 0`, a linha saía para a esquerda da bounding box. A correcção mapeia
início/fim dentro da bounding box com base no sinal dos deltas.

---

## DEBT-34b — Parâmetro rows em Content::Grid ignorado — **ENCERRADO (Passo 83)** ✓

`Content::Grid` passa a respeitar `rows: Vec<TrackSizing>`. O motor de layout
implementa três passagens (Fixed → Auto → Fraction) espelhando a resolução de
colunas, com indexação cíclica `N % rows.len()` e decisão de paginação antes da
fase Fraction (evita alturas `fr` "fósseis" da página anterior).

---

## DEBT-34c — Alinhamento vertical de células no Grid — **ENCERRADO (Passo 83)** ✓

`cell_height` é agora passado como `available_h` a `resolve_alignment` para
items dentro de células (campo `cell_available_h: Option<f64>` no Layouter).
`VAlign::Bottom` ancora ao limite inferior da célula e `VAlign::Horizon` centra
verticalmente.

---

## DEBT-36 — Operadores simbólicos de alinhamento — **ENCERRADO (Passo 84.5)** ✓

**Registado no Passo 82. Resolvido no Passo 84.5.**

`Value::Align(Align2D)` adicionado ao enum `Value`. Constantes top-level
`left`, `center`, `right`, `top`, `horizon`, `bottom` registadas em
`make_stdlib()` como valores de `Align2D`. `eval_binary_op` trata
`BinOp::Add` entre dois `Value::Align` com **semântica vanilla**: combina
componentes de eixos distintos (`center + bottom` → Both) e devolve `Err`
em conflito de eixo (`center + right` → erro "cannot add two horizontal
alignments"). Esta semântica diverge da sugestão original do passo
("`b.h.or(a.h)` com sobrescrita") em favor da fidelidade ao vanilla,
conforme autorizado pelo enunciado.

Sintaxe preferida: `align(center + bottom, ...)`, `place(top + right, ...)`.

Sintaxe legacy `align("center", ...)` preservada — `Align2D::from_string`
continua a ser usada como fallback em `native_align` e `native_place` via
helper `extract_alignment(args, default)`. Remoção da sintaxe legacy e dos
4 testes L3 que ainda a usam fica para passo dedicado posterior.

Divergência estrutural não tratada: `Align2D` é struct + Option (permite
`(None, None)` "vazio"), enquanto vanilla `Alignment` é enum tagged
`{ H | V | Both }` (não permite vazio). Conversão para enum tagged está
fora do escopo deste passo.

Testes de regressão: `align_plus_combina_eixos_distintos`,
`align_plus_eixo_horizontal_repetido_falha`, `align_plus_eixo_vertical_repetido_falha`
(L1, unitários sobre `eval_binary_op`); `align_aceita_constante_simbolica`
e `align_aceita_composicao_via_plus` (L3, pipeline real).

---

## DEBT-37 — Place relativo ao contentor pai — **ENCERRADO (Passo 84.6)** ✓

**Registado no Passo 82. Resolvido no Passo 84.6.**

A descrição original do DEBT estava parcialmente incorrecta: o eixo X de
`Content::Place` já ancorava à coluna desde o Passo 81.5 (via
`line_start_x`); só o eixo Y era absoluto à margem da página.

**Diagnóstico (Passo 84.6):** o vanilla tem `enum PlacementScope { Column
(default), Parent }`. `Column` ancora ao "current container" (`regions.base()`)
— célula activa quando dentro de Grid; `Parent` ancora à página
("spans columns") e é restrito a `float: true`.

**Resolvido no Passo 84.6 (cenário A do enunciado):**

- `PlaceScope { Column (default), Parent }` adicionado em `layout_types.rs`.
- Campo `scope: PlaceScope` adicionado a `Content::Place`. Cascata em
  `PartialEq`, `map_content`, `map_text`, `introspect::materialize_time`.
- Layouter recebeu 3 campos novos: `cell_origin_x`, `cell_origin_y`,
  `cell_origin_w` (`Option<f64>`), em paralelo ao `cell_available_h` do
  Passo 83. Save/restore por célula no braço Grid.
- Braço `Content::Place` selecciona área de ancoragem por scope: Column
  com cell_* todos Some → célula; Column sem cell_* (fora de Grid) →
  página; Parent → sempre página.
- Compensação Y para evitar dupla translação no sub_frame de célula:
  quando `cell_origin_y.is_some()`, subtrai `cell_origin_y` em vez de
  `sub_origin_y` ao transferir items — anula a translação posterior do
  Grid (`row_start_y + (item.y - ascender_local)`).
- `native_place` aceita argumento nomeado `scope: "column" | "parent"`;
  default `Column`. String inválida → erro explícito.

**Divergência face ao vanilla a documentar:** o vanilla restringe `Parent`
a `float: true` (erro caso contrário, `collect.rs:309`). O cristalino não
tem `float` implementado — `Parent` é aceite incondicionalmente, com
efeito visual de ancoragem à página sem layout flutuante. Quando `float`
for adicionado, repor a restrição.

Sintaxe: `place("bottom-right", scope: "parent", rect(...))`.
Sintaxe legacy `place("bottom-right", ...)` continua a funcionar (default
Column → ancora à célula se dentro de Grid, à página caso contrário).

Testes de regressão: `place_dentro_de_grid_ancora_a_celula` (Column dentro
de Grid → coords da célula), `place_dentro_de_grid_com_scope_parent_ancora_a_pagina`
(Parent dentro de Grid → coords da página). `place_nao_altera_cursor_y`
(P82) continua a passar — place fora de Grid ancora à página por default
Column (cell_* todos None → fallback para página).

---

## DEBT-38 — Cache de sub-frames no Grid Auto — **ENCERRADO (Passo 84.2)** ✓

**Registado no Passo 83.**

A resolução de altura de linhas Auto chamava `layout_sub_frame_with_width`
para medir a altura intrínseca de cada item, descartando os FrameItems
produzidos. Quando a célula era emitida no documento, a mesma função
era chamada de novo para o mesmo item com a mesma largura, duplicando
o trabalho de layout em todas as células Auto.

**Resolvido no Passo 84.2.** Cache local `HashMap<usize, (f64, Vec<FrameItem>)>`
no braço `Content::Grid`, populado na fase de medição Auto e consumido
(via `remove`) na fase de emissão. Chave: `row_idx * num_cols + col_idx`.
Cache sai de escopo no fim do braço — sem invalidação manual.

Não usa `ptr::addr_of` nem cast de ponteiro (`unsafe` excluído pela
convenção cristalina). Não usa `Arc<[FrameItem]>` no valor (transferência
única do cache para o frame, sem partilha). `std::collections::HashMap`
é permitida em L1 por ADR-0029 (pureza física — RAM é domínio, não I/O).

---

## DEBT-41 — Sealed traits no scanner usam `unsafe trait` — **ENCERRADO (Passo 85)** ✓

`01_core/src/rules/lexer/scanner.rs` tem 6 `unsafe impl Sealed<T>`
usando o padrão sealed-trait clássico da stdlib Rust. A palavra
`unsafe` aqui é mecanismo de encapsulamento (impedir
implementações externas), não indicação de memória não-segura.

ADR-0032 estabelece que `unsafe` em L1 é eliminado por defeito;
este caso tem custo zero — é refactor mecânico para o padrão
"sealed via private module".

### Proposta de resolução

Substituir:

```rust
pub unsafe trait Sealed<T> { ... }
unsafe impl Sealed<char> for ... { ... }
```

Por:

```rust
mod sealed {
    pub trait Sealed<T> { ... }
}

pub trait Pattern: sealed::Sealed<char> { ... }
impl sealed::Sealed<char> for ... { ... }
```

Verificar que nenhum consumer externo das traits do scanner
depende da assinatura `unsafe trait` (uso em bounds genéricos
deveria continuar a funcionar).

### Critério de conclusão

- Zero ocorrências de `unsafe` associadas ao padrão Sealed em
  `scanner.rs`.
- Testes do scanner continuam a passar sem alteração.
- Nenhum impacto visível na API pública de `01_core/src/rules/lexer/`.

### Dependências

Nenhuma. Refactor trivial, pronto para atacar.

**Resolvido no Passo 85.** Padrão "sealed via private module"
aplicado. Zero `unsafe` associadas a Sealed em `scanner.rs`.
Get_unchecked permanece (DEBT-42, bloqueado por benchmark).

---

## DEBT-40 — `ImportGuard::drop` com raw pointer — **ENCERRADO (Passo 90)** ✓

`01_core/src/rules/eval.rs:235` tem `unsafe { (*self.stack_ptr).retain(...) }`
no `Drop for ImportGuard`. O raw pointer é usado porque a vida do
`EvalContext` não é expressível como lifetime do guard (RAII com
scope de função).

ADR-0032 estabelece que `unsafe` em L1 é eliminado por defeito;
este caso tem custo de eliminação zero a baixo. Resolução fica
para passo dedicado que escolha entre as opções abaixo.

### Opções de resolução (sugestões, não obrigatórias)

1. **`Rc<RefCell<Vec<FileId>>>`** no `EvalContext` + clone no
   guard. Custo: uma alocação `Rc` + borrow check runtime.
2. **Eliminar o guard**. Push/pop manual em torno da chamada
   recursiva. Perde-se RAII — se erro acontece entre push e pop,
   `pop` não corre. Aceitável se o `EvalContext` é descartado em
   erro.
3. **Índice + verificação com `len()`**. Guardar posição no
   stack em vez de ponteiro; comparar com `len()` no drop para
   confirmar que é o topo; pop se sim, erro ou no-op se não.
   Sem custo de performance, sem `unsafe`, mas perde a garantia
   estrutural de que o guard corresponde à sua entrada.

### Critério de conclusão

- Nenhuma ocorrência de `unsafe` em `eval.rs` relacionada com
  `ImportGuard`.
- Testes de `import_stack` (detecção de ciclos) continuam a passar.
- ADR específico escrito se a resolução introduzir decisão
  arquitectural não prevista.

### Dependências

Nenhuma. Pode ser atacado quando o utilizador decidir.

**Resolvido no Passo 90.** `ImportGuard`, `impl Drop for ImportGuard`
e o bloco `unsafe { (*stack_ptr).retain(...) }` foram removidos.
`EvalContext.import_stack` substituído por `EvalContext.route:
Vec<FileId>` com API segura `with_route_id(id, span, f)` que empurra
antes e retira após a closure (incluindo em `Err`). Simultaneamente:
(i) fecha a divergência estrutural face ao vanilla (ADR-0033) ao
materializar `Route<'a>` em `01_core/src/entities/world_types.rs`
com a forma canónica (`outer: Option<Tracked<'a, Self>>`, `id`,
`len`, `upper: AtomicUsize`, `#[comemo::track]` em `contains`/`within`,
`check_*_depth` para as 4 categorias); (ii) elimina o último
`unsafe` de `eval.rs` (ADR-0032). A escolha entre as opções acima
foi variante (2) — eliminar o guard, push/pop explícito — porque
`EvalContext` em cristalino é mutado em vez de recriado por
recursão (Classe A do plano do Passo 90). Teste E2E
`import_cycle_detectado_retorna_err_sem_panic` valida, via API
pública de `eval`, que um ciclo `main → other → main` retorna
`Err` sem panic. O `scanner.rs` continua a ser o único `unsafe`
em L1 (DEBT-42, bloqueado por infra de benchmark).

---

## DEBT-44 — `EvalContext` não usa estruturalmente `Route<'a>` — **ENCERRADO (Passo 92)** ✓

O Passo 90 materializou `Route<'a>` em
`01_core/src/entities/world_types.rs` com paridade estrutural
completa face ao vanilla (`outer: Option<Tracked<'a, Self>>`,
linked list imutável, `#[comemo::track]` em `contains`/`within`).

No entanto, `EvalContext` (em `01_core/src/rules/eval.rs`) não usa
`Route<'a>` estruturalmente. Mantém campo `pub route: Vec<FileId>`
como projecção plana — uma lista linear que imita a cadeia mas não
é a estrutura vanilla. A API `with_route_id(id, span, f)` substitui
o `ImportGuard` antigo com push/pop explícito sobre este `Vec`, sem
`unsafe`.

### O que o Passo 90 resolveu

- `unsafe` em `ImportGuard::drop` eliminado (ADR-0032 avança).
- `Route<'a>` existe como tipo em L1 para uso futuro.
- DEBT-40 encerrado (o `unsafe` era o seu critério de fecho).

### O que o Passo 90 não resolveu

- Divergência estrutural face ao vanilla: o `eval` do vanilla
  propaga `Route<'a>` por valor entre frames (linked list imutável);
  o cristalino propaga estado partilhado num `Vec` dentro de
  `EvalContext`.
- `Tracked<'a, Route>` não é usado como parâmetro de funções
  memoizadas, mesmo quando a infraestrutura `comemo` já está
  disponível.

### Razão do adiamento

Opção escolhida no Passo 90 por decisão pragmática: integrar
`Route<'a>` estruturalmente exigiria refactor transversal de ~12
funções `eval_*` (parâmetro `route: &Route<'_>` ou campo
`route: Route<'a>` no `EvalContext<'w>`). Benefício observável
limitado enquanto `Expr::ModuleImport` e `Engine<'a>` continuam
stubs. A decisão preservou o valor principal (fechar `unsafe`,
fechar DEBT-40) e adiou o valor estrutural.

### Critério de conclusão

- [x] `EvalContext.route: Vec<FileId>` eliminado.
- [x] Mecanismo de detecção de ciclo usa `Route<'a>` directamente
      (parâmetro `route: Tracked<'r, Route<'r>>` nas 10 funções
      `eval_*`/`apply_*`/`intercept_*` que participam na recursão).
- [x] API pública do eval não muda (comportamento observável
      preservado — ADR-0033).
- [x] Teste E2E `import_cycle_detectado_retorna_err_sem_panic`
      continua a passar sem alteração.
- [x] Nenhum novo `unsafe` introduzido.

### Dependências

Nenhuma técnica. Passo dedicado quando priorizado. O Passo 90 deixou
o cenário preparado: `Route<'a>` existe e tem API funcional; falta
apenas ligá-lo ao `EvalContext`.

### Nota sobre escopo

A integração estrutural, quando atacada, pode revelar que o escopo
real é maior do que ~12 funções — por exemplo, se o `Engine<'a>`
for materializado simultaneamente. Nesse caso, este DEBT pode
dividir-se em sub-DEBTs.

**Resolvido no Passo 92.** Primeira aplicação concreta da ADR-0036
(atomização progressiva). Mudanças efectivas:

1. `EvalContext.route: Vec<FileId>`, `route_contains`, `with_route_id`
   e o pré-push de `current_file` em `EvalContext::new` foram
   removidos. `EvalContext` deixa de carregar estado de
   detecção-de-ciclo.
2. As 10 funções que participam na recursão
   (`eval`, `eval_markup`, `eval_expr`, `eval_markup_body`,
   `eval_args`, `eval_conditional`, `eval_while`, `eval_for`,
   `eval_let`, `eval_counter_method`) ganham parâmetro
   `route: Tracked<'r, Route<'r>>` (lifetime nomeada `'r` devido
   à invariância de `Route<'_>`). `apply_func`, `apply_closure`,
   `apply_show_rules` e `intercept_content` ganham o mesmo
   parâmetro por propagação.
3. `eval_math_content`/`eval_math_expr` permanecem inalteradas
   (tier 3 — não têm caminho a `Expr::ModuleInclude`).
4. `world_types.rs::Route::outer` passou a parametrizar
   explicitamente a `Constraint` via
   `Tracked<'a, Self, <Route<'static> as Validate>::Constraint>`
   — pattern documentado pelo `comemo 0.4.0` para habilitar
   covariância em cadeias `Tracked`. Sem isto o encadeamento
   recursivo de `Route::extend` não compilava (erro de
   invariância).
5. `Expr::ModuleInclude` verifica ciclos via `route.contains(id)`
   real e cria frame filho via `Route::extend(route).with_id(id)`,
   passando `child_route.track()` à chamada recursiva — paridade
   directa com `typst-eval/src/import.rs:232` do vanilla.
6. Os 3 testes `with_route_id_*` do Passo 90 foram removidos
   (testavam o mecanismo intermédio que desapareceu). O teste
   E2E `import_cycle_detectado_retorna_err_sem_panic` passa sem
   alteração — valida apenas comportamento observável. Contagem
   L1: 744 (747 − 3).

DEBT-45 (`check_*_depth` não chamados) continua em aberto;
Passo 93 ligá-los-á agora que `route` está propagado.

---

## DEBT-39 — `active_guards` com push/pop frequente — **ENCERRADO (Passo 95)** ✓

Diagnóstico do Passo 84.4 revelou que `EvalContext.active_guards`
(`Vec<RuleId>`) tem padrão push/pop por entrada/saída de cada show rule
em `apply_show_rules`, **sem clones**. Difere de `show_rules` (resolvido
no 84.4 via `Arc<[ShowRule]>`) — `Arc<[RuleId]>` aqui forçaria
reconstrução O(n) por push e por pop, regressão face ao push/pop O(1) do
`Vec` actual.

Padrão actual (eval.rs):
- `ctx.active_guards.push(rule.id)` antes de `apply_func`.
- `ctx.active_guards.pop()` imediatamente após.
- `ctx.active_guards.contains(&rule.id)` na decisão de saltar regra.

Soluções candidatas para revisão futura:
- Manter `Vec<RuleId>` — aceitar custo zero do clone (nenhum existe) e
  push/pop O(1) como o melhor compromisso. Encerrar este DEBT como
  "registado para clarificar que não é regressão face a 84.4".
- Estrutura persistente (`im::Vector`, `rpds::List`) — push/pop O(1)
  imutável, mas introduz dependência externa não justificada hoje.

Não é candidato a resolução até surgir evidência empírica de problema
(ex: composição profunda de show rules a degradar em documentos reais).

**Resolvido no Passo 95 (Tarefa C, Opção 1).** `active_guards` deixou
de ser campo de `EvalContext` e passou a ser parâmetro
`active_guards: &mut Vec<RuleId>` propagado pelas mesmas 13 funções
que já recebem `route`/`styles`/`show_rules` desde a série de
atomização da ADR-0036 (Passos 92/94/95).

A **preocupação de performance** original do DEBT (não regredir para
`Arc<[RuleId]>`) mantém-se respeitada: o tipo continua `Vec<RuleId>`;
push/pop continuam O(1); zero clones por iteração. A mudança é
**exclusivamente de ownership**, não de representação.

Razão da decisão (Opção 1 em vez de Opção 2 "excepção da Regra 4"):
o DEBT-39 documentava preferência de TIPO (`Vec` sobre `Arc<[T]>`),
não preferência de LOCALIZAÇÃO (campo vs. parâmetro). A ADR-0036
trata da segunda dimensão. Extrair como parâmetro mantendo o tipo é
a solução que satisfaz ambas as restrições — o que o DEBT protege
(O(1) push/pop) é independente de onde o `Vec` vive.

---

## Secção 3 — Dívida de instrumentação

# Dívida de instrumentação — ADR-0006

Os seguintes pontos de timing foram removidos para manter L1 puro.
Religação prevista no Passo 10 (isolamento de comemo/infra).

| Função       | Nome do scope original |
|--------------|------------------------|
| parse()      | "parse"                |
| parse_code() | "parse-code"           |
| parse_math() | "parse-math"           |

## Como religar no Passo 10

Localizar todos os `// ADR-0006: timing removed — ver 00_nucleo/DEBT.md`
em `01_core/src/rules/parse.rs` e substituir `timing_scope!("...")` por
o mecanismo de telemetria escolhido (trait injectável ou outro).

Ver: `00_nucleo/adr/typst-adr-0006-typst-timing.md`

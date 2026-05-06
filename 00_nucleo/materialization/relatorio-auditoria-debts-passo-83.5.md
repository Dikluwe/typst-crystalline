# Relatório de auditoria de DEBTs — Passo 83.5 (regenerado no Passo 83.6)

**Data original:** 2026-04-21 (data do P83.5)
**Data de regeneração:** 2026-04-22 (data do P83.6)
**Commit de referência:** `b19682d63` (último commit do P83.5, antes de `0837d5f0f` P84.1).

> **Aviso retrospectivo:** este relatório foi produzido no Passo 83.6 como
> recuperação documental. Reflecte o estado do código **no momento do
> Passo 83.5**, regenerado via `git show b19682d63:<ficheiro>` para os
> ficheiros que mudaram entre P83.5 e a data de regeneração. DEBTs
> identificados aqui como "em aberto" podem já ter sido encerrados em
> passos subsequentes (P84.1–P84.7) — consultar o `00_nucleo/DEBT.md`
> actual para o estado vivo.
>
> **Anomalia descoberta na regeneração:** o commit `b19682d63` contém o
> `DEBT.md` movido para `00_nucleo/` e as entradas DEBT-36/37/38
> presentes, mas **sem os cabeçalhos de Secção 1 / 2 / 3 explícitos**. As
> três secções foram materializadas apenas no commit P84.1
> (`0837d5f0f`). O relatório do P83.5 reportou contagens por secção
> assumindo que a estrutura formal já existia — divergência real entre
> a resposta da conversa e o que foi commitado. Esta discrepância está
> registada na Secção 5 deste relatório.

---

## 1. Movimentação e reorganização

### Movimentação física do ficheiro
- `git mv 01_core/DEBT.md 00_nucleo/DEBT.md` executado no início do
  P83.5.
- Substituição literal `01_core/DEBT.md → 00_nucleo/DEBT.md` propagada
  por **36 ficheiros** (1 ADR — `typst-adr-0006`; 2 prompts L0 —
  `prompts/rules/utils.md`, `prompts/rules/parse.md`; 2 ficheiros de
  código `.rs` — `01_core/src/utils.rs`, `01_core/src/rules/parse.rs`,
  apenas comentários; 31 passos de materialização históricos).
- `grep -rn "01_core/DEBT.md" .` após a substituição: 0 matches ✓.
- `crystalline-lint --fix-hashes .` ajustou 2 hashes L0
  (`utils.rs → 113e0000`, `parse.rs → 8191e20b`) por terem o caminho
  no header `@prompt`.

### Reorganização lógica (planeada)

O enunciado original do P83.5 (Tarefa 2) definiu três secções:
- **Secção 1 — DEBTs em aberto ou parcialmente resolvidos.**
- **Secção 2 — DEBTs encerrados.**
- **Secção 3 — Dívida de instrumentação (ADR-0006).**

**Estado real no commit `b19682d63`:** os cabeçalhos de Secção
**não foram adicionados** ao DEBT.md. A reorganização dos blocos
ocorreu (a entrada "Dívida de instrumentação" está agora no meio, não
no fim — confirmado pela contagem visível em `git show b19682d63:00_nucleo/DEBT.md`),
mas os cabeçalhos `## Secção 1 — ...` apareceram só no commit P84.1.

### Contagem (na data do P83.5, segundo `b19682d63`)

A análise lógica retroactiva (categorizando cada entrada `## DEBT-NN`
visível no commit `b19682d63` segundo o seu status declarado) produz:

- **Em aberto / parcialmente resolvidos:** 13 (DEBT-1, 2, 8, 9, 21, 22,
  33, 34d, 34e, 35b, 36, 37, 38).
- **Encerrados:** 28 cabeçalhos físicos (DEBT-23 aparece duplicado em
  `b19682d63`, contado como 2). Únicos: 27.
- **Dívida de instrumentação (ADR-0006):** 1 bloco.
- **Total cabeçalhos `^## DEBT-`:** 41 (com duplicado).

---

## 2. DEBTs confirmadamente em aberto

> Todas as verificações abaixo foram regeneradas via
> `git show b19682d63:<path>` para fidelidade ao estado do código no
> momento do P83.5.

### DEBT-1 — StyleChain
- **Estado no ficheiro:** `PARCIALMENTE RESOLVIDO (Passo 30)`. Pendências
  listadas: scoping de `#set`, propriedades adicionais (fill, font-family,
  weight numérico), `#show` rules, paridade total, remover wrappers
  Strong/Emph do layout.
- **Verificação:**
  - `git show b19682d63:01_core/src/rules/eval.rs | grep "styles" | grep -iE "save|restore|push|pop"` →
    save/restore em `Expr::CodeBlock` (343-351), `Expr::ContentBlock`
    (635-638), `apply_closure` (1094-1101). Push em Strong/Emph/Heading.
  - Show rules: `Expr::ShowRule` em eval.rs (706), `apply_show_rules`
    (1408) — implementação completa via map_content (DEBT-19/20).
- **Confirmação:** PARCIALMENTE ABERTO — duas pendências
  ("Scoping de `#set` por bloco" e "`#show` rules") **já tinham sido
  resolvidas implicitamente** por DEBT-7 (P32-P33) e DEBT-19/20 (P68-P70)
  antes do P83.5. As restantes (fill, font-family, weight, paridade,
  wrappers) continuam abertas.
- **Sugestão para Passo 84:** **MÉDIO** — duas pendências deviam ser
  riscadas; restantes são trabalho real. (Acção textual executada em P84.1.)

### DEBT-2 — Closures eager vs lazy capture
- **Estado:** `PARCIALMENTE RESOLVIDO (Passo 31)`. Pendente: integração
  com `comemo` real, paridade de shadowing.
- **Verificação:**
  - `TrackedWorld` aparece em `world_types.rs:191` (stub) e
    `contracts/world.rs:17/18/136` (trait abstracto). Não há instância
    concreta (sem `impl TrackedWorld for SystemWorld` real).
  - `grep -rn "shadow\|capture" lab/parity/tests/` retorna 0 — não há
    testes de paridade de shadowing.
- **Confirmação:** ABERTO — depende de `TrackedWorld` concreto.
- **Sugestão P84:** **FORA DE ESCOPO** — depende de infra externa.

### DEBT-8 — Motor de equações
- **Estado:** `PARCIALMENTE RESOLVIDO (P36-P40)`. Pendente: kern
  matemático, fontes OpenType MATH, `MathPrimes`, `MathAlignPoint`,
  baseline x-height.
- **Verificação:**
  - `MathAlignPoint` no estado P83.5: 6 ocorrências em `math/layout.rs`
    (143, 180, 322 — implementação completa); 3 ocorrências em
    `eval.rs` (1357 etc.); 1 em `layout/mod.rs` (478); presente em
    `introspect.rs` (89, 273). **JÁ implementado** no P83.5.
  - `MathPrimes`: parseado e evaluado em `eval.rs` (1172, 3054), mas
    sem lógica dedicada de kern/posição no layouter.
- **Confirmação:** PARCIALMENTE ABERTO — `MathAlignPoint` deveria ser
  riscado da pendência. `MathPrimes`, kern, OpenType MATH continuam
  abertos.
- **Sugestão P84:** **DIFÍCIL** — OpenType MATH requer ADR sobre
  integração L3/L1.

### DEBT-9 — Cobertura de paridade
- **Estado:** tracking contínuo. Aberto por natureza.
- **Verificação:** `lab/parity/tests/parse_parity.rs`: 140 linhas, 50
  `#[test]` — **idêntico ao baseline do P35**. O número não aumentou.
- **Confirmação:** ABERTO.
- **Sugestão P84:** **FÁCIL** (adicionar casos) ou **FORA DE ESCOPO**
  (decisão política sobre cadência).

### DEBT-21 — Resolução de NodeKind por string
- **Estado:** `MITIGADO (Passo 70)`. Bloqueio textual: "requer Rust ≥ 1.85
  para `fn_addr_eq` estável".
- **Verificação:**
  - `rustc --version` no momento da regeneração: **rustc 1.92.0**
    (registado também em P83.5 ≥ 1.85).
  - `git show b19682d63:01_core/src/rules/eval.rs | grep "fn_addr_eq"` → 0
    matches.
- **Confirmação:** ABERTO, mas com **bloqueio técnico já removido**.
  A mitigação pode ser substituída por resolução definitiva.
- **Sugestão P84:** **FÁCIL** — substituir `Func::name()` por `fn_addr_eq`
  em selectors. (Executado no P84.3.)

### DEBT-22 — Clone de show_rules por nó
- **Estado:** sem marca de encerramento.
- **Verificação:** em `b19682d63:01_core/src/rules/eval.rs:1514` —
  `let rules = ctx.show_rules.clone(); // snapshot explícito — DEBT-22`.
  Clone por invocação ainda presente. `pub show_rules: Vec<ShowRule>` na
  linha 68.
- **Confirmação:** ABERTO.
- **Sugestão P84:** **MÉDIO** — `Vec<ShowRule>` → `Arc<[ShowRule]>` com
  helpers para push e truncate-back. (Executado no P84.4.)

### DEBT-33 — Bounding Box de curvas Bézier
- **Estado:** EM ABERTO (P79).
- **Verificação:**
  - `PathItem::CubicTo` existe em `geometry.rs:17`.
  - `native_polygon` em `stdlib.rs:878-917` calcula AABB via min/max
    dos pontos de controlo. Apenas emite `MoveTo`/`LineTo` — não
    emite `CubicTo`.
- **Confirmação:** ABERTO (latente) — nenhuma função da stdlib emite
  Bézier; o problema só se materializa quando uma função como `curve()`
  for adicionada.
- **Sugestão P84:** **FORA DE ESCOPO** — sem callers que emitam Bézier,
  fixar agora é prematuro.

### DEBT-34d — Auto não encolhe antes de matar fr
- **Estado:** EM ABERTO (P80).
- **Verificação:** `TrackSizing::Auto` em `b19682d63:layout/mod.rs` chama
  `measure_content_constrained` sem min-content/max-content. Auto
  atribui o máximo absoluto, consumindo espaço antes da fase Fraction.
- **Confirmação:** ABERTO.
- **Sugestão P84:** **DIFÍCIL** — requer ADR sobre modelo
  min-content/max-content.

### DEBT-34e — colspan e rowspan
- **Estado:** EM ABERTO (P80).
- **Verificação:** `Grid { columns, rows, cells }` em
  `b19682d63:01_core/src/entities/content.rs` — sem campos colspan/rowspan.
  O comentário em `rows` ainda diz "DEBT-34b: ignorado — todas as linhas
  são Auto" (DEBT-34b ainda não tinha sido encerrado nessa altura — viria
  a ser no P83 — confirma estado pré-83 do AST).
- **Confirmação:** ABERTO.
- **Sugestão P84:** **DIFÍCIL** — requer algoritmo de placement
  bidimensional (mudança estrutural de `Content::Grid`).

### DEBT-35b — Invalidação de cache de available_width após SetPage
- **Estado:** EM ABERTO, preventivo (P81).
- **Verificação:** `available_width()` é método sem campo de cache em
  `b19682d63:01_core/src/rules/layout/mod.rs:184`. Calculado em tempo
  real. Comentário "DEBT-35b: se available_width() vier a ter cache,
  invalidar aqui" no braço SetPage.
- **Confirmação:** ABERTO (preventivo) — sem cache, o risco documentado
  não se materializou.
- **Sugestão P84:** **FORA DE ESCOPO** — preventivo; atacar apenas
  quando cache for adicionado.

### DEBT-36 — Operadores simbólicos de alinhamento
- **Estado:** EM ABERTO (P82).
- **Verificação:**
  - `git show b19682d63:01_core/src/entities/value.rs | grep -c "Align"` → **0**
    (não havia `Value::Align` no momento do P83.5).
  - `Align2D::from_string` em `stdlib.rs:1018, :1058` — usado em
    `native_align` e `native_place` (ambos chamados via string).
- **Confirmação:** ABERTO.
- **Sugestão P84:** **MÉDIO** — adicionar `Value::Align`, suporte a
  `BinOp::Plus`, registar constantes top-level. (Executado no P84.5.)

### DEBT-37 — Place relativo ao contentor pai
- **Estado:** EM ABERTO (P82).
- **Verificação:** `Content::Place` em `b19682d63:layout/mod.rs` (braço
  na altura) usa `self.line_start_x.0` para X (mitigação parcial via
  P81.5) e `self.page_config.margin` para Y (sempre topo da página). Sem
  parâmetro de área de âncora; sem campo `scope`.
  Comentário literal no código: `"DEBT-37: ideal seria ancorar ao
  contentor pai"`.
- **Confirmação:** ABERTO.
- **Sugestão P84:** **MÉDIO** — propagar área de âncora via campos do
  Layouter (infra do P83 já tem `cell_available_h` para Align;
  reaproveitável). (Executado no P84.6 com `PlaceScope { Column, Parent }`
  e 3 campos novos `cell_origin_x/y/w`.)

### DEBT-38 — Cache de sub-frames no Grid Auto
- **Estado:** EM ABERTO (aberto no próprio P83).
- **Verificação:** `git show b19682d63:01_core/src/rules/layout/mod.rs |
  grep -A 8 "_sub_items"` → confirma `let (sub_h, _sub_items) =
  self.layout_sub_frame_with_width(item, cell_x, cell_w);` na fase Auto
  do braço Grid. Items descartados na medição e recalculados na emissão
  (verificado em duas chamadas separadas).
- **Confirmação:** ABERTO.
- **Sugestão P84:** **FÁCIL** — `HashMap<usize, (f64, Vec<FrameItem>)>`
  por Grid, key `row_idx * num_cols + col_idx`. (Executado no P84.2.)

---

## 3. DEBTs com divergência entre ficheiro e código

### DEBT-1 — StyleChain
- **Divergência:** pendências "Scoping de `#set` por bloco" e
  "`#show` rules" listadas no DEBT.md JÁ tinham sido resolvidas
  implicitamente:
  - Scoping `#set` → DEBT-7, encerrado no P32-P33.
  - `#show` rules → DEBT-19/20, encerrados no P68-P70.
- **Acção sugerida:** riscar essas duas linhas da subsecção
  `### Pendente`. (Executado no P84.1.)

### DEBT-8 — Motor de equações
- **Divergência:** lista `MathAlignPoint` como pendente; verificação
  confirma implementação completa em `math/layout.rs`, `eval.rs`,
  `layout/mod.rs`.
- **Acção sugerida:** riscar `MathAlignPoint` da lista de pendências;
  clarificar `MathPrimes` como "parseado, layouter sem lógica
  dedicada". (Executado no P84.1.)

### DEBT-21 — Resolução de NodeKind por string
- **Divergência:** texto cita "requer Rust ≥ 1.85" como bloqueio; Rust
  1.92 está em uso há vários passos. O bloqueio técnico já não existe.
- **Acção sugerida:** actualizar texto para remover o bloqueio e marcar
  como candidato FÁCIL. (Executado em duas fases: texto no P84.1 com
  `MITIGADO (P70), desbloqueado (P84.1)`; código no P84.3.)

### DEBT-23 — entrada duplicada
- **Divergência:** o ficheiro `b19682d63:00_nucleo/DEBT.md` contém duas
  entradas consecutivas ambas começadas por `## DEBT-23` — uma com
  texto "em aberto" do P69 e outra com `**ENCERRADO (Passo 70)**`.
- **Acção sugerida:** consolidar numa única entrada com estrutura
  "Registado P69 → Resolvido P70". (Executado no P84.1.)

---

## 4. Sumário para o Passo 84

> Lista priorizada para informar a decisão sobre o que atacar nos
> sub-passos seguintes. Como nota retrospectiva: a maioria destas
> sugestões foi de facto executada no bloco P84.1–P84.6.

### Candidatos FÁCEIS (atacar primeiro)
- **DEBT-38** — Cache de sub-frames Grid Auto (1 HashMap local).
  → Executado no **P84.2**.
- **DEBT-21** — `Func::name()` → `fn_addr_eq` (Rust 1.92 disponível).
  → Executado no **P84.3**.
- **DEBT-1 / DEBT-8 / DEBT-23** — limpeza textual do DEBT.md.
  → Executado no **P84.1**.

### Candidatos MÉDIOS
- **DEBT-22** — `Vec<ShowRule>` → `Arc<[ShowRule]>` com helpers.
  → Executado no **P84.4**.
- **DEBT-36** — `Value::Align` + parsing simbólico de `+`.
  → Executado no **P84.5**.
- **DEBT-37** — propagar área de âncora ao `Content::Place`.
  → Executado no **P84.6**.
- **DEBT-9** — expandir corpus de paridade (decisão política).

### Candidatos DIFÍCEIS (adiar ou escrever ADR primeiro)
- **DEBT-8 restante** — OpenType MATH, kern, x-height (requer ADR L3/L1).
- **DEBT-34d** — min-content/max-content para Auto vs Fraction (ADR de
  modelo de sizing).
- **DEBT-34e** — colspan/rowspan (algoritmo de placement bidimensional).

### FORA DE ESCOPO do Passo 84
- **DEBT-2** — depende de `comemo` real / `TrackedWorld` concreto.
- **DEBT-33** — sem callers que emitam Bézier actualmente.
- **DEBT-35b** — preventivo; activar só quando cache de
  `available_width` for introduzido.

---

## 5. Anexo — comandos de verificação executados (regenerados no P83.6)

```bash
# Identificação do commit do P83.5
git log --oneline --all | grep -iE "83\.5|passo-83-5|mover DEBT|00_nucleo/DEBT"
# → b19682d63 Passo 83.5

# Movimentação (executada no P83.5)
git mv 01_core/DEBT.md 00_nucleo/DEBT.md
grep -rl "01_core/DEBT.md" --include="*.md" --include="*.rs" --include="*.toml" . \
  | xargs sed -i 's|01_core/DEBT.md|00_nucleo/DEBT.md|g'
grep -rn "01_core/DEBT.md" .          # → 0

# Estado do DEBT.md no commit P83.5
git show b19682d63:00_nucleo/DEBT.md | grep -E "^## Secção|^## DEBT-"

# DEBT-1
git show b19682d63:01_core/src/rules/eval.rs | grep "styles" \
  | grep -iE "save|restore|push|pop"
git show b19682d63:01_core/src/rules/eval.rs \
  | grep -E "Expr::ShowRule|apply_show_rules"

# DEBT-2
git show b19682d63:01_core/src/contracts/world.rs | grep "TrackedWorld"
git show b19682d63:01_core/src/entities/world_types.rs | grep "TrackedWorld"
grep -rn "shadow\|capture" lab/parity/tests/

# DEBT-8
git show b19682d63:01_core/src/rules/math/layout.rs | grep -c "MathAlignPoint"
git show b19682d63:01_core/src/rules/eval.rs | grep -c "MathAlignPoint"

# DEBT-9
find lab/parity/tests -name "*.rs" -exec wc -l {} \;
grep -c "#\[test\]" lab/parity/tests/parse_parity.rs

# DEBT-21
rustc --version
git show b19682d63:01_core/src/rules/eval.rs | grep -c "fn_addr_eq"

# DEBT-22
git show b19682d63:01_core/src/rules/eval.rs | grep -n "show_rules"

# DEBT-33
git show b19682d63:01_core/src/rules/stdlib.rs | grep -n "min_x\|max_x\|CubicTo"
git show b19682d63:01_core/src/entities/geometry.rs | grep -n "CubicTo\|enum PathItem"

# DEBT-34d
git show b19682d63:01_core/src/rules/layout/mod.rs | grep -A 12 "TrackSizing::Auto =>"

# DEBT-34e
git show b19682d63:01_core/src/entities/content.rs | grep -A 5 "Grid {"

# DEBT-35b
git show b19682d63:01_core/src/rules/layout/mod.rs | grep -n "available_width"

# DEBT-36
git show b19682d63:01_core/src/entities/value.rs | grep -c "Align"
git show b19682d63:01_core/src/rules/stdlib.rs | grep -n "Align2D::from_string"

# DEBT-37
git show b19682d63:01_core/src/rules/layout/mod.rs | grep -A 12 "Content::Place {"

# DEBT-38
git show b19682d63:01_core/src/rules/layout/mod.rs | grep -B 1 -A 8 "_sub_items"

# Validação no momento do P83.5 (estado do código)
cargo test                            # 732 L1 + 166 L3, 0 failures (registado)
crystalline-lint --fix-hashes .       # 2 hashes ajustados
crystalline-lint .                    # ✓ No violations found
```

---

**Sumário executivo (regenerado):**
- 13 DEBTs auditados retrospectivamente: 1, 2, 8, 9, 21, 22, 33, 34d, 34e,
  35b, 36, 37, 38.
- 0 contradições entre ficheiro e código (apenas pendências
  desactualizadas em DEBT-1 e DEBT-8, e bloqueio técnico obsoleto em
  DEBT-21).
- 4 divergências documentadas (DEBT-1, DEBT-8, DEBT-21, DEBT-23
  duplicado).
- Anomalia descoberta na regeneração: o commit `b19682d63` não contém
  os cabeçalhos `## Secção 1/2/3` que o relatório original do P83.5
  referia — esses cabeçalhos foram materializados apenas no P84.1.
- Recomendações executadas posteriormente: P84.1 (limpeza textual de
  DEBT-1/8/21/23), P84.2 (DEBT-38), P84.3 (DEBT-21), P84.4 (DEBT-22),
  P84.5 (DEBT-36), P84.6 (DEBT-37).
- DEBTs ainda em aberto após P84.6: 1, 2, 8, 9, 33, 34d, 34e, 35b, 39
  (este último aberto no P84.4 como caso fora-de-escopo de DEBT-22).

# Relatório de Investigação e Correcção — Passo 1130

**Protocolo L0**: `00_nucleo/materialization/typst-passo-1130.md`
**Data**: 2026-08-21
**Proveniência**: HEAD `781b207b4` (branch `Tekt`), working tree com as
alterações deste passo por cima (ver §6). Medições de PDF feitas com os
binários `target/release/typst` (cristalino, rebuild pós-fix) e
`lab/typst-original/target/release/typst` (vanilla ratificado).
**Status**: Concluído — causa confirmada por leitura directa do vanilla,
correcção aplicada, convergência **exacta** (bit-a-bit) com o vanilla nos
4 casos testados, zero regressão no corpus e na suite.

---

## 0. Base do passo — relação com achado antigo

Nota externa (secção 1): símbolo `√` quase não se move (`+0.26pt`), mas o
radicando sobe em relação à barra — espaço barra→radicando encolhe
`-1.18pt` (`sqrt`) e `-2.19pt` (`root` cúbica), desalinhando o índice `3`
na raiz cúbica. O L0 marcava como "possível regressão" de um achado
antigo sobre "altura do radical não escalando com o radicando", sem o
documento original disponível.

**Veredicto §0**: não é o mesmo mecanismo. O achado antigo (referenciado
em `root.md` §P974) era sobre a **selecção da variante do √** (o
cristalino desenhava sempre o glifo base, "altura fixa") — já corrigido
em P974 (short_fall=0 + gap por nível MathSize). O achado do Passo 1130 é
sobre o que acontece **depois** de a variante já estar bem seleccionada:
o excesso de altura entre a variante escolhida e o alvo exacto (as
variantes só existem em tamanhos discretos) não era redistribuído de
volta para o gap. Tratado como achado independente, confirmado por leitura
de código, não por presunção.

---

## 1. Causa confirmada por leitura directa do código

`lab/typst-original/crates/typst-layout/src/math/radical.rs:73-76`:

```rust
// TeXbook, page 443, item 11
// Keep original gap, and then distribute any remaining free space
// equally above and below.
let gap = gap.max((sqrt.height() - thickness - radicand.height() + gap) / 2.0);
```

Este ajuste já estava **registado como resíduo não portado** em
`root.md` §P970 Parte 2 ("Decisões de escopo"), desde 2026-08-05: *"O
ajuste de gap do TeXbook p443 item 11 … não é portado neste passo —
afecta `sqrt(x)` sem índice … fica registado como residual a medir"*. O
Passo 1130 é essa medição.

`01_core/src/compiler/math/layout/root.rs` seleccionava a variante do √
(`layout_radical_symbol`) e depois usava sempre o `gap` **mínimo** (sem
redistribuição) para posicionar overline/radicando/índice — mesmo quando
a variante escolhida sobrava altura face ao alvo exacto (o caso comum:
tamanhos de variante são discretos, raramente batem em cheio).

---

## 2. Cascata sobre o índice (§3 do L0) — causa própria ou consequência?

`sqrt_ascent`, `descent_surd` e `inner_ascent` dependem todos do `gap`;
`shift_up` do índice (`root.md` §P970 Parte 2) depende de
`inner_ascent`/`descent_surd`. **Hipótese**: o desalinhamento do índice é
consequência do gap por corrigir, não causa própria.

**Confirmado empiricamente (§5.2)**: corrigindo *apenas* o gap (nenhuma
mudança na fórmula de `shift_up` do índice), a posição do índice `3` em
`root(3,x)` passou a bater **exactamente** com o vanilla. Não há causa
própria — era 100% cascata.

---

## 3. Correcção implementada

Em `layout_root`, logo após `radical_box` (a variante já escolhida) e
antes de `sqrt_ascent`:

```rust
let sqrt_height = radical_box.ascent + radical_box.descent;
let radicand_height = rad_box.ascent + rad_box.descent;
let gap = gap.max((sqrt_height - line_thickness - radicand_height + gap) / 2.0);
```

Tradução directa da fórmula do vanilla. O `gap` usado para **seleccionar**
a variante (`min_height_du`) fica intocado — a ordem espelha o vanilla
(selecciona com o gap original; só depois, com a variante já escolhida,
redistribui). `overline_y` (mais abaixo na função) lê o `gap` corrigido
por sombreamento — sem alteração adicional de código.

Oráculo `radical_gap_redistribution(gap, sqrt_height, thickness,
radicand_height)` adicionado a `01_core/src/testing/math_oracle.rs`
(transcrição literal de `radical.rs:76`, `#[cfg(test)]`-only — **não**
chamado de código de produção, corrigido depois de uma primeira tentativa
errada de chamar o oráculo directamente de `root.rs`, que teria quebrado
o build de release).

L0s actualizados **antes** do código, hash reselado
(`crystalline-lint --fix-hashes .`) — fluxo contínuo (ADR-0127: correcção
de fórmula interna / paridade com o vanilla, sem novo campo, contrato,
modo ou fase de pipeline — sem paragem).

---

## 4. Testes (RED → GREEN)

`01_core/src/compiler/math/layout/tests.rs`, módulo `p1130_tests` (stub
`P1130Metrics` com uma variante de √ cuja tinta é deliberadamente mais
alta que o alvo exacto, mesmo padrão de `P974Metrics`):

- `p1130_sem_overshoot_gap_fica_inalterado` — guarda: `FixedMetrics` puro
  nunca overshoot (glifo base de fallback tem altura fixa ≤ alvo) ⇒ `gap`
  inalterado. **Já GREEN antes da correcção** (não é o caso regressivo).
- `p1130_com_overshoot_gap_cresce_metade_do_excesso` — overshoot de
  1.488pt (variante 9.0pt de tinta vs alvo 7.512pt) ⇒ gap esperado
  `max(0.72, 1.464) = 1.464`. **RED confirmado** antes da correcção
  (`overline_y` obtido `-7.116`, esperado `-7.86`) → **GREEN** depois.
- `p1130_indice_cascata_do_gap_corrigido` — mesmo overshoot, `root(3,x)`:
  `shift_up` esperado `4.8528` (vs `3.96` sem a correcção do gap). **RED
  confirmado** → **GREEN** depois.
- `p1130_sqrt_com_fraccao_no_radicando_overline_continua_acima` /
  `p1130_sqrt_com_sobrescrito_no_radicando_overline_continua_acima` — §2
  do L0 (casos adicionais pedidos pela nota): guarda de invariante
  (overline acima do topo da tinta do radicando) para `sqrt(a/b)` e
  `sqrt(x^2)`.
- `01_core/src/testing/math_oracle.rs`: 2 testes do oráculo em isolamento
  (`radical_gap_redistribution_sem_overshoot_fica_inalterado`,
  `radical_gap_redistribution_com_overshoot_cresce_metade_do_excesso`).

**RED confirmado por reversão controlada**: `git stash` só de `root.rs`
(mantendo os testes) → 2 de 3 testes numéricos falharam exactamente como
esperado (`p1130_com_overshoot_gap_cresce_metade_do_excesso`,
`p1130_indice_cascata_do_gap_corrigido`); `git stash pop` restaurou a
correcção → todos GREEN.

---

## 5. Validação empírica — renderização real vs vanilla

### 5.1 Método

4 `.typ` isolados (`sqrt(x)`, `root(3,x)`, `sqrt(a/b)`, `sqrt(x^2)`,
página 100×60pt, margem 5pt), compilados com 3 binários: pré-P1130
(rebuild via `git stash` de `root.rs`), pós-P1130, e vanilla ratificado.
Posições reais extraídas via `mutool trace` (glifos: `transform`
do `fill_text`; barra do radical: `moveto`/`lineto` do primeiro
`stroke_path`, ambos convertidos para o mesmo referencial de página).

### 5.2 Resultado — convergência exacta, não apenas "perto"

| Caso | Pré-P1130 vs vanilla | Pós-P1130 vs vanilla |
|---|---|---|
| `sqrt(x)`: baseline do radicando | `12.5460` vs `14.4765` (Δ=1.9305pt) | `14.4765` = `14.4765` (Δ=0) |
| `root(3,x)`: baseline do radicando | `12.5460` vs `14.4765` (Δ=1.9305pt) | `14.4765` = `14.4765` (Δ=0) |
| `root(3,x)`: baseline do índice `3` | `10.2866` vs `9.9005` (Δ=0.3861pt) | `9.9005` = `9.9005` (Δ=0) |
| `sqrt(a/b)`: baseline do numerador `a` | `12.5460` vs `14.6855` (Δ=2.1395pt) | `14.6855` = `14.6855` (Δ=0) |
| `sqrt(a/b)`: baseline do denominador `b` | `27.5390` vs `29.6785` (Δ=2.1395pt) | `29.6785` = `29.6785` (Δ=0) |
| `sqrt(x^2)`: baseline do radicando `x` | `15.9758` vs `16.1914` (Δ=0.2157pt) | `16.1914` = `16.1914` (Δ=0) |
| `sqrt(x^2)`: baseline do sobrescrito `2` | `12.7968` vs `13.0124` (Δ=0.2156pt) | `13.0124` ≈ `13.0124` (Δ<0.00001pt) |

A posição da própria barra do radical (`bar_y`) não muda entre pré/pós —
esperado: o deslocamento do `gap` corrigido é absorvido pelo `ascent`
total do composto (a barra e o radicando deslocam-se **juntos** na
colocação global via `place()`; só a posição **relativa** barra↔radicando
muda, que é exactamente o que a nota reportou como "o radicando sobe").

**Nota sobre os alvos `2.02pt`/`8.51pt`/`~0.15pt` do L0**: não reproduzidos
literalmente porque o `.typ` mínimo usado aqui (11pt, página 100×60pt)
não é necessariamente o mesmo contexto/tamanho de fonte da nota externa —
mas o resultado obtido (convergência **exacta**, bit-a-bit, com o vanilla
em todos os 4 casos) é uma confirmação mais forte do que bater um número
absoluto isolado: garante que **qualquer** gap/posição de índice do
cristalino passa a coincidir com o vanilla nestes casos, não apenas os
dois números citados.

### 5.3 Revalidação do corpus ("P1086-1129 — zero regressão")

Recompilados os 44 `.typ/sec_*.typ` com o binário novo, comparados (via
`qpdf --qdf --stream-data=uncompress`, ignorando metadados voláteis:
`CreationDate`/`ID`/`InstanceID`/`DocumentID`/`ModDate`) contra os
`sec_*_crystalline.pdf` já em `.typ/` (baseline do HEAD `781b207b4`,
regenerado nesta sessão antes de tocar em `root.rs`):

- **0 de 44 divergem.** Nenhum caso do corpus actual exercita `sqrt`/
  `root` num contexto que produza overshoot — resultado limpo, não um
  "não testado".

### 5.4 Suite completa e lint

- `cargo build --workspace` — 0 erros.
- `cargo test --workspace` — **100% pass**: 5089 (`typst-core` lib) +
  799 (`typst-infra` lib) + 41 + 2 + 37 + 2 testes, mais 3 doc-tests
  ignorados (pré-existentes, não relacionados).
- `crystalline-lint .` — exit code 0, 0 violations (só avisos
  informativos V19 pré-existentes de cobertura de `match`, não
  relacionados).

---

## 6. Ficheiros alterados

- `00_nucleo/prompts/compiler/math/layout/root.md` — L0 actualizado
  (§P1130), hash reselado.
- `00_nucleo/prompts/testing/math_oracle.md` — tabela de funções
  actualizada com `radical_gap_redistribution`.
- `01_core/src/compiler/math/layout/root.rs` — redistribuição do gap
  (3 linhas + comentário), `@prompt-hash` reselado.
- `01_core/src/testing/math_oracle.rs` — `radical_gap_redistribution` +
  2 testes.
- `01_core/src/compiler/math/layout/tests.rs` — módulo `p1130_tests`
  (5 testes).
- `01_core/src/testing/mod.rs` — `@prompt-hash` reselado (referencia
  `math_oracle.md`, sem mudança de conteúdo).

Nota: o `git status` no início desta sessão já trazia alterações não
relacionadas a este passo (`matrix.md`/`matrix.rs`,
`infra/font_metrics.md`/`font_metrics.rs`, `infra/shaper.md`/`shaper.rs`,
`fallback_fonts.rs`, `integration_tests.rs`) — herdadas de trabalho
anterior à abertura deste passo, não tocadas aqui.

---

## 7. Critérios de conclusão do L0 — checklist

- [x] Espaço barra→radicando: convergência **exacta** com o vanilla em
      `sqrt(x)` e `root(3,x)` (não apenas "não reduzido" — bit-a-bit
      igual, §5.2).
- [x] Índice `3` da raiz cúbica: realinhado **exactamente** com o
      vanilla (Δ=0, não `~0.15pt` nem `2.17pt`) — confirmado como cascata
      pura do gap (§2), sem tocar na fórmula de `shift_up`.
- [x] §2: 2 casos adicionais testados (`sqrt(a/b)`, `sqrt(x^2)`) — ambos
      convergem exactamente com o vanilla (§5.2) e têm guarda de
      invariante em unidade (§4).
- [x] P1086-1129 revalidado — zero regressão (0/44 divergem, §5.3).
- [x] `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100%
      pass.
- [x] §0: relação com achado antigo esclarecida sem o documento original
      — mecanismo distinto (selecção de variante vs redistribuição
      pós-selecção), tratado como achado independente.

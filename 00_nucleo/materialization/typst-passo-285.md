# Passo 285 — `FrameItem::Line` ganha `color`; emit `RG`

**Frente**: `P-line-color-rg-emit`.
**Origem dupla**:
1. P282 §1.5 — pendência `P-line-color-rg-emit` registada como
   limitação simétrica `RG` em Line.
2. P284 §5.4 — `stroke: Option<Color>` em
   `Underline`/`Strike`/`Overline` é parseado mas inerte no emit
   porque `FrameItem::Line` não tem campo `color`.

**Pré-requisitos**: nenhum bloqueador. Frente rank 1 do relatório
P284 §9.

---

## §1 — Objectivo

Adicionar campo `color: Option<Color>` (ou equivalente literal) ao
`FrameItem::Line` em `layout_types.rs` e emitir `RG` (stroke colour)
no PDF correspondente em `export.rs`.

Razão de ser dupla:

1. **Resolver pendência P282 §1.5** — `FrameItem::Shape` já suporta
   `stroke` colour via `rg`/`RG` desde Passo 102 (`text.fill`).
   `FrameItem::Line` é a única primitiva linear que ainda não.
   Limitação simétrica registada P282.
2. **Activar `stroke` inerte de P284 §5.4** — três variants
   (`Underline`/`Strike`/`Overline`) parseiam `stroke: Option<Color>`
   mas o consumer Layouter ignora-o porque o destino
   (`FrameItem::Line`) não tem onde colocá-lo.

**Sítios consumidores afectados** (inventariar exaustivamente na
Fase A.1):

- P38 — math fraction line (introduziu `FrameItem::Line`).
- P78 — Line shape primitive (`Content::Shape::Line`).
- P156I — Stack (se usa linhas separadoras; verificar).
- P227 — Grid/Table stroke borders (se usa `FrameItem::Line`
  directamente ou via `FrameItem::Shape::Line`; verificar).
- P284 — Underline/Strike/Overline (3 sítios consumer único).

Objectivo arquitectural: zero variants novos no `Content` enum.
Modificação cirúrgica em `FrameItem::Line` + emit + propagação aos
consumers existentes (default `None` → preserva comportamento
actual bit-exact onde não há colour).

Objectivo numérico:
- Hash L0 `export.rs` muda (esperado e desejado).
- Hash L0 `layout_types.rs` muda (estrutura alterada).
- Cobertura agregada: sem alteração — esta é correcção de bug
  latente, não nova feature.
- Eliminação da classe "parseado-mas-inerte": **5 sítios**
  (decorações ×3 + Line shape ×1 + futuro use ×1+ liberados).

---

## §2 — Fase A — diagnóstico empírico (obrigatória)

Três ambiguidades factuais antes de materializar:

### A.1 — Inventário de consumers de `FrameItem::Line`

Listar literalmente em
`00_nucleo/diagnosticos/diagnostico-line-color-passo-285.md`:

1. **Sítios que produzem `FrameItem::Line` actualmente** — `grep -rn
   "FrameItem::Line"` em `01_core/src/`. Para cada sítio, registar:
   - Ficheiro:linha.
   - Origem da cor (constante? campo do variant? hardcoded preto?).
   - Comportamento desejado pós-P285 (preserva preto default? lê
     campo novo?).
2. **Sítios que **consomem** `FrameItem::Line` em emit** — única
   referência esperada: `export.rs:2256-2264` (precedente Passo 38 +
   referência P284 §3.2 do relatório).
3. **Sítios que poderiam vir a usar mas usam workaround actual** —
   especialmente decorações P284 (consumer Layouter linhas
   `current_line.push(FrameItem::Line { ... thickness: ... })` —
   onde injectar o `color`?).

### A.2 — Tipo do campo: `Option<Color>` vs sentinela

Decisão de modelação (per ADR-0029 simplicidade de tipos L1):

| Opção | Prós | Contras |
|---|---|---|
| **(a)** `color: Option<Color>` — `None` significa "default (preto)" | Explícito; preserva backward-compat trivialmente (todos os call-sites actuais ficam `color: None`) | Acoplamento implícito ao default emit "preto se None" |
| **(b)** `color: Color` — sem Option; call-sites passam `Color::BLACK` explicitamente | Sem implicit-default; mais explícito | Refactor ruidoso em todos os sítios actuais (~5 chamadas, mas cosmético) |
| **(c)** `paint: Paint` — abstracção forward-looking (gradient/tiling no futuro) | Alinha com `FrameItem::Shape` (Tabela B.5 — verificar literalmente) | Sobre-engenharia: tiling/gradient em linha não existe vanilla; ADR-0054 graded desencoraja |

Default sugerido: **(a)** `Option<Color>` — minimiza touch
points, explicita ausência. **(b)** se a Fase A.1 revelar que todos
os call-sites têm cor real disponível (não há genuíno "default").
**(c)** rejeitada salvo se Fase A.1 revelar pressão concreta para
gradiente em linha (improvável dado scope vanilla).

### A.3 — Política para decorações de P284 quando `stroke = None`

Em P284 §5.4, `stroke: Option<Color>` no variant. Quando
`stroke = None` em `Content::Underline { stroke: None, ... }`,
qual o comportamento no consumer Layouter pós-P285?

| Opção | Comportamento |
|---|---|
| **(α)** `None` → emit `FrameItem::Line { color: None, ... }` → PDF emit preto (default) | Paridade vanilla (decoração herda cor do texto não-implementada; default preto é fallback aceitável) |
| **(β)** `None` → consultar `TextStyle.fill` do contexto Layouter → emit `FrameItem::Line { color: Some(text_color), ... }` | Paridade vanilla **real** — vanilla typst decoração herda cor do texto |
| **(γ)** `None` → emit `FrameItem::Line { color: None, ... }` → PDF emit preto **mas** documentar como divergência ADR-0054 graded | Compromisso: simples agora; honesto sobre divergência |

Default sugerido: **(β)** se `Layouter` já tem acesso fácil ao
`text_color` corrente (provável — P284 §2.3 mostra `font_pt`
disponível, `text_color` deve estar no mesmo contexto). **(γ)** se
o acesso não é trivial — divergência documentada é preferível a
herança aproximada.

**(α)** rejeitada salvo se Fase A.1 mostrar que vanilla também
defaulta preto (improvável — vanilla herda do texto).

---

## §3 — Materialização

Após Fase A produzir inventário + decisão tipo + decisão herança:

1. Adicionar `color: Option<Color>` (ou conforme A.2) em
   `02_layout_types/src/lib.rs` (ou caminho equivalente do tipo
   `FrameItem::Line`).
2. Actualizar todos os sítios produtores inventariados em A.1 —
   maioria passa `color: None` (backward-compat); decorações P284
   passam `color: stroke` (ou conforme A.3 herança).
3. Actualizar emit em `export.rs` —
   - Se `color = Some(Color::Rgb { r, g, b })`: emitir `r g b RG`
     antes do `S` final.
   - Se `color = None`: nada (PDF defaulta preto).
   - Reusar helper existente `emit_color_rg` se houver (verificar
     em A.1).
4. Actualizar L0 `layout_types.md` + propagar hash.
5. Actualizar L0 `export.md` (se existir secção sobre `FrameItem::Line`
   emit) + propagar hash.
6. Testes:
   - Unitário L1 em `layout_types`: `FrameItem::Line { color: Some(red), ... }`
     constrói; PartialEq distingue por color.
   - Integração L3 export: PDF para `#line(stroke: red)` contém
     `1 0 0 RG`; PDF para `#line()` **não** contém `RG` (default
     preto preserved bit-exact).
   - Integração L3 export decoração: PDF para
     `#underline(stroke: blue)[texto]` contém `0 0 1 RG` na linha.
   - Regressão bit-exact: PDFs produzidos por
     `#line()`/`$frac(a, b)$`/etc. (sem stroke explícito) preservam
     bytes exactos vs baseline P284 (validação por hash do output).

**Sem caps** (per P282 §7). Estimativa de testes: ~10-15 (delta
modesto; mudança cirúrgica).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P284: 2 716 testes.
  Esperado: ~2 726 a ~2 731.
- `crystalline-lint` zero violations.
- Hash L0 `export.rs` muda — registar novo hash no diagnóstico
  (era `bc7b8b95` desde P281; preserva-se desde P284 §5.1).
- Hash L0 `layout_types.md` muda — registar novo hash.
- **Regressão bit-exact validada** para call-sites sem cor
  explícita (call-sites com `color: None` produzem mesmo PDF byte-a-byte).
- Tabela A.7 linha 192 (`line(start, end, ...)`): manter
  `implementado` mas actualizar nota para mencionar `stroke` agora
  funcional (precedente: nota `text.fill` linha 87 com referência
  `rg`/`RG` operators).
- Diagnóstico A.1+A.2+A.3 produzido em
  `diagnostico-line-color-passo-285.md`.
- Tabela A.3 linha 103 (decorações P284): adicionar nota "stroke
  agora funcional (P285)".
- P282 §1.5 pendência marcada como RESOLVIDA.
- P284 §5.4 marcada como RESOLVIDA.

---

## §5 — Não-objectivos

- **Não** adicionar `thickness: Option<Length>` em `FrameItem::Line`
  como override per-instance. Thickness é calculada pelo Layouter
  (P284 §2.3: `thickness: (font_pt * 0.05).max(0.4)`). Mudar isto é
  passo distinto (`P-stroke-thickness-override`).
- **Não** materializar objecto `Stroke` rico vanilla
  (`Stroke { paint, thickness, cap, dash, miter_limit }`). Tabela
  A.7 linha 201 lista como `parcial`. Passo dedicado futuro per
  ADR-0054 graded.
- **Não** adicionar suporte a `Paint::Gradient` em `FrameItem::Line`.
  Gradiente em linha não existe vanilla typst. Scope-out per
  ADR-0054 graded.
- **Não** alterar `FrameItem::Shape::Line` (variant de `Shape`,
  distinto de `FrameItem::Line` top-level). Se Fase A.1 revelar
  ambiguidade entre os dois, registar mas não unificar — passo
  distinto.
- **Não** mexer no consumer de Underline/Strike/Overline para
  herança vanilla-fiel além do que A.3 decidir. Se A.3 → (γ), a
  divergência fica documentada; ressuscitar paridade real é passo
  futuro condicional.

---

## §6 — Pendências relacionadas (resolução parcial)

- **P282 §1.5** `P-line-color-rg-emit` — esta spec é a resolução
  literal.
- **P284 §5.4** `stroke parseado mas inerte` — esta spec resolve
  para Underline/Strike/Overline (se A.3 → (β) ou (γ)).

Pendências **não** resolvidas por este passo:

- `P-text-deco-multiline` (P284 §5.3) — sub-passo P284.1; multi-line
  wrap-aware emission. Ortogonal a esta spec.
- Objecto `Stroke` rico (Tabela A.7 linha 201) — scope-out distinto.

---

## §7 — Risco residual

Risco principal: violação de bit-exactness em call-sites
"backward-compat" (color = None). Se a ordem dos operadores PDF
mudar (e.g. RG antes vs depois de outros operadores no mesmo
state), os bytes diferem mesmo quando a cor é a default.

Mitigação: estrutura do emit é condicional `if let Some(color) = ...`
**antes** do bloco existente `q w m l S Q`. Quando `color = None`,
o ramo `if` não dispara — bytes preservados literalmente. Validação:
hash SHA-256 dos PDFs de teste P38 (math frac) + P78 (line shape) +
P227 (grid borders, se aplicável) **antes** e **depois**; devem
coincidir nos casos sem stroke.

Risco secundário: a Fase A.1 pode revelar que `FrameItem::Line` é
produzido por mais sítios do que o esperado (5 enumerados acima),
forçando refactor maior. Mitigação: per ADR-0065, inventário-primeiro
exaustivo via `grep` — se aparecerem >10 sítios, considerar abrir
sub-passo P285.1 para refactor de helper produtor centralizado.

Risco terciário: decisão A.3 → (β) (herança real do `text_color`)
pode revelar que o Layouter não expõe `text_color` no contexto
onde a decoração é emitida (paralelo invertido do risco §7 de P284
que se mostrou não-bloqueante). Mitigação: A.1 inclui inspecção do
Layouter pós-P284 + verificação da disponibilidade de `text_color`.

---

## §8 — Ponteiros

- Tipo a modificar: `02_layout_types/src/lib.rs` (`FrameItem::Line`).
- Emit: `03_infra/src/export.rs:2256-2264` (referência P284 §3.2).
- Consumer decorações: P284 §2.3 (`layout/mod.rs:1974`).
- Precedente arquitectural mais próximo: Passo 102 (`text.fill` —
  introduziu `rg`/`RG` em `FrameItem::Text`). Esta spec é a
  aplicação simétrica do mesmo padrão a `FrameItem::Line`.
- ADR processual: ADR-0065 (inventariar-primeiro — A.1
  exaustivo).
- ADR scope: ADR-0054 graded (justifica não-objectivos §5).
- ADR pureza física: ADR-0029 (justifica decisão A.2 favorecer
  `Option<Color>` simples).
- Pendências fontes: P282 §1.5; P284 §5.4 do relatório.

---

*Spec P285 produzida 2026-05-18 pós-P284 (Text decorations fechadas
com 3 variants). Frente `P-line-color-rg-emit` — resolução literal
da pendência P282 §1.5 + activação do `stroke` inerte de P284 §5.4.
Modificação cirúrgica `FrameItem::Line` + emit `RG`; zero variants
novos. Fase A obrigatória (inventário consumers, tipo Option vs
direct, política herança em decorações). Critério de fecho inclui
validação bit-exact para call-sites backward-compat. Hash
`export.rs bc7b8b95` muda intencionalmente (era preservado desde
P281; quebra esperada). Sem caps LOC ou magnitude (P282 §7).*

# Passo 284 — `underline` + `strike` + `overline`

**Frente**: `P-text-deco-emit`.
**Origem**: cobertura A.3 linha 103 (ausente, sem referência) + C linha 383
(escopo S, bloqueador único = variants `Content::Underline`/`Strike`/`Overline`).
**Pré-requisitos**: nenhum — bloqueador único é a própria materialização do
variant; sem dependências em rustybuzz/regex/locate-runtime/ADR-0017.

---

## §1 — Objectivo

Adicionar as três funções vanilla de decoração textual ao cristalino:
`underline(body)`, `strike(body)`, `overline(body)`. Materializar as três
em conjunto porque partilham:

- mesmo ficheiro vanilla (`text/deco.rs`).
- mesma estrutura de variant (envolve `body: Content`; parâmetros
  cosméticos opcionais).
- mesmo mecanismo de emissão PDF (linha horizontal sobreposta/sublinhar
  glifos já emitidos).

Estado actual:

- Tabela A.3 linha 103: `underline` / `strike` / `overline` → `ausente`,
  referência `—`.
- Tabela B.2 `**Vanilla-only (ausentes)**` linha 327: implícito via reticências;
  variants ausentes.
- Tabela C linha 383: bloqueante = `Content::Underline` etc. ausentes;
  ADR/DEBT/Próximo passo = "escopo S".

Objectivo numérico: Text features (categoria Tabela A linha 434) sobe
de 12/23 (52%, impl+impl⁺) para 15/23 (65%) — três linhas `ausente` →
`implementado` numa só passada.

Objectivo arquitectural:

- 3 variants novas em `Content` (precedente directo: `Quote` em P155,
  `Block` em P156G — variant rico com `body` + atributos opcionais).
- 3 funções `native_*` registadas em `make_stdlib`.
- 1 ou 3 helpers de emit em export.rs (decisão arquitectural não-trivial;
  ver §2.A.2 — Fase A obrigatória per ADR-0065).

---

## §2 — Fase A — diagnóstico empírico (obrigatória)

Três ambiguidades factuais antes de materializar:

### A.1 — Scope dos atributos vanilla

Vanilla `text/deco.rs` expõe três funções com **conjuntos de atributos
parcialmente sobrepostos**. Listar literalmente em
`00_nucleo/diagnosticos/diagnostico-deco-passo-284.md`:

1. `underline(body, stroke?, offset?, extent?, evade?, background?)`.
2. `strike(body, stroke?, offset?, extent?, background?)` — `evade` não
   se aplica.
3. `overline(body, stroke?, offset?, extent?, evade?, background?)`.

Decidir buckets:

| Bucket | Critério |
|---|---|
| **Materializar** | atributos cujo tipo já existe em L1 (`Length` para offset/extent; `bool` para evade/background; Color para stroke simples per ADR-0028→0029) |
| **scope-out ADR-0054 graded** | `stroke` como objecto rico (`Stroke{ paint, thickness, cap, dash, ... }`) — Tabela A.7 linha 201 lista `stroke(...)` (object) como `parcial`; não é deste passo resolver |
| **scope-out semântico** | `evade` (desviar descendentes) requer cálculo geométrico glifo-a-glifo; **scope-out** com nota cruzada para passo futuro (precedente: `extent` cosmético) |

### A.2 — Helper único vs três helpers em export.rs

Decisão arquitectural não-trivial (per ADR-0065 critério #4 — atravessa
camadas L1 variant + L3 emit):

| Opção | Prós | Contras |
|---|---|---|
| **(a)** Três funções separadas `emit_underline_pdf`, `emit_strike_pdf`, `emit_overline_pdf` | Simétrico aos variants; cada emit lê os seus campos directamente | Replicação ~80% (única diferença: posição Y da linha em coordenadas locais do glifo) |
| **(b)** Um helper único `emit_deco_pdf(kind: DecoKind, body, ...)` + enum interno L3 com 3 variantes | Single source of truth (precedente P281 PageContext + FontScenario) | Acopla as três decisões num único sítio; se uma divergir (e.g. `strike` ganhar `evade: false` hardcoded), helper passa a ter ramos |
| **(c)** Helper `emit_line_over_text(y_offset_em: f64, body, ...)` parametrizado por offset Y em em-units; cada native_* passa o seu offset | Captura a única diferença material entre os três; sem enum extra | Exige descoberta empírica dos offset_em vanilla durante materialização |

Default sugerido: **(c)** se a inspecção vanilla revelar três offsets como
constantes simples (precedente: P281 mostrou que paramatrizar pelo eixo
de variação reduz LOC). **(a)** se cada um tem lógica não-trivial específica.
**(b)** se aparece um campo discriminador natural.

### A.3 — Naming dos variants em Rust

Decisão de naming per ADR-0065 critério #1 (colisão de convenções):

- `Content::Underline` — sem colisão.
- `Content::Strike` — sem colisão (stdlib Rust não tem `Strike`).
- `Content::Overline` — sem colisão.

Mas atributos partilhados sugerem uma alternativa:

- **(α)** Três variants distintos: `Underline { body, offset, ... }`,
  `Strike { body, offset, ... }`, `Overline { body, offset, ... }`.
- **(β)** Um variant tagged: `Decoration { kind: DecoKind, body, offset, ... }`
  com `enum DecoKind { Under, Strike, Over }`.

Precedente vanilla: três `Elem` separados (UnderlineElem, StrikeElem,
OverlineElem). Precedente cristalino: `Shape { kind: ShapeKind, ... }`
unifica Rect/Ellipse/Line/Path/Polygon (Tabela B.2 linha 311). **Mas**
para Pad/Block/Boxed/Stack (P156C/G/H/I) o padrão escolhido foi
variants separados.

Default sugerido: **(α)** três variants distintos — coerente com o
padrão Layout Fase 2 mais recente (P156G/H/I) e com vanilla. **(β)**
só se atributos forem 100% idênticos e código de emit colapsar
naturalmente para tabela-driven (improvável dado A.2 cenário (c)).

---

## §3 — Materialização

Após Fase A produzir buckets + decisão helper + naming:

1. Adicionar variants em `01_core/src/entities/content.rs` (ou
   módulo equivalente pós-ADR-0037).
2. Adicionar funções `native_underline`, `native_strike`,
   `native_overline` em stdlib (módulo text).
3. Registar em `make_stdlib`.
4. Adicionar consumer no Layouter — emite os glifos do `body` como
   habitual + regista decoração na frame (campo novo em
   `FrameItem::Text` ou novo `FrameItem::TextDecoration`; decisão na
   Fase A.2).
5. Adicionar emit em export.rs (PDF operadores `q w m l S Q` para
   linha; precedente directo: `FrameItem::Line` em `text.tracking`
   passo 137).
6. Testes:
   - Unitários L1 por variant: construção, PartialEq, `plain_text`
     delega no body.
   - Integração L3 (cargo test workspace): renderiza
     `#underline[hello]` produz PDF com operadores `q ... S Q` na
     posição esperada (Y = baseline + offset_default).
   - Cross-check: `#strike[$\sum_{i=1}^n i$]` — math em decoração
     mantém layout interno.
7. Actualizar L0 (`00_nucleo/prompts/rules/...` — caminho depende de
   onde stdlib text vive pós-ADR-0037) com tabela dos três variants +
   propagar hash via `crystalline-lint --fix-hashes`.
8. Actualizar Tabela A.3 linha 103: `ausente` → `implementado` com
   referência `Passo 284`.
9. Actualizar Tabela B.2: adicionar 3 variants entre as existentes.
10. Actualizar Tabela C linha 383: remover (resolvida); ou marcar
    com `~~strikethrough~~` como em linha 402 (precedente `repeat`).

**Sem caps** (per P282 §7). Estimativa numérica de testes: ~20-35
(3 variants × 2-3 unitários L1 + 5-10 integração L3). Reformula-se se
Fase A revelar scope diferente.

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline desconhecida pós-P283;
  delta esperado +20 a +35.
- `crystalline-lint` zero violations.
- Hashes L0 propagados (text stdlib + content variants se aplicável).
- Tabela A.3 actualizada (linha 103 com referência P284).
- Tabela B.2 actualizada (+3 variants).
- Tabela C linha 383 marcada resolvida.
- Tabela A resumo (linha 434): Text passa de 7/5/1/8/2 para 10/5/1/5/2
  (= 23; `ausente` desce 3, `implementado` sobe 3).
- Total user-facing (linha 441 + notas cumulativas): `+3 implementado,
  -3 ausente`.
- Diagnóstico A.1+A.2+A.3 produzido em
  `diagnostico-deco-passo-284.md`.

---

## §5 — Não-objectivos

Explicitar para evitar deriva:

- **Não** materializar objecto `Stroke` rico vanilla
  (`Stroke { paint, thickness, cap, dash, miter_limit }`). Tabela A.7
  linha 201 lista `stroke(...)` (object) como `parcial`; passo próprio
  resolve. Neste passo: aceitar `stroke: Option<Color>` ou similar
  simples (decisão Fase A.1 bucket scope-out).
- **Não** implementar `evade` (descender skipping). Geometria glifo
  por glifo; passo dedicado futuro per ADR-0054 graded.
- **Não** materializar `background` se vanilla o expuser — atributo
  cosmético com baixo valor visível; scope-out graded.
- **Não** tocar em `text.script` (super/sub standalone — Tabela A.3
  linha 98). É feature distinta da decoração.
- **Não** mexer em hash L0 `export.rs` `bc7b8b95` salvo o
  estritamente necessário para emit das três decorações. Se decisão
  A.2 → (c), `emit_line_over_text` é função nova; hash export.rs
  muda mas o resto preserva-se bit-exact.

---

## §6 — Pendências relacionadas (não-objectivos)

`P-line-color-rg-emit` (pendência P282 §1.5) — emit `RG` em Line.
Não incluir. Continua aberta como passo separado; toca o mesmo
ficheiro export.rs mas área diferente (paint operators em
`FrameItem::Line`, não em decorações textuais novas).

---

## §7 — Risco residual

Risco principal: decisão A.2 pode revelar acoplamento não-trivial
entre o consumer do Layouter (a decoração precisa saber as métricas
de texto pós-shaping para posicionar a linha) e a infraestrutura
actual. Se a inspecção empírica mostrar que `FrameItem::Text` não
expõe as métricas necessárias (ascent, descent, x-advance acumulado),
abrir sub-passo P284.1 dedicado a expor estas métricas antes de
materializar as decorações.

Mitigação: a Fase A inclui leitura empírica do consumer Layouter
actual antes de decidir A.2 — se as métricas não estiverem
disponíveis, a Fase A regista o gap e P284.1 surge naturalmente.
Per ADR-0065, decisão atravessa camadas (L1 variant + L1 Layouter
consumer + L3 export emit) — inventário-primeiro é obrigatório.

Risco secundário: P156G/H/I estabeleceram patamar N≥3 de variants
ricos novos. Adicionar mais 3 pode disparar pressão para promover o
padrão a ADR meta (precedente: ADR-0065 emergiu de N=5 aplicações).
Esta promoção **não** é deste passo — registar em diagnóstico A se
o patamar atingir limiar explícito.

---

## §8 — Ponteiros

- Vanilla: `lab/typst-original/crates/typst-library/src/text/deco.rs`.
- Cobertura: Tabela A.3 linha 103 (estado actual); B.2 linha 327
  (lista vanilla-only); C linha 383 (vista cruzada).
- Precedente arquitectural mais próximo: P156G/H/I (variant rico
  com atributos opcionais; subset Fase 1 per ADR-0054 graded).
- Precedente de emit linear PDF: `FrameItem::Line` (passo 38 math
  fracs + passo 78 line shape) — operadores `q w m l S Q`.
- Precedente cosmético com Color simplificado: ADR-0028→0029
  (`Length::Pt`/`Em`; Color RGB/RGBA simples).
- ADR processual: ADR-0065 (inventariar-primeiro — A.1+A.2+A.3
  cobrem 3 critérios distintos: scope, atravessamento camadas,
  naming).
- ADR scope graded: ADR-0054 (atributos cosméticos adiados com
  registro explícito).

---

*Spec P284 produzida 2026-05-18 pós-P283 (calc trig fechado, ver
linha 212 do ficheiro de cobertura). Frente `P-text-deco-emit` —
identificada na Tabela C linha 383 como escopo S sem bloqueador
arquitectural. Fase A obrigatória (3 ambiguidades: scope atributos,
helper único vs três, naming variant). Sem caps LOC ou magnitude
(P282 §7). Materialização condicional a inspecção empírica de
métricas de texto disponíveis no Layouter actual.*

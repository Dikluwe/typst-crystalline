# Diagnóstico — Fase A do Passo 287 (`P-smartquote`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-287.md`
**Origem**: Tabela C linha 380 — `smartquote` (função stdlib)
ausente; bloqueador único `Content::SmartQuote`.

---

## A.1 — Inventário da infraestrutura P155 existente

### A.1.1 — `01_core/src/rules/lang/quotes.rs`

| Símbolo | Localização | Notas |
|---|---|---|
| `LANG_QUOTES: &[(&str, (&str, &str))]` | linha 24 | Tabela estática 6 idiomas — pt/en/de/fr/es/it |
| `DEFAULT_QUOTES: (&str, &str) = ("\"", "\"")` | linha 35 | ASCII fallback |
| `pub fn localize_quotes(lang: &Lang) -> (&'static str, &'static str)` | linha 41 | Resolve par `(open, close)` por exact match ISO 639-1/2/3 |

API é puramente lookup — **sem state**. Cada chamada é independente.

### A.1.2 — Alternância open/close em `eval_markup`

`01_core/src/rules/eval/mod.rs:269-311`:

```rust
let mut double_open = true; // true = próximo `"` é open
let mut single_open = true;

for child in node.children() {
    match child.kind() {
        SyntaxKind::SmartQuote => {
            let is_double = raw.as_str() == "\"";
            let (open, close) = localize_quotes(&lang).../* ou DEFAULT_QUOTES */;
            let glyph: &str = if is_double {
                let g = if double_open { open } else { close };
                double_open = !double_open;
                g
            } else {
                single_open = !single_open;
                "'"  // aspas simples: always ASCII, scope-out smart-apostrophes
            };
            let quote_node = Content::Text(glyph.into(), style);
            parts.push(rules::intercept_content(quote_node, ctx, engine)?);
        }
        /* ... */
    }
}
```

### A.1.3 — Estado actual: scope e persistência

| Pergunta | Resposta empírica |
|---|---|
| Onde reside o estado open/close? | **Local ao `eval_markup`** (variables `double_open`/`single_open` em stack frame) |
| Persistência? | **Per-`eval_markup` invocation** — uma chamada = um parseamento de markup; nova invocação reseta |
| Per-document, per-paragraph, per-runtime? | **Per-parseamento de bloco markup**. Não há state cross-block nem cross-document |
| Consultável de fora? | **Não** — é estado local de função. Layouter não o vê |
| `text.smartquotes` (atributo `set text`)? | **Não existe em cristalino** — vanilla tem `text.smartquotes: bool` para desligar; cristalino diverge silenciosamente (sempre activo). Registar divergência |
| `quotes` (custom override do utilizador)? | **Não existe** — vanilla aceita `set smartquote(quotes: ...)` com custom string/array/dict; cristalino diverge (apenas lang-based) |

### A.1.4 — Vanilla `text/smartquote.rs:35-89`

`SmartQuoteElem` tem 4 atributos:
- `double: bool` (default `true`)
- `enabled: bool` (default `true`)
- `alternative: bool` (default `false`)
- `quotes: Smart<SmartQuoteDict>` (custom override; complexo)

E há um `SmartQuoter { depth: u8, ... }` runtime que rastreia estado.

### A.1.5 — Tabela A.3 da cobertura (apresentação)

Linha 60 já existe para markup `"foo"` (P155 `implementado`).
Linha 380 da Tabela C lista `smartquote` (função) como ausente.

**Decisão de apresentação**: vou estender a linha 60 existente com
nota cruzada P287 (em vez de criar linha nova) — markup e função
partilham a mesma feature conceptual (smart quotes). Mantém
Tabela A.3 simples; total Text features inalterado (10/5/1/5/2 = 23).

---

## A.2 — Estrutura do variant `Content::SmartQuote`

### A.2.1 — Análise crítica das opções

| Opção | Estrutura | Veredicto |
|---|---|---|
| (a) `{ double: bool }` | mínimo vanilla | ✅ **Escolhida** — leaf-like, simétrico vanilla signature; estado open/close fica externo |
| (b) `{ double, opening }` | estado explícito | ❌ Forçaria parser a calcular `opening` em construção; acoplamento desnecessário |
| (c) `{ double, alternative }` | extra cosmético | ❌ `alternative` é scope-out per spec §5 (ADR-0054 graded) |
| (d) `{ double, enabled, alternative }` | paridade 1:1 | ❌ 3 atributos para variant leaf-like; over-engineering |

### A.2.2 — Importante: variant é **leaf**, não container rico

`Content::SmartQuote` é **leaf-like** — análogo a `Content::Space`,
`Content::Linebreak`, `Content::MathAlignPoint`. **Não tem `body:
Content`** porque a "aspa" é 1 glyph, não um container que envolve
texto.

**Consequência arquitectural**: o variant **não qualifica** como
"variant rico com `body` + cosméticos opcionais" (padrão N=4
cumulativo P156G/H/I+P284). Por isso **N=5 NÃO é atingido por
este passo** — `SmartQuote { double: bool }` é leaf com 1 campo
`bool` required, sem cosméticos opcionais.

Honestidade epistémica registada — o relatório P287 deve documentar
isto explicitamente para que futuros passos não contem incorrectamente
N=5 acidental.

### A.2.3 — Sintaxe final

```rust
Content::SmartQuote {
    double: bool,  // true = "double" (aspas duplas); false = 'single'
}
```

`enabled` e `alternative` resolvidos na função stdlib (não no
variant) — ver §A.3.

---

## A.3 — Estado open/close: onde reside?

### A.3.1 — Decisão

**Decidido**: opção **(γ′)** — refinamento minimalista de (γ).

| Componente | Estratégia |
|---|---|
| Markup `"foo"` (caminho P155) | **Inalterado** — `eval_markup` continua a pré-resolver glyph via `localize_quotes` + variables locais. Bit-exact preservado |
| Função stdlib `#smartquote(...)` | Emite `Content::SmartQuote { double }` (variant novo) |
| Consumer Layouter | Mantém state **próprio independente** (`smartquote_double_open: bool`, `smartquote_single_open: bool`) — estado per-document ao Layouter; consulta `localize_quotes` para obter par; emite `FrameItem::Text` com glyph resolvido + alterna state |

### A.3.2 — Divergência aceite vs vanilla

Cristalino tem **2 estados independentes** para alternância:

1. **Markup parser** (`eval_markup` local) — alterna durante
   parseamento de bloco markup.
2. **Layouter consumer** (`smartquote_*_open` no Layouter) —
   alterna durante walk de `Content::SmartQuote` emitidos por
   `native_smartquote`.

Caso edge: `"foo" #smartquote() bar"` mistura markup + função na
mesma sequência. Vanilla mantém estado partilhado (`SmartQuoter`
único). Cristalino terá `"foo"` → markup state; `#smartquote()` →
Layouter state. Os dois open/close vão começar em `open` cada um —
pode produzir 2 quotes "open" consecutivos visualmente.

**Justificação**: caso edge raro (mistura programática + markup
literal); ADR-0054 graded justifica divergência aproximada; refactor
para unificar estados é passo futuro condicional. Registado em
diagnóstico e nota L0.

### A.3.3 — Porquê (γ′) sobre (β)

(β) full refactor consolidaria os 2 estados num só — mas exigiria
re-trabalhar `eval_markup` (~30 LOC) e introduzir interacção
parser↔Layouter. Custo desproporcional ao caso edge resolvido.

(γ′) preserva bit-exact estricto do markup P155 (critério §4
duro) e adiciona feature mínima para casos novos. Aplicação directa
do padrão "activação posterior de feature graded" (P285§8.3, P286§8.1
— N=2 cumulativo até P286; **P287 leva a N=3** porque adiciona
feature stdlib que estava ausente, sem perturbar markup existente).

---

## A.4 — Política do glyph emitido

### A.4.1 — Decisão

**Decidido**: opção **(i)** — reusar `rules/lang/quotes.rs::localize_quotes`
no consumer Layouter.

| Sinal | Decisão |
|---|---|
| Single source of truth para `(lang, double) → glyph` | (i) reusa P155 |
| Padrão "Win arquitectural single source of truth" (P282+P285+P286 = N=3 limiar atingido) | (i) **cita o padrão**, **N=4 cumulativo** após P287 |
| Show rules sobre `SmartQuote` (motivo de opção ii) | ⏸ Scope-out — `#show smartquote: ...` é passo distinto |

### A.4.2 — Interacção com gatilhos ADR meta

P287 cita o padrão "single source of truth" (N=3 → **N=4
cumulativo**). Não dispara nada — limiar histórico ADR-0065 cita
N=5 (padrão "variant rico" N=4 pré-P287, **NÃO** atinge N=5 aqui
por causa da decisão A.2 leaf — ver §A.2.2).

Para evitar confusão: **dois padrões diferentes** com contadores
independentes:
- "Variant rico com `body` + cosméticos opcionais" — N=4 cumulativo
  pós-P287 (sem mudança porque SmartQuote é leaf).
- "Win arquitectural single source of truth como invariante
  anti-bug" — N=3 → **N=4** após P287 (consumer reusa `localize_quotes`).

Promoção ADR meta de qualquer um **NÃO é objectivo deste passo**
(per spec §5 não-objectivo + §7 risco quaternário).

---

## §Métricas do impacto

| Métrica | Antes | Pós-P287 |
|---|---:|---:|
| `Content` variants | 63 | **64** (+SmartQuote leaf) |
| Funções stdlib | 91 | **92** (+native_smartquote) |
| `Layouter` campos | N+1 (pós-P286) | **N+3** (+`smartquote_double_open` +`smartquote_single_open`) |
| Hash L0 `content.md` | actual | **muda** (+1 variant) |
| Hash L0 `stdlib.md` | actual | **muda** (+1 função) |
| Hash L0 `lang.md` | actual | **preservado** (A.3 → γ′ não toca P155) |
| Hash L0 `export.rs` | `66cb8ac3` (P285) | **preservado** (A.4 → (i) reusa Text emit existente) |
| Markup `"..."` bit-exact | n/a | **Preservado** (critério §4 duro; A.3 → γ′ não altera eval_markup) |
| Padrão "variant rico" N | 4 cumulativo | **4 cumulativo** (SmartQuote leaf não qualifica — §A.2.2) |
| Padrão "single source of truth" N | 3 (P286 limiar) | **4 cumulativo** (consumer reusa localize_quotes) |

---

## §Risco residual mitigado

- **Risco principal** (§7 spec): bit-exactness markup `"..."`.
  ✅ **Mitigado** — A.3 → γ′ preserva `eval_markup` literalmente;
  zero alteração no caminho P155.
- **Risco secundário** (A.4 → ii cria divergência): ✅ refutado por
  decisão A.4 → (i) — single source of truth via `localize_quotes`.
- **Risco terciário** (estado open/close persistence): ✅ resolvido
  explicitamente — Layouter state é **per-document** (campo no
  Layouter struct, sobrevive entre `layout_content` calls;
  reinicializado em `Layouter::new`).
- **Risco quaternário** (gatilho N=5 acidental): ✅ refutado por
  §A.2.2 — SmartQuote leaf NÃO qualifica como variant rico;
  contador permanece N=4.

---

## §Fecho da Fase A

Inventário P155 literal (4 entidades + 6 idiomas tabela) + decisão
estrutura variant (a leaf-like) + decisão estado open/close (γ′
preserva markup bit-exact) + decisão glyph emit (i single source
of truth) registadas. Variant **não qualifica** como "variant rico"
— honestidade epistémica registada explicitamente. Material
suficiente para materialização. Procede-se a §3 do passo.

**Decisão emergente**: opção (a) refinada para "leaf-like com 1
campo bool" + opção (γ′) refinada de (γ) — ambas mais minimalistas
que sugerido pela spec. Padrão P285§8.3 "refutação pragmática de
pressuposto da spec" replicado aqui em A.2 e A.3 (N=3 cumulativo
do mesmo padrão).

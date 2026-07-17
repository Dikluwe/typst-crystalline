# Relatório — Passo 295 (`P-footnote-cluster` Fase 1)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-295.md`
**Diagnóstico Fase A**: `00_nucleo/diagnosticos/diagnostico-footnote-cluster-passo-295.md`
**Tipo declarado spec**: Cluster M ortogonal genuíno (1º pós-série
cumulativa cirúrgica P288-P294). Magnitude reset.
**Hipótese adoptada**: **HE Fase 1 (marker only)** com **A.2 → (a)**
variant minimal `Footnote { body: Box<Content> }`.
**Baseline P294**: 2 808 testes  →  **P295**: 2 817 testes (Δ = +9)
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**12º passo
consecutivo**: P282→P295)
**Hash `content.rs`**: `4861affa` → `2fc32a66` (mudança esperada — +1 variant)
**ADRs meta novas**: 0

---

## §1 — Sumário executivo

P295 materializa `Content::Footnote` como variant de primeira classe
no `Content` enum, com stdlib `native_footnote(body)` e consumer
Layouter que emite **marker `[N]` superscript inline** via walker
counter simples (`Layouter::footnote_counter: u32`).

**Decisão arquitectural HE Fase 1 marker only**: body é **armazenado
estructuralmente mas não renderizado** no rodapé nesta fase. Os
sub-passos **P295.1** (nota rodapé via 2-pass layout) e **P295.2**
(overflow multi-página) ficam registados como frentes pendentes.

**Resultado funcional**: `#footnote[corpo]` em Typst produz output
PDF com marker `[1]`, `[2]`, `[3]`... inline.

**Resultado metodológico — refutação factual de bloqueador histórico**:
A Fase A reaplicou A.0.0 (N=3 do template §8.7' inaugurado em P293,
reaplicado em P294) e confirmou via inspecção literal que **Tabela C
linha 387 está factualmente desactualizada**: o bloqueador duplo
`Content::Footnote` + `locate runtime` ficou reduzido ao primeiro
elemento, porque P208B/C já materializaram `locate`/`here`/`query`.
Refutação factual modesta vs P293 (H6 não-listada) ou P294 (spec
inteira invalidada) — registada como **§8.3 N=7 candidato adiado**.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=3 reaplica §8.7') | Refutação factual de Tabela C linha 387 (bloqueador `locate` já implementado P208B+C) |
| A.0 (ADR-0098 hash) | ✅ preservado bit-exact — emit agnóstico via `FrameItem::Text` |
| A.1 inventário | `Content::Footnote` confirmado ausente; precedente arquitectural = `Cite` (P159A) |
| A.2 decisão | **(a) minimal** — `body` only; sem `numbering` cosmético; padrão "variant rico" N=4 preservado |
| A.3 numeração | Walker counter simples no Layouter (sem Counter/Introspector machinery) |
| A.4 Layouter | **Fase 1 only** — marker `[N]`; body armazenado mas não renderizado |
| A.5 bugs latentes | 6 cenários verificados; nenhum bug |
| A.5' anti-reflexão | N=5 cumulativo (P291+P292+P293+P294+P295); 5 elementos novos |

Detalhe completo: `00_nucleo/diagnosticos/diagnostico-footnote-cluster-passo-295.md`.

---

## §3 — Materialização

### §3.1 — `01_core/src/entities/content.rs` (+1 variant)

```rust
// ── Passo 295 — `Footnote` cluster Fase 1 (marker only) ─────────────
/// Footnote inline — vanilla `FootnoteElem`.
///
/// **Fase 1 P295 (HE marker only)**: variant minimal com `body`
/// armazenado mas **não renderizado** no rodapé nesta fase. Layouter
/// emite apenas marker `[N]` superscript inline onde a footnote
/// aparece. Numeração via walker counter simples (sem
/// Introspector/Counter machinery).
///
/// Cristalino simplifications per ADR-0054 graded vs vanilla:
/// - `numbering: Numbering` (default `"1"`) **scope-out** (cosmético;
///   numeração arábica default implícita).
/// - `FootnoteBody::Reference(Label)` **scope-out** (multi-ref
///   footnotes — frente futura P295.X).
Footnote {
    body: Box<Content>,
},
```

### §3.2 — Match arms exhaustive (defesa compilador, 8 sítios)

| Local | Operação |
|---|---|
| `content.rs:is_empty()` | `false` (marker sempre observable) |
| `content.rs:plain_text()` | recursa em `body.plain_text()` |
| `content.rs:PartialEq` | `body == body` |
| `content.rs:map_content()` | recursa em body |
| `content.rs:map_text()` | recursa em body |
| `rules/introspect.rs:materialize_time` | recursa em body |
| `rules/introspect.rs:walk` | walk em body |
| `rules/introspect/locatable.rs:is_locatable` | `false` (Fase 1 não-locatable) |

Match exaustivo identificou todos os sítios via compiler errors —
paradigma robusto de defesa cumulativa.

### §3.3 — `01_core/src/engine/stdlib/structural.rs` (+`native_footnote`)

```rust
/// `footnote(body)` — emite `Content::Footnote { body }`. Body
/// posicional obrigatório (content ou string).
pub fn native_footnote(_ctx, args, _world, _current_file, _figure_numbering)
    -> SourceResult<Value>
{
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(...),
        None => return Err("footnote() exige body como argumento posicional"),
    };

    // Validar ausência de named args (P295 Fase 1: sem cosméticos).
    for k in args.named.keys() {
        return Err(format!(
            "footnote(): argumento nomeado '{}' não suportado em P295 \
             Fase 1 (numbering/cosméticos scope-out per ADR-0054 graded)", k));
    }

    Ok(Value::Content(Content::Footnote { body: Box::new(body) }))
}
```

### §3.4 — `01_core/src/engine/stdlib/mod.rs` + `eval/mod.rs`

- `mod.rs:45`: `native_footnote` adicionado ao re-export.
- `eval/mod.rs:556`: import adicionado.
- `eval/mod.rs:683+1`: `scope.define("footnote", ...)` registado.

### §3.5 — `01_core/src/engine/layout/mod.rs` (counter + arm)

**(a) Field no struct `Layouter`**:

```rust
/// **P295 (Footnote Fase 1)** — counter monotónico incrementado em
/// cada `Content::Footnote` consumido. Marker `[N]` emitido como
/// superscript inline. Walker counter simples (sem
/// Counter/Introspector machinery) — magnitude reduzida para Fase 1.
pub(super) footnote_counter: u32,
```

**(b) Inicializado em `Layouter::new`**: `footnote_counter: 0`.

**(c) Consumer arm em `layout_content`**:

```rust
// P295 — Footnote Fase 1 (marker only). Walker counter simples;
// marker `[N]` emitido inline como `Content::text`. Body armazenado
// mas **não renderizado** no rodapé nesta fase — sub-passos P295.1
// (nota rodapé) + P295.2 (overflow) renderizam via 2-pass layout
// futuro.
Content::Footnote { body: _ } => {
    self.footnote_counter += 1;
    let n = self.footnote_counter;
    let marker = format!("[{}]", n);
    self.layout_content(&Content::text(marker));
}
```

### §3.6 — Zero alterações em emit

`03_infra/src/export.rs` **inalterado bit-exact**. Hash `66cb8ac3`
preservado pelo 12º passo consecutivo. ADR-0098 honrada.

---

## §4 — Testes

### §4.1 — `01_core/src/engine/stdlib/mod.rs` (+7 testes L1)

| Teste | Verifica |
|---|---|
| `p295_native_footnote_body_posicional` | Construção minimal com `Content` body |
| `p295_native_footnote_body_string_converte_para_text` | Auto-conversão `Value::Str` → `Content::text` |
| `p295_native_footnote_sem_body_retorna_err` | Robustez input |
| `p295_native_footnote_named_arg_rejeitado_fase1` | Scope-out cosméticos (`numbering` etc.) |
| `p295_native_footnote_body_content_complexo_preservado` | Sequence preservada estructuralmente |
| `p295_footnote_partial_eq_por_body` | PartialEq honra invariante body |
| `p295_footnote_is_empty_sempre_false` | Marker sempre observable mesmo com body vazio |

### §4.2 — `03_infra/src/export.rs` (+2 testes L3 PDF)

| Teste | Verifica |
|---|---|
| **`p295_footnote_marker_emite_n_inline_no_pdf`** | 3 footnotes consecutivas → `[1]`/`[2]`/`[3]` no PDF (walker counter funcional) |
| **`p295_footnote_body_nao_renderizado_no_pdf_fase1`** | **Invariante crítica Fase 1**: body string `"BODYSECRET"` ausente do PDF. Quando P295.1 materializar nota rodapé, este teste vai falhar — sinal de actualização. |

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2320 passed; 0 failed; 0 ignored
test result: ok.  450 passed; 0 failed; 6 ignored
test result: ok.   24 passed; 0 failed; 0 ignored
test result: ok.    2 passed; 0 failed; 0 ignored
test result: ok.   21 passed; 0 failed; 0 ignored
                  -----
                  2817 passed total
```

Baseline P294 = 2 808; delta = +9 = 7 (L1) + 2 (L3) ✓.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — Hashes pós-P295

| Ficheiro L0 / código | Antes P295 | Pós P295 |
|---|---|---|
| `entities/content.md` | `4861affa` | **`2fc32a66`** (mudança esperada — +1 variant) |
| `entities/content.rs` (`@prompt-hash`) | `4861affa` | `2fc32a66` (propagado via `--fix-hashes`) |
| `rules/stdlib.md` | inalterado | inalterado (política única) |
| `infra/export.md` | `31a37c57` | inalterado |
| `infra/export.rs` (`@prompt-hash`) | `66cb8ac3` | **`66cb8ac3` preservado bit-exact** (12º passo consecutivo) |

---

## §6 — Padrões metodológicos

### §6.1 — §8.7' "A.0.0 template" (**N=3 reaplica — adiado**)

P293 inaugurou (H6 não-listada). P294 reaplicou (spec inteira
invalidada). **P295 atinge N=3 reaplicação** — limiar tentativo
para promoção a ADR meta. **Adiado** porque:

- A.0.0 P295 refutação é **factual mas magnitude pequena** (linha
  administrativa desactualizada).
- Próxima reaplicação A.0.0 (P296+) com refutação significativa →
  promoção §8.7' N=4 mais robusta.

### §6.2 — §8.3 "refutação pragmática" (**N=7 candidato adiado**)

P293 §7.3 refutou §8.3 N=6 candidato em favor de §8.7' inaugural.
P294 §7.1 manteve §8.3 N=6 candidato genuíno mas adiou-o per P273.17
§0. **P295 atinge N=7** — refutação factual de Tabela C linha 387.
**Adiado** porque magnitude da refutação é menor que P294.

### §6.3 — §8.6 "A.5' anti-reflexão" (**N=5 cumulativo**)

P291+P292+P293+P294+P295. **Limiar passado** mas anti-padrão
"over-formalização" P273.17 §0 adia. Padrão é metodológico interno;
não promovível a ADR sem justificação adicional.

### §6.4 — "Variant rico com cosméticos opcionais" (**N=4 preservado**)

Decisão consciente **A.2 → (a)** preserva o padrão inalterado.
P295 NÃO qualifica gratuitamente N=5 — `numbering` cosmético
scope-out é divergência consciente per ADR-0054 graded.

### §6.5 — ADR-0098 "single source of truth" (**N=12 cumulativo**)

Hash `export.rs` preservado bit-exact pelo 12º passo consecutivo.
Invariante robusta sobre 12 features distintas (Style cumulativos
P288-P292; curve P293; quadratic P294; footnote P295).

### §6.6 — Risco "sequência reflexa" §7 septenário documentado

A.0.0 N=3 consecutivo com refutações de magnitude decrescente:
- P293 H6 — hipótese não-listada (alta magnitude).
- P294 vanilla pattern — spec inteira invalidada (magnitude máxima).
- **P295 Tabela C 387 — linha administrativa desactualizada (baixa magnitude)**.

**Risco**: A.0.0 pode degenerar em rubber-stamp se refutações são
sempre factuais-modestas. **Vigilância P296+**: registar honestamente
se A.0.0 não descobre nada empírico significativo.

---

## §7 — Cobertura vanilla vs cristalino

`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- **Linha 178** (`footnote(body)`): `ausente` → `parcial ⁸⁰` com
  nota completa sobre Fase 1 marker only, walker counter,
  scope-outs cosméticos, e sub-passos pendentes.
- **Linha 402** (Tabela C): bloqueador "locate ADR-0017 adiada"
  riscado (`~~`) com nota explícita de refutação factual via P295
  A.0.0 (P208B+C já implementado).

---

## §8 — Frentes pendentes pós-P295

| Frente | Magnitude | Estado |
|---|---|---|
| **P295.1** — nota corpo renderizada no rodapé da página | L | Registada; requer 2-pass layout |
| **P295.2** — overflow multi-página (footnote ocupa páginas subsequentes) | M | Registada; depende de P295.1 |
| **P295.X** — footnote reference via `#footnote(<label>)` | XS+ | Bloqueado por scope methods em stdlib (frente independente) |
| **Numbering customisation** (`numbering: "*"` etc.) | XS | Scope-out P295; cosmético per ADR-0054 graded |

---

## §9 — Decisão sobre P296

Frentes ortogonais disponíveis (P294 §10 ranking + P295 §10):

1. **math-accent-cancel** — frente original P292 ranking.
2. **`curve.move`/scope-methods** — sintaxe vanilla fiel curve.
3. **`native_path` SVG-string parser**.
4. **`Length` em `Stroke`** — refino tabela cobertura.
5. **P295.1 nota rodapé** — extensão directa P295 com magnitude L
   (2-pass layout).

Decisão fica para o operador humano. **P295 não dita P296**.

Se §8.7' (A.0.0 template) reaplicar em P296 com refutação
**significativa** (não factual-modesta), N=4 + promoção robusta
viável.

---

## §10 — Honestidade epistémica

P295 documenta o **padrão decrescente de magnitude de refutação
A.0.0**:

| Passo | A.0.0 N | Magnitude da refutação |
|---|---:|---|
| P293 | 1 (inaugural) | Hipótese H6 não-listada na spec |
| P294 | 2 | Toda a estrutura proposta pela spec invalidada |
| **P295** | **3** | **Linha de tabela administrativa desactualizada** |

A.0.0 N=3 reaplica genuinamente (não é copy-paste mecânico) mas a
**genuinidade da refutação está em declínio**. P295 inaugura o
**risco "sequência reflexa" §7 septenário** como pendência
metodológica.

**P296+ deve testar se A.0.0 ainda gera valor empírico** ou se
degenerou em ritual procedimental. Se últimas 3 aplicações
descobrem apenas refutações factuais-modestas, considerar
desformalização do template.

---

## §11 — Fecho

P295 fechado com:

- **+9 testes** (7 L1 + 2 L3) — todos verdes.
- **0 violations** no `crystalline-lint`.
- **Hash `export.rs` preservado** bit-exact (12º passo consecutivo).
- **Hash `content.rs`** mudou esperadamente (+1 variant Footnote).
- **0 ADRs meta novas** — §8.7' N=3, §8.3 N=7 ambos adiados per
  P273.17 §0.
- **Padrão "variant rico" N=4 preservado** (A.2 → (a) minimal).
- **8.º paradigma consumer arquitecturalmente distinto** registado
  (walker counter no Layouter — primeiro em série P288-P295).
- **Refutação factual de bloqueador histórico** (Tabela C linha 387)
  com tabelas actualizadas.

**MARCO P295**:
- **1.º cluster M pós-série cumulativa cirúrgica** P288-P294 —
  magnitude reset.
- **A.0.0 N=3** — limiar tentativo §8.7' atingido mas adiado.
- **`Content::Footnote` materializado pela primeira vez** desde
  P282 §6 rank #7 (~13 passos de pendência).
- **Hash `export.rs` preservado pelo 12º passo consecutivo**
  (P282→P295) — ADR-0098 robusta sobre 12 features distintas.
- **Sub-passos registados** P295.1 (nota rodapé) + P295.2 (overflow)
  — frentes futuras claras.
- **Risco "sequência reflexa" §7 septenário documentado** —
  vigilância A.0.0 P296+.

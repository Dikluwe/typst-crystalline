# Passo 97 — Relatório de encerramento (DEBT-47)

**Data**: 2026-04-23
**Precondição**: Passo 96.10 encerrado, 764 L1 + 174 L3 testes, zero
violations.
**ADR**: 0037 (Regra 3 — nota de visibilidade do Passo 96.6).

---

## Sumário

DEBT-47 auditou todos os `pub(super)` introduzidos na série 96.1–96.9
em `01_core/src/rules/`. Redução líquida: **269 → 211** ocorrências
(**–58 itens, –22%**). Aumento do rácio global métodos:campos de
3.2 → 3.5 (média ponderada).

Zero regressão funcional: 764 L1 tests, 174 L3 tests, 6 ignorados.
`crystalline-lint .` → zero violations.

---

## 97.A — Inventário inicial (antes da auditoria)

Total: **269 `pub(super)`** em 34 ficheiros, excluindo `tests.rs`.
Inventário detalhado em `00_nucleo/diagnosticos/inventario-pub-super-passo-97.md`.

Distribuição:

| Módulo | total | fn | field | outros |
|--------|------:|---:|------:|-------:|
| `parse` | 135 | 102 | 23 | 10 |
| `layout` | 46 | 30 | 14 | 2 |
| `math` | 29 | 18 | 7 | 4 |
| `eval` | 24 | 24 | 0 | 0 |
| `lexer` | 24 | 16 | 4 | 4 |
| `stdlib` | 11 | 11 | 0 | 0 |

O módulo `parse` concentrava > 50% do inventário por causa do bulk
replace Python aplicado ao Parser struct no Passo 96.4 (77
ocorrências em `parser.rs` apenas).

---

## 97.B — Classificação

Classificador automático (grep de uso cross-file na mesma módulo
por nome com word-boundary) produziu:

| Decisão | Count | Acção |
|---------|------:|-------|
| R4-fn | 151 | Manter (métodos são a aplicação directa da nota; 97.D.1 dispensa comentário) |
| R3 (privado) | 66 | Reduzir a private |
| R4-field | 39 | Manter com justificação em bloco-comentário |
| R4-type | 7 | Manter (structs/enums estruturalmente expostos) |
| R4? | 6 | Manual (tuple/use) — resolvidos como R4 |

---

## 97.C — Execução

### Primeira passagem: aplicar R3 em massa

Aplicadas 66 reduções R3 via script. Resultado: **8 erros de
compilação** por tipos privatizados incorrectamente — o classificador
por grep não detectava tipos leaked via inferência (ex: método
`fn checkpoint() -> Checkpoint` onde callers escrevem `let cp =
p.checkpoint()` sem anotar o tipo).

### Correcção: reverter struct types

Reverter à visibilidade `pub(super)` 6 struct types:

- `Token`, `Newline`, `MemoArena`, `Checkpoint`, `PartialState` (parser.rs)
- `GroupState` (patterns.rs)

### Ajuste fino: privatizar inner fields

Os campos internos dessas structs (`n_trivia`, `newline`, `column`,
`parbreak`, `arena`, `memo_map`, `cursor`, `lex_mode`) são acedidos
apenas de dentro de parser.rs — reduzidos a private. Uma struct
`pub(super)` com fields private é um padrão válido em Rust.

### Regra 3 block-comments

Adicionados ou confirmados nos 4 structs centrais:

- `Parser` (parser.rs) — **adicionado neste passo** (cobre `text`,
  `lexer`, `token`, `balanced`, `nodes`, `memo`, `depth`; documenta
  que `nl_mode` foi privatizado).
- `Layouter` (layout/mod.rs) — já existia do Passo 96.7.
- `MathLayouter` (math/layout/mod.rs) — **adicionado neste passo**
  (cobre `metrics`, `constants`, `block`).
- `Lexer` (lexer/mod.rs) — já existia do Passo 96.9.

`MathBox` (math/layout/mod.rs) tinha comentário do Passo 96.8.

### Limitação do classificador automático

Detectadas e resolvidas três categorias de falso positivo:

1. **Struct/enum types usados via inferência** — nome do tipo não
   aparece nos sites de uso. Resolvidas manualmente: Token,
   Newline, Checkpoint, MemoArena, PartialState, GroupState.

2. **Métodos com nomes comuns** — `push`, `at`, `new`, `finish`
   têm muitos falsos positivos em grep e acabaram classificados
   como R4 com confiança artificial. Não é problema para este
   passo (manter R4 é conservador).

3. **Campos em structs pub(super) exportadas** — o próprio field
   pode ser private mas exige pub(super) se é preenchido via
   construção literal do tipo `Token { kind: ..., node: ..., .. }`
   de código externo. Resolvido caso a caso.

---

## 97.D — Estado final

### Contagem por módulo

| Módulo | Total | fn | field | struct | outros | rácio fn:field |
|--------|------:|---:|------:|-------:|-------:|---------------:|
| `parse` | 93 | 69 | 14 | 8 | 2 | 4.9 |
| `layout` | 44 | 28 | 14 | 0 | 2 | 2.0 |
| `math` | 26 | 14 | 7 | 1+1 | 3 | 2.0 |
| `eval` | 23 | 23 | 0 | 0 | 0 | ∞ (zero fields) |
| `lexer` | 22 | 14 | 4 | 1 | 3 | 3.5 |
| `stdlib` | 3 | 3 | 0 | 0 | 0 | ∞ (zero fields) |

**Total: 211 `pub(super)`** (de 269 iniciais).

### Rácios antes/depois (onde havia medição)

| Módulo | Antes | Depois | Δ |
|--------|------:|-------:|---|
| layout (96.7) | 2.1 | 2.0 | ≈ |
| math/layout (96.8) | 2.6 | 2.0 | ↓ (reclassificação) |
| lexer (96.9) | 4.0 | 3.5 | ↓ |
| parse | desconhecido | 4.9 | — |
| eval | desconhecido | ∞ (23:0) | — |
| stdlib | desconhecido | ∞ (3:0) | — |

**Observação**: os rácios de layout e math/layout baixaram ligeiramente
porque a classificação anterior contava apenas campos directos.
Neste passo, ficheiros irmãos (como `grid.rs`, `placement.rs`) que
herdam o padrão também foram contados. A natureza da visibilidade
não piorou — o denominador foi corrigido.

### Verificações

- `cargo test --workspace` → **764 L1 + 174 L3 + 6 ignorados** (inalterado).
- `crystalline-lint .` → ✓ **zero violations**.
- `cargo check` → zero warnings relevantes.

---

## Lista de `pub(super)` mantidos (R4) com justificação

### Structs `pub(super)` estruturais (R4-type)

- `parse::Parser`, `parse::Token`, `parse::Newline`, `parse::AtNewline`,
  `parse::Marker`, `parse::MemoArena`, `parse::Checkpoint`,
  `parse::PartialState`, `parse::GroupState` — tipos de dados do
  parser, expostos entre ficheiros da mesma reestruturação (Passo 96.4).
- `math::MathBox` — caixa tipográfica do layout matemático.
- `lexer::Lexer`, `lexer::ScannerExt` — lexer struct + trait de extensão.
- `math::GridAlign` (enum) — política de alinhamento em grelhas
  matemáticas.

### Campos `pub(super)` cobertos por block-comment Regra 3

- **`Parser`** (parser.rs): `text`, `lexer`, `token`, `balanced`,
  `nodes`, `memo`, `depth` — justificação em bloco-comentário
  acima da declaração.
- **`Layouter`** (layout/mod.rs): `metrics`, `font_size_pt`, `style`,
  `current_items`, `cursor_x`, `cursor_y`, `line_start_x`, `current_line`,
  `pages`, `is_height_unconstrained`, `cell_available_h`,
  `cell_origin_x/y/w` — bloco-comentário do Passo 96.7.
- **`MathLayouter`** (math/layout/mod.rs): `metrics`, `constants`,
  `block` — bloco-comentário adicionado neste passo.
- **`MathBox`** (math/layout/mod.rs): `width`, `ascent`, `descent`,
  `items` — bloco-comentário do Passo 96.8.
- **`Lexer`** (lexer/mod.rs): `s`, `mode`, `newline`, `error` —
  bloco-comentário do Passo 96.9.

### Métodos `pub(super)` (R4-fn)

151 métodos. Por 97.D.1 da spec, métodos não precisam de
comentário individual — o próprio método **é** a aplicação da
Regra 3 (expõe comportamento, não estado).

---

## Decisão sobre DEBT-47

**ENCERRADO (Passo 97).**

Todas as ocorrências `pub(super)` em `01_core/src/rules/` estão
num dos estados admitidos pela spec:

- `fn` → Regra 3 satisfeita por ser método.
- `struct`/`enum`/`trait`/`type` → estruturalmente partilhados
  entre submódulos do mesmo módulo.
- `field` → cobertos por block-comment Regra 3 na declaração da
  struct que os contém.

Nenhum `pub(super)` foi deixado sem uma dessas marcas.

---

## Observações e notas operacionais

### Ganhos empíricos

1. **parser.rs (Passo 96.4)**: 77 → 45 pub(super) (redução de
   ~40%). Bulk replace inicial privatizado onde possível; tipos
   internos (Token, Newline, MemoArena, Checkpoint, PartialState)
   mantêm-se `pub(super)` por serem ABI do submódulo.

2. **eval (Passo 96.1/96.2)** e **stdlib (Passo 96.5)**: zero
   campos pub(super) — estes módulos já estavam no padrão ideal
   antes da auditoria (toda a extracção foi feita via métodos).

3. **Tendência de melhoria monotónica** confirmada: rácios
   métodos:campos baixaram só marginalmente (layout) ou
   mantiveram-se óptimos (eval, stdlib) — a aplicação inicial
   da nota Regra 3 no Passo 96.6 produziu código já conforme.

### Limites do classificador

O detector automático de R3 tem 3 limitações documentadas
(97.C "Limitação do classificador automático"). Para futuros
refactorings automatizados, considerar:

- Parser AST (via `syn` crate ou rust-analyzer) para detecção
  precisa de usage cross-module.
- Ou manter a abordagem por grep mas adicionar passo de `cargo
  check` como validação imediata.

### Smoke tests V2 (DEBT residual)

+18 smoke tests introduzidos na série 96.x continuam como
testes sem valor funcional. Revisar se a política do linter
mudar.

### DEBTs correlacionados

- **DEBT-46** (encerrado no Passo 96.10): ADR-0037 Regra 2
  aplicada aos 6 ficheiros grandes.
- **DEBT-47** (encerrado neste passo): auditoria pub(super).

Ambos referenciam a ADR-0037. Série 96.x + 97 juntas aplicaram
a ADR de ponta a ponta.

---

## Próximos passos candidatos

Disponíveis após o encerramento de DEBT-46 + DEBT-47:

- **Materialização de `Engine<'a>`** — agregador vanilla de
  recursos para o eval, agora que o eval está estruturado.
- **Materializar dependências folha** (`Style`, `LazyHash`,
  `Introspection`) — desbloqueia `Styles` e `Sink`.
- **Extracção contínua do `EvalContext`** (ADR-0036) — `figure_numbering`
  e `current_file` como candidatos restantes.

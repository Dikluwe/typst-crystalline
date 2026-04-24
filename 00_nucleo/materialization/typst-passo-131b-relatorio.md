# Passo 131B — Relatório (materialização `Lang` em L1)

**Data**: 2026-04-24
**Precondição**: Passo 131A encerrado; diagnóstico + ADR-0052
em `PROPOSTO`; 1057 total tests; 11 DEBTs abertos.
**Natureza**: passo S em L1. Código novo + migração + inversão
de teste. **Pattern DEBT-1 XS não se aplica** — refactor por
paridade ADR-0033.
**ADR**: **ADR-0052** transita `PROPOSTO` → `IMPLEMENTADO`.
ADR-0038 ganha quarta nota.

---

## Sumário

Tipo `Lang` materializado em L1 (`01_core/src/entities/lang.rs`,
~125 linhas). `StyleDelta.lang: Option<EcoString>` migrado
para `Option<Lang>` com validação e erro hard.

**Breaking semantic change face ao Passo 130**: `"en-GB"`
deixa de ser silent e passa a erro. Testes adaptados
(invertidos) para reflectir nova semântica.

Primeiro `return Err` dentro do loop de argumentos de
`eval_set_rule` — precedente estabelecido para futuras
validações.

**838 L1 (+12) + 24 L2 + 186 L3 + 21 L4** + 6 ignorados =
**1069 total** (+12 vs 1057). Zero violations.

---

## 131B.A — Inventário confirmatório

| Item | Resultado |
|------|-----------|
| A.1 — `lang: Option<EcoString>` + arm em estado esperado | ✓ linha 55 de `style_chain.rs`; arm em `rules.rs:334` |
| A.2 — `Err` precedente em `rules.rs` | ✓ múltiplos `return Err(vec![SourceDiagnostic::error(...)])` em linhas 132, 369, 403, 411 |
| A.3 — variável `named` no scope | ✓ `if let Arg::Named(named) = arg` no arm loop |
| A.4 — `SourceDiagnostic::error` acessível | ✓ já importado via `entities::source_result` |
| A.5 — 826 L1 tests base | ✓ confirmado |

**Gate 131B.A**: passa sem bloqueios.

---

## 131B.B — ADR-0038 quarta nota

Adicionada ao final de ADR-0038. Documenta:
- Campo `lang` migrado para tipo semântico `Option<Lang>`.
- Erro hard em inválido — ruptura do pattern "só warnings".
- **Não é variante do pattern DEBT-1 XS** — refactor por
  paridade ADR-0033. Ver ADR-0052.
- Futuras propriedades com paridade similar seguem padrão
  131B (diagnóstico → ADR → materialização), não acumulam em
  ADR-0038.

---

## 131B.C — `entities/lang.rs` criado

- **54 linhas de código**:
  - `struct Lang([u8; 3], u8)` + 7 derives.
  - `impl Lang` com `ENGLISH` const + `as_str`.
  - `impl FromStr` com parser fiel vanilla + mensagem literal.
- **71 linhas de tests** (11 unit tests, todos verdes):
  - `lang_from_str_iso_639_1_aceita_2_letras_passo_131b` (pt,
    en, de).
  - `lang_from_str_iso_639_3_aceita_3_letras_passo_131b` (por,
    fil).
  - `lang_from_str_normaliza_case_passo_131b` (PT → pt).
  - `lang_from_str_vazio_devolve_erro_passo_131b`.
  - `lang_from_str_1_letra_devolve_erro_passo_131b`.
  - `lang_from_str_4_letras_devolve_erro_passo_131b`.
  - `lang_from_str_nao_ascii_devolve_erro_passo_131b`.
  - `lang_from_str_com_hyphen_devolve_erro_passo_131b`.
  - `lang_as_str_preserva_canonico_passo_131b`.
  - `lang_as_str_trim_padding_3_letter_passo_131b`.
  - `lang_english_constante_passo_131b`.

`entities/mod.rs`: `pub mod lang;` adicionado.

---

## 131B.D — `StyleDelta` migrado

Diff em `entities/style_chain.rs`:

- `use ecow::EcoString;` → `use crate::entities::lang::Lang;`
  (EcoString não era usado por mais nenhum campo).
- `pub lang: Option<EcoString>` → `pub lang: Option<Lang>`.
- Comentário do campo actualizado referenciando ADR-0052 +
  131B.
- `StyleDelta::empty()`: `lang: None` continua válido; const
  fn preservado.

---

## 131B.E — Arm `"lang"` com erro hard

Diff em `rules/eval/rules.rs`:

```rust
// + imports:
use std::str::FromStr;
use crate::entities::lang::Lang;

// arm novo:
"lang" => {
    if let Value::Str(s) = val {
        match Lang::from_str(&s) {
            Ok(lang) => delta.lang = Some(lang),
            Err(msg) => {
                return Err(vec![SourceDiagnostic::error(
                    named.expr().span(),
                    msg.to_string(),
                )]);
            }
        }
    }
}
```

**Primeiro `return Err`** em `eval_set_rule` dentro de um arm
do loop de args. Aborta imediatamente — outros args na mesma
set rule não são processados.

---

## 131B.F — Tests L1 do Passo 130 adaptados

| Nome antigo | Novo | Mudança |
|-------------|------|---------|
| `eval_set_text_lang_passo_130` | `eval_set_text_lang_valido_passo_131b` | renomeado; assertion idêntica |
| `eval_set_text_lang_bcp47_composto_passo_130` | `eval_set_text_lang_composto_emite_erro_passo_131b` | **INVERTIDO** — de silent para erro hard |
| `eval_set_text_font_canary_passo_130` | `eval_set_text_font_canary_passo_131b` | renomeado; mesma assertion |

Teste invertido documenta a **breaking semantic change** face
ao 130.

---

## 131B.G — Novo integration test

`eval_set_text_lang_invalido_emite_erro_hard_passo_131b`:
- Input `"xxxx"` (4 letras, non-ISO).
- Assert `result.is_err()`.
- Assert mensagem contém `"expected two or three letter language code"`.

---

## 131B.H — ADR-0052 transição

- Status: `PROPOSTO` → **`IMPLEMENTADO`**.
- Adicionado `**Materializado em**: Passo 131B (2026-04-24)`.
- Secção "Estado final" com números concretos do passo
  (125 linhas, 12 tests, erro hard em 2 cenários).

---

## 131B.J — Verificação

### Cargo tests

```
test result: ok. 838 passed ...       (L1 +12 vs 826)
test result: ok. 186 passed, 6 ign    (L3 inalterado)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 inalterado)
```

**Nota**: spec estimou +14 L1 (840); real +12 (838). A
estimativa contava 3 tests adaptados do 130 como "+0 net",
mas a adaptação incluiu 1 novo teste (`composto_emite_erro`
substituiu `composto_silent`). Cálculo real:
- 11 unit em `lang.rs` = +11.
- 130→131B: 3 existentes adaptados → 3 novos (1 renomeado,
  1 invertido, 1 canary renomeado). Líquido **0**.
- 1 novo `lang_invalido_emite_erro_hard` = +1.
- **Total +12**, OK.

### `crystalline-lint .`

```
✓ No violations found
```

### Manual — 5 cenários validados

```bash
$ typst ok.typ    # lang: "pt"
exit=0, stderr: (vazio)

$ typst case.typ  # lang: "PT" — normalizado
exit=0, stderr: (vazio)

$ typst comp.typ  # lang: "en-GB"
comp.typ:1:17: error: expected two or three letter language code (ISO 639-1/2/3)
exit=1

$ typst inv.typ   # lang: "xxxx"
inv.typ:1:17: error: expected two or three letter language code (ISO 639-1/2/3)
exit=1

$ typst f.typ     # font: "X" — canary
f.typ:1:11: warning: text: propriedade 'font' ainda não suportada
exit=0
```

---

## Ficheiros tocados / criados

| Ficheiro | Natureza | Mudança |
|----------|----------|---------|
| `01_core/src/entities/lang.rs` | **novo** | tipo + `ENGLISH` + FromStr + 11 tests (~125 linhas) |
| `01_core/src/entities/mod.rs` | modificado | `pub mod lang;` |
| `01_core/src/entities/style_chain.rs` | modificado | `EcoString` → `Lang`; import; comentário |
| `01_core/src/rules/eval/rules.rs` | modificado | imports + arm com `return Err` |
| `01_core/src/rules/eval/tests.rs` | modificado | 3 tests adaptados + 1 novo |
| `00_nucleo/adr/typst-adr-0052-*.md` | modificado | status `IMPLEMENTADO` + estado final |
| `00_nucleo/adr/typst-adr-0038-*.md` | modificado | quarta nota |
| `00_nucleo/prompts/entities/style_chain.md` | modificado | campo `lang: Option<Lang>` |
| `00_nucleo/prompts/entities/lang.md` | **novo** | prompt L0 para `Lang` |

### Números finais

| Métrica | Antes (131A) | Depois |
|---------|------:|-------:|
| L1 tests | 826 | **838** (+12) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1057** | **1069** (+12) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 51 | **52** (ADR-0052 `IMPLEMENTADO`) |
| DEBTs abertos | 11 | 11 |

---

## Breaking semantic change (face ao Passo 130)

**Afectados**: inputs `#set text(lang: "X")` onde X não é 2-3
letras ASCII.

### Antes (130)

- `lang: "en-GB"` → captura silent como `EcoString("en-GB")`.
- `lang: "xxxx"` → captura silent como `EcoString("xxxx")`.
- Zero stderr, exit 0.

### Depois (131B)

- `lang: "en-GB"` → **erro hard**, exit 1.
- `lang: "xxxx"` → **erro hard**, exit 1.
- Stderr mostra `error: expected two or three letter language code (ISO 639-1/2/3)`.

### Justificação

ADR-0033 paridade funcional. Vanilla erra hard nestes casos.
Cristalino alinha. Impacto em utilizadores reais:
**zero conhecido** — não há código de produção a depender do
silent. Tests do cristalino actualizados.

### Mitigação futura

Para suportar `"en-GB"` legitimamente, materializar `Region`
em passo dedicado + adicionar hint "put region in region
parameter" (vanilla faz isso). Registado em candidatos
futuros.

---

## Lições

1. **Diagnóstico 131A pagou-se com juros**: todo o trabalho
   do 131B foi mecânico — ler o plano, aplicar. Zero decisões
   arquitecturais mid-stream. Split 131A/131B validado como
   disciplina.

2. **`Err` em loop de args é simples quando imports já
   existem**: `SourceDiagnostic::error` e pattern
   `return Err(vec![...])` estão bem estabelecidos. Primeira
   vez em `eval_set_text` mas compatível com código existente.

3. **`named.expr().span()` é span correcto**: aponta para o
   *valor* (`"en-GB"`), não para toda a chamada `#set text(...)`.
   UX melhor — utilizador vê sublinhar onde corrigir.

4. **Copy type paga-se subtilmente**: `Option<Lang>` = 5 bytes
   vs `Option<EcoString>` = ~16 bytes. `StyleDelta` encolhe
   marginalmente. Não medido em bench, mas direção correcta
   ADR-0030.

5. **Estimativa ligeiramente acima da realidade**: spec
   previu +14 L1 tests, real foi +12. Causa: teste "composto
   silent" transformado em "composto erro hard" é substituição,
   não adição. Diferença marginal; não exigiu ajuste de outros
   passos.

6. **Pattern DEBT-1 XS não se aplica por natureza**: 5 passos
   anteriores (126-130) foram XS uniformes. 131B quebra a
   cadência com refactor + diagnóstico + ADR. ADR-0038 quarta
   nota documenta explicitamente para evitar que passos
   futuros herdem expectativa errada.

7. **Candidato "extract helper `eval_with_warnings`" ganha
   urgência**: 4 testes em `tests.rs` repetem 15+ linhas de
   harness inline. Refactor para helper reduziria ~60 linhas.
   Priorizar após fechar DEBT-1.

---

## Estado pós-Passo 131B

### DEBT-1 progresso

Propriedades capturadas em `#set text`:
- ✓ bold, italic, size, fill (Passos 30/102)
- ✓ weight numérico, tracking, leading (Passos 126/127/128)
- ✓ weight simbólico (Passo 129)
- ✓ lang (Passo 130 → **paridade vanilla 131B**)
- ✗ font (próximo — Passo 132 com canary migration)
- ✗ stroke, alignment, justify, hyphenate, dir, region, ...

### ADR-0052 `IMPLEMENTADO`

Contrato arquitectural cumprido. Disponível como precedente
para futuras materializações de tipos vanilla
(`Region`, `Dir`, `WritingScript`, `Stroke`, etc.).

### Candidatos futuros imediatos

1. **Passo 132**: materializar `font` + migração canary
   DEBT-50 (canary migra de `font` para outra propriedade
   não-L1).
2. **Extract helper `eval_with_warnings`** em harness L1 —
   priorizar após fechar DEBT-1.
3. **Materializar `Region`** — habilita hint vanilla
   "put region in region parameter" para `"en-GB"` inputs.
4. **Materializar `Dir`** + método `Lang::dir()` — base para
   propriedade `text.dir`.
5. **Expandir constantes `Lang::*`** on-demand quando
   consumer (shaping, hyphenation, translations) precisar.

Estimativa restante para fechar DEBT-1: **131B + 132 + 133 +
134 + 135 = ~4h** (131B sai em ≈1.5h; 132-135 ≈ 30-60min
cada).

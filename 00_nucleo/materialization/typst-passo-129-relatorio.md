# Passo 129 — Relatório (DEBT-1 subset: `text.weight` simbólico)

**Data**: 2026-04-24
**Precondição**: Passo 128 encerrado; 1049 total tests; zero
violations; 51 ADRs activas; 11 DEBTs abertos.
**Natureza**: passo XS em L1; **quarta aplicação consecutiva
do pattern DEBT-1 XS**. Introduz pattern variante "helper
simbólico em tipo semântico L1".
**ADR tocada**: **ADR-0038** — terceira nota (primeira a
documentar variante).

---

## Sumário

Arm `"weight"` em `eval_set_text` estendido para aceitar
`Value::Str` com 9 nomes canónicos do Typst vanilla
(`thin`..`black`). Conversão via
`FontWeight::from_name(&str) -> Option<Self>` — método novo em
`impl FontWeight` (tipo L1 que já tinha os 9 constantes).

**Nome desconhecido → silent skip** (coerente pattern DEBT-1 XS).
**Valor numérico continua a funcionar** (regressão 126 preservada).
**Canary DEBT-50** preservado em quarta iteração.

**823 L1 (+5) + 24 L2 + 186 L3 + 21 L4** + 6 ignorados =
**1054 total** (+5 tests). Zero violations. **51 ADRs**
(ADR-0038 com 3 notas). **11 DEBTs** — DEBT-1 subset ganha
capacidade simbólica.

---

## 129.A — Inventário

Completo em
`00_nucleo/diagnosticos/inventario-weight-simbolico-passo-129.md`.

### Findings-chave

1. **Vanilla**: cast! macro em `text/font/variant.rs:149-180`
   lista os 9 canónicos; nome inválido → erro de cast.
2. **Cristalino**: `FontWeight(u16)` em
   `entities/font_book.rs:29-43` com **9 constantes já
   presentes** (`THIN`..`BLACK`). `from_number(u16) -> Self`
   clamp existia; faltava `from_name`.
3. **Pool DEBT-49 L3**: input `font/lang/stroke` — weight já
   conhecido desde 126. Sem rotação.
4. **Nenhum alias** vanilla (`"normal"` não mapeia para
   `"regular"`).

### Decisão: (b) helper em `impl FontWeight`

Escolhido por:
- Tipo semântico e constantes já vivem em `font_book.rs`.
- Helper encaixa naturalmente como método (ADR-0037, coesão
  por domínio).
- `StyleDelta.weight` permanece `Option<u16>` (decisão 126);
  arm faz `.to_number()` no path simbólico.

### Gate 129.A

**Passa**. 2 ficheiros L1 (+ unit tests no mesmo `font_book.rs`).
Tests +5. Zero ripple L3/L4.

---

## 129.B — ADR-0038 anotada

**Terceira nota** adicionada ao final de ADR-0038:
**"Passo 129 — `weight` simbólico via helper em tipo semântico
L1"**.

Documenta:
- Pattern variante "helper simbólico em tipo semântico L1"
  (P126 foi primitivo; P127/P128 foi tipo semântico como campo;
  P129 é primeira variante a **delegar** para método de tipo
  semântico).
- Divergência vanilla (silent em nome inválido vs erro de cast).
- Aplicável a futuras propriedades simbólicas
  (`font-stretch`, `style`, etc.).

---

## 129.C — Implementação

### Diff `FontWeight`

```rust
impl FontWeight {
    // ... existentes ...
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "thin" => Some(Self::THIN),
            // ... 8 nomes + default None ...
        }
    }
}
```

### Diff arm `"weight"`

```rust
"weight" => {
    if let Value::Int(n) = val {
        if let Ok(w) = u16::try_from(n) {
            delta.weight = Some(w);
        }
    } else if let Value::Str(s) = val {
        if let Some(fw) = FontWeight::from_name(s.as_str()) {
            delta.weight = Some(fw.to_number());
        }
    }
}
```

Import novo em `rules.rs`: `use crate::entities::font_book::FontWeight;`.

### Testes novos (5)

**Em `entities/font_book.rs` (unit)**:
- `font_weight_from_name_nomes_canonicos_passo_129`: 9 mapeamentos
  corretos.
- `font_weight_from_name_desconhecido_e_none_passo_129`:
  nomes inválidos (`"arcoiris"`, `""`, `"Bold"` case-sensitive,
  `"normal"` sem alias) → `None`.

**Em `rules/eval/tests.rs` (integration)**:
- `eval_set_text_weight_simbolico_passo_129`: 9 nomes em loop,
  cada um sem warning.
- `eval_set_text_weight_simbolico_desconhecido_silent_passo_129`:
  `"arcoiris"` sem warning.
- `eval_set_text_font_canary_passo_129`: font continua com
  warning (canary).

### L3 DEBT-49: **intacto** (como esperado)

---

## 129.D — Verificação

### Cargo tests

```
test result: ok. 823 passed ...       (L1 +5 vs 818)
test result: ok. 186 passed, 6 ign    (L3 inalterado)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 inalterado)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Manual

```bash
$ typst wb.typ -o wb.pdf        # #set text(weight: "bold")
# stderr: (vazio)
exit=0

$ typst wu.typ -o wu.pdf        # #set text(weight: "arcoiris")
# stderr: (vazio) — silent skip
exit=0

$ typst wn.typ -o wn.pdf        # #set text(weight: 700) — regressão 126
# stderr: (vazio)
exit=0

$ typst f.typ -o f.pdf          # #set text(font: "X") — canary
f.typ:1:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
exit=0
```

4 comportamentos distintos validados.

---

## 129.E — Encerramento

### Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `01_core/src/entities/font_book.rs` | +`from_name` em `impl FontWeight`; +2 unit tests |
| `01_core/src/rules/eval/rules.rs` | +import `FontWeight`; +`else if Value::Str` no arm weight |
| `01_core/src/rules/eval/tests.rs` | +3 integration tests |
| `00_nucleo/adr/typst-adr-0038-...md` | terceira nota (variante pattern) |
| `00_nucleo/prompts/entities/font-book.md` | actualiza `impl FontWeight` na interface pública |

### Números finais

| Métrica | Antes (128) | Depois |
|---------|------:|-------:|
| L1 tests | 818 | **823** (+5) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1049** | **1054** (+5) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 51 | 51 (0038 com 3 notas) |
| DEBTs abertos | 11 | 11 (DEBT-1 subset cresce capacidade) |

---

## Limitações aceites

1. **Nome inválido silent**: vanilla emite erro de cast listing
   valid options. Divergência categoria semântica ADR-0033,
   aceite temporalmente porque pattern DEBT-1 XS é silencioso
   por design. Teste
   `eval_set_text_weight_simbolico_desconhecido_silent_passo_129`
   documenta estado actual; se migrar para warning no futuro,
   teste falha e força revisão consciente.
2. **Case-sensitive**: `"Bold"` → `None`. Vanilla é case-sensitive
   também (confirmado por test assertion). Sem lowercase
   normalization implícita.
3. **Sem aliases** (`"normal"`/`"regular"`): alinhado vanilla.
   Se utilizador pedir alias no futuro, adicionar à tabela.
4. **Inerte em layout**: como `weight` numérico do 126.
5. **`StyleDelta.weight` continua `Option<u16>`**: decisão 126
   preservada. `FontWeight` é ferramenta de conversão, não tipo
   de armazenamento.

---

## Lições

1. **Helper em tipo L1 é pattern economico**: `FontWeight` já
   tinha 9 constantes + `from_number`. Adicionar `from_name`
   foi linha por linha de correspondência. Zero invenção
   arquitectural. Tipo semântico existente atrai o trabalho
   naturalmente.

2. **Unit tests no tipo + integration tests no eval é dupla
   cobertura útil**: 2 testes em `font_book.rs` validam a
   tabela canónica; 3 em `rules/eval/tests.rs` validam o
   pipeline. Cada um protege contra tipo diferente de
   regressão (mudar tabela vs romper fluxo eval → arm →
   helper).

3. **Iteração de casos em loop funciona bem**:
   `eval_set_text_weight_simbolico_passo_129` testa 9 nomes
   num único teste via `for` loop. 1 teste conceptual, 9
   asserções. Alternativa (9 testes nomeados) seria ruidosa
   sem valor adicional — cada nome não tem comportamento
   distinto.

4. **Quarta aplicação revelou variante**: 126 (primitivo
   simples), 127 (tipo semântico como campo), 128 (tipo
   semântico com contexto divergente), 129 (delegação a método
   de tipo semântico). Cada aplicação expôs nuance — pattern
   DEBT-1 XS é mais rico do que parecia em 126. ADR-0038 com
   3 notas cobre o espaço.

5. **Case-sensitive teste é cheap insurance**:
   `assert_eq!(FontWeight::from_name("Bold"), None)` protege
   contra adição acidental de `.to_lowercase()` no helper —
   mudaria comportamento semântico. Custo: 1 linha; valor:
   documento + guard.

6. **Pool DEBT-49 continua saudável**: 7 propriedades activas
   (bold/italic/size/fill/weight/tracking/leading), pool de
   desconhecidas ainda inclui `stroke/alignment/lang/
   first-line-indent/justify/hyphenate/dir/...`. Mais 1-2
   passos antes de pool esgotar.

---

## Estado pós-Passo 129

### DEBT-1 progresso

Propriedades capturadas em `#set text`:
- ✓ bold (Passo 30)
- ✓ italic (Passo 30)
- ✓ size (Passo 30; f64 legado)
- ✓ fill (Passo 102)
- ✓ weight numérico (Passo 126; u16)
- ✓ tracking (Passo 127; Length)
- ✓ leading (Passo 128; Length, divergência contextual)
- ✓ **weight simbólico (Passo 129; helper em FontWeight)** — este
- ✗ font, lang, stroke, alignment, justify, hyphenate, dir, ...

### Pattern DEBT-1 XS — quatro variantes confirmadas

| Variante | Passo | Tipo valor em StyleDelta | Captura |
|---------|------:|-------------------------:|---------|
| Primitivo simples | 126 | `Option<u16>` | `Value::Int` + `try_from` |
| Tipo semântico L1 como campo | 127/128 | `Option<Length>` | `Value::Length` directo |
| Helper simbólico em tipo L1 | 129 | `Option<u16>` (via `.to_number()`) | `Value::Str` → `FontWeight::from_name` |

Esta taxonomia cobre a maioria dos futuros subsets previstos.

### Candidatos próximos DEBT-1

1. **`font-stretch` simbólico** (`"normal"`, `"condensed"`, etc.):
   usa pattern variante "helper simbólico" com
   `FontStretch::from_name`. Pattern XS. Mas **exige**
   `StyleDelta.stretch` primeiro — dois passos (campo + helper).
2. **`style` simbólico** (`"italic"`, `"oblique"`, `"normal"`):
   `FontStyle` enum já existe em L1. Passo helper apenas.
3. **`stroke`** — tipo composto `Stroke { width, paint }` —
   exige tipo novo em L1. S-M.
4. **Consumer de `weight` em layout** — resolver em StyleChain
   + selecção de variante de fonte via `FontBook::select`. M-L.

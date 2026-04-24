# Passo 130 — Relatório (DEBT-1 subset: `text.lang`)

**Data**: 2026-04-24
**Precondição**: Passo 129 encerrado; 1054 total tests; zero
violations; 51 ADRs activas; 11 DEBTs abertos.
**Natureza**: passo XS em L1; **quinta aplicação consecutiva
do pattern DEBT-1 XS**. Primeiro de 5 sub-passos para fechar
DEBT-1 (sequência: 130 lang → 131 font → 132 par target →
133 leading migration → 134 DEBT-1 close).
**ADR**: **não tocada**. Quinta aplicação literal (variante 1
do pattern: primitivo/string como campo).

---

## Sumário

`StyleDelta.lang: Option<EcoString>` adicionado. `eval_set_text`
captura `lang` de `#set text(lang: "pt")` (ou `"en-GB"`, ou
`"xx-invalid"`) sem warning — **vanilla valida BCP 47 via tipo
`Lang` dedicado; cristalino captura raw e defere validação**.

**Rotação DEBT-49 L3**: 2 tests actualizados — `lang` sai do
pool de "desconhecidas"; `alignment` entra. Pool continua
saudável (15+ candidatas ainda desconhecidas).

**Canary DEBT-50** preservado em quinta iteração consecutiva.

**826 L1 (+3) + 24 L2 + 186 L3 (2 adaptados) + 21 L4** + 6
ignorados = **1057 total** (+3 tests). Zero violations.

---

## 130.A — Inventário

Completo em `00_nucleo/diagnosticos/inventario-lang-passo-130.md`.

### Findings-chave

1. **Vanilla**: `pub lang: Lang` em `TextElem` (`text/mod.rs:440`).
   `Lang` é tipo próprio com parser BCP 47, constantes
   (`ENGLISH`, etc.), e validação no `FromValue`.
2. **Cristalino**: usar **`EcoString` raw**. Validação deferida
   porque:
   - Tipo `Lang` dedicado seria refactor substancial.
   - Consumer futuro (shaping/hyphenation) precisa de normalizar
     de qualquer forma.
3. **`Value::Str(EcoString)`**: match directo `Value::Str(s) → s:
   EcoString`. Zero cast.
4. **Pool DEBT-49 L3**: **2 testes** usam `lang`:
   - `debt49_set_text_lang_emite_warning` — rotado para `alignment`.
   - `debt49_set_text_multiplas_propriedades_desconhecidas` —
     trio `font/lang/stroke` → `font/alignment/stroke`.

### Gate 130.A

**Passa**. 3 ficheiros tocados:
- `entities/style_chain.rs` (+import EcoString +campo +init).
- `rules/eval/rules.rs` (+arm).
- `integration_tests.rs` (2 testes rotados).

---

## 130.B — ADR-0038

**Sem anotação.** Quinta aplicação literal; variante 1
(primitivo como campo). ADR-0038 já tem 3 notas a cobrir o
espaço. Divergência vanilla (sem validação BCP 47) registada
em relatório + teste
`eval_set_text_lang_bcp47_composto_passo_130` como canary.

---

## 130.C — Implementação

### Diff `StyleDelta`

```rust
use ecow::EcoString;  // + import

pub struct StyleDelta {
    // ...
    pub lang: Option<EcoString>,
}

// empty() ganha lang: None (const fn preservado — Option::None é const)
```

### Diff arm `"lang"`

```rust
"lang" => {
    if let Value::Str(s) = val {
        delta.lang = Some(s);  // move directo; EcoString clone O(1)
    }
}
```

### Diff testes L3 (rotação DEBT-49)

**`debt49_set_text_lang_emite_warning` →
`debt49_set_text_alignment_emite_warning`**:
- Input: `#set text(lang: "pt")` → `#set text(alignment: "center")`.
- Assertion: `"'lang'"` → `"'alignment'"`.

**`debt49_set_text_multiplas_propriedades_desconhecidas`**:
- Trio: `font/lang/stroke` → `font/alignment/stroke`.
- Assertions actualizadas.

### Testes L1 novos (3)

- `eval_set_text_lang_passo_130`: `#set text(lang: "pt")` silent.
- `eval_set_text_lang_bcp47_composto_passo_130`:
  `#set text(lang: "en-GB")` silent — documenta aceitação de
  valores compostos.
- `eval_set_text_font_canary_passo_130`: canary (quinta iteração).

---

## 130.D — Verificação

### Cargo tests

```
test result: ok. 826 passed ...       (L1 +3 vs 823)
test result: ok. 186 passed, 6 ign    (L3 — 2 tests rotados)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 inalterado)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Manual

```bash
$ typst pt.typ    # #set text(lang: "pt")
exit=0, stderr: (vazio)

$ typst en.typ    # #set text(lang: "en-GB") — BCP 47 composto
exit=0, stderr: (vazio)

$ typst xx.typ    # #set text(lang: "xx-invalid") — divergência: sem validação
exit=0, stderr: (vazio)

$ typst f.typ     # #set text(font: "X") — canary
f.typ:1:11: warning: text: propriedade 'font' ainda não suportada
exit=0
```

4 comportamentos validados.

---

## 130.E — Encerramento

### Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `01_core/src/entities/style_chain.rs` | +import EcoString; +campo `lang`; init |
| `01_core/src/rules/eval/rules.rs` | +arm `"lang"` |
| `01_core/src/rules/eval/tests.rs` | +3 integration tests |
| `03_infra/src/integration_tests.rs` | rotação DEBT-49 (2 testes) |
| `00_nucleo/prompts/entities/style_chain.md` | actualiza StyleDelta |

### Números finais

| Métrica | Antes (129) | Depois |
|---------|------:|-------:|
| L1 tests | 823 | **826** (+3) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 (2 adaptados) |
| L4 tests | 21 | 21 |
| **Total** | **1054** | **1057** (+3) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 51 | 51 |
| DEBTs abertos | 11 | 11 (DEBT-1 subset cresce) |

---

## Limitações aceites

1. **Sem validação BCP 47**: `#set text(lang: "xx-gibberish")`
   é aceite. Vanilla rejeita. Divergência categoria semântica
   suave ADR-0033. Consumer futuro (shaping) valida/normaliza.

2. **Sem `region` / `script` / `dir`**: propriedades
   relacionadas ficam para passos dedicados. `region` em
   particular é separada em vanilla (campo próprio de `TextElem`)
   — passo XS quando chegar.

3. **Inerte em layout**: como weight/tracking/leading. Consumer
   (shaping engine, hyphenation) materializa quando chegar.

4. **`EcoString` permanece raw**: cristalino não usa tipo
   `Lang`. Trade-off:
   - (+) XS minimalista; `Value::Str` move directo.
   - (-) Sem interface para extrair language tag separado do
     region tag (`"en-GB"` → `Lang::EN + Region::GB`).
   - Resolução futura: materializar `Lang` struct se/quando
     consumer precisar de distinguir components.

---

## Lições

1. **Quinta aplicação expôs nuance**: até 129, `Value::Str` só
   aparecia em `"weight"` simbólico onde era *convertido* para
   `u16`. Em 130, `Value::Str` é *armazenado* como
   `EcoString`. Pattern variante 1 generaliza para string.

2. **Rotação DEBT-49 é custo pequeno mas recorrente**: 2
   tests adaptados por passo quando a propriedade em rotação
   sai do pool. Pool saudável mas custo acumula. Candidato
   "substituir rotativo por positivos específicos" ganha
   peso — após 131 (font) pode ser hora de executar.

3. **`alignment` como substituto temporal**: escolhido como
   próxima unknown em rotação. Alternativas eram
   `hyphenate`/`justify`. `alignment` tem vantagem de ser
   conceitualmente "centro" (propriedade text+layout comum);
   improvável ser capturada imediatamente.

4. **Const fn preservada**: `Option::None` é const para
   qualquer T, incluindo `Option<EcoString>`. Nenhum problema
   com `pub const fn empty()`.

5. **Divergência vanilla em documentação**: teste
   `eval_set_text_lang_bcp47_composto_passo_130` actua
   como documentação executável — registo de que
   `"en-GB"` é aceite sem warning. Quando validação chegar,
   este teste falha e força revisão consciente.

6. **Roadmap DEBT-1 está a agir como plano de 5 passos**:
   130/131/132/133/134. Sequência concebida no spec deste
   passo permite estimar esforço — cada um XS excepto 132
   (S: activar target `par`). Visibilidade rara em roadmap
   de DEBT — pagou-se escrever.

---

## Estado pós-Passo 130

### DEBT-1 progresso

Propriedades capturadas em `#set text`:
- ✓ bold, italic, size, fill (Passos 30/102)
- ✓ weight numérico, tracking, leading (Passos 126/127/128)
- ✓ weight simbólico (Passo 129)
- ✓ **lang (Passo 130)** — este
- ✗ font, stroke, alignment, justify, hyphenate, dir, region, ...

### Pattern DEBT-1 XS — 5 iterações, 4 variantes

| Variante | Passos exemplares |
|----------|-------------------|
| **1.** Primitivo como campo | 126 (u16), 130 (EcoString) |
| **2.** Tipo semântico L1 como campo | 127 (Length), 128 (Length com divergência) |
| **3.** Helper simbólico em tipo L1 | 129 (FontWeight::from_name) |

Pattern consolidado; próximos passos continuam a encaixar.

### Roadmap DEBT-1 (restante)

- **131 — text.font**: substituir canary DEBT-50 (único
  candidato a warning). Maior sensibilidade — canary migra
  para outra propriedade não-L1 (ex: `stroke`).
- **132 — activar target `par`** em `eval_set_rule`: S. Adiciona
  arm novo ao dispatcher; breaks 2 testes par-unknown.
- **133 — migrar `leading`** de `text` para `par`: XS. Arm
  migration.
- **134 — fechar DEBT-1** no DEBT.md: micro. Actualizar
  "PARCIALMENTE RESOLVIDO" → "RESOLVIDO" + relatório final.

Estimativa total para fechar DEBT-1: **3-4h cumulativo** a
partir deste ponto.

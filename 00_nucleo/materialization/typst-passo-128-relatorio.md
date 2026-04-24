# Passo 128 — Relatório (DEBT-1 subset: `text.leading`)

**Data**: 2026-04-24
**Precondição**: Passo 127 encerrado; 1046 total tests; zero
violations; 51 ADRs activas; 11 DEBTs abertos.
**Natureza**: passo XS em L1; **terceira aplicação consecutiva
do pattern Passos 126/127**. Pattern consolidado.
**ADR**: **sem tocar**. Terceira aplicação literal → ADR-0038
não ganha terceira nota. Divergência vanilla registada em
relatório + candidato.

---

## Sumário

`StyleDelta.leading: Option<Length>` adicionado. `eval_set_text`
captura `leading` de `#set text(leading: 0.65em)` (ou pt, ou
`abs + em`) sem warning.

**Decisão (a)** do 128.A: capturar em `#set text` em vez de
`#set par`. Diverge vanilla (onde `leading` pertence a
`ParElem`) — aceite temporariamente porque:
- `eval_set_par` não existe em L1 (target `par` continua a
  emitir warning de unknown target).
- Valor é inerte — zero impacto visível.
- Migração futura é XS (mover arm de `text` para `par`).

**Canary DEBT-50 preservado**.

**818 L1 (+3) + 24 L2 + 186 L3 + 21 L4** + 6 ignorados =
**1049 total** (+3 tests). Zero violations. **51 ADRs**
inalteradas. **11 DEBTs** — DEBT-1 subset cresce +1 propriedade.

---

## 128.A — Inventário

Completo em `00_nucleo/diagnosticos/inventario-leading-passo-128.md`.

### Findings

1. **Vanilla**: `pub leading: Length` em `model/par.rs:210` —
   pertence a `ParElem`, não `TextElem`.
2. **Cristalino** `eval_set_rule`: dispatcher com arms para
   `heading`, `page`, `figure`, fallback `text`. **`par` não é
   target válido** — emite warning de unknown target.
3. **Testes que dependem**:
   - L1 `eval_set_target_desconhecido_ignora` — input usa
     `"#set par(leading: 1em)"`; só testa `result.is_ok()`.
   - L3 `debt49_set_target_desconhecido_emite_warning` —
     input `"#set par(leading: 10pt)"`; asserta 1 warning
     com "'par'" + "target".
4. **DEBT-49 "3 desconhecidas"**: input usa `font/lang/stroke`
   — `leading` não aparece, zero rotação necessária.

### Decisão (a)

**Capturar em `#set text(leading: ...)`** — mantém XS literal,
zero ripple de testes existentes.

Alternativas (b)/(c) exigiriam:
- Adicionar `par` como target válido (+ arm dedicado).
- Adaptar 2 testes existentes que assertam "par = unknown target".
- ADR-0038 ganharia nota sobre extensão de contexto.

Custo (b)/(c) > XS → fora de escopo 128.

### Gate 128.A

**Passa**. 2 ficheiros L1 tocados. Zero ripple L3/L4 (DEBT-49
intacto; tests de `par` continuam a passar pois `par` permanece
unknown target).

---

## 128.B — ADR-0038

**Sem anotação.** Terceira aplicação idêntica ao template 126/127.
Pattern consolidado — ADR-0038 não precisa de terceira nota.

Divergência vanilla (`leading` em `text` vs `par`) é nuance
*temporal*, não *arquitectural*. Registada em relatório +
candidato futuro.

---

## 128.C — Implementação

### Diff `StyleDelta`

```rust
// + campo:
pub leading: Option<crate::entities::layout_types::Length>,

// empty() ganha `leading: None`
```

### Diff `eval_set_text`

```rust
"leading" => {
    if let Value::Length(l) = val {
        delta.leading = Some(l);
    }
}
```

Réplica exacta do arm `tracking` do Passo 127.

### Testes novos (3)

- `eval_set_text_leading_passo_128`: captura OK + zero warning
  com "'leading'".
- `eval_set_text_font_canary_passo_128`: font continua a emitir
  warning (canary, terceira iteração).
- `eval_set_par_leading_ainda_emite_warning_target_passo_128`:
  documenta que `#set par(...)` continua a emitir warning de
  target desconhecido — defende contra futura activação
  acidental.

### L3 DEBT-49 test: **intacto**

Input `font/lang/stroke` — sem rotação.

---

## 128.D — Verificação

### Cargo tests

```
test result: ok. 818 passed ...       (L1 +3 vs 815)
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
$ typst lt.typ -o lt.pdf       # #set text(leading: 0.65em)
# stderr: (vazio)
exit=0

$ typst lp.typ -o lp.pdf       # #set par(leading: 10pt)
lp.typ:1:6: warning: set: target 'par' ainda não suportado
  hint: targets suportados: heading, page, figure, text
exit=0

$ typst f.typ -o f.pdf         # #set text(font: "X")
f.typ:1:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
exit=0
```

Três comportamentos distintos como esperado:
- `text(leading)` → silent ✓
- `par(leading)` → warning de target ✓ (divergência vanilla
  documentada)
- `text(font)` → warning de propriedade ✓ (canary)

---

## 128.E — Encerramento

### Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `01_core/src/entities/style_chain.rs` | +campo `leading: Option<Length>`; init |
| `01_core/src/rules/eval/rules.rs` | +arm `"leading"` em match |
| `01_core/src/rules/eval/tests.rs` | +3 testes L1 |
| `00_nucleo/prompts/entities/style_chain.md` | actualiza StyleDelta no prompt |

### Números finais

| Métrica | Antes (127) | Depois |
|---------|------:|-------:|
| L1 tests | 815 | **818** (+3) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1046** | **1049** (+3) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 51 | 51 |
| DEBTs abertos | 11 | 11 |

---

## Limitações aceites

1. **Divergência vanilla explícita**: `leading` é capturado em
   `#set text(...)` mas canonicamente pertence a `par`. UX
   razoável (utilizador que experimenta sintaxe *errada* não
   vê warning, o que pode confundir). Mitigação: test
   `eval_set_par_leading_ainda_emite_warning_target_passo_128`
   garante que a sintaxe canónica emite warning — utilizador
   percebe que `par` não é suportado.
2. **`leading` inerte**: igual aos outros subsets (weight,
   tracking). Captura sem consumer.
3. **Pool DEBT-49 não esgotou**: input L3 continua com
   `font/lang/stroke` — três desconhecidas válidas. Próximo
   passo DEBT-1 pode precisar de rotar se tocar `stroke`.

---

## Lições

1. **Gate 128.A evitou escopo creep**: decisão (b)/(c) seria
   tentadora ("alinhar vanilla, vir depois"). Gate detectou
   que exigiria adaptar 2 testes + novo arm + possivelmente
   ADR nova — não XS. Disciplina pagou-se.

2. **Documentar divergência em test é estratégia forte**:
   `eval_set_par_leading_ainda_emite_warning_target_passo_128`
   documenta o estado actual através de assertion — quando
   alguém migrar para (b), o test falha e serve de recordação
   para adaptar.

3. **Terceira aplicação valida pattern como XS**:
   126 (u16), 127 (Length), 128 (Length com nuance contextual)
   — todos 2 ficheiros L1 + 2-3 tests. Pattern está a produzir
   XS replicáveis com zero surpresas.

4. **Canary DEBT-50 é cheap forever test**: já apareceu em 3
   passos consecutivos como salvaguarda simples. Valor
   acumulado supera o custo de escrever.

5. **"Temporal divergence" é vocabulário útil**: decisão (a)
   cria divergência *que se vai resolver* quando `eval_set_par`
   for activado. Não é divergência *permanente*. ADR-0033
   (paridade) aceita; ADR-0038 não precisa anotar porque nada
   arquitectural muda.

6. **Pool DEBT-49 está a encolher mas não esgota rápido**:
   `stroke`, `alignment`, `first-line-indent`, `justify`, etc.
   ainda estão em pool. Rotação continua disponível por mais
   alguns passos.

---

## Estado pós-Passo 128

### DEBT-1 progresso

Propriedades capturadas em `#set text`:
- ✓ bold (Passo 30)
- ✓ italic (Passo 30)
- ✓ size (Passo 30; f64 legado)
- ✓ fill (Passo 102)
- ✓ weight (Passo 126; u16)
- ✓ tracking (Passo 127; Length)
- ✓ **leading (Passo 128; Length)** — este, divergência vanilla
- ✗ font, lang, stroke, alignment, justify, ...

### Candidato futuro registado

**"Migrar `leading` para `eval_set_par` quando activado"**:
quando alguém implementar `par` como target válido (passo
dedicado), adicionar arm que também captura `leading`. Arm em
`text` pode ser mantido (dual-write) ou removido (single-source).
Decisão ao executar.

### Pattern L1 — três iterações confirmadas

```rust
// 1. StyleDelta: +campo Option<T>
// 2. empty(): +init None
// 3. eval_set_text match: +arm capturador
// 4. tests.rs: +2-3 tests (captura + canary + opcional-contexto)
```

126 (`u16`), 127 (`Length`), 128 (`Length` divergente) —
template estável. Próxima aplicação (quando for) deve correr
em < 30 minutos.

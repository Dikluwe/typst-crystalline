# Passo 134 — Relatório (migrar `leading` de `text` para `par`)

**Data**: 2026-04-24
**Precondição**: Passo 133 encerrado; 1083 total tests; zero
violations; 53 ADRs activas; 11 DEBTs abertos; target `par`
activado sem arms.
**Natureza**: passo XS em L1. Semântica de propriedade (não
infra). Fecha divergência `leading` introduzida no 128.
**ADR**: **não tocada**. Migração semântica absorve-se na
ADR-0033 existente; sem nova decisão arquitectural.

---

## Sumário

Arm `"leading"` movido do bloco `text` para o bloco `par` em
`eval_set_rule`. `#set par(leading: 0.65em)` é agora silent
(captura canónica); `#set text(leading: 0.65em)` passa a
emitir warning de propriedade não suportada (divergência
temporal do Passo 128 **fechada**).

Helper `unsupported_property_warn` parametrizado com
`adr_ref: Option<&str>` — `text` continua a referenciar
ADR-0040; `par` passa `None` (hint genérico).

**853 L1 (+1) + 24 L2 + 186 L3 + 21 L4** + 6 ignorados =
**1084 total** (+1 vs 1083). Zero violations.

---

## 134.A — Inventário confirmatório

| Item | Resultado |
|------|-----------|
| A.1 arm `"leading"` | ✓ `rules.rs:351` dentro do bloco `text` |
| A.2 helper | ✓ `fn unsupported_property_warn(target, field)` com hint hardcoded "ADR-0040" |
| Call sites | 2: par (linha 281, 133) + text (linha 430) |
| A.3 tests L1 com `leading:` | 2: `eval_set_text_leading_passo_128` + `eval_set_par_known_target_sem_arms_passo_133` (este usa `par(leading:)`) |
| A.4 tests L3/L4 | **zero** — nenhum usa `#set text(leading:` |
| A.6 hint para `par` | Opção d escolhida: `Option<&str>` com `None` → hint genérico |

**Gate passou**. Zero bloqueios.

---

## 134.B — Fix helper

### Diff

```rust
// antes:
fn unsupported_property_warn(target: &str, field: &str) -> (String, String) {
    (
        format!("{target}: propriedade '{field}' ainda não suportada"),
        format!("ver ADR-0040 para propriedades cobertas por set {target}"),
    )
}

// depois (Opção d — parâmetro Option<&str>):
fn unsupported_property_warn(
    target: &str,
    field: &str,
    adr_ref: Option<&str>,
) -> (String, String) {
    let msg = format!("{target}: propriedade '{field}' ainda não suportada");
    let hint = match adr_ref {
        Some(adr) => format!("ver ADR-{adr} para propriedades cobertas por set {target}"),
        None      => format!("propriedades de set {target} ainda não são capturadas"),
    };
    (msg, hint)
}
```

### Call sites actualizados

```rust
// par (linha 281):
unsupported_property_warn("par", &key, None)

// text (linha 430):
unsupported_property_warn("text", &key, Some("0040"))
```

---

## 134.C — Migração do arm `leading`

### Removido do bloco `text`

O arm que capturava `leading` foi removido do loop de args do
bloco `text`. Agora `leading` em text cai no fallback default
→ warning "text: propriedade 'leading' ainda não suportada".

### Adicionado ao bloco `par`

Bloco `par` convertido de "só warning fallback" para match
sobre `key` com arm dedicado:

```rust
if target == "par" {
    // Passo 134: arm leading migrado de text para par.
    let mut delta = StyleDelta::empty();
    for arg in set.args().items() {
        if let Arg::Named(named) = arg {
            let key = named.name().as_str().to_owned();
            let val = eval_expr(named.expr(), scopes, ctx, engine)?;
            match key.as_str() {
                "leading" => {
                    if let Value::Length(l) = val {
                        delta.leading = Some(l);
                    }
                }
                _ => {
                    let (msg, hint) = unsupported_property_warn("par", &key, None);
                    engine.sink.warn_note(
                        named.name().to_untyped().span(),
                        &msg,
                        &hint,
                    );
                }
            }
        }
    }
    *engine.styles = engine.styles.push(delta);
    return Ok(Value::None);
}
```

**Semântica idêntica ao bloco text** (eval expr, match,
push delta), mas limitado a 1 arm.

---

## 134.D/E — Tests invertidos + novo

### Test 128 renomeado + invertido

`eval_set_text_leading_passo_128` →
`eval_set_text_leading_emite_warning_passo_134`:
- Antes: assertava captura silent em text.
- Depois: asserta warning `"text: propriedade 'leading'"`.

### Test 133 adaptado

`eval_set_par_known_target_sem_arms_passo_133` →
`eval_set_par_known_target_com_leading_passo_134`:
- Antes: assertava `par` emite warning de `'leading'` (sem arms).
- Depois: usa `justify` (ainda fallback) para asserting par
  emite warning de propriedade. `leading` é silent agora.

### Test novo

`eval_set_par_leading_captura_passo_134`:
- Input `#set par(leading: 0.65em)`.
- Asserta `result.is_ok()` + **sem warning** para `'leading'`.

### Delta L1

- Teste 128 renomeado/invertido (0 net).
- Teste 133 adaptado (0 net).
- +1 teste novo (`eval_set_par_leading_captura_passo_134`).
- **Líquido: +1**. L1: 852 → 853. ✓

---

## 134.F — Canary preservado

`eval_set_text_hyphenate_canary_passo_132b` passa sem mudança.
Validação de regressão implícita: mecanismo de warning para
propriedades não suportadas continua a operar.

---

## 134.G — L3/L4

**Zero tests afectados** (confirmado em 134.A.4). Pool DEBT-49
L3 continua `hyphenate/alignment/stroke`.

---

## 134.H — Prompts L0

`rules/eval.md` não enumera propriedades capturadas —
`crystalline-lint --fix-hashes` reportou `Nothing to fix`.

---

## 134.I — Verificação

### Cargo tests

```
test result: ok. 853 passed ...       (L1 +1 vs 852)
test result: ok. 186 passed, 6 ign    (L3 inalterado)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 inalterado)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Manual — 4 cenários

```bash
$ typst pl.typ     # #set par(leading: 0.65em)
exit=0, stderr: (vazio)

$ typst tl.typ     # #set text(leading: 0.65em)
tl.typ:1:11: warning: text: propriedade 'leading' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
exit=0

$ typst pj.typ     # #set par(justify: true) — hint novo
pj.typ:1:10: warning: par: propriedade 'justify' ainda não suportada
  hint: propriedades de set par ainda não são capturadas
exit=0

$ typst h.typ      # canary hyphenate
h.typ:1:11: warning: text: propriedade 'hyphenate' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
exit=0
```

Quatro comportamentos validados. Hint diferenciado para `par`
vs `text` visível.

---

## Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `01_core/src/rules/eval/rules.rs` | helper parametrizado; arm leading de text → par; bloco par ganha match com captura |
| `01_core/src/rules/eval/tests.rs` | test 128 renomeado+invertido; test 133 adaptado; +1 novo test |

### Números finais

| Métrica | Antes (133) | Depois |
|---------|------:|-------:|
| L1 tests | 852 | **853** (+1) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1083** | **1084** (+1) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 53 | 53 |
| DEBTs abertos | 11 | 11 |

---

## Mudança observável

### Utilizadores com `#set par(leading: ...)`

- **Antes (128)**: warning "target 'par' ainda não suportado".
- **133**: warning "par: propriedade 'leading' ainda não suportada".
- **Depois (134)**: **silent** — captura canónica vanilla.

### Utilizadores com `#set text(leading: ...)`

- **Antes (128)**: silent (capturado em `text` com divergência
  documentada).
- **Depois (134)**: warning "text: propriedade 'leading' ainda
  não suportada".

### Hint diferenciado por target

- `text`: `hint: ver ADR-0040 para propriedades cobertas por set text`.
- `par`: `hint: propriedades de set par ainda não são capturadas`.

Pequena melhoria de UX — utilizador de `par` deixa de ver
referência absurda a ADR-0040 (que cobre `text`).

---

## Divergência `leading` fechada

| Passo | Estado `leading` |
|------:|------------------|
| 128 | Capturado em `text` (divergência temporal aceite) |
| 133 | `par` activado como known target (infra) |
| **134** | **Migrado para `par` — paridade vanilla obtida** |

Divergência introduzida no 128 **totalmente fechada**. Teste
invertido serve como contrato executável da nova semântica.

---

## Lições

1. **Migração semântica é XS limpo quando infra está pronta**:
   passo 134 foi 3 edits pontuais em 2 ficheiros. A preparação
   do 133 (infra) tornou este passo mecânico. Split 133/134
   foi acertado — infra e semântica têm riscos diferentes e
   isolá-los facilita debugging.

2. **Opção d para hint parametrizado paga-se**: alternativa
   era deixar hint hardcoded. Com `None` para par, utilizador
   recebe mensagem correcta. Quando ADR dedicada de `par` for
   criada, call site muda de `None` para `Some("XXXX")` — 1
   linha futura.

3. **"Inverter teste em vez de deletar" terceira aplicação**:
   128→134 inversão + 128→133 renomeação (anterior) +
   130→131B inversão. Pattern consolidado. Teste antigo
   deixaria "morto" no código; **invertido** torna-se
   contrato activo.

4. **Test 133 ficou obsoleto rapidamente**: `_sem_arms_passo_133`
   referenciava "sem arms" mas 134 adicionou 1 arm. Renomeado
   para `_com_leading_passo_134` e input mudou de `leading`
   (capturado) para `justify` (continua fallback). Sinal de
   que o nome do teste deve reflectir o invariante, não o
   passo — candidato para convenção.

5. **Valor inerte (sem consumer) é invariante**: `leading`
   continua a não afectar layout. Captura é pura — utilizador
   vê Typst aceitar o código sem feedback visual. Quando
   consumer de layout chegar (passo dedicado), resolve-se.

6. **2 testes em 2 ficheiros + 1 ficheiro helper = ritmo XS
   real**: 45min aproximado. Preparação 133 + 132 amortizam
   complexidade deste passo.

---

## Estado pós-Passo 134

### DEBT-1 — estado da lista canónica

- ✓ `text.font` (132B; dict deferido, ADR-0053).
- ✓ `text.lang` (131B; tipo Lang, ADR-0052).
- ✓ `par.leading` (134; contexto correcto, paridade vanilla).
- ✓ `text.weight` numérico + simbólico (126/129).

**Lista canónica 100% capturada no contexto canónico** com
única ressalva documentada (`font` dict deferido, registado
em ADR-0053 como trade-off consciente).

### Pronto para Passo 135

Último passo de fecho DEBT-1: **actualização do DEBT.md**.
- Mover DEBT-1 da secção "abertos" para "encerrados".
- Anotar Passo 135 como ponto de fecho.
- Registar data e trail (131B/132B/133/134).

Estimativa 135: **micro** (~15-30min; só documentação).

### DEBTs abertos após 134

Continua em **11** — DEBT-1 ainda não fechado formalmente
(só o será no 135).

### Candidatos futuros registados

1. **Consumer de propriedades de StyleDelta em layout**:
   shaping engine consome `font`, `weight`, `tracking`,
   `leading`. Grande trabalho, passo dedicado.
2. **ADR-0054 (potencial)**: autorizar `regex` em L1 +
   `Covers` concreto para dict form de `font`.
3. **Extract helper `eval_with_warnings`** no test harness.
   Custo crescente — cada teste duplica ~15 linhas.
4. **ADR dedicada para propriedades de `par`**: quando
   primeira propriedade de `par` for capturada que requer
   decisão arquitectural, cria-se ADR-0055 (ou próximo
   número) e call site muda `None` → `Some("XXXX")`.

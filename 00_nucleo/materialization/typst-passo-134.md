# Passo 134 — Migrar `leading` de `text` para `par`

**Série**: 134 (passo **XS** em L1; penúltimo passo para fechar
DEBT-1).
**Precondição**: Passo 133 encerrado; 1083 total tests; zero
violations; 53 ADRs activas; 11 DEBTs abertos. Target `par` é
known em `eval_set_rule`; bloco `par` existe mas sem arms.
`leading` continua a ser capturado em `#set text` (divergência
temporal do Passo 128).

**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional) — este passo resolve a
  divergência `leading` criada no 128.
- **ADR-0038** — esperado: sem nota (semântica migra, pattern
  não introduz variante nova).
- **ADR-0040** — referenciada por helper `unsupported_property_warn`.
  Fix cosmético incluído.

**Natureza**: passo L1. Move arm `"leading"` de bloco `text`
para bloco `par`. Resolve divergência vanilla do Passo 128.
Inverte 1 teste (text) + cria 1 teste novo (par). Fix cosmético
do helper de warning.

Pattern DEBT-1 XS **aplica parcialmente** — é semântica da
propriedade, mas migração (mudança de contexto), não primeira
captura.

---

## Contexto

Passo 128 capturou `leading` em `#set text` como **divergência
temporal aceite** — `par` não era target conhecido. Registado
como candidato futuro: "migrar `leading` para `eval_set_par`
quando activado".

Passo 133 activou `par` sem arms concretos. Este passo
completa a transição: `leading` passa a viver onde pertence
canonicamente.

Este é o último passo estrutural antes de fechar DEBT-1
(Passo 135).

---

## Contexto estratégico

Penúltimo passo de fecho de DEBT-1:

- **133** (feito): target `par` activado como known.
- **134** (este): migrar `leading` de text para par.
- **135**: fechar DEBT-1 formalmente em `DEBT.md`.

Após 134, **toda a lista canónica DEBT-1 está capturada no
contexto correcto**, com as ressalvas documentadas:
- `font` dict deferido (ADR-0053, divergência consciente com
  ADR dedicada).

A divergência `leading` **fecha neste passo**.

---

## Decisões já tomadas (entrada do passo)

1. **Remover arm `"leading"` do bloco `text`**. Não manter
   dual-capture. `#set text(leading: ...)` passa a emitir
   warning "leading não suportada em text".
2. **Fix cosmético do hint no helper** `unsupported_property_warn`:
   passar ADR de referência como parâmetro (actualmente
   hardcoded em "ADR-0040"). `text` continua a usar 0040;
   `par` usa valor diferente (a decidir em 134.A conforme
   contexto — pode ser outra ADR ou string sem referência
   específica).

---

## Objectivo

Ao fim do passo:

1. Arm `"leading"` existe **apenas** no bloco `par` de
   `eval_set_rule`.
2. `#set par(leading: 0.65em)` é silent, captura em
   `StyleDelta.leading`.
3. `#set text(leading: 0.65em)` emite warning "leading não
   suportada em text".
4. Helper `unsupported_property_warn` aceita ADR de referência
   como parâmetro (fix cosmético).
5. Teste L1 do 128 **invertido** para assertar warning em vez
   de silent.
6. Novo teste L1 asserta captura positiva em `par`.
7. Canary DEBT-50 (`hyphenate`) preservado.
8. `cargo test --workspace` passa. `crystalline-lint` zero.

Este passo **não**:

- Adiciona outras propriedades ao bloco `par`.
- Toca `StyleDelta.leading` (campo e tipo inalterados).
- Resolve o hint absurdo de forma mais ampla (se outros arms
  usam o helper com outras combinações, o fix acomoda mas não
  audita sistematicamente).
- Fecha DEBT-1 (Passo 135).

---

## Escopo

**Dentro**:
- `01_core/src/rules/eval/rules.rs`:
  - Arm `"leading"` removido do bloco `text`.
  - Arm `"leading"` adicionado ao bloco `par`.
  - Helper `unsupported_property_warn` — assinatura adaptada.
  - Call sites do helper — passar ADR referência.
- `01_core/src/rules/eval/tests.rs`:
  - `eval_set_text_leading_passo_128` invertido + renomeado.
  - Novo teste `eval_set_par_leading_captura_passo_134`.
- `03_infra/src/integration_tests.rs` (se algum teste usa
  `#set text(leading: ...)` como válido).
- `00_nucleo/prompts/rules/eval.md` (se referencia leading
  em text).

**Fora**:
- Novas propriedades de `par`.
- L2, L4.
- ADR nova.

---

## Sub-passos

### 134.A — Inventário confirmatório

**A.1 — Localizar arm `"leading"` actual**:

`grep -n "\"leading\"" 01_core/src/rules/eval/rules.rs`

Esperado: arm dentro do bloco `if target == "text"` (ou
equivalente). Registar linhas.

**A.2 — Assinatura actual de `unsupported_property_warn`**:

`grep -n "unsupported_property_warn\|fn unsupported" 01_core/src/rules/eval/rules.rs`

Ler corpo. Registar:
- Parâmetros actuais.
- Como o hint "ADR-0040" é construído.
- Call sites (quantos blocos invocam — esperado: text, par).

**A.3 — Testes L1 afectados**:

Listar testes que usam `#set text(leading: ...)`:

`grep -n "leading:" 01_core/src/rules/eval/tests.rs`

Esperado: pelo menos `eval_set_text_leading_passo_128`.
Registar outros se existem.

**A.4 — Testes L3/L4 afectados**:

`grep -rn "#set text(leading:" 03_infra/ 04_wiring/`

Se algum teste L3/L4 usa `#set text(leading: ...)` como input
válido (esperando silent), adaptar.

**A.5 — Contagem base**:
- L1: 852.
- Total: 1083.

**A.6 — Decisão do hint para `par`**:

Dado que ADR-0040 é sobre `#set text`, o hint para `par`
precisa de:
- Opção a: hint sem referência a ADR (`"ver documentação"` ou
  remover o hint).
- Opção b: ADR nova específica para `par` (fora de escopo deste
  passo).
- Opção c: hint com referência genérica
  (`"ver ADRs da secção set rules"`).
- Opção d: parametrizar mas passar `None` para `par`, construindo
  mensagem sem hint quando param é `None`.

Recomendação pragmática: **opção d**. `par` passa `None`;
`text` continua a passar `"0040"`. Mantém a info útil para text
sem absurdo em par. Decisão final no relatório.

**Gate 134.A**:
- Se A.1 revela arm `"leading"` em sítio inesperado (não bloco
  text): investigar antes.
- Se A.2 revela que `unsupported_property_warn` é mais complexo
  do que assumido (ex: hardcoded em múltiplos sítios): adaptar
  plano.
- Se A.4 revela testes L3/L4 com `#set text(leading: ...)`
  silent: cada um precisa adaptação — estimativa cresce.
- Outros casos: prosseguir.

### 134.B — Fix do helper

**Ficheiro**: `01_core/src/rules/eval/rules.rs`.

Adaptar assinatura conforme Opção d (recomendada):

```rust
// antes (esperado):
fn unsupported_property_warn(target: &str, prop: &str) -> (String, String) {
    let msg = format!("{}: propriedade '{}' ainda não suportada", target, prop);
    let hint = format!("ver ADR-0040 para propriedades cobertas por set {}", target);
    (msg, hint)
}

// depois:
fn unsupported_property_warn(
    target: &str,
    prop: &str,
    adr_ref: Option<&str>,
) -> (String, String) {
    let msg = format!("{}: propriedade '{}' ainda não suportada", target, prop);
    let hint = match adr_ref {
        Some(adr) => format!(
            "ver ADR-{} para propriedades cobertas por set {}",
            adr, target,
        ),
        None => format!(
            "propriedades de set {} ainda não são capturadas",
            target,
        ),
    };
    (msg, hint)
}
```

Call sites (actualizar):
- Bloco `text`: `unsupported_property_warn("text", &key, Some("0040"))`.
- Bloco `par`: `unsupported_property_warn("par", &key, None)`.

Nota: a assinatura exacta depende de 134.A.2. Se o helper tem
outras responsabilidades (ex: span, formato diferente), adaptar.

### 134.C — Mover arm `"leading"`

**Ficheiro**: `01_core/src/rules/eval/rules.rs`.

**C.1 — Remover do bloco `text`**:

```rust
// dentro de if target == "text":
// REMOVER este arm:
"leading" => {
    if let Value::Length(l) = val {
        delta.leading = Some(l);
    }
}
```

Após remoção, `leading` em `text` cai no fallback:
`unsupported_property_warn("text", "leading", Some("0040"))`.

**C.2 — Adicionar ao bloco `par`**:

```rust
// dentro de if target == "par" (criado em 133):
// ANTES:
for arg in set.args().items() {
    if let Arg::Named(named) = arg {
        let key = named.name().as_str().to_owned();
        let (msg, hint) = unsupported_property_warn("par", &key, None);
        engine.sink.warn_note(
            named.name().to_untyped().span(),
            &msg,
            &hint,
        );
    }
}

// DEPOIS:
for arg in set.args().items() {
    if let Arg::Named(named) = arg {
        let key = named.name().as_str();
        let val = /* avaliar named.expr() conforme pattern existente */;
        match key {
            "leading" => {
                if let Value::Length(l) = val {
                    delta.leading = Some(l);
                }
            }
            _ => {
                let (msg, hint) = unsupported_property_warn(
                    "par", key, None,
                );
                engine.sink.warn_note(
                    named.name().to_untyped().span(),
                    &msg,
                    &hint,
                );
            }
        }
    }
}
return Ok(Value::None);
```

Nota: se a avaliação de `named.expr()` precisa de contexto
(ex: engine state), seguir pattern exacto do bloco `text`
confirmado em 134.A.

### 134.D — Inverter teste L1 do 128

**Ficheiro**: `01_core/src/rules/eval/tests.rs`.

```rust
// antes (128):
#[test]
fn eval_set_text_leading_passo_128() {
    let src = "#set text(leading: 0.65em)";
    let (delta, warnings) = eval_inline(src);
    assert_eq!(delta.leading, Some(/* 0.65em */));
    assert!(warnings.iter().all(|w| !w.message.contains("'leading'")));
}

// depois (134):
#[test]
fn eval_set_text_leading_emite_warning_passo_134() {
    // Passo 134 migrou `leading` para `par` (ADR-0033 paridade).
    // `#set text(leading: ...)` passa a emitir warning de
    // propriedade não suportada em text.
    let src = "#set text(leading: 0.65em)";
    let (delta, warnings) = eval_inline(src);
    assert_eq!(delta.leading, None);
    assert!(warnings.iter().any(|w|
        w.message.contains("'leading'")
        && w.message.contains("text")
    ));
}
```

### 134.E — Novo teste L1 captura em `par`

```rust
#[test]
fn eval_set_par_leading_captura_passo_134() {
    // Captura positiva em par — contexto canonicamente correcto
    // após migração do 134.
    let src = "#set par(leading: 0.65em)";
    let (delta, warnings) = eval_inline(src);
    assert_eq!(delta.leading, Some(/* Length 0.65em */));
    assert!(warnings.iter().all(|w| !w.message.contains("'leading'")));
}
```

### 134.F — Canary preservado

`eval_set_text_hyphenate_canary_passo_132b` (consolidado em
132B) continua a passar sem mudança. Regressão-check implícito.

### 134.G — Adaptar testes L3/L4 se aplicável

Conforme 134.A.4. Se algum teste usa `#set text(leading: ...)`
esperando silent, inverter input ou assertion.

Esperado: **zero testes** precisam de migração em L3/L4 — o
`leading` no 128 foi capturado só em L1. Confirmar.

### 134.H — Prompts L0

Se `rules/eval.md` referencia `leading` como propriedade de
`text`, corrigir para `par`. `crystalline-lint --fix-hashes`.

### 134.I — Verificação

1. `cargo test -p typst-core` — L1: 852 → **853** (+1: teste
   novo de captura em par; teste 128 renomeado in-place).

2. `cargo test --workspace` — total ≥ 1084.

3. `crystalline-lint` zero violations.

4. Manual:

```bash
$ cat pl.typ
#set par(leading: 0.65em)
Texto
$ typst pl.typ -o pl.pdf
exit=0, stderr: (vazio)

$ cat tl.typ
#set text(leading: 0.65em)
Texto
$ typst tl.typ -o tl.pdf
tl.typ:1:11: warning: text: propriedade 'leading' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
exit=0

$ cat pj.typ
#set par(justify: true)       # par unknown property
Texto
$ typst pj.typ -o pj.pdf
pj.typ:1:10: warning: par: propriedade 'justify' ainda não suportada
  hint: propriedades de set par ainda não são capturadas
exit=0

$ cat h.typ
#set text(hyphenate: true)     # canary
Texto
$ typst h.typ -o h.pdf
h.typ:1:11: warning: text: propriedade 'hyphenate' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
exit=0
```

4 comportamentos correctos.

### 134.J — Encerramento

Relatório em `typst-passo-134-relatorio.md`:

- Inventário 134.A (localização actual, assinatura do helper,
  decisão de hint para par).
- Diff do helper (3 opções consideradas, escolhida).
- Diff dos arms (removido de text, adicionado em par).
- Tests inverted + novo.
- Resultado final.
- **Divergência `leading` fechada**: 128 criou, 134 resolveu.
- Preparação para 135 (fechar DEBT-1).

---

## Critério de conclusão

1. Inventário 134.A escrito.
2. Helper `unsupported_property_warn` aceita `adr_ref: Option<&str>`.
3. Arm `"leading"` removido do bloco `text`.
4. Arm `"leading"` adicionado ao bloco `par` com captura.
5. Teste do 128 invertido para assertar warning.
6. Novo teste L1 `eval_set_par_leading_captura_passo_134`.
7. Canary DEBT-50 (`hyphenate`) preservado.
8. L1 tests: **853** (+1 net).
9. `cargo test --workspace` passa (≥ 1084).
10. `crystalline-lint` zero violations.
11. Teste manual confirma 4 cenários.
12. Relatório 134.J escrito.

---

## O que pode sair errado

- **Arm `"leading"` está noutro sítio do que o esperado**
  (ex: função auxiliar de text): ajustar plano.

- **Helper `unsupported_property_warn` tem múltiplos call
  sites além de text e par**: fix da assinatura propaga-se.
  Baixo risco pelo levantamento de 133 mas possível.

- **Teste 128 depende de mais do que apenas silent behavior**
  (ex: verifica tipo exacto do Length capturado): inversão
  precisa de mais que alterar assertions — pode precisar de
  refactor completo do teste.

- **`#set text(leading: ...)` aparece em código de exemplo
  nos prompts L0**: corrigir para `par`.

- **L3/L4 têm tests silent com `#set text(leading:)`**:
  inesperado pelo levantamento mas possível. Adaptar.

- **Avaliação de `named.expr()` em bloco par requer mais
  contexto do que bloco text fornece**: se há engine state
  ou scope necessário, replicar pattern. Improvável ser
  diferente entre blocos.

---

## Notas operacionais

- **Fecho de divergência**: este passo resolve o único item
  estrutural pendente antes do fecho de DEBT-1. 135 fica
  como pura documentação.

- **Pattern "inverter teste em vez de remover"**: terceira
  aplicação (128→133, 130→131B, agora 128→134). Consistência
  valida a estratégia como default.

- **Fix cosmético do helper**: acoplado à migração para evitar
  "helper não reflecte estado". 2 linhas extra no helper + 2
  call sites é proporcional ao benefício (mensagem correcta
  para quem lê o warning de `par`).

- **Opção d escolhida para hint**: `None` para par é melhor
  que referenciar ADR errada. Quando propriedades de par
  forem capturadas e houver ADR-XXXX dedicada, altera-se o
  call site do par para `Some("XXXX")`.

- **Consumer de `leading` continua inexistente**: o valor é
  capturado mas inerte (layout ainda não o consome). Igual
  aos outros campos de `StyleDelta` sem consumer.

- **Ritmo estimado**: XS, ≈ 45min-1h. Pequeno porque o 133
  preparou o terreno. Risco principal é o helper refactor
  acidentalmente tocar outros call sites.

- **Candidato `eval_with_warnings`** continua pendente.
  Priorizar após 135.

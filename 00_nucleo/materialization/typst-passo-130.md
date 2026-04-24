# Passo 130 — DEBT-1 subset: `text.lang`

**Série**: 130 (passo XS em L1; **quinta aplicação consecutiva
do pattern DEBT-1 XS**).
**Precondição**: Passo 129 encerrado; 1054 total tests (823 L1 +
24 L2 + 186 L3 + 21 L4 + 6 ignorados); zero violations; 51 ADRs
activas (ADR-0038 com 3 notas); 11 DEBTs abertos. DEBT-1 subset
com 8 propriedades activas (bold, italic, size, fill, weight
numérico, tracking, leading, weight simbólico).
**ADRs aplicáveis**: ADR-0038 (Style/Styles/StyleChain; 3 notas —
Passos 126, 127, 129), ADR-0040 (#set text activo via bake-in).
**ADR tocada**: decidida em 130.A (esperado: nenhuma, quinta
aplicação literal).

**Contexto estratégico**: primeiro de 5 sub-passos para fechar
DEBT-1. Sequência prevista:
- **130**: `text.lang` (este).
- **131**: `text.font` + substituição canary DEBT-50.
- **132**: activar target `par` em `eval_set_rule` (S).
- **133**: migrar `leading` de `text` para `par` (XS).
- **134**: fechar DEBT-1 formalmente no DEBT.md (micro).

---

## Objectivo

Adicionar `lang: Option<EcoString>` a `StyleDelta` (tipo
confirmado em 130.A). `eval_set_text` captura `lang` de
`#set text(lang: "pt")` sem warning, sem validação de formato
BCP 47.

Ao fim do passo:

1. `StyleDelta.lang: Option<EcoString>` adicionado.
2. `eval_set_text` captura arm `"lang"` de `Value::Str`.
3. Canary `font` (DEBT-50) preservado — quinta iteração.
4. 2-3 testes L1 novos (captura + canary + opcional regressões).
5. Efeito visível no PDF: nenhum (documentado, como 126–129).

Este passo **não**:
- Valida formato BCP 47 (divergência consciente; vanilla valida,
  cristalino aceita tudo e ignora no consumer futuro).
- Adiciona outras propriedades.
- Implementa consumo em layout (shaping, hyphenation, direction
  por lang).
- Toca `region` (propriedade relacionada mas separada em vanilla).
- Toca L2, L3, L4 (excepto rotação DEBT-49 se pool exigir).

---

## Decisões já tomadas

1. **Só `lang` neste passo** — disciplina dos 126–129 reiterada.
2. **Tipo `EcoString`** — confirmado em 130.A.1. Vanilla usa
   `EcoString` para identificadores de lang (BCP 47 code).
3. **Captura sem validação** — replica pattern 126: valor
   chega, captura directa, zero normalização.
4. **Nome de propriedade `"lang"`** — alinhado vanilla. Não
   `"language"`.

## Decisões diferidas (130.A)

5. **Tipo exacto em `StyleDelta`**: confirmar se é `EcoString` ou
   `String`. Vanilla usa `EcoString` (ecow crate). Em L1, `ecow`
   está autorizado (ADR-0018 permite crates puras; `ecow`
   explicitamente documentado no CLAUDE.md). Preferência:
   `EcoString`.

6. **Default behavior**: `StyleDelta.lang` inicializa como `None`
   em `empty()`. Nenhum default semântico neste passo — quando
   consumer chegar, `None` significa "herdado da cadeia" (igual
   a outras propriedades Option).

---

## Escopo

**Dentro**:
- `01_core/src/entities/style_chain.rs` (ou equivalente) — campo
  `lang: Option<EcoString>`.
- `01_core/src/rules/eval/rules.rs` — arm `"lang"` em
  `eval_set_text`.
- `01_core/src/rules/eval/tests.rs` — 2-3 testes.
- Prompt L0 correspondente se existe + hash.

**Fora**:
- `region` (propriedade separada).
- Validação BCP 47.
- Consumer em layout.
- `eval_set_par` / outras propriedades.
- L2, L3 (excepto DEBT-49 rotação), L4.

---

## Sub-passos

### 130.A — Inventário

**Parte 1 — Vanilla `lang` em `TextElem`**:

1. `grep -rn "pub lang" lab/typst-original/crates/typst-library/src/text/`
   (procurar campo `lang` em `TextElem` ou equivalente).
2. Registar:
   - Tipo exacto (esperado: `EcoString` ou similar).
   - Default (esperado: `"en"`).
   - Se há validação no `FromValue` (BCP 47 format check).
   - Se há propriedades relacionadas (`region`, `script`, `dir`).

**Parte 2 — `StyleDelta` actual em L1**:

1. `grep -n "pub struct StyleDelta" 01_core/src/entities/style_chain.rs`
   (ou outro caminho se diferente).
2. Registar:
   - Campos actuais (esperado: bold, italic, size, fill, weight,
     tracking, leading).
   - Tipo que deve ser adicionado — confirmar que `EcoString` é
     autorizado e já é usado (Passo 99 materializou Styles com
     EcoString provavelmente).

**Parte 3 — Import de `EcoString` em L1**:

1. `grep -rn "use ecow" 01_core/src/entities/` e `rules/eval/`.
2. Registar se `EcoString` já é importado no ficheiro onde o arm
   vai. Se não, anotar o import a adicionar.

**Parte 4 — Pool DEBT-49**:

1. `grep -n "lang" 03_infra/src/integration_tests.rs`.
2. Confirmar se `debt49_set_text_multiplas_propriedades_desconhecidas`
   (ou variante L3) usa `lang` como propriedade desconhecida.
3. Esperado pelo relatório 129: pool usa `font/lang/stroke`.
   **Se pool inclui `lang`, rotação obrigatória** — substituir
   por `stroke`, `alignment`, ou `hyphenate`.

**Parte 5 — Registo do inventário**:

Escrever em
`00_nucleo/diagnosticos/inventario-lang-passo-130.md`:

```
Vanilla lang:
  struct: TextElem
  tipo: <EcoString/String>
  default: "<en>"
  validação BCP 47: sim/não
  propriedades relacionadas: [region, script, dir, ...]

StyleDelta actual:
  campos: [bold, italic, size, fill, weight, tracking, leading]
  localização: 01_core/src/entities/style_chain.rs:XXX

EcoString disponível:
  importado em rules/eval/rules.rs: sim/não
  importado em entities/style_chain.rs: sim/não

DEBT-49 pool:
  inclui lang: sim/não
  rotação necessária: <propriedade substituta>

Divergência vanilla:
  cristalino não valida BCP 47 → captura tudo como string
  categoria ADR-0033: semântica suave
  aceita-se: sim (consumer futuro valida/normaliza)

Decisão tipo:
  EcoString (preferido) / String
  razão: [...]
```

**Gate 130.A**:

- Se pool DEBT-49 esgotou (4 propriedades conhecidas + 3
  desconhecidas no input, e `lang` vira conhecida restando só
  2 desconhecidas na rotação) → **parar e reportar**. Substituir
  teste rotativo por positivos específicos é passo dedicado.
- Se vanilla `TextElem.lang` tem tipo composto (ex: `Lang` struct
  com validação interna em vez de `EcoString` plana) → **parar e
  discutir**. Pode exigir materializar tipo.
- Se total linhas esperadas > 30, reportar. Limite XS.

### 130.B — ADR (condicional)

**Esperado**: sem anotação. Quinta aplicação literal do pattern
126/127/129 (variante 1 ou 2 — primitivo/tipo semântico como
campo). ADR-0038 já tem 3 notas a cobrir o espaço.

**Se 130.A revela divergência semântica** (ex: vanilla faz
validação e cristalino não) → anotar em ADR-0033 ou abrir
diagnóstico.

### 130.C — Implementação

**130.C.1 — `StyleDelta`**:

```rust
// + campo:
pub lang: Option<EcoString>,

// empty() ganha `lang: None`
```

Import de `EcoString` se não existe. Esperado: já está importado
por outros campos (ex: font quando chegar, ou outros já lá).

**130.C.2 — Arm no eval**:

```rust
"lang" => {
    if let Value::Str(s) = val {
        delta.lang = Some(s);
    }
}
```

Réplica literal do pattern variante 1. Se `Value::Str` já
contém `EcoString` (confirmar em 130.A.2), cast directo. Se
contém `&str` ou `String`, adaptar.

Nota: possível que `Value::Str` já seja wrapper de `EcoString`
(Passo 24 activou `ecow` para `Value::Str` por ADR-0024). Se
sim, cast é `s` directo; se é wrapper, `s.into()` ou
`.as_str().into()`.

**130.C.3 — Testes novos (L1)**:

```rust
#[test]
fn eval_set_text_lang_passo_130() {
    let src = r#"#set text(lang: "pt")"#;
    let (delta, warnings) = eval_inline(src);
    assert_eq!(delta.lang, Some(EcoString::from("pt")));
    assert!(warnings.iter().all(|w| !w.message.contains("'lang'")));
}

#[test]
fn eval_set_text_lang_bcp47_composto_passo_130() {
    // Documenta por assertion: valores BCP 47 compostos (ex: "en-GB",
    // "zh-Hant") são aceites literalmente sem validação.
    let src = r#"#set text(lang: "en-GB")"#;
    let (delta, warnings) = eval_inline(src);
    assert_eq!(delta.lang, Some(EcoString::from("en-GB")));
    assert!(warnings.iter().all(|w| !w.message.contains("'lang'")));
}

#[test]
fn eval_set_text_font_canary_passo_130() {
    // Canary DEBT-50: font continua a emitir warning (quinta iteração).
    let src = "#set text(font: \"X\")\nOlá";
    let (_, warnings) = eval_inline(src);
    assert!(warnings.iter().any(|w| w.message.contains("'font'")));
}
```

**130.C.4 — Rotação DEBT-49**:

Conforme 130.A.4. Esperado: substituir `lang` por `stroke` ou
`alignment` no input do teste.

**130.C.5 — Prompt L0 + hash**:

Actualizar prompt L0 correspondente se existe (provavelmente
`00_nucleo/prompts/entities/style_chain.md`). Correr
`crystalline-lint --fix-hashes .`.

### 130.D — Verificação

1. `cargo test -p typst-core` — L1: 823 → **826** (+3).
2. `cargo test --workspace` — total ≥ 1057.
3. `crystalline-lint` zero violations.
4. Manual:

   ```bash
   $ cat l1.typ
   #set text(lang: "pt")
   Olá
   $ typst l1.typ -o l1.pdf 2>&1
   # stderr: vazio
   exit=0

   $ cat l2.typ
   #set text(lang: "en-GB")
   Hello
   $ typst l2.typ -o l2.pdf 2>&1
   # stderr: vazio
   exit=0

   $ cat l3.typ
   #set text(lang: "xx")   # código inválido
   Texto
   $ typst l3.typ -o l3.pdf 2>&1
   # stderr: vazio (sem validação — divergência aceite)
   exit=0
   ```

5. Canary:

   ```bash
   $ typst f.typ -o f.pdf 2>&1       # #set text(font: "X")
   # stderr contém warning 'font'
   ```

### 130.E — Encerramento

Relatório em `typst-passo-130-relatorio.md`:

- Inventário resultado (especialmente 130.A.4 — pool DEBT-49).
- Decisão tipo (EcoString vs String).
- Pattern aderência (variante 1 ou 2).
- Diff `StyleDelta` + arm + testes.
- Divergência vanilla (sem validação) documentada.
- Rotação DEBT-49 executada ou não.
- **Candidatos próximos para fechar DEBT-1**:
  - 131 text.font + substituição canary.
  - 132 activar target par.
  - 133 migrar leading.
  - 134 fechar DEBT-1 no DEBT.md.

---

## Critério de conclusão

1. Inventário 130.A escrito.
2. Decisão tipo documentada.
3. `StyleDelta.lang: Option<EcoString>` adicionado.
4. `eval_set_text` captura `lang`.
5. Valores compostos BCP 47 aceites literalmente.
6. Canary DEBT-50 preservado.
7. 3 testes L1 novos passam.
8. DEBT-49 rotado se necessário.
9. `cargo test --workspace` passa (≥ 1057).
10. `crystalline-lint` zero violations.
11. Relatório 130.E escrito.

---

## O que pode sair errado

- **Pool DEBT-49 esgota a rotar**: se `font/lang/stroke` era o
  input e `lang` deixa o pool, restam 2 conhecidas. Substituir
  por `hyphenate`, `alignment`, `first-line-indent`,
  `justify`. Pool de propriedades ainda não capturadas é amplo
  — não deve esgotar este passo. 131 (font) será o próximo
  ponto de tensão.

- **`Value::Str` wrapper inesperado**: se `Value::Str` não é
  `EcoString` directa mas algo como `Str(EcoString)` tuple
  struct, o match precisa de `Value::Str(Str(s))` ou similar.
  Verificar no ficheiro de `Value` antes de assumir.

- **Tipo `EcoString` não importado onde preciso**: adicionar
  `use ecow::EcoString;` nos ficheiros afectados. Trivial.

- **Vanilla tem validação que queremos espelhar**: improvável
  para XS. Se 130.A revela validação (ex: BCP 47 parser), aceitar
  divergência — consumer futuro normaliza/valida. Registar em
  relatório como divergência semântica suave.

- **`region` aparece no input de teste rotativo DEBT-49**: se
  pool já usa `region` como propriedade desconhecida, não tocar
  (ainda não capturamos). Se não usa, candidato para rotação
  futura.

---

## Notas operacionais

- Quinta aplicação do pattern DEBT-1 XS. Esperadamente literal
  variante 1 (primitivo simples com `Value::Str` → `EcoString`)
  ou variante 2 (tipo semântico como campo, se `EcoString` é
  considerado tipo semântico L1). Provavelmente variante 1.

- **Estratégia de fecho DEBT-1**: este passo é o mais limpo da
  sequência. `font` (131) será mais complicado por causa do
  canary. `par` (132) introduz novo dispatcher. `leading` (133)
  é migração. Começar pelo mais fácil dá momentum.

- **Pool DEBT-49 em pressão crescente**: cada passo que captura
  propriedade reduz pool. Após 130 (se lang sai do pool) e 131
  (font sai), pool fica pequeno. Candidato dedicado para passo
  135+: **"substituir teste rotativo DEBT-49 por positivos
  específicos"** — documenta cada captura via assertion positivo
  em vez de "estas propriedades não são capturadas".

- **Candidato antigo ainda pendente**: "Extract helper
  `eval_with_warnings` em L1 test harness" (registado no Passo
  127). Continua a valer para passo futuro. Cada teste novo
  (130, 131) acumula fricção de usar warnings manualmente.

- **Divergência vanilla sem validação**: cristalino aceita
  `#set text(lang: "pt-XX-variant-not-bcp47")` sem warning.
  Vanilla provavelmente erra no parse. Aceitar como temporal —
  consumer futuro (shaping, hyphenation) vai precisar de
  normalizar de qualquer forma e pode validar nesse ponto.

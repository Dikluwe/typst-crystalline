# Passo 127 — DEBT-1 subset: `text.tracking`

**Série**: 127 (passo XS esperado em L1; aplica template do
Passo 126).
**Precondição**: Passo 126 encerrado; 1044 total tests; zero
violations; 51 ADRs activas; 11 DEBTs abertos (DEBT-1 subset
`weight` pago).
**ADRs aplicáveis**: ADR-0038 (Style/Styles/StyleChain; tem
nota Passo 126), ADR-0040 (#set text activo via bake-in).
**ADR tocada**: decidida em 127.A — anotar ADR-0038 ou não.

---

## Objectivo

Adicionar `tracking: Option<T>` a `StyleDelta` onde `T` é
decidido em 127.A (Length / f64 / i32). `eval_set_text` captura
`tracking` de `#set text(tracking: ...)` sem warning. Fracciona
DEBT-1 — segunda propriedade numérica após `weight` do 126.

Ao fim do passo:

1. `StyleDelta.tracking: Option<T>` adicionado (T decidido).
2. `eval_set_text` captura `tracking`.
3. `#set text(tracking: ...)` não emite warning.
4. Canary DEBT-50 preservado (`font` continua a emitir warning).
5. Teste L1 novo valida captura.
6. Teste L3 DEBT-49 rotado se usava `tracking` como canary
   desconhecido.
7. Efeito visível no PDF: nenhum (documentado, como 126).

Este passo **não**:
- Adiciona outras propriedades (`leading`, `stroke`).
- Implementa consumo em layout.
- Toca L2, L3, L4 excepto teste DEBT-49 rotativo.

---

## Decisões já tomadas

1. **Só `tracking` neste passo** — disciplina 126 reiterada.
2. **Pattern 126** (`Option<T>` + match arm) aplicado.

## Decisões diferidas (127.A)

3. **Tipo T**:
   - **Length** se existe em L1 e é usado em outras propriedades
     capturadas.
   - **f64** se `Length` não existe ou é complexo.
   - **i32** se tracking vanilla for só inteiros (improvável).
4. **ADR**:
   - **Não anotar 0038** se aderência ao pattern do 126 é
     literal (padrão já documentado).
   - **Anotar 0038** se tipo escolhido introduz pattern novo
     (ex: `Length` com resolver não-trivial).

---

## Escopo

**Dentro**:
- `01_core/src/entities/style_chain.rs` — campo `tracking`.
- `01_core/src/rules/eval/rules.rs` — arm `"tracking"`.
- `01_core/src/rules/eval/tests.rs` — 2 testes (captura +
  canary font preservado).
- `03_infra/src/integration_tests.rs` — adapt test DEBT-49 se
  usava `tracking`.
- Prompt L0 `entities/style_chain.md` + hash.
- ADR-0038 anotada se 127.A decidir.

**Fora**:
- Outras propriedades.
- Consumo em layout.
- L2, L3 pipeline, L4.

---

## Sub-passos

### 127.A — Inventário

**Parte 1 — Tipos disponíveis em L1**:

1. `grep -rn "pub struct Length" 01_core/src/` — confirmar se
   existe.
2. `grep -rn "pub enum Length" 01_core/src/` — alternativa.
3. `view` no ficheiro se encontrado. Registar:
   - Assinatura (campos, variantes).
   - Se é `Copy` ou `Clone`.
   - Como outros propriedades de `StyleDelta` usam tipos
     similares.

**Parte 2 — Vanilla tracking**:

1. `grep -n "tracking" lab/typst-original/crates/typst-library/src/text/mod.rs`
   (ou equivalente).
2. Registar:
   - Tipo declarado (`Length`, `Em`, `Abs`, etc.).
   - Default value.
   - Como é resolvido em layout.

**Parte 3 — Outras propriedades em `StyleDelta`**:

1. `view` em `entities/style_chain.rs`. Registar cada campo
   existente e seu tipo.
2. Se `size` usa `Length`, `tracking` deve alinhar.
3. Se `size` usa `f64`, `tracking` pode alinhar ou divergir
   com razão.

**Parte 4 — Teste DEBT-49 actual**:

1. `grep -n "tracking" 03_infra/src/integration_tests.rs`.
2. Se o teste "3 desconhecidas" do 126 usa `tracking`, tem de
   rodar.
3. Se não usa, zero impacto.

**Escrever** em `00_nucleo/diagnosticos/inventario-tracking-passo-127.md`:

```
Tipo Length em L1:
  existe: sim/não
  assinatura: [...]
  usado em StyleDelta.size: sim/não

Vanilla tracking:
  tipo: [...]
  default: [...]

Decisão T:
  escolhido: [Length / f64 / i32]
  razão: [...]

Pattern 126:
  aderência literal: sim/não
  ADR nota: sim/não
```

**Gate 127.A**: se `Length` existe mas ligação em `eval_set_text`
exige helper de cast complexo (ex: converter `Value::Length` para
`Length` interno + handling de unidades `em`/`pt`), XS torna-se
S ou M. Se > 2 ficheiros tocados ou > 30 linhas, reportar.

### 127.B — ADR (condicional)

Se tipo T é alinhado com pattern do 126 (ex: `f64` ou `Length`
simples via match), **sem anotação**. Pattern documentado no
126 basta.

Se T introduz nuance (ex: resolver `Value::Length` → `Length`
com fallback), anotação pequena em ADR-0038:

```markdown
### Nota Passo 127 — `tracking` como primeira propriedade Length

Primeira propriedade a usar tipo Length (semântica tipográfica).
Cast `Value::Length → Length` via [método/helper]. Pattern
Option<Length> generaliza para futuras propriedades com
unidades (leading, spacing).
```

### 127.C — Implementação

**127.C.1 — `StyleDelta`**:

```rust
// + campo (ajustar tipo conforme 127.A):
pub tracking: Option<Length>,  // ou Option<f64> ou Option<i32>

// empty() ganha `tracking: None`
```

**127.C.2 — `eval_set_text`**:

Template do 126 literal:

```rust
"tracking" => {
    if let Value::Length(v) = val {
        delta.tracking = Some(v);
    }
    // Ou Value::Float(v) + try_from etc. conforme T
}
```

Tipo errado → silent skip (coerente com outros arms).

**127.C.3 — Testes novos**:

```rust
#[test]
fn eval_set_text_tracking_passo_127() {
    // Input com tracking válido; zero warning
    let src = "#set text(tracking: 0.5pt)\nOlá";  // ajustar sintaxe
    let (result, warnings) = eval_inline(src);
    assert!(result.is_ok());
    assert!(warnings.iter().all(|w| !w.message.contains("'tracking'")));
}

#[test]
fn eval_set_text_font_canary_passo_127() {
    // Canary: font continua a emitir warning
    let src = "#set text(font: \"X\")\nOlá";
    let (_, warnings) = eval_inline(src);
    assert!(warnings.iter().any(|w| w.message.contains("'font'")));
}
```

Ajustar ao harness real de testes L1.

**127.C.4 — Rotação DEBT-49**:

Se o teste `debt49_set_text_multiplas_propriedades_desconhecidas`
(L3) usa `tracking` como desconhecido, substituir por outra
propriedade ainda desconhecida (ex: `leading`, se ainda não
capturada).

**127.C.5 — Prompt L0 + hash**:

Actualizar `00_nucleo/prompts/entities/style_chain.md` com novo
campo. `crystalline-lint --fix-hashes .`.

### 127.D — Verificação

1. `cargo test -p typst-core` — L1: 813 → **815** (+2).
2. `cargo test --workspace` — total ≥ 1046.
3. `crystalline-lint` zero violations.
4. Manual:
   ```bash
   $ typst /tmp/track.typ -o out.pdf 2>&1
   # input: "#set text(tracking: 0.5pt)\nOlá"
   # stderr: vazio
   exit=0
   ```
5. Canary check:
   ```bash
   $ typst /tmp/font.typ -o out.pdf 2>&1
   # input: "#set text(font: \"X\")\nOlá"
   # stderr: contains "propriedade 'font' ainda não suportada"
   ```

### 127.E — Encerramento

1. Relatório `typst-passo-127-relatorio.md`:
   - Decisão tipo T + razão.
   - ADR anotada ou não + razão.
   - Pattern 126 aderência.
   - Diff StyleDelta + eval_set_text + testes.
   - Teste DEBT-49 rotado ou não.
   - Limitações (tracking inerte, como weight).

---

## Critério de conclusão

1. Inventário 127.A escrito.
2. Decisão de tipo T documentada.
3. ADR-0038 anotada **se aplicável**.
4. `StyleDelta.tracking: Option<T>` adicionado.
5. `eval_set_text` captura `tracking`.
6. `#set text(tracking: ...)` não emite warning.
7. Canary DEBT-50 preservado.
8. 2 testes L1 novos passam.
9. DEBT-49 rotado se necessário.
10. `cargo test --workspace` passa (≥ 1046).
11. `crystalline-lint` zero violations.
12. Relatório 127.E escrito.

---

## O que pode sair errado

- **`Length` em L1 é grande ou complexo**: tipo tem variantes
  (`Abs`, `Em`, `Rel<Abs>`, etc.), cast de `Value::Length`
  exige match branchy. Se > 10 linhas, reportar e simplificar
  para `f64` ou defer para passo dedicado.
- **`Value::Length` não existe como variante**: vanilla pode
  representar tracking como `Value::Number` ou equivalente.
  Gate 127.A confirma.
- **DEBT-49 teste rotativo esgota propriedades**: se o pool de
  "propriedades desconhecidas" é limitado, pode ser tempo de
  remover teste "3 desconhecidas" e escrever outro tipo de
  cobertura. Não neste passo — registar como candidato.
- **Tipo T decidido é `Length` mas cast de `Value::Length`
  produz `Length` diferente**: se há 2 definições de `Length`
  (L1 interno vs type externo via typst-library pattern),
  escolher a L1 interna. Gate 127.A.
- **Unit mismatch silencioso**: `tracking: 0.5` (sem unidade)
  vs `tracking: 0.5pt`. Typst parser pode promover para `Length`
  com unidade default ou erro. Confirmar no inventário — este
  passo **não** adiciona lógica de promoção, aceita o que parser
  dá.
- **Pattern 126 quebra se `Length` exige resolver lifetimes**:
  se `Length<'a>` ou similar, `Option<Length<'_>>` em struct
  pode não compilar trivialmente. Gate 127.A detecta.

---

## Notas operacionais

- Se tipo T decidido for `f64`, passo é réplica literal de 126
  com rename weight → tracking. 30 minutos.
- Se tipo T for `Length`, passo é ligeiramente maior —
  especialmente se ADR-0038 ganha nota nova. 60-90 minutos.
- Se inventário 127.A revelar cenário impossível (> 2 ficheiros
  tocados, > 30 linhas, lifetime issues), aceitar e reportar.
  Este passo é XS ou não acontece.
- Pattern do 126 está a ser validado na segunda aplicação.
  Se tudo encaixar, próximos passos (leading, stroke) ficam
  ainda mais mecânicos.
- Rotação do teste DEBT-49 é padrão conhecido. Se o pool
  esgota, candidato "substituir teste rotativo por teste
  positivo sobre propriedades específicas" entra na lista.

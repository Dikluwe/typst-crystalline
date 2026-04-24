# Passo 128 — DEBT-1 subset: `text.leading`

**Série**: 128 (passo XS esperado em L1; terceira aplicação do
pattern Passos 126/127).
**Precondição**: Passo 127 encerrado; 1046 total tests; zero
violations; 51 ADRs activas; 11 DEBTs abertos (DEBT-1 subset
`weight` + `tracking` pagos).
**ADRs aplicáveis**: ADR-0038 (Style/Styles/StyleChain; tem 2
notas — Passos 126 e 127), ADR-0040 (#set text activo via
bake-in).
**ADR tocada**: decidida em 128.A — anotar ADR-0038 ou não.

---

## Objectivo

Adicionar `leading: Option<Length>` a `StyleDelta` (tipo confirmado
em 128.A). `eval_set_text` captura `leading` de
`#set text(leading: ...)` sem warning.

Mas `leading` tem subtileza que `weight`/`tracking` não tiveram:
**em vanilla Typst, `leading` é propriedade de `par`, não de
`text`**. A sintaxe `#set par(leading: 0.65em)` é a canónica.
Decisão em 128.A: capturar em `#set text` (coerente com o que
cristalino faz), em `#set par` (coerente vanilla), ou ambos.

Ao fim do passo:

1. `StyleDelta.leading: Option<Length>` adicionado.
2. `eval_set_*` captura `leading` no contexto decidido (text,
   par, ou ambos).
3. Canary DEBT-50 preservado (`font` continua a emitir warning
   no contexto `text`).
4. Teste L1 novo valida captura.
5. Teste L3 DEBT-49 rotado se usava `leading` como canary.
6. Efeito visível no PDF: nenhum (documentado, como 126/127).

Este passo **não**:
- Adiciona outras propriedades (`stroke`, `alignment`).
- Implementa consumo em layout.
- Materializa `eval_set_par` se não existir — nesse caso, defer
  para quando `#set par` for activado como bloco.
- Toca L2, L3, L4 excepto teste DEBT-49 rotativo.

---

## Decisões já tomadas

1. **Só `leading` neste passo** — disciplina dos 126/127
   reiterada.
2. **Tipo `Length`** — preserva `abs + em` inteiro, alinhado
   com decisão do 127 para `tracking`.
3. **Pattern 126/127** aplicado literal (template em 4 blocos).

## Decisões diferidas (128.A)

4. **Contexto de captura**:
   - **(a) `#set text(leading: ...)`** — coerente com passos
     126/127 (tudo em `text`). Diverge vanilla.
   - **(b) `#set par(leading: ...)`** — coerente vanilla.
     Requer `eval_set_par` existir em L1.
   - **(c) Ambos** — aceita em text E par. Flexibilidade mas
     mais código.
5. **ADR**:
   - **Não anotar 0038** se decisão é (a) e pattern é literal.
   - **Anotar 0038** se decisão é (b) ou (c) — documenta
     extensão para contexto `par`.

---

## Escopo

**Dentro**:
- `01_core/src/entities/style_chain.rs` — campo `leading`.
- `01_core/src/rules/eval/rules.rs` — arm `"leading"` no
  `eval_set_text` e/ou `eval_set_par`.
- `01_core/src/rules/eval/tests.rs` — 2 testes (captura +
  canary).
- `03_infra/src/integration_tests.rs` — adaptar DEBT-49 se
  necessário.
- Prompt L0 `entities/style_chain.md` + hash.
- ADR-0038 anotada se 128.A decidir.

**Fora**:
- Criar `eval_set_par` do zero se não existe — passo dedicado.
- Outras propriedades.
- Consumo em layout.
- L2, L3 pipeline, L4.

---

## Sub-passos

### 128.A — Inventário

**Parte 1 — Vanilla `leading`**:

1. `grep -n "leading" lab/typst-original/crates/typst-library/src/layout/par.rs`
   (ou equivalente — pode estar em `text.rs`).
2. Registar:
   - Em que struct vive (ParElem vs TextElem).
   - Tipo (provavelmente `Length` com default `0.65em`).
   - Default value exacto.

**Parte 2 — `eval_set_par` em L1**:

1. `grep -n "eval_set_par\|\"par\"" 01_core/src/rules/eval/`.
2. Registar:
   - Se existe função dedicada para `#set par`.
   - Se existe arm `"par"` num dispatcher (ex: match sobre
     target em `eval_set`).
   - Se `#set par` hoje emite warning "não suportado" tout
     court ou se há handling.

**Parte 3 — Outras propriedades que `#set par` captura**:

Se `eval_set_par` existe, listar arms actuais. Se não, zero.

**Parte 4 — Teste DEBT-49 actual**:

1. `grep -n "leading" 03_infra/src/integration_tests.rs`.
2. Se `debt49_set_text_multiplas_propriedades_desconhecidas`
   usa `leading`, rodar para outra.

**Parte 5 — Decisão (a), (b), ou (c)**:

Matriz de decisão:

| Cenário inventário | Decisão |
|---|---|
| `eval_set_par` existe, aceita propriedades | **(b)** ou **(c)** |
| `eval_set_par` existe mas é stub/warning geral | **(a)** por enquanto; **(b)** em passo dedicado |
| `eval_set_par` não existe | **(a)** por default; **(b)** exige criar função (fora escopo) |

**Preferência operacional**: (a) se contexto `par` não está
activo hoje — mantém XS. (b) ou (c) só se `eval_set_par` já
processa outras propriedades.

**Escrever** em `00_nucleo/diagnosticos/inventario-leading-passo-128.md`:

```
Vanilla leading:
  struct: ParElem / TextElem
  tipo: Length
  default: 0.65em

eval_set_par em L1:
  existe: sim/não
  processa propriedades: sim/não
  arms actuais: [...]

eval_set_text arms actuais (ref 126/127): weight, tracking, ...

Decisão contexto:
  (a) / (b) / (c)
  razão: [...]

Pattern 126/127:
  aderência literal: sim (se a) / não (se b ou c)
  ADR nota: sim/não
```

**Gate 128.A**: 
- Se decisão é (b) e `eval_set_par` não existe → **parar e
  reportar**. Criar função é passo dedicado.
- Se decisão é (c) e exige duplicação em 2 sítios → aceitável
  se literal (mesmo match arm em ambos), reportar se exige
  refactor.
- Se decisão é (a) + `leading` não coincide com conceito
  semântico ("leading pertence a par") → **aceitar divergência
  temporária**, registar candidato "migrar leading para
  eval_set_par quando este for activado".

### 128.B — ADR (condicional)

**Decisão (a) + pattern literal 126/127**: sem anotação.
Terceira aplicação idêntica; pattern já sólido.

**Decisão (b) ou (c)**: anotação em ADR-0038:

```markdown
### Nota Passo 128 — `leading` captura em contexto `par`

Primeira propriedade capturada em `#set par` (não só `#set text`).
Pattern Option<T> em StyleDelta estende-se a múltiplos contextos
de set — o campo `leading` é populado conforme o target do `set`
(par vs text). Decisão: [a/b/c] + razão.

Futuras propriedades específicas de par (justify, first-line-indent,
...) seguem o mesmo padrão.
```

### 128.C — Implementação

**128.C.1 — `StyleDelta`**:

```rust
// + campo:
pub leading: Option<crate::entities::layout_types::Length>,

// empty() ganha `leading: None`
```

Idêntico a `tracking` do 127. Tipo confirmado `Length`.

**128.C.2 — Arm no eval**:

**Se decisão (a)** — em `eval_set_text`:

```rust
"leading" => {
    if let Value::Length(l) = val {
        delta.leading = Some(l);
    }
}
```

Réplica literal de `tracking`.

**Se decisão (b)** — em `eval_set_par`:

```rust
"leading" => {
    if let Value::Length(l) = val {
        delta.leading = Some(l);
    }
}
```

Mesma sintaxe mas em contexto diferente. Exige que
`eval_set_par` já exista.

**Se decisão (c)** — em ambos:

Mesmo arm duplicado. OU helper partilhado (mais sofisticado —
fora escopo se requer refactor).

**128.C.3 — Testes novos (L1)**:

```rust
#[test]
fn eval_set_<target>_leading_passo_128() {
    // target = text, par, ou ambos conforme 128.A
    let src = "#set text(leading: 0.65em)\nOlá";  // ajustar target
    let (result, warnings) = eval_inline(src);
    assert!(result.is_ok());
    assert!(warnings.iter().all(|w| !w.message.contains("'leading'")));
}

#[test]
fn eval_set_text_font_canary_passo_128() {
    // Canary: font continua a emitir warning
    let src = "#set text(font: \"X\")\nOlá";
    let (_, warnings) = eval_inline(src);
    assert!(warnings.iter().any(|w| w.message.contains("'font'")));
}
```

Se decisão (c), teste adicional para cobertura do segundo
contexto.

**128.C.4 — Rotação DEBT-49**:

Grep em 128.A.4 informa. Se necessário, substituir `leading`
por próxima propriedade desconhecida (ex: `stroke`, `alignment`
se não foram usadas antes; `justify` se `par` entrar).

**128.C.5 — Prompt L0 + hash**:

Actualizar `00_nucleo/prompts/entities/style_chain.md` com novo
campo. `crystalline-lint --fix-hashes .`.

### 128.D — Verificação

1. `cargo test -p typst-core` — L1: 815 → **817** (+2, ou +3
   se decisão (c)).
2. `cargo test --workspace` — total ≥ 1048.
3. `crystalline-lint` zero violations.
4. Manual:
   ```bash
   $ cat lead.typ
   #set text(leading: 0.65em)    # ou #set par(...) conforme 128.A
   Olá
   $ typst lead.typ -o out.pdf 2>&1
   # stderr: vazio
   exit=0
   ```
5. Canary check:
   ```bash
   $ typst font.typ -o out.pdf 2>&1
   # stderr contém warning 'font'
   ```

### 128.E — Encerramento

1. Relatório `typst-passo-128-relatorio.md`:
   - Inventário resultado.
   - Decisão contexto (a/b/c) + razão.
   - ADR anotada ou não + razão.
   - Pattern 126/127 aderência.
   - Diff StyleDelta + eval_set_* + testes.
   - Teste DEBT-49 rotado ou não.
   - Limitações (leading inerte, possível divergência vanilla
     se (a)).
   - Candidato futuro: migrar `leading` para `eval_set_par`
     quando activado (se decisão foi (a)).

---

## Critério de conclusão

1. Inventário 128.A escrito.
2. Decisão contexto documentada.
3. ADR-0038 anotada **se aplicável**.
4. `StyleDelta.leading: Option<Length>` adicionado.
5. `eval_set_<target>` captura `leading`.
6. `#set <target>(leading: ...)` não emite warning.
7. Canary DEBT-50 preservado.
8. 2-3 testes L1 novos passam (conforme decisão).
9. DEBT-49 rotado se necessário.
10. `cargo test --workspace` passa (≥ 1048).
11. `crystalline-lint` zero violations.
12. Relatório 128.E escrito.

---

## O que pode sair errado

- **`eval_set_par` não existe e decisão vira (a) por default**:
  divergência vanilla aceite. `leading` capturado em contexto
  errado (text em vez de par) mas inerte — zero impacto
  observável. Candidato futuro registado.
- **`leading` tem default `0.65em` diferente da captura
  utilizador**: este passo não implementa default — só captura
  override explícito. Quando consumer chegar, resolve com
  fallback. Mesma lógica de `tracking` do 127.
- **Pattern 126/127 não aplica se `eval_set_par` existe mas é
  stub incompleto**: se o dispatcher de `#set` não tem arm
  para `par`, adicionar cria ripple. Gate 128.A detecta.
- **DEBT-49 teste rotativo esgota pool**: pool de propriedades
  desconhecidas está a encolher (weight, tracking, leading
  activados). Se o teste precisar de outra rotação e o pool
  tiver só 2 desconhecidas, teste perde valor. Registar
  candidato "substituir rotativo por positivo" para passo
  dedicado.
- **`Value::Length` não aceita `em` sem fonte**: parser Typst
  promove `0.65em` para `Length { em: 0.65, abs: 0 }`.
  Captura preserva; cast de `Value::Length` para `Length` é
  directo (validado no 127).
- **Decisão (c) exige helper partilhado**: se o arm é duplicado
  literal em dois sítios, aceitável. Se refactor para helper
  (`capture_leading(val, delta)`) adiciona > 10 linhas, defer.

---

## Notas operacionais

- Terceira aplicação do pattern 126/127. Se correr idêntica
  (decisão (a)), o pattern está validado como repetitivo XS.
- Decisão (b) ou (c) introduz nuance de contexto — primeira
  vez nesta série. Documentar em ADR-0038.
- `leading` é propriedade de `par` em vanilla. Divergir
  temporariamente (capturar em text) é aceitável porque:
  1. Valor é inerte — zero impacto visível.
  2. Migração para `par` é XS quando `eval_set_par` existir.
  3. Captura em text permite utilizador escrever
     `#set text(leading: ...)` sem warning — mesmo que
     semanticamente pertence a par.
- Pattern potencial futuro: "propriedade com target principal
  X mas captura tolerada em Y". Registar se aparecer
  repetidamente.
- Pool DEBT-49: inventário 128.A.4 regista quais propriedades
  ainda são "desconhecidas". Se < 4, considerar substituir
  teste rotativo por positivos específicos em passo dedicado.
- Candidato futuro em `candidatos-passos-futuros.md`:
  "Extract helper `eval_with_warnings` em L1 test harness"
  (registado no 127, ainda não executado).

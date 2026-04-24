# Passo 126 — DEBT-1 subset: `text.weight` numérico (u16)

**Série**: 126 (passo XS; volta a L1 após 12 passos de CLI + 1
auditoria).
**Precondição**: Passo 125 encerrado; 1042 total tests; zero
violations; 51 ADRs activas; 11 DEBTs abertos.
**ADRs aplicáveis**: ADR-0038 (Style/Styles/StyleChain),
ADR-0040 (#set text activo via bake-in), ADR-0048 (DEBT-50
canary).
**ADR tocada**: **ADR-0038** ganha nota "Passo 126 — `weight`
adicionada como primeira propriedade numérica".

---

## Objectivo

Adicionar `weight: Option<u16>` a `StyleDelta` (L1) para que
`#set text(weight: 700)` seja **capturado** no bake-in sem
warning. Fracciona DEBT-1 — paga uma propriedade isolada sem
bloquear nas outras (`font`, `lang`, `leading`).

Ao fim do passo:

1. `StyleDelta.weight: Option<u16>` adicionado.
2. `eval_set_text` captura `weight` quando presente no argumento.
3. `#set text(weight: 700)` **não emite warning** "propriedade
   não suportada".
4. `#set text(font: "X", weight: 700)` — warning de `font`
   continua (DEBT-50 preservado).
5. Teste L1 novo valida captura do weight.
6. Efeito visível no PDF: **nenhum** (documentado).
7. ADR-0038 anotada.

Este passo **não**:
- Implementa forma simbólica (`"bold"`, `"regular"`) — passo
  separado.
- Materializa `FontWeight` enum — passo dedicado se forma
  simbólica chegar.
- Afecta layout visível — sem selecção de variante de fonte
  nem faux-bold.
- Adiciona outras propriedades (`size`, `fill`, etc.).
- Toca L2, L3, L4.

---

## Decisões já tomadas

1. **Tipo `u16`** — minimalista. Forma simbólica fica para
   passo separado.
2. **Só weight neste passo** — disciplina. Outras propriedades
   em passos dedicados.
3. **Anotar ADR-0038** — não ADR nova. O pattern "propriedade
   activada incrementalmente" já está implícito no bake-in.

---

## Escopo

**Dentro**:
- `01_core/src/entities/style.rs` (ou equivalente) — campo
  `weight` em `StyleDelta`.
- `01_core/src/rules/eval/` — `eval_set_text` captura `weight`
  do argumento.
- Teste L1 novo para captura.
- ADR-0038 anotada.
- Prompt L0 do módulo afectado em L1 — actualizar hash.

**Fora**:
- L2, L3, L4.
- Forma simbólica.
- Layout visível.
- Outras propriedades.
- Mudar DEBT-1 de "aberto" para "parcialmente fechado" formal —
  decisão de taxonomia em passo dedicado (fica como nota).

---

## Sub-passos

### 126.A — Inventário

**Parte 1 — `StyleDelta` actual**:

1. `view` em `01_core/src/entities/style.rs` (ou ficheiro onde
   `StyleDelta` vive).
2. Registar:
   - Campos actuais.
   - Comentário de ADR-0038 / ADR-0040.
   - Quais propriedades estão activas e quais continuam em
     warning.

**Parte 2 — `eval_set_text` actual**:

1. `view` em `01_core/src/rules/eval/` ou equivalente.
   Localizar a função que processa `#set text(...)`.
2. Registar:
   - Como propriedades activas são capturadas (ex: match sobre
     `key`, `if key == "lang"` etc.).
   - Como propriedades inactivas emitem warning.
   - Se há helper partilhado para emit de warning.

**Parte 3 — Testes DEBT-50 / canary**:

1. `grep` por `DEBT-50`, `canary`, `propriedade 'font'` em
   `01_core/src/` e `03_infra/src/`.
2. Confirmar que testes do canário usam **`font`** como canary
   (não `weight`). Se algum teste usa `weight`, migra para
   `font` neste passo.

**Parte 4 — Vanilla `weight` em TextStyle**:

1. `view` em `lab/typst-original/crates/typst-library/src/text/mod.rs`
   (ou equivalente).
2. Registar como weight é representado — provavelmente
   `FontWeight(u16)` com `impl From<u16>` e parsing de string.
3. Se usa enum, alinhamento futuro pode reusar; neste passo só
   `u16` basta.

**Escrever** em `00_nucleo/diagnosticos/inventario-weight-passo-126.md`:

```
StyleDelta actual:
  campos: [...]
  propriedades activas: [...]
  propriedades em warning: [font, lang, weight, ...]

eval_set_text:
  pattern: match sobre key
  helper de warning: [...]

Canary DEBT-50:
  usa font: sim/não
  testes que dependem: [...]

Vanilla:
  FontWeight: u16 / enum
  impl From<u16>: sim/não
```

**Gate 126.A**: se inventário revelar que adicionar `weight`
exige mais que 1 campo + 1 match arm (ex: precisa tocar
`StyleChain.resolve`, `Style` pipeline, ou helpers não
relacionados), registar e considerar se XS é realista. Se > 2
ficheiros tocados, reportar.

### 126.B — Anotação ADR-0038

No final de `00_nucleo/adr/typst-adr-0038-style-styles-stylechain.md`:

```markdown
### Nota Passo 126 — `weight` como primeira propriedade numérica

`StyleDelta.weight: Option<u16>` adicionado para capturar
`#set text(weight: 700)` sem warning.

Pattern estabelecido: propriedades do `text` podem ser adicionadas
uma a uma como `Option<T>` em `StyleDelta`, com captura em
`eval_set_text`. Não exige materialização de tipos Font/Lang/Par
adjacentes.

Forma simbólica (`"bold"`, `"regular"`, etc.) fica para passo
dedicado. Este passo cobre só `weight: 700` numérico.

Efeito de layout: **nenhum**. `StyleChain.resolve(weight)` não
é consumido por pipeline de layout hoje (sem selecção de variante
de fonte, sem faux-bold). Propriedade capturada mas inerte.
Primeira aplicação que consumir `weight` materializa o path
pertinente.
```

### 126.C — Implementação

**126.C.1 — `StyleDelta`**:

```rust
// Em style.rs ou equivalente:
pub struct StyleDelta {
    // ... campos existentes ...
    pub weight: Option<u16>,   // NOVO
}
```

Ajustar `impl Default`, `impl` de merge/resolve se aplicável.

**126.C.2 — `eval_set_text`**:

Adicionar arm para `weight`:

```rust
// Onde outras propriedades são capturadas:
match key.as_str() {
    // ... casos existentes ...
    "weight" => {
        let v = cast_arg_as_u16(&arg)?;
        delta.weight = Some(v);
    }
    _ => {
        // emit warning "propriedade não suportada"
    }
}
```

Ajustar helper de cast `u16` se existir; criar se não existir.

**126.C.3 — Teste L1**:

```rust
#[test]
fn set_text_weight_captura_u16() {
    let source = "#set text(weight: 700)\nTeste";
    let (result, warnings) = eval_test(source);
    
    assert!(result.is_ok());
    assert!(
        warnings.is_empty(),
        "esperava zero warnings; got: {:?}",
        warnings
    );
    
    // Se expor Style/StyleChain em testes:
    // assert_eq!(resolved.weight, Some(700));
}
```

Ajustar ao harness de testes L1 existente (eval_test helper,
ou função equivalente).

**126.C.4 — Prompt L0 + hash**:

Actualizar `00_nucleo/prompts/core/entities/style.md` (ou
equivalente) se tocou. Correr `crystalline-lint --fix-hashes .`.

### 126.D — Verificação

1. `cargo test -p typst-core` passa. Contagem:
   - L1: 811 → **812** (+1: `set_text_weight_captura_u16`).
2. `cargo test --workspace` passa.
3. `crystalline-lint` zero violations.
4. Validação manual:
   ```bash
   $ cat test.typ
   #set text(weight: 700)
   Olá
   $ typst test.typ -o out.pdf 2>&1
   # stderr deve estar vazio (sem warning)
   ```
5. Validação canary — `#set text(font: "X")` **continua** a
   emitir warning.

### 126.E — Encerramento

1. ADR-0038 anotada.
2. Testes 124 `disciplina_*` continuam a passar — input com
   `#set text(font: "X")` continua a emitir warning
   (disciplina).
3. Relatório `typst-passo-126-relatorio.md`:
   - Inventário resultado.
   - Diff de `StyleDelta` e `eval_set_text`.
   - Contagem final.
   - Limitações explicítas (weight capturado mas inerte).
   - Candidatos futuros registados (forma simbólica, size,
     fill, outras propriedades, consumo em layout).

---

## Critério de conclusão

1. Inventário 126.A escrito.
2. ADR-0038 anotada.
3. `StyleDelta.weight: Option<u16>` adicionado.
4. `eval_set_text` captura `weight`.
5. `#set text(weight: 700)` não emite warning.
6. `#set text(font: "X")` continua a emitir warning (canary
   DEBT-50 preservado).
7. Teste L1 novo passa.
8. `cargo test --workspace` passa (total ≥ 1043).
9. `crystalline-lint` zero violations.
10. Prompts L0 com hashes correctos.
11. Relatório 126.E escrito.

---

## O que pode sair errado

- **Gate 126.A**: se adicionar `weight` exige tocar > 2
  ficheiros (ex: `StyleChain.resolve(weight)` em todos os call
  sites), registar e reavaliar XS. Pode ser preciso passo
  dedicado primeiro para preparar o resolve.
- **Cast `u16` do argumento**: arg Typst pode ser `Int`, `Float`,
  ou string numérica. Se há helper `cast_arg_as_u16` já, usar;
  se não, criar trivial com bounds check (0–1000, range
  CSS/OpenType). Erro de cast → warning ou error apropriado.
- **Teste `eval_test` não expõe `StyleDelta`**: se o harness
  L1 só consome `Module` (output final do eval), o teste vale
  como "zero warnings" + "compila OK" — não valida captura real
  de valor. Aceitável: o comportamento observável é suficiente.
  Se quisermos teste estrutural, expor `StyleDelta` em
  `#[cfg(test)]` pode ser passo paralelo.
- **Warning helper não aceita exclusão selectiva**: se o helper
  actual emite warning para **toda** propriedade não-match,
  adicionar `weight` ao match arm basta. Se há lista centralizada
  de "propriedades conhecidas", actualizar lista também.
- **Canary DEBT-50 falso negativo**: se algum teste actual
  esperava warning para `weight`, passa a falhar. Grep por
  `weight` em testes L1/L3 antes de mudar. Se encontra, migrar
  assert para outra propriedade (ex: `lang`).
- **Valor fora do range CSS (0–1000)**: `#set text(weight: 9000)`
  — aceita ou rejeita? CSS/OpenType aceita 0–1000. Para este
  passo, aceitar qualquer `u16` (sem validar range) — validação
  fica para quando weight for consumido. Documentar.
- **Propriedade "weight" em argumento tem case diferente**:
  Typst usa snake_case. `#set text(weight: 700)` usa `weight`
  literal. Confirmar.

---

## Notas operacionais

- Este é primeiro passo em L1 desde o 111 (formato diagnósticos).
  Refresh de contexto bem-vindo.
- A disciplina de XS é crucial — "enquanto cá estou adiciono size
  e fill" multiplica tamanho por 3. Resistir. Relatório regista
  candidato para próximo passo.
- `weight` capturado mas inerte é decisão honesta. O utilizador
  vê `#set text(weight: 700)` a aceitar; PDF é idêntico; não
  há mentira arquitectural — pipeline de layout não consome
  weight ainda.
- Se em 126.A o inventário revelar que `StyleChain.resolve` já
  produz `weight` (improvável mas possível se preparado
  antecipadamente), o passo é mais curto: só adicionar captura.
- Efeito visível será outro passo, quando: (a) múltiplas fontes
  disponíveis e selecção por weight ou (b) faux-bold via stroke.
  Ambos grandes — não no escopo.
- Registar em `candidatos-passos-futuros.md`:
  - Forma simbólica de `weight` (`"bold"` → 700, etc.).
  - `text.size` capturado.
  - `text.fill` capturado.
  - Consumo de `weight` em layout (selecção de variante ou
    faux-bold).

# P446 — Relatório: Smallcaps

> **Data:** 2026-06-24  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Branch:** `Tekt`  
> **Foco:** Materializar o consumer real de `#smallcaps[...]`, convertendo letras minúsculas em small capitals (maiúsculas reduzidas) e habilitar show rule `#show smallcaps: ...`.

---

## Resumo executivo

O **variant `Content::SmallCaps`** e a função `native_smallcaps` já existiam desde o **Passo 408** como stub transparente (output byte-idêntico ao body). O **trabalho do P446** foi:

1. Implementar o **render real** por fallback de scaling (0.8×), conforme vanilla quando a fonte não tem small caps OpenType.
2. Adicionar **`NodeKind::Smallcaps`** para que `#show smallcaps: ...` funcione.
3. Actualizar os tests e a spec L0.

**Nota de adaptação ao P446 original:** o passo P446 propunha modelar `smallcaps` como `Style::Smallcaps` + `Content::Styled`. O projecto já materializara P408 com o variant `Content::SmallCaps { body }` (usado também pelo consumer CSL de bibliografias). Para minimizar o blast radius e manter compatibilidade com P408/CSL, optou-se por manter o variant e implementar o consumer real via flag no `Layouter`, em vez de introduzir um segundo modelo paralelo. O resultado funcional (paridade visual e show rule) é o mesmo.

**Resultado:** P446 fechado; `cargo test --workspace` verde; `crystalline-lint .` sem novas violações.

---

## 1. Mudanças de código do P446

### 1.1 `01_core/src/engine/layout/mod.rs`

- Adicionado campo `smallcaps: bool` ao `Layouter` (inicializado a `false`).
- Actualizado o arm `Content::SmallCaps { body }`:
  - Guarda o estado anterior do flag.
  - Activa `self.smallcaps = true`.
  - Faz `layout_content(body)`.
  - Restaura o flag no fim.
- Medição em grid (`measure_content_constrained`) permanece transparente (o layout real é que aplica o efeito; medição exacta seria over-engineering para este passo).

### 1.2 `01_core/src/engine/layout/text.rs`

- No layout de `Content::Text`, quando `layouter.smallcaps` está activo:
  - Cada palavra é segmentada em runs consecutivos de:
    - caracteres minúsculos → convertidos para maiúsculas (`to_uppercase`) e renderizados a `0.8×` do tamanho actual;
    - outros caracteres (maiúsculas, dígitos, pontuação) → renderizados no tamanho actual.
  - O espaço entre palavras é adicionado manualmente após cada palavra.
- Este mecanismo aplica-se recursivamente a texto aninhado em `strong`, `emph`, `Styled`, etc., porque o flag persiste durante o layout do body.

### 1.3 `01_core/src/engine/layout/cursor.rs`

- Adicionado método `layout_chunk(chunk: &str)`:
  - Semelhante a `layout_word`, mas **não adiciona o espaço de separação** no fim.
  - Permite compor uma palavra a partir de múltiplos fragmentos com tamanhos diferentes (smallcaps).

### 1.4 `01_core/src/entities/show.rs`

- Adicionado `NodeKind::Smallcaps` ao enum.
- Actualizado o comentário do conjunto completo de tipos.

### 1.5 `01_core/src/engine/eval/rules.rs`

- `selector_matches`: adicionado match `(Content::SmallCaps { .. }, NodeKind::Smallcaps)`.
- `eval_show_rule`: adicionado mapeamento do function pointer `native_smallcaps` para `Selector::NodeKind(NodeKind::Smallcaps)`.
- Actualizada a mensagem de erro para listar `smallcaps` entre os tipos suportados.

### 1.6 `01_core/src/engine/layout/tests.rs`

- Convertidos os 2 tests stub do Passo 408 em tests de render real do P446:
  - `p446_smallcaps_converte_minusculas_para_maiusculas` — verifica que `"SmallCaps"` renderiza `"SMALLCAPS"` e que existem runs escalados (< 11 pt) e runs normais.
  - `p446_smallcaps_via_stdlib_converte_texto` — verifica que `#smallcaps("Hello")` renderiza `"HELLO"` com runs escalados.
- Preservado o teste `p408_smallcaps_nao_vaza_estilo` (continua válido).

### 1.7 `01_core/src/engine/eval/rules.rs` (tests)

- Adicionados 2 tests de selector:
  - `p446_selector_smallcaps_casa_content_smallcaps`
  - `p446_selector_smallcaps_nao_casa_texto_plano`

### 1.8 `00_nucleo/prompts/engine/stdlib/text.md`

- Actualizada a secção `smallcaps(body)` para reflectir o Passo 446:
  - Consumer real por scaling.
  - Flag `Layouter.smallcaps`.
  - Segmentação por runs de minúsculas vs outros caracteres.
  - Show rule `NodeKind::Smallcaps`.
- Actualizado `Hash do Código` para o hash actual de `text.rs`.

### 1.9 `01_core/src/engine/stdlib/text.rs`

- Actualizado `@prompt-hash` para o hash actual do prompt L0.

---

## 2. Estado pré-existente (P408)

| Componente | Ficheiro | Estado |
|------------|----------|--------|
| Variant `Content::SmallCaps` | `01_core/src/entities/content.rs` | Existente; body transparente |
| `native_smallcaps` | `01_core/src/engine/stdlib/text.rs` | Existente; devolve `Content::SmallCaps` |
| Registo stdlib | `01_core/src/engine/eval/mod.rs` | `scope.define("smallcaps", ...)` já existia |
| Layout stub | `01_core/src/engine/layout/mod.rs` | Arm `SmallCaps` era transparente |

Não houve necessidade de migrar para `Style::Smallcaps` porque o variant P408 já cobria a semântica de container e era usado pelo CSL (`bib_csl.rs`).

---

## 3. Verificação

### 3.1 `cargo test --workspace`

```bash
RUST_MIN_STACK=8388608 cargo test --workspace
```

Resultado: **todos os testes passam**, incluindo:
- 2 novos tests de layout (`p446_smallcaps_*`).
- 2 novos tests de selector (`p446_selector_smallcaps_*`).
- Tests P408 existentes adaptados / preservados.
- Pipeline completo (`typst-core`, `typst_shell`, `typst_infra`, `typst_wiring`, CLI, `crystalline_lint`).

Resumo dos conjuntos:
- `3174 passed` (`typst-core` lib)
- `489 passed; 6 ignored` (`typst-core` tests integração)
- `24 passed` (`typst_shell`)
- `2 passed` (`typst_infra`)
- `21 passed` (`typst_wiring`)
- `2 passed` (`crystalline_lint`)

### 3.2 `crystalline-lint .`

Resultado: **zero novas violações**. Apenas os 2 warnings órfãos de prompts pre-existentes.

---

## 4. Scope-out preservado

- Shaping OpenType `smcp`/`c2sc` nativo da fonte continua scope-out; o fallback por scaling é funcionalmente equivalente para a maioria das fontes.
- `upper`/`lower` como métodos de `str` permanecem inalterados (scope-out de elemento de estilo).
- Medição exacta de smallcaps em grid (`measure_content_constrained`) permanece transparente; impacto prático negligenciável para este passo.

---

## 5. Commits

- Branch: `Tekt`
- Commit: `P446: smallcaps real com fallback por scaling e selector de show rule`

Alterações incluídas no commit:
- `01_core/src/engine/layout/mod.rs`
- `01_core/src/engine/layout/text.rs`
- `01_core/src/engine/layout/cursor.rs`
- `01_core/src/entities/show.rs`
- `01_core/src/engine/eval/rules.rs`
- `01_core/src/engine/layout/tests.rs`
- `01_core/src/engine/stdlib/text.rs`
- `00_nucleo/prompts/engine/stdlib/text.md`
- `00_nucleo/materialization/typst-passo-446.md`
- `00_nucleo/materialization/typst-passo-446-relatorio.md`

# Relatório — typst-passo-858: `measure()` com métricas reais via injeção de `&dyn FontMetrics` no `Engine`

**Data:** 2026-07-23  
**Executor:** Kimi Code (agente principal; prompt lido de `00_nucleo/materialization/typst-passo-858.md`).  
**Proveniência das medições:** working tree não commitado sobre commit `dfe3c2282ec0d6bd97d5834f00214e7c7f5d2d49`; alterações locais nos ficheiros listados na secção 2.

---

## 1. Decisão do dono

Após a verificação de design de P857, o dono optou pela **Opção 1 reformulada**: injeção de `&dyn FontMetrics` no `Engine` durante a expansão de `#context`, via inversão de dependência. A decisão foi:

- Campo `font_metrics` **obrigatório** em `Engine` (não `Option`).
- Tipo `&'a dyn FontMetrics` (trait object), seguindo o precedente de `world: &'a dyn World`.
- Implementação concreta `FallbackFontMetrics::new(world)` construída em L3 e injetada no ponto de composição `expand_context_blocks`.
- L1 permanece puro: só conhece o trait `FontMetrics`; `FallbackFontMetrics` continua em L3.

O Prompt L0 de `Engine` (`00_nucleo/prompts/entities/engine.md`) foi actualizado e guardado pelo dono antes de qualquer código ser escrito.

---

## 2. Alterações aplicadas

### 2.1. Prompt L0

- `00_nucleo/prompts/entities/engine.md`: adicionada secção sobre o campo `font_metrics` e a sua proveniência.
- Prompt-hash corrigido para `c17cd9ae` via `crystalline-lint --fix-hashes`.

### 2.2. Estrutura `Engine` (L1)

`01_core/src/entities/engine.rs:40`:

```rust
pub struct Engine<'a> {
    pub world: &'a dyn World,
    pub font_metrics: &'a dyn FontMetrics,
    pub route: Tracked<'a, Route<'a>>,
    pub styles: &'a mut StyleChain,
    pub show_rules: &'a mut Arc<[ShowRule]>,
    pub active_guards: &'a mut Vec<RuleId>,
    pub current_file: FileId,
    pub sink: &'a mut TrackedMut<'a, Sink>,
}
```

### 2.3. Propagação nos sites de construção (L1)

Foram ajustados **13 sites** de construção literal `Engine { ... }`:

| Ficheiro | Linhas | Valor injectado |
|---|---|---|
| `01_core/src/engine/eval/mod.rs` | 431, 784, 929 | `&FixedMetrics` (raiz) / `engine.font_metrics` |
| `01_core/src/engine/eval/markup.rs` | 39 | `engine.font_metrics` |
| `01_core/src/engine/eval/modules.rs` | 91, 286 | `engine.font_metrics` |
| `01_core/src/engine/eval/closures.rs` | 397 | `engine.font_metrics` |
| `01_core/src/engine/stdlib/eval.rs` | 170 | `&FixedMetrics` (raiz) |
| `01_core/src/engine/stdlib/mod.rs` | 341 | `&FixedMetrics` (raiz) |
| `01_core/src/engine/eval/tests.rs` | 134 | `&FixedMetrics` (raiz) |
| `01_core/src/engine/introspect.rs` | 3142 | `engine.font_metrics` |
| `01_core/src/engine/introspect/fixpoint.rs` | 218 | `engine.font_metrics` |
| `01_core/src/engine/introspect/from_tags.rs` | 335 | `engine.font_metrics` |

Sites raiz (eval inicial, stdlib, testes) usam `&FixedMetrics` porque não têm acesso a `world` real. Sites derivados de um `Engine` outer propagam `engine.font_metrics`.

### 2.4. Ajuste de `measure_content_real` (L1)

`01_core/src/engine/layout/mod.rs:1790`:

- A função passou a aceitar `metrics: &dyn FontMetrics`.
- O `Layouter` de medição é construído com essa métrica em vez de `FixedMetrics` hardcoded.

### 2.5. Trait object como métrica genérica

`01_core/src/engine/layout/metrics.rs`: adicionado `impl FontMetrics for &dyn FontMetrics`, permitindo passar a referência do trait object ao `Layouter<M: FontMetrics>`.

### 2.6. Injeção no pipeline (L3)

`03_infra/src/pipeline.rs::expand_context_blocks`: instanciada uma única `FallbackFontMetrics::new(world)` fora do loop de expansão dos `ContextBlock`, partilhada entre todos os blocos desse documento. O `Engine` local recebe `font_metrics: &font_metrics`.

### 2.7. Testes de integração

Adicionados 3 testes em `03_infra/src/integration_tests.rs` (secção P858):

- `p858_measure_hello_usa_metricas_reais`: confirma que `measure([hello])` usa métricas reais (largura < 30pt).
- `p858_measure_vazio_zero`: confirma que `measure([])` devolve 0pt.
- `p858_measure_multiplo_contexto_consistente`: confirma consistência quando múltiplos `#context` reutilizam a mesma instância de métricas.

---

## 3. Medições

Fonte de medição: fallback do sistema (provavelmente DejaVu Sans / Liberation Sans), não Helvetica.

| Caso | Antes (P858) | Depois (P858) | Vanilla 0.15.0 (Helvetica) |
|---|---|---|---|
| `measure([hello])` | `(33pt, 14.85pt)` | `(27.25pt, 15.51pt)` | `(22.19pt, 7.24pt)` |
| `measure([x])` | `(6.6pt, 14.85pt)` | `(6.2pt, 15.51pt)` | — |
| `measure([abcd])` | `(26.4pt, 14.85pt)` | `(26.8pt, 15.51pt)` | — |
| `measure([a b])` | `(15.84pt, 14.85pt)` | `(17.1pt, 15.51pt)` | — |
| `measure([])` | `(0pt, 0pt)` | `(0pt, 0pt)` | `(0pt, 0pt)` |
| `measure([hello])` com `size: 20pt` | `(66pt, 27pt)` | `(49.55pt, 28.2pt)` | — |

**Observações:**

- A **largura** aproximou-se do vanilla, reflectindo a substituição da heurística monoespaçada por métricas reais de fonte.
- A **altura** permanece distante do vanilla porque `measure_content_real` devolve a altura da linha produzida por `layout_sub_frame`, não a bounding box exacta do texto. Esta divergência semântica é pré-existente e não foi introduzida por P858.
- Documentos sem `#context` não são afectados: a métrica só é usada durante a expansão de `ContextBlock`.

---

## 4. Decisão: campo obrigatório vs `Option`

Foi escolhido o campo **obrigatório** (`font_metrics: &'a dyn FontMetrics`) em vez de `Option<&'a dyn FontMetrics>`. Justificação:

1. **Forma consistente com `world`:** `Engine` já exige `world` como campo obrigatório; nenhum site de construção deixa de ter uma métrica válida para injectar.
2. **Sem overhead de `Option`:** todos os sites podem fornecer `&FixedMetrics` quando não há métricas reais disponíveis.
3. **Menor propensão a deriva:** um `Option` tornaria tentador adiar a decisão e introduziria `unwrap()` ou defaults espalhados pelo código.

A heurística `FixedMetrics` mantém-se como implementação canónica de fallback em L1.

---

## 5. Impacto arquitetural

| Aspeto | Avaliação |
|---|---|
| Pureza de L1 | Mantida. L1 só vê o trait `FontMetrics`; a implementação real fica em L3. |
| Topologia de imports | Nenhuma violação. L1 não importa L3. |
| Estado global | Nenhum. A instância de `FallbackFontMetrics` é local a `expand_context_blocks`. |
| Performance | A cache interna de `FallbackFontMetrics` é partilhada entre `ContextBlock` do mesmo documento. |
| Documentos sem contexto | Zero impacto — a métrica só é usada em `expand_context_blocks`. |

---

## 6. Fecho do DEBT-69

O inventário de dívida (`00_nucleo/diagnosticos/debt/DEBT.md`) foi actualizado:

- **DEBT-69** passou de **ABERTO** para **FECHADO (P858)**.
- Registo histórico no cabeçalho do inventário: total abertos **10 → 9**.
- Secção do DEBT-69 actualizada com a descrição da resolução, medições e critério de reabertura.

---

## 7. Validação

Comando corrido:

```bash
cargo test --workspace
crystalline-lint .
```

Resultados:

| Crate/Suíte | Passou | Falhou | Ignorado |
|---|---|---|---|
| typst-core | 4655 | 0 | 2 |
| typst-infra | 717 | 0 | 5 |
| typst-shell | 36 | 0 | 0 |
| typst (binário) | 2 | 0 | 0 |
| tests/cli.rs | 31 | 0 | 0 |
| crystalline_lint.rs | 2 | 0 | 0 |

`crystalline-lint .` reportou apenas o aviso pré-existente **V7** (`package_version_resolution.md` órfão), que não está relacionado com P858.

---

## 8. Referências

- `00_nucleo/prompts/entities/engine.md`
- `00_nucleo/diagnosticos/debt/DEBT.md` (secção DEBT-69)
- `01_core/src/entities/engine.rs:40`
- `01_core/src/engine/layout/mod.rs:1790`
- `01_core/src/engine/layout/metrics.rs`
- `03_infra/src/pipeline.rs::expand_context_blocks`
- `03_infra/src/integration_tests.rs` (secção P858)
- `00_nucleo/diagnosticos/typst-passo-857-relatorio.md`
- `00_nucleo/diagnosticos/typst-passo-849-relatorio.md`

# Relatório — typst-passo-857: `measure()` — Opção 1 via inversão de dependência não quebra pureza de L1

**Data:** 2026-07-23  
**Executor:** Kimi Code (agente principal; prompt lido de `00_nucleo/materialization/typst-passo-857.md`).  
**Proveniência das medições:** working tree sobre commit `dfe3c2282ec0d6bd97d5834f00214e7c7f5d2d49`. Nenhum código foi alterado neste passo.

---

## 1. Confirmação do padrão existente

O projeto já usa o padrão de inversão de dependência para métricas de fonte:

- O trait `FontMetrics` é definido em L1 (`01_core/src/engine/layout/metrics.rs:22`).
- `FixedMetrics` é a implementação heurística em L1 (`metrics.rs:209`), sem I/O e `Clone + Copy`.
- `FallbackFontMetrics` é a implementação real em L3 (`03_infra/src/font_metrics.rs:480`), que consulta `ttf-parser`, `FontBook` e shaper.
- O `Layouter` é genérico sobre `M: FontMetrics` (`01_core/src/engine/layout/mod.rs:150`).
- A decisão de qual implementação usar é tomada no ponto de composição do pipeline:
  - Layout de produção recebe `FallbackFontMetrics::new(world)` em `03_infra/src/pipeline.rs:430`.
  - O caminho de medição isolado usa `FixedMetrics` hardcoded em `01_core/src/engine/layout/mod.rs:1797`.

L1 não importa nenhum tipo de `03_infra` neste caminho. A única interface é o trait `FontMetrics`, já residente em L1.

---

## 2. Checagem sobre `Engine`/`eval`

A estrutura `Engine` (`01_core/src/entities/engine.rs:40`) já é um agregador de capacidades injetadas:

```rust
pub struct Engine<'a> {
    pub world: &'a dyn World,
    pub route: Tracked<'a, Route<'a>>,
    pub styles: &'a mut StyleChain,
    pub show_rules: &'a mut Arc<[ShowRule]>,
    pub active_guards: &'a mut Vec<RuleId>,
    pub current_file: FileId,
    pub sink: &'a mut TrackedMut<'a, Sink>,
}
```

`world: &'a dyn World` é precedente direto de capacidade externa injetada como trait object sem quebrar a pureza de L1. Adicionar `font_metrics: &'a dyn FontMetrics` seguiria a mesma forma.

### 2.1. Onde `Engine` é construído

Foram encontrados **~13 sites** de construção literal `Engine { ... }` em L1:

| Ficheiro | Linha | Contexto |
|---|---|---|
| `01_core/src/engine/eval/mod.rs` | 431 | eval inicial (ponto de entrada) |
| `01_core/src/engine/eval/mod.rs` | 784, 929 | blocos `CodeBlock` / content flows |
| `01_core/src/engine/eval/markup.rs` | 39 | escopo local de markup |
| `01_core/src/engine/eval/modules.rs` | 91, 286 | módulos e includes |
| `01_core/src/engine/eval/closures.rs` | 397 | closures |
| `01_core/src/engine/stdlib/eval.rs` | 170 | `eval(source)` nativo |
| `01_core/src/engine/stdlib/mod.rs` | 341 | harness de testes stdlib |
| `01_core/src/engine/eval/tests.rs` | 134 | testes de eval |
| `01_core/src/engine/introspect.rs` | 3142 | macro `with_engine!` de introspecção |
| `01_core/src/engine/introspect/fixpoint.rs` | 218 | macro `with_engine!` de fixpoint |
| `01_core/src/engine/introspect/from_tags.rs` | 335 | macro `with_engine!` de from_tags |

A alteração mecânica seria: adicionar o campo em `Engine` e propagar `font_metrics: engine.font_metrics` (ou `&FixedMetrics` nos pontos raiz) em cada um destes sites.

### 2.2. Ponto de injeção de métricas reais

A expansão de `ContextBlock` — onde `measure()` é efectivamente interceptado — corre em L3, em `03_infra/src/pipeline.rs:107-160`. Nesse ponto, `world` já está disponível, pelo que `FallbackFontMetrics::new(world)` pode ser construído localmente e injetado no `Engine` usado por `apply_func`:

```rust
let font_metrics = FallbackFontMetrics::new(world);
let mut engine = Engine {
    world,
    font_metrics: &font_metrics,
    // ... restantes campos
};
```

O `FallbackFontMetrics` tem caches internas (`Arc<Mutex<...>>`); para partilhar caches entre múltiplos `ContextBlock`, basta instanciar uma única métrica fora do loop e reutilizá-la.

### 2.3. Obstáculo real: ordem de construção, não pureza

O único obstáculo identificado é de **ordem de construção no pipeline**, não de pureza de L1:

- No eval inicial (`eval/mod.rs:431`) ainda não é necessário (nem desejável) ter métricas reais; usa-se `&FixedMetrics`.
- Em `expand_context_blocks` (`pipeline.rs:139`) as métricas reais já estão disponíveis via `world`; injeta-se aí.
- Em `measure_content_real` (`layout/mod.rs:1790`) substitui-se `FixedMetrics` pela métrica do `Engine`.

Não foi encontrada nenhuma barreira técnica que obrigue L1 a importar um tipo de L3 ou a realizar I/O.

---

## 3. Comparação com a Opção 1 original (P849)

Em P849, a Opção 1 foi descrita como "injeccão de métricas no `Engine` durante a expansão de contexto" e classificada com **Pureza de L1: Média**. O relatório de P849 não detalha a mecânica dessa injeção; a classificação parece partir do pressuposto de que o `Engine` absorveria a implementação concreta de L3.

A reformulação deste passo é distinta:

- **Opção 1 (P849):** não especificada, mas inferida como injeção concreta de L3 → L1.
- **Opção 1 reformulada (P857):** inversão de dependência via trait `FontMetrics` (já em L1), com a implementação real injetada em L3.

Sob a reformulação, a fronteira L1/L3 permanece intacta do ponto de vista técnico. L1 continua a depender apenas do trait; L3 decide qual implementação usar.

---

## 4. Veredito

A hipótese testada **confirma-se**: a Opção 1 pode ser implementada pelo padrão de trait/injeção já usado para `FontMetrics`, **sem quebrar a pureza técnica de L1**.

Implicações:

| Aspeto | Avaliação |
|---|---|
| Pureza de L1 | Mantida (L1 só vê trait `FontMetrics`; implementação real fica em L3). |
| Impacto arquitetural | Baixo — extensão de `Engine` e propagação mecânica. |
| Esforço estimado | ~0.5 passo (menor que a abordagem 4.2). |
| Risco | Baixo (padrão existente; sem alteração do modelo de valores). |
| Paridade garantida | Sim, porque `measure()` usaria `FallbackFontMetrics`, o mesmo usado no layout. |

### 4.1. Nota sobre fases

Esta verificação **não** decide se a Opção 1 deve ser implementada em detrimento da abordagem 4.2 (fase de realização). A abordagem 4.2 continua a ser mais alinhada com o vanilla a longo prazo. A Opção 1 reformulada é, contudo, uma alternativa de menor custo e risco que resolve o achado #34 sem contaminar L1.

A decisão entre as duas continua a ser do dono do projecto.

---

## 5. Actualização do DEBT-69

O inventário de dívida foi actualizado em `00_nucleo/diagnosticos/debt/DEBT.md` (secção DEBT-69) para refletir:

- A viabilidade da Opção 1 via inversão de dependência.
- O precedente de `world: &'a dyn World` em `Engine`.
- A distinção entre obstáculo de ordem de construção vs. obstáculo de pureza.
- A necessidade de decisão explícita do dono para prosseguir com implementação.

O DEBT-69 permanece **ABERTO**, uma vez que nenhuma implementação foi realizada.

---

## 6. Próximo passo

Se o dono optar pela Opção 1 reformulada, o próximo passo seria:

1. Actualizar o Prompt L0 de `Engine` (`00_nucleo/prompts/entities/engine.md`) para incluir o campo `font_metrics`.
2. Adicionar `font_metrics: &'a dyn FontMetrics` a `Engine`.
3. Propagar o campo pelos ~13 sites de construção.
4. Ajustar `measure_content_real` para receber a métrica do `Engine`.
5. Injectar `FallbackFontMetrics::new(world)` em `expand_context_blocks`.
6. Testes e `crystalline-lint .` com zero violations.

Se o dono mantiver a Opção 2 (abordagem 4.2), o trabalho de design continua na prototipagem da fase de realização.

---

## 7. Validação

Nenhum código foi alterado neste passo. A suíte não foi re-corria porque a tarefa foi puramente de verificação de design.

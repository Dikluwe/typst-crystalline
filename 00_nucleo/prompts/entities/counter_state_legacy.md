# Prompt L0 — `entities/counter_state_legacy`
Hash do Código: 3d0ac75f

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/counter_state_legacy.rs`
**Criado em**: 2026-04-02 (Passo 57; nome original `counter_state.rs`)
**Renomeado em**: 2026-04-30 (P161 sub-passo .2 — liberta o nome `CounterState` para tipo de paridade vanilla a introduzir em M3)
**ADRs relevantes**: nenhum ADR dedicado; parte integrante do motor de introspecção single-pass (Passos 57–58)

---

## Contexto

`CounterStateLegacy` é o objecto de estado mutável que rastreia numeração de elementos durante o layout single-pass. Era originalmente `CounterState`; foi renomeado em P161 para libertar o nome `CounterState` para o tipo de paridade vanilla (`SmallVec<[u64; 3]>` em `Counter`, conforme `desenho-introspection-fixpoint.md` §2.1).

Não é alteração de comportamento. Apenas renomeação de tipo + ficheiro. Walk continua a operar sobre este struct via `&mut CounterStateLegacy`.

Suporta os mesmos três modos de contagem:

1. **Hierárquico** (`HashMap<String, Vec<usize>>`): numeração de headings multi-nível — `"heading"` → `[1, 2]` representa a secção `1.2`.
2. **Plano** (`HashMap<String, usize>`): contadores lineares para figuras, equações e chaves arbitrárias.
3. **Numeração activa** (`HashMap<String, bool>`): flag por chave que controla se a numeração está habilitada.

E todos os campos auxiliares acumulados nos Passos 60–159 (resolved_labels, headings_for_toc, label_pages, known_page_numbers, has_outline, is_readonly, figure_numbers, figure_label_numbers, local_figure_counters, lang, bib_entries, bib_numbers, auto_label_counter).

`CounterAction` (enum vizinho com `Step` / `Update(usize)`) **permanece neste ficheiro** com nome inalterado. P161 sub-passo .6 adapta o nome do tipo da família "instrução de contador" mas via criação de `CounterUpdate` em ficheiro próprio (`counter_update.rs`); ver `entities/counter_update.md`. `CounterAction` mantém-se para retrocompatibilidade dos call-sites existentes em `Content::CounterUpdate` variant.

DEBT-10 e divergência single-pass vs duas-passagens vanilla mantêm-se documentadas — não são alteradas por este passo.

---

## Restrições Estruturais

- Camada **L1**: zero I/O de sistema. Apenas `HashMap` e `Vec` em memória.
- Sem `Arc` partilhado mutável entre threads — uso restrito a single-threaded layout.
- Mutação só via métodos `step_*` / `update_*` / setters explícitos para que `is_readonly` possa bloquear globalmente.

---

## Interface pública

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum CounterAction {
    Step,
    Update(usize),
}

#[derive(Debug, Clone, Default)]
pub struct CounterStateLegacy {
    // Campos exactamente iguais aos do CounterState original.
    // Ver código actual em counter_state.rs antes da renomeação.
}

impl CounterStateLegacy {
    pub fn new() -> Self;
    pub fn is_numbering_active(&self, key: &str) -> bool;
    pub fn step_hierarchical(&mut self, key: &str, level: usize);
    pub fn format_hierarchical(&self, key: &str) -> Option<String>;
    pub fn step_flat(&mut self, key: &str);
    pub fn update_flat(&mut self, key: &str, value: usize);
    pub fn get_flat(&self, key: &str) -> usize;
    pub fn display_value(&self, kind: &str) -> String;
}
```

Sem alterações de assinatura face ao `CounterState` actual. Apenas o nome do tipo muda.

---

## Semântica

Inalterada. Ver `00_nucleo/prompts/entities/counter_state.md` antes da renomeação para histórico completo dos 12 fields agregados ao longo dos Passos 57–159.

Resumo:
- `step_hierarchical(key, level)`: avança contador hierárquico ao nível indicado, com truncamento e push.
- `step_flat(key)`: incrementa contador plano em 1.
- `update_flat(key, value)`: força contador plano para valor exacto.
- `display_value(kind)`: lê valor actual em formato textual (hierárquico → "1.2", plano → "5").
- `is_readonly` bloqueia mutações em qualquer step/update — usado durante renderização de TOC com clones de AST (Passo 63).

---

## Invariantes

- Nenhum field é redundante face ao output observable (P159G validado).
- `CounterStateLegacy::default()` produz instância vazia válida — usada como ponto inicial pelo Layouter.
- `Clone` é O(N) onde N = total de entries em hashmaps + tamanho de TOC + bib_entries; aceitável porque clone só ocorre na fronteira do walk de `materialize_time` (Passo 66, DEBT-18).

---

## Consumers actuais

- `01_core/src/rules/introspect.rs` (walk + materialize_time).
- `01_core/src/rules/layout/mod.rs` (Layouter campo `counter`).
- `01_core/src/rules/layout/counters.rs`, `references.rs`, `outline.rs`, `figure.rs` (leitura/escrita per feature).

---

## Sobre paridade

Vanilla decompõe esta funcionalidade por 13 ficheiros em `introspection/` (Counter, State, Introspector, Locator, Tag, etc.). Cristalino mantém um único agregador legacy enquanto os tipos de paridade são introduzidos progressivamente nos Passos P161+ (Location, Locator, Tag, ElementInfo, ElementPayload). Quando o motor de introspecção fixpoint estiver completo (M9 do desenho), `CounterStateLegacy` pode ser totalmente substituído ou retido como vista achatada do estado moderno.

Ver auditoria `00_nucleo/diagnosticos/auditoria-isolamento-vs-vanilla.md` (2026-04-29) para a classificação literal de `CounterState` (agora legacy) como concentrador de 12 razões orthogonais.

---

## Resultado Esperado

- `01_core/src/entities/counter_state_legacy.rs` — implementação idêntica ao actual `counter_state.rs`, só com `pub struct CounterStateLegacy` em vez de `pub struct CounterState`.
- Tests co-localizados em `#[cfg(test)]` permanecem inalterados — só os call-sites internos referem `CounterStateLegacy`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-02 | Criação — Passo 57: counter hierárquico para headings (nome original `CounterState`) | `counter_state.rs` |
| 2026-04-12 | Passo 58: contadores planos (`step_flat`, `update_flat`, `get_flat`), enum `CounterAction`, campo `numbering_active` | `counter_state.rs`, `counter_state.md` |
| 2026-04-13 | Passo 63: campos `label_pages`, `is_readonly`, motor de congelamento | `counter_state.rs`, `counter_state.md` |
| 2026-04-20 | Passos 60–75: TOC, figure_numbers, label_pages, known_page_numbers | `counter_state.rs`, `counter_state.md` |
| 2026-04-24..27 | Passos 131–159: lang, bib_entries, bib_numbers | `counter_state.rs`, `counter_state.md` |
| 2026-04-30 | P161 sub-passo .2: renomeação `CounterState` → `CounterStateLegacy` + ficheiro renomeado | `counter_state_legacy.rs`, `counter_state_legacy.md` |

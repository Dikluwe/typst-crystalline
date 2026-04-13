# Prompt L0 — `entities/counter_state`
Hash do Código: 70125601

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/counter_state.rs`
**Criado em**: 2026-04-02 (Passo 57)
**Atualizado em**: 2026-04-13 (Passo 63 — `label_pages`, `is_readonly`, motor de congelamento)
**ADRs relevantes**: nenhum ADR dedicado; parte integrante do motor de introspecção (Passos 57–58)

---

## Contexto

`CounterState` é o objeto de estado mutável que rastreia a numeração de
elementos durante o layout de passagem única (*Single-Pass*). Viaja com o
`Layouter` e é atualizado à medida que os nós são processados.

Suporta dois modos de contagem:

1. **Hierárquico** (`HashMap<String, Vec<usize>>`): numeração de headings
   multi-nível — `"heading"` → `[1, 2]` representa a secção `1.2`.
2. **Plano** (`HashMap<String, usize>`): contadores lineares para figuras,
   equações e chaves arbitrárias.
3. **Numeração activa** (`HashMap<String, bool>`): flag que controla se a
   numeração está habilitada por chave. Permite que `#set heading(numbering: none)`
   pause a numeração sem zerar o estado.

O cristalino diverge do Typst original aqui: o original resolve contadores
em duas passagens com `comemo` (para suportar referências para a frente).
Esta implementação usa uma única passagem — suficiente para numeração
sequencial sem referências para a frente.

**DEBT-10**: Resolver contadores em duas passagens com estado global quando o
motor de introspecção completo for implementado (Passos 60+).

---

## Restrições Estruturais

- Camada **L1**: zero I/O de sistema. Apenas `HashMap` e `Vec` em memória.
- `Default` derivado — estado inicial: todos os mapas vazios.
- Não tem dependências externas além de `std::collections::HashMap`.
  (`FxHashMap` pode ser adotado numa revisão futura se performance justificar,
  mas N é tipicamente pequeno — `HashMap` padrão é suficiente aqui.)
- O `Layouter` é o único produtor de mutações; nada em L2/L4 mutaciona
  diretamente `CounterState`.

---

## Estrutura de Dados

```rust
pub enum CounterAction {
    /// Avança o contador em 1 (flat) ou avança o nível (hierárquico).
    Step,
    /// Força o contador para o valor indicado.
    Update(usize),
}

#[derive(Debug, Clone, Default)]
pub struct CounterState {
    /// Contadores hierárquicos (ex: heading).
    /// Chave "heading" → [1, 2] representa a secção 1.2.
    hierarchical: HashMap<String, Vec<usize>>,
    /// Contadores planos (ex: equation, figure, ou chaves arbitrárias).
    flat: HashMap<String, usize>,
    /// Flags de numeração activa por chave.
    pub numbering_active: HashMap<String, bool>,
    /// Mapa de labels para o número de página onde aterraram.
    /// Populado por `layout_labelled` na Passagem 2 (draft).
    /// Injectado no estado da Passagem 3 (final) para a TOC mostrar páginas reais.
    pub label_pages: HashMap<Label, usize>,
    /// Modo read-only: bloqueia mutações de contadores (step_*, update_*).
    /// Activado durante a renderização de clones de AST na TOC (DEBT-13)
    /// para evitar que CounterUpdate dispare duas vezes.
    pub is_readonly: bool,
}
```

---

## Instrução

### Interface pública obrigatória

```rust
impl CounterState {
    pub fn new() -> Self { Self::default() }

    // ── Numeração activa ─────────────────────────────────────────────────
    pub fn is_numbering_active(&self, key: &str) -> bool

    // ── Contadores hierárquicos ──────────────────────────────────────────
    /// Avança o contador ao nível indicado (1-indexed).
    /// Trunca o vector para `level`, preenche com 0 se necessário, e incrementa.
    pub fn step_hierarchical(&mut self, key: &str, level: usize)

    /// Retorna "1", "1.2", "1.2.3" ou None se vazio.
    pub fn format_hierarchical(&self, key: &str) -> Option<String>

    // ── Contadores planos ────────────────────────────────────────────────
    pub fn step_flat(&mut self, key: &str)
    pub fn update_flat(&mut self, key: &str, value: usize)
    pub fn get_flat(&self, key: &str) -> usize
}
```

### Regra de read-only

`step_hierarchical`, `step_flat` e `update_flat` verificam `self.is_readonly`
no início e retornam imediatamente sem efeito se `true`. As leituras
(`format_hierarchical`, `get_flat`, `is_numbering_active`, `resolved_labels`,
`label_pages`) não são afectadas.

```rust
pub fn step_flat(&mut self, key: &str) {
    if self.is_readonly { return; }
    // ... lógica existente ...
}
```
```

### Regras de negócio de `step_hierarchical(key, level)`

```
level = level.max(1)              // nível mínimo é 1
counter.truncate(level)           // descarta níveis mais profundos que o alvo
if counter.len() < level:
    counter.resize(level - 1, 0) // preenche com 0 os níveis intermédios
    counter.push(1)              // inicia o nível alvo em 1
else:
    counter.last_mut() += 1      // incrementa o nível alvo
```

---

## Critérios de Verificação

```
Dado CounterState::new()
Então format_hierarchical("heading") = None

Dado step_hierarchical("heading", 1)
Então format_hierarchical("heading") = Some("1")

Dado step_hierarchical("heading", 1) → step_hierarchical("heading", 2)
Então format_hierarchical("heading") = Some("1.1")

Dado step_hierarchical("heading", 1) → step_hierarchical("heading", 2) → step_hierarchical("heading", 2)
Então format_hierarchical("heading") = Some("1.2")

Dado step_hierarchical("heading", 1) → step_hierarchical("heading", 2) → step_hierarchical("heading", 1)
Então format_hierarchical("heading") = Some("2")

Dado sequência 1, 2, 3, 2, 1
Então format_hierarchical("heading") = Some("2")

Dado step_flat("equation") → step_flat("equation")
Então get_flat("equation") = 2

Dado step_flat("figure") → update_flat("figure", 5)
Então get_flat("figure") = 5

Dado step_flat("equation") × 2 e step_flat("figure") × 1
Então get_flat("equation") = 2 e get_flat("figure") = 1
     (contadores são independentes entre si)
```

---

## Resultado Esperado

- `01_core/src/entities/counter_state.rs` — implementação e testes co-localizados
- Testes co-localizados em `#[cfg(test)]` no mesmo ficheiro

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-02 | Criação — Passo 57: counter hierárquico para headings | `counter_state.rs` |
| 2026-04-12 | Passo 58: adição de contadores planos (`step_flat`, `update_flat`, `get_flat`), enum `CounterAction`, campo `numbering_active` | `counter_state.rs`, `counter_state.md` |

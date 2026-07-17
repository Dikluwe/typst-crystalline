# Prompt L0 — `stdlib/context` — delayed evaluation via `context`
Hash do Código: 67a0cf1d

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/stdlib/context.rs` (novo; função exportada para `rules/stdlib/mod.rs` e registada em `rules/eval/mod.rs::make_stdlib`).
**Origem**: Passo 506 — fecho do gap P500 (runtime state mutável).
**ADRs**: ADR-0033 (paridade vanilla), ADR-0054 (graded parity), ADR-0107 (paridade linguagem), ADR-0118 (runtime state via context).
**Convenções partilhadas**: ver `00_nucleo/prompts/engine/stdlib/_comum.md`.

---

## 1. Visão geral

Este módulo implementa `context { expr }` — bloco de delayed evaluation que permite acessar runtime state (`state`, `counter`) no ponto onde o bloco aparece no documento.

A sintaxe vanilla suportada:

```typst
#let s = state("key", 0)
#s.update(5)
#context s.get()

#counter(heading).update(1)
#context counter(heading).get()
```

Paridade com Typst 0.15.0 para o subset identificado em P500/P506.

---

## 2. Construtor

### `native_context` — `context { expr }`

**Assinatura**: `context(body: function) -> content`

**Argumentos**:
- `body`: closure sem argumentos que captura o scope atual.

**Semântica**:
- Rejeita argumentos nomeados.
- O argumento deve ser `Value::Func` (closure).
- Não avalia `body` imediatamente.
- Retorna `Content::ContextBlock(Arc<ContextBlockElem>)`.
- `ContextBlockElem` contém:
  - `id: u64` — identificador sequencial gerado no eval para mapeamento estável na expansão.
  - `closure: Func` — corpo a avaliar posteriormente.

**Testes canônicos**:
```
context { 1 + 2 } -> Content::ContextBlock
context 1 + 2     -> Err "context() requer corpo em bloco"
```

---

## 3. Fase de expansão pós-introspecção

`context { expr }` é resolvido numa fase separada entre introspecção e layout/query.

### Algoritmo

1. Após `introspect_with_introspector(content)` produzir `TagIntrospector` populado:
   - percorrer as tags `Tag::Start(loc, info)` onde `info.payload == ElementPayload::ContextBlock { id }`;
   - construir mapa `context_locations: HashMap<u64, Location>`.

2. Avaliar cada ContextBlock:
   - para cada `(id, loc)` em `context_locations`:
     - construir `Args::positional(vec![])`;
     - criar `EvalContext` derivado do contexto original com:
       - `in_context = true`;
       - `introspector = intr.clone()`;
       - `current_location = Some(loc)`;
     - chamar `apply_func(closure, args, scopes_empty, ctx, engine)`;
     - converter o `Value` resultante para `Content`:
       - `Value::Content(c) => c`;
       - `Value::Str(s) => Content::text(s)`;
       - `Value::Int(i) => Content::text(i.to_string())`;
       - outros → `Content::Empty` (defensive).
     - armazenar `resolved: HashMap<u64, Content>`.

3. Substituir ContextBlocks no Content:
   - percorrer o Content recursivamente;
   - quando encontrar `Content::ContextBlock(elem)`, substituir por `resolved[&elem.id]`.

### Localização

A location associada a cada ContextBlock é a do `Tag::Start` emitido durante o walk. Isso garante que `state.value_at(key, loc)` e `counters.value_at(key, loc)` devolvam o valor acumulado até aquele ponto do documento.

---

## 4. Interação com `state` e `counter`

Dentro da expansão (`in_context = true`):

- `Value::State.get()` consulta `ctx.introspector.state.value_at(key, loc)`.
- `Value::Counter.get()` consulta `ctx.introspector.counters.value_at(key, loc)`.
- `Value::Counter.at(label)` consulta o valor na location do label.
- `Value::State.display()` e `Value::Counter.display()` formata o valor.

Fora de context (`in_context = false`), os métodos `.get()`/`.display()` produzem erro descritivo.

---

## 5. `Content::ContextBlock` no content tree

- É locatável: `extract_payload` retorna `ElementPayload::ContextBlock { id }`.
- É terminal no walk: não recursa no corpo (o corpo é uma closure, não content).
- É ignorado pelo layout se não expandido (defensive).

---

## 6. Paridade vanilla

- `context { expr }` delaya a avaliação de `expr` até o ponto do documento onde o bloco aparece.
- O bloco vê o runtime state acumulado até esse ponto.
- `state.get()`/`counter.get()` só funcionam dentro de `context`.

---

## 7. Scope-outs

- `context` com acesso a métricas de layout (`measure`, posições de página) — scope-out; este prompt cobre apenas runtime state.
- `context` aninhados — suportados se a implementação recursiva permitir, mas não são gate deste prompt.

---

## 8. Testes obrigatórios

- `context { 1 + 2 }` cria `Content::ContextBlock`.
- Após expansão, `context { 1 + 2 }` vira `Content::text("3")`.
- `context s.get()` retorna valor correto após `s.update(...)`.
- `context [O valor é #s.get()]` retorna content com texto correto.
- ContextBlock sem expansão não gera panic no layout.

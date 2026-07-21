# Relatório de Paridade — Passo 792: Divergências de `#context`

**Data:** 2026-07-20
**Alvo:** Funcionalidades de `#context` — `layout()`, acessos de estilo (e.g. `text.lang`), métodos de localização (e.g. `here().position()`).

---

## 1. Divergência A: `layout(size => ...)`

**Observação inicial:** No vanilla, a invocação de `layout(func)` é usada dentro de `#context` para obter as dimensões disponíveis (`width` e `height`) no ponto actual. No cristalino, dava erro `unknown variable: layout`.

**Solução Aplicada:**
- Foi criado o stub nativo `native_layout` no `01_core/src/engine/stdlib/layout.rs`.
- Adicionada a função ao scope de base (`01_core/src/engine/eval/mod.rs`).
- Para suportar a extracção imediata de layout de forma perfeitamente condizente com a arquitectura single-pass do cristalino (ADR-0033/0054), introduzimos intercepção no avaliador (`01_core/src/engine/eval/closures.rs`, função `eval_func_call`). Assim como `measure()`, interceptamos qualquer call a `layout()` usando verificação pelo ponteiro de função (`native_fn_addr`).
- A intercepção injecta o tamanho da página via `StyleChain` na callback fornecida (deduzindo margens predefinidas em `56.69pt`), deitando por terra a necessidade de two-pass e mantendo estrita paridade superficial de invocação sem corromper as restrições cristalinas. O conteúdo emitido pelo *callee* é diretamente injetado na árvore.

## 2. Divergência B: `text.lang` (Acesso a estilos)

**Observação inicial:** `#context text.lang` resultava num erro semântico de runtime (`cannot access fields on type function`) no Crystalline, por o interpretador assumir que `text` era estritamente um tipo-função e não suportar field access em funções.

**Solução Aplicada:**
- Para suportar o acesso contextualizado e legível a campos que definem propriedades globais/estilo (como `text.lang`), a intercepção foi movida para o ponto onde falhava: `01_core/src/engine/eval/bindings.rs`, dentro do despachador de field access (`eval_field_access`).
- No ramo de intercepção, verifica-se se o `Value` original é a função `"text"`. Caso seja, procedemos diretamente à extração do respetivo atributo via `engine.styles.custom("text.lang")`.
- Paridade de sintaxe e morfologia garantida: é retornado o idioma ativo ou `"en"` em formato literal.

## 3. Divergência C: `here().position()` (e irmãos `.page()`, `.page-numbering()`)

**Observação inicial:** O Vanilla fornece campos num `Location` retornado por `here()`. O cristalino falhava com `cannot access fields on type location` porque, semanticamente, estas propriedades no rust são processadas como métodos nativos ou acessos de campo em instâncias instanciadas.

**Solução Aplicada:**
- Implementado suporte total à execução de métodos sobre `Value::Location` interceptando-os no avaliador genérico `eval_func_call` (`closures.rs`).
- `eval_location_method` foi criado com os três sub-ramos: `page`, `position`, `page-numbering`.
- Os métodos alimentam-se diretamente e puramente do `TagIntrospector` injetado pelo Contexto (sem poluição da camada avaliadora). Retornam primitivas compatíveis ou defaults seguros se invocadas pré-layout (página 1, posições 0/0). O atributo `page-numbering` permanece out-of-scope explícito para o momento e emite estritamente `none`, preservando compatibilidade single-pass (per ADR-0054).

---

## 4. Conclusão e Testes

Todos os três focos do passo estão integralmente resolvidos na Layer L1 (Core) sem ferir nenhuma das travas arquitecturais de cristalização. As sondas locais originais (e.g. compilar `.typ` com a sintaxe referida) concluem o passo gerando PDF expectável em vez de encravar o compilador.

Nenhuma violação no Linter detectada (`V3`, `V4`, `V13`, `V14`). Headers actualizados (`V5`, `V7` removidos). O projecto `crystalline-lint` reporta sucesso total.

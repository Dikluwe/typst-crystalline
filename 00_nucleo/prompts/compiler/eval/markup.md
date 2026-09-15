# Prompt L0 — `compiler/eval/markup`
Hash do Código: bb4490fc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/markup.rs`

## Medição e contrato

Avalia strong, emph, heading, raw, link, list e enum item. Estilos locais são
empilhados em Engine reborrowed sem vazar; conteúdo atravessa show aplicável.
Morfologia e spans vêm do AST, sem antecipar layout.

## Aceitação

Corpo, destino, nível, marcador e estilo semântico preservam scopes e show.

## P1413 — raw atravessa intercepção (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `e7b2ddd44`, `eval_raw` é o único braço morfológico desta unidade
que não recebe `Scopes`, `EvalContext` e `Engine` nem chama
`intercept_content`; assim uma regra sobre as linhas sintetizadas não pode ser
observada pelo raw vindo da sintaxe dedicada.

### Decisão proposta

`eval_raw` conserva a extração atual de `text/lang/block`, mas entrega o
`RawElem` a `intercept_content` com o contexto vigente. O dispatcher passa as
três dependências já disponíveis. A unidade não divide linhas nem aplica
regras por conta própria; essa política permanece no owner de rules.

Esta ampliação de comportamento público aguarda confirmação humana explícita
antes de L1.

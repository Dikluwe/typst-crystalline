# P1166 — materialização do gate HTML e `html.elem`

**Data:** 2026-08-25  
**Baseline:** `30a6f11bcb8344f083a88edc8de89b3a1b5d6d07` + working tree P1165/P1166  
**Vanilla:** pin ratificado `a51e02804`  
**Estado:** GREEN

## Proveniência

A execução começou sobre a árvore documental não commitada de P1165/P1166.
O binário vanilla tinha SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
o cristalino pré-materialização tinha
`4213b506b9d23d2310850d52e35370cfef4178530295f5b0405f1614f0d3e759`.
Nenhuma medição usa a versão impressa como prova do pin.

## RED observado

Após introduzir a variante fechada `Content::HtmlElem`, `cargo check -p
typst-core` falhou em 13 matches não exaustivos. Isso confirmou que repr,
introspecção, layout, math spacing, plain-text e walkers ainda não tratavam o
novo contrato. Após os braços L1, o check revelou mais 2 consumidores L3 em
`query_helpers`. Todos foram tratados explicitamente antes do GREEN.

## Resultado materializado

- `Feature::Html` e `Features`, default vazio e ativação explícita;
- `--features html` em compile/watch/eval;
- binding `html` gated com diagnóstico e dois hints medidos;
- módulo público contendo somente `html.elem`;
- `HtmlElem` com tag, attrs ordenados opcionais e body opcional;
- `Content::HtmlElem` com integração exaustiva;
- propagação L2→L4→L3→L1 sem env/estado global em L1;
- compile HTML recusado sem feature e autorizado com ela;
- exporter preserva elemento, ordem/escape de attrs e body, sem wrapper de
  parágrafo no topo;
- `info.features.html` passou de true fixo para false por default.

A medição ratificada mostrou que construir `html.elem` em `eval` paged é
válido; o L0 foi corrigido antes de remover a restrição de target do código.

## Sondas binárias GREEN

| sonda cristalina | resultado |
|---|---|
| `eval repr(type(html))`, feature off | exit 1, diagnóstico gated + 2 hints |
| mesma com `--features html` | exit 0, `module` |
| `repr(html.elem("article", attrs: (lang: "pt"))[Olá])` | repr idêntico ao vanilla medido |
| compile HTML feature off | exit 1, exige `--features html` |
| compile HTML feature on | exit 0 + warning experimental |
| `info --format json` | `features.html: false` |
| elemento article com attrs/body | DOM semântico igual ao vanilla da fixture |

## Verificação

- `cargo build --workspace`: exit 0;
- `cargo test --workspace`: 5233 core + 847 infra + 55 shell + 2 wiring unit
  + 57 integração CLI + 2 testes do linter, todos aprovados; doc-tests sem
  falhas (3 ignorados já existentes);
- testes P1166 adicionados: 2/2 integração CLI e testes unitários de features/
  exporter aprovados;
- `crystalline-lint --fix-hashes .`: 20 consumers ressellados, reanálise com
  zero drift;
- `cargo fmt --all`: aplicado.

Warnings históricos do compilador/linter permanecem fora do escopo; não houve
falha de comando nem nova deriva de prompt.

## Scope-outs preservados

Tags tipadas (112), `html.frame`, void/raw completos, CSS, MathML, expansão
rica, introspecção/positions e os achados `<h1>/<h2>` e espaço antes de `<br>`
continuam abertos. O próximo passo é P1167, primeiro lote medido de tags
tipadas.

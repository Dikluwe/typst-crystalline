# P1168 — materializar o primeiro lote de tags HTML tipadas

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`
**Baseline cristalina:** working tree pós-P1167, ainda não commitada
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Gate herdado:** aprovado em 2026-08-25 para exatamente 12 bindings

## Objetivo

Ressellar os L0s aprovados e materializar, por metadados estáticos
reutilizáveis, exatamente:

```text
html.div, html.span, html.p,
html.h1, html.h2, html.h3, html.h4, html.h5, html.h6,
html.strong, html.em, html.ul
```

Cada função deve aceitar integralmente os 76 atributos globais tipados fixados
no L0 e `body: content` posicional opcional. Não implementar `ol`, `li`, `a`,
`br`, outras tags, `html.frame`, atributos `data-*`, void/raw ou CSS.

## 1. Proveniência e proteção da árvore

Antes de editar código, registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
sha256sum /usr/local/bin/typst ./target/debug/typst
```

Preservar integralmente P1165-P1167. Confirmar que nada está staged. Não
confundir a versão impressa do vanilla com prova do pin.

## 2. Fontes obrigatórias

Ler integralmente, sem entrar em `00_nucleo/context/` ou
`00_nucleo/materialization/`:

- `AGENTS.md` e `01_core/CLAUDE.md`;
- diagnóstico P1167 e os L0s `compiler/stdlib/html.md`, `entities/html.md`,
  `entities/content.md` e `infra/export/html.md`;
- ADR-0107, ADR-0108, ADR-0127 e ADR-0128;
- `01_core/src/compiler/stdlib/html.rs` e owners de args/casts/valores usados;
- `lab/typst-original/crates/typst-html/src/typed.rs`;
- entradas globais e das 12 tags no `typst-assets` pinado `94dcb99`.

## 3. Resselo antes do código

Executar `crystalline-lint --fix-hashes .` somente depois de confirmar o diff
L0 aprovado. Registrar exatamente os consumers ressellados e verificar que não
houve edição sem relação com os L0s. O resselo precede qualquer teste/código.

## 4. RED — contratos de língua primeiro

Adicionar testes antes da implementação e confirmar falha pelos bindings
ausentes. Cobrir no mínimo:

1. presença das 12 funções somente com `Feature::Html` ativa;
2. feature off preservando o diagnóstico gated existente;
3. chamada vazia e body content para cada binding, com repr `elem`;
4. nesting entre tags tipadas e `html.elem`;
5. todos os 76 named globais, ao menos uma sonda válida por variante de cast;
6. Presence `true` → valor vazio e `false` → atributo omitido;
7. listas com shorthand escalar e array, separadas por espaço;
8. unions `none`, boolean, enum e string conforme a matriz L0;
9. cast inválido representativo por família e named desconhecido;
10. `data-*` rejeitado;
11. DOM para `div/span/p/h1..h6/strong/em/ul` normal, aninhado e escapado;
12. confirmação de que nenhuma tag fora do lote apareceu.

Para a longa enumeração de `role`, testar primeiro/último token, um token
intermédio e rejeição fora da tabela; a tabela estática deve conter todos os 80
tokens aprovados, não só os exemplos.

Separar testes unitários de cast/metadados dos testes de integração de eval e
exporter. Registrar comando, exit code e erro RED causal.

## 5. Implementação L1

No owner `compiler/stdlib/html`:

- definir metadados estáticos dos 76 atributos, seus casts e enums;
- definir uma única descrição estrutural partilhada pelas 12 tags;
- registrar as 12 funções no módulo feature-gated;
- consumir named conhecidos, convertê-los para string canónica e preservar a
  ordem dos argumentos no `HtmlAttrs`;
- omitir atributo cujo cast resulte em ausência;
- consumir body content opcional e construir a entidade P1166;
- deixar o parser comum produzir erro para named/posicional restante;
- manter L1 puro, sem dependência de `lab`, `typst-assets`, I/O ou estado global
  mutável.

Não copiar a mecânica `LazyLock`/bump/vtable do vanilla. Dispatch estático e
tabela declarativa cristalina são permitidos. Não aceitar string como fallback
para casts estritos e não implementar exceções por nome de tag.

Se um tipo Typst necessário ao cast não existir no cristalino (por exemplo
`none`, `auto`, direção ou char), medir o modelo de valores vigente e usar o
owner L0 já aprovado. Se isso exigir novo contrato público, parar num novo gate
ADR-0127; não degradar silenciosamente o cast.

## 6. Exporter

As 12 tags são normais e já usam `Content::HtmlElem`; o exporter não deve
ganhar branches por binding. Validar que:

- abertura/body/fecho usam a tag preservada;
- atributos convertidos são escapados uma única vez;
- nesting não cria wrapper de parágrafo dentro ou fora do nó explícito;
- a ordem observável coincide com a fixture ratificada.

Alterar L3 apenas se um teste RED revelar gap coberto pelo L0 P1167. Não tocar
na serialização void; `<br></br>` permanece gap deliberado até P1171.

## 7. GREEN e regressão

Executar em ordem:

```text
cargo fmt --all -- --check
cargo test -p typst-core <testes HTML focados>
cargo test -p typst-infra <testes HTML focados>
cargo test -p typst-wiring --test cli <testes HTML focados>
cargo test --workspace
cargo build --workspace
crystalline-lint .
git diff --check
git diff --cached --stat
```

Repetir as sondas binárias vanilla/cristalino com feature on/off. Exigir zero
falhas e zero violações; warnings históricos devem ser separados de falhas.
Nada deve ficar staged.

## 8. Diagnóstico de materialização

Criar `00_nucleo/diagnosticos/typst-p1168-materializacao-tags-html-lote1.md`
com proveniência, hashes ressellados, RED observado, forma da tabela/casts,
sondas GREEN, totais de testes e scope-outs P1169-P1171. Todo número deve citar
HEAD/working tree conforme a regra de proveniência.

## Critérios de aceitação

- exatamente 12 bindings novos e nenhum outro;
- assinatura completa dos 76 globais, sem aceitação permissiva;
- body/repr/diagnósticos/DOM no nível da língua coincidem com o pin;
- feature off não regride;
- nenhuma mudança pública em `HtmlElem`/`Content`;
- exporter permanece genérico para tags normais;
- testes RED observados antes do código e suíte integral GREEN;
- L0s ressellados, lint sem violações, diff-check limpo e staging vazio.

## Próximo passo

Após P1168 GREEN, escrever P1169 para auditar/materializar `html.ol` e
`html.li` com `reversed`, `start`, `type` e `value`, reutilizando a tabela de
atributos globais sem ampliar casts por atalho.

# P1169 — auditar e nuclear `html.ol` e `html.li`

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GATE ADR-0127 APROVADO EM 2026-08-25`
**Baseline cristalina:** working tree P1168 GREEN, ainda não staged
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Gate:** ADR-0127 obrigatório antes de acrescentar `html.ol` e `html.li`

## Objetivo

Medir integralmente e nuclear os constructors tipados de lista ordenada:

```text
html.ol(reversed: ..., start: ..., type: ...)[body]
html.li(value: ...)[body]
```

Ambos reutilizam os 76 atributos globais e o `HtmlBody` triádico materializados
em P1168. Este passo confirma os quatro atributos específicos, repr,
diagnósticos, nesting e DOM, atualiza os L0s necessários e para no gate. Não
escreve Rust, testes RED, hashes, staging ou commit.

## Estado herdado que deve ser preservado

P1168 deixou GREEN:

- 12 tags normais tipadas;
- tabela única dos 76 globais e 80 roles;
- casts estritos, listas, enums, Presence, `none` e `auto`;
- `HtmlBody::{Unset, None, Content}`;
- atributo vazio emitido sem `=""`;
- feature HTML desligada por default;
- `ol`, `li`, `a`, `br` e `frame` ainda ausentes.

Uma fixture formatada entre expressões HTML ainda produz whitespace adicional
no cristalino. Esse eixo pertence a P1171, junto da interação de `br`; não o
resolver nem mascarar neste passo.

## 1. Proveniência obrigatória

Antes de medir:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
sha256sum /usr/local/bin/typst ./target/debug/typst
/usr/local/bin/typst --version
./target/debug/typst --version
```

Declarar a árvore como não commitada e registrar todos os paths P1168. O
índice deve continuar vazio. Não usar `--version` como prova do pin.

## 2. Fontes obrigatórias

Ler integralmente, sem listar ou acessar `00_nucleo/context/` e
`00_nucleo/materialization/`:

- `AGENTS.md` e `01_core/CLAUDE.md`;
- diagnóstico P1167 e diagnóstico de materialização P1168;
- `typst-passo-1168.md`;
- ADR-0107, ADR-0108, ADR-0127 e ADR-0128;
- L0s `compiler/stdlib/html.md`, `entities/html.md`, `entities/content.md` e
  `infra/export/html.md`;
- `01_core/src/compiler/stdlib/html.rs`;
- `lab/typst-original/crates/typst-html/src/typed.rs`;
- entradas `ol`, `li`, atributos 181/201/214/220, `ATTR_STRINGS[229..234]` e
  os 76 globais no `typst-assets` pinado `94dcb99`.

## 3. Revalidar a assinatura ratificada

A medição P1167 encontrou:

```text
ol: reversed = Presence, start = Int, type = Strings(229, 234)
li: value = Int
ATTR_STRINGS[229..234] = "1", "a", "A", "i", "I"
```

Reconfirmar na fonte pinada e por sondas. Não inferir esses tipos da
especificação HTML geral.

Medir no vanilla, com `--features html`:

```typst
repr(type(html.ol))
repr(type(html.li))
repr(html.ol())
repr(html.li())
repr(html.ol[html.li[X]])
repr(html.ol(reversed: true, start: -2, type: "A")[X])
repr(html.li(value: -3)[X])
```

Usar sintaxe de código que realmente avalie o `html.li` aninhado, sem deixá-lo
virar texto markup literal.

## 4. Matriz de casts e erros

### `ol.reversed`

Medir:

- omitido;
- `true` → atributo presente com valor vazio no repr e `reversed` no DOM;
- `false` → atributo ausente;
- `none`, `0` e string → diagnóstico inválido.

### `ol.start`

Medir zero, positivo, negativo e limites relevantes de `int`. Confirmar
decimalização e erro para float/string/none. Não impor positividade por
semântica HTML se a tabela Typst aceita `Int` irrestrito.

### `ol.type`

Medir os cinco valores válidos `1`, `a`, `A`, `i`, `I`; confirmar repr/DOM.
Medir `"x"`, string vazia, bool e none. Registrar a mensagem enumerada e span.

### `li.value`

Medir zero, positivo e negativo; confirmar `Int` irrestrito, decimalização e
erros de tipo.

Para ambos, confirmar ainda:

- um atributo global de cada classe continua funcionando;
- atributo específico da outra tag é rejeitado (`ol.value`, `li.start`);
- named desconhecido e `data-*` continuam rejeitados;
- dois posicionais e body de tipo inválido mantêm a classe de erro.

## 5. Morfologia, nesting e exporter

Medir no vanilla e depois no cristalino P1168:

1. chamadas vazias: `body: none`, não unset;
2. `ol` contendo um e vários `li` reais;
3. `li` contendo texto e tags do lote P1168;
4. lista aninhada;
5. attrs específicos misturados com globais, preservando a ordem fornecida;
6. fixture compacta sem whitespace estrutural, para comparação byte a byte;
7. fixture formatada somente como controle do gap P1171, sem corrigi-lo aqui.

O DOM esperado para o caso canónico deve ser medido, não pressuposto, por
exemplo:

```html
<ol start="2" reversed type="A"><li value="3">X</li></ol>
```

Confirmar que o exporter existente não precisa conhecer `ol` ou `li`: ambos
são tags normais e devem seguir abertura/body/fecho genéricos.

## 6. Repetição no cristalino P1168

Com feature off, confirmar o diagnóstico gated existente. Com feature on,
espera-se antes da implementação:

- `html.ol` e `html.li` ausentes;
- `html.elem("ol")`/`html.elem("li")` capazes de aproximar o DOM somente com
  attrs string, mas sem assinatura/casts tipados;
- os 12 bindings P1168 continuam GREEN.

Registrar comando, stdout, stderr e exit code. `html.elem` não transforma
binding tipado ABSENT em MATCH.

## 7. Decisão e L0

Produzir
`00_nucleo/diagnosticos/typst-p1169-auditoria-html-listas-tipadas.md` com:

```text
binding | atributo | tipo/cast vanilla | repr | diagnóstico |
estado P1168 | dependência | entra no lote?
```

Somente após a medição:

1. atualizar `00_nucleo/prompts/compiler/stdlib/html.md` com as assinaturas
   completas de `ol` e `li`, casts e erros;
2. atualizar `entities/html.md` apenas se a representação triádica existente
   for insuficiente;
3. atualizar `infra/export/html.md` apenas se houver observável novo além da
   serialização normal já aprovada;
4. não atualizar `entities/content.md` sem mudança real no enum/contrato.

Os L0s devem declarar explicitamente que `a`, `br`, outras tags e whitespace
continuam fora. Não ressellar hashes neste passo.

Então **PARAR NO GATE ADR-0127** e pedir aprovação objetiva para exatamente os
dois novos bindings públicos e quatro named específicos. Qualquer mudança
adicional de entidade, default ou pipeline exige ser declarada separadamente.

## Classificação ADR-0107/0108

São língua: presença dos bindings, assinatura, casts, unset/Presence, body,
repr, diagnóstico e DOM. São mecânica: array de metadados, função de lookup,
wrappers Rust e compartilhamento da tabela global.

Inferência a testar: `HtmlElem`, `HtmlBody` e o exporter normal de P1168 são
suficientes, sendo necessária apenas a extensão declarativa da tabela de tags
e atributos específicos. Refutador: qualquer diferença de repr/DOM que exija
estado não preservado ou semântica contextual própria.

## Entrega obrigatória

1. proveniência reproduzível;
2. matriz completa dos quatro atributos específicos;
3. sondas vanilla e cristalinas feature off/on;
4. nesting e fixtures compacta/formatada;
5. classificação MATCH/PARTIAL/ABSENT antes do código;
6. diagnóstico P1169;
7. L0s atualizados sem hashes;
8. lista exata do contrato no gate;
9. nenhuma alteração L1-L4, teste, staging ou commit.

## Critérios de aceitação

- `Int` permanece irrestrito onde a tabela declara `Int`;
- `ol.type` aceita exatamente cinco tokens;
- Presence `false` omite e `true` preserva atributo vazio;
- atributos específicos não vazam entre tags;
- os 76 globais são reutilizados sem duplicação ou regressão;
- `HtmlBody::None` preserva chamada tipada vazia;
- exporter não recebe branches ad hoc para `ol`/`li`;
- whitespace permanece scope-out explícito de P1171;
- `git diff --check` limpo e staging vazio;
- parada efetiva no gate ADR-0127.

## Próximo passo após aprovação

Escrever **P1169.1** para resselo, testes RED e materialização GREEN de
`html.ol` e `html.li`. A numeração fracionária preserva o roadmap já registado:
P1170 continua reservado para `html.a` e P1171 para `br`/void/whitespace.

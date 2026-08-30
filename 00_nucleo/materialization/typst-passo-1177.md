# P1177 — auditar família HTML ruby

**Data:** 2026-08-25
**Estado:** `EXECUTADO — PARADO NO GATE ADR-0127`
**Baseline cristalina:** working tree P1176.1 GREEN, ainda não staged
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Gate:** ADR-0127 obrigatório antes de acrescentar bindings públicos

## Objetivo

Auditar e nuclear conjuntamente:

```text
ruby rp rt
```

Confirmar assinatura global-only e medir a morfologia própria da família:
`ruby` display ruby, `rt` ruby-text e `rp` display none. Determinar se o
exporter genérico atual preserva nesting, grouping e whitespace ou se exige
correção interna. Atualizar L0s após a evidência e parar no gate ADR-0127 antes
de resselo, testes ou código público.

## 1. Proveniência

Registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
git diff --cached --quiet
sha256sum /usr/local/bin/typst ./target/debug/typst
/usr/local/bin/typst --version
./target/debug/typst --version
```

Declarar a working tree P1173.1–P1176.1, enumerar paths e provar índice vazio.
Versão impressa não substitui pin/hash do binário.

## 2. Fontes obrigatórias

Ler integralmente, sem acessar/listar `00_nucleo/context/` ou
`00_nucleo/materialization/`:

- `AGENTS.md`, `01_core/CLAUDE.md`;
- ADR-0107, ADR-0108, ADR-0109, ADR-0127 e ADR-0128;
- P1167–P1176.1 e diagnósticos HTML relevantes;
- L0s `compiler/stdlib/html.md`, `entities/html.md` e
  `infra/export/html.md`, com hashes vigentes;
- owners L1/L3 atuais;
- vanilla pinado: `typed.rs`, `tag.rs`, `property.rs`, conversão/DOM e dados
  das três entradas;
- inventário P1140.26 apenas como índice auxiliar.

Se assets gerados não estiverem materializados, triangular lockfile,
inventário e binário sem inventar linhas.

## 3. Assinatura e categorias

Comparar cada lista integralmente com `html.div`:

```text
tag | total | globais | específicos | body/input | void/raw |
display | phrasing/flow | content model | decisão
```

Critério global-only:

- mesmos 76 globais, ordem, casts e metadados;
- body content posicional opcional;
- zero específicos;
- não void/raw/escapable-raw;
- nenhuma validação contextual no constructor.

Não confundir content model (`rt`/`rp` filhos de ruby) com validação real da
API. Uma divergência remove somente a tag afetada.

## 4. Sondas vanilla

Por tag:

```typst
repr(type(html.TAG))
repr(html.TAG())
repr(html.TAG[X])
repr(html.TAG(id: "k", class: "c", hidden: true)[X])
```

Medir `href`, `value`, `start`, unknown named, segundo body e body named.
Construir `rp` e `rt` isolados para provar ou refutar validação parental.
Confirmar que chamadas vazias preservam `body: none`.

## 5. Morfologia ruby

Comparar vanilla tipado com cristalino genérico em fixtures compactas e
formatadas:

1. `ruby[漢 rt[kan]]`;
2. fallback completo `ruby[漢 rp[(] rt[kan] rp[)]]`;
3. múltiplos pares base/anotação;
4. `ruby`, `rt` e `rp` vazios;
5. cada tag isolada no topo;
6. família entre phrasing siblings com e sem conteúdo;
7. whitespace antes/depois de `rt`/`rp` e entre famílias;
8. attrs vazios e escaping.

Responder:

- somente `ruby` deve ser agrupável em `<p>`, ou `rt`/`rp` isolados também?
- `rp` display none atua como boundary top-level ou inline dentro de ruby?
- espaços ao redor de filhos vazios exigem pre-wrap, descarte ou literal?
- o exporter precisa conhecer `Ruby`/`RubyText`, ou preservar tags basta?
- `GROUPABLE_PHRASING_TAGS`, `BLOCK_TAGS` e boundary top-level atuais
  coincidem com o vanilla?

Qualquer correção L3 deve ser especificada separadamente antes do código.

## 6. Baseline cristalina

Com feature off, preservar o gate. Com feature on:

- provar `ruby`, `rp`, `rt` ABSENT antes de código;
- provar os 55 bindings até P1176.1 presentes;
- comparar `html.elem` sem promover equivalência DOM a MATCH de assinatura;
- classificar semântica, sintaxe e morfologia por tag.

Bindings, erros, repr, nesting e DOM são língua; wrappers/tabelas Rust são
mecânica.

## 7. Decisão após medição

Hipótese inicial: os três constructors usam
`native_typed_html(tag, &[], args)` e `TYPED_TAGS`; nenhuma entidade muda.
Refutadores: body distinto, validação parental, realização ruby bespoke,
estado novo ou regra L3 ainda ausente.

Continuam fora:

- documento/raw: `html`, `head`, `body`, `title`;
- tabela: `caption`, `table`, `tbody`, `tfoot`, `thead`, `tr`.

Não antecipar essas famílias.

## 8. Diagnóstico, L0 e gate

Criar:

```text
00_nucleo/diagnosticos/typst-p1177-auditoria-html-familia-ruby.md
```

Após medir:

1. atualizar `compiler/stdlib/html.md` com as sobreviventes e assinatura;
2. atualizar `infra/export/html.md` somente se houver observável novo;
3. atualizar `entities/html.md` somente se a representação falhar;
4. não ressellar e não escrever testes/código L1–L4.

Parar e solicitar aprovação ADR-0127 para exatamente os bindings
sobreviventes. A aprovação não cobre documento, tabela, outros constructors,
entidade, default, pipeline ou correções não especificadas.

## Critérios de aceitação

- assinatura integral e content model medidos;
- `rp`/`rt` isolados testados;
- nesting, vazio, grouping e whitespace ruby comparados;
- 55 bindings anteriores e feature gate preservados;
- decisão posterior à evidência;
- L0 atualizado sem resselo/código;
- `git diff --check` limpo e índice vazio;
- parada efetiva no gate ADR-0127.

## Próximo passo após aprovação

Escrever P1177.1 para resselo, RED e materialização GREEN somente da família e
das correções internas explicitamente nucleadas.

## Resultado da execução

As três tags sobreviveram à auditoria de assinatura: 76 globais mais body
Content posicional opcional, sem específicos ou validação parental. O vanilla
agrupa somente `ruby`; `rp` e `rt` isolados são boundaries. A comparação
diferencial encontrou uma correção L3 localizada: descartar espaço somente
entre dois filhos `rp`/`rt` dentro de `ruby`, preservando espaços junto à base
e a inline comum. Os L0s L1 e L3 foram atualizados sem resselo; entidade não
mudou. Nenhum teste ou código L1–L4 de P1177 foi escrito.

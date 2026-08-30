# P1176 — auditar lote residual normal HTML

**Data:** 2026-08-25
**Estado:** `EXECUTADO — PARADO NO GATE ADR-0127`
**Baseline cristalina:** working tree P1175.1 GREEN, ainda não staged
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Gate:** ADR-0127 obrigatório antes de acrescentar bindings públicos

## Objetivo

Auditar e nuclear o lote residual de constructors aparentemente normais:

```text
datalist noscript summary
```

Confirmar se as três tags possuem somente os 76 globais e body content
opcional, sem realização especial de constructor. Medir separadamente as
categorias incomuns — `datalist` display none, `noscript` inline/phrasing e
`summary` block — e qualquer efeito no exporter. Atualizar os L0s necessários
e parar no gate ADR-0127 antes de resselo, testes ou código público.

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

Declarar a working tree herdada P1173.1–P1175.1, listar todos os paths e
confirmar índice vazio. O hash pinado prova a referência; `--version` sozinho
não prova.

## 2. Fontes obrigatórias

Ler integralmente, sem acessar/listar `00_nucleo/context/` ou
`00_nucleo/materialization/`:

- `AGENTS.md`, `01_core/CLAUDE.md`;
- ADR-0107, ADR-0108, ADR-0109, ADR-0127 e ADR-0128;
- P1167–P1175.1 e diagnósticos HTML relevantes;
- L0s `compiler/stdlib/html.md`, `entities/html.md` e
  `infra/export/html.md`, com hashes vigentes;
- owners L1/L3 atuais;
- vanilla pinado: `typed.rs`, `tag.rs`, `property.rs`, conversão/realização e
  dados das três entradas;
- inventário P1140.26 apenas como índice auxiliar.

Se os assets gerados não estiverem materializados, triangular lockfile,
inventário e binário sem inventar `file:line`.

## 3. Composição e assinatura

Comparar cada parâmetro integralmente com `html.div`, não apenas a contagem:

```text
tag | total | globais | específicos | body/input | void/raw |
display | categoria HTML | regra de realização | decisão
```

Critério para compartilhar global-only:

- mesmos 76 globais em nome, ordem, casts e metadados;
- body content posicional opcional;
- zero específicos;
- não void/raw/escapable-raw;
- nenhuma validação contextual ou transformação própria do constructor.

Uma divergência retira apenas a tag afetada. Display none não significa body
ausente; categoria HTML não deve ser confundida com assinatura Typst.

## 4. Sondas vanilla

Para cada tag, medir:

```typst
repr(type(html.TAG))
repr(html.TAG())
repr(html.TAG[X])
repr(html.TAG(id: "k", class: "c", hidden: true)[X])
```

Testar `href`, `value`, `start`, named desconhecido, segundo body e body named.
Confirmar classes e spans de erro por tag, sem extrapolar. Para `noscript`,
medir se body continua Content e se target/feature altera o constructor.

## 5. Morfologia e regras especiais

Comparar vanilla tipado com cristalino genérico em fixtures compactas e
formatadas:

1. cada tag vazia e com texto;
2. cada tag isolada no topo;
3. `datalist` e `noscript` entre phrasing siblings visíveis;
4. `summary` entre block siblings e dentro de `html.elem("details")`;
5. whitespace antes/depois e entre versões vazias;
6. atributos vazios, escaping e nesting com lotes anteriores.

Responder com evidência:

- `datalist` display none ainda é agrupado em parágrafo?
- conteúdo de `datalist` aparece no DOM apesar de invisível por CSS default?
- `noscript` recebe tratamento condicionado a scripting ou é nó genérico?
- `summary` é boundary block na formação de parágrafo e na proteção de espaço?
- `BLOCK_TAGS` e `GROUPABLE_PHRASING_TAGS` cristalinos cobrem exatamente os
  observáveis medidos?

Qualquer correção interna L3 exige L0 antes do código e deve permanecer
separada do consentimento público.

## 6. Baseline cristalina

Com feature off, preservar o gate. Com feature on:

- provar as três tags ABSENT antes do código;
- provar os 52 bindings materializados até P1175.1 presentes;
- comparar `html.elem` sem promover aproximação DOM a MATCH de assinatura;
- classificar semântica, sintaxe e morfologia por tag.

São língua: binding, assinatura, casts, erros, repr, DOM e comportamento
display observável. São mecânica: wrappers Rust, macro, tabela e algoritmo.

## 7. Decisão após medição

Hipótese inicial: as sobreviventes usam `native_typed_html(tag, &[], args)` e
`TYPED_TAGS`; nenhuma muda entidade. Refutadores: específico, body distinto,
raw/void, validação contextual, tratamento target-dependent ou estado novo.

As famílias restantes ficam fora:

- documento/raw: `html`, `head`, `body`, `title`;
- tabela: `caption`, `table`, `tbody`, `tfoot`, `thead`, `tr`;
- ruby: `ruby`, `rp`, `rt`.

Não puxar membros dessas famílias para P1176 mesmo se a cardinalidade for 77.

## 8. Diagnóstico, L0 e gate

Criar:

```text
00_nucleo/diagnosticos/typst-p1176-auditoria-html-tags-residuais-normais.md
```

Após medir:

1. atualizar `compiler/stdlib/html.md` com exatamente as sobreviventes;
2. atualizar `infra/export/html.md` apenas se houver regra nova;
3. atualizar `entities/html.md` somente se a representação falhar;
4. não ressellar e não escrever testes/código L1–L4.

Parar e pedir aprovação ADR-0127 para os bindings sobreviventes. A aprovação
não cobre famílias fora, correções não especificadas, novo `AttrKind`,
entidade, default ou pipeline.

## Critérios de aceitação

- assinatura integral das três tags medida no pin;
- categorias incomuns verificadas no DOM, não presumidas;
- whitespace vazio e `summary` boundary comparados;
- 52 bindings anteriores e feature gate preservados;
- L0 posterior à evidência;
- zero resselo/código;
- `git diff --check` limpo e índice vazio;
- parada efetiva no gate ADR-0127.

## Próximo passo após aprovação

Escrever P1176.1 para resselo, RED e materialização GREEN somente das tags
confirmadas e das correções internas explicitamente nucleadas.

## Resultado

As três candidatas sobreviveram. Foram nucleadas duas correções L3: `summary`
como boundary block também em bodies e descarte de espaço top-level adjacente
a HtmlElem não agrupável. Nenhum hash foi ressellado e nenhum teste/código
L1–L4 foi escrito. P1176 está parado no gate ADR-0127.

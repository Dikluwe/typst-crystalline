# P1170 — auditar e nuclear `html.a`

**Data:** 2026-08-25  
**Estado:** `EXECUTADO — PARADO NO GATE ADR-0127`  
**Baseline cristalina:** working tree P1169.1 GREEN, ainda não staged  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Gate:** ADR-0127 obrigatório antes de acrescentar o binding público `html.a`

## Objetivo

Medir integralmente e nuclear o constructor tipado `html.a`, incluindo os oito
atributos específicos apontados pelo inventário P1167, os 76 globais já
materializados, body, repr, diagnósticos, nesting e DOM. Atualizar somente os
L0s cuja medição exigir e **parar no gate ADR-0127** antes de escrever Rust,
testes RED, hashes ou código de produção.

Este passo não materializa `html.a`. Também não inclui `br`, tabela void,
whitespace, resolução de links Typst, `html.frame` nem qualquer outra tag.

## 1. Proveniência e preservação da árvore

Antes da primeira sonda, registrar:

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

Declarar explicitamente `working tree não commitado`, listar os paths
alterados de P1168–P1169.1 e confirmar índice vazio. A prova do vanilla é o
pin `a51e02804` mais o hash do binário; `--version` isolado não prova origem.
Não sobrescrever, stagear nem reorganizar os hunks herdados.

## 2. Fontes obrigatórias

Ler integralmente, sem acessar ou listar `00_nucleo/context/` e
`00_nucleo/materialization/`:

- `AGENTS.md` e `01_core/CLAUDE.md`;
- ADR-0107, ADR-0108, ADR-0127 e ADR-0128;
- P1167, P1169, P1169.1 e os diagnósticos correspondentes;
- L0s `compiler/stdlib/html.md`, `entities/html.md`, `entities/content.md` e
  `infra/export/html.md`;
- `01_core/src/compiler/stdlib/html.rs`;
- `lab/typst-original/crates/typst-html/src/typed.rs`;
- entrada `a`, seus oito atributos específicos, suas faixas de strings e os
  76 globais no `typst-assets` pinado `94dcb99`.

Localizar a fonte efetivamente resolvida pelo lockfile; não substituir a tabela
gerada pela memória da especificação HTML nem por documentação web.

## 3. Medição estrutural antes da decisão

Registrar `file:line` para:

1. nome e índice da entrada `a`;
2. os oito índices de atributos específicos;
3. nome, `AttrType` e eventual faixa/list separator de cada atributo;
4. todas as strings das faixas referenciadas;
5. concatenação dos 76 globais;
6. classificação de `a` como normal, não void e não raw;
7. criação do parâmetro body content posicional opcional em `typed.rs`.

Os candidatos esperados pelo inventário — a confirmar, corrigir ou refutar na
fonte — são `download`, `href`, `hreflang`, `ping`, `referrerpolicy`, `rel`,
`target` e `type`. Não decidir casts, cardinalidade ou enums pelo nome.

## 4. Sondas vanilla ratificadas

Executar com `/usr/local/bin/typst eval --features html`, registrando expressão,
stdout, stderr e exit code:

```typst
repr(type(html.a))
repr(html.a())
repr(html.a[X])
repr(html.a(id: "k", href: "https://example.test")[X])
```

Para cada um dos oito atributos específicos, medir:

- omitido;
- pelo menos um valor válido de cada ramo do tipo;
- extremos ou tokens de enum relevantes;
- tipo errado (`none`, bool, int, string ou array conforme o caso);
- valor lexical inválido quando houver enum ou lista restrita;
- interação com atributo global antes/depois dele, preservando ordem.

Para listas, medir shorthand escalar, array vazio, um item, vários itens e item
contendo o separador proibido. Para Presence/uniões, medir cada ramo. Para
strings livres, confirmar que string vazia e conteúdo não URL não recebem
validação externa não declarada. Nunca inferir validação de URL pelo nome
`href` ou `ping`.

## 5. Diagnósticos e isolamento de assinatura

Confirmar e registrar:

- named desconhecido e `data-*`;
- atributo específico de `ol`/`li` aplicado a `a`;
- atributo de `a` aplicado a `ol` e `li`;
- dois posicionais;
- body de tipo inválido;
- classe, texto e span de cada cast inválido;
- se enum inválida enumera tokens e se tipo errado acrescenta `found TYPE`.

Um atributo aceito por `html.elem("a", attrs: ...)` não prova que pertence à
assinatura tipada. Separar rigorosamente morfologia genérica de contrato
público tipado.

## 6. Morfologia, nesting e DOM

Medir no vanilla:

1. chamada vazia preservando `body: none`;
2. texto e conteúdo tipado P1168 dentro de `a`;
3. `a` dentro de `div`, `li` e `ol`;
4. atributos específicos e globais misturados na ordem de chamada;
5. escaping de valores com `&`, aspas e caracteres não ASCII;
6. fixture HTML compacta com todos os oito específicos em valores válidos;
7. fixture formatada apenas como controle do gap de whitespace P1171.

Comparar a fixture compacta por bytes com a aproximação cristalina via
`html.elem("a", attrs: ...)`, somente quando os casts puderem ser representados
como strings. Não atribuir ao constructor tipado a resolução de links nativos
feita por show rules em `typst-html/src/rules.rs`; testar se há diferença e
classificá-la antes de decidir.

Inferência inicial: `a` é um nó HTML normal e o exporter genérico já preserva
todos os observáveis necessários. Refutador: sonda que demonstre estado,
normalização ou serialização contextual não representável por `HtmlElem`.

## 7. Baseline cristalina P1169.1

Com feature off, confirmar o diagnóstico gated vigente. Com feature on:

- `html.a` deve continuar ausente;
- `html.elem("a")` deve continuar disponível;
- os 14 bindings tipados P1168/P1169.1 devem permanecer presentes;
- nenhum atributo específico de `a` deve vazar para `ol` ou `li`.

Classificar por semântica, sintaxe e morfologia:

```text
binding | atributo | tipo/cast vanilla | repr | diagnóstico |
DOM | estado cristalino | MATCH/PARTIAL/ABSENT | evidência file:line
```

O uso de tabela estática, lookup encadeado e wrappers Rust é mecânica; presença
do binding, assinatura, casts, body, repr, erros e DOM são língua.

## 8. Diagnóstico e atualização L0

Criar
`00_nucleo/diagnosticos/typst-p1170-auditoria-html-a.md` com proveniência,
fontes, matriz completa, sondas, classificação e scope-outs.

Somente após todas as medições:

1. atualizar `00_nucleo/prompts/compiler/stdlib/html.md` com a assinatura
   completa medida de `html.a`;
2. atualizar `entities/html.md` apenas se `HtmlElem`/`HtmlBody` forem
   comprovadamente insuficientes;
3. atualizar `infra/export/html.md` apenas se existir observável de export novo;
4. não alterar `entities/content.md` sem mudança real de representação;
5. declarar explicitamente que `br`, void, whitespace, demais tags, raw,
   `frame`, CSS, MathML e positions continuam fora.

Não ressellar hashes neste passo. Não escrever testes ou código L1–L4.

## 9. Gate ADR-0127

Depois do diagnóstico e do L0, **PARAR** e pedir aprovação objetiva para:

- exatamente um novo binding público, `html.a`;
- exatamente os oito named específicos confirmados, com seus casts medidos;
- reutilização dos 76 globais e body content opcional existentes;
- nenhuma alteração de entidade, exporter, default ou pipeline, salvo se a
  medição tiver refutado essa premissa e o gate declarar a mudança separada.

Qualquer nono atributo, alias, fallback string, `data-*`, normalização de URL,
resolução de link ou mudança de whitespace fica fora do consentimento implícito.

## Critérios de aceitação

- fonte pinada e vanilla concordam sobre os oito atributos;
- toda faixa de strings/lista é enumerada a partir da fonte;
- casts válidos e inválidos têm prova reproduzível;
- `body: none`, nesting, ordem e escaping são medidos;
- resolução de link Typst é separada do constructor `html.a`;
- estado cristalino é classificado sem usar `html.elem` como falso MATCH;
- L0 é atualizado antes de qualquer código;
- `git diff --check` limpo e índice vazio;
- parada efetiva no gate ADR-0127.

## Próximo passo após aprovação

Escrever **P1170.1** para resselo, testes RED e materialização GREEN de
`html.a`. P1171 permanece reservado para `br`, tabela void e whitespace.

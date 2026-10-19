# P1173 — auditar segundo lote global-only de tags HTML tipadas

**Data:** 2026-08-25  
**Estado:** `EXECUTADO — PARADO NO GATE ADR-0127`  
**Baseline cristalina:** working tree P1172.1 GREEN, ainda não staged  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Gate:** ADR-0127 obrigatório antes de acrescentar novos bindings públicos

## Objetivo

Auditar e nuclear um segundo lote candidato de 12 tags normais sem atributos
específicos:

```text
abbr address article aside b bdi bdo cite code dfn i kbd
```

Confirmar na fonte pinada e no vanilla que cada uma possui somente os 76
globais e body content opcional. Atualizar o L0 e parar no gate antes de
resselo, testes ou código. Qualquer tag com atributo específico, body raw,
classificação void ou semântica contextual é removida do lote e encaminhada a
passo próprio.

## 1. Proveniência

Registrar antes de medir:

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

Declarar working tree não commitado P1168–P1172.1, listar todos os paths e
confirmar índice vazio. Preservar os hunks. O pin/hash prova o vanilla;
`--version` sozinho não prova.

## 2. Fontes obrigatórias

Ler integralmente, sem acessar/listar `00_nucleo/context/` ou
`00_nucleo/materialization/`:

- `AGENTS.md`, `01_core/CLAUDE.md`;
- ADR-0107, ADR-0108, ADR-0109, ADR-0127 e ADR-0128;
- P1167–P1172.1 e diagnósticos relevantes;
- L0s `compiler/stdlib/html.md`, `entities/html.md` e
  `infra/export/html.md`;
- `01_core/src/compiler/stdlib/html.rs` e `03_infra/src/export/html.rs`;
- vanilla `typst-html/src/typed.rs`, `tag.rs`, `property.rs`;
- as 12 entradas candidatas, seus índices de atributos e os 76 globais em
  `typst-assets` pinado `94dcb99`;
- inventário `superficie-linguagem-p1140.26.json` apenas como índice auxiliar,
  nunca como substituto dos casts da fonte/binário.

Se a fonte da dependência não estiver materializada, registrar a limitação,
provar o pin pelo lockfile e triangular nomes/assinaturas por inventário +
binário ratificado. Não inventar `file:line`.

## 3. Provar a composição do lote

Para cada candidata, registrar:

```text
tag | entrada data.rs | attrs específicos | globais | void/raw |
body | display default | phrasing/groupable | decisão de lote
```

Critério de entrada cumulativo:

- zero atributos específicos;
- concatenação exata dos 76 globais;
- não void;
- não raw nem escapable-raw;
- body content posicional opcional;
- nenhuma regra própria no constructor tipado.

Uma divergência remove somente a tag afetada; não relaxar o lote. Medir
separadamente display/phrasing porque afeta export, mas não confundir categoria
DOM com assinatura do constructor.

## 4. Sondas vanilla por tag

Com `/usr/local/bin/typst eval --features html`, executar para cada tag:

```typst
repr(type(html.TAG))
repr(html.TAG())
repr(html.TAG[X])
repr(html.TAG(id: "k", class: "c", hidden: true)[X])
```

Confirmar:

- `function`;
- chamada vazia com `body: none`;
- body content real;
- casts globais reutilizados;
- ordem `id`, `class`, `hidden` preservada após omissões;
- named específico de outra tag (`href`, `value`, `start`) rejeitado;
- `data-*` e named desconhecido rejeitados;
- body inteiro, dois posicionais e named body mantêm classes de erro medidas.

Não repetir exaustivamente os 76 casts já fechados em P1168; usar sentinelas
de classes distintas e proteger o compartilhamento da tabela.

## 5. Morfologia e export

Medir fixtures compactas e formatadas:

1. cada tag isolada com texto;
2. nesting representativo, por exemplo
   `article[address[b[...]]]`, `aside[code[...]]`, `abbr[i[...]]`;
3. tags phrasing candidatas no topo, comprovando wrapper `<p>` P1172.1;
4. tags block candidatas no topo, comprovando ausência de wrapper;
5. phrasing dentro de block, sem `<p>` interno indevido;
6. mistura com `a`, `br`, `ol/li`, `strong/em` e whitespace formatado;
7. escaping e atributo vazio.

Comparar vanilla com o cristalino pré-implementação via `html.elem("TAG")`.
Um DOM aproximável por `html.elem` não transforma binding tipado ABSENT em
MATCH. Confirmar se a tabela phrasing/block P1172.1 já cobre todas as tags do
lote e registrar qualquer erro de classificação.

## 6. Baseline cristalina

Com feature off, preservar o diagnóstico gated. Com feature on:

- as 12 candidatas devem continuar ausentes antes do código;
- os 16 bindings tipados já materializados devem continuar presentes
  (12 P1168 + `ol`, `li`, `a`, `br`);
- `html.elem` deve produzir a mesma entidade genérica;
- nenhuma candidata deve aceitar aliases/fallback.

Classificar por tag:

```text
binding | assinatura | repr | DOM | estado cristalino |
MATCH/PARTIAL/ABSENT | evidência | entra no lote?
```

## 7. Decisão arquitetural posterior à medição

Hipótese inicial: o lote exige somente wrappers estáticos apontando para
`native_typed_html(tag, &[], args)` e entradas em `TYPED_TAGS`; nenhuma mudança
de AttrKind, entidade ou exporter.

Refutadores:

- atributo específico encontrado;
- categoria raw/void;
- body não-content;
- repr que exija estado além de `HtmlBody`;
- DOM que exija nova regra fora das tabelas void/phrasing existentes.

São língua: bindings, assinaturas, casts, body, repr, erros e DOM. São mecânica:
macro/wrapper Rust, array de tags e compartilhamento do dispatcher.

## 8. Diagnóstico e L0

Criar
`00_nucleo/diagnosticos/typst-p1173-auditoria-html-tags-global-only-lote2.md`
com proveniência, matriz completa, sondas, classificação e scope-outs.

Somente após medir:

1. atualizar `00_nucleo/prompts/compiler/stdlib/html.md` com a lista exata e
   assinatura integral do lote confirmado;
2. atualizar `infra/export/html.md` apenas se houver observável novo;
3. atualizar `entities/html.md` somente se a representação for insuficiente;
4. não alterar `entities/content.md` sem mudança real;
5. declarar todas as demais tags fora.

Não ressellar hashes e não escrever testes/código L1–L4 nesta auditoria.

## 9. Gate ADR-0127

Parar e pedir aprovação para exatamente os bindings que sobreviverem à
medição, cada um com 76 globais e body content opcional. A aprovação não cobre:

- atributos específicos;
- tags retiradas do lote;
- raw/void;
- novo AttrKind;
- entidade, exporter, default ou pipeline;
- qualquer outro constructor.

## Critérios de aceitação

- composição de cada tag provada, não presumida;
- zero específicos e body content confirmados;
- repr/erros/DOM medidos por classe representativa;
- classificação phrasing/block validada contra P1172.1;
- estado pré-código é ABSENT por binding;
- L0 atualizado antes do código e sem resselo;
- `git diff --check` limpo e índice vazio;
- parada efetiva no gate ADR-0127.

## Próximo passo após aprovação

Escrever P1173.1 para resselo, RED e materialização GREEN somente do lote
confirmado. Tags com atributos específicos seguem para auditorias próprias.

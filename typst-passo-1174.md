# P1174 — auditar terceiro lote global-only de tags HTML tipadas

**Data:** 2026-08-25
**Estado:** `EXECUTADO — PARADO NO GATE ADR-0127`
**Baseline cristalina:** working tree P1173.1 GREEN, ainda não staged
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Gate:** ADR-0127 obrigatório antes de acrescentar bindings públicos

## Objetivo

Auditar e nuclear o terceiro lote candidato de tags HTML normais:

```text
dd dl dt figcaption figure footer header hgroup legend main mark menu
```

Confirmar, antes de decidir, que cada candidata possui exatamente os 76
atributos globais de P1168 e body content opcional, sem atributos específicos,
regra raw/void ou constructor especial. Atualizar os L0s estritamente
necessários e parar no gate ADR-0127 antes de testes ou código público.

## 1. Proveniência obrigatória

Registrar no início:

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

Declarar explicitamente a working tree não commitada herdada, enumerar os
paths alterados e provar se o índice está vazio. O hash pinado e o SHA-256 do
binário estabelecem a referência; a string `--version` não basta.

## 2. Fontes e estado vigente

Ler integralmente, sem acessar nem listar `00_nucleo/context/` ou
`00_nucleo/materialization/`:

- `AGENTS.md` e `01_core/CLAUDE.md`;
- ADR-0107, ADR-0108, ADR-0109, ADR-0127 e ADR-0128;
- P1167–P1173.1 e diagnósticos HTML correspondentes;
- L0s `compiler/stdlib/html.md`, `entities/html.md` e
  `infra/export/html.md`, conferindo os hashes vigentes;
- `01_core/src/compiler/stdlib/html.rs` e `03_infra/src/export/html.rs`;
- vanilla pinado: `typst-html/src/typed.rs`, `tag.rs`, `property.rs` e as
  entradas geradas/asset das 12 candidatas;
- `00_nucleo/diagnosticos/superficie-linguagem-p1140.26.json` somente como
  índice auxiliar.

Não inventar `file:line` se a fonte gerada não estiver materializada. Nesse
caso, registrar a limitação e triangular lockfile, inventário e binário.

## 3. Medir a composição por tag

Produzir a matriz:

```text
tag | total params | globais | específicos | body | void/raw |
display default | phrasing/groupable | regra contextual | decisão
```

Comparar mecanicamente nomes, ordem e metadados dos parâmetros contra
`html.div`, não apenas a cardinalidade. Uma candidata só permanece se tiver:

- os mesmos 76 globais, sem nomes extras ou ausentes;
- body content posicional opcional;
- zero atributos específicos;
- categoria normal, não void, raw nem escapable-raw;
- nenhuma realização especial no constructor tipado.

Qualquer divergência retira apenas a tag afetada. Regras de conteúdo HTML
(`dd/dt` em `dl`, `legend` em agrupamentos, `figcaption` em `figure`) devem
ser registradas como semântica contextual, sem presumir validação no constructor.

## 4. Sondas no vanilla ratificado

Para cada candidata, executar com `/usr/local/bin/typst eval --features html`:

```typst
repr(type(html.TAG))
repr(html.TAG())
repr(html.TAG[X])
repr(html.TAG(id: "k", class: "c", hidden: true)[X])
```

Confirmar function, `body: none`, body content, casts globais e ordem dos
atributos. Provar a ausência de específicos tentando ao menos `href`, `value`
e `start`. Medir também named desconhecido, dois bodies posicionais e body
named para uma representante de cada classe de display; não extrapolar a
mensagem de erro sem evidência.

## 5. Morfologia e export HTML

Compilar no vanilla fixtures compactas e formatadas que cubram:

1. cada candidata vazia e com texto;
2. `dl[dd/dt]`, `figure[figcaption]` e um uso de `legend`;
3. `header`, `footer`, `main`, `menu` e `hgroup` como boundaries block;
4. `mark` no topo e dentro de block, confirmando grouping phrasing;
5. whitespace entre block siblings e entre phrasing siblings;
6. nesting com tags P1168–P1173.1, atributo vazio e escaping.

Reproduzir o DOM pré-implementação com `html.elem("TAG")` no cristalino e
comparar o observável. Confirmar se `BLOCK_TAGS` e
`GROUPABLE_PHRASING_TAGS` já classificam as 12 corretamente. Um DOM genérico
equivalente não torna o binding tipado presente.

## 6. Baseline cristalina e classificação

Com feature off, preservar o gate HTML existente. Com feature on:

- provar que as 12 candidatas estão ausentes antes de código;
- provar que todos os bindings fechados até P1173.1 permanecem presentes;
- confirmar que `html.elem` aproxima a morfologia sem criar assinatura tipada;
- registrar qualquer divergência de wrapper ou whitespace separadamente.

Classificar cada tag em semântica, sintaxe e morfologia como MATCH, PARTIAL ou
ABSENT. Macro Rust, tabela estática e forma do dispatcher são mecânica; binding,
assinatura, casts, erros, repr e DOM são língua.

## 7. Decisão somente após a medição

Hipótese inicial, ainda não ratificada: as sobreviventes exigem apenas wrappers
estáticos para `native_typed_html(tag, &[], args)` e entradas em `TYPED_TAGS`.
Refutam a hipótese: atributo específico, body distinto, raw/void, validação
contextual no constructor, novo estado de entidade ou regra nova de exporter.

Se surgir correção interna de paridade, atualizar primeiro o L0 dono e seguir
fluxo contínuo somente para essa correção. Não misturá-la com a autorização dos
bindings públicos.

## 8. Diagnóstico, L0 e gate

Criar:

```text
00_nucleo/diagnosticos/typst-p1174-auditoria-html-tags-global-only-lote3.md
```

Depois das medições:

1. atualizar `00_nucleo/prompts/compiler/stdlib/html.md` com a lista exata das
   sobreviventes e sua assinatura completa;
2. atualizar `infra/export/html.md` apenas se houver novo observável de export;
3. atualizar `entities/html.md` somente se a representação atual for
   insuficiente;
4. declarar tags retiradas e todo o restante fora do escopo;
5. não ressellar hashes e não escrever testes ou código L1–L4.

Parar e solicitar aprovação ADR-0127 para exatamente os bindings que
sobreviverem. A aprovação não cobre tags retiradas, atributos específicos,
novo `AttrKind`, entidade, default, pipeline ou qualquer outro constructor.

## Critérios de aceitação

- composição das 12 candidatas medida contra o vanilla pinado;
- intenção não inferida apenas do comportamento;
- regras contextuais separadas da assinatura pública;
- repr, erros e DOM medidos por classes representativas;
- estado cristalino pré-código classificado por tag;
- L0 atualizado somente após as evidências;
- nenhuma alteração de código ou resselo;
- `git diff --check` limpo e índice preservado;
- parada efetiva no gate ADR-0127.

## Próximo passo após aprovação

Escrever P1174.1 para resselo, RED e materialização GREEN somente das tags
confirmadas. Qualquer candidata com especificidade segue para passo próprio.

## Resultado da execução

As 12 candidatas sobreviveram. O diagnóstico e a proposta L0 foram escritos;
nenhum hash foi ressellado e nenhum teste ou código L1–L4 foi alterado por
P1174. A execução está parada antes do contrato público, aguardando aprovação
explícita do gate ADR-0127.

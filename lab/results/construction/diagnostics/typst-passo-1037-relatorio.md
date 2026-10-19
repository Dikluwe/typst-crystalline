# Passo 1037 — `#context`: um defeito fechado, outro localizado e escalado

**Data**: 2026-08-13
**Estado**: **parcialmente fechado**. O achado #1 de P1031 continha **dois** defeitos com
causas distintas — a investigação separou-os por medição. O primeiro (descida parcial dos
walks de `ContextBlock`) está corrigido e testado. O segundo (show rules não estabelecem
contexto) tem agora a cadeia causal localizada por `file:line`, e fica no gate ADR-0127.

---

## Proveniência

`HEAD = 0c8b64a41` (P1033); árvore não commitada, com as alterações de P1036 já aplicadas
quando estas medições foram feitas (por isso o binário cristalino usado aqui **não** é o de
`0c8b64a41` puro — é o dessa árvore). Vanilla `/usr/local/bin/typst`, md5
`36da18895eeb5e0136c068a7634e3f82`. Medições 18:05–18:35 -03:00. Documentos em
`…/scratchpad/p1037`.

---

## Fase A — os dois casos do enunciado, reproduzidos

| forma | vanilla | cristalino |
|---|---|---|
| `#show heading: it => [Nº #counter(heading).get().first() — #it.body]` | `Nº 1 — Alpha Nº 2 — Beta` | **erro** `counter.get() can only be used inside context` |
| a mesma, com `#context` explícito | `Nº 1 — Alpha Nº 2 — Beta` | `Nº — Alpha Nº — Beta` — **vazio** |

Ambos como o enunciado descrevia. O segundo é o que ele chama *"a parte mais estranha do
achado"*, e foi por aí que a investigação começou.

## Fase B — o vazio silencioso não era um defeito de show rules

A sonda que separou as coisas: a **mesma** expressão `#context counter(heading).get()` em
cinco posições, **sem show rule nenhuma**.

| posição | vanilla | cristalino (antes) |
|---|---|---|
| topo da sequência | `(2,)` | `(2,)` ✅ |
| `#emph[…]` | `(2,)` | `(2,)` ✅ |
| `#par[…]` | `(2,)` | `(2,)` ✅ |
| `#box[…]` | `(2,)` | **vazio** ❌ |
| item de lista `- …` | `(2,)` | **vazio** ❌ |

O padrão bate exactamente com a whitelist de `collect_context_blocks`
(`03_infra/src/pipeline.rs`): Sequence, Styled, Strong, Emph, Heading. Um `ContextBlock` que
o walk não visita nunca entra em `resolved`, e `substitute_context_blocks` troca por
`Content::Empty` todo o bloco cujo `id` não esteja lá. Daí "silenciosamente vazio".

O L0 chamava a isto um scope-out benigno — *"`ContextBlock` não é esperado aninhado dentro
de Grid/Table/etc. neste subset"*. A premissa é falsa: `#box[…]` e itens de lista são uso
corrente, e a consequência não é conteúdo em falta, é conteúdo **errado sem aviso**.

**Correcção**: os dois walks passam a delegar a descida ao `map_content` de L1 — o `match`
exaustivo que já é a fonte única da forma da árvore — em vez de reenumerarem containers em
L3. Era essa duplicação de conhecimento entre camadas que produzia o defeito.

## A inferência que a medição refutou (ADR-0108)

A hipótese de trabalho era que o mesmo walk parcial explicasse **também** o `#context` vazio
dentro de show rules — as duas coisas terminavam em `Content::Empty`, o que tornava a
hipótese cómoda. **É falsa.** Com os dois walks já exaustivos, a suite verde e o binário
reconstruído, os três documentos do achado #1 saem **exactamente como antes**. Os dois
defeitos partilham a última linha do caminho, não a causa.

## A causa do segundo defeito, localizada

A introspecção corre sobre a árvore **pré-show-rules**; a substituição, sobre a
**pós-show-rules**:

1. `01_core/src/entities/module.rs:96` — `introspection_content` é, literalmente,
   *"conteúdo original (pré-show-rules) para introspecção"*;
   `01_core/src/compiler/eval/mod.rs:458-474` guarda aí `original_content` (P498).
2. `03_infra/src/pipeline.rs` — `intr_content = module.introspection_content()`, logo
   `intr.context_block_locations` só conhece blocos anteriores às show rules.
3. `expand_context_blocks` itera `for (id, loc) in &intr.context_block_locations`. Um
   `ContextBlock` **criado pela** show rule recebe `id` novo (`ctx.next_context_id()`,
   `eval/mod.rs:1174`) que nunca está nesse mapa.
4. Não entrando em `resolved`, é substituído por `Content::Empty`.

E a face sem `#context` (o erro) é o gate `ctx.in_context`
(`compiler/stdlib/counter.rs:132`) mais a ausência de `current_location` — que as show
rules, corridas no `eval`, não têm.

**As duas faces fecham na mesma mudança**: dar ao corpo de uma show rule uma `Location` e um
introspector que o conheçam, isto é, introspectar o produto das show rules. Isso move
trabalho entre `eval` e `introspect` → **mudança de fase do pipeline, ADR-0127 ponto 3**,
paragem obrigatória. Reabre ainda o argumento de terminação antecipada de
`compiler/eval/show_rule_termination.md` §3 — cujo gatilho de reabertura essa secção já
declarava activo, e que agora tem causa nomeada.

## Testes

Quatro novos em `03_infra/src/integration_tests.rs`, todos RED antes da correcção (os quatro
devolviam vazio) e GREEN depois:

- `p1037_context_dentro_de_box_resolve`
- `p1037_context_dentro_de_item_de_lista_resolve`
- `p1037_context_dentro_de_grid_resolve`
- `p1037_context_aninhado_nao_perde_a_cadeia_de_estilos` — guarda de não-regressão de
  **P711**: `#set text(size: 20pt)` + `#box[#context (1em).to-absolute()]` tem de dar `20pt`,
  não o default. A descida exaustiva não podia perder a `StyleChain` da posição; o walk
  reentra no caminho consciente da cadeia ao encontrar um `Styled`, e como `map_content` é
  bottom-up a reinserção posterior sobrepõe-se, ficando a cadeia correcta.

## O que ficou escrito

- **`infra/pipeline.md` §P1037** — a tabela das cinco posições, a inferência refutada, a
  cadeia causal do segundo defeito por `file:line`, e o bloco escalado com as duas faces.
- **`compiler/eval/show_rule_termination.md`** — nota a apontar para a causa localizada e a
  afinar a classificação do gate: o ponto aplicável é o **3** (fase do pipeline), não só o 2.
- Código: `03_infra/src/pipeline.rs` (`collect_context_blocks` +
  `collect_context_blocks_into` novos, `substitute_context_blocks` reescrita sobre
  `map_content`), `03_infra/src/integration_tests.rs` (4 testes).

## Validação

```
crystalline-lint .       → 0 erros; 0 V5; 3 avisos V7 pré-existentes
cargo test --workspace   → 5864 passed; 0 failed  (4989 + 793 + 41 + 2 + 37 + 2)
```

Revalidação ponta-a-ponta: as cinco posições da sonda passam a dar `(2,)` no cristalino,
iguais ao vanilla.

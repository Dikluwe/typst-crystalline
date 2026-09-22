# Prompt L0 — `compiler/stdlib/structural/lists` — nativas de lista
Hash do Código: b3c1776e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/lists.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/{list,enum,terms}.rs`. A co-mudança agrega-as: P494-P495 e P505 movem `native_list` e `native_enum` sempre em conjunto.

---

## Contexto

As três formas de lista da língua. `list` e `enum` partilham a totalidade dos parâmetros
de indentação (`indent`, `body-indent`, `tight`) e a mesma validação; `terms` é a forma
por pares chave→descrição.

## Instrução

| Nativa | Assinatura |
|---|---|
| `list` | `list(..items, marker: ?, marker-align: ?, indent: ?, body-indent: ?, tight: ?)` |
| `enum` | `enum(..items, numbering: ?, start: ?, indent: ?, body-indent: ?, tight: ?)` |
| `terms` | `terms(named: descrição, …)` — a **ordem dos argumentos nomeados é preservada** |

Validação partilhada (P505): `indent`/`body-indent` têm de ser comprimentos; `tight` tem
de ser booleano; nome nomeado desconhecido é erro. `enum` propaga `start` para a
numeração.

## Restrições Estruturais

- L1 puro.
- `terms` depende da ordem de inserção dos nomeados — não reordenar nem usar mapa não
  ordenado.

## Critérios de Verificação

```
- a
- b                          → list com 2 itens, sem marker
+ a
+ b                          → enum com 2 itens
#enum(start: 3)[a]           → numeração começa em 3
#list(indent: 1)             → Err (não é comprimento)
#list(tight: 1)              → Err (não é bool)
#list(zz: 1)                 → Err (nomeado desconhecido)
#enum(numbering: 1)          → Err
#list()                      → lista vazia
```

## P1473 — `list.item`/`enum.item`/`terms.item` qualificados; `terms()` ganha forma posicional (MATERIALIZADO)

### Medição anterior à decisão

Sonda bilateral (2026-09-21, vanilla ratificado `586e1bd43fae`): `type(list.item)`,
`type(enum.item)` e `type(terms.item)` são `function` no vanilla, `function <nome> does
not contain item` no cristalino (`list`/`enum`/`terms` já são `Value::Func` bare, sem
namespace — não `Value::Module`, ao contrário de `pdf`/`html` em P1471).

Fonte pinada (`lab/typst-original/crates/typst-library/src/model/{list,enum,terms}.rs`):

- `ListItem { #[required] body: Content }` — repr medido `item(body: [hi])`; a entidade
  cristalina `entities::elements::list_item::ListItemElem` já tem exatamente este campo
  mais cinco opcionais de override (`marker`/`marker_align`/`indent`/`body_indent`/
  `tight`, todos `None` = herdar do `list` pai) — nenhum campo novo necessário.
- `EnumItem { #[positional] number: Smart<u64>, #[required] body: Content }` — os dois
  positional são casados por **tipo**, não por ordem: `enum.item(3, [a])` e
  `enum.item([a], 3)` produzem o mesmo `item(number: 3, body: [a])` (medido). A entidade
  cristalina `EnumItemElem` já tem `number: Option<u32>` + `body: Content` (mais os
  mesmos três opcionais de override de `list`) — compatível, nenhum campo novo.
- `TermItem { #[required] term: Content, #[required] description: Content }` — a
  entidade cristalina `Content::TermItem(TermItemElem { term, description })`
  (`entities/content.rs:2228-2229`) já espelha isto exatamente.

**Achado — divergência real em `terms()`, não só rota ausente:** o vanilla aceita
`terms(...)` como **variádica posicional** de `TermItem` (`#terms(terms.item([A], [1]),
terms.item([B], [2]))`, ou a sintaxe `/ termo: descrição`); `terms(chave: descrição)`
com chave nomeada é **rejeitado** no vanilla (`error: unexpected argument: <chave>`,
medido). O cristalino atual (`native_terms`, `structural/lists.rs:27-58`) só aceita a
forma nomeada e rejeita qualquer positional (`args.items.is_empty()` obrigatório) — esta
é a forma **documentada acima** neste L0 (`terms(named: descrição, …)`), não um bug
silencioso; mas sem `terms.item` como constructor não há como produzir a forma posicional
que o vanilla realmente usa. Fechar `terms.item` sem estender `terms()` deixaria a rota
qualificada existir mas inútil (nada consegue montar uma lista de termos com ela).

### Decisão

1. `list`/`enum`/`terms` passam de `Func::native(<nome>, native_<nome>)` para
   `Func::native_with_namespace(<nome>, native_<nome>, Arc::new(scope))`, cada um com
   namespace contendo exatamente um membro `item`. Local do registo:
   `compiler/eval.md` (`eval/mod.rs`), ver nota lá — mesmo mecanismo P493 já usado por
   `place.flush`/`table.header`/P1471/P1472, sem novo tipo.
2. `list.item(body)`: um `Content` posicional obrigatório, sem named. Constrói
   `Content::ListItem(ListItemElem { body, marker: None, marker_align: None, indent:
   None, body_indent: None, tight: None })` — os cinco opcionais ficam `None` (herdam do
   `list` pai quando o item entra numa lista real; fora desse contexto, o item isolado
   simplesmente não tem override). Diagnósticos: corpo ausente → `missing argument:
   body`; segundo positional → `unexpected argument`; `body:` nomeado →
   ``the argument `body` is positional`` + hint `try removing \`body:\``; tipo errado →
   `expected content, found <tipo>`.
3. `enum.item(number?, body)`: aceita dois positionals, casados por tipo — o primeiro
   `Int` encontrado (se houver) vira `number: Some(n as u32)`, o `Content` vira `body`
   (obrigatório). Constrói `Content::EnumItem(EnumItemElem { number, body, numbering:
   None, indent: None, body_indent: None, tight: None })`. `body` ausente → `missing
   argument: body`; primeiro positional `Float` (não casa `Int` nem `Content`) →
   `expected content, found float` (medido: o cast tenta o slot `body` quando o
   valor não é `Int`, não relata o slot `number`); demais
   diagnósticos espelham `list.item`.
4. `terms.item(term, description)`: dois `Content` posicionais obrigatórios, casados por
   **ordem** (não por tipo — ambos são `Content`). Constrói `Content::term_item(term,
   description)` (constructor já existente, `entities/content.rs:2228`). Faltando
   qualquer um → `missing argument: term`/`missing argument: description` conforme o
   ausente; excesso → `unexpected argument`.
5. `native_terms` ganha uma segunda forma de chamada: se `args.items` não estiver vazio,
   cada positional deve ser `Value::Content(Content::TermItem(_))` (produzido por
   `terms.item(...)` ou por markup `/ t: d`); constrói `Content::terms(items)` a partir
   deles diretamente, **sem** consultar `args.named`. Se `args.items` estiver vazio,
   comportamento **inalterado**: a forma nomeada already documentada acima continua
   válida. As duas formas não se combinam no mesmo `call` — positional não-vazio e
   named não-vazio ao mesmo tempo é erro (`unexpected argument`, ancorado no primeiro
   named). A forma nomeada não é removida: é uma extensão cristalina aditiva sem
   equivalente vanilla (não gera crédito de paridade), preservada por não quebrar
   nenhum consumer existente.

### Gate ADR-0127

Três novas rotas qualificadas mais uma mudança de contrato aditiva em `terms()`
(segunda forma de chamada). Requer L0 causal (este), hashes ressealados, V5/V15/V26,
commit documental e **confirmação humana explícita posterior**.

Confirmação recebida em 2026-09-21, posterior ao commit causal `3ac60a578`.
Agente A (`01a82593d`) implementou `native_list_item`/`native_enum_item`/
`native_terms_item` e estendeu `native_terms` para a forma posicional sem
ver os testes. Agente B (`0258eab89`) escreveu 20 testes RED sem ver a
implementação, via `eval_expression` (nunca construção Rust direta dos
tipos ainda inexistentes). Suíte: GREEN, zero regressão em typst-core
(6129/6129 final, após correções de integração noutras famílias).

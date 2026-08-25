# Prompt L0 — `entities/elements/page_run` — `PageRunElem`

## P1140.20.2 — canvas lexical condicionado ao gate

Após confirmação, transportar bleed/fill/background/foreground, incluir no
snapshot LIFO e recursar layers presentes em map_*. Preservar omitido/none.
Running matter/supplement continuam excluídos.

## P1140.20.1 — extensão condicionada ao gate

Após confirmação, acrescentar paper/flipped/binding opcionais e margem com
modo lateral. A exclusão desses campos na secção P1140.19 é histórica e deixa
de valer apenas para estes três após o gate; P1140.20.2–.4 seguem excluídos.
Map/hash/eq preservam deltas e só body recursa. Snapshot/restauração inclui
binding e margem folded. Paper resolve eixos e não é introspectável. O
constructor interno segue incompleto e `page` fica para P1140.21.
Hash do Código: dfe29fc9

**Camada:** L1  
**Alvo:** `01_core/src/entities/elements/page_run.rs`  
**Origem:** P1140.18–P1140.19  
**ADRs:** ADR-0026, ADR-0107, ADR-0108, ADR-0109, ADR-0127  
**Estado:** especificado; implementação condicionada ao gate P1140.19

## Medição anterior à decisão

No vanilla ratificado `a51e02804`, `page[...]` é um constructor vigente. A
fonte `lab/typst-original/crates/typst-library/src/layout/page.rs:504-523`
produz uma sequência com fronteira inicial fraca, marker `flush`, body,
fronteira final boundary e mapa local de estilos.

Medições P1140.19 no nível da linguagem:

- run `100×120` entre conteúdo `200×200` produz páginas
  `200×200 → 100×120 → 200×200`;
- body vazio no meio conserva a página `100×120`;
- body vazio sozinho produz exatamente uma página;
- runs consecutivos `100×120` e `140×160` produzem exatamente essas duas
  páginas, sem página vazia intermediária;
- runs aninhados produzem
  `180×180 → 100×120 → 180×180 → 240×240`, demonstrando restauração LIFO;
- pagebreak explícito dentro de run `100×120` produz duas páginas do run e o
  conteúdo posterior volta a `200×200`;
- page configuration dentro de `block` ou `columns` falha com
  `page configuration is not allowed inside of containers`.

Proveniência: HEAD `45b547073d7686cdd5d3e3030c82de3e22ec395f`, working tree
não commitada, medição em `2026-08-24T13:43:00-03:00`; antes dos L0s deste
passo, `84 files changed, 775 insertions(+), 508 deletions(-)`.

## Decisão

Adotar **α — `Content::PageRun(Arc<PageRunElem>)`**. Um contentor único impede
pares start/end desequilibrados por construção e faz runs aninhados restaurarem
pela descendência normal do layout. Eventos `PageRunStart`/`End` e extensão
push/pop de `SetPage` ficam rejeitados: ambos permitem sequência malformada e
misturam escopo lexical com a progressão de set-rule.

Esta variante é infraestrutura interna preparatória. P1140.19 não expõe
`page`, `std.page` nem parsing do constructor.

## Estrutura

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct PageRunElem {
    pub width: Option<PageDimension>,
    pub height: Option<PageDimension>,
    pub margin: Option<PageMarginSpec>,
    pub numbering: Option<EcoString>,
    pub columns: Option<usize>,
    pub body: Content,
}
```

Os cinco campos são exatamente o subconjunto já transportado por
`Content::SetPage`. `None` significa preservar o valor ativo. `numbering:
Some("")` conserva a representação vigente de desligar numeração. Não
adicionar `paper`, `flipped`, `bleed`, `binding`, `fill`, supplement, headers,
footers, background ou foreground neste passo.

## Contrato de `Element`

- `plain_text()` delega ao body;
- `is_empty()` é sempre `false`, pois um run vazio conserva uma página;
- `map_content` e `map_text` recursam somente no body e preservam configuração;
- igualdade compara os cinco campos e body;
- não é locatável, não possui payload e não é selecionável por show rule;
- `get_field` não expõe propriedades antes do constructor público P1140.21.

## Constructor interno

Fornecer helper tipado que recebe os cinco campos e body. Não aceitar mapa de
propriedades, `dyn`, strings de dispatch ou defaults numéricos. O helper não
faz layout nem muta estado.

## Critérios

- variante fechada no enum `Content`;
- todos os matches são atualizados exaustivamente;
- nenhuma configuração vaza para irmãos;
- body vazio permanece semanticamente não vazio;
- runs aninhados são representáveis sem identificador ou estado global;
- o binding `page` continua ausente até P1140.21.
## P1140.24 — running matter

O page-run transporta os seis argumentos de running matter definidos em
`entities/page_running.md`, incluindo os três estados de header/footer, e
restaura integralmente a configuração exterior.
## P1140.25 — supplement

O page-run transporta o delta de `PageSupplement` e restaura integralmente a
configuração exterior. Ver `entities/page_supplement.md`.

## P1140.26 — constructor público condicionado ao gate ADR-0127

Medição: o `Construct` vanilla em
`lab/typst-original/crates/typst-library/src/layout/page.rs:504-523` isola o
body e aplica localmente as propriedades; não cria um elemento selecionável.
O `PageRunElem` cristalino é a representação fechada dessa morfologia.

Após confirmação do gate, `page(.., body)` e `std.page(.., body)` podem
construir `Content::PageRun(Arc<PageRunElem>)` preenchendo os 18 deltas
existentes. O body é obrigatório. Omissão continua `None`; nenhum default é
assado no constructor, pois defaults e herança pertencem ao consumer de
layout. `plain_text`, `is_empty`, igualdade e `map_*` mantêm o contrato atual.

## P1157 — delta de numbering tipado

### Medição antes da decisão

Dois `page(..)[body]` consecutivos preservaram callback e pattern lexicalmente,
produzindo `X1/2` e `II`.

### Decisão

`PageRunElem.numbering` passa a `Option<Option<Numbering>>`, com omissão,
desativação e instalação distintas. `map_content`/`map_text` clonam Numbering
sem executar Func. `to_set_page` e aplicação lexical transportam o mesmo delta;
defaults continuam no layout. Mudança pública bloqueada no gate P1157.

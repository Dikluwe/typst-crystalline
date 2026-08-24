# Diagnóstico P1140.20 — nucleação das propriedades de `page`

**Data:** 2026-08-24  
**Estado:** fechado; divisão vinculante escrita  
**Código L1–L4:** não alterado

## 1. Conclusão

As 15 propriedades restantes não formam um único contrato. A leitura da fonte
ratificada e dos consumers produz quatro núcleos:

| Núcleo | Propriedades | Consumer final |
|---|---|---|
| P1140.20.1 | `paper`, `flipped`, `binding`, `margin` | resolução da região de página |
| P1140.20.2 | `bleed`, `fill`, `background`, `foreground` | composição do canvas e export |
| P1140.20.3 | `numbering`, `number-align`, `header`, `header-ascent`, `footer`, `footer-descent` | composição marginal por página e contador |
| P1140.20.4 | `supplement` | introspecção e referência de página |

A hipótese de três unidades foi parcialmente refutada: `supplement` não deve
ser desenhado junto de header/footer. No vanilla ele é preservado em `Page`,
consultado pelo introspector e consumido por referências; não participa da
composição visual marginal.

## 2. Proveniência

- HEAD: `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- working tree não commitada;
- hora da medição estrutural: `2026-08-24T14:06:59-03:00`;
- estado nessa hora: `90 files changed, 1027 insertions(+), 511 deletions(-)`;
- vanilla ratificado: upstream/main `a51e02804`;
- fonte principal: `lab/typst-original/crates/typst-library/src/layout/page.rs`;
- consumer de layout: `lab/typst-original/crates/typst-layout/src/pages/run.rs`;
- export: `typst-pdf/src/convert.rs`, `typst-svg/src/lib.rs` e
  `typst-render/src/lib.rs` dentro do lab.

Hashes SHA-256 lidos antes da decisão: `page_run.md` `6cdb0ab3`,
`content.md` `2aeea16a`, `layout_types.md` `b40e60c5`, `eval.md` `321550eb`,
`layout.md` `9d5fa14f`, `stdlib/layout.md` `573b7c06`, `page_store.md`
`11ce08bb`, `introspect.md` `15da02f8`, `export/mod.md` `19251956`,
`export/render.md` `d6d9ee31` e `export/stream.md` `1960ca52`.

## 3. Matriz medida

| Propriedade | Entrada/default normativo | Efeito e owner | Consumer/fonte | Classe |
|---|---|---|---|:---:|
| `paper` | nome de `Paper`; default `Paper::A4`, nunca literal calibrado | shorthand externo que fornece width/height quando estes não são explícitos | parse `page.rs:54-103`; tabela `page.rs:818-960` | C |
| `flipped` | bool; default `false` | troca os eixos depois de resolver width/height | `page.rs:105-125`; `pages/run.rs:105-110` | C |
| `margin` | `auto`, length relativo ou dict; fold; lados omitidos herdam | região do body; default estrutural `(2.5/21) × min(width,height)` | `page.rs:127-168,598-735`; `pages/run.rs:112-130` | B |
| `binding` | `auto`, `left`, `right`; auto deriva de `text.dir` | resolve inside/outside por paridade física | `page.rs:221-231,760-801`; `pages/run.rs:149-155` e finalização de página | C |
| `bleed` | length relativo ou dict; omissão por lado = zero | aumenta canvas sem aumentar frame trimado; lados lógicos seguem binding | `page.rs:170-219`; `pages/run.rs:132-138,228-243`; PDF `convert.rs:108-123` | C |
| `fill` | `auto`, `none`, paint | fill do canvas completo; auto é transparente em PDF e branco em raster/SVG | `page.rs:257-278`; `document.rs:88-121`; PDF `convert.rs:159-160`; SVG `lib.rs:298-302`; render `lib.rs:34` | C |
| `background` | content ou `none` por omissão | frame atrás do body, área inclui bleed | `page.rs:452-474`; `pages/run.rs:139-140,223-238` | C |
| `foreground` | content ou `none` por omissão | frame acima do body, área inclui bleed | `page.rs:476-492`; `pages/run.rs:139-140,228-238` | C |
| `numbering` | `none`, pattern ou função; função recebe 1 ou 2 números conforme contexto | usa contador lógico atual/final; marginal automático | `page.rs:280-311`; `pages/run.rs:157-186` | B |
| `number-align` | alinhamento horizontal + top/bottom; default center+bottom; horizon proibido | escolhe header/footer automático e alinha número | `page.rs:326-344`; `pages/run.rs:171-186` | C |
| `header` | `auto`, `none`, content | repete no topo; explícito suprime numbering top | `page.rs:346-367`; `pages/run.rs:180-186,204-237` | C |
| `header-ascent` | length relativo; default normativo `30%` | ratio relativo à margem top; reduz região do header | `page.rs:369-373`; `pages/run.rs:141,226` | C |
| `footer` | `auto`, `none`, content | repete no fundo; explícito suprime numbering bottom | `page.rs:375-404`; `pages/run.rs:180-186,204-237` | C |
| `footer-descent` | length relativo; default normativo `30%` | ratio relativo à margem bottom; reduz região do footer | `page.rs:406-450`; `pages/run.rs:142-143,227` | C |
| `supplement` | `auto`, `none`, content; auto = nome local de page | metadado da página lógica para referências | `page.rs:313-324`; `pages/run.rs:144-148,233-234`; `location.rs:263-289`; `reference.rs:249-257,334-355` | C |

Classes A/B/C seguem P1140.18: B é suporte parcial e C é ausência no
cristalino. Nenhum número observado em render foi promovido a regra. A4 vem da
tabela `Paper`; margem automática vem da fórmula; 30% vem do atributo default
da fonte.

## 4. Tipos, rejeições e precedência

- `paper` pode ser posicional ou `paper:`. Seu parse fornece dimensões apenas
  quando `width`/`height` correspondentes não foram fornecidos
  (`page.rs:81-100`); portanto overrides explícitos vencem por eixo.
- `flipped` troca o tamanho já resolvido (`pages/run.rs:105-110`), logo não
  altera a tabela nem a precedência de paper.
- `Margin::from_value` aceita valor escalar ou dict. `rest → x/y → lado` é a
  ordem operacional; `inside/outside` com `left/right` é erro explícito e
  chaves restantes são rejeitadas (`page.rs:667-735`). Fold preserva lados
  omitidos (`page.rs:613-620`).
- `binding: auto` usa LTR→left e qualquer direção RTL→right
  (`pages/run.rs:149-155`). Left-bound troca lados nas páginas pares;
  right-bound, nas ímpares (`page.rs:760-779`).
- `bleed` reutiliza a morfologia de lados, mas seus valores não aceitam
  `auto`; lado omitido resolve para zero (`pages/run.rs:132-138`).
- `fill` tem distinção ternária real: `auto` não é `none`. A resolução depende
  do target e pertence ao consumer de export (`document.rs:103-121`).
- `background`/`foreground` usam `Option<Content>`, portanto `auto` não faz
  parte do contrato. O frame é resolvido contra
  `inner + margin + bleed` (`pages/run.rs:228-238`).
- `number-align` não aceita horizon por contrato documentado
  (`page.rs:326-344`). Header/footer explícito no lado escolhido vence o
  marginal automático (`pages/run.rs:180-186`).
- numbering pattern com duas peças pede total; função sempre usa o caminho de
  ambos para apresentação visível (`pages/run.rs:157-169`). Em referências a
  função recebe um argumento, conforme contrato `page.rs:283-289`.
- `supplement: auto` materializa o nome local de `page`; `none` materializa
  conteúdo vazio; content é preservado (`pages/run.rs:144-148`).

Os diagnósticos textuais exatos devem ser fixados por RED em cada subpasso;
esta fase identifica a classe de rejeição, não congela mensagens ainda não
expostas pelo cristalino.

## 5. Herança, restauração e morfologia

Todas as propriedades são page-wide styles no vanilla
(`pages/run.rs:101-104`). P1140.19 já demonstrou que o constructor lexical
restaura configuração em LIFO e conserva body vazio. Cada novo campo deve
participar do mesmo snapshot; nenhum poderá ser aplicado fora de
`PageRunElem`/`SetPage` e depois “desfeito” por valor hardcoded.

`paper` é shorthand e não pode ser recuperado em context (`page.rs:55-58`):
a morfologia introspectável é width/height resolvido. Background, foreground,
header e footer são conteúdo real, mas header/footer são marcados como
artefatos na árvore de acessibilidade (`pages/run.rs:221-223`). Fill e bleed
são metadados físicos de `Page`; supplement é metadado lógico. Essa separação
é a razão da atomização final.

## 6. Mapa de dependências e owners

```text
eval/stdlib
  ├─ paper/flipped/binding/margin ─→ PageConfig ─→ layout/page geometry
  ├─ bleed/fill/layers ───────────→ PageConfig ─→ Page + canvas ─→ exporters
  ├─ numbering/running matter ────→ PageConfig ─→ counter/fixpoint + marginals
  └─ supplement ──────────────────→ Page ───────→ introspector ─→ ref(page)

PageRunElem snapshot/restoration receives every accepted field.
SetPage transports the same property types without a dynamic property map.
```

Owners candidatos, a validar no L0 de cada unidade: tipos fechados de papel,
lados e propriedades de página em módulos próprios de `entities`; parsing na
unidade `stdlib/layout`; aplicação por free functions em `compiler/layout`;
canvas final em `Page`; export em L3; supplement em `PageStore`/introspecção.
Não há razão medida para `dyn`, PropMap ou import reverso.

## 7. Decisão de divisão

### P1140.20.1 — geometria lógica

Confirmado. `paper`, `flipped`, `binding` e a conclusão de `margin` são
resolvidos antes da região do body e compartilham paridade/direção.

### P1140.20.2 — canvas físico e camadas

Confirmado. `bleed` permanece aqui: embora influencie `full_size`, não altera a
região trimada do body e só fica completo quando `Page` e exporters carregam
caixas/fill. Separá-lo em geometria aceitaria um argumento sem consumer final.

### P1140.20.3 — running matter

Mantém numbering, number-align, header/footer e offsets. A apresentação de
numbering é construída como marginal e a precedência é indivisível. O passo
pode internamente escrever REDs por subconjunto, mas não expõe estados
intermediários silenciosos.

### P1140.20.4 — supplement e referências

Novo passo. A fonte refuta o agrupamento anterior: o dado atravessa `Page`,
introspector e `ref(form: "page")`, sem depender do desenho de marginais.

## 8. Gates e fila

Cada um dos quatro passos altera contrato público e/ou comportamento padrão e
tem gate ADR-0127 independente: atualizar L0, resselo, parar, receber
confirmação, então RED→GREEN. O L0 inicial de cada constructor deve declarar a
incompletude até os demais passos. P1140.21 só expõe `page` e `std.page` quando
os 15 argumentos estiverem implementados ou explicitamente rejeitados.

## 9. Validação

P1140.20 não alterou código nem L0. A validação aplicável é documental:
`git diff --check`, `crystalline-lint .` e conferência de que os quatro passos
existem.

Resultado em `2026-08-24T14:09:08-03:00`:

- `git diff --check`: exit 0;
- `crystalline-lint .`: exit 0, zero violações bloqueantes; avisos e infos
  preexistentes permanecem fora do escopo deste passo documental;
- stat dos ficheiros já rastreados: `90 files changed, 1027 insertions(+),
  511 deletions(-)`;
- documentos ainda não rastreados desta execução: este diagnóstico,
  `typst-passo-1140.20.md` e os quatro subpassos `.1`–`.4`;
- nenhum build/teste foi repetido porque nenhum código, manifest ou L0 mudou.

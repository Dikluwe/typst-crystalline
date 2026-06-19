# Passo 379 — arms restantes do `layout_content`: tamanho de fatia + core/infra (recon read-only)

> **Read-only.** Mede o que resta do `layout_content` (929 linhas, pós-P378) para o dono decidir,
> **em valores**: (1) o tamanho de fatia dos não-math (acoplamento entre arms + famílias + trade);
> (2) a classificação do core/infra (atomizável-como-elemento vs máquina-do-layouter). **Não move
> código, não decide, não recomenda.** Árvore limpa; lint 0/0 inalterado. Caveat de stack:
> `RUST_MIN_STACK=33554432`. HEAD pós-P378 (`26c822677`).
>
> **Caveat de medição (ADR-0108):** o mapa "estado que lê" foi extraído por grep de `self.<tok>` no
> corpo de cada arm — **inclui tokens citados em comentários** (ex.: `CounterUpdate` "lê" `counter`
> só no comentário; o corpo é vazio). As famílias são robustas a isto: o estado **partilhado real**
> (`regions`/`flush_line`/`font_size_pt`/`page_config`/`style`/`layout_content`) é acesso de código.
> Marcado [medido]/[inferido].

---

## Parte 1 — não-math diretos: acoplamento + famílias + trade

### A descoberta que decide o tamanho de fatia [medido]
**O acoplamento entre arms é BAIXO para efeito de fatiar.** Quase todos os arms restantes leem o
**mesmo estado de fluxo de texto** do `Layouter` — `regions`, `flush_line()`, `font_size_pt`,
`page_config`, `style`, `layout_content()` — que **todo arquivo descendente acede de forma
idêntica** (provado 15× em P376-378). Logo, **mover arms em fatias separadas NÃO duplica acesso nem
risco**: o mecanismo de acesso (módulo descendente) é o mesmo em qualquer arquivo. Não há
"penalidade de separação".

Os **únicos** clusters com estado dedicado/único (que pedem para andar juntos):
- **Grid + Table** → ambos chamam `self.layout_grid()` (método dedicado partilhado) [medido
  `:928`,`:957`]. Par natural.
- **Footnote** → `footnote_counter` + `pending_footnote_bodies` (estado único) [medido `:1020`].
- **SmartQuote** → `smartquote_double_open`/`smartquote_single_open` (estado único) [medido `:1401`].
- **Pagebreak/Colbreak** → `new_page()` (partilhado, mas `new_page` é usado por muitos) [medido].

**Conclusão (sem decidir):** como o acoplamento é baixo, **fatia pequena é barata** — o custo de
fatiar é só **processo** (mais Travas), não duplicação de código nem risco.

### Inventário por família (linhas = span com comentários, **limite superior**; arms ainda inline)
| Família | Arms (`@linha`) | Σ linhas | Estado partilhado / dedicado |
|---|---|---|---|
| **Listas** | EnumItem `@834` (18), ListItem `@820` (14), Terms `@1165` (7), TermItem `@1172` (25) | ~64 | fluxo-texto + recursão; TermItem lê `chain` |
| **Tabelas/Grid** | Grid `@928` (9), Table `@957` (18), GridCell/Header/Footer, TableCell/Header/Footer | ~73 | **`layout_grid`** (Grid/Table); restantes só recursam |
| **Breaks** | Pagebreak `@1322` (25), Colbreak `@1347` (34) | ~59 | `new_page`, `regions`, `flush_line`; Pagebreak lê `pages` |
| **Spacing** | VSpace `@1218` (23), HSpace `@1214` (4), Repeat `@1291` (25) | ~52 | `regions`/`font_size_pt`; Repeat só recursa |
| **Refs/citações** | Cite `@1028` (35), Ref `@905` (8), Link `@852` (6), Bibliography `@1001` (19), Footnote `@1020` (8) | ~76 | Cite lê `introspector`; Footnote estado único; resto recursa |
| **Avulsos** | Quote `@1426` (46), SmartQuote `@1401` (25), Raw `@805` (15), Hide `@1199` (15), Divider `@1146` (19) | ~120 | fluxo-texto; Raw/SmartQuote leem `layout_word`; SmartQuote estado único |

**Total não-math restante ≈ 27 arms / ~444 linhas (span)** [medido]. O inline real é menor (os spans
incluem comentários e arms já-delegados intercalados).

### O trade tamanho-vs-segurança, em valores (sem recomendar)
- **Família-pequena (6 fatias):** as 6 famílias acima — ~50-120 linhas / 2-8 arms cada → **~6 Travas**.
  Cada fatia trivial de verificar; mais paragens.
- **Lote-médio (3 fatias):** ~2 famílias por fatia (ex.: Listas+Tabelas; Breaks+Spacing;
  Refs+Avulsos) → **~3 Travas**, ~150 linhas/fatia.
- **Lote-grande (1-2 fatias):** os ~27 arms de uma vez (~444 linhas) ou em 2 → **1-2 Travas**; mais
  superfície por revisão, mas a forma é idêntica e provada 15×.

Como o acoplamento é baixo, **qualquer corte é válido** — a escolha é só quantas Travas vs quanta
superfície por lote. [a leitura honesta, sem decidir]

---

## Parte 2 — core/infra: atomizável-como-elemento vs máquina-do-layouter

| Item | `@linha` (linhas) | Veredito | Porquê (medido) |
|---|---|---|---|
| **Text** | `@635` (82) | **[atomizável-como-elemento]** | Renderizador **folha**: decodifica o render do `#set text` da chain (canais `custom`), faz merge top-wins com `self.style`, e itera `text.split_whitespace()` chamando `layout_word`. **Não re-entra `layout_content`** (não orquestra filhos). Separável como `text.rs::layout(lo, &str)`. Nota: grande, chain-pesado, caminho quente. |
| **Sequence** | `@724` (51) | **[máquina-do-layouter]** | Itera os filhos (`layout_content(part)`), gere a **chain de espaçamento de blocos** (`prev_block_below_pending`/`block_chain_active`), faz **sticky-lookahead** (peek no próximo) e decide **page-break** (`new_page`/`page_bottom_limit`). É o **motor de recursão+paginação**; mover cortaria a orquestração. |
| **Styled** | `@1110` (14) | **[máquina-do-layouter]** | Push/pop de estilo na `chain` à volta do `layout_content(body)` (mecanismo `#set`/`*bold*`). Orquestra o filho com chain modificada. (Strong/Emph `@1124` **replicam** este motor.) |
| **Dynamic** | `@561` (31) | **[máquina-do-layouter]** | Fronteira de extensão **E1**: resolve settable da chain, faz **downcast pelo trait `DynElement`** (`dyn_get_field`/`dyn_plain_text`) e re-entra `layout_content`. É o despacho dinâmico, não um elemento de domínio. |
| **SetPage** | `@1063` (26) | **[máquina-do-layouter]** | **Diretiva de reconfiguração**: muta `page_config` + re-sincroniza `regions` (width/height/cursor). É **struct-variant** (`SetPage { width, height, margin }`, **sem `XElem`**) → a forma B (`elem::layout(self, e)`) nem se aplica. Reconfigura o motor. |
| **Metadata / State / StateUpdate** | `@592`/`@597`/`@598` (5/1/7) | **[não-elemento / no-op]** | Zero-size em layout (`=> {}` com comentário). Nada a atomizar. |
| **CounterUpdate** | `@779` (8) | **[não-elemento / no-op]** | No-op (`self.counter` eliminado P190I; corpo vazio — o token `counter` é só comentário). |
| **CounterDisplay / CounterDisplayCallback / StateDisplay** | `@787`/`@624`/`@605` (18/11/19) | **[a-decidir]** | Adaptadores finos do `Introspector`: leem valor pré-computado (`formatted_counter_at`/pre-rendered) e re-entram `layout_content` com um `Content::text`. Plumbing de introspeção, não render de domínio — atomizáveis como grupo `introspect_display.rs`, mas é fronteira (o dono decide). |

---

## A leitura honesta (sem decidir)
- **Tamanho de fatia:** os números dizem que **fatiar é barato** (acoplamento baixo; acesso uniforme
  por descendência). O custo é só processo (Travas). Família-pequena = 6 Travas / fatias triviais;
  lote-grande = 1-2 Travas / mais superfície. Os dois extremos são válidos — **escolha do dono**.
- **Core/infra:** **só o `Text` é atomização legítima** (folha de render, não orquestra). **Sequence,
  Styled, Dynamic, SetPage são máquina do layouter** — ficam onde estão **por não serem elementos de
  domínio** (orquestram/reconfiguram/despacham), não por preguiça. Os no-ops (Metadata/State/Counter
  Update) não têm o que atomizar. Os displays de counter/state são **[a-decidir]** (plumbing de
  introspeção na fronteira).
- **Fecho do monólito:** atomizar os ~27 não-math + o `Text` deixaria o `layout_content` essencialmente
  com a **máquina** (Sequence/Styled/Dynamic/SetPage + no-ops + os já-magros) + a fatia **math**
  (17 variantes, `@858`/`@868`, path próprio). "Fechar" ≠ 0 arms — é **só máquina + math**. [inferido]

---

## Gates
- **read-only:** nenhum código de produto, nenhum L0; só leitura + grep; suíte não re-rodada. ✔
- **saída:** este ficheiro (Parte 1: acoplamento+famílias+trade; Parte 2: core/infra classificado; a
  leitura honesta). Marcado [medido]/[inferido]. ✔
- **INTACTOS:** nada tocado. Lint 0/0 inalterado. Árvore limpa. ✔

**Termina aqui — não decide nem move.** A escolha (tamanho de fatia + destino do core/infra,
incl. os [a-decidir]) é do dono, com estes valores.

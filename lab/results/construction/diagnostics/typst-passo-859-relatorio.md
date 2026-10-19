# Relatório — typst-passo-859: o que a fase `realize` do vanilla garante de verdade

**Data:** 2026-07-23  
**Executor:** Kimi Code (agente principal; prompt lido de `00_nucleo/materialization/typst-passo-859.md`).  
**Proveniência das medições:** análise de código estático em `lab/typst-original/crates/typst-realize/` e no código cristalino L1/L3; nenhum código foi alterado neste passo.

---

## 1. Objetivo e delimitação

Este passo é **pesquisa pura**, sem código e sem proposta de arquitetura. A decisão sobre `measure()` (P858) permanece fechada via Opção 1. A pergunta aqui é mais ampla: a fase `realize` do vanilla existe por razões tipográficas que o cristalino, por não ter uma fase equivalente, pode não estar a garantir? Se sim, que mecanismos são esses e onde é que o cristalino os resolve (ou não resolve)?

---

## 2. O que a fase `realize` do vanilla faz

A implementação está concentrada em `lab/typst-original/crates/typst-realize/src/lib.rs` (1 534 linhas) e `spaces.rs` (122 linhas). A função `realize` recebe `content`, `Engine`, `locator`, `arenas` e `StyleChain`, e devolve uma lista plana de `Pair` (conteúdo + estilos). Os mecanismos principais são:

### 2.1. Aplicação de show rules e preparação (`visit_show_rules` + `prepare`)

- Determina, para cada elemento, se existe uma show rule aplicável (user-defined ou built-in).
- Aplica pre-synthesis em elementos que implementam `Synthesize` para que `show figure.where(kind: table)` funcione.
- Prepara o elemento uma única vez: gera `location`, aplica `ShowSet`, corre `synthesize`, materializa styles, cria tags de início/fim para locatables.
- Incrementa a profundidade da route e verifica `MAX_SHOW_RULE_DEPTH`.

### 2.2. Regras específicas do tipo de realização (`visit_kind_rules`)

- **Document/Fragment/Par:** mathy content fora de equação é envolvido automaticamente em `EquationElem`; `SymbolElem` converte-se em `TextElem`.
- **Math:** equações aninhadas são processadas transparentemente; regex show rules aplicam-se a `SymbolElem`/`TextElem` individualmente.

### 2.3. Grouping rules (`visit_grouping_rules` + finishers)

O vanilla tem 6 grouping rules ativas em realização normal (`FLOW_RULES`), por ordem de prioridade:

| Regra | Prioridade | O que agrupa | Resultado |
|---|---|---|---|
| `TEXTUAL` | 3 | Texto, space, linebreak, smart quote adjacentes | Aplica regex show rules; se não houver match, delega para `PAR` |
| `PAR` | 1 | Inline elements (text, h, box, inline, linebreak, smart quote, html neutro) + spaces | `ParElem` |
| `CITES` | 2 | `CiteElem` consecutivos com spaces entre eles | `CiteGroup` |
| `LIST` | 2 | `ListItem` consecutivos com `SpaceElem`/`ParbreakElem` entre eles | `ListElem` |
| `ENUM` | 2 | `EnumItem` consecutivos | `EnumElem` |
| `TERMS` | 2 | `TermItem` consecutivos | `TermsElem` |

As regras podem ser interrompidas por estilos ou por elementos de prioridade superior. Neutral elements (certos HTML) permitem misturar flows inline/blocky.

### 2.4. Colapso de espaços (`collapse_spaces`)

- Descarta `SpaceElem` nos extremos de um grupo ou junto a elementos "destructive" (`LinebreakElem`, `HElem` fracionário/weak, HTML whitespace-collapsing, etc.).
- Colapsa espaços adjacentes num só, mantendo os estilos do primeiro.
- Corre tanto nos grupos `PAR`/`TEXTUAL` como em realizações `Par`/`Math` top-level.

### 2.5. Filtros de realização (`visit_filter_rules`)

- Fora de parágrafo/math, `SpaceElem` e `ParbreakElem` não são guardados no sink.
- `VElem` com `attach: true` é descartado se não seguir imediatamente um parágrafo (attach spacing collapse).

### 2.6. Page styles → pagebreak implícito (`visit_styled`)

- `#set page(...)` num contexto paged e top-level gera um weak pagebreak antes e um boundary pagebreak depois, permitindo mudanças de configuração de página ao longo do documento.

### 2.7. Reconstrução de conteúdo (`repack`)

- Após o grouping, os elementos agrupados são convertidos de volta em `Content` (sequence + trunk style chain).

---

## 3. Cruzamento com achados já registrados

Foram revistos os relatórios de P810 a P858. Os achados com ligação potencial à fase `realize` são:

| Achado | Passo | Descrição | Ligação com `realize` |
|---|---|---|---|
| #41 (F2) | P843 | Parser cristalino funde `[hello world]` num só `Text`; vanilla mantém `Text+Space+Text` (que o realize depois agrupa em `ParElem`). | Morfologia diferente; o cristalino não passa pelo mesmo content tree intermédio. |
| #41 (F2) | P843 | Render de `\n` embutido em texto está quebrado (`#"a,\nb,"` → `a,,`). | Bug de layout/texto, não directamente de falta de realize. |
| #55 | P831/P845 | Texto com `\n` truncado na primeira newline no shaping. | Resolvido em P845 (`bidi_runs` itera todos os parágrafos); não é falta de realize. |
| — | P845 | Fluxo vertical em L1 ainda trata texto com `\n` como uma palavra. | Limitação de layout de texto em L1, não de realize. |
| #38 (L7) | P842 | `#h(1fr)` rejeitado. | Resolvido com `Spacing::Fractional`; não é realize. |
| #54 (A8) | P844 | `#context` entre headings desalinhava numeração. | Resolvido com re-introspecção pós-expansão de `ContextBlock`; o realize do vanilla evitaria isto porque context é resolvido durante realize, antes do layout. |
| — | P854 | Análise: vanilla `typst-realize` aplica show rules e agrupa parágrafos/listas/espaçamento numa fase separada. | Diagnóstico sistémico, não um achado funcional novo. |
| #34 (L3) | P842/P849 | `measure()` devolvia métricas heurísticas. | Resolvido em P858; o realize do vanilla resolve `measure()` durante layout com métricas reais. |
| — | P856 | `#ref(<label>)` em texto/item de lista/raw dava erro errado. | Resolvido com walk de introspecção; não é realize. |

Não foram encontrados achados específicos para: agrupamento de itens de lista, agrupamento de citações, formação de `ParElem`, ou envolvimento automático de mathy elements em equação.

---

## 4. Classificação dos mecanismos do `realize`

### Grupo 1 — Já coberto pelo cristalino (em outro ponto/de outra forma)

| Mecanismo | Onde no cristalino | Avaliação |
|---|---|---|
| Aplicação de show rules built-in e user-defined | `01_core/src/engine/eval/rules.rs` | Coberto. Show rules são aplicadas durante eval; a ordem e profundidade são geridas com `active_guards` e `StyleChain`. |
| Page styles → implicit pagebreak | `01_core/src/engine/layout/set_page.rs:17-67` | Coberto. Mudança de configuração de página força `new_page()` se a página actual não estiver vazia. |
| Locatable tags (início/fim) | `01_core/src/engine/introspect.rs:1042` (`walk`) | Coberto. O introspetor emite `Tag::Start`/`Tag::End` para elementos locatable durante a fase de introspecção. |
| Show-set via `Styled` | `01_core/src/engine/eval/rules.rs:643-662` | Coberto. Regras de transformação de estilo envolvem o elemento em `Content::Styled`. |
| Space collapsing parcial | `01_core/src/engine/layout/mod.rs:789-800`, `01_core/src/engine/layout/sequence.rs:23-57` | Parcialmente coberto. Espaços iniciais de linha são suprimidos; margens block-level têm colapso; `weak` space está scope-out. |
| Math nested em equações | não aplicável diretamente | O cristalino trata math como `Content::MathSequence`/`Equation` sem a mesma re-entrada do vanilla, mas não há achado de que isto falhe. |

### Grupo 2 — Não coberto e sem sintoma observado até agora

| Mecanismo | O que falta no cristalino | Porque ainda não é sintoma |
|---|---|---|
| Agrupamento em `ParElem` | Não existe `Content::Par`. Parágrafos são sequências planas de `Text`/`Space`/`Parbreak` que o layout de texto processa directamente. | O layout cristalino já consegue renderizar texto corrido correctamente em muitos casos testados; a ausência de `ParElem` só se tornaria sintoma em cenários que dependam da semântica de parágrafo (ex.: estilos aplicados ao parágrafo como um todo, medidas de parágrafo, alinhamento por parágrafo). |
| Agrupamento de listas (`ListElem`/`EnumElem`/`TermsElem`) | Itens de lista vivem isolados ou em `Sequence`; não há container que os agrupe. | A morfologia dos itens é preservada e o layout renderiza-os consecutivamente; a falta de container só se manifestaria em comportamentos como spacing entre itens de lista controlado ao nível da lista, ou enumeração contínua entre blocos separados. |
| Agrupamento de citações (`CiteGroup`) | Não existe `CiteGroup`. Cada `CiteElem` é processado individualmente. | A lógica de `ibid.`/`op. cit.` existe no `Layouter` (`01_core/src/engine/layout/cite.rs:25-67`); a falta de grupo só afectaria numeração/ordenação de citações quando o vanilla as trata como uma unidade. |
| Mathy elements → `EquationElem` automático | Símbolos mathy fora de `$...$` não são envoltos em equação. | O cristalino degrada para texto normal (`layout_math_fallback`); não há achado de documento que dependa deste comportamento. |
| `Synthesize` como trait separado | Não existe `Synthesize`. Campos derivados são computados inline nas funções de layout/eval. | Funcionalmente coberto caso a caso, mas sem o mesmo ponto único de pre-synthesis do vanilla. |
| Regex show rules cross-node | Regex aplica-se a nós `Text` individuais; não atravessa `Space` ou múltiplos nós de texto. | Scope-out documentado; seria sintoma apenas para padrões que casam texto separado por spaces ou por mudanças de estilo. |

### Grupo 3 — Não coberto e já é sintoma de um achado conhecido

| Mecanismo | Achado | Observação |
|---|---|---|
| Morfologia do content tree para texto corrido | #41 (F2), P843 | O vanilla mantém `Text+Space+Text` até ao realize; o cristalino funde no parser. Isto é um sintoma conhecido de divergência de morfologia, embora a causa não seja "falta de realize" mas sim uma decisão diferente de parsing. |
| `measure()` com métricas reais | #34 (L3), P842/P849/P858 | O vanilla resolve `measure()` durante realize/layout; o cristalino resolveu via injeção de métricas no `Engine` durante `expand_context_blocks`. Está fechado, mas é um exemplo de comportamento que no vanilla depende da fase de realização. |

---

## 5. Observações sobre a arquitetura cristalina

O cristalino não tem uma fase de realização porque várias das suas responsabilidades estão distribuídas:

- **Eval** (`01_core/src/engine/eval/`) já aplica show rules, resolve `Styled`, e constrói a content tree.
- **Introspecção** (`01_core/src/engine/introspect.rs`) gera tags e resolve locatables/counters/states.
- **Expansão de contexto** (`03_infra/src/pipeline.rs::expand_context_blocks`) resolve `ContextBlock` e `measure()` com métricas reais (P858).
- **Layout** (`01_core/src/engine/layout/`) formata texto, listas, espaços, quebras de página, etc., directamente sobre a content tree.

Esta separação funciona para os casos testados até agora, mas concentra emocionalidades tipográficas no layout que no vanilla são resolvidas antes, numa fase comum.

---

## 6. Conclusão

A fase `realize` do vanilla garante mecanismos de composição que o cristalino não possui como fase explícita. A maioria desses mecanismos **ainda não produziu sintomas observados** nos achados de P810 a P858, porque:

1. O layout cristalino processa sequências planas de forma equivalente em casos simples.
2. A morfologia diferente do content tree (especialmente a fusão de texto no parser) mascara ou substitui algumas das necessidades do realize.
3. A triagem sistemática testou módulo a módulo; comportamentos de composição entre módulos (ex.: "duas listas consecutivas deveriam fundir-se numa") não foram o foco.

Os únicos sintomas já reconhecidos que têm ligação com realize são a divergência de morfologia de texto (#41 F2, P843) e o caso de `measure()` (#34, já fechado em P858). Não foram identificados bugs ativos que justifiquem, por si só, a introdução de uma fase de realização no cristalino.

**Nenhuma proposta de arquitetura é feita neste passo.** Se o dono quiser aprofundar, os buracos mais plausíveis de se tornarem sintomas são: formação de `ParElem`, agrupamento de listas/citações, e envolvimento automático de mathy elements em equação.

---

## 7. Referências

- `lab/typst-original/crates/typst-realize/src/lib.rs`
- `lab/typst-original/crates/typst-realize/src/spaces.rs`
- `00_nucleo/diagnosticos/typst-passo-843-relatorio.md` (#41 F2)
- `00_nucleo/diagnosticos/typst-passo-844-relatorio.md` (#54 A8)
- `00_nucleo/diagnosticos/typst-passo-849-relatorio.md` (#34 L3)
- `00_nucleo/diagnosticos/typst-passo-854-relatorio.md`
- `00_nucleo/diagnosticos/typst-passo-857-relatorio.md`
- `00_nucleo/diagnosticos/typst-passo-858-relatorio.md`
- `01_core/src/engine/eval/rules.rs`
- `01_core/src/engine/layout/mod.rs`
- `01_core/src/engine/layout/set_page.rs`
- `01_core/src/engine/introspect.rs`
- `01_core/src/engine/stdlib/structural.rs`
- `01_core/src/engine/layout/cite.rs`

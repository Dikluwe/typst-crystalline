# Roteiro para completar a refatoração — typst-cristalino

**Versão:** P463  
**Data:** 2026-06-25  
**Base:** Inventário `typst-cobertura-vanilla-vs-cristalino.md` (snapshot P447/P453/P455),  
scope-outs acumulados P388–P463, DEBT.md atual (zero débitos ativos).

---

## Estado das trilhas — resumo executivo

| Trilha | Descrição | Estado | Próximo passo |
|--------|-----------|--------|---------------|
| **1** | Numeração (heading, figure, equation, TOC, table) | **COMPLETA** | — |
| **2** | Referências cruzadas (label, ref, /GoTo) | **COMPLETA** | — |
| **3** | Selectors completos em show rules | Pronta | Sonda `Selector::Where` |
| **4** | Visuais (gradientes, espaços de cor, tiling) | Pronta | `Value::Gradient` tipo real |
| **5** | Shaping / rustybuzz | Bloqueada | Sonda de viabilidade rustybuzz |
| **6** | Bibliografia Fase 2 (estilos numéricos, ibid, LoF/LoT) | Desbloqueada | Estilos numéricos |
| **7** | Layout multi-região (columns, measure, table real) | Pronta | `columns`/`colbreak` |
| **8** | Refinos de stdlib e tipos (repr, array/dict/str, Value::Relative, etc.) | Pronta | `repr()` completo |
| **9** | DEBT-2 (closures) | **FECHADA** | Premissa refutada em P458 |

---

## Regras do roteiro (gates a aplicar em cada passo)

1. **Sonda A.0 real antes da spec (ADR-0114).** A sonda é `grep`/diagnóstico
   executado, com `file:line` + commit anexados. Não é preenchida de memória.
   Casos que violaram: P388, P409, P413, P416, P421, P447, P451, P452, P459.
2. **Verificar fronteiras/ADR vigentes antes de propor estrutura (ADR-0117
   Cláusula 4).** Antes de propor campo novo num elemento ou módulo novo,
   confirmar a decisão que fixou a forma atual. Casos: P427 (L0 `stream.md`),
   P454 (P365 chain vs campo), P459 (contador local vs oráculo).
3. **Declarar divergência de paridade de saída (ADR-0107).** Se o texto
   renderizado diverge do vanilla por default (ex.: prefixo "Figura" vs
   "Figure"), registar no inventário de cobertura, não só como scope-out.
4. **Confirmar o gate da ADR-0117 no linter.** O gate "spec propõe ficheiro que
   já existe" só fecha o ciclo se estiver implementado no `crystalline-lint`.
   Se ainda é proposta, a disciplina continua manual.

---

## Trilha 1 — Numeração (COMPLETA)

Fechada em P451 (heading), P454 (figure), P456 (equation), P457 (TOC), P459
(table), P461 (correção de coerência: table_counter -> CounterRegistry).

Todos os contadores usam `CounterRegistry`/`Introspector` com chaves semânticas
(`"heading"`, `"figure"`, `"equation"`, `"table"`), padrão na `StyleChain`, e
`format_counter` para formatação. Coerente arquiteturalmente.

---

## Trilha 2 — Referências cruzadas e navegação interna (COMPLETA)

Fechada em P460 (label + /Dests), P462 (ref + resolução de número), P463
(/GoTo links internos no PDF).

- `label(name, body)` — wrapper transparente, regista posição + página.
- `ref(name)` / `@name` — resolve para número do elemento via `Introspector`.
- PDF exporta `/Names /Dests` (P460) + `/Annots` com `/S /GoTo` (P463).
- `LinkTarget` enum unifica URLs externas (`/URI`) e destinos internos (`/GoTo`).

---

## Trilha 3 — Selectors completos em show rules

Fecha o `#show ...where(...)` e o split do trecho casado do regex.

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| Sonda: `Selector::Where` materializado no consumer (se ainda parcial) | — | S | **Próximo** |
| `#show elem.where(field: value): ...` | `Selector::Where` | S | Depende do acima |
| `#show regex(...)` — split do trecho casado (P393 scope-out) | — | M | Pronto |

**Nota:** O inventário marca `.where(field:)` como `ausente` (precisa
`Selector::Where`) na linha 111, mas P417/P423 mexeram em `Selector`. A primeira
ação desta trilha é uma **sonda** que resolva essa contradição entre o inventário
e o código. Se `Selector::Where` já existe parcialmente, esta trilha fecha em
S-M; se não existe, é M.

---

## Trilha 4 — Tipos visuais: gradientes, espaços de cor, tiling

Cluster `Visualize`: 5 `ausente` + tiling com fallback. Abre pelo tipo (gate
ADR-0017), depois consumer, depois render PDF.

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| `Value::Gradient` tipo real (hoje placeholder em `TilingBody`) | gate ADR-0017 | M | Pronto |
| `gradient.linear/radial/conic` consumers | `Value::Gradient` | M | Depende do tipo |
| Render PDF de gradiente (`/Sh` shading operators) | consumers | L | Depende dos consumers |
| Espaços de cor: `cmyk`, `oklab`, `oklch`, `linear_rgb`, `hsl`, `hsv` | `Value::Color` estende | M | Pronto |
| Render PDF de tiling/pattern fill (P395 deixou Color fallback) | `Value::Tiling` (existe) | M | Pronto |

**Nota:** Esta trilha é grande no total (L). Pode ser fatiada e intercalada com
trilhas mais pequenas. A sonda inicial deve verificar se `Value::Gradient` é
mesmo placeholder ou se já tem infraestrutura parcial (P395/P396).

---

## Trilha 5 — Shaping / rustybuzz (BLOQUEADA)

Cluster de texto que depende de integração rustybuzz. É o maior bloco de
`ausente` em Text features.

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| Sonda de viabilidade de rustybuzz no projeto | — | S | **Próximo** |
| smallcaps OpenType `smcp`/`c2sc` nativo (P446 fez fallback por scaling) | rustybuzz | M | Bloqueado |
| `text.dir` / RTL / bidi | rustybuzz + bidi | L | Bloqueado |
| `text.region` (variantes regionais) | rustybuzz | M | Bloqueado |
| `text.script` | rustybuzz | M | Bloqueado |
| Soft hyphen `\u{00AD}` na quebra de linha | hyphenation | S | Parcialmente bloqueado |

**Nota:** Tratada como épico próprio. Não intercalar com as outras trilhas até
a sonda de viabilidade de rustybuzz determinar o tamanho real. O projeto já
tem `typst-core` com dependência em `comemo` (0.4); verificar se `rustybuzz` já
está no `Cargo.toml` ou se é adição nova.

---

## Trilha 6 — Bibliografia Fase 2 (DESBLOQUEADA)

DEBT-55 fechou a Fase 1 (autor-data + alfabética). A Fase 2 foi explicitamente
diferida em P388. **Desbloqueada por Trilha 1 e Trilha 2 completas** (numeração +
label/ref permitem back-references e numeração por ordem de aparição).

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| Estilos numéricos (`[1]`, `[2]`) | Trilha 1 (numeração) + Trilha 2 (label/ref) | M | **Próximo** |
| Numeração por ordem de aparição | introspecção ordenada | M | Pronto |
| Back-references ("ver [3]") | Trilha 2 (label/ref) | M | Pronto |
| `ibid` / `op. cit.` | acima | S | Depende dos anteriores |
| Múltiplas bibliografias num documento | P420 deixou scope-out | M | Pronto |
| List of Figures / List of Tables | Trilha 1 + Trilha 2 | S-M | Pronto |

**Nota:** A sonda de P388 confirmou que `query_by_kind`, `position_of` e
`layout_with_introspector` existem. O substrato 2-pass está lá; falta a
materialização dos estilos de citação.

---

## Trilha 7 — Layout multi-região

Os refactors pesados do Layouter, diferidos por ADR.

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| `columns`/`colbreak` fluxo multi-região real (hoje width reduzida, single-render) | refactor Layouter | L | Pronto, pesado |
| `measure()` queries de runtime genuínas (ADR-0066) | introspecção | L | Pronto, pesado |
| Elemento `table` real (hoje só `grid`) | grid existente | M | Pronto |

**Nota:** Trilha pesada (L-L). Recomenda-se intercalar com trilhas S (Trilha 8)
para manter momentum. A sonda inicial deve verificar o estado actual do
Layouter: `columns` é realmente single-render com width reduzida, ou já há
infraestrutura de multi-região parcial?

---

## Trilha 8 — Refinos de stdlib e tipos (preenchimento de baixo risco)

Itens `parcial` pequenos, bons para intercalar entre trilhas grandes.

| Passo-tópico | Tamanho | Estado |
|---|---|---|
| `repr()` completo (hoje subset) | S | Pronto |
| Métodos restantes de `array`/`dict`/`str` | S | Pronto |
| Tipos `Value`: `Relative` (Rel<Length>), `Symbol`, `Dyn` | S cada | Pronto |
| `pad`/`corners`/`sides` inset modeling | S | Pronto |
| Parâmetros configuráveis de `sub`/`super`/`highlight`/decorações (offset, extent, size) | S cada | Pronto |
| Marcadores configuráveis de `list`/`enum` | S | Pronto |
| Prefixo i18n de caption ("Figure" vs "Figura") — divergência declarada de P454 | S | Pronto |

**Nota:** Bons para manter momentum entre trilhas grandes (4, 6, 7). Podem ser
executados em sequência rápida (XS-S cada).

---

## Trilha 9 — DEBT-2 (FECHADA)

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| Premissa "vanilla é lazy" | teste de paridade | S | **FECHADO em P458** |
| Oráculo de introspecção (consolidação) | — | M-L | Não necessário (premissa refutada) |
| `TrackedWorld` via `comemo` (performance) | benchmark P441 | XL | Não necessário (premissa refutada) |

**Nota:** Fechado em P458. O Typst vanilla também é eager (captura por valor no
momento da definição). O cristalino bate com o vanilla. Sem divergência
semântica. Sem código necessário.

---

## Dívida técnica residual (não ativa, mas a registar)

| Item | Origem | Impacto | Estado |
|------|--------|---------|--------|
| `Content::Label` vs `Content::Labelled` | P460/P329 | Dois tipos para mesmo conceito; complica walk de introspeção, layout, export | **RESOLVIDO em P464** |
| `label_to_counter_key` duplicado em `Introspector` e `TagIntrospector` | P462 | Manutenção dupla; um é trait, outro é impl | Não alterado em P464 (scope-out explícito); refacto futuro separado |
| `LinkItem` com `items: Vec<FrameItem>` vs `body: Frame` | P452 | Divergência documentada em nota retroativa P452 | Não bloqueante; manter `items` |
| Gate ADR-0117 no `crystalline-lint` | P453 | Proposta, não implementado | Implementar ou manter disciplina manual |
| Stack overflow em `p350c_flag_on_nao_convergente_classifica` | Pré-existente | Teste passa com `RUST_MIN_STACK=8388608`; investigar separadamente se necessário | Conhecido |

---

## Estado pós-P464

- **P464 FECHADO** — `Content::Label` e `Content::Labelled` unificados num único
  `Content::Label` com campo `auto: bool`.
- **Inventário de débitos técnicos: LIMPO** (apenas items conscientemente
  mantidos, nenhum bloqueador).
- **Trilha 1: COMPLETA E COERENTE.**
- **Trilha 2: COMPLETA.**

## Ordem sugerida para próxima conversa

A ordem considera dependências, custo e momentum. Não é obrigatória.

1. **Trilha 8** — 2-3 refinos de stdlib (S cada, ~30 min total).
   Mantém momentum com vitórias rápidas.
2. **Trilha 3** — Sonda `Selector::Where` (S, ~15 min).
   Se parcial, fecha rápido; se ausente, vira S-M.
3. **Trilha 6** — Bibliografia Fase 2: estilos numéricos (M, ~40 min).
   Maior impacto de valor agora que Trilha 1+2 estão completas.
5. **Trilha 8** — Mais 2-3 refinos (intercalar).
6. **Trilha 4** — `Value::Gradient` tipo real (M, ~35 min).
   Abre trilha visual; pode ser fatiada.
7. **Trilha 7** — `columns`/`colbreak` (L, ~60+ min).
   Refactor pesado; deixar para quando houver bloco de tempo.
8. **Trilha 5** — Sonda rustybuzz (S, ~15 min).
   Decide se trilha é viável ou se fica para épico separado.

---

## O que confirmar antes de começar a próxima conversa

1. **P464** — Cleanup Label/Labelled: executar ou adiar?
2. **Trilha 3** — Sonda `Selector::Where`: o inventário diz "ausente" mas
   P417/P423 mexeram em `Selector`. Verificar estado real.
3. **Trilha 4** — `Value::Gradient`: é mesmo placeholder ou já tem infra parcial?
4. **Trilha 5** — rustybuzz: já está no `Cargo.toml`?
5. **Trilha 7** — `columns`: é realmente single-render com width reduzida?
6. **Gate do linter** — ADR-0117 cláusula 3: implementar no `crystalline-lint`?

---

## Inventário de cobertura — atualização rápida

- **68 ADRs** documentadas (P453 + ADR-0117).
- **Zero débitos técnicos ativos** (DEBT-2 fechado em P458, DEBT-58 dissolvido em P453).
- **Trilha 1 completa**, **Trilha 2 completa**.
- **Divergências de paridade declaradas:**
  - Prefixo de caption: "Figura" (cristalino) vs "Figure" (vanilla) — P454.
  - `link` styling (underline, color) não aplicado por default — P452/P463 scope-out.
  - i18n de supplements de `ref` — P462 scope-out.
  - Page numbers em TOC como placeholder — P457 scope-out.

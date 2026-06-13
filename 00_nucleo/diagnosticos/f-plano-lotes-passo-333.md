# Plano de lotes do F sob a fronteira E1 (P333 Parte 4 — proposta, não execução)

> **Proposta.** Nenhum lote executa neste passo. A sequência segue ADR-0106
> item 3 (migração incremental) e o L0 `entities/f_fronteira_e1.md`. Custo por
> **preditor** (largura por grep, inventário 1c P331 + baseline da lente P333).
> Cada lote na faixa validada (~110–150 sites) ou com a **válvula declarada**.
> Antes/depois: **perf** = baseline 10× `0.6518 s ± 0.0057` (P330); **estrutura**
> = lente `tekt-cargo-dsm@98d8f9e` (edges `content→elements::*`, P333 Parte 5).

---

## Sequência (por valor e dependência)

### Lote F-1 — A fronteira (aditivo) · primeiro movimento

**Conteúdo** (L0 §3a): variante `Content::Dynamic(Arc<dyn DynElement>)` + trait
object-safe `DynElement` + **blanket impl** `impl<T: Element + Any> DynElement for
T` + os 6 arms do hub + `extract_payload`/`locatable`/`content_hash` dinâmicos +
o **registro injetado** (construtor por nome, sem global — pureza L1) + identidade
dinâmica para `#show` (⟨S* do spike-2⟩).

**Largura (preditor)**: aditivo — **+1 variante**, **~9 arms** em `content.rs` (6
matches + eq + hash + payload/locatable), **+1 trait + 1 blanket** em
`elements/mod.rs`. **Os 65 nativos NÃO mudam** (o blanket bridga; o `&mut dyn
FnMut` é aceite pelos `map_*<F>` existentes sem editar os 65). **Faixa: pequeno-
médio** (~15–25 sites), bem abaixo do teto.

**Válvula**: se a identidade dinâmica para `#show` (S*) exigir mais do trait do que
`dyn_kind_name`, o excedente é declarado e fica para F-2 (não incha F-1).

**Verificação**: suíte verde (aditivo, 0 asserção alterada) + teste de
object-safety (`Arc<dyn DynElement>` construível + 6 dispatches) + um elemento de
utilizador de teste (o `callout`, **fora** dos 65, em fixtures) prova os dois
públicos. **Lente**: `edges(content→elements::*)` **inalterado** (F-1 não remove
acoplamento ainda; só adiciona a porta). **Perf**: nativos sem regressão (§4* do
P332 — o `dyn` só toca a folha de utilizador).

### Lote F-2 — O canal único das `Set*` (o F-D renascido)

**Conteúdo** (L0 §3b.5): a chain única estendida (10 nativas fechadas + canal
aberto) + as 4 `Set*` como **entradas na chain** resolvidas por fallback léxico
(fecha o DEBT 99.E: set rule respeita escopo de container). Resolve `SetPage`
(2 produtores, D4/Q4) e a lacuna do produtor de `SetEquationNumbering` (D1/Q3).

**Largura (preditor, 1c)**: `SetHeadingNumbering` **62** + `SetEquationNumbering`
**16** + `SetPage` **8** + `SetFigureNumbering` **5** + os 4 canais distintos
(Introspector ×2, `page_config`, assar em `Figure`) = **~108 sites** (eco da
opção D/B do dossiê). **Faixa: dentro do validado (~110).**

**Válvula**: se `SetPage` (o caso difícil) sozinho empurrar acima do teto, fatia-se
em F-2a (`SetHeadingNumbering`+`SetFigureNumbering`+`SetEquationNumbering`, ~83) e
F-2b (`SetPage`, ~8 + a mecânica de `page_config`←chain).

**Verificação**: a **rede de caracterização (+11, P331 Fase 2)** é a spec de
paridade — **nenhuma asserção alterada**. **Lente**: primeiro par antes/depois
real (`--comparar`) — exercita R5; mede se o canal único **reduz** acoplamento dos
4 canais. **Perf**: antes/depois vs `0.6518 s`.

### Emenda de sequência (carona C3, P335)

A reconhecida heterogeneidade das `Set*` (3 subsistemas: numbering via
Introspector/StateRegistry; `SetPage` via `page_config`; `SetFigureNumbering`
assado em `FigureElem`) + a dependência real (a **realização** do `#show` monta
sobre a chain léxica que o canal `Set*` prova) emendam a fila:

- **F-2** = **só** o canal `Set*` (este lote; a chain léxica + B1 + B3 + a trava).
- **F-3 (novo)** = **realização / `#show`** (S2–S6 + guards na camada `rules/` +
  o teste de transparência da condição **Trava-Q1** + o **fecho do DEBT C2**, o
  no-op de layout do `Content::Dynamic`).
- Fila renumerada: `Styled` → **F-4**; de-bake `#set text` → **F-5**; 3 folhas →
  **F-6**.

**Ordem dimensionada e DECIDIDA (P337, recon `f-recon-passo-337.md`):**
`F-4 → F-realização → F-5 → F-6`. O recon mapeou o **conjunto-interseção
F-5↔F-realização**: o de-bake-para-chain precisa do `custom` do `#set` disponível
no consumidor, mas `#set` **não emite `Content::Styled`** (`rules.rs:242/265/297`)
— o valor só chega por **baking**; o transporte confinado que o de-bake precisaria
é **exatamente** o `StyledElem`-scoped que a F-realização constrói. Logo F-realização
**antes** de F-5 (não des-assar duas vezes; não migrar consumo para modelo
substituído). Dono aceitou front-loadar o risco (F-realização é o lote maior).
Tabela abaixo reordenada e dimensionada (números exatos no recon).

| Lote | Conteúdo | Gatilho de revisita | Largura (recon P337) |
|------|----------|---------------------|--------------------|
| **F-3 — fronteira na linguagem** ✅ **FECHADO** | inc-1: DEBT C2 (dinâmico renderiza); inc-2: registry→escopo (`#name(args)`) + `#show <dyn>` eager (`Selector::DynKind`, guard por RuleId) + Stage 0 executado | — | feito (P336; commits inc-2 S1/S2/S3/S4) |
| **① F-4 — `Styled`** | colapsa a **dualidade de backing** `StyleDelta`↔`Styled.Styles` (não "2ª chain" — recon corrigiu: 1 chain/fase) + fecha **B2** (`is_empty` sem arm Styled, `content.rs:1566`) | fundação — antes de tudo | 2 construtores prod. + 3 preservadores + ~7 consumidores |
| **② F-realização** (gatilho disparado em F-3 inc-2 S3) | `#show` **léxico** (caso 4: `[]` vaza em `eval/mod.rs:460`, `{}` confina; `#set` já escopado) + composição (caso 1) + `Transformation::Style` show-set (caso 3) — multi-passe `StyledElem`-scoped (`lab/.../content/mod.rs:744-752`) | **já disparado** | ~15 sítios show-state; toca o eager dos **nativos**; **20** testes de `#show` (1 vira, 18 revisão, 1 permanece) |
| **③ F-5 — de-bake** | os **4 pontos assados** (heading/equation/figure `numbering` + `Content::Text` `TextStyle`) deixam de assar; consumidor lê a chain que a F-realização garante no nó | depois de F-realização (transporte pronto) | 4 pontos; 28 refs de teste `numbering_active`; **risco**: 47 refs `is_numbering_active` em `introspector.rs` (vivo/morto?) |
| **④ F-6 — 3 folhas** | `Text`/`MathText`/`MathIdent` recebem estilo via chain (DEBT-58) | após F-4 (tampão possível) | Text 4/7 · MathText 6/5 · MathIdent 2/5 |

**Condição do tampão F-6 (C1, P338 — lição do S5b):** se o F-6 for usado como
**tampão** (rotear o estilo da folha pela chain **mantendo o campo assado** —
content-preserving), o caminho duplo que nasce **não** pode dormir: nasce com
**(a)** gatilho de remoção escrito (o F-5 remove o campo assado) e **(b)** teste de
**paridade entre os dois caminhos** enquanto coexistirem. Sem isso, o caminho
morto-mas-alimentado mascara (foi exatamente o que escondeu o auto-TOC no S5b).

**Destino dos consertos B1/B2/B3 do P331** (registrar; B1/B3 neste lote F-2):
- **B1** (`SetEquationNumbering` sem produtor eval) → resolvido **no canal F-2**
  (P335; paridade de linguagem declarada).
- **B2** (`Styled.is_empty` cai em `_ => false`) → resolvido **no lote F-4**.
- **B3** (`world_types::Styles(())` stub morto) → removido em **F-2** (a chain
  real substitui o stub).

---

## Critérios transversais a todos os lotes

1. **Aditivo/content-preserving**: nenhuma asserção existente alterada (alterar
   teste para passar = bug). Um commit isolável por lote.
2. **Trava ADR-0105 cláusula 3**: o lote que introduzir o caminho dinâmico de
   layout/show (F-1/F-2) **constrói a trava** (teste-varre-registro ou regra
   `crystalline-lint`) ANTES de relaxar o compilador. A lente **não** a substitui
   (P333 Parte 5 §3: módulo-level, não desce à tabela de handlers — R3/R4).
3. **Lente por lote**: `edges(content→elements::*)` e o delta `--comparar` entram
   no relatório de cada lote (separação de camadas medida, não argumentada).
4. **Perf por lote**: `0.6518 s ± 0.0057` (P330) é o antes; cada lote reporta o
   depois (≥10 execuções).
5. **L0 primeiro**: cada lote audita/atualiza o L0 `f_fronteira_e1.md` e
   sincroniza o hash ANTES do código (Trava arquitetural, CLAUDE.md).

---

## O que a lente recomendou de volta (P333 Parte 5 — R* para a sessão do tekt-cargo-dsm)

R1 (multi-crate), R2 (instabilidade/violação de camada), R3 (granularidade de
item, não de módulo), R4 (distinguir import-de-trait vs import-de-struct — **o
critério exato do F**), R5 (diff em par real — exercitado em F-2). **Não
consertados aqui** (material para a sessão da lente; precedente 0052 do linter).
R4 é o mais relevante para o F: sem ele, a "separação de camadas" medida é
grosseira (conta edges, não distingue o `use trait Element` legítimo do
`use ConcreteElem` acoplante).

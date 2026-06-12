# Relatório P335 — Lote F-2: canal único das `Set*` (fecha DEBT 99.E)

**Pré-condição**: F-1 (P334) fechado — fronteira no produto, suíte 2720, lint 0/0. ✅
**Tipo**: caronas (commit próprio) + **Lote F-2** — as `Set*` como entradas na
chain léxica. Decisão do dono: **tudo (as 4), faseado**.
**Resultado**: **DEBT 99.E fechado** para a tríade de numeração (heading/equation/
figure), com escopo léxico **provado**. Suíte 2720 → **2727**, lint 0/0.

---

## Caronas (commit `249497558`)

- **C1** — protocolo canônico de perf: o absoluto `0.6518 s` (P330) fica superseded
  (deriva de ambiente comprovada no P334); a prova de não-regressão é o par
  back-to-back na **mesma sessão** (`medicao-pre-f-passo-330.md` §C1).
- **C2** — DEBT do no-op de layout do `Content::Dynamic` (`debt-layout-noop-dinamico.md`),
  fecha no lote de realização (F-3).
- **C3** — emenda de sequência: F-3 = realização/`#show`; fila renumerada
  (Styled→F-4, de-bake→F-5, folhas→F-6).

## Fase A — reconhecimento (3 agentes) + checkpoint

Os 4 fluxos das `Set*` mapeados com `file:line`. **Achado-chave**: as "4 Set*"
são **3 subsistemas heterogéneos** — numbering (heading/equation via Introspector
→ StateRegistry), `SetPage` (`page_config`, 34 leitores), figure (assado em
`FigureElem`). **D4** resolvido: `native_page` é legacy. **B1** localizado:
equation sem produtor eval. O dono escolheu **tudo, faseado** (checkpoint).

**Mecanismo estabelecido**: o scoping léxico **já existe** (`engine.styles` é
escopado por `local_styles`); as `Set*` eram outliers que devolviam marcadores
flat. Migrá-las = adotar o padrão escopado (eco do figure-baking).

## Fase B — estágios (commits isoláveis)

| Estágio | O quê | Commit |
|---------|-------|--------|
| Fundação | canal aberto `StyleDelta.custom: Vec<(PropKey,Value)>` + `push_custom`/`custom` (fallback léxico) + **B3** (stub `Styles(())` removido) | `8a6aadeef` |
| **S1 heading** | `#set heading(numbering:)` → chain; `HeadingElem.numbering_active` assado; consumer lê o campo; +3 testes eval (inclui **escopo léxico não-vaza**) | `4d0a52571` |
| **S2 equation + B1** | idem + **B1** (produtor eval em falta — target pontuado `math.equation` via `FieldAccess`); gate do contador via `ElementPayload::Equation.numbering_active` | `d64950919` |
| **S3 figure** | move a fonte de `engine.figure_numbering` (global) para `engine.styles.custom` (léxico); `native_figure` assa da chain | `0d37be3bc` |
| **S4 SetPage** | **D4** (remove `native_page` legacy) + registro: geometria de página é **estado de região, por desenho** (fora do DEBT 99.E) + item de paridade futura | `c36dd223a` |
| **S5a trava** | teste-varre-tabela do canal (chaves × tipos + top-wins léxico) | `1567f51e6` |

**DEBT 99.E fechado** para heading/equation/figure: `#set X(numbering:)` dentro
de um bloco `[...]` **não vaza** para fora — provado por 3 testes eval
`f2s{1,2,3}_..._escopo_lexical_nao_vaza`. **B1** fechado: a numeração de equação
**funciona pela primeira vez** (nunca teve produtor em produção). **B3** e **D4**
feitos.

## Decisões registadas

- **SetPage = estado de região por desenho** (decisão do dono, S4): a geometria
  de página usa o modelo marcador/`page_config`/nova-página (~34 leitores; "nova
  página ao mudar") — **não** estilo lexical. Fora do alvo do DEBT 99.E.
  Registrado em `debt-stylechain-nao-materializada.md` §Geometria de página +
  item de paridade futura (`#set page` dentro de bloco — medir o vanilla quando
  a cobertura chegar lá).
- **Opção B** (estado misto): os marcadores `Set*Numbering` + plumbing de
  introspecção ficam **inertes em produção** (o eval já não os produz) mas ainda
  testados — minimizou o churn de teste por estágio (5/6 funções por estágio em
  vez de ~45).

## S5b — adiado (cleanup de código morto)

A remoção dos marcadores `Set*Numbering` + plumbing revelou superfície de **~50
funções de teste** (umas testam a plumbing removida → apagar; outras usam o
marcador como veículo para TOC/labels/contadores → migrar caso a caso). A
remoção do **código de produção** compila limpa, mas as ~50 edições de teste são
um cleanup dedicado (julgamento por função) — **adiado para passagem focada** em
vez de arriscar erro no fim de uma sessão longa. **Nada bloqueia**: os marcadores
inertes não afetam produção. (Estado retomável em `f2-progresso-passo-335.md`.)

## Verificação

- **Suíte** `cargo test -p typst-core --release` → **2727 passed; 0 failed**
  (2720 + 7 líquidos: +trava +3 heading eval +2 equation eval +2 figure eval, −1
  stub B3, +migrações content-preserving).
- **Lint** `crystalline-lint .` → **0 violations, 0 warnings**.
- **Workspace** `cargo build` limpo.
- **Perf** (protocolo C1): a tríade migra via eval/elemento; nativos sem regressão
  estrutural (enum `Content` inalterado em tamanho — sem variante nova; só campos
  `numbering_active` em Heading/Equation, atrás do `Arc`).
- **Lente** `--comparar` (R5): adiada com o cleanup S5b (o colapso dos canais que
  reduz acoplamento acontece quando os marcadores saírem).
- **Caveat de stack**: `RUST_MIN_STACK=33554432`.

## Contabilidade do F

- **F-2 funcionalmente fechado** — DEBT 99.E fechado para a tríade de numeração;
  `SetPage` registado por desenho; B1/B3/D4 feitos; a trava no lugar.
- **Pendente**: S5b (cleanup de código morto, passagem focada) + lente `--comparar`.
- **Próximo: F-3** — realização/`#show` (S2–S6 do spike-2, guards, transparência
  Trava-Q1, fecho do DEBT C2 do no-op de layout).

## git log (P335)

`249497558` caronas · `8a6aadeef` fundação+B3 · `4d0a52571` S1 · `d64950919` S2+B1
· `0d37be3bc` S3 · `c36dd223a` S4+D4 · `1567f51e6` S5a trava.

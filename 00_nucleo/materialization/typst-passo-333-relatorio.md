# Relatório P333 — Fronteira E1 decidida + spike-2 `#show` + L0 do F + plano de lotes + baseline da lente

**Pré-condição**: P332 fechado — experimento E1/E2/E3, tabela, produto intocado,
suíte 2708, lint 0. ✅ Verificado.
**Tipo**: registro da decisão (do dono) + experimento descartável (spike-2) + L0
completo do F (design-ahead) + plano de lotes + baseline estrutural (lente).
**Zero código de produto em todo o passo.** Termina na **Trava** (checkpoint).
**Commits** (5, isoláveis): decisão · baseline estrutural · spike-2 · L0 · plano.

---

## Estado por parte

| Parte | Estado | Entregável | Commit |
|-------|--------|-----------|--------|
| 1 — decisão da fronteira | ✅ | ADR-0106 + DEBT 99.E + nota de fecho P332 + adendo dossiê | `15e384833` |
| 5 — baseline estrutural (lente) | ✅ | `baseline-estrutural-lente-passo-333.md` | `2405e48aa` |
| 2 — spike-2 `#show` | ✅ | `lab/spikes/f-extensao/e1/` + `f-spike2-show-passo-333.md` | `feb7d2117` |
| 3 — L0 do F | ✅ | `prompts/entities/f_fronteira_e1.md` | `0450341e9` |
| 4 — plano de lotes | ✅ | `f-plano-lotes-passo-333.md` | `50cfea5a2` |

Progresso retomável: `f-progresso-passo-333.md`. (Ordem executada: 1 → 5 ‖ 2
em paralelo → 3 → 4.)

## Parte 1 — onde mora a decisão

**Fronteira E1** escolhida pelo dono. **ADR-0106** (nova) grava: a decisão, as
razões (tabela P332 §4* nativos sem imposto, precedente vivo → menor custo-IA,
migração aditiva), os descartes (E2 rejeitada por downcast silencioso; E3
alternativa), o `enum Value` fechado por espelhar a linguagem (escape `Custom`
condicional), a migração incremental, Q1–Q7 do P332 respondidas por consequência.
Replicado em **DEBT 99.E** (§Decisão da fronteira), **nota de fecho** no
`f-experimento-extensao-passo-332.md` (+ conserto do placeholder duplicado da
Parte 4, defeito do commit P332), **adendo final** no dossiê P331.

## Parte 2 — spike-2 `#show` (5 casos, todos PASS)

`cd lab/spikes/f-extensao/e1 && cargo run --release`:

| Caso | Comportamento | Resultado |
|------|---------------|-----------|
| 1 Multi-regra | innermost-first, 1 func/passe; converge em 3 passes | **PASS** |
| 2 Recursão | recipe auto-produtora termina em 2 passes via guard | **PASS** |
| 3 Show-set | `#show k: set k(...)` sobrepõe prop sem consumir passe | **PASS** |
| 4 Escopo | recipe scoped não vaza para o irmão | **PASS** |
| 5 Nativo+Dinâmico | a mesma chain transforma `heading` nativo E `callout` dinâmico | **PASS** |

**S1–S7** (exigências da semântica vanilla sobre E1): S1 id estável `Eq`; S2
guard por-nó na camada de realização (não no `Arc<dyn>`); S3 recipes na chain,
innermost-first, 1 func/passe; S4 loop multi-passe + teto; S5 `Transformation =
Content|Func|Style`; S6 nó dinâmico = membro pleno da árvore; S7 `get_field` para
o closure. **Sem contradição dura.** Duas dependências de Trava: **Q1** (sítio do
guard — a única divergência estrutural vs vanilla) e **Q2** (id+`get_field` no
contrato). Limites: guards num wrapper, escopo por chains separadas, selector só
por kind, filhos de dinâmico não re-realizados, sem comemo/introspection.

## Parte 3 — L0 do F (`prompts/entities/f_fronteira_e1.md`, design-ahead)

- **§0** estatuto: design-ahead, não hashed até F-1 aterrar; L0 existentes
  intocados.
- **§3a** lado elemento: `Content::Dynamic(Arc<dyn DynElement>)`; **object-safety
  resolvida sem tocar os 65 e sem trait novo p/ o utilizador** — trait object-safe
  `DynElement` + **blanket `impl<T: Element> DynElement for T`** (o utilizador
  escreve o **mesmo** `trait Element`, precedente vivo); eq/hash/clone dinâmicos;
  identidade+`get_field` p/ `#show` (S1/S7); realização+guards na camada `rules/`
  (S2/S3/S4/S6 — resolução proposta de Trava-Q1: guard é estado da realização, num
  invólucro transparente, não no `dyn` nem nos 65); registro injetado (pureza L1);
  hub +1 variante e ~9 arms (aditivo).
- **§3b** lado estilo: chain única (10 nativas fechadas + canal aberto
  `PropKey→Value`); `Value` fechado por espelhar a linguagem; **`Value::Custom`
  NÃO entra** (evidência P333: tudo coube no `Value` fechado — gatilho de
  reabertura registrado); canal único das `Set*` (~108 sites, fecha DEBT 99.E,
  resolve `SetPage`/`SetEquationNumbering`); `#show` desenhado por S3–S6;
  fronteiras (`Styled`/de-bake/3 folhas) declaradas por lote.
- **§3c** contrato: C1–C8 + S1–S7 + rede de caracterização (+11, P331) + trava
  ADR-0105 cláusula 3 (a lente **não** a substitui — R3/R4) + teste de
  object-safety. Trava-Q1/Q2 = decisões do dono.

## Parte 4 — plano de lotes (`f-plano-lotes-passo-333.md`, proposta)

**F-1** a fronteira (aditivo, ~15–25 sites, 65 nativos intocados) → **F-2** canal
único das `Set*` (~108 sites, fecha DEBT 99.E; válvula: fatiar `SetPage`) → fila
incremental **F-3** `Styled` · **F-4** de-bake `#set text` (~283 pior caso) ·
**F-5** 3 folhas, com gatilhos. Consertos B1→F-2, B2→F-3, B3→primeiro lote.
Transversais: aditivo/content-preserving; trava ADR-0105 cláusula 3 antes de
relaxar o compilador; lente + perf (`0.6518 s`) por lote; L0 primeiro.

## Parte 5 — baseline estrutural (`baseline-estrutural-lente-passo-333.md`)

**Lente**: `tekt-cargo-dsm@98d8f9e` (grafo via `cargo-modules` v0.27.0), binário
`lente`. **Comandos** (de `01_core/`, `--pacote typst-core`): `lente --estrutura
--so-referencia --filtrar-stdlib` (+`--text`/`--html`), `--ranking --top 15`,
`--diff/--comparar`. **Números** (so-referencia / acoplamento genuíno):
- typst-core: **217 módulos, 667 edges, 3 ciclos**.
- Hub `content`: fan-in **85**, fan-out **82** (65 → módulos de elemento, 17 →
  outros) — assinatura do enum fechado (D).
- 65 módulos de elemento: **0 edges elemento→elemento, 0 ciclos** entre eles —
  **atomização confirmada por computação**.
- SCC grande (81 módulos) = content + 65 elementos via content; remover os 65
  `use elements::*Elem` de content.rs derruba fan-out 82→17 e colapsa o ciclo.

**R\***: R0 (não ocorreu — rodou) · R1 (sem multi-crate) · R2 (sem instabilidade/
violação de camada) · R3 (granularidade de módulo, não de item) · **R4 (não
distingue import-de-trait vs import-de-struct — o critério exato do F)** · R5
(diff em par real — exercitado em F-2). §3: separação de camadas **SIM** (grosseira,
via `--comparar`); trava ADR-0105 cláusula 3 **NÃO** (módulo-level — R3/R4).

---

## Verificação final

- **Produto intocado**: `git status` em `01_core/`–`04_wiring/` limpo (só
  `lab/spikes/` e docs novos; o passo spec `typst-passo-333.md` fica untracked).
- **Suíte**: `cargo test -p typst-core --release` → **2708 passed; 0 failed**
  (3 ignored = `recursao_infinita_*`).
- **Lint produto**: `crystalline-lint .` → **exit 0, zero violations**, com **1
  warning V7 esperado**: o L0 `f_fronteira_e1.md` é **órfão** (nenhum código L1–L4
  o referencia) — exatamente o estado design-ahead (§0); o warning **clears**
  quando F-1 aterrar e declarar `@prompt entities/f_fronteira_e1.md`. Os spikes
  ficam **fora do gate** (workspace próprio).
- **Caveat de stack**: `RUST_MIN_STACK=33554432`.
- **git log** (P333): `15e384833` decisão · `2405e48aa` lente · `feb7d2117`
  spike-2 · `0450341e9` L0 · `50cfea5a2` plano.

## Checkpoint (a Trava — perguntas ao dono)

1. **Aprova o L0** `f_fronteira_e1.md` (object-safety por blanket `DynElement`;
   chain única; `Value` fechado; canal `Set*`; `#show` por S3–S6)?
2. **Trava-Q1**: aceita a resolução proposta (guard/lifecycle na **camada de
   realização** `rules/`, invólucro transparente uniforme nativo+dinâmico) em vez
   de um campo `meta` ao lado do `Arc` na variante `Dynamic`?
3. **Trava-Q2**: confirma `dyn_kind_name` (id estável) + `get_field` (campos) como
   contrato obrigatório do elemento dinâmico?
4. **`Value::Custom`**: confirma mantê-lo **fora** (evidência P333) com o gatilho
   de reabertura registrado?
5. **Plano de lotes**: aprova F-1 (fronteira) como primeiro lote, com a trava
   ADR-0105 cláusula 3 construída em F-1/F-2?
6. **R4 da lente**: quer abrir a sessão `tekt-cargo-dsm` para R4 (distinguir
   import-de-trait vs struct) antes de F-1, para a "separação de camadas" ser
   medida fina? (Não bloqueia F-1.)

**Nenhum código de produto até o dono aprovar o L0.**

## Fora de escopo (confirmado)

Implementação de qualquer parte do F (por lotes, pós-Trava); mudanças em
`Set*`/`Styled`/folhas (desenhadas, não tocadas); consertos B1/B2/B3 (destinos
registrados); consertos/refinamentos na lente (R* são material da sessão
`tekt-cargo-dsm`); otimizações.

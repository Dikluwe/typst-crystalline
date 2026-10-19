# Relatório P338 — Lote F-4 `Styled`: colapso da dualidade de backing + B2

**Pré-condição**: P337 fechado (`1be7083df`), ordem F-4→F-realização→F-5→F-6
decidida. **Suíte-base do P338 = 2718** (C0: o `1be7083df` adicionou o teste-
contrato da `FuncRepr::Element`, +1).
**Resultado**: a **dualidade de backing** `Styles(Vec<Style>)`↔`StyleDelta`
colapsou — `Styles` virou **fachada sobre `StyleDelta`** (backing único); **B2**
fechado; a API legada `is_numbering_active` (morta pós-F-2) removida. **F-4 ✅.**

---

## Contabilidade por commit

| # | Commit | Conteúdo | Δ suíte (typst-core) |
|---|--------|----------|----------------------|
| caronas | `a7b8ac7e1` | C0 (contabilidade 2718) + C1 (condição do tampão F-6) | 0 (docs) |
| E0 | `8e4eaf80c` | triagem-47: remove `is_numbering_active`/`_at` (morto) | **2718 → 2708** (−10) |
| E0-fix | `07425e1d7` | **correção**: completa a remoção em L3 (proxy de medição) | 0 (renumber) |
| S1 | `9f7e0b605` | fecha **B2** (`is_empty` arm `Content::Styled`) | 2708 → **2709** (+1) |
| S2 | `bcc20a07e` | **o colapso** (Styles vira fachada de StyleDelta) | 2709 → **2710** (−2 iter +3 delta) |
| S3 | `ea6f5a5e2` | trava do backing único (varre-10-campos) | 2710 → **2711** (+1) |
| S4 | (este) | registro + fecho (fossil, plano, perf, lente, relatório) | 0 |

**Workspace inteiro**: 3230 testes verdes (todas as crates). Lint 0/0.

## Estágio 0 — triagem-47: VEREDITO morto (Desfecho A), com uma lição

A API `is_numbering_active`/`is_numbering_active_at` (gate por StateRegistry,
chave `numbering_active:*`) tinha **zero chamadores de produção**: todas as refs
em `01_core` eram comentários ou testes da própria plumbing, e o canal perdeu o
produtor no **F-2 S5**. **Prova de mordida**: removidos os 3 injectores
`state_update("numbering_active:equation")` dos testes de equação, o counter
manteve [1,2,3] — vem do **campo assado** `ElementPayload::Equation.numbering_
active`, não da chave. Removido: 2 métodos de trait + 2 impls + 10 testes + 1
asserção-cauda + 3 injectores.

**A lição (registrada honestamente):** a triagem fez grep só em `01_core/src` e
**MISSED** o proxy de medição `CountingIntrospector` em `03_infra/src/
measurements.rs`, que forwarda cada método do trait. `cargo test -p typst-core`
passou (não compila L3); o **build release do workspace falhava** (E0407/E0599),
o que só foi apanhado ao construir o binário para a perf. **Regra nova**: remoção
de símbolo de um trait L1 tem de varrer **todas** as camadas (L1–L4), não a do
trait. O veredito "morto" manteve-se (o proxy só espelha a superfície do trait —
não é consumidor de numeração); a correção (`07425e1d7`) removeu os 2 forwarders
e re-compactou os índices de `record_call` 26→24 (**sem tombstone** — evita o
morto-alimentado do S5b). **Consequência na fila: o F-5 tem 4 pontos, não 5.**

## Fase A — direção do colapso (checkpoint do dono)

O recon confirmou contra o HEAD: a dualidade é `Styles(Vec<Style>)` (10 variantes,
em `Content::Styled`) ↔ `StyleDelta` (10 campos + canal `custom` do F-2, na
chain), com `push_styles` a converter (`style_chain.rs`). **Um único consumidor
de produção** lia o enum `Styles` direto (`eval/rules.rs:113-116`); os demais já
liam via chain→`StyleDelta`. `Styles::from_iter` aparece em **74 sítios** (13
produção, 61 testes).

**Assimetria decisiva**: `StyleDelta` tem `custom`; `Style` **não** — só a direção
que faz `Styled` carregar `StyleDelta` abre a porta para `#set` viajar no Styled
(que F-realização/F-5 precisam). **Decisão do dono (ii)**: `Styles` vira **fachada
sobre `StyleDelta`** (`Content::Styled(Box, Styles)` inalterado nos 74 sítios; o
enum `Style` fica como vocabulário-construtor que dobra na borda). Rejeitada: (i)
`Styled(Box, StyleDelta)` direto com `Style` removido — mesmo backing, mas 74+61
sítios reescritos (churn pesado por nada).

## Fase B — estágios

- **S1 (B2):** `is_empty` caía em `_ => false` para `Content::Styled` —
  styled-de-vazio reportava não-vazio. Arm novo delega ao body (paridade
  Block/Pad/Boxed). Varredura prévia: zero dependentes do bug. +1 teste.
- **S2 (o colapso):** `style.rs`: `Styles { delta: StyleDelta }` (era
  `Vec<Style>`); a projeção `Style→StyleDelta` mudou-se para `Style::fold_into`
  (chamada por `from_iter`/`push`, last-write-wins por campo); `iter()`/`len()`
  removidos, `delta()` adicionado. `style_chain.rs`: `push_styles` vira fachada
  fina (`push(styles.delta().clone())`), sem o `match`; `StyleDelta::is_empty()`
  adicionado. `eval/rules.rs`: deteção `#show strong/emph` lê `delta().bold/
  italic`. **L0 primeiro**: `style.md` (contrato fachada) + `f_fronteira_e1.md`
  §3b.7 (F-4 ✅, direção ii) atualizados antes do código; hashes sincronizados.
  74 sítios de construção **intactos**; render de strong/emph idêntico
  (asserções intactas). Testes do builder migrados (−2 iter +3 delta).
- **S3 (a trava):** `f4_s3_trava_backing_unico_varre_10_campos` — varre as 10
  variantes provando (a) `from_iter` dobra TODAS num só `StyleDelta` (a fachada É
  o backing) e (b) `push_styles` é **definicionalmente** `push(delta)` (compara
  accessor-a-accessor a chain via `push_styles` com a via `push(delta)`). Se
  alguém reviver um `Vec<Style>` ou reintroduzir um `match` divergente, quebra.
- **S4 (fecho):** `style_chain.rs:12` (o "coexistência até migrar") reescrito — a
  migração aconteceu; B2 ✅ e F-4 ✅ na fila; este relatório.

## Verificação transversal (critérios do plano)

- **Suíte**: typst-core **2711**; workspace **3230** — verdes. Nenhuma asserção
  de render alterada (content-preserving). Deltas exatos por estágio (tabela).
- **Lint** `crystalline-lint .`: **0 violations**. **L0 primeiro** cumprido
  (style.md/f_fronteira_e1.md/measurements.md atualizados antes/com o código;
  hashes sincronizados).
- **Perf (protocolo C1, par back-to-back na mesma sessão)** — `hyperfine`
  indisponível → fallback `/usr/bin/time -f %e`, corpus 10× (70 030 linhas), 12
  runs/cenário (1ª descartada), **interleaved**. Endpoints: **antes** = caronas
  tip `a7b8ac7e1` (último release-buildable pré-colapso — o intervalo E0→S3 teve
  o release quebrado até `07425e1d7`); **depois** = HEAD.

  | Cenário | Média ± σ (n=11) | min/max |
  |---------|------------------|---------|
  | **antes** (pré-colapso) | **0.7364 s ± 0.0677** | 0.67 / 0.92 |
  | **depois** (F-4) | **0.7391 s ± 0.0271** | 0.70 / 0.78 |

  **Delta +0.37 % (+0.0027 s)** — dentro do ruído (σ ≈ 9 %/3.7 %; um outlier 0.92
  no "antes"). **Sem regressão**: o `fold` apenas mudou de push-time para
  construct-time (ambos O(pequeno)); o push quente da chain agora só clona um
  delta. (Absolutos não comparam entre sessões — P335 C1.)
- **Lente** (`tekt-cargo-dsm --estrutura/--comparar`): **binário ausente neste
  ambiente** — não corrida (declarado, não mascarado). Efeito estrutural esperado
  **~nulo** no nível de edges `content→elements::*`: o colapso é **intra-módulo**
  (`style.rs`/`style_chain.rs`) — `Content::Styled` continua a carregar `Styles`
  (tipo inalterado), só o backing interno de `Styles` mudou; nenhuma aresta de
  módulo nova/removida. (A medir quando o `dsm` estiver disponível.)

## Contabilidade do F (fila)

- **F-4 ✅ FECHADO** — `Styles` é fachada sobre `StyleDelta` (backing único; canal
  `custom` disponível no `Styled`); **B2** fechado; triagem-47 morta-removida (o
  F-5 encolheu para 4 pontos). Correção de recon registrada: não havia "2ª chain"
  — era dualidade de backing.
- **Próximo: ② F-realização** (o gatilho disparado em F-3): `#show` léxico (caso
  4: `[]` vaza, `{}` confina) + composição + show-set. Toca o eager dos nativos.
- Depois: **③ F-5 de-bake** (4 pontos; lê a chain que a F-realização garante no
  nó — o `custom` no `Styled` que o F-4 abriu) e **④ F-6 folhas**.

# Passo 125 — Auditoria profunda dos 11 DEBTs abertos

**Data**: 2026-04-24
**Método**: grep empírico em `01_core/src/` e consulta de ADRs
activas desde o Passo 105 (25 passos: 106–124).
**Referência prévia**: `00_nucleo/diagnosticos/auditoria-debts-passo-105.md`
(tudo M; DEBT-45/49/51 fechados desde).

---

## Sumário executivo

| DEBT | Tipo | 105 | 125 | Razão |
|------|------|:---:|:---:|-------|
| 1  | partial — stylechain props adicionais + `#show` refinements | M | **M** | `StyleDelta` ainda tem 5 campos; tipos Font/Lang/Par não materializados. |
| 2  | partial — closures eager | M | **M** | `ClosureRepr.captured: Arc<Scope>` inalterado; lazy exige comemo-tracked world. |
| 8  | partial — motor de equações | M | **M** | MathPrimes só no AST; kern OpenType MATH pendente. |
| 9  | tracking contínuo paridade | M | **M** | Tracking permanente; não encerra. |
| 33 | bézier bbox conservadora | M | **M** | `CubicTo` em `geometry.rs` com comment "pode ser conservadora". |
| 34d | auto guloso vs fr | M | **M** | Grid não tem min/max-content negotiation. |
| 34e | colspan/rowspan | M | **M** | Grid.rs zero hits para colspan/rowspan — não implementado. |
| 35b | cache available_width (preventivo) | M | **M** | `available_width()` calculado em tempo real; comentário guardião em `layout/mod.rs:507`. |
| 42 | `get_unchecked` no scanner | M | **M** | 5+ `unsafe { get_unchecked(...) }` em `scanner.rs`; bloqueado por falta de infra bench. |
| 43 | linter whitelist crate-level | M | **M** | `crystalline.toml` ainda `rust = [array]` flat; bloqueado por projecto externo. |
| 50 | show selector origem (latente) | M | **M** | Canary test `debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in` em `layout/tests.rs:1787` passa. |

**Todos 11 mantêm M.** Zero fechos triviais neste passo.
Zero ADR nova necessária.

---

## ADRs emitidas entre Passos 106 e 124

**Domínio CLI** (sem impacto em DEBTs L1/L3):
- ADR-0046 (CLI mínima)
- ADR-0047 (clap)
- ADR-0048 (cores ANSI)
- ADR-0049 (CLI em L2)
- ADR-0050 (formatter em L2)
- ADR-0051 (flags funcionais)

**Domínio L3 diagnostics**:
- ADR-0042 (Sink materializado)
- ADR-0043 (canal Sink → L3)
- ADR-0044 (Engine<'a>)
- ADR-0045 (formato diagnósticos)

Candidatas a impactar DEBTs:
- **DEBT-49 (ENCERRADO 107)**: warnings de `#set`. ADR-0042/0043
  resolveram o canal Sink.
- **DEBT-51 (ENCERRADO 106)**: warnings do Sink ao caller L3/CLI.
  ADR-0043 resolveu o canal.
- **DEBT-45 (ENCERRADO 110)**: Route depth checks. Resolvido com
  decisão "não aplicável".

Nenhuma ADR recente afecta os 11 que restam abertos.

---

## DEBT-1 — StyleChain (partial)

**Última actualização**: Passo 103.

**Grep empírico**:
```
01_core/src/entities/style_chain.rs (StyleDelta):
  pub bold, italic, size, fill, heading_level  -- 5 campos
```

Nenhuma propriedade adicional (`font`, `lang`, `weight`, `leading`)
foi adicionada. Entry-point para migrar bake-in → `Content::Styled`
continua adiado (ADR-0040 em vigor, decisão registada).

**Classificação**: **M**. Trabalho real (novos campos + migração)
exige tipos Font/Lang/Par/ParagraphStyle em L1 — bloqueio
persiste.

---

## DEBT-2 — Closures eager vs lazy capture

**Última actualização**: Passo 31.

**Grep empírico**:
```rust
// 01_core/src/entities/func.rs:46
pub captured: Arc<Scope>,
```

`ClosureRepr` inalterado. Captura continua eager via snapshot
Arc. Lazy exige `TrackedWorld` real integrado com comemo.

**Classificação**: **M**. Sem mudança de contexto.

---

## DEBT-8 — Motor de equações (partial)

**Última actualização**: Passo 84.1.

**Grep empírico**:
```
01_core/src/entities/ast/math.rs: MathPrimes só AST
01_core/src/rules/math/layout/attach.rs: math_kern via FixedMetrics
```

MathPrimes tem parsing + AST; sem lógica de layout dedicada.
`math_kern` existe mas com fallback de métricas fixas, não via
tabelas MATH de OpenType.

**Classificação**: **M**. Fonts OpenType MATH + kern real são
trabalho substancial.

---

## DEBT-9 — Cobertura de paridade (tracking contínuo)

Natureza: não é dívida com "fecho". É **convenção de disciplina**
— adicionar casos de paridade a cada novo SyntaxKind ou mudança
semântica. Baseline Passo 35 mantém-se.

**Classificação**: **M**. "Nunca fecha" é a natureza do DEBT.

---

## DEBT-33 — Bézier bbox conservadora

**Grep empírico**:
```
01_core/src/entities/geometry.rs:17:    CubicTo(Point, Point, Point),
01_core/src/entities/geometry.rs:45: // pode ser conservadora para segmentos CubicTo
```

Cálculo analítico de extremos de B(t) não implementado.

**Classificação**: **M**. Trabalho matemático dedicado.

---

## DEBT-34d — Auto não encolhe antes de matar fr

**Grep empírico**:
- `grid.rs`: `available_width` consumido por Auto sem negociação
  com fr; zero hits para "min.content" / "max.content".

**Classificação**: **M**. Algoritmo de negociação min/max-content
é passo dedicado substancial.

---

## DEBT-34e — colspan e rowspan

**Grep empírico**:
```
grid.rs: zero hits para colspan|rowspan
```

Feature **não existe**. DEBT documenta o gap.

**Classificação**: **M**. Implementação requer mudança do
algoritmo de placement inteiro.

---

## DEBT-35b — Invalidação de cache de available_width (preventivo)

**Grep empírico**:
```
01_core/src/rules/layout/mod.rs:152:
  pub(super) fn available_width(&self) -> f64 { /* calcula em tempo real */ }
01_core/src/rules/layout/mod.rs:507:
  // DEBT-35b: se available_width() vier a ter cache, invalidar aqui.
```

`available_width()` continua **sem cache**. Comentário
explícito em `SetPage` serve como guardião documental para
quem no futuro pensar em adicionar cache.

**Classificação**: **M** — **preventivo activo por natureza**.
O DEBT é a documentação do risco; remover a entrada apagaria
esse guardião. Continuará M mesmo se nunca surgir cache.

---

## DEBT-42 — `get_unchecked` no scanner

**Grep empírico**:
```
01_core/src/rules/lexer/scanner.rs: 5+ `unsafe { get_unchecked(...) }`
```

Bloqueio original: "infra de benchmarking reprodutível não
existe". Esta infra **continua a não existir** em
`01_core/` (zero tests com `criterion` ou similar).

**Classificação**: **M**. Bloqueio externo persiste —
benchmark crate adoption é passo dedicado com ADR próprio.

---

## DEBT-43 — Linter whitelist crate-level vs type-level

**Grep empírico**:
```toml
# crystalline.toml
[l1_allowed_external]
rust = [ "thiserror", "comemo", "ecow", ... ]   # flat array, crate-level
```

Formato continua flat. Type-level whitelisting exige alteração
do binário `crystalline-lint` (repositório externo).

**Classificação**: **M**. Bloqueio em projecto externo.

---

## DEBT-50 — Show selector Strong/Emph origem (latente)

**Grep empírico**:
```
01_core/src/rules/layout/tests.rs:1787:
  fn debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in()
```

Canary test existe e passa (verificado pelo cargo test deste
passo: 811 L1 OK).

Dívida continua **latente** — activa-se apenas quando bake-in
→ wrapping migration acontecer. Zero trabalho aplicável hoje.

**Classificação**: **M** — latente.

---

## Observações sobre a natureza da lista

3 dos 11 são qualitativamente diferentes dos outros:

- **DEBT-9** (tracking contínuo): convenção de disciplina, não
  dívida finita.
- **DEBT-35b** (preventivo): guardião documental activo.
- **DEBT-50** (latente): guardião de regressão, activo via
  canary test.

Os 8 restantes são dívidas técnicas finitas bloqueadas por:
- **Tipos não materializados** (DEBT-1, DEBT-8).
- **Infra externa** (DEBT-42 bench, DEBT-43 linter).
- **Trabalho substantivo em código novo** (DEBT-2, DEBT-33,
  DEBT-34d, DEBT-34e).

---

## Candidatos de fecho dedicado (P — passo futuro)

Embora todos os 11 sejam M neste passo, destacam-se candidatos
com fecho mais próximo:

1. **DEBT-43 (linter type-level)**: depende apenas de 2
   alterações (1 no `crystalline-lint`, 1 no `crystalline.toml`).
   Escopo pequeno por passo; executar quando o projecto externo
   estiver acessível.
2. **DEBT-42 (`get_unchecked` scanner)**: criar infra `criterion`
   em `01_core/benches/` é passo pequeno; medição + decisão ADR
   é passo pequeno. Dois passos pequenos fecham.
3. **DEBT-1 propriedades simples** (`text.weight` como número):
   `StyleDelta.weight: Option<u16>` é 1 campo extra + 1 setter
   em `eval`. Pequeno. Outras (`font`, `lang`) dependem de tipos.

Os 3 "preventivos" (9, 35b, 50) **nunca** aparecem como
candidatos — é a sua natureza permanecer abertos.

---

## Comparação com Passo 105

| Métrica | Passo 105 | Passo 125 |
|---------|----------|----------|
| DEBTs abertos | 11 + 45 + 49 + 51 = 14 | 11 |
| Fechados desde | — | 3 (45, 49, 51) |
| Classificados M neste passo | 14/14 | 11/11 |
| Fechos triviais neste passo | 0 | 0 |

Taxa de fecho desde 105: **3 de 14 em 25 passos** (DEBT-45 pelo
110; DEBT-49 pelo 107; DEBT-51 pelo 106). Auditoria periódica
paga-se: ADRs novas ≈ fechos possíveis, mesmo sem fecho explícito
no próprio passo.

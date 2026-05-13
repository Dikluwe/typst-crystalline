# Relatório do passo P213 — Recálculo Introspection pós-M9c

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-213.md`.
**Tipo**: recálculo de cobertura + actualização de documentos
canónicos (administrativo XS documental).
**Magnitude planeada**: XS (~30min). **Magnitude real**: XS (~40min).
**Marco**: nenhum (passo pós-M9c; primeiro pós-marco).

---

## §1 O que foi feito

Recalculada empíricamente a categoria Introspection (declarada
17% no inventário 148 §A.9 + blueprint §2.1, datada de
2026-04-25) face ao estado factual pós-P212 (M9c ACEITE). 5
reclassificações fixadas: 4 ausente → implementado/implementado⁺
+ 1 ausente → parcial + 1 implementado → implementado⁺.
Distribuição: **`1/0/0/5/0 = 6 (17%)` → `3/2/1/0/0 = 6
(83%)`**. Tabela A linha + footnote ³⁸ adicionados em
`typst-cobertura-vanilla-vs-cristalino.md`; §A.9 reescrita
inline; blueprint §2.1 actualizado; marca §3.0decies adicionada
ao blueprint. Zero código tocado; tests/lint preservados.

---

## §2 Reconta empírica §A.9

| # | Entrada | Antes (2026-04-25) | Pós-P212 | Sub-passo de fecho |
|---|---------|---------------------|----------|---------------------|
| 1 | `counter(key)` | implementado | **implementado⁺** | P176 (counter_final) + P177 (counter_at) + P210B (counter_step) |
| 2 | `state(key, ...)` | ausente | **implementado** | P171 (state + state_update + state_update_with) |
| 3 | `here()` / `locate()` | ausente | **implementado** | P208B (here) + P208C (locate) |
| 4 | `query(...)` | ausente | **implementado⁺** | P175 minimal + P209A-D (Selector 6 variants) |
| 5 | `metadata(value)` | ausente | **implementado** | P169 (M9 sub-passo 1) |
| 6 | `position(target)` | ausente | **parcial** | P204D + P205B/C (`Introspector::position_of` + `SealedPositions`); stdlib expose standalone ainda ausente |

**5 reclassificações totais**: 4 ausente→impl/impl⁺ + 1
implementado→implementado⁺. Pattern análogo a P156B (3
Layout) e P154A (5-7 Model), direcção inversa (declarado é
sub-estimativa face ao empírico).

**Deferreds documentados com gatilho de reabertura**
(per ADR-0076 ACEITE):
- `counter.display(numbering)` here-aware → walk advance.
- `state.get()` here-aware → idem.
- `position()` standalone stdlib → Bloco B candidato.

---

## §3 Tabela A inventário 148 actualizada

**Antes (pré-P213)**:
```
Introspection | 1 | 0 | 0 | 5 | 0 | 6 |
```
(1 implementado, 0 implementado⁺, 0 parcial, 5 ausente, 0 scope-out — 17% impl puro)

**Depois (pós-P213)**:
```
Introspection ³⁸ | 3 | 2 | 1 | 0 | 0 | 6 |
```
(3 implementado, 2 implementado⁺, 1 parcial, 0 ausente, 0
scope-out — **83% impl puro / 100% impl + parcial**)

**Total user-facing**:
- Antes: `64/22/24/29/2 = 141` (~61% impl + impl⁺)
- Depois: `66/24/25/24/2 = 141` (~63-64% impl + impl⁺)

**Δ user-facing total**: +2.8pp via peso categoria
Introspection ~4.3% × Δ +66pp categoria = +2.8pp.

§A.9 entry-by-entry inline também reescrita com sub-passos
de fecho + deferreds explícitos.

Footnote ³⁸ adicionada com detalhe completo (cumpre pattern
footnotes ¹ a ³⁷).

---

## §4 Blueprint §2.1 + marca §3.0decies

**§2.1 linha**:

Antes: `| Introspection | 17% | gap maior |`

Depois: `| Introspection ⁽ᴾ²¹³⁾ | 83% | quase total (paridade arquitectural pós-M9c) |`

**Marca §3.0decies adicionada** (Opção γ fixada em C3):
"Recálculo de categoria pós-fecho de marco M9c"
qualitativamente distinta de:
- Marcas-por-série (§3.0quater P207E a §3.0octies P211A).
- Marca-por-marco (§3.0nonies P212).

Conteúdo §3.0decies:
- 5 reclassificações empíricas com sub-passos de fecho.
- Distribuição `1/0/0/5/0 → 3/2/1/0/0`.
- Cobertura categoria 17% → 83%; user-facing total ~61% →
  ~63-64%.
- Causa: política "reescrita ampla fora-de-escopo" preservou
  histórico mas produziu pendência cumulativa.
- Pattern emergente novo: "diagnóstico-recálculo pós-marcos"
  N=1.
- Subpadrão "passo administrativo XS" N=3 (atinge limiar
  formalização).
- Política "sem novas reservas" preservada.
- Bloco A (`measure`) + Bloco B (`position()` stdlib +
  `query_count_before`) candidatos identificados, NÃO
  reservados.

---

## §5 Decisões substantivas

- **Opção γ marca §3.0decies** (vs Opção α "marca paridade"
  vs β "section nova"): híbrida — preserva pattern
  marca-por-fecho mas distingue na semântica via nota
  explícita "recálculo de categoria pós-fecho de marco".
  Seriado 9ª marca cumulativa (§3.0quater + §3.0quinquies +
  §3.0sexies + §3.0septies + §3.0octies + §3.0nonies +
  §3.0decies = 7 cirúrgicas pós-§3.0bis/ter = 9 total).
- **Política "sem novas reservas" preservada** per P158:
  candidatos Bloco A/B identificados em §A.9 footnote ³⁸ +
  blueprint §3.0decies + relatório §8 mas **não reservados**.
  Slot reservas pré-existentes (e.g. 0063 column flow)
  mantém-se documentadas mas não reforçadas.
- **§A.9 estricto vs estendido**: P213 actualiza apenas
  §A.9 estricto (6 entradas). §A.9 estendido (15 entradas
  com `position`/`measure`/`outline.entry`/etc.) NÃO
  materializado em P213 — opção futura se humano julgar
  útil para granularidade adicional.
- **Footnote ³⁸ paridade metodológica** com footnotes ¹ a
  ³⁷: per-entrada detalhe + sub-passo de fecho + deferreds
  com gatilho. Não inflada — minimal mas completa.
- **Tabela A Total user-facing** recalculada via aritmética
  honesta: 5 entradas movidas (4 ausente → impl/impl⁺ + 1
  implementado → implementado⁺ + 1 ausente → parcial); total
  preservado em 141 entradas.
- **Subpadrão "passo administrativo XS" N=3 não promove ADR
  meta**: per spec §5 risco 2, promoção fica para passo
  dedicado se N=4+ ocorrer (e.g. P213-série
  Visualize/Text/Foundations).

---

## §6 Métricas pós-P213

**Estado pós-P213** (sem mudança código):

| Métrica | Pré-M9c | Pós-M9c (P212) | Pós-P213 (Δ doc) |
|---------|---------|-----------------|--------------------|
| Trait `Introspector` métodos | 20 | 26 | 26 |
| `Selector` enum variants | 1 | 6 | 6 |
| Sub-stores L1 | 23 | 25 | 25 |
| Stdlib funcs registadas | ~50 | ~53 | ~53 |
| Allowlist L1 deps externas | 11 | 12 | 12 |
| Tests workspace | 1873 | 1939 | 1939 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| ADRs ACEITES M9c | — | 2 (0076, 0077) | 2 |
| **Categoria Introspection §A.9** | (17% pré) | (17% — não recalculada) | **83%** |
| **Cobertura user-facing total** | (~58% pré) | (~61% — não recalculada) | **~63-64%** |
| Blueprint marcas cirúrgicas | 3 | 9 (§3.0/bis/ter/quater/quinquies/sexies/septies/octies/nonies) | **10** (+§3.0decies) |
| Footnotes inventário 148 | 37 | 37 | **38** (+³⁸ P213) |

---

## §7 Pattern emergente "diagnóstico-recálculo pós-marcos"

**Pattern formalizado N=1 em P213**:

P213 é a primeira aplicação de **diagnóstico-recálculo
pós-fecho de marco**. Distinto de:

- **Recontas pre-materialização** (P154A Model, P156B Layout,
  P157, P158, P159 vários): precederam materialização;
  ajustaram estimativas antes de implementar.
- **Encerramentos de série** (P207E, P208D, P209E, P210C,
  P211A): pós-materialização incremental por série; sem
  recálculo cumulativo.
- **Encerramento de marco** (P212 — ACEITE M9c): pós-marco
  inteiro; sem recálculo de categorias documentais.

**P213** introduz o pattern: **reconta cirúrgica pós-fecho
de marco** quando política "reescrita ampla fora-de-escopo"
acumulou pendência documental.

**Subpadrão consolidado "passo administrativo XS"** N=3:

1. `ADR-0062-create` — administrativo XS criação ADR.
2. P160A — administrativo XS reorganização.
3. **P213** — administrativo XS recálculo cobertura.

Atinge **limiar formalização N=3-4**. **Promoção a ADR meta
diferida** para passo dedicado se N=4+ ocorrer (e.g. recontas
paralelas Visualize 54% / Text features 52% / Foundations
67% — todas pré-M9c não recalculadas).

---

## §8 Próximo passo (fora P213)

P213 é diagnóstico-recálculo. Identifica candidatos mas **não
compromete** trabalho subsequente. Decisão humana sobre
próximo passo fica em aberto entre 4 opções:

| Opção | Trabalho | Magnitude |
|-------|----------|-----------|
| **Bloco A** | Fechar §A.9 estricto 83% → 100% via `measure(body)` stdlib expose | S+ |
| **Bloco B** | Avançar §A.9 estendido 53% → ~67% via `position()` stdlib + `query_count_before` | S+ cada |
| **Outra categoria** | Replicar método P213 em Visualize 54% / Text features 52% / Foundations 67% (pré-M9c não recalculadas) — provável Δ similar | XS-S por categoria (paralelo a P213) |
| **Outro módulo** | Layout Fase 3 columns/colbreak DEBT-56; Model bibliography hayagriva DEBT-55 | M+ |

**Política "sem novas reservas" preservada per P158**:
candidatos identificados acima são **opções**, não compromissos.
Reservas pré-existentes mantêm-se documentadas mas não
reforçadas em P213.

**Promoção subpadrão "passo administrativo XS" a ADR meta**:
diferida para passo dedicado se N=4+ ocorrer (e.g. recálculo
Visualize/Text como P214/P215 série).

**Estado final pós-P213**:
- Marco M9c: ✅ ACEITE 2026-05-12 (preservado).
- ADRs M9c: 2 ACEITES (0076 + 0077; preservadas).
- ADR-0073: ACEITE com fecho retroactivo M9c (preservada).
- Categoria Introspection §A.9: **83%** (recalculada em P213).
- Cobertura user-facing total: **~63-64%** (recalculada em
  P213).
- Tests workspace: **1939 verdes**; `crystalline-lint`: **0
  violations** (preservados).
- Trajectória aberta: 4 opções para próxima sessão; decisão
  humana.

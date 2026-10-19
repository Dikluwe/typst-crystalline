# Passo 213 — Recálculo Introspection + actualização inventário 148

**Série**: 213 (passo único — administrativo XS
documental).
**Marco**: nenhum (passo pós-M9c; primeiro pós-marco).
**Tipo**: recálculo de cobertura + actualização de
documentos canónicos.
**Magnitude**: XS (~30min documental).
**Pré-condição**: P212 concluído (M9c ACEITE);
ADR-0076 ACEITE 2026-05-12; trait 26 métodos; Selector
6 variants; stdlib funcs ~53; 10 sub-stores; tests 1939
verdes; 0 violations; inventário 148 §A.9 desactualizado
desde 2026-04-25 (declarado 17% pré-M9c); blueprint
§2.1 desactualizado (mesma fonte 17%).
**Output**: 1 ficheiro (relatório curto recálculo) + 2
ficheiros canónicos editados.

---

## §1 Trabalho

Recalcular empíricamente cobertura Introspection
(declarada 17% no inventário 148 §A.9 + blueprint §2.1)
face ao estado factual pós-P212. Actualizar Tabela A do
inventário 148 + quadro §2.1 do blueprint sem tocar em
código. Paridade metodológica com P156B (recálculo
Layout 38% → 22%) e P154A (recálculo Model 38% →
32-36%), mas com direcção inversa — declarado é
sub-estimativa face ao empírico, não sobre-estimativa.

**Decisão central de P213**: aplicar método P148/P156B
de reconta empírica das entradas §A.9 face ao estado
factual pós-M3 → M9c. Reclassificar onde o estado mudou.
Actualizar documentos canónicos. Sem ADR nova; sem
reservas novas (política preservada de P158).

Reuso de dados pós-M9c (sem recolha nova):

- 5 séries M9c fechadas (P207-P211) + P212.
- 13 ADRs ACEITES directamente relacionadas com
  Introspection (0033, 0054, 0066, 0067, 0068, 0069,
  0070, 0071, 0072, 0073, 0074, 0075, 0076, 0077).
- Trait `Introspector` 20 → 26 métodos.
- `Selector` 1 → 6 variants.
- 3 stdlib funcs novas (`here`, `locate`, `counter_step`).
- 10 sub-stores em `TagIntrospector`.
- 11 deferreds documentados com gatilho de reabertura.

---

## §2 Cláusulas (5)

### C1 — Reconta empírica §A.9 (6 entradas)

§A.9 do inventário 148 declara 6 entradas observable
user-facing. C1 reconta cada uma face ao estado
factual pós-P212:

| # | Entrada | Declarado 2026-04-25 | Esperado pós-P212 | Sub-passo de fecho |
|---|---------|----------------------|--------------------|---------------------|
| 1 | `counter` | implementado | implementado⁺ | P170 hierarchical + P210B step + P177 at |
| 2 | `measure` | parcial | parcial | (inalterado — sem stdlib expose) |
| 3 | `here` | ausente | implementado | P208B |
| 4 | `locate` | ausente | implementado | P208C |
| 5 | `query` | ausente | implementado⁺ | P175 + Selector 6 variants P209 |
| 6 | `state` | ausente | implementado | P171 (M9) |

Hipótese provável: **5 reclassificações** — 4 ausente
→ implementado/implementado⁺ + 1 implementado →
implementado⁺. Padrão análogo a P154A (5-7
reclassificações Model) e P156B (3 reclassificações
Layout).

Cobertura empírica esperada:
- Implementado puro (impl + impl⁺) = 5/6 = **83%**.
- Implementado + parcial = 6/6 = **100%**.

Se reconta produzir distribuição diferente: registar
`P213.div-N` e ajustar antes de C2.

### C2 — Actualização Tabela A inventário 148

Editar
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**Tabela A linha Introspection**:

Antes (pré-P213):
```
Introspection | 1/0/1/4/0 = 6 | 17%
```

Depois (pós-P213):
```
Introspection | 3/2/1/0/0 = 6 | 83%
```

Distribuição pós-recálculo: 3 implementado (`here`,
`locate`, `state`); 2 implementado⁺ (`counter`,
`query`); 1 parcial (`measure`); 0 ausente; 0 scope-out.

**Cobertura user-facing total**: recálculo por
ponderação. Introspection tem peso ~4.3% das ~138
entradas user-facing. Δcategoria +66pp × peso 4.3% ≈
+2.8pp.

- Pré-P213: ~61% user-facing total (pós-P159G).
- Pós-P213: ~**63-64%**.

**§A.9 estendido** (15 entradas — entradas adicionais
não listadas em §A.9 original): documentar em nota
adicional após Tabela A. Não substitui §A.9 estricto
(paridade metodológica com Math 92% / Foundations 67%
/ etc.) mas dá granularidade para auditoria pós-M9c.
Distribuição esperada §A.9 estendido: 8/0/2/5/0 = 53%
implementado puro; 67% incluindo parcial.

### C3 — Actualização blueprint §2.1 + marca §3.0decies

Editar
`00_nucleo/diagnosticos/blueprint-projecto.md`:

**§2.1 linha Introspection**:

Antes:
```
| Introspection | 17% | gap maior |
```

Depois:
```
| Introspection | 83% | quase total (paridade arquitectural pós-M9c) |
```

**Decisão sobre forma da marca cirúrgica**:

- **Opção α — §3.0decies marca paridade encerramentos
  série** (§3.0quater P207E → §3.0nonies P212):
  consistência com pattern marca-por-fecho.
- **Opção β — Section nova distinguindo "recálculo
  pós-marco" de "fecho de série/marco"**: mais
  estruturante mas inflação possível.
- **Opção γ — §3.0decies marca com nota explícita
  "recálculo de categoria pós-fecho de marco"**:
  híbrida. Distingue na semântica sem criar section
  nova.

Critério: continuidade do pattern emergente
marca-por-fecho (8 marcas cumulativas pré-P213:
§3.0quater P207E + §3.0quinquies P208D + §3.0sexies
P209E + §3.0septies P210C + §3.0octies P211A +
§3.0nonies P212 — P211 sem marca per Caminho 1 puro
fixado em P211A; correcção: 5 marcas + P212 = 6
marcas cumulativas).

Hipótese provável: **Opção γ** — §3.0decies marca com
nota "recálculo de categoria pós-fecho de marco
M9c (não fecho de série/marco)".

Conteúdo da marca:
- Recálculo §A.9 17% → 83% (5 reclassificações).
- Cobertura user-facing total ~61% → ~63-64%.
- Distribuição pós-recálculo `3/2/1/0/0`.
- Causa: política "reescrita ampla fora-de-escopo"
  preservou histórico mas produziu pendência
  cumulativa.
- Pattern emergente novo "diagnóstico-recálculo
  pós-marcos" N=1.
- Subpadrão "passo administrativo XS" cresce N=2→3
  (`ADR-0062-create` + P160A + **P213**) atinge
  limiar formalização.
- Política "sem novas reservas" preservada — candidatos
  Bloco A/B/C identificados (medir/position/etc) NÃO
  reservados.

### C4 — Verificação final

```
grep -n "Introspection" 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md
grep -n "P213" 00_nucleo/diagnosticos/blueprint-projecto.md
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
```

Critério:
- Tabela A linha Introspection mostra `3/2/1/0/0` e `83%`.
- Blueprint §3.0decies existe entre §3.0nonies e §3.1.
- Tests 1939 verdes (Δ 0 — sem código tocado).
- 0 violations.

### C5 — Decisão sobre próximo trabalho

P213 é diagnóstico-recálculo. Identifica candidatos mas
**não compromete** trabalho subsequente. Decisão humana
sobre próximo passo fica em aberto entre:

- **Bloco A** (Introspection — fechar §A.9 estricto
  83% → 100% via `measure` stdlib expose; S+).
- **Bloco B** (Introspection — avançar §A.9 estendido
  53% → ~67% via `position()` stdlib + `query_count_
  before`; S+ cada).
- **Outra categoria** (replicar método P213 em
  Visualize 54% / Text features 52% / Foundations
  stdlib 67% ainda não recalculadas).
- **Outro módulo** (Layout Fase 3 columns/colbreak
  DEBT-56; Model bibliography hayagriva DEBT-55).

C5 documenta opções em §8 do relatório; não fixa
nenhuma.

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-213-relatorio.md`.

Estrutura (~5-7 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Reconta empírica §A.9 (tabela compacta — 6 entradas
  com declarado/empírico/sub-passo de fecho).
- §3 Tabela A inventário 148 actualizada (antes/depois
  + cobertura user-facing total recalculada).
- §4 Blueprint §2.1 + marca §3.0decies (forma fixada
  em C3).
- §5 Decisões substantivas (incluindo política "sem
  novas reservas" preservada).
- §6 Métricas: trait 26 métodos / Selector 6 / stdlib
  funcs +3 / sub-stores 10 / 11 deferreds (Δ M9c
  cumulativo).
- §7 Pattern emergente "diagnóstico-recálculo
  pós-marcos" N=1 + subpadrão "passo administrativo
  XS" N=3.
- §8 Próximo passo (fora P213; opções A/B/outras para
  decisão humana).

---

## §4 Não-objectivos

- Materializar `measure` stdlib expose (Bloco A) —
  decisão humana posterior; passo dedicado se aceite.
- Materializar `position()` stdlib ou `query_count_
  before` (Bloco B) — idem.
- Recalcular outras categorias (Visualize / Text
  features / Foundations / Markup) — passos dedicados
  futuros se método P213 for replicado.
- Promover ADR-0076 ACEITE → IMPLEMENTADO — ADR já
  ACEITE em P212; transição IMPLEMENTADO exigiria
  reauditoria não-prevista.
- Reabrir deferreds M9c (`Content::Context`, walk
  advance, `native_regex`, etc.) — todos com gatilho
  de reabertura documentado em P212.
- Re-escrita blueprint ampla — preserva pattern
  marca-por-fecho de P204H/P205E/P206E/P207E/P208D/
  P209E/P210C/P211A/P212.
- Tocar em código `.rs` — passo documental puro.
- Tocar em hashes L0 — nenhum prompt L0 alterado.
- Abrir/fechar DEBTs — nenhum DEBT tocado.

---

## §5 Riscos a evitar

1. **Recálculo "óptico" sem verificação empírica**: C1
   exige confirmar cada reclassificação contra
   sub-passo de fecho real (não assumpção). Pattern
   paralelo a P156B que verificou cada entrada Layout
   empíricamente.
2. **Inflar para "ADR meta" prematuro**: subpadrão
   "passo administrativo XS" N=3 atinge limiar
   formalização N=3-4 mas P213 NÃO promove a ADR meta.
   Promoção fica para passo dedicado se N=4+ ocorrer
   (e.g. P213-série Visualize/Text).
3. **Confundir §A.9 estricto vs estendido**: 17%
   declarado refere-se a §A.9 estricto (6 entradas).
   83% pós-P213 também §A.9 estricto. §A.9 estendido
   (15 entradas) é métrica complementar, não
   substitutiva.
4. **Recálculo "user-facing total" sem ponderação
   honesta**: Introspection tem peso ~4.3% do total.
   Δ +2.8pp no total é resultado correcto, não
   +66pp.
5. **Política "sem novas reservas" violada**: P213
   identifica candidatos Bloco A/B/C mas não os
   reserva. Reservas pré-existentes (slot 0063 column
   flow) mantêm-se documentadas mas não reforçadas.
6. **Marca blueprint §3.0decies inflada**: preserva
   pattern minimal de marca-por-fecho. Não reescrever
   §3.1 datado 2026-04-25.
7. **Esquecer paridade metodológica P148/P156B**: P213
   é continuação de método estabelecido, não novo
   método. Estrutura do recálculo replica P156B
   (reconta + reclassificações + adicionar entradas
   omitidas + recálculo formal).

---

## §6 Hipótese provável

C1 confirmará 5 reclassificações esperadas (4 ausente
→ implementado/implementado⁺; 1 implementado →
implementado⁺) com distribuição empírica `3/2/1/0/0`
= 83%.

C2 actualizará Tabela A; cobertura user-facing total
sobe de ~61% para ~63-64%.

C3 fixará Opção γ — §3.0decies marca com nota
"recálculo de categoria pós-fecho de marco M9c".

C4 reportará tests 1939 / 0 violations preservados.

C5 listará 4 opções de próximo trabalho sem fixar.

Custo real: XS (~30-45min documental). Sem código
tocado.

Mas é hipótese, não decisão. C1-C5 fixam-se
empíricamente.

---

## §7 Particularidade P213

P213 é estruturalmente único na trajectória pós-M9c:

- **Primeiro passo pós-M9c** — abre trajectória nova.
  Pré-condições M9c ACEITE estáveis; P213 não toca em
  marcos anteriores.
- **Primeiro recálculo pós-fecho de marco** — pattern
  emergente novo distinguindo de P154A/P156B/P157/
  P158/P159/P160 (todos precederam materialização).
  P213 reconta depois de série completa de marcos
  fechados sem actualização intermédia dos quadros
  de cobertura.
- **Terceiro passo administrativo XS** (subpadrão
  N=3): `ADR-0062-create` + P160A + **P213**. Atinge
  limiar de formalização N=3-4. Promoção a ADR meta
  diferida.
- **Sem reservas novas** — política P158 preservada.
  Candidatos Bloco A/B/C identificados em §8 do
  relatório mas não comprometidos.

Por isso §5 risco 5 ("nova reserva acidental") é
relevante — P213 é diagnóstico, não compromisso de
trabalho subsequente.

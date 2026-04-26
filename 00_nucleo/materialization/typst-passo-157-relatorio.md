# Relatório P157 — Diagnóstico Model Fase 2 (table foundations)

Passo arquitectural de diagnóstico precedendo materialização.
**Não materializa código**. **Primeira aplicação concreta de
ADR-0065 critério #5** (scope determinado por inventário) —
auto-validação cumulativa do ADR meta P156K (P156L já validou
critério #3; P157 valida critério #5). **Décima primeira
aplicação consecutiva** do padrão diagnóstico-primeiro
(décima sob critério estendido de ADR-0065).

---

## 1. Resumo do diagnóstico (síntese das 5 secções)

### 1.1 §1 — ADR-0060 lida e resumida

ADR-0060 status: **`IMPLEMENTADO`** (Fase 1 fechada em P155 —
terms + divider em P154B; quote em P155). Fase 2 e Fase 3
prosseguem como roadmap planeado sem necessidade de re-abertura.

**Subset declarado para P157** (literal ADR-0060 §"Decisão 1"
sub-passo 3): `Content::Table` variant nova + sub-elementos
`TableCell`, `TableHeader`, `TableFooter` (M+; reaproveita
`Content::Grid` parcial para layout). Renumerado de P156 para
P157 em P156B.

### 1.2 §2 — Estado factual em código

Pesquisa exaustiva confirmou:
- **`table` factualmente ausente** em código cristalino: zero
  variants `Content::Table`/`TableCell`/etc; zero stdlib funcs
  `native_table*`. "table" aparece apenas como string literal
  `Figure.kind: "table"` para counters.
- **`grid` parcial mas funcional**: `Content::Grid` com
  `columns/rows/cells: Vec<Content>` (sem TableCell estruturado);
  layout em `01_core/src/rules/layout/grid.rs` (272 linhas) —
  algoritmo TrackSizing completo; cells distribuídas via
  `idx % num_cols`; 9 atributos vanilla scope-out (DEBT-34d/e).
- **`Figure.kind: "table"` slot** já existe — preparação directa
  para P158 figure-table.

### 1.3 §3 — Scope determinado por inventário

Subset máximo M+ (4 variants + 4 stdlib + ~25-30 sítios
pattern-match + ~30+ tests num único passo) **rejeitado** por
violar granularidade N=9.

**Recomendação adoptada**: dividir em **3 sub-passos M cada**
preservando granularidade:
- **P157A** — `Content::Table` minimal (1 feature; M;
  granularidade preservada N=10; tests ~10-15).
- **P157B** — `Content::TableCell` + colspan/rowspan armazenados
  per ADR-0054 graded (1 feature; M; N=11; tests ~12-18).
- **P157C** — `Content::TableHeader` + `Content::TableFooter`
  (par simétrico paridade vanilla; S+/M; N=12; tests ~10-15).

Soma cumulativa: ~32-48 tests; cobertura Model ~45% → ~64%
pós-P157C.

### 1.4 §4 — Dependências bloqueantes

**Zero bloqueios hard** para P157A. Limitações scope-out per
ADR-0054 graded:
- **DEBT-34e** (colspan/rowspan em Grid) limita P157B —
  fields armazenados mas ignorados.
- **DEBT-56** (column flow multi-region) limita P157C `repeat`
  real — `repeat: bool` armazenado mas ignorado.
- **DEBT-55** (Bibliography + Cite) **não aplicável** — fora de
  scope P157.

ADRs em vigor relevantes: 0017, 0026/-R1, 0033, 0034, 0054,
0060, **0064 (P156K)**, **0065 (P156K)**.

### 1.5 §5 — Esboço de P157A

**Identificador**: P157A (sufixo letra após número base; segue
precedente da série Layout P156C-L; estabelece simetria
reconhecível ADR-0061 ↔ ADR-0060).

**Tamanho**: M.

**Subset**: `Content::Table { columns: Vec<TrackSizing>, rows:
Vec<TrackSizing>, children: Vec<Content> }` — sem TableCell
estruturado; cells passam Content directo; layouter delega a
`layout_grid` clone simples.

**Stdlib**: `native_table` em `stdlib/model.rs` (módulo novo) ou
`stdlib/layout.rs` — **decisão arquitectural deferida ao
inventário .1 de P157A**.

**Granularidade preservada**: N=10. Padrão #1 cresce.

**Padrões aplicáveis**: ADR-0064 Caso A (TableCell.x/y futuro
P157B); Caso D (TableHeader.repeat futuro P157C); ADR-0065
critério #5 (scope determinado em §3 deste diagnóstico —
auto-aplicação activa).

**Risco estimado**: baixo-médio. Reusos significativos (Grid
layout + extract_tracks); inventário .1 cobre divergências
antes de execução.

---

## 2. Decisão final de scope para P157A

**Subset MÍNIMO recomendado** (P157A isoladamente):

```rust
Content::Table {
    columns:  Vec<TrackSizing>,
    rows:     Vec<TrackSizing>,
    children: Vec<Content>,
}
```

**Características**:
- 1 variant Content nova.
- 1 stdlib func `native_table`.
- ~12 sítios pattern-match estruturais (paridade P156I Stack).
- ~10-15 tests novos.
- Reusa `layout_grid` directamente.
- Sem TableCell/Header/Footer (futuros P157B/C).
- Granularidade N=10 preservada.

**P157B + P157C ficam como passos seguintes**, cada um com
diagnóstico próprio per ADR-0065. Sequência completa P157A/B/C
materializa "table foundations" declarado em ADR-0060 sem
quebra de granularidade, em paralelo conceptual com a série
Layout P156C-J/L.

**Subset rejeitado** (alternativa M+ num único passo):
- 4 variants + 4 stdlib + ~30 tests num passo único.
- Quebra granularidade N=9 sem benefício compensatório.
- Risco médio-alto sem mitigação clara.

---

## 3. Dependências identificadas a tratar antes de P157A

**Zero pré-requisitos hard**. P157A pode iniciar sem trabalho
prévio.

Notas operacionais:
- **`Content::Grid` parcial é suficiente**: ADR-0060 declara
  reaproveitamento e P157A confirma factualidade — algoritmo
  `layout_grid` aceita `children: Vec<Content>` como cells
  directamente (passa `&children` em vez de `&cells`).
- **`extract_tracks` helper** em `stdlib/layout.rs` é reusável
  para parsing de `columns`/`rows` de `native_table` — N=2 do
  helper (subpadrão análogo a `extract_length` N=7).
- **DEBT-34d/e** ficam **abertos** após P157A (não consumidos);
  fechamento de DEBT-34e fica condicionado a P157B com decisão
  posterior sobre algoritmo de placement.

---

## 4. Análise de risco (padrão N=6 → 7; passo diagnóstico)

P157 é **passo diagnóstico** sem alteração de código. **Sétima
aplicação consecutiva** de §análise de risco (P156F/G/H/I/J/K/L/+P157)
preservando precedente.

### 4.1 Riscos identificados

| Risco | Avaliação | Mitigação aplicada |
|-------|-----------|---------------------|
| ADR-0060 conter scope diferente do reservado | Baixo | Inventário §1 confirmou literal: P157 = table foundations (renumerado de P156 em P156B); status `IMPLEMENTADO` em P155 não bloqueia Fase 2 |
| Subset `Content::Table` ser ambíguo entre estilo de `Grid` (alias) e variant dedicada | Médio | §3 documenta decisão arquitectural: variant dedicado per ADR-0060 Decisão 4 (variant novo, não Styled); reaproveitamento de Grid é apenas no algoritmo de layout, não na estrutura do enum |
| Granularidade N=9 ser quebrada por subset M+ declarado | Baixo, neutralizado | §3 propõe divisão em 3 sub-passos M cada (P157A/B/C) preservando N=10/11/12; subset M+ rejeitado |
| Inventário superficial não detectar dependência crítica | Baixo | §4 cobertura sistemática: Layout/Introspection/DEBTs/ADRs em vigor + pendentes |
| Estimativa de tests Δ ser desafiada em P157A | Baixo | Range ~10-15 baseado em P156I Stack (similar complexidade); estimativa ajustável no diagnóstico .1 de P157A |

### 4.2 Riscos não-aplicáveis

- **Refactor de código**: zero (passo puramente documental).
- **Quebra de contrato API**: zero (sem código alterado).
- **Drift de hashes L0/L1**: zero (sem código → sem hash a
  propagar).
- **Quebra de paridade observável**: zero (sem alteração de
  comportamento).

### 4.3 Conclusão de risco

**Risco residual: muito baixo.** Padrão "passo diagnóstico
documental + scope determinado por inventário (ADR-0065 #5) +
divisão em sub-passos preservando granularidade" replica
tratamento bem-sucedido de P156B (diagnóstico Layout) e P156K
(ADRs meta).

**Auto-validação ADR-0065 critério #5**: este passo exemplifica
"scope determinado por inventário" — a decisão M+ vs 3xM é
informada por estado factual, não inferida da spec original
(que sugeria M+ sem detalhar). Padrão consolida-se com auto-
aplicação cumulativa: P156L validou critério #3; P157 valida
critério #5; restantes critérios (#1/#2/#4/#6) ainda sem
aplicação concreta isolada — candidatos futuros conforme
necessidade.

---

## 5. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | Diagnóstico produzido com 5 secções | **✓** `diagnostico-model-fase-2-passo-157.md` (5 secções §1-§5) |
| 2 | ADR-0060 lida e resumida em §1 | **✓** status `IMPLEMENTADO` confirmado; definição de fases citada literalmente |
| 3 | Estado de `grid` em crystalline determinado factualmente | **✓** `parcial` (272 linhas em `layout/grid.rs`); 9 atributos vanilla scope-out |
| 4 | Estado de `table` em crystalline determinado factualmente | **✓** **ausente** (zero matches em pesquisa exaustiva grep) |
| 5 | Subset concreto definido em §3 com recomendação para P157A | **✓** subset mínimo P157A (variant `Content::Table` minimal); subset máximo M+ rejeitado por violar granularidade |
| 6 | Dependências bloqueantes listadas em §4 | **✓** DEBT-34d/e/56 + ADRs em vigor + pendentes; zero bloqueios hard para P157A |
| 7 | Esboço de P157A em §5 | **✓** identificador, tamanho M, subset, sub-passos, granularidade preservada N=10 |
| 8 | ADR-0061 §"Aplicações cumulativas" actualizada com linha P157 | **✓** linha P157 com slope "—"; padrões N actualizados (granularidade inalterada N=9; inventariar primeiro N=6 → 7; §análise risco N=6 → 7) |
| 9 | `crystalline-lint`: zero violations | **✓ No violations found** |
| 10 | Sem alteração de hashes (passo documental) | **✓** zero código modificado; hash `entities/content.rs` mantém-se `ec58d849` |

---

## 6. Confirmação: ADR-0065 critério #5 aplicado pela primeira vez

P157 é **primeira aplicação concreta** de ADR-0065 critério #5
(scope: atributos a incluir/diferir per ADR-0054 graded).

Auto-validação cumulativa do ADR meta P156K:
- **ADR-0064**: Caso A/B/C/D já validados em P156D-J (formalização
  em P156K); Caso D ganha patamar adicional (TableHeader/Footer.repeat
  default true) implícito em §3.2 — N=4 → ~5 implícito.
- **ADR-0065**:
  - Critério #1 (naming) — sem aplicação isolada concreta ainda.
  - Critério #2 (escolha de tipo) — implícito em série P156C-J/L.
  - Critério #3 (expansão de variant) — primeira aplicação
    concreta em **P156L** (refactor `Pad`).
  - Critério #4 (atravessamento de camadas) — implícito.
  - Critério #5 (scope) — **primeira aplicação concreta em P157**
    (este passo).
  - Critério #6 (divergência da spec) — **primeira aplicação
    concreta em P156L** (divergência factual de cobertura
    declarada na spec); reforçada em P157 com decisão de
    divisão M+ → 3xM.

**Padrão emergente**: cada passo da série P156-P157 valida
empiricamente um critério distinto de ADR-0065. Em ~5 passos
adicionais, todos os 6 critérios terão aplicação concreta
isolada — a auto-validação do ADR meta torna-se completa
empiricamente.

---

## 7. Estado pós-P157

- **Cobertura Layout**: **78%** (inalterada — escopo Model).
- **Cobertura Model**: ~45% pós-P155 (inalterada — P157 é
  diagnóstico).
- **Variants Content**: **52** (inalterada).
- **Stdlib funcs**: **42** (inalterada).
- **Tests**: **1319** (inalterada — sem código).
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados.
- **ADR-0061** §"Aplicações cumulativas": tabela slope ganha
  linha P157 (slope "—"); padrões N actualizados.
- **README ADRs**: entrada P157 adicionada antes de P156L
  (preserva ordem cronológica reversa).
- **Reservas P158/P159/ADR-0062**: inalteradas.
- **Hash `content.rs`**: `ec58d849` (preservado — passo
  documental).
- **Total ADRs**: **63** (inalterado).

### 7.1 Próxima decisão (per spec do passo)

**P157A** redigido como spec separada com base no diagnóstico
§5. Ponto de validação humana antes de redigir spec.

Se P157A for aprovado:
- Materializa `Content::Table` minimal.
- Cadência granular preservada (N=10).
- Inicia série granular Model Fase 2 análoga a Layout
  P156C-J/L.

Se P157A for redirigido:
- Diagnóstico §3.6 documenta alternativas (subset M+ no único
  passo; iniciar com P157C Header/Footer; etc.).

Outras direcções pendentes (per relatório P156L §7):
- Continuar Fase 3 Layout (columns/colbreak).
- Footnote area.
- Promover ADR-0061 a IMPLEMENTADO.
- Promover `extract_length` a helper público.
- Atacar Introspection.

---

## 8. Fechamento

P157 fecha como **passo diagnóstico documental** sem alteração
de código. **Auto-validação ADR-0065 critério #5** activa pela
primeira vez — scope de "table foundations" (M+ → 3xM) decidido
informadamente por inventário factual em vez de assumido pela
spec.

**Padrões pós-P157**:
- Granularidade 1-2 features/passo: **N=9 inalterada** (P157
  é diagnóstico, não materialização).
- Inventariar primeiro: **N=6 → 7** (primeiro critério #5;
  P156L já tinha primeiro critério #3).
- §análise de risco no relatório: **N=6 → 7** (passo diagnóstico
  baixo risco).
- Smart→Option/default Caso D: patamar implícito **N=4 → ~5**
  (TableHeader/Footer.repeat default true).
- Reuso `Sides<T>`: N=2 (inalterado).
- Reuso `extract_length`: N=7 (inalterado).

**ADRs meta P156K** continuam a ganhar evidência empírica:
ADR-0064 reforçado por aplicação implícita Caso D; ADR-0065
ganha primeira aplicação concreta de critério #5. Patamar
empírico cresce naturalmente sem nova ADR.

ADR-0060 status `IMPLEMENTADO` mantido; P157A é primeiro sub-
passo Fase 2 Model com diagnóstico estruturado.

**Próxima decisão humana**: validação de §3 (scope) e §5
(esboço P157A) antes de redacção da spec P157A.

**Pausa natural após P157 — diagnóstico estruturado completo;
P157A pronto para spec; padrões consolidados a patamar
crescente. Decisão humana sobre próxima direcção tem máxima
informação acumulada.**

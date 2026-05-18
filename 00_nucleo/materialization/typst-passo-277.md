# typst-passo-277 — DEBT-33 fecho CLOSED (Bézier bbox analítica via extremos B(t))

**Magnitude**: S+M com código (cap L1 ~120 LOC + testes ~60 LOC; cap doc Fase A ~500; relatório ~700).
**Cluster**: Visualize / Geometry / Fecho DEBT real.
**Origem**: relatório P275 §6.1 "DEBT-33 accionável directo S+M; sem bloqueador"; sequência aprovada humana 2026-05-18 — "fechar DEBTs que reforçam o projecto".
**Tipo**: passo principal P277 — fecho **CLOSED** de DEBT material (não OBSOLETED como P276).
**Sequência**: P275 (auditoria) → P276 (DEBT-35b OBSOLETED) → **P277 (DEBT-33 CLOSED)** → P278 (cleanup XS combinado).
**Estratégia decidida**: fecho honesto via materialização do **cálculo analítico dos extremos da curva cúbica de Bézier B(t)** conforme texto literal do DEBT.md. Algoritmo conhecido; sem bloqueador externo; reforça o projecto (corrige vazamento visual em curvas).

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L1 só após prompt L0 + ADR. Ordem: diagnóstico Fase A → confirmação L0 → testes-primeiro → código.

2. **ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-debt-33-passo-277.md` imutável. **31º consumo** do pattern diagnóstico-primeiro (continuação P276 N=35; 30º consumo).

3. **NÃO criar ADRs novas** — algoritmo de cálculo de bbox analítica de Bézier é **refino de implementação existente** (substituir min/max dos pontos de controlo por raízes de B'(t)=0), não decisão arquitectural. Sub-padrão "ADR não necessário para refino algorítmico" preserved.

4. **ADR-0029 pureza física L1** preserved absoluto — algoritmo é matemática pura (raízes de polinómio quadrático); zero dependências externas novas. Sem `crate` externa.

5. **ADR-0033 paridade vanilla** considerada — verificar em Fase A se vanilla calcula bbox analítica de Bézier ou também usa pontos de controlo. Paridade preferível mas não obrigatória (ADR-0054 graded permite divergência se melhorar correcção).

6. **Crystalline-lint zero violations** obrigatório. Hash L0 de `geometry.md` propagado se ficheiro tocado.

7. **Sub-padrão "Fecho CLOSED de DEBT real com material"** — N=1 inaugural P277. Pattern P206E "3 caminhos" (CLOSED / REPLACED-BY / OBSOLETED) reforçado: P276 usou OBSOLETED, P277 usa CLOSED. Padrão equilibrado.

8. **Tests workspace 2644 → ~2650-2660** esperado (~6-10 testes novos: unit raízes de polinómio + bbox de cubic + paridade vs min/max actual + casos limite).

9. **Cap LOC L1 hard 150 / soft 100** (algoritmo é pequeno; ~50-80 LOC core + ~40 LOC integração). **Cap testes hard 80 / soft 60**.

10. **Caps documentais** (ADR-0094 Pattern 1):
    - Diagnóstico Fase A: hard ~500 linhas; soft ~350.
    - Relatório consolidado: hard ~800 linhas; soft ~550.

---

## §1 — Sub-passo P277.A — Fase A diagnóstico empírico

Produz `00_nucleo/diagnosticos/diagnostico-debt-33-passo-277.md`.

### §A.1 — Localização empírica de `ShapeKind::Path` e `CubicTo`

**Pergunta central**: o L0 actual `00_nucleo/prompts/entities/geometry.md` lista `ShapeKind` apenas como `Rect / Ellipse / Line`. Mas DEBT.md menciona `ShapeKind::Path` e `CubicTo`. Resolver discrepância antes de qualquer código.

```bash
# 1. ShapeKind::Path existe no código?
rg "ShapeKind::Path|Path \{|Path\(" 01_core/src/entities/geometry.rs -n
rg "ShapeKind" 01_core/src/entities/ -n

# 2. CubicTo existe?
rg "CubicTo|MoveTo|LineTo|QuadTo" 01_core/src/ -n

# 3. PathOp ou similar?
rg "pub enum.*Path|pub struct.*Path" 01_core/src/entities/ -n

# 4. Stdlib native_path ou similar?
rg "native_path|fn path\(|\"path\"" 01_core/src/rules/stdlib/ -n

# 5. Layouter consume Path?
rg "ShapeKind::Path|Content::Path" 01_core/src/rules/layout/ -n

# 6. Exportador emite Path?
rg "ShapeKind::Path|m l c.*PDF" 03_infra/src/export.rs -n

# 7. Histórico git — passo 79 alterou geometry.rs?
git log --oneline -G "Path|CubicTo" -- 01_core/src/entities/geometry.rs | head -10
```

**Output esperado §A.1** (3 cenários possíveis):

| Cenário | Descrição | Implicação |
|---|---|---|
| **C1** — Path existe e contém CubicTo | DEBT-33 é factual; L0 `geometry.md` está desactualizado | Avançar com algoritmo; em §C tocar L0 para reflectir Path |
| **C2** — Path existe mas sem CubicTo (só LineTo) | DEBT-33 é **prematuro** — não há curvas | Gate §A.6 dispara; fecho como OBSOLETED por irrelevância (sem curvas para optimizar) |
| **C3** — Path **não** existe no código | DEBT-33 é **especulativo** — DEBT preventivo sobre tipo nunca materializado | Gate §A.6 dispara; fecho como OBSOLETED (análogo a DEBT-35b P276) |

**A spec assume C1 como cenário principal** (texto literal DEBT.md fala em "actualmente é calculado verificando o min/max dos pontos de controlo" — implica que o cálculo existe). Cenários C2/C3 forçam paragem e reformulação.

### §A.2 — Inventário da função actual de bbox

Confirmado C1, localizar função actual:

```bash
# Função que calcula bbox de Path actualmente
rg "fn.*bbox|fn.*bounding|fn.*aabb" 01_core/src/entities/geometry.rs -n
rg "fn.*bbox|fn.*bounding" 01_core/src/rules/layout/ -n

# Texto literal DEBT.md menciona "min/max dos pontos de controlo" — onde?
rg "min.*max|control point|pontos? de controlo" 01_core/src/entities/geometry.rs -n
```

**Output §A.2**: localização exacta (file:line) da função `bbox` actual + corpo literal para comparação com algoritmo analítico.

### §A.3 — Algoritmo a materializar

**Curva cúbica de Bézier B(t)** parametrizada por 4 pontos de controlo P₀, P₁, P₂, P₃:

```
B(t) = (1-t)³·P₀ + 3(1-t)²t·P₁ + 3(1-t)t²·P₂ + t³·P₃     para t ∈ [0, 1]
```

**Derivada** (vector de velocidade):

```
B'(t) = 3(1-t)²(P₁-P₀) + 6(1-t)t(P₂-P₁) + 3t²(P₃-P₂)
```

Expandindo em forma de polinómio quadrático at² + bt + c em cada eixo (x e y separadamente):

```
B'(t) = 3 · [ (P₁-P₀)(1-t)² + 2(P₂-P₁)(1-t)t + (P₃-P₂)t² ]
       = 3 · [ a·t² + b·t + c ]                                     (após expansão)
```

onde, para cada eixo (substituir P por px ou py):

```
a = -P₀ + 3·P₁ - 3·P₂ + P₃
b = 2·P₀ - 4·P₁ + 2·P₂
c = -P₀ + P₁
```

**Raízes de B'(t) = 0** (extremos locais) via fórmula quadrática:

```
t = (-b ± √(b² - 4ac)) / (2a)              se a ≠ 0
t = -c / b                                  se a == 0 e b ≠ 0
sem solução                                 se a == 0 e b == 0 (curva degenerada)
```

**Algoritmo final** (pseudo-código):

```text
fn bezier_cubic_bbox(p0, p1, p2, p3) -> (min_x, min_y, max_x, max_y):
    candidates_x = [p0.x, p3.x]    # endpoints sempre extremos
    candidates_y = [p0.y, p3.y]

    # Para cada eixo, calcular raízes de B'(t) = 0 em [0, 1]
    for axis in [x, y]:
        a = -p0[axis] + 3*p1[axis] - 3*p2[axis] + p3[axis]
        b = 2*p0[axis] - 4*p1[axis] + 2*p2[axis]
        c = -p0[axis] + p1[axis]

        for t in solve_quadratic(a, b, c):
            if 0.0 < t < 1.0:
                B_t = bezier_at(t, p0, p1, p2, p3)
                candidates[axis].push(B_t[axis])

    return (min(candidates_x), min(candidates_y),
            max(candidates_x), max(candidates_y))
```

**Complexidade**: O(1) por curva (no máximo 6 candidatos a comparar: 2 endpoints + 2 raízes x + 2 raízes y).

**Pureza L1**: matemática f64 pura; zero I/O; zero dependências externas. ADR-0029 preserved.

### §A.4 — Paridade vanilla (verificação)

```bash
# Vanilla calcula bbox analítica ou min/max?
rg "bbox|bounding|aabb" lab/typst-original/crates/typst-library/src/visualize/ -n
rg "bezier|cubic" lab/typst-original/crates/typst-library/src/visualize/ -n
```

**Output §A.4**: confirmar se vanilla usa algoritmo analítico (provável dado vanilla é maduro). Se vanilla também usa min/max simples, cristalino diverge intencionalmente por correcção (ADR-0054 graded permite).

### §A.5 — Casos de teste planeados

| Test | Descrição | Tipo |
|---|---|---|
| `bezier_bbox_linha_recta` | P0=P1=P2=P3 colineares → bbox = bounding dos endpoints | sanidade |
| `bezier_bbox_curva_dentro_pontos_controlo` | Cubic onde curva fica dentro de control points → analítica = pontos de controlo | concordância |
| `bezier_bbox_curva_excede_pontos_controlo_x` | P1/P2 forçam excursão extra em x → analítica > min/max simples | **vazamento corrigido** |
| `bezier_bbox_curva_excede_pontos_controlo_y` | Análogo eixo y | **vazamento corrigido** |
| `bezier_bbox_curva_degenerada_a_zero` | a=0, b=0 → fallback aos endpoints | edge case |
| `bezier_bbox_extremo_em_t_zero_um` | raízes exactamente em 0 ou 1 → não duplicar | edge case |
| `bezier_bbox_endpoints_unicos_extremos` | P1/P2 entre P0 e P3 monotónico → endpoints vencem | sanidade |
| `bezier_bbox_quadrante_negativo` | coordenadas negativas → ordenação preserved | edge case |

**Estimativa**: 8 testes unit (~50-80 LOC). Cabe em cap testes hard 80.

### §A.6 — Gates de paragem (§política condição)

Disparam paragem antes de §C:

1. **§A.1 detecta C2 ou C3** — Path inexistente ou sem CubicTo. Reformular spec: fecho OBSOLETED (não CLOSED), análogo a P276 DEBT-35b.
2. **§A.2 falha** — função bbox actual não localizada. Possível regressão arquitectural; investigar antes de avançar.
3. **§A.4 revela paridade vanilla incompatível** — vanilla usa abordagem fundamentalmente diferente (e.g. flattening recursivo). Avaliar antes de divergir.
4. **Cap LOC L1 hard 150 ameaçado** — algoritmo cresce além do esperado. Reformular.
5. **Cap doc Fase A hard 500 ameaçado** — diagnóstico demasiado detalhado. Reformular (cenário B2).
6. **Tests workspace ≠ 2644** baseline — regressão pré-existente; investigar antes de avançar.
7. **Algum teste novo da §A.5 falha após implementação** — algoritmo incorrecto; debugar.

---

## §2 — Sub-passo P277.B — Anotação cumulativa (não aplicável)

Passo é fecho CLOSED de DEBT material; não introduz decisão arquitectural nova nem aplica anotação cumulativa em ADR existente.

**Default**: §2 do relatório regista "B sub-passo não aplicado — fecho algorítmico não requer ADR. ADR-0029/0033/0054 preserved sem anotação."

Se Fase A revelar necessidade de ADR (e.g. divergência forte vs vanilla justificada formalmente), reformular spec para incluir B.

---

## §3 — Sub-passo P277.C — Materialização (testes-primeiro + código)

### §C.1 — Confirmar/actualizar L0 `prompts/entities/geometry.md`

Se Fase A §A.1 confirma C1 (Path existe com CubicTo), L0 actual está **desactualizado** (lista só `Rect / Ellipse / Line`).

**Operação**: estender L0 `geometry.md` para incluir `ShapeKind::Path` com descrição do algoritmo de bbox analítica:

```markdown
### `ShapeKind::Path`
Caminho composto por segmentos `MoveTo`, `LineTo`, `CubicTo`.
Bbox calculada **analiticamente** para `CubicTo` via raízes de B'(t)=0 em
cada eixo (extremos da curva paramétrica). Endpoints (P₀, P₃) e raízes
em t ∈ (0, 1) compõem o conjunto de candidatos para min/max por eixo.

Esta abordagem corrige o **vazamento visual subtil** que `CubicTo` produzia
quando calculado apenas por min/max dos pontos de controlo (curva real
pode exceder a bounding box dos pontos de controlo). Materializado em
P277 (DEBT-33 fecho CLOSED).
```

Aplicar `crystalline-lint --fix-hashes .` para propagar hash novo.

### §C.2 — Testes-primeiro

Adicionar 8 testes da §A.5 ao módulo de testes apropriado (geometry.rs `#[cfg(test)] mod tests` ou módulo de testes do layouter, conforme convenção localizada em Fase A).

Confirmar que testes **falham** com o algoritmo actual (min/max de pontos de controlo) para os casos `bezier_bbox_curva_excede_pontos_controlo_*` — esta falha é a evidência do vazamento que o DEBT-33 reporta.

### §C.3 — Implementação do algoritmo

Adicionar funções privadas (ou `pub(super)`) em `01_core/src/entities/geometry.rs` ou módulo identificado em Fase A:

```rust
/// Calcula a bbox analítica de uma curva cúbica de Bézier.
/// Retorna (min_x, min_y, max_x, max_y) em coordenadas locais.
fn bezier_cubic_bbox(p0: Point, p1: Point, p2: Point, p3: Point) -> (f64, f64, f64, f64) { ... }

/// Avalia B(t) num parâmetro t ∈ [0, 1].
fn bezier_at(t: f64, p0: Point, p1: Point, p2: Point, p3: Point) -> Point { ... }

/// Resolve at² + bt + c = 0 e retorna raízes em [0, 1].
fn solve_quadratic_in_unit(a: f64, b: f64, c: f64) -> Vec<f64> { ... }
```

Integrar em `bbox` (ou nome real identificado em Fase A) substituindo o cálculo min/max actual para o arm `CubicTo`. Arms `LineTo` e `MoveTo` preservados.

### §C.4 — Actualização DEBT.md

Mover DEBT-33 da Secção 1 para Secção 2 com etiqueta **ENCERRADO (Passo 277) ✓**:

```markdown
## DEBT-33 — Bounding Box de curvas Bézier — ENCERRADO (Passo 277) ✓

**Aberto em**: Passo 79.
**Fechado em**: 2026-05-XX (Passo 277).
**Etiqueta de fecho**: **CLOSED** (pattern P206E — materializado).

**Justificação literal**: P277 materializou o cálculo analítico dos
extremos da curva paramétrica B(t) para obter a AABB exacta de
`ShapeKind::Path` arm `CubicTo`. Algoritmo: raízes de B'(t)=0 em cada
eixo, candidatos {endpoints, raízes em t ∈ (0, 1)} comparados via
min/max. Complexidade O(1) por curva (≤6 candidatos). Pureza L1
preserved (matemática f64 pura, zero deps externas).

**Resultado observável**: vazamento visual subtil em curvas que
excedem os pontos de controlo deixa de ocorrer. Testes
`bezier_bbox_curva_excede_pontos_controlo_x` e `_y` confirmam
correcção analítica.

**Histórico preservado abaixo** per pattern P201/P202.

### (Histórico) Estado pré-fecho — DEBT-33 — EM ABERTO (Passo 79)

A bounding box de `ShapeKind::Path` é calculada verificando o min/max
dos pontos de controlo. Para `CubicTo`, a curva real pode ultrapassar
a caixa delimitadora dos pontos de controlo, causando vazamento visual
subtil. Resolução futura: cálculo analítico dos extremos da curva
paramétrica B(t) para obter a AABB exacta.
```

Acrescentar linha ao cabeçalho cumulativo do DEBT.md:

```markdown
> **Passo 277 (2026-05-XX)**: fecho de **DEBT-33** como CLOSED
> (pattern P206E). Materialização do cálculo analítico dos extremos
> de Bézier cúbica via raízes de B'(t)=0. Vazamento visual em curvas
> que excediam pontos de controlo corrigido. Total abertos: **7 → 6**.
> Detalhe em [`diagnosticos/diagnostico-debt-33-passo-277.md`].
```

### §C.5 — Validação final

```bash
cargo test --workspace 2>&1 | grep "test result"
# Esperado: ~2650-2660 passed (2644 baseline + 8 novos)

cargo run -p crystalline-lint --quiet
# Esperado: ✓ No violations found
```

### §C.6 — Relatório consolidado

Produz `/mnt/user-data/outputs/typst-passo-277-relatorio.md`. Estrutura:

- §1 — Validação contra spec (tabela critérios §7).
- §2 — Resumo factual: cenário §A.1 confirmado; algoritmo materializado; testes verdes.
- §3 — Operações realizadas:
  - L0 `geometry.md` actualizado (+ hash propagado).
  - Testes adicionados (lista 8).
  - Funções `bezier_cubic_bbox` / `bezier_at` / `solve_quadratic_in_unit` adicionadas.
  - Integração no arm `CubicTo` do cálculo bbox.
  - DEBT.md actualizado (DEBT-33 → Secção 2; cabeçalho com linha P277).
- §4 — Métricas: 2644 → ~2650-2660 tests; LOC L1 +60-100; LOC testes +50-80; hash L0 propagado.
- §5 — Sub-padrões: "Fecho CLOSED de DEBT real" N=1 inaugural; pattern P206E N=5 cumulativo (P206E + P276 OBSOLETED + P277 CLOSED).
- §6 — Próximo passo: P278 cleanup XS combinado.
- §7 — Referências cross-passos.

---

## §4 — Caps e gates de protecção

- **LOC L1**: hard 150 / soft 100.
- **LOC testes**: hard 80 / soft 60.
- **Modificações `.rs`**: `01_core/src/entities/geometry.rs` (ou módulo localizado em §A.2). Sem outras alterações de produção.
- **Modificações L0**: `prompts/entities/geometry.md` (estender com Path/Bézier; propagar hash).
- **Modificações `DEBT.md`**: mover + cabeçalho (operações análogas a P276).
- **Tests workspace**: 2644 → 2650-2660 esperado (delta apenas testes novos).
- **Lint**: zero violations preserved.

---

## §5 — Sub-padrões esperados aplicados

- **Fecho CLOSED de DEBT real com material** — N=1 inaugural P277. Aguardar reaplicação para considerar formalização.
- **Pattern P206E (3 caminhos fecho)** — N=4 (P276) → N=5 cumulativo. Combina OBSOLETED (P276) + CLOSED (P277) → primeiro consumo balanceado.
- **Diagnóstico imutável** — N=35 → N=36 cumulativo (31º consumo).
- **Testes-primeiro** — preserved (cf. CLAUDE.md Protocolo de Nucleação).
- **L0 actualizado antes do código** — verificado (P277.C.1 antes de P277.C.2/.3).
- **Algoritmo matemático puro em L1** — preserved (zero crates externas; ADR-0029 absoluto).

---

## §6 — Workflow operacional

1. Utilizador upload `00_nucleo/DEBT.md` literal + opcionalmente `01_core/src/entities/geometry.rs` se quiser passar o conteúdo directamente.
2. Claude Code executa Fase A:
   - Produz `typst-passo-277A-diagnostico.md` em `/mnt/user-data/outputs/`.
   - Resolve cenário C1/C2/C3 §A.1.
   - Se C2/C3: para; spec reformulada como OBSOLETED.
3. Utilizador valida Fase A (gates §A.6 não dispararam).
4. Claude Code executa §C:
   - Actualiza L0 `geometry.md` (§C.1) + `--fix-hashes`.
   - Escreve 8 testes (§C.2); confirma falham para casos de vazamento.
   - Implementa algoritmo (§C.3).
   - Confirma testes passam (`cargo test --workspace`).
   - Edita `DEBT.md` (§C.4).
   - Re-corre `crystalline-lint` zero violations.
   - Produz `typst-passo-277-relatorio.md` (§C.6).
5. Utilizador valida relatório.
6. Próximo passo: **P278 cleanup XS combinado** (content-md-debt56-update + helper-group-bbox + draw-item-local-text-image).

---

## §7 — Critério de fecho

P277 fecha quando:

- [ ] Fase A produzida; §A.1 confirma C1 (ou gate disparou e spec reformulada).
- [ ] §A.2 localizou função bbox actual.
- [ ] §A.3 algoritmo registado.
- [ ] §A.4 paridade vanilla documentada.
- [ ] §A.5 testes planeados (8).
- [ ] L0 `geometry.md` actualizado com secção Path/Bézier; hash propagado.
- [ ] 8 testes novos verdes (confirmam algoritmo correcto + casos de vazamento corrigidos).
- [ ] Algoritmo implementado em `geometry.rs` (ou módulo identificado).
- [ ] Arm `CubicTo` do bbox usa cálculo analítico.
- [ ] DEBT-33 movido para Secção 2 com etiqueta CLOSED.
- [ ] Cabeçalho DEBT.md com linha P277.
- [ ] Tests workspace 2650-2660 passed.
- [ ] Lint zero violations.
- [ ] Cap LOC L1 hard 150 respeitado.
- [ ] Cap doc respeitado.
- [ ] Relatório consolidado §1-§7 completos.

P277 NÃO fecha se:

- §A.1 cenário C2/C3 — reformular como OBSOLETED.
- §A.4 paridade vanilla materialmente incompatível sem justificação ADR.
- Algum teste novo falha após implementação.
- Regressão tests baseline (2644).
- Cap LOC L1 hard estourado.

---

## §8 — Referências cross-passos

- **P79** — origem DEBT-33 (Passo dedicado a `ShapeKind::Path` no layout vectorial).
- **P125** — auditoria DEBTs original; DEBT-33 classificado como "manter".
- **P206E** — pattern fecho 3-caminhos.
- **P275** — auditoria empírica pós-cluster Gradient; DEBT-33 listado como "S+M; sem bloqueador".
- **P276** — DEBT-35b OBSOLETED; primeiro consumo P206E pós-auditoria; precedente metodológico imediato.
- **P201/P202** — pattern "histórico textual preservado".
- **ADR-0029** — Pureza física L1 (preserved absoluto neste passo).
- **ADR-0033** — Paridade vanilla (verificar §A.4).
- **ADR-0054** — Critério fecho graded (permite divergência justificada).
- **ADR-0085** — Diagnóstico imutável (31º consumo).
- **ADR-0094** — Meta-operacional specs (Pattern 1 cap LOC).

---

## §9 — Notas de execução para Claude Code

- **NÃO assumir cenário C1 sem confirmar §A.1** — primeiro acto é localizar Path/CubicTo no código.
- **Se §A.1 retornar C2 ou C3, PARAR** e reportar ao humano antes de prosseguir. Spec reformulada (OBSOLETED) pode aproveitar estrutura de P276 como template.
- **Testes-primeiro**: escrever os 8 testes (§C.2) **antes** do código (§C.3). Confirmar falha esperada para casos de vazamento, depois implementar.
- **L0 antes do código**: `geometry.md` actualizado (§C.1) **antes** de tocar `.rs`. Protocolo de nucleação CLAUDE.md.
- **Apply `crystalline-lint --fix-hashes`** após cada edição L0.
- **Outputs**: 2 ficheiros em `/mnt/user-data/outputs/` (`typst-passo-277A-diagnostico.md` + `typst-passo-277-relatorio.md`).
- **Tempo estimado**: 60-120 min (passo S+M com código).
- **Confirmação visual final**: `rg "^## DEBT-33" 00_nucleo/DEBT.md` deve mostrar entrada na Secção 2 com "ENCERRADO (Passo 277) ✓".
- **Anti-padrão a evitar**: NÃO usar `crate` externa de geometria (`lyon`, `kurbo`, etc.). Algoritmo é matemática f64 pura — implementar localmente. ADR-0029 absoluto.

---

*Spec produzida em 2026-05-XX como primeiro fecho CLOSED de DEBT real pós-cluster Gradient. DEBT-33 (Bézier bbox) é candidato natural — algoritmo conhecido, sem bloqueador, materialização pura L1 que corrige vazamento visual subtil em curvas que excedem pontos de controlo. Reforça o projecto via correcção observável + base sólida para futuras features Visualize (Curve/Polygon/Stroke).*

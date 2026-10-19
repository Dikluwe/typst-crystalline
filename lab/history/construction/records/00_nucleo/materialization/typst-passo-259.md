# Passo 259 — Pivot Visualize (auditoria Fase A + actualização docs L0 + decisão condicional B1/B2/B3)

**Data**: 2026-05-15
**Tipo**: passo composto sequencial; magnitude estimada XS-L
conforme cenário Fase A.
**Pré-requisito leitura obrigatória** (CLAUDE.md Regra de Ouro):
- `CLAUDE.md` (Regra de Ouro + Protocolo de Nucleação + Ordem
  testes-primeiro).
- `00_nucleo/diagnosticos/diagnostico-visualize-passo-259.md`
  (diagnóstico pai com 9 subsistemas + cenários B1/B2/B3).
- `00_nucleo/diagnosticos/fase-a-checklist-visualize-passo-259.md`
  (checklist executável Fase A).
- ADR-0029 (EM VIGOR — obriga diagnóstico vanilla + ADR
  explícita para scope-outs).
- ADR-0083 (P257 Color paridade vanilla — anotações cumulativas
  podem ser adicionadas).
- ADR-0033, ADR-0034, ADR-0054, ADR-0065.
- DEBT-33 (Bézier bbox; alvo possível).
- Relatórios precedentes: P255 (DEBT-8 Math), P257 (Color
  paridade), P258 (Model fecho conceptual) — padrões
  metodológicos estabelecidos.

**Outputs canónicos esperados** ao fim do passo:
- `00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md`
  (Fase A executada; tabelas A+B preenchidas; imutável per
  ADR-0034).
- Prompts L0 obsoletos actualizados (esperado: `entities/geometry.md`
  desactualizado vs Path P79 + clip + P252 refactor); hashes
  propagados.
- DEBT-33 actualizada conforme cenário.
- Eventual ADR nova (cenário B2 Opção 1 Paint enum) ou
  ADR-0083 anotação cumulativa.
- Eventual código L1 novo (cenário B2) com prompts L0
  actualizados primeiro per Regra de Ouro.
- Relatório do passo em
  `00_nucleo/materialization/typst-passo-259-relatorio.md`.

---

## §0 — Princípios vinculativos para este passo

1. **Regra de Ouro CLAUDE.md** — código L1 nunca antes de
   prompt L0. Order: Fase A audit → docs L0 → fix-hashes →
   (se materialização) testes-primeiro → código.
2. **ADR-0029 §"Diagnosticar primeiro"** — qualquer tipo
   vanilla materializado neste passo (Paint, Gradient,
   Polygon, etc.) obriga leitura literal vanilla antes de
   definir estrutura.
3. **ADR-0029 §"Simplificações aceites apenas com ADR
   explícita"** — qualquer scope-out de tipo vanilla obriga
   ADR (nova ou anotação ADR-0083).
4. **ADR-0034 + ADR-0065 inventariar primeiro** — Fase A
   produz evidência factual antes de qualquer decisão.
5. **Ordem testes-primeiro** — para cada código novo: testes
   antes de implementação.
6. **`crystalline-lint .`** zero violations no fim do passo.
7. **Tests workspace** sem regressão (contagem ≥ baseline
   2334 pós-P258).
8. **Materialization é leitura proibida por iniciativa
   própria** — Claude Code não deve ler
   `00_nucleo/materialization/` excepto com path explícito.
9. **Política "sem novas reservas"** preservada — se
   descoberta nova obstrução, registar como achado no
   diagnóstico imutável, não criar DEBT/ADR reserva.

---

## §1 — Sub-passo P259.A: Fase A (auditoria empírica)

**Objectivo**: produzir evidência factual sobre estado real
dos 9 subsistemas Visualize.

**Materialização**: zero código novo. Apenas leitura e
produção de diagnóstico imutável.

### Acções obrigatórias

Executar **integralmente** os 10 blocos de comandos listados
em `00_nucleo/diagnosticos/fase-a-checklist-visualize-passo-259.md`:

- **Bloco 1** Color subsistema (confirmar P257).
- **Bloco 2** Shapes (Rect/Ellipse/Line/Path + Polygon/Curve).
- **Bloco 3** Path bbox (DEBT-33).
- **Bloco 4** Stroke (Paint, dash, caps).
- **Bloco 5** Gradient (ausente esperado).
- **Bloco 6** Paint, Tiling (ausente esperado).
- **Bloco 7** Image (SVG + metadata).
- **Bloco 8** Transform `origin`.
- **Bloco 9** Inconsistências documentais.
- **Bloco 10** Exportador PDF visualize features.

### Output exigido — ficheiro novo

Criar
`00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md`
com a seguinte estrutura (imutável após criação per ADR-0034;
análogo a `diagnostico-math-fase-a-passo-255.md` /
`diagnostico-color-vanilla-passo-257.md` /
`diagnostico-model-fase-a-passo-258.md`):

```markdown
# Diagnóstico Visualize Fase A — Passo 259 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0034 diagnóstico canónico + ADR-0065
inventariar primeiro critério #5.
**Diagnóstico pai**: `diagnostico-visualize-passo-259.md` +
`fase-a-checklist-visualize-passo-259.md`.
**Análogo estrutural**: P255 (Math) + P257 (Color) + P258
(Model).

---

## §1 — Comandos executados e output literal

(Colar output literal completo dos 10 blocos. Sem
interpretação aqui — só registar.)

## §2 — Classificação por subsistema (Tabela A — 27 entradas)

| # | Subsistema | Pré-audit | Audit P259 | Hits literais | Justificação |
|---|------------|-----------|------------|---------------|--------------|
(27 entradas conforme checklist Tabela A)

## §3 — Estado agregado (Tabela B)

| Estado | Pré-P259 estimado | Audit P259 | Δ |
|--------|-------------------|------------|---|
(Linhas implementado / implementado⁺ / parcial / ausente)

## §4 — Achados inesperados

(Materializações fora do esperado; subsistemas com expansão
não documentada; inconsistências documentais detectadas.)

## §5 — Decisão cenário Fase B

**Contagem fechados/abertos**: _/27 fechados; _/27 abertos.

**Cobertura agregada empírica**: _%.

**Cenário escolhido**: ☐ B1 / ☐ B2 / ☐ B3.

**Se B2, opção(ões) recomendada(s)**:
- ☐ Opção 1 — Paint enum + Gradient Linear (M+S+; +11pp).
- ☐ Opção 2 — Polygon + Ellipse refino (S+S; +6pp).
- ☐ Opção 3 — DEBT-33 + Stroke<Length> (S+M; +5pp).
- ☐ Opção 4 — Transform origin pivot (S+; +2-3pp).
- ☐ Opção 5 — SVG image format (L+; NÃO recomendado P260).

## §6 — Referências

(P255/P257/P258 precedentes; ADR-0083 P257; DEBT-33; etc.)
```

### Critério de aceitação P259.A

- Ficheiro
  `diagnostico-visualize-fase-a-passo-259.md` criado em
  `00_nucleo/diagnosticos/`.
- §1-§5 preenchidos com **conteúdo literal** (hits/no-hits
  factuais), não interpretativos.
- Tabelas A+B preenchidas para todos os 27 subsistemas.
- Decisão cenário B1/B2/B3 explicitada em §5 com
  justificação factual.
- Zero alterações em código L1/L2/L3/L4.
- Zero alterações a prompts L0, ADRs ou DEBT.md (ainda — vem
  em P259.B+).

---

## §2 — Sub-passo P259.B: Reconciliação documental L0

**Objectivo**: actualizar prompts L0 obsoletos descobertos em
P259.A §4.

**Materialização**: edição de prompts L0 + `--fix-hashes`. Sem
código L1.

### Acções obrigatórias

#### B.1 — Identificar L0 prompts obsoletos

Baseado em P259.A §4. Esperado (per precedente P255/P257/P258):
- `00_nucleo/prompts/entities/geometry.md` — provavelmente lista
  apenas Stroke + ShapeKind base (Rect/Ellipse/Line); falta:
  - Path expansão P79 (MoveTo/LineTo/CubicTo + Close + QuadTo
    se existir).
  - Clip via FrameItem::Group.clip_mask (P79).
  - P252 refactor cross-cutting Stroke (paridade pattern).
- Eventuais outros prompts L0 descobertos em audit.

#### B.2 — Editar L0 prompts

Para cada prompt L0 desactualizado:
- Substituir secções obsoletas por estado actual descoberto
  na Fase A.
- Preservar histórico (não apagar; anotar como "actualizado
  por P259 reconciliação documental").
- Manter restrições arquitecturais (L1 puro; sem dependências
  externas além das declaradas).
- **Decisão arquitectural** (precedente ADR-0080 §"refactor
  aditivo" aplicado em P258.B): preservar representação base
  como histórico cumulativo; secções subsequentes cobrem
  materializações reais. **Não reconciliação destructiva**.

#### B.3 — Propagar hashes

```bash
cargo run -p crystalline-lint -- --fix-hashes .
```

#### B.4 — Verificação

```bash
cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found
cargo test --workspace
# Esperado: contagem inalterada (≥2334)
```

### Critério de aceitação P259.B

- Prompts L0 reconciliados reflectem código real.
- Hashes propagados (zero violations V5 PromptStale).
- Tests workspace inalterados em contagem.
- Zero alterações a código L1/L2/L3/L4 (esta fase é
  puramente documental).

---

## §3 — Sub-passo P259.C: Materialização condicional

**Executar apenas** se P259.A §5 escolheu cenário **B2** ou
**B3**.

**Se P259.A escolheu B1**, saltar P259.C directamente para
P259.D.

### Cenário B1 — Fecho conceptual

Sem materialização. P259.D actualiza DEBTs + ADR-0083
anotação cumulativa + relatório.

### Cenário B2 — Sub-passos prioritários

**Decisão de qual(is) Opção(ões) materializar** registada em
P259.A §5. Materializar uma de cada vez (granularidade
preservada per ADR-0061 §"granularidade 1-2 features/passo").

**P259 não materializa código** — opções B2 ficam para
P260+ dedicados.

**Excepção**: se Opção 4 (Transform `origin` pivot) for
cenário, magnitude S+ permite materialização **dentro** de
P259.C como sub-passo aditivo (precedente: P156L primeiro
refactor real em série aditiva).

#### Opção 1 — Paint enum + Gradient Linear (P260 + P261 separados)

**P260 — Paint enum** (S+):

**Pré-requisitos** (Regra de Ouro):
1. Decisão ADR: ADR nova OU anotação cumulativa ADR-0083 (per
   magnitude). Se anotação cumulativa: ADR-0083 ganha secção
   "Anotação P260 — Paint enum wrapper como pré-requisito
   Gradient/Tiling materialização".
2. Criar L0 prompt `entities/paint.md` per estrutura
   `entities/color.md` analógico.
3. Diagnóstico vanilla per ADR-0029:
   `lab/typst-original/crates/typst-library/src/visualize/paint.rs`
   ou similar — confirmar estrutura `enum Paint { Solid,
   Gradient, Tiling }`.

**Materialização** (testes primeiro):

1. **Testes**:
   ```rust
   #[test]
   fn paint_color_variant() { ... }
   #[test]
   fn paint_eq() { ... }
   // Paint::Gradient e Paint::Tiling: variants placeholders
   // ou comentadas até materialização real (decisão local).
   ```
   Executar `cargo test paint::` — verificar falham.

2. **`01_core/src/entities/paint.rs`**:
   ```rust
   #[derive(Debug, Clone, PartialEq)]
   pub enum Paint {
       Solid(Color),
       // Gradient(Gradient),  // P261
       // Tiling(Tiling),       // futuro
   }
   ```
   **Decisão minimalista** (precedente Color P25 → P257): só
   `Solid(Color)` inicial; expandir consumer-driven.

3. **Adaptar consumers** (cross-cutting análogo P252):
   - `Stroke.paint: Color → Paint`.
   - `Style::Fill: Color → Paint`.
   - `FrameItem::Text.fill` etc.
   - Helper `Paint::to_color()` para PDF exporter (que sabe
     emitir Color, não Paint).

4. Magnitude esperada: S+ (~1-2h; +8-12 tests).
   Cobertura: +3pp Visualize agregada.

**P261 — Gradient Linear** (M):

**Pré-requisitos**:
1. P260 Paint enum completo.
2. L0 prompt novo `entities/gradient.md`.
3. Diagnóstico vanilla per ADR-0029:
   `lab/typst-original/crates/typst-library/src/visualize/gradient.rs`.
4. ADR explícita (nova): "Gradient subset materializado +
   scope-outs Radial/Conic" — análogo ADR-0083 estrutura
   per ADR-0029 §"Simplificações aceites".

**Materialização** (testes primeiro):

1. **Testes** (+15-20):
   - `gradient_linear_construcao`.
   - `gradient_stop_eq`.
   - `gradient_linear_angle_radianos`.
   - PDF emit: shading pattern test.

2. **Tipos novos**:
   - `entities/gradient.rs`:
     ```rust
     pub struct GradientStop { offset: f64, color: Color }
     pub enum Gradient {
         Linear { stops: Arc<[GradientStop]>, angle: Angle },
         // Radial { ... },  // scope-out ADR nova
         // Conic { ... },   // scope-out ADR nova
     }
     ```
3. **Expandir Paint**: `Paint::Gradient(Gradient)` activado.
4. **Stdlib**: `native_gradient_linear(stops, angle)`.
5. **PDF exporter**: emit `/Pattern /Shading /ShadingType 2`
   (axial shading) com colour stops.

6. Magnitude esperada: M (~3-4h; +15-20 tests).
   Cobertura: +8pp Visualize agregada.

#### Opção 2 — Polygon + Ellipse refino (P260 + P261)

**P260 — Polygon**:
- L0 prompt `entities/geometry.md` actualizado com Polygon.
- `ShapeKind::Polygon { points: Arc<[Point]> }` ou similar
  (decisão local: Point existe? ou usar `(f64, f64)`?).
- Stdlib `native_polygon(points: array)`.
- PDF exporter: MoveTo + N LineTo + ClosePath.
- Magnitude: S+ (~1-2h; +5-8 tests).
- Cobertura: +3pp.

**P261 — Ellipse real**:
- PDF exporter: substituir rectângulo placeholder por 4
  arcos Bézier (constante mágica `0.5522847498`).
- Geometry helper `ellipse_to_cubic_segments(rx, ry)`.
- Magnitude: S (~1h; +3-5 tests).
- Cobertura: +3pp.

#### Opção 3 — DEBT-33 + Stroke<Length> (P260 + P261)

**P260 — DEBT-33 Bézier bbox exacto**:
- Cálculo analítico extremos B(t) (raízes da derivada
  `B'(t) = 0` em [0,1]).
- Magnitude: S+ (~1-2h; +5 tests).
- DEBT-33 transita CLOSED em P259.D ou P260.D.

**P261 — Stroke<Length>**:
- L0 prompt `entities/geometry.md` actualizado.
- Possível ADR nova: "Stroke thickness em Length unit em vez
  de f64".
- `struct Stroke { paint, thickness: Length }`.
- Adaptar consumers (geometry, exporter, layouter).
- Magnitude: M (~2-3h; +10-15 tests).
- Cobertura: +2pp.

#### Opção 4 — Transform `origin` pivot (P259.C aditivo)

**Pode materializar dentro de P259.C** se cenário Fase B
permitir (magnitude S+ aditiva).

**Pré-requisitos**:
- Confirmar Bloco 8 P259.A — `origin` ausente.
- L0 prompt `entities/layout_types.md` actualizado (campo
  `origin` em TransformMatrix?).

**Materialização** (testes primeiro):
- `TransformMatrix::with_origin(matrix, origin: Point)` ou
  similar.
- Stdlib `native_rotate`/`native_scale`/`native_skew`
  aceitam `origin: ?` param.
- Cobertura cross-tipo (Move/Rotate/Scale/Skew).

Magnitude: S+ (~1h; +5 tests).
Cobertura: +2pp.

#### Opção 5 — SVG image format (L+; NÃO recomendado P260)

**Não recomendado neste passo**. Magnitude L+ (6-10h);
exige ADR crate `usvg` + `resvg`.

Adiar para passo dedicado pós-P259+P260+P261.

### Cenário B3 — Re-classificação primeiro

**Improvável**. Se materializar:

1. Re-classificar Tabela A conservadoramente.
2. ADR-0083 anotação cumulativa de revisão para baixo.
3. Sub-passos de elevação prioritários como cenário B2.

### Critério de aceitação P259.C

- Cada Opção materializada (B2) respeita ordem
  testes-primeiro.
- Cada feature tem prompt L0 actualizado **antes** do código.
- Hashes propagados.
- Tests workspace +N (consoante Opções resolvidas).
- Zero violations linter.
- Paridade observable preservada.

---

## §4 — Sub-passo P259.D: Actualização cumulativa + relatório

**Objectivo**: actualizar DEBT-33, ADRs cumulativas, e
produzir relatório final.

### D.1 — Actualizar `00_nucleo/DEBT.md`

**Se cenário B1** ou **B2 Opção 3 DEBT-33 materializada em
P259.C**:
- DEBT-33 transita para **CLOSED**.
- Secção "Resolvido em Passo 259" com referência cruzada.

**Se outras Opções B2** ou **B3**:
- DEBT-33 preservada (estado actual).

### D.2 — ADRs cumulativas

**ADR-0083** (Color paridade vanilla; IMPLEMENTADO P257):
- Anotação cumulativa P259 com cobertura Visualize empírica
  revista; achados Fase A; Opções materializadas; estado
  Gradient/Paint/Tiling.
- Status `IMPLEMENTADO` preservado literal.

**ADR-0062** (hayagriva PROPOSTO):
- Preservada — não tocada por P259 (Visualize, não Model).

**ADR-0061** (Layout Fase X; PROPOSTO):
- Preservada — Visualize não Layout.

**Eventual ADR nova** (cenário B2 Opções 1/3 com decisões
arquitecturais):
- Opção 1 Gradient: ADR nova "Gradient subset materializado"
  ou anotação ADR-0083.
- Opção 3 Stroke<Length>: ADR nova "Stroke thickness Length
  unit" se decisão local exigir.

### D.3 — README ADRs

`00_nucleo/adr/README.md`:
- Entrada P259 nos passos-chave.
- Distribuição ADRs actualizada se transições ocorrerem.

### D.4 — Relatório do passo

`00_nucleo/materialization/typst-passo-259-relatorio.md`
estrutura canónica (análogo P255/P257/P258):

- **§1 Sumário executivo** — cenário Fase A; opções
  materializadas; tests delta; ADRs tocadas; prompts L0
  actualizados.
- **§2 Sub-passo P259.A** — output Fase A resumido.
- **§3 Sub-passo P259.B** — prompts L0 editados; hashes.
- **§4 Sub-passo P259.C** — código materializado (se
  aplicável) com referências a ficheiros.
- **§5 Sub-passo P259.D** — DEBT-33 + ADRs cumulativas.
- **§6 Padrões metodológicos** — ADR-0065 critério #5
  aplicado; subpadrão "auditoria condicional" cresce **N=4
  → N=5** (atinge limiar formalização clara); eventuais
  subpadrões novos.
- **§7 Cobertura** — Visualize passa de ~60-65% (estimativa
  pré-P259) para X% (audit empírico).
- **§8 Limitações e trabalho futuro** — Opções não
  materializadas; scope-outs registados; subsistemas
  ausentes preservados.
- **§9 Critério de aceitação global P259 — Checklist final**.
- **§10 Referências**.

### Critério de aceitação P259.D

- DEBT-33 reflecte estado real pós-passo.
- ADRs cumulativas anotadas.
- README ADRs actualizado.
- Relatório criado.
- Cross-references coerentes.

---

## §5 — Critério de aceitação global P259

Ao fim do passo, todos os seguintes têm de ser verdadeiros:

- [ ] `cargo run -p crystalline-lint -- .` retorna
  `✓ No violations found`.
- [ ] `cargo test --workspace` retorna contagem ≥ baseline
  2334 (sem regressão).
- [ ] `diagnostico-visualize-fase-a-passo-259.md` existe com
  tabelas A+B preenchidas para 27 subsistemas.
- [ ] Prompts L0 obsoletos reconciliados.
- [ ] Hashes propagados (zero violations V5).
- [ ] DEBT-33 actualizada conforme cenário.
- [ ] ADR-0083 anotação cumulativa adicionada.
- [ ] (Se B2 Opção 1) ADR nova ou anotação ADR-0083 cobrindo
  Paint/Gradient.
- [ ] (Se B2 Opção 4) Transform `origin` materializado dentro
  de P259.C com testes-primeiro respeitado.
- [ ] Relatório criado em `00_nucleo/materialization/`.
- [ ] Se cenário B2/B3 e materializaste código: cada Opção
  respeitou ordem testes-primeiro e teve L0 actualizado antes
  do código.

---

## §6 — Sequência operacional condensada

Para Claude Code seguir linearmente:

1. **Ler** `CLAUDE.md`, diagnóstico P259, checklist P259,
   ADRs 0029/0033/0034/0054/0065/0083, DEBT-33, relatórios
   precedentes P255 + P257 + P258 (padrões metodológicos
   estabelecidos).
2. **Reportar** estado inicial: tests count (esperado 2334
   pós-P258) + lint baseline.
3. **P259.A** — Executar 10 blocos de comandos do checklist;
   criar diagnóstico Fase A imutável; tabelas A+B preenchidas;
   decisão B1/B2/B3 explícita em §5.
4. **P259.B** — Editar prompts L0 obsoletos descobertos;
   `--fix-hashes`; lint limpo; tests inalterados.
5. **P259.C condicional** — Se B2 e Opção 4 escolhida:
   materializar `origin` pivot transform aditivo (testes
   primeiro, magnitude S+). Outras Opções B2 (1/2/3/5):
   adiar para P260+ dedicados.
6. **P259.D** — Actualizar DEBT-33, ADRs cumulativas, README
   ADRs; criar relatório.
7. **Verificação final** — todo o checklist §5 satisfeito.
8. **Reportar** ao utilizador: cenário escolhido, Opções
   materializadas, tests delta, ficheiros criados/editados,
   recomendação P260+ pós-P259.

---

## §7 — Política de paragem

Claude Code **deve parar e perguntar ao utilizador** se:

- P259.A revela cobertura empírica muito diferente do
  esperado (e.g. ≥85% — sugere Cenário B1 fecho conceptual
  mas pode esconder regressão ou re-classificação optimista).
- P259.A revela materialização de subsistemas **não previstos**
  no diagnóstico pai (e.g. Gradient parcial já materializado;
  Polygon presente; SVG infraestrutura existente).
- P259.B descobre prompt L0 com critérios contraditórios face
  ao código real (não apenas desactualizados — exigem decisão
  arquitectural).
- P259.C Opção 4 (origin pivot) revela conflict com
  TransformMatrix existente (P78+P156F) que exija refactor
  maior do esperado.
- P259.D revela que Opção 1 Paint enum exige ADR independente
  PROPOSTO (decisão arquitectural fora do scope deste passo —
  exemplos: Paint::Image como variant adicional ausente do
  vanilla; Tiling antes de Gradient por razão histórica).
- Decisão de granularidade ADR (B2 Opção 1): ADR nova
  dedicada Paint+Gradient OU anotações cumulativas ADR-0083?
- `crystalline-lint` reporta violations não-triviais.
- Tests regridem sem causa óbvia.
- Magnitude real Opção materializada em P259.C ultrapassa S+
  (e.g. cascade Stroke.paint → Paint exige adaptar >10
  consumers — sair imediatamente para passo dedicado P260).

Em qualquer paragem, registar contexto no relatório parcial e
aguardar instrução.

---

## §8 — Notas estratégicas

### Relação com P258 (Model)

P258 fechou Model conceptualmente (~73% cobertura). P259
pivota para Visualize — outro módulo cross-funcional com
múltiplos subsistemas independentes.

### Subpadrão "auditoria condicional" cresce N=4 → N=5

Cumulativo:
- N=1 P192A.
- N=2 P255.
- N=3 P257.
- N=4 P258.
- **N=5 P259**.

**Patamar N=5 atinge limiar formalização clara**. Política
consistente N=4-5 para promoção formal. Candidato a
formalização em ADR meta admin XS futuro (paridade ADR-0080
N=9; ADR-0082 N=8 promoções reais; ADR-0064 N=8 saturação).

### Subpadrão "diagnóstico imutável precedente à acção" N=3 → N=4

Cumulativo:
- N=1 P255.
- N=2 P257.
- N=3 P258.
- **N=4 P259** (se P259.A materializar).

### Subpadrão "Refactor cross-cutting entity primitivo" N=2

Cumulativo:
- N=1 P252 (Stroke).
- N=2 P257 (Color).

Se P259.C Opção 1 materializar Paint enum (cross-cutting
Color → Paint em Stroke + Fill + FrameItem::Text), **N=2 →
N=3** (formalização sólida).

### Subpadrão "ADR PROPOSTO+IMPLEMENTADO mesmo passo"

Cumulativo:
- N=1 P257 (ADR-0083).
- **Se P259.C Opção 1 com ADR nova materializar**: N=2.

### Política "sem novas reservas"

Preservada. Recomendações P259.C Opções são para validação
humana; não compromissos automáticos.

### Pós-P259 — sequência lógica recomendada

Conforme decisão Fase B:

- **B1**: pivot para Text audit (P260) ou Layout audit
  (P261).
- **B2 Opção 1**: P260 Paint enum (S+); P261 Gradient Linear
  (M).
- **B2 Opção 2**: P260 Polygon (S+); P261 Ellipse refino (S).
- **B2 Opção 3**: P260 DEBT-33 (S+); P261 Stroke<Length> (M).
- **B2 Opção 4**: materializada dentro de P259.C; pivot a
  outra Opção em P260.
- **B2 Opção 5**: NÃO em P260 (L+ adiar).

---

## §9 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0019, ADR-0026, ADR-0029, ADR-0033, ADR-0034,
  ADR-0054, ADR-0065, ADR-0083.
- DEBT-33 (Bézier bbox; alvo possível).
- DEBT-31, DEBT-30 (encerrados — contexto histórico).
- `00_nucleo/diagnosticos/diagnostico-visualize-passo-259.md`
  — diagnóstico pai (planeamento Fase A/B).
- `00_nucleo/diagnosticos/fase-a-checklist-visualize-passo-259.md`
  — comandos exactos P259.A.
- P25 — Color simplificado original (REVOGADO).
- P72-P74 — Image stack (JPEG + PNG + dimensões).
- P76 — geometry primitivos (Stroke + ShapeKind).
- P78 — Transform (Move/Rotate/Scale).
- P79 — Path + clip + DEBT-30/33.
- P156F — Skew.
- P252 — Refactor cross-cutting Stroke (precedente N=1).
- P257 — Color paridade vanilla 8/8 (precedente N=2;
  template "ADR PROPOSTO+IMPLEMENTADO mesmo passo" N=1).
- P255, P258 — precedentes "auditoria condicional".
- ADR-0029 §"Sobre os tipos tipográficos vanilla" §enumeração
  — fonte canónica das pendências Visualize.

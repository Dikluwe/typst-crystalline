# typst-passo-267 — P-Gradient-Conic L1+stdlib

**Magnitude**: M (cap: ≤ 350 LOC L1 + ≤ 80 LOC stdlib; ≤ 40 testes novos).
**Cluster**: Visualize / Gradient.
**Origem**: §7 contexto pós-P266 candidato #1.
**Sequência**: P267 (L1+stdlib) → P268 (PDF /ShadingType 3 Conic) — análogo literal P262+P263 (Linear) e P264+P265 (Radial).
**Sub-padrão "dividir granularidade L1+stdlib / L3 dedicado"**: aplicação N=4 cumulativa (P262+P263; P264+P265; **P267+P268**).

---

## §0 — Princípios vinculativos

Numerados; cada um literal e não-negociável.

1. **Regra de Ouro CLAUDE.md**: código L1 nunca antes de prompt L0. Ordem: diagnóstico → ADR → prompt L0 → fix-hashes → testes-primeiro → código.
2. **ADR-0034** (diagnóstico imutável) + **ADR-0085** (formaliza padrão N=7). Fase A produz ficheiro `00_nucleo/diagnosticos/diagnostico-gradient-conic-passo-267.md` que nunca é editado após criação.
3. **ADR-0084** (auditoria condicional). Se Fase A revelar que Conic já está parcialmente materializado em L1 (não esperado, mas verificar), passo aborta sub-passo .C e converte-se em reconciliação documental.
4. **ADR-0087 + ADR-0088** (Gradient Linear-only + Radial-only). Esta passo **revoga parcialmente** o scope-out Conic ADR-0088 §"variants não materializados". Criar **ADR-0089 Gradient Conic-only** EM VIGOR.
5. **ADR-0086** (Paint wrapper Solid only — Gradient/Tiling reserva). `Paint::Gradient(Gradient)` já activo desde P261; Conic encaixa transparente via enum `Gradient::Conic(ConicGradient)`.
6. **ADR-0054 perfil graded DEBT-1**: anotação cumulativa P267 (cluster Gradient agora 3/3 variants quanto a L1+stdlib). Status EM VIGOR preservado literal.
7. **ADR-0039 preservado**: `TextStyle.fill: Option<Color>` intocado — Conic não toca Text.
8. **ADR-0065 auto-aplicação inline**: passo é elegível (N=6 cumulativa após este).
9. **Crystalline-lint zero violations** obrigatório no fecho.
10. **Vanilla read-first autorizado**: `lab/typst-original/` consultado literal para shape de `ConicGradient` e ângulo de partida (`angle`).

---

## §1 — Fase A audit/diagnóstico empírico

Produz ficheiro imutável `00_nucleo/diagnosticos/diagnostico-gradient-conic-passo-267.md`.

### Comandos exactos a executar

```bash
# 1. Vanilla: shape ConicGradient + campos
rg -n "ConicGradient|conic" lab/typst-original/crates/typst-library/src/visualize/gradient.rs | head -60

# 2. Vanilla: stdlib bindings conic gradient
rg -n "conic" lab/typst-original/crates/typst-library/src/ | grep -v target

# 3. Vanilla: PDF Conic (para conhecer P268 ainda que não materializar agora)
rg -n "conic|ShadingType.*[13]" lab/typst-original/crates/typst-pdf/src/ | head -30

# 4. Cristalino: estado actual Gradient enum + variants
rg -n "enum Gradient|impl Gradient" 01_core/src/visualize/gradient.rs

# 5. Cristalino: stdlib gradient bindings (linear/radial actuais)
rg -n "gradient" 01_core/src/library/visualize/ | head -40

# 6. Cristalino: testes Gradient existentes (paridade contagem pre)
find 01_core/src/visualize -name "*.rs" -exec grep -l "gradient" {} \;
cargo test -p typst-cristalino-core visualize::gradient 2>&1 | tail -20

# 7. Contagem hashes L0 actuais
grep -c "^hash:" 00_nucleo/prompts/L1/visualize/gradient.md
```

### Estrutura do diagnóstico

```
§A.1 Vanilla ConicGradient shape — campos exactos + tipos.
§A.2 Vanilla stdlib `gradient.conic(...)` — assinatura + named args.
§A.3 Cristalino Gradient enum estado — variants actuais (Linear, Radial).
§A.4 Cristalino stdlib actual — paridade com vanilla (linear, radial).
§A.5 Gap a fechar — lista literal de itens a materializar.
§A.6 Cenário detectado — B1 (fecho conceptual) / B2 (sub-passos) / outro.
§A.7 Cobertura empírica pré-P267 cluster Gradient — pp citado vs factual.
§A.8 Decisão arquitectural — minimalista (subset variants comentadas reserva) vs completo.
```

### Critério paridade

`ConicGradient` campos esperados (a confirmar Fase A):
- `stops: Vec<(Color, Ratio)>`
- `angle: Angle` (ângulo inicial; default 0deg)
- `center: Axes<Ratio>` (default 50%/50%)
- `space: ColorSpace` (interpolação; default Oklab)
- `relative: Option<Smart<RelativeTo>>` (default Smart::Auto)

Campos opcionais vanilla a marcar **scope-out** literal:
- `focal_center` / `focal_radius` (não aplicáveis a Conic; só Radial).

---

## §2 — Sub-passo .B — ADR

Criar **ADR-0089** PROPOSTO → IMPLEMENTADO mesmo passo (sub-padrão N=5 cumulativa após este; ADR-0086 a ADR-0088 + este).

### ADR-0089 título

`ADR-0089 — Gradient Conic-only L1+stdlib (fecha cluster 3/3 variants quanto a representação)`

### Estrutura

```
Status: PROPOSTO → IMPLEMENTADO (mesma transição P267).
Contexto: ADR-0087 (Linear) e ADR-0088 (Radial) materializaram 2/3 variants. Conic restante para fechar cluster representacional. PDF /ShadingType 3 Conic adiado para P268 (análogo divisão P262/P263 e P264/P265).
Decisão:
  1. `Gradient::Conic(ConicGradient)` activado em L1.
  2. Stdlib `gradient.conic(stops, angle: 0deg, center: (50%, 50%), space: oklab, relative: auto)`.
  3. Interpolação Oklab default (paridade ADR-0087 e ADR-0088).
  4. PDF rendering scope-out preserved (P268 candidato).
  5. `focal_*` permanece scope-out literal (Radial-only).
Consequências:
  + Cluster Gradient L1 completo 3/3.
  + Anotação cumulativa ADR-0054 perfil graded DEBT-1.
  + ADR-0088 §"variants não materializados" parcialmente revogada (Conic sai; focal_* preserva).
Anotação cumulativa P268 reservada (PDF Conic shading).
```

### Reconciliação L0

Actualizar `00_nucleo/prompts/L1/visualize/gradient.md`:
- Secção `## Variants` adicionar `### Conic` com campos + defaults.
- Hash actualizado; `crystalline-lint` deve passar zero violations.

---

## §3 — Sub-passo .C — Materialização condicional (testes primeiro)

### Ordem literal

1. **L0 prompt actualizado** primeiro (§2 reconciliação).
2. **Hashes corrigidos** via `crystalline-lint --fix-hashes`.
3. **Testes-primeiro** — adicionar ~30-40 testes Conic antes de qualquer LOC L1.
4. **L1 código** após testes vermelhos confirmados.
5. **Stdlib bindings** após L1 verde.
6. **Verificação final** — `cargo test` workspace + `crystalline-lint` zero.

### Cap LOC

- L1: ≤ 350 LOC (`gradient.rs` + helpers Oklab interpolação se ausentes; reutilizar de P262/P264 se já existirem).
- Stdlib: ≤ 80 LOC (`library/visualize/gradient.rs` `conic` function + named args).
- Testes: ≤ 40 novos.

### Estrutura testes esperada

```
- Construction: `Gradient::Conic(ConicGradient { ... })` válido com defaults.
- Stops validação: ≥ 2 stops; ratios crescentes 0..=1.
- Angle wrap: 0deg, 90deg, 360deg (= 0deg), -45deg (= 315deg).
- Center default 50%/50%; custom center.
- Interpolação Oklab amostra N=16 (paridade P263/P265 helpers).
- Stdlib `gradient.conic(red, blue)` — 2 stops shortcut.
- Stdlib `gradient.conic((red, 0%), (blue, 100%))` — explícito.
- Stdlib named args (angle, center, space, relative).
- Erro: < 2 stops; ratios fora 0..=1; ratios decrescentes.
- ColorSpace alternativos (srgb, hsl, oklab) — paridade P257.
```

---

## §4 — Sub-passo .D — Promoção ADR + README + relatório

1. **ADR-0089** PROPOSTO → IMPLEMENTADO inline (sub-padrão N=5).
2. **ADR-0054** anotação cumulativa P267 adicionada (cluster Gradient 3/3 L1+stdlib).
3. **ADR-0088** §"variants não materializados" actualizada: Conic riscado (struck-through), focal_* preservado.
4. **README.md** actualizar tabela cobertura Visualize (+~3pp esperados).
5. **Relatório** `00_nucleo/materialization/typst-passo-267-relatorio.md` produzido por Claude Code com:
   - Métricas finais (testes verdes workspace, hash drift, lint).
   - Cobertura Visualize empírica pós-P267 vs pré.
   - Sub-padrões aplicados + N cumulativo.
   - Pendências P268 reservadas (PDF Conic shading).
   - Decisão minimalista preservada — focal_* permanece scope-out.

---

## §política de paragem

Claude Code para e pergunta ao utilizador se qualquer das seguintes condições se verificar:

1. Fase A revela `ConicGradient` já parcialmente materializado em L1 cristalino (cenário inesperado).
2. Vanilla `ConicGradient` tem campos adicionais não previstos no §1 (ex.: campo novo não listado em `focal_*`).
3. Helpers Oklab interpolação N=16 de P263/P265 não existem ou não são reutilizáveis literal (cap LOC pode estourar).
4. Stdlib `gradient.conic` em vanilla tem named args adicionais não previstos (`angle`, `center`, `space`, `relative` esperados).
5. Crystalline-lint reporta violations não-triviais após reconciliação L0.
6. Testes-primeiro revelam ambiguidade arquitectural (ex.: angle wrap behavior; ratio inclusivo vs exclusivo nos extremos).
7. Cap LOC L1 (350) ou stdlib (80) ameaça ser ultrapassado.
8. Contagem testes novos ameaça ultrapassar 40 (indica scope creep).
9. ADR-0088 revogação parcial revela dependência cruzada não prevista (ex.: PDF exporter assume Linear/Radial só).
10. Cobertura empírica Visualize pós-P267 fica fora do intervalo esperado +2 a +5pp (indica mismatch entre citado e factual).

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N após P267 | Nota |
|-----------|-------------|------|
| Auditoria condicional (ADR-0084) | N=7 | P192A/P255/P257/P258/P259/P266/**P267** |
| Diagnóstico imutável (ADR-0085) | N=8 | + P267 |
| ADR PROPOSTO+IMPLEMENTADO mesmo passo | N=5 | + ADR-0089 |
| Decisão minimalista (subset + reserva) | N=5 | + Conic (focal_* preserva scope-out) |
| Dividir granularidade L1+stdlib / L3 dedicado | **N=4** | + P267+P268 |
| Anotação cumulativa em vez de ADR nova | N=4 preservado | ADR-0054 anotada; sem nova ADR |
| Auto-aplicação ADR-0065 inline | N=6 | + P267 |

### Sequência pós-P267

- **P268 PDF Conic shading** — sub-passo dedicado L3 (análogo P263/P265 literal). Reutiliza helpers Oklab N=16 amostragem de P263+P265. Magnitude S-M esperada.
- **P-Gradient-Focal** (M) — activa `focal_center` + `focal_radius` Radial; revoga ADR-0088 §focal_* scope-out.
- **Tiling pattern** (M-L) — `Paint::Tiling` activação; análogo estrutural P261+P262+P263.

### Sequência alternativa (caso utilizador prefira)

- ADR-0055bis variant-aware fonts (refino Text).
- P-Footnote-N (Model pendência).

---

## §referências cross-passos

- **P261** — `Paint::Solid` + `Paint::Gradient` wrapper (ADR-0086).
- **P262** — Gradient Linear L1+stdlib (ADR-0087).
- **P263** — PDF Linear shading /ShadingType 2 (anotação cumulativa ADR-0087).
- **P264** — Gradient Radial L1+stdlib (ADR-0088).
- **P265** — PDF Radial shading /ShadingType 3 (anotação cumulativa ADR-0088; helpers Oklab N=16).
- **P257** — Color 8/8 espaços + Oklab default interpolação (ADR-0083).
- **P266** — Audit Text Fase A (precedente metodológico imediato; sub-padrão auditoria condicional N=6 → N=7 com P267).

---

## §0.1 — Notas de execução para Claude Code

- Lê `lab/typst-original/crates/typst-library/src/visualize/gradient.rs` literal **antes** de escrever `ConicGradient` em cristalino.
- Reutiliza helpers Oklab interpolação N=16 de P263 + P265 literal — não duplicar.
- Stdlib bindings: paridade exacta com vanilla quanto a ordem de argumentos e defaults; verificar com `rg "fn conic" lab/typst-original/` antes.
- Se cap LOC ameaçar, parar via §política condição 7 antes de exceder.
- Relatório final inclui contagem testes verdes workspace (esperado ~2393 + ~30-40 = ~2423-2433).

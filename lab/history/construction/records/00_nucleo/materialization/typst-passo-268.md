# typst-passo-268 — PDF Conic shading (fecha cluster Gradient 3/3 L1+stdlib+PDF)

**Magnitude**: S-M (cap: ≤ 250 LOC L3 + ≤ 30 testes novos; expansão proporcional caso Fase A revele Type 6/7 patch).
**Cluster**: Visualize / Gradient / PDF export.
**Origem**: §11 relatório P267 candidato #1; promessa P267 explícita.
**Sequência**: P267 (L1+stdlib) → **P268 (PDF)** — fecha divisão granular sub-padrão N=3 cumulativo (P262+P263; P264+P265; P267+P268).
**Análogo estrutural canónico**: P263 (PDF Linear /ShadingType 2) + P265 (PDF Radial /ShadingType 3).
**Estratégia decidida**: **A — replicar vanilla literal** (Fase A decide entre Type 4/5/6/7 ou alternativa vanilla; spec mantém-se agnóstica até §A.2).

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L3 nunca antes de prompt L0 + ADR. Ordem: diagnóstico → ADR (ou anotação cumulativa) → prompt L0 → fix-hashes → testes-primeiro → código.
2. **ADR-0034 + ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-pdf-conic-passo-268.md` imutável. **Quarto consumo directo vanilla** (P262/P264/P267 precedentes).
3. **ADR-0084** (auditoria condicional). Se Fase A revelar que vanilla não materializa Conic em PDF (ou usa fallback raster), passo aborta sub-passo .C e converte-se em ADR de scope-out preserved.
4. **ADR-0089** (Gradient Conic-only L1+stdlib). P268 fecha promessa P267 §scope-out "PDF emit adiado P268".
5. **ADR-0087 + ADR-0088** preservadas literais (Linear axial /ShadingType 2 + Radial /ShadingType 3 intocados).
6. **Anotação cumulativa em vez de ADR nova** — anotar ADR-0089 com secção P268; **não** criar ADR-0090. Sub-padrão N=4 → N=5 cumulativo (P258.B/P259.B/P263/P265/**P268**).
7. **ADR-0054 perfil graded DEBT-1**: anotação cumulativa P268 (cluster Gradient agora 3/3 quanto a L1+stdlib+PDF). Status EM VIGOR preservado literal.
8. **ADR-0039 preservado**: `TextStyle.fill: Option<Color>` intocado.
9. **ADR-0086 preservado**: Paint wrapper Solid/Gradient intocado; P268 só toca emit path PDF.
10. **Crystalline-lint zero violations** obrigatório no fecho.
11. **Reutilização literal helpers Oklab** N=16 amostragem de P263+P265 quando aplicável; **sub-padrão N=2 → N=3 cumulativo**.
12. **Vanilla read-first autorizado**: `lab/typst-original/crates/typst-pdf/` consultado literal para decisão estratégica Type 4/5/6/7 ou alternativa.

---

## §1 — Fase A audit/diagnóstico empírico

Produz ficheiro imutável `00_nucleo/diagnosticos/diagnostico-pdf-conic-passo-268.md`.

### Comandos exactos a executar

```bash
# 1. Vanilla: como Conic é renderizado em PDF (decisão estratégica central)
rg -n "conic|Conic" lab/typst-original/crates/typst-pdf/src/ | grep -v "^Binary" | head -60

# 2. Vanilla: ShadingType usado para Conic (se algum)
rg -n "ShadingType|/ShadingType|shading_type" lab/typst-original/crates/typst-pdf/src/ | head -40

# 3. Vanilla: gradient rendering paths PDF (Linear vs Radial vs Conic)
rg -n "fn.*gradient|render_gradient|emit_gradient|pdf.*gradient" lab/typst-original/crates/typst-pdf/src/ | head -40

# 4. Vanilla: se Conic não tem ShadingType directo, procurar fallback (raster? tiling pattern? Type 4 mesh?)
rg -n "Conic|conic" lab/typst-original/crates/typst-pdf/src/gradient.rs 2>/dev/null || \
  rg -n "Conic|conic" lab/typst-original/crates/typst-pdf/src/ 2>&1 | head -40

# 5. Cristalino: estado actual export.rs Gradient::Conic fallback P267
rg -n "Conic" 03_infra/src/export.rs

# 6. Cristalino: emit Linear + Radial existentes (templates a replicar)
rg -n "emit.*[Ll]inear|emit.*[Rr]adial|ShadingType.*[23]" 03_infra/src/export.rs

# 7. Cristalino: helpers Oklab P263/P265 (reutilização literal)
rg -n "interpolate_oklab|color_to_oklab|sample_oklab|N=16|sample_16" 03_infra/src/

# 8. Contagem hashes L0 actuais entities/gradient.md
grep -c "^hash:" 00_nucleo/prompts/entities/gradient.md
```

### Estrutura do diagnóstico (§A.1 a §A.11)

```
§A.1 Vanilla typst-pdf paths gradient — listagem ficheiros relevantes.
§A.2 ESTRATÉGIA VANILLA CONIC PDF — decisão factual:
     - Opção identificada: Type 4 (free-form Gouraud) / Type 5 (lattice) /
       Type 6 (Coons) / Type 7 (tensor) / fallback raster / tiling pattern /
       outra.
     - Justificação: literal do vanilla (linhas + ficheiro citado).
§A.3 Vanilla parâmetros emit (N triangulação se Type 4; resolução se raster).
§A.4 Cristalino emit_linear (P263) — template estrutural.
§A.5 Cristalino emit_radial (P265) — template estrutural.
§A.6 Helpers Oklab N=16 existentes — assinatura + reutilização viável.
§A.7 Gap a fechar — lista literal de itens a materializar L3.
§A.8 Cenário detectado — B1 fecho conceptual / B2 sub-passos / outro.
§A.9 Magnitude empírica revisada — S-M confirmada ou expansão M-L se Type 6/7.
§A.10 Cobertura empírica pré-P268 Visualize PDF — pp.
§A.11 Decisão arquitectural — replicar vanilla literal (estratégia A).
```

### Critério de aceitação Fase A

- §A.2 identifica estratégia vanilla literal (não inferida).
- §A.7 lista no máximo 5-7 itens; se exceder, magnitude cap revisado §política condição 7.
- §A.11 não desvia da estratégia A escolhida pelo utilizador.

---

## §2 — Sub-passo .B — ADR

**Não criar ADR-0090**. Anotar ADR-0089 com secção cumulativa P268 (sub-padrão N=5 cumulativo).

### Anotação cumulativa em ADR-0089

Adicionar ao final de `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md`:

```
## Anotação cumulativa P268 — PDF Conic shading

**Data**: 2026-05-15.
**Promessa P267 fechada**: scope-out §"PDF emit adiado P268" → IMPLEMENTADO P268.

Estratégia vanilla literal replicada (Fase A §A.2 decidiu entre
Type 4/5/6/7 ou alternativa).

**Cluster Gradient L1+stdlib+PDF agora 3/3 completo**:
- Linear axial /ShadingType 2 (P263).
- Radial /ShadingType 3 (P265).
- Conic [estratégia decidida Fase A] (P268).

**Helpers Oklab N=16 amostragem** reutilizados literal de P263+P265
(sub-padrão N=2 → N=3 cumulativo).

**Pattern-match `Gradient::Conic(_) => continue / fallback Solid`** dos 3
sítios export.rs P267 substituído por emit real P268.

**Cluster Gradient extensões adicionais** preservadas scope-out
(focal_*, space custom, relative custom).
```

### Anotação cumulativa em ADR-0054

Adicionar linha:
```
P268 — cluster Gradient L1+stdlib+PDF agora 3/3 completo;
ADR-0054 perfil graded DEBT-1 cobertura cumulativa +~2pp Visualize PDF.
```

### Reconciliação L0

Actualizar `00_nucleo/prompts/entities/gradient.md`:
- Secção P267 (já existente) recebe linha "PDF emit P268: [estratégia Fase A]".
- Não criar prompt L3 novo; render path PDF é detalhe de infraestrutura, não L0 entity.

Hashes propagados; `crystalline-lint` zero violations.

---

## §3 — Sub-passo .C — Materialização condicional (testes primeiro)

### Ordem literal

1. **L0 anotação** (§2 reconciliação).
2. **ADR-0089 anotação cumulativa** P268.
3. **Hashes** via `crystalline-lint --fix-hashes`.
4. **Testes-primeiro** — adicionar ~20-30 testes PDF Conic antes de qualquer LOC L3.
5. **L3 código** — substituir 3 sítios pattern-match fallback P267 por emit real.
6. **Verificação final** — `cargo test` workspace + `crystalline-lint` zero.

### Cap LOC

- L3: ≤ 250 LOC em `03_infra/src/export.rs` (e helpers internos se Fase A justificar; reutilizar helpers Oklab P263/P265 literal).
- Testes: ≤ 30 novos.

### Estrutura testes esperada

Templates a replicar literal de P263 (Linear) + P265 (Radial):

```
- emit Conic shading dictionary: estrutura PDF válida.
- ShadingType correcto (4/5/6/7 ou alternativa) conforme §A.2.
- N stops Oklab amostrados (paridade P263/P265 N=16 se aplicável).
- Center custom (50%/50% vs outros).
- Angle inicial: 0deg / 90deg / 180deg / 270deg / -45deg.
- Stops mínimos: 2 stops válido.
- Stops múltiplos: 3-5 stops cobertura completa.
- ColorSpace interpolação: Oklab default (paridade P263/P265).
- Resource registo: `scan_all_gradients` regista Conic (deixa de ser `continue`).
- Pattern resources: `pattern_resources_for_page` regista Conic.
- Stroke paint: `emit_stroke_paint` Conic real (deixa de fallback Solid).
- Fill paint: análogo emit_fill_paint.
- Integration: documento mínimo com `gradient.conic(red, blue)` produz PDF válido.
- Snapshot bytes: hash bytes PDF reproduzível.
```

### Helpers reutilizáveis (paridade P263+P265)

Confirmar Fase A §A.6:
- `interpolate_oklab(c1, c2, t)` — interpolação par.
- `sample_oklab_n16(stops, n=16)` — pré-amostragem.
- `color_to_oklab_with_alpha`.
- `srgb_to_linear`, `linear_rgb_to_oklab`.

Se ausentes ou não reutilizáveis literal, §política condição 3 dispara.

---

## §4 — Sub-passo .D — Promoção + README + relatório

1. **ADR-0089** anotação cumulativa P268 fechada.
2. **ADR-0054** anotação cumulativa P268 adicionada.
3. **README.md** actualizar:
   - Tabela cobertura Visualize PDF (+~2pp esperados).
   - Entrada P268 ~30-40 linhas (paridade entrada P265).
   - Cross-reference ADR-0089 §anotação P268.
4. **Distribuição ADRs preservada** — total 76 mantido (sem ADR nova).
5. **Relatório** `00_nucleo/materialization/typst-passo-268-relatorio.md` com:
   - Métricas finais (testes verdes; esperado ~2407 + ~20-30 = ~2427-2437).
   - Hash drift zero; lint zero.
   - Cobertura Visualize PDF pós-P268.
   - Sub-padrões aplicados + N cumulativo.
   - **Cluster Gradient 3/3 L1+stdlib+PDF completo** — marco arquitectural.
   - Pendências reservadas (focal_*, space custom, relative custom).
   - 3 sítios export.rs já não têm fallback Conic.

---

## §política de paragem

Claude Code para e pergunta se qualquer das seguintes condições ocorrer:

1. **Fase A §A.2 revela que vanilla não materializa Conic em PDF** (usa raster fallback, ou Conic só funciona em SVG export). Cenário força conversão para ADR de scope-out preserved.
2. Estratégia vanilla identificada é **Type 6 (Coons) ou Type 7 (tensor)** que exigem patches paramétricos curvos — cap LOC 250 ameaça estourar; magnitude revisada para L.
3. Helpers Oklab N=16 de P263+P265 não existem ou não são reutilizáveis literal.
4. Vanilla PDF Conic depende de crates externos não autorizados (verificar ADR-0018 lista autorizada).
5. Crystalline-lint reporta violations não-triviais após anotação L0.
6. Testes-primeiro revelam ambiguidade (ex.: N triangulação default; precisão Oklab; gestão angle wrap em rendering).
7. Cap LOC L3 (250) ou testes (30) ameaça ser ultrapassado.
8. Pattern-match expand revela call sites adicionais não previstos nos 3 P267 conhecidos.
9. Snapshot bytes PDF não reproduzíveis (indica dependência float não-determinística).
10. Cobertura empírica Visualize PDF pós-P268 fica fora intervalo +1 a +3pp.
11. Integration test `gradient.conic(red, blue)` produz PDF que renderer externo (pdftoppm / mupdf) não consegue parsear.

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N após P268 | Nota |
|-----------|-------------|------|
| Auditoria condicional (ADR-0084) | N=8 | + P268 |
| Diagnóstico imutável (ADR-0085) | N=9 (quarto consumo directo vanilla) | + P268 (P262/P264/P267/**P268**) |
| Anotação cumulativa em vez de ADR nova | **N=5** | + P268 anotada ADR-0089 |
| Reutilização literal helpers cross-passos | **N=3** | + P268 (helpers Oklab P263+P265) |
| Dividir granularidade L1+stdlib / L3 dedicado | **N=3** completo | P262+P263; P264+P265; **P267+P268** |
| Auto-aplicação ADR-0065 inline | N=7 | + P268 |
| Refactor cross-cutting entity primitivo | N=4 preservado | (Gradient já era cross-cutting P263/P265) |

### Marco arquitectural P268

**Cluster Gradient L1+stdlib+PDF agora 3/3 completo**:

| Variant | L1 | Stdlib | PDF |
|---------|----|----|-----|
| Linear | P262 | P262 | P263 |
| Radial | P264 | P264 | P265 |
| **Conic** | **P267** | **P267** | **P268** |

Promessa P267 fechada. Cluster Gradient encerrado quanto a 3 variants base.

### Sequência pós-P268

- **P-Gradient-Focal** (M; activa focal_* Radial; revoga ADR-0088 §focal scope-out).
- **P-Gradient-Space-Custom** (S+; activa `space: ColorSpace` cross-variant; revoga Oklab fixo).
- **P-Gradient-Relative-Custom** (M; activa `relative: RelativeTo`).
- **ADR-0055bis variant-aware fonts** (M; refino Text).
- **P-Footnote-N** (M; Model pendência).
- **DEBT-33 Bézier bbox** + outros Visualize.

---

## §referências cross-passos

- **P267** — Gradient Conic L1+stdlib (ADR-0089 PROPOSTO+IMPLEMENTADO; precedente directo).
- **P263** — PDF Linear /ShadingType 2 (template emit).
- **P265** — PDF Radial /ShadingType 3 (template emit; helpers Oklab N=16).
- **P262** — Gradient Linear L1+stdlib (ADR-0087).
- **P264** — Gradient Radial L1+stdlib (ADR-0088).
- **P261** — Paint::Gradient wrapper (ADR-0086).
- **P257** — Color 8/8 espaços (ADR-0083; Oklab interpolação default).
- ADR-0084 + ADR-0085 (auditoria condicional + diagnóstico imutável).

---

## §0.1 — Notas de execução para Claude Code

- **Fase A é decisão estratégica central** — §A.2 não é negociável; identifica literalmente como vanilla materializa Conic em PDF. Se ambíguo, §política condição 1 dispara antes de qualquer LOC.
- **Reutilizar helpers Oklab P263+P265 literal** — não duplicar. Verificar §A.6 antes.
- **3 sítios export.rs P267** (`scan_all_gradients` linha 364-373; `pattern_resources_for_page` linha 414; `emit_stroke_paint` linha 1140-1146) substituídos por emit real; **+ verificar se existe `emit_fill_paint` análogo não tocado P267**.
- Snapshot bytes PDF determinísticos (sem float não-reproduzível em angle/center).
- Integration test `gradient.conic(red, blue)` deve produzir PDF parseável por pdftoppm ou mupdf (cap §política 11).
- Relatório final esperado: ~2407 + 20-30 = ~2427-2437 testes verdes.
- Distribuição ADRs preservada total 76 (sem ADR nova; só anotação cumulativa).

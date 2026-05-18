# typst-passo-278 — Cleanup XS+S combinado (3 alvos: L0 update + helper bbox + Group catch-all)

**Magnitude**: passo combinado XS+S (cap LOC L0 ~10; cap LOC L3 hard 150 / soft 100; cap testes hard 60 / soft 40).
**Cluster**: Cleanup / Cluster Gradient residual / Fix funcional.
**Origem**: relatório P275 §7 Cenário A; sequência humana confirmada 2026-05-18 — "Vamos resolver os DEBTs. (...) → P278 cleanup XS combinado"; relatório P277 §6 confirma sequência.
**Tipo**: passo principal P278 — três sub-operações independentes mas atómicas no mesmo passo (cleanup orgânico).
**Sequência**: P275 (auditoria) → P276 (DEBT-35b OBSOLETED) → P277 (DEBT-33 CLOSED) → **P278 (cleanup combinado)** → P279+ (próximo DEBT a decidir).
**Estratégia decidida**: agrupar 3 pendências cluster Gradient residuais (P273.X-bis-*) num passo cleanup, dado que cada uma sozinha não justifica passo dedicado. **Item 3 é S (fix funcional, não cosmético)**.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: zero alterações ADR-novo; código L3 com cobertura testes. Sub-operações ordenadas: L0 update → helper L3 → fix funcional Group.

2. **ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-cleanup-passo-278.md` imutável. **32º consumo** (continuação P277 N=36; 31º consumo).

3. **NÃO criar ADRs novas** — 3 sub-operações são refinos de código existente, não decisões arquitecturais.

4. **ADR-0029 pureza física L1** preserved — sub-operações 2 e 3 são em L3 (export.rs); sub-operação 1 é L0. Zero alteração L1.

5. **ADR-0094 Pattern 1 cap LOC** aplicado por sub-operação:
   - Sub-op 1 (content-md update): hard 10 LOC L0 / soft 7.
   - Sub-op 2 (helper-group-bbox): hard 30 LOC L3 add / hard -20 LOC L3 remove (net negativo ~10 LOC).
   - Sub-op 3 (draw-item-local Text+Image): hard 80 LOC L3 / soft 50.
   - **Combinado**: hard 150 / soft 100 LOC L3 líquido.

6. **Crystalline-lint zero violations** obrigatório. Hash L0 propagado para `content.md`.

7. **Tests workspace 2652 → 2660-2680** esperado (~8-25 testes novos: 0 testes na sub-op 1; ~3-5 na sub-op 2; ~5-15 na sub-op 3).

8. **Pattern P277 §3.3 "consolidação via helper criado"** já aplicado parcialmente — `path_bbox` consumida por `polygon()`. P278 sub-op 2 estende o pattern para 2 outros sítios. **Sub-padrão "Extract helper de replicação inline" N=2 → N=3 cumulativo** se sub-op 2 materializar. **Limiar formalização N≥3-4 atingido** → considerar registar (não ADR; nota no §5 do relatório).

9. **Atomicidade do passo**: as 3 sub-operações são **independentes mas executadas no mesmo passo**. Se uma falhar (e.g. teste novo da sub-op 3 não passa), apenas essa sub-operação é revertida; as outras 2 podem persistir. Cada uma tem o seu critério de fecho.

10. **Caps documentais** (ADR-0094 Pattern 1):
    - Diagnóstico Fase A: hard 600 / soft 400.
    - Relatório consolidado: hard 1000 / soft 700.

---

## §1 — Sub-passo P278.A — Fase A diagnóstico empírico

Produz `00_nucleo/diagnosticos/diagnostico-cleanup-passo-278.md`.

### §A.1 — Sub-op 1: content-md-debt56-update — auditoria das referências literais

**Origem**: P273.16 originalmente estimou ~1 LOC L0 (linha 824); auditoria P275 §4.2 revisou para **~5 LOC L0** detectando 5 ocorrências em linhas 283/436/686/796/824.

Verificar empíricamente:

```bash
# 1. Conta TODAS as ocorrências de "DEBT-56" em content.md
rg -n "DEBT-56" 00_nucleo/prompts/entities/content.md

# 2. Conta ocorrências em todos os L0 (sanity)
rg -n "DEBT-56" 00_nucleo/prompts/

# 3. Verificar texto literal de cada uma — algumas podem ser históricas (a preservar)
rg -n -A 2 "DEBT-56" 00_nucleo/prompts/entities/content.md
```

**Pergunta crítica para §A.1**: cada referência é (a) **factualmente desactualizada** (fala de DEBT-56 como aberto) ou (b) **histórica** (regista quando foi aberto/fechado, a preservar)?

**Output §A.1**: tabela por linha:

| Linha | Texto literal | Tipo | Acção |
|---|---|---|---|
| 283 | ... | (factual/histórico) | (update/preserve) |
| 436 | ... | ... | ... |
| 686 | ... | ... | ... |
| 796 | ... | ... | ... |
| 824 | ... | ... | ... |

Só **factuais desactualizadas** são tocadas. Históricas preservadas per pattern P201/P202.

### §A.2 — Sub-op 2: helper-group-bbox — verificação empírica de sítios remanescentes

**Origem**: P273.13 §9 identificou 3 sítios replicados:
- `scan_all_gradients.walk` (`03_infra/src/export.rs`).
- `pattern_resources_for_page.walk` (`03_infra/src/export.rs`).
- `draw_item_local` (`03_infra/src/export.rs`).

**Estado pós-P277**: relatório P277 §3.3 reportou que `polygon()` em `01_core/src/rules/stdlib/shapes.rs` foi consolidado para usar `path_bbox()` — mas isso é **outro helper, em outra camada** (L1, não L3) e cobre **outro pattern** (Path bbox, não Group bbox). **Sub-op 2 fica intocada por P277**.

Verificar empíricamente as 3 replicações em L3:

```bash
# Localizar funções
rg -n "fn scan_all_gradients|fn pattern_resources_for_page|fn draw_item_local" \
   03_infra/src/export.rs

# Inspeccionar corpo cada uma — procurar pattern Group bbox calculation
rg -n -A 20 "FrameItem::Group" 03_infra/src/export.rs | head -150

# Localizar duplicação concreta
rg -n "items.iter().*bbox|bbox.*Group|Group.*bbox" 03_infra/src/export.rs
```

**Output §A.2**: 3 blocos de código com replicação confirmada (ou negativa). Se algum sítio já foi alterado entre P273.13 e P278 (improvável; não há registo de passo intermédio), reformular.

**Critério para extracção**: helper extraível com signature provavelmente `fn group_bbox(items: &[FrameItem]) -> (f64, f64, f64, f64)` ou similar, retornando bbox dos items contidos num Group. Localização: módulo privado em `export.rs` ou helper pub(crate) em local apropriado.

### §A.3 — Sub-op 3: draw-item-local-text-image — auditoria do catch-all

**Origem**: P273.13 §9 segundo bullet — *"Text + Image em Groups silenciosamente descartados via `_ => {}` catch-all em `draw_item_local`"*.

Verificar empíricamente:

```bash
# 1. Localizar draw_item_local + match catch-all
rg -n -B 2 -A 30 "fn draw_item_local" 03_infra/src/export.rs

# 2. Procurar match arm explícito
rg -n "_ => \{\}|_ => \(\)" 03_infra/src/export.rs

# 3. FrameItem variants existentes — verificar quais estão cobertos
rg -n "pub enum FrameItem" 01_core/src/entities/layout_types.rs -A 30
```

**Verificar invariante**: o catch-all está a descartar **silenciosamente**. Isto é **bug funcional** — se Text ou Image aparecer dentro de Group (e.g. via Container com transform), o conteúdo não é emitido no PDF.

**Pergunta para §A.3**: quais variantes de `FrameItem` o match cobre e quais ficam para o catch-all? Inventário literal.

**Output §A.3**: tabela:

| FrameItem variant | Arm explícito em `draw_item_local`? | Comportamento actual |
|---|---|---|
| Text | (sim/não) | (delegado/descartado) |
| Glyph | ... | ... |
| Line | ... | ... |
| Image | ... | ... |
| Shape | ... | ... |
| Group | ... | ... |

**Reprodução do bug**: criar exemplo `.typ` minimal onde Text dentro de Group transformado **não** apareça no PDF (e.g. `rotate(45deg, [hello])`). Confirmar bug empíricamente antes de fix.

### §A.4 — Estado workspace baseline

Verificar:

```bash
cargo test --workspace 2>&1 | grep "test result"
# Esperado: 2652 passed (baseline P277)

cargo run -p crystalline-lint --quiet
# Esperado: ✓ No violations found
```

### §A.5 — Casos de teste planeados

**Sub-op 1**: zero testes (alteração L0 documental).

**Sub-op 2**: ~3-5 testes (regressão dos 3 sítios para garantir refactoring não alterou comportamento):
- `p278_scan_all_gradients_preserva_bbox_pos_helper`.
- `p278_pattern_resources_for_page_preserva_bbox_pos_helper`.
- `p278_draw_item_local_preserva_bbox_pos_helper`.
- Opcional: `p278_group_bbox_helper_directo` (test unit do helper isolado).

**Sub-op 3**: ~5-15 testes (fix bug + regressão):
- `p278_text_em_group_aparece_no_pdf` — Text dentro de Group emite no PDF.
- `p278_image_em_group_aparece_no_pdf` — Image dentro de Group emite no PDF.
- `p278_glyph_em_group_continua_funcionar` — Glyph (caminho B CIDFont) preserved.
- `p278_line_em_group_continua_funcionar` — Line preserved.
- `p278_shape_em_group_continua_funcionar` — Shape preserved.
- `p278_group_nested_em_group_funciona` — Group dentro de Group (recursão).
- Adicionais conforme variantes cobertas pelo match.

**Estimativa total**: 8-15 testes novos. Cap testes hard 60 confortável.

### §A.6 — Gates de paragem (§política condição)

Disparam paragem antes de §C:

1. **§A.1 detecta 0 ocorrências factuais** — sub-op 1 OBSOLETA; pular para sub-op 2.
2. **§A.1 detecta >10 ocorrências factuais** — magnitude L0 maior que estimado; reformular sub-op (extracted para passo dedicado).
3. **§A.2 detecta 0 ou 1 sítio replicado** (i.e. helper já foi materializado entre P273.13 e P278) — sub-op 2 OBSOLETA; verificar com humano antes de prosseguir.
4. **§A.3 não consegue reproduzir bug Text+Image** (e.g. arm explícito foi adicionado entretanto) — sub-op 3 OBSOLETA; consolidar.
5. **§A.3 revela que catch-all não é literal `_ => {}` mas tem lógica intencional** — sub-op 3 reformulada; pode não ser bug.
6. **Tests workspace ≠ 2652** baseline — regressão pré-existente; investigar.
7. **Cap LOC L3 hard 150 ameaçado** — reformular passo (e.g. extrair sub-op 3 para P279 dedicado).
8. **Cap doc Fase A hard 600 ameaçado** — reformular.

**Em qualquer gate disparado**: o passo continua com as sub-operações restantes (atomicidade per §0 ponto 9). A sub-operação afectada é reportada como OBSOLETA ou reformulada no relatório §3.

---

## §2 — Sub-passo P278.B — Anotação cumulativa (condicional)

**Default**: §2 não aplicado — refinos de código sem decisão arquitectural nova.

**Excepção**: se sub-op 2 materializar, sub-padrão "Extract helper de replicação inline" atinge **N=3 cumulativo** (P273.11 P276.intra + P277 implícito via `path_bbox`-`polygon` + P278 sub-op 2). N=3 atinge limiar formalização. Decisão:

- **Opção A (recomendada)**: registar em §5 do relatório como sub-padrão consolidado N=3, **sem formalizar ADR** (per anti-padrão over-formalização P273.17 §0).
- **Opção B**: formalizar ADR nova "ADR-XXXX: Extract helper de replicação inline em refactorings de cleanup" — **rejeitar** por anti-padrão (pattern é demasiado óbvio para merecer ADR; documenta-se em si mesmo na história git).

Spec aplica Opção A. §2 do relatório regista: "B sub-passo aplicado parcialmente — sub-padrão 'Extract helper de replicação inline' atinge N=3 cumulativo em sub-op 2; registado em §5 sem formalização ADR."

---

## §3 — Sub-passo P278.C — Materialização (ordem definida)

Ordem das sub-operações: **C.1 → C.2 → C.3** (mais simples para mais complexa).

### §C.1 — Sub-op 1: content-md-debt56-update (~5 LOC L0)

Para cada linha factual (§A.1):

1. Substituir referência "DEBT-56 EM ABERTO" / "DEBT-56 pendente" / etc. por referência ao estado real (e.g. "DEBT-56 (encerrado P221 — column flow Layout Fase 3)" ou simples remoção se a frase não fizer mais sentido).
2. Preservar referências históricas — não tocar linhas que falam do *contexto* de quando DEBT-56 foi aberto/fechado.

Aplicar `crystalline-lint --fix-hashes .` para propagar hash de `content.md`.

**Validação C.1**:

```bash
cargo test --workspace 2>&1 | grep "test result"
# Esperado: 2652 passed (zero código tocado; sub-op é L0 puro)

cargo run -p crystalline-lint --quiet
# Esperado: ✓ No violations found
```

### §C.2 — Sub-op 2: helper-group-bbox (extracção L3)

#### §C.2.1 — Adicionar helper privado em `03_infra/src/export.rs`

```rust
/// Calcula bbox dos items dentro de um Group.
/// Helper extraído em P278 para consolidar 3 replicações em
/// scan_all_gradients, pattern_resources_for_page e draw_item_local.
fn group_bbox(items: &[FrameItem]) -> (f64, f64, f64, f64) {
    // Algoritmo idêntico ao que estava replicado nos 3 sítios.
    // (signature exacta a confirmar pela Fase A — pode usar Pt, Point, etc.)
    ...
}
```

#### §C.2.2 — Substituir nos 3 sítios

Cada sítio elimina o cálculo inline e chama `group_bbox(items)`. Net LOC esperado: **-15 a -20 linhas** (replicação eliminada) + **~10 linhas adicionadas** (helper). Net negativo confirma cleanup genuíno.

#### §C.2.3 — Testes regressão (sub-op 2)

3-5 testes confirmando que o comportamento dos 3 sítios é bit-exact após a extracção. Comparar bytes do PDF antes/depois para um documento que exerça cada caminho.

#### §C.2.4 — Validação C.2

```bash
cargo test --workspace 2>&1 | grep "test result"
# Esperado: 2652 + ~3-5 = 2655-2657 passed

cargo run -p crystalline-lint --quiet
# Esperado: ✓ No violations found
```

### §C.3 — Sub-op 3: draw-item-local-text-image (fix funcional)

#### §C.3.1 — Inventário do match

Pegar lista §A.3 — variantes cobertas vs catch-all. Para cada variante actualmente no catch-all (esperado: Text e Image, possivelmente outras), adicionar arm explícito que delega à função de desenho apropriada.

#### §C.3.2 — Implementação dos arms

Para `FrameItem::Text { pos, text, style }` dentro de Group: invocar a mesma lógica de desenho de Text que o caminho top-level usa, com offset relativo ao Group. Análogo para `FrameItem::Image { pos, data, width, height, .. }`.

Verificar:
- Coordenadas locais (relative ao Group transform) vs absolutas (page). Inverter eixo Y dentro do Group conforme convenção do exportador (PDF y=0 inferior; Cristalino y=0 topo).
- `q`/`Q` (push/pop state graphics) preservado.
- Imagem dedup (`image_resources` map) reutilizada — não duplicar XObject.

#### §C.3.3 — Eliminar `_ => {}` catch-all

Substituir por **match exaustivo** (per pattern emergente "match exaustivo sem fall-through" — análogo a `is_locatable` L0). Isto força revisão se novo variant for adicionado a `FrameItem`.

Alternativa aceitável: manter `_ => {}` mas adicionar comentário `// FrameItem variants explicitly unhandled at Group-local level: <list>` documentando a decisão.

**Recomendação**: match exaustivo (mais robusto contra regressões futuras).

#### §C.3.4 — Testes (sub-op 3)

5-15 testes conforme §A.5 — pelo menos:
- Text em Group emite PDF (regressão do bug).
- Image em Group emite PDF (regressão do bug).
- Glyph/Line/Shape preserved (sanidade).
- Group nested funciona (recursão).

#### §C.3.5 — Validação C.3

```bash
cargo test --workspace 2>&1 | grep "test result"
# Esperado: ~2660-2670 passed

cargo run -p crystalline-lint --quiet
# Esperado: ✓ No violations found
```

### §C.4 — Actualização DEBT.md

**Não é fecho de DEBT** — sub-operações cleanup. DEBT.md cabeçalho recebe linha cumulativa:

```markdown
> **Passo 278 (2026-05-XX)**: cleanup combinado pós-cluster Gradient.
> 3 sub-operações: (1) ~5 LOC L0 content.md actualizado DEBT-56;
> (2) helper `group_bbox` extraído consolidando 3 replicações L3;
> (3) Text+Image em Group fix funcional (catch-all silencioso
> corrigido). Sub-padrão "Extract helper de replicação inline"
> N=3 cumulativo (não-formalizado). Tests: 2652 → ~2660-2670.
```

**Pendências cluster Gradient residuais** §3 da transição P273.17:
- P273.X-bis-helper-group-bbox: ✓ fechada em P278 sub-op 2.
- P273.X-bis-content-md-debt56-update: ✓ fechada em P278 sub-op 1.
- P273.X-bis-draw-item-local-text-image: ✓ fechada em P278 sub-op 3.

Cluster Gradient **encerrado em todos os planos**: principal P273.17, residual P278.

### §C.5 — Relatório consolidado

Produz `/mnt/user-data/outputs/typst-passo-278-relatorio.md`. Estrutura:

- §1 — Validação contra spec (tabela critérios §7).
- §2 — Resumo factual por sub-operação:
  - §2.1 — Sub-op 1 resultado.
  - §2.2 — Sub-op 2 resultado.
  - §2.3 — Sub-op 3 resultado.
- §3 — Operações realizadas (lista detalhada).
- §4 — Sub-padrões emergentes:
  - "Extract helper de replicação inline" N=3 → registado N=3 cumulativo (sem ADR).
  - "Match exaustivo sem fall-through em export.rs" — N=? (verificar precedente).
- §5 — Métricas (tabela pré/pós).
- §6 — Cluster Gradient encerrado em todos os planos.
- §7 — Próximos passos: P279+ decisão humana.
- §8 — Referências cross-passos.

---

## §4 — Caps e gates de protecção

- **LOC L0**: hard 10 / soft 7 (sub-op 1).
- **LOC L3**: hard 150 / soft 100 combinado (sub-ops 2+3).
- **LOC testes**: hard 60 / soft 40 combinado.
- **Modificações `.rs`**: `03_infra/src/export.rs` (sub-ops 2+3). Sem outras alterações.
- **Modificações L0**: `prompts/entities/content.md` (sub-op 1; hash propagado).
- **Tests workspace**: 2652 → 2660-2670 esperado.
- **Lint**: zero violations preserved.

---

## §5 — Sub-padrões esperados aplicados

- **Extract helper de replicação inline** — N=2 (P273.11 + P277 implícito) → **N=3 cumulativo** se sub-op 2 materializar. Limiar formalização atingido; **decisão**: registar em §5 do relatório, NÃO formalizar ADR (anti-padrão over-formalização).
- **Match exaustivo sem fall-through** — pattern arquitectural já presente em `is_locatable` (L1 introspect/locatable.rs). Aplicação em L3 (export.rs `draw_item_local`) é primeira incidência fora L1. N=1 cumulativo em L3 (reuso de pattern cross-layer).
- **Cleanup combinado em passo único** — N=1 inaugural P278 (3 sub-operações atómicas mas independentes).
- **Diagnóstico imutável** — N=36 → N=37 cumulativo (32º consumo).
- **Pattern P273.X-bis (pendência cluster preserved)** — fecho N=3 simultâneo (todas as 3 candidatas P273.X-bis fecham em P278).

---

## §6 — Workflow operacional

1. Utilizador upload literal `00_nucleo/DEBT.md` + `00_nucleo/prompts/entities/content.md` + opcionalmente excerpts relevantes de `03_infra/src/export.rs`.
2. Claude Code executa Fase A:
   - Produz `typst-passo-278A-diagnostico.md` em `/mnt/user-data/outputs/`.
   - §A.1 tabela linhas DEBT-56.
   - §A.2 verificação 3 sítios Group bbox.
   - §A.3 inventário match `draw_item_local` + reprodução bug.
3. Utilizador valida Fase A (gates §A.6).
4. Claude Code executa §C em ordem C.1 → C.2 → C.3:
   - Cada sub-operação verificada independentemente.
   - Tests workspace correm após cada sub-op (não só no final).
   - Se uma sub-op falha gate ou teste, regista falha mas continua com as restantes.
5. Edita `DEBT.md` (§C.4 cabeçalho).
6. Produz `typst-passo-278-relatorio.md` (§C.5).
7. Utilizador valida relatório.
8. Próximo passo: **P279** — decisão humana entre DEBTs restantes (DEBT-50 / DEBT-43 / outros).

---

## §7 — Critério de fecho

P278 fecha (parcial ou totalmente) quando:

- [ ] Fase A produzida; §A.1/A.2/A.3 preenchidos empíricamente.
- [ ] Sub-op 1: content.md actualizado nas linhas factuais; hash L0 propagado.
- [ ] Sub-op 2: helper `group_bbox` extraído; 3 sítios consolidados; tests regressão verdes.
- [ ] Sub-op 3: Text+Image arm adicionados; catch-all eliminado ou documentado; bug reproduzido **e** corrigido pelos testes.
- [ ] DEBT.md cabeçalho com linha P278.
- [ ] Tests workspace 2652 → ≥2660 (mínimo +8 com sub-op 3 completa).
- [ ] Lint zero violations.
- [ ] Cap LOC L3 hard 150 respeitado.
- [ ] Relatório consolidado §1-§8 completos.

**P278 fecha PARCIAL** se alguma sub-operação disparar gate §A.6 e for marcada OBSOLETA — o passo continua com as restantes; relatório regista falha individual.

**P278 NÃO fecha** se:
- Regressão tests baseline 2652 não causada por sub-operação esperada.
- Lint não-zero não justificado.
- Sub-op 3 introduzir bug nova (e.g. duplicação de Image XObject).

---

## §8 — Referências cross-passos

- **P79** — origem cluster Visualize.
- **P273.13 §9** — origem das 3 pendências XS/S (`helper-group-bbox` e `draw-item-local-text-image`).
- **P273.16** — origem `content-md-debt56-update` (originalmente ~1 LOC; revisado para ~5 LOC em P275).
- **P273.17** — encerramento cluster Gradient principal.
- **P275 §4.2/4.3/4.4** — acções de manutenção propostas; este passo executa as 3.
- **P275 §7** — cenário A recomendado; este passo é P278 desse cenário.
- **P276** — DEBT-35b OBSOLETED (precedente metodológico imediato).
- **P277** — DEBT-33 CLOSED; consolidação `path_bbox`-`polygon` (sub-padrão N=2 estabelecido).
- **P277 §3.3** — consolidação `polygon()` para usar `path_bbox()` (precedente directo do pattern "Extract helper").
- **ADR-0029** — Pureza física L1 (preserved; sub-ops em L3/L0).
- **ADR-0094** — Meta-operacional specs (Pattern 1 caps LOC; aplicado per sub-op).
- **ADR-0085** — Diagnóstico imutável (32º consumo).
- **`is_locatable`** (L1) — precedente match exaustivo sem fall-through.

---

## §9 — Notas de execução para Claude Code

- **Ordem de execução obrigatória**: C.1 → C.2 → C.3. Sub-ops independentes mas a ordem respeita complexidade crescente.
- **Verificar tests workspace após cada sub-op** (não só no final). Isolamento de regressões facilita debug.
- **Sub-op 1** é zero risco (L0 documental); 5 min execução.
- **Sub-op 2** é refactoring puro; testes regressão devem **passar bit-exact** — qualquer diferença em bytes PDF indica bug introduzido no helper.
- **Sub-op 3** é **fix funcional**; o teste `p278_text_em_group_aparece_no_pdf` deve **falhar antes** da implementação e **passar depois**. Esta inversão é a evidência do bug corrigido.
- **NÃO duplicar XObject de Image** em sub-op 3 — reusar `image_resources` dedup map per ADR existente em `export.md` L0.
- **Anti-padrão a evitar**: NÃO criar ADR para o helper `group_bbox` — é refactoring mecânico. Per anti-padrão over-formalização P273.17 §0.
- **Outputs**: 2 ficheiros em `/mnt/user-data/outputs/` (`typst-passo-278A-diagnostico.md` + `typst-passo-278-relatorio.md`).
- **Tempo estimado**: 90-150 min (passo combinado; sub-op 3 domina).
- **Confirmação visual final**: `rg "P278" 00_nucleo/DEBT.md` deve mostrar linha cumulativa no cabeçalho.

---

*Spec produzida em 2026-05-XX como passo de cleanup combinado pós-fechos DEBT (P276 OBSOLETED + P277 CLOSED). 3 sub-operações atómicas mas independentes: actualização L0 content.md (sub-op 1 XS), extracção helper Group bbox L3 (sub-op 2 XS), fix funcional Text+Image em Group (sub-op 3 S). Cluster Gradient encerra em todos os planos (principal P273.17 + residual P278). Próximo passo P279+ decisão humana entre DEBTs restantes.*

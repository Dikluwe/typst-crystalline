# Diagnóstico Math — Passo 254B

**Data**: 2026-05-15
**Tipo**: passo arquitectural de diagnóstico (não materializa código)
**Estrutura**: duas fases registadas no mesmo trabalho:
- **Fase A** — inventário empírico das 4 pendências DEBT-8 listadas
  desde 2026-03 (Passo 40), comparadas com evidência cumulativa
  pós-P96.8 + P184-P199.
- **Fase B** — decisão condicional: fecho DEBT-8 (padrão P192A) se
  4/4 pendências caíram, ou diagnóstico Fase 1 minimal (padrão
  P160) se restar pendência real.

**Análogo estrutural**: P192A (auditoria M7 estruturalmente fechado
sem ADR explícita) + P160 (diagnóstico Introspection com
recomendação).

---

## §1 — ADRs e DEBTs relevantes

| ADR/DEBT | Status | Relevância |
|----------|--------|------------|
| ADR-0009 | PROPOSTO | `MathClass` + `default_math_class` → L1 |
| ADR-0011 | (referência) | Delegação `unicode-math-class` |
| ADR-0019 | IMPLEMENTADO | `ttf-parser` + `rustybuzz` → L3 |
| ADR-0033 | EM VIGOR | Paridade observable |
| ADR-0034 | EM VIGOR | Diagnóstico canónico — este ficheiro segue |
| ADR-0054 | EM VIGOR | Perfil graded |
| ADR-0065 | EM VIGOR | Inventariar primeiro — este passo aplica |
| **DEBT-8** | **PARCIALMENTE RESOLVIDO** (2026-03 Passo 40; **não actualizado desde**) | Motor de equações |

DEBT-8 lista factualmente desde 2026-03-26 quatro pendências:
1. Kern matemático entre símbolos.
2. Fontes OpenType MATH (tabelas MATH, variantes de tamanho).
3. `MathPrimes` (parseado e evaluado em `eval.rs`, sem lógica de
   kern/posição no layouter).
4. Baseline correcta em relação ao x-height da fonte.

---

## §2 — Fase A: Inventário empírico das 4 pendências DEBT-8

### Item 1 — Kern matemático entre símbolos

**Estado da pendência (2026-03)**: aberto.

**Evidência cumulativa pós-2026-03**:
- Tipo L1 `MathGlyphKern` existe (referido em prompt
  `infra/font_metrics.md`).
- Trait `FontMetrics::math_kern(&self, c: char) -> MathGlyphKern`
  declarado e implementado em L3 (`font_metrics.rs`).
- L3 lê tabela `kern_infos` por quadrante (top-right, top-left,
  bottom-right, bottom-left); tabela tem `n` alturas + `n+1`
  valores; fallback `MathGlyphKern::default()` se fonte não tem
  tabela kern.
- **Não há evidência empírica** no contexto disponível de que o
  Layouter (`MathLayouter`, ou submódulos `attach`/`hconcat`)
  **consuma** `math_kern()` para ajustar posicionamento. P96.8
  lista `hconcat` como helper livre em `mod.rs:484`, mas o
  conteúdo do helper não está expandido no contexto.

**Classificação cautelosa**: **infra-estrutura completa; consumer
em Layouter por verificar**.

**Acção empírica necessária para classificar definitivamente**:
- `grep -rn "math_kern" 01_core/src/rules/math/` para confirmar
  consumer real.
- `grep -rn "MathGlyphKern" 01_core/src/rules/math/` idem.
- Se zero hits → **pendência real activa** (infra-estrutura
  pronta, ligação não feita).
- Se ≥1 hit em `attach.rs` ou `hconcat` → **pendência fechada
  estruturalmente** (faltam tests E2E para declarar funcionalmente
  fechada).

### Item 2 — Fontes OpenType MATH (tabelas MATH, variantes de tamanho)

**Estado da pendência (2026-03)**: aberto.

**Evidência cumulativa pós-2026-03**:
- Tipo L1 `MathConstants` existe (10 campos enumerados em prompt
  `entities/math_constants.md` hash `73380d77`; campo extra
  `axis_height` referido em critério de verificação L3 sugere que
  a struct real tem mais campos do que o prompt L0 mostra).
- Tipo L1 `GlyphVariants { variants: Vec<GlyphVariant> }` existe
  com método `select(min_advance)`.
- Tipo L1 `GlyphAssembly` existe (assembly por partes para
  delimitadores extensíveis grandes).
- L3 lê tudo: `math_constants()`, `vertical_glyph_variants()`,
  `vertical_glyph_assembly()`, `build_math_glyph_reverse_map()`.
- P96.8 cria submódulo `math/layout/stretchy.rs` (64 linhas) +
  `math/layout/assembly.rs` (82 linhas) — sugestão forte de que
  o Layouter consome variantes + assembly.
- Método `MathLayouter::apply_axis_offset` existe (P96.8
  listagem) — uso de `axis_height` muito provável.

**Classificação cautelosa**: **infra-estrutura completa;
consumer em Layouter estruturalmente provável; cobertura
funcional por verificar**.

**Acção empírica necessária**:
- Ler `01_core/src/rules/math/layout/stretchy.rs` e confirmar
  uso de `GlyphVariants`.
- Ler `01_core/src/rules/math/layout/assembly.rs` e confirmar
  uso de `GlyphAssembly`.
- Ler `01_core/src/rules/math/layout/mod.rs` `apply_axis_offset`
  e confirmar consulta a `MathConstants.axis_height`.
- Confirmar se `MathConstants::fallback()` é o caminho activo
  (sem fonte MATH real disponível) ou se há fixture de teste
  com fonte MATH real (e.g. STIX Two Math).

### Item 3 — `MathPrimes` sem lógica de layout dedicada

**Estado da pendência (2026-03; clarificado P84.1)**: parseado e
evaluado em `eval.rs`, **sem lógica de kern/posição no
Layouter**.

**Evidência cumulativa pós-2026-03**:
- AST `MathPrimes` completo em `entities/ast/math.rs` com
  `count(self) -> usize`.
- `MathAttach.primes(self) -> Option<MathPrimes<'a>>` declarado
  (P6).
- P96.8 listou submódulo `attach.rs` (221 linhas) — o maior dos
  8 submódulos. Tratamento de primes provavelmente está aqui se
  foi materializado.
- **Sem evidência cumulativa** explícita no contexto disponível
  de que `MathPrimes` ganhou arm de layout pós-P40.

**Classificação cautelosa**: **provável manutenção de pendência
real**, mas confirmação exige leitura de `attach.rs`.

**Acção empírica necessária**:
- `grep -n "MathPrimes\|primes" 01_core/src/rules/math/layout/attach.rs`.
- Se zero hits → pendência confirmada.
- Se ≥1 hit → ler arm e classificar como minimal/completo.

### Item 4 — Baseline correcta em relação ao x-height da fonte

**Estado da pendência (2026-03)**: aberto.

**Evidência cumulativa pós-2026-03**:
- `MathConstants` enumerado no prompt **não** inclui
  `axis_height` explicitamente como campo público.
- Critério de verificação em prompt `infra/font_metrics.md`
  refere `MathConstants::fallback().axis_height = 500.0` — implica
  fortemente que o campo existe na struct real mas o prompt L0
  está desactualizado.
- `MathLayouter::apply_axis_offset` (P96.8 listagem) sugere que
  axis baseline já é aplicado.
- Vertical metrics (`ascender`, `descender`) lidos em L3.

**Classificação cautelosa**: **provável fecho estrutural**
(baseline aplicado via `apply_axis_offset`); cobertura funcional
exacta vs x-height vanilla por verificar.

**Acção empírica necessária**:
- Ler `MathLayouter::apply_axis_offset` em `math/layout/mod.rs`.
- Confirmar que usa `MathConstants.axis_height` (e não
  hardcoded `ascender * 0.5` legacy).
- Verificar se tests de paridade comparam baseline com vanilla.

### Resumo Fase A

| # | Pendência DEBT-8 | Infra L1 | Reader L3 | Consumer Layouter | Classificação cautelosa |
|---|-------------------|----------|-----------|-------------------|--------------------------|
| 1 | Kern matemático | `MathGlyphKern` ✓ | `math_kern()` ✓ | ❓ por verificar | Provável aberto |
| 2 | OpenType MATH tables + variantes | `MathConstants`, `GlyphVariants`, `GlyphAssembly` ✓ | tudo lido ✓ | submódulos `stretchy.rs`, `assembly.rs` existem (P96.8) | Provável fecho estrutural |
| 3 | `MathPrimes` layout | AST ✓ | n/a | ❓ por verificar | Provável aberto |
| 4 | Baseline x-height | `MathConstants` ✓ | reader ✓ | `apply_axis_offset` existe (P96.8) | Provável fecho estrutural |

**Inconsistências documentais detectadas** (achado adicional):

- Prompt L0 `rules/math/layout.md` (hash `d76fb51b`) descreve
  interface "Passo 36 / 37+ / 38+" como se P38+ ainda fosse
  futuro. P96.8 reestruturou em 8 submódulos. Prompt L0
  **desactualizado vs código real**.
- Prompt L0 `entities/math_constants.md` (hash `73380d77`) lista
  10 campos. Critério L3 menciona `axis_height = 500.0`. Campo
  no prompt **incompleto vs struct real**.
- DEBT.md entrada DEBT-8 não actualizada desde Passo 40
  (2026-03-26). 8 semanas de materialização não reflectidas.

---

## §3 — Fase B: Decisão condicional

### Cenário B1: Fase A confirma 4/4 fechados (apenas tests E2E faltam)

**Improvável**, mas se materializar:

- Acção: **passo de fecho DEBT-8 + actualização documental**
  (padrão P192A).
- Magnitude: XS-S (~30-60 min documental).
- Componentes:
  - DEBT-8 transita "PARCIALMENTE RESOLVIDO" → "ENCERRADO" com
    secção de evidência cumulativa.
  - Prompt L0 `rules/math/layout.md` actualizado: nova listagem
    de submódulos pós-P96.8 + interface `apply_axis_offset` +
    consumers de `MathConstants`/`MathGlyphKern`/`GlyphVariants`/
    `GlyphAssembly`.
  - Prompt L0 `entities/math_constants.md` actualizado:
    enumeração completa de campos (incluir `axis_height` e
    outros omitidos).
  - Relatório de auditoria análogo a P192A.

### Cenário B2: Fase A confirma 2/4 ou 3/4 fechados (1-2 pendências reais)

**Mais provável** dado evidência indirecta.

- Acção: **diagnóstico Math + recomendação Fase 1 minimal**
  (padrão P160).
- Magnitude: S-M (~1-2 sub-passos pós-diagnóstico).
- Componentes:
  - Diagnóstico §1-§6 análogo P160 com cobertura: estado actual,
    pendências reais, dependências, tecto realista, sequência
    candidata, recomendação.
  - Fase 1 minimal recomendada por pendência:
    - **Kern matemático** (se aberto): integrar `math_kern()` em
      `hconcat` ou `attach` ajustando offset; M; +5-8 tests.
    - **`MathPrimes` layout** (se aberto): arm em
      `attach.rs` para emitir glifo `′` repetido `count` vezes
      como superscript; S+/M; +3-5 tests.
  - DEBT-8 actualizado com lista revista de pendências e
    secção de evidência cumulativa intermédia.

### Cenário B3: Fase A confirma ≤1/4 fechados (≥3 pendências reais)

**Improvável** dado P96.8 + tipos L1 completos + L3 reader
completo.

- Acção: **diagnóstico Math amplo** análogo P159B (multi-feature).
- Magnitude: M-L (4-8 sub-passos).

### Critério de decisão entre cenários

Decisão depende exclusivamente dos resultados das 4 acções
empíricas listadas no §2. Recomendação concreta:

1. Executar 4 acções empíricas (~10-15 min de `grep`/`view`).
2. Preencher tabela §2 com hits/no-hits factuais.
3. Decidir cenário entre B1/B2/B3.
4. Materializar conforme cenário.

---

## §4 — Recomendação concreta

### Recomendação primária

**P254B-aud — Acções empíricas Fase A** (XS; ~15 min de leitura
de código). Output: tabela §2 com colunas hits factuais
preenchidas; decisão B1/B2/B3 explícita.

Passos:

1. `view` ou `grep` os 4 ficheiros suspeitos:
   - `01_core/src/rules/math/layout/mod.rs` (`apply_axis_offset`).
   - `01_core/src/rules/math/layout/attach.rs` (primes; kern).
   - `01_core/src/rules/math/layout/stretchy.rs` (variantes).
   - `01_core/src/rules/math/layout/assembly.rs` (delimitadores
     grandes).
2. Confirmar consumers de `MathConstants`, `MathGlyphKern`,
   `GlyphVariants`, `GlyphAssembly`.
3. Decidir cenário B1/B2/B3 com base em evidência factual.

### Recomendação secundária

**P254B-doc — Actualização documental dos prompts L0 obsoletos**
(XS; ~15 min). Independente da decisão B1/B2/B3:

- Prompt `rules/math/layout.md` precisa de revisão pós-P96.8.
- Prompt `entities/math_constants.md` precisa de enumeração
  completa de campos.

Pode ser feita conjuntamente com P254B-aud para amortizar
overhead.

### Recomendação terciária

**Apenas após P254B-aud completar**: materializar o cenário
decidido (B1, B2, ou B3). Não é prudente comprometer com
materialização antes de Fase A produzir evidência factual.

---

## §5 — Padrões metodológicos aplicados

### ADR-0065 critério #5 — scope determinado por inventário

Aplicação particular: **inventário em duas fases**. Fase A
identifica pendências reais; Fase B determina scope da
materialização. Padrão análogo a P192A (auditoria que descobre
fecho retroactivo) combinado com P160 (diagnóstico que recomenda
Fase 1).

Subpadrão emergente: **"auditoria condicional"** — diagnóstico
que adia decisão de scope até evidência empírica ser produzida.
N=1 (este passo). Precedente próximo: P192A (auditoria sem
decisão condicional explícita).

### ADR-0034 — diagnóstico canónico

Aplicado: §1-§5 padrão; persistência em
`00_nucleo/diagnosticos/`.

### Política "sem novas reservas"

Preservada. Diagnóstico identifica pendências reais (não cria
novas reservas) e recomendações §4 são para validação humana.

---

## §6 — Referências

- DEBT-8 (`00_nucleo/DEBT.md`) — origem da lista de 4 pendências.
- ADR-0009 — `MathClass` L1 PROPOSTO.
- ADR-0019 — `ttf-parser` + `rustybuzz` L3 IMPLEMENTADO.
- Prompt L0 `entities/ast/math.md` — AST completo.
- Prompt L0 `entities/math_constants.md` — `MathConstants` L1.
- Prompt L0 `entities/glyph_variants.md` — `GlyphVariants` L1.
- Prompt L0 `rules/math/layout.md` — interface Layouter
  (**desactualizado**).
- Prompt L0 `infra/font_metrics.md` — reader L3 completo.
- Passo 96.8 — reestruturação P96 em 8 submódulos
  (`attach`/`root`/`frac`/`matrix`/`cases`/`stretchy`/`assembly`/
  `delimited`).
- Passo 199B — `Content::SetEquationNumbering` materializado
  (numbering desbloqueado).
- P192A — precedente "auditoria que descobre fecho retroactivo".
- P160 — precedente "diagnóstico com recomendação Fase 1".

# Passo 41 — Constantes OpenType MATH via ttf-parser

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/math/layout.rs` — `MathLayouter`, `layout_frac`, `layout_attach`, `layout_root`
- `01_core/src/rules/layout.rs` — `FontMetrics` trait
- `03_infra/src/font_metrics.rs` — `FontBookMetrics<'a>` via `ttf-parser`
- `03_infra/src/export.rs` — PDF com Helvetica fallback e `FrameItem::Line`
- ADR-0019 — `ttf-parser` e `rustybuzz` autorizados em L3

Pré-condição: `cargo test` — 475 L1 + 61 L3 + 50 parity, zero violations.

---

## Contexto

O `MathLayouter` usa valores hardcoded para posicionamento:
- `0.7` para tamanho de numerador/denominador em fracções
- `0.65` para tamanho de sup/sub em `MathAttach`
- `0.65` para tamanho de índice em `MathRoot`
- `0.5 * ascender` para elevação de superscript
- `0.3 * descender` para abaixamento de subscript
- `0.04 * size` para espessura de linha em fracções e radicais
- `0.1 * size` para gap entre overline e radicando

Fontes OpenType com tabela MATH (ex: STIX Two Math, Latin Modern Math,
Cambria Math) definem estas constantes com precisão. A tabela MATH
contém ~55 constantes (MathConstants) incluindo:

- `FractionNumeratorShiftUp`
- `FractionDenominatorShiftDown`
- `FractionRuleThickness`
- `RadicalVerticalGap`
- `RadicalRuleThickness`
- `SuperscriptShiftUp`
- `SubscriptShiftDown`
- `ScriptPercentScaleDown` (tipicamente 70–80%)
- `ScriptScriptPercentScaleDown` (tipicamente 50–60%)

O `ttf-parser` já disponibiliza `face.tables().math` para acesso à
tabela MATH. Este passo extrai essas constantes e as passa ao
`MathLayouter` via uma nova interface.

---

## Decisão arquitectural

A tabela MATH é parsing de dados binários de fonte → L3 (ADR-0019).
O `MathLayouter` em L1 não pode receber `ttf_parser::Face`.

Solução: criar um struct `MathConstants` em L1 com os valores
numéricos puros (em unidades de design, não pts). L3 preenche a
struct a partir de `ttf-parser`. L1 converte para pts usando
`size * (value / upem)`.

Isto segue o padrão `FontMetrics`: L1 define o trait/struct,
L3 implementa com dados reais.

**Não é necessária ADR nova** — ADR-0019 já autoriza `ttf-parser`
em L3, e o struct `MathConstants` em L1 é um tipo de domínio puro
(apenas campos `f64`).

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. ttf-parser suporta tabela MATH?
grep -rn "math\|Math" \
  $(cargo metadata --format-version 1 2>/dev/null \
    | python3 -c "import sys,json; \
      pkgs=json.load(sys.stdin)['packages']; \
      p=[x for x in pkgs if x['name']=='ttf-parser']; \
      print(p[0]['manifest_path'].rsplit('/',1)[0] if p else '')" \
  )/src/ 2>/dev/null | head -30

# Se o comando acima falhar, alternativa directa:
find ~/.cargo/registry/src -path "*/ttf-parser-*/src" -type d 2>/dev/null \
  | head -1 | xargs -I{} grep -rn "math\|Math\|MATH" {} | head -30

# 2. API da tabela MATH no ttf-parser
find ~/.cargo/registry/src -path "*/ttf-parser-*/src" -type d 2>/dev/null \
  | head -1 | xargs -I{} grep -rn "MathConstants\|math_constants\|radical\|fraction\|superscript\|subscript" {} | head -30

# 3. Versão actual de ttf-parser no projecto
grep "ttf-parser" 03_infra/Cargo.toml Cargo.lock | head -5

# 4. Valores hardcoded actuais no MathLayouter
grep -n "0\.7\|0\.65\|0\.5\|0\.3\|0\.04\|0\.1" \
  01_core/src/rules/math/layout.rs | head -20

# 5. Interface actual do MathLayouter — campos da struct
grep -A 20 "pub struct MathLayouter" 01_core/src/rules/math/layout.rs

# 6. Como o MathLayouter recebe métricas actualmente
grep -n "metrics\|FontMetrics\|advance\|vertical" \
  01_core/src/rules/math/layout.rs | head -20

# 7. Confirmar que FontBookMetrics tem acesso a Face
grep -n "face\|Face" 03_infra/src/font_metrics.rs | head -10
```

**Reportar o output antes de continuar.**

Se `ttf-parser` não suportar a tabela MATH na versão instalada,
verificar se uma versão mais recente a suporta. Se nenhuma versão
suportar, o passo muda de âmbito para leitura manual da tabela
MATH via bytes raw (alternativa documentada abaixo).

---

## Tarefa 1 — MathConstants em L1

Criar o struct de constantes matemáticas em L1. Todos os valores
são `f64` em unidades de design (dividir por upem para converter).

```rust
// 01_core/src/entities/math_constants.rs

//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/math_constants.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-04-03

/// Constantes matemáticas extraídas da tabela OpenType MATH.
///
/// Valores em unidades de design (design units). Para converter
/// para pontos tipográficos: `pt = size * (value / upem)`.
///
/// Fonte: OpenType spec §6.3.4 — MathConstants table.
/// Fallback: valores baseados nos defaults do Typst original
/// e em STIX Two Math como referência.
#[derive(Debug, Clone)]
pub struct MathConstants {
    /// units_per_em da fonte — necessário para conversão.
    pub upem: f64,

    // ── Fracções ─────────────────────────────────────────
    /// Deslocamento vertical do numerador acima da baseline.
    pub fraction_numerator_shift_up: f64,
    /// Deslocamento vertical do denominador abaixo da baseline.
    pub fraction_denominator_shift_down: f64,
    /// Espessura da barra de fracção.
    pub fraction_rule_thickness: f64,
    /// Gap mínimo entre numerador e barra.
    pub fraction_num_gap: f64,
    /// Gap mínimo entre barra e denominador.
    pub fraction_denom_gap: f64,

    // ── Scripts (sup/sub) ────────────────────────────────
    /// Deslocamento vertical do superscript.
    pub superscript_shift_up: f64,
    /// Deslocamento vertical do subscript.
    pub subscript_shift_down: f64,

    // ── Radicais ─────────────────────────────────────────
    /// Gap vertical entre radicando e overline.
    pub radical_vertical_gap: f64,
    /// Espessura da overline do radical.
    pub radical_rule_thickness: f64,
    /// Kern extra antes do grau (índice do radical).
    pub radical_degree_bottom_raise_percent: f64,

    // ── Escala de scripts ────────────────────────────────
    /// Percentagem de escala para script (sup/sub/frac).
    /// Tipicamente 70–80%. Armazenado como 0.0–1.0.
    pub script_percent_scale_down: f64,
    /// Percentagem para script-script (sup de sup).
    /// Tipicamente 50–60%. Armazenado como 0.0–1.0.
    pub script_script_percent_scale_down: f64,
}

impl MathConstants {
    /// Fallback com valores hardcoded baseados em STIX Two Math.
    ///
    /// Usado quando a fonte não tem tabela MATH (ex: Helvetica).
    /// upem assume 1000 (standard para fontes Type 1).
    pub fn fallback() -> Self {
        let upem = 1000.0;
        Self {
            upem,
            fraction_numerator_shift_up:      420.0,
            fraction_denominator_shift_down:   340.0,
            fraction_rule_thickness:            66.0,
            fraction_num_gap:                   50.0,
            fraction_denom_gap:                 50.0,
            superscript_shift_up:              362.0,
            subscript_shift_down:              130.0,
            radical_vertical_gap:               60.0,
            radical_rule_thickness:              66.0,
            radical_degree_bottom_raise_percent: 0.6,
            script_percent_scale_down:          0.7,
            script_script_percent_scale_down:   0.5,
        }
    }

    /// Converte um valor em design units para Pt.
    pub fn to_pt(&self, value: f64, size: Pt) -> Pt {
        size * (value / self.upem)
    }
}
```

Adicionar a `entities/mod.rs`:
```rust
pub mod math_constants;
```

---

## Tarefa 2 — Expandir FontMetrics com math_constants()

Adicionar um método opcional ao trait `FontMetrics` em L1.
Default retorna `MathConstants::fallback()`.

```rust
// Em 01_core/src/rules/layout.rs — adicionar ao trait FontMetrics:

/// Constantes da tabela OpenType MATH, se disponível.
///
/// Default: MathConstants::fallback() para fontes sem tabela MATH.
fn math_constants(&self) -> MathConstants {
    MathConstants::fallback()
}
```

`FixedMetrics` herda o default — sem alteração necessária.

---

## Tarefa 3 — FontBookMetrics lê a tabela MATH em L3

Em `03_infra/src/font_metrics.rs`, implementar `math_constants()`
para `FontBookMetrics`.

```rust
// Em impl FontMetrics for FontBookMetrics<'_>:

fn math_constants(&self) -> MathConstants {
    // Verificar se ttf-parser expõe a tabela MATH
    // A API exacta depende da versão — confirmar no diagnóstico
    match self.face.tables().math {
        Some(math) => {
            let c = math.constants; // ou equivalente na API
            MathConstants {
                upem: self.upem,
                fraction_numerator_shift_up:
                    c.fraction_numerator_shift_up() as f64,
                fraction_denominator_shift_down:
                    c.fraction_denominator_shift_down() as f64,
                fraction_rule_thickness:
                    c.fraction_rule_thickness() as f64,
                fraction_num_gap:
                    c.fraction_num_display_style_gap_min() as f64,
                fraction_denom_gap:
                    c.fraction_denom_display_style_gap_min() as f64,
                superscript_shift_up:
                    c.superscript_shift_up() as f64,
                subscript_shift_down:
                    c.subscript_shift_down() as f64,
                radical_vertical_gap:
                    c.radical_vertical_gap() as f64,
                radical_rule_thickness:
                    c.radical_rule_thickness() as f64,
                radical_degree_bottom_raise_percent:
                    c.radical_degree_bottom_raise_percent() as f64 / 100.0,
                script_percent_scale_down:
                    c.script_percent_scale_down() as f64 / 100.0,
                script_script_percent_scale_down:
                    c.script_script_percent_scale_down() as f64 / 100.0,
            }
        }
        None => MathConstants::fallback(),
    }
}
```

**Nota**: a API exacta de `ttf-parser` para tabela MATH varia por
versão. O diagnóstico vai revelar os nomes correctos dos métodos.
Se `self.face.tables().math` não existir, verificar:
- `self.face.raw_face().table(Tag::from_bytes(b"MATH"))`
- Versão mais recente de `ttf-parser` que suporte a tabela

**Alternativa se ttf-parser não suportar tabela MATH**:
Ler manualmente os bytes raw da tabela MATH. A estrutura é:
- Offset 0: MajorVersion (u16), MinorVersion (u16)
- Offset 4: MathConstantsOffset (u16)
- Na posição MathConstantsOffset: 56 campos MathValueRecord
  (cada um é i16 value + u16 deviceTableOffset)
Criar um parser mínimo em `03_infra/src/math_table.rs`.

---

## Tarefa 4 — MathLayouter usa MathConstants

Modificar `MathLayouter` para receber e usar `MathConstants`.

```rust
// Em 01_core/src/rules/math/layout.rs

pub struct MathLayouter<'a, M: FontMetrics> {
    metrics: &'a M,
    size: Pt,
    constants: MathConstants,  // NOVO
}

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub fn new(metrics: &'a M, size: Pt) -> Self {
        let constants = metrics.math_constants();
        Self { metrics, size, constants }
    }
}
```

Substituir os valores hardcoded:

### layout_frac

```rust
// ANTES:
// let script_size = Pt(self.size.val() * 0.7);
// let rule_thickness = Pt(self.size.val() * 0.04);

// DEPOIS:
let script_size = self.size * self.constants.script_percent_scale_down;
let rule_thickness = self.constants.to_pt(
    self.constants.fraction_rule_thickness, self.size
);
```

### layout_attach

```rust
// ANTES:
// let sup_offset_y = Pt(ascender.val() * 0.5);
// let script_size = Pt(self.size.val() * 0.65);

// DEPOIS:
let script_size = self.size * self.constants.script_percent_scale_down;
let sup_offset_y = self.constants.to_pt(
    self.constants.superscript_shift_up, self.size
);
let sub_offset_y = self.constants.to_pt(
    self.constants.subscript_shift_down, self.size
);
```

### layout_root

```rust
// ANTES:
// let line_thickness = Pt(self.size.val() * 0.04);
// let gap = Pt(self.size.val() * 0.1);

// DEPOIS:
let line_thickness = self.constants.to_pt(
    self.constants.radical_rule_thickness, self.size
);
let gap = self.constants.to_pt(
    self.constants.radical_vertical_gap, self.size
);
let idx_scale = self.constants.script_percent_scale_down;
```

---

## Tarefa 5 — Testes

### Testes unitários em L1

```rust
#[cfg(test)]
mod tests_math_constants {
    use super::*;

    #[test]
    fn fallback_valores_sanos() {
        let c = MathConstants::fallback();
        assert!(c.upem > 0.0);
        assert!(c.fraction_rule_thickness > 0.0);
        assert!(c.radical_rule_thickness > 0.0);
        assert!(c.script_percent_scale_down > 0.0);
        assert!(c.script_percent_scale_down <= 1.0);
        assert!(c.script_script_percent_scale_down > 0.0);
        assert!(c.script_script_percent_scale_down <= 1.0);
        assert!(c.superscript_shift_up > 0.0);
        assert!(c.subscript_shift_down > 0.0);
    }

    #[test]
    fn to_pt_converte_correctamente() {
        let c = MathConstants::fallback(); // upem = 1000
        let pt = c.to_pt(500.0, Pt(12.0));
        // 12.0 * (500.0 / 1000.0) = 6.0
        assert!((pt.val() - 6.0).abs() < 0.001);
    }

    #[test]
    fn to_pt_zero_value() {
        let c = MathConstants::fallback();
        let pt = c.to_pt(0.0, Pt(12.0));
        assert!((pt.val()).abs() < 0.001);
    }

    #[test]
    fn fixed_metrics_retorna_fallback() {
        let m = FixedMetrics;
        let c = m.math_constants();
        assert!((c.upem - 1000.0).abs() < 0.001);
    }

    // ── Layout com fallback (sem fonte MATH) ─────────────

    #[test]
    fn layout_frac_com_constants_fallback() {
        // Verificar que frac ainda funciona após substituição dos hardcodes
        let doc = layout_test("$frac(a, b)$");
        let text = doc.plain_text();
        assert!(text.contains('a'), "numerador: {}", text);
        assert!(text.contains('b'), "denominador: {}", text);
    }

    #[test]
    fn layout_sqrt_com_constants_fallback() {
        let doc = layout_test("$sqrt(x)$");
        let text = doc.plain_text();
        assert!(text.contains('√'), "radical: {}", text);
        assert!(text.contains('x'), "radicando: {}", text);
    }

    #[test]
    fn layout_attach_com_constants_fallback() {
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('x'), "base: {}", text);
        assert!(text.contains('2'), "superscript: {}", text);
    }
}
```

### Testes em L3 (com fonte MATH real, se disponível)

```rust
#[cfg(test)]
mod tests_math_font {
    use super::*;

    #[test]
    #[ignore = "requer fonte com tabela MATH em tests/fixtures/"]
    fn font_com_tabela_math_retorna_constants_reais() {
        // Usar STIX Two Math, Latin Modern Math, ou similar
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/stix-two-math.otf")
        ).expect("fixture necessária");

        let m = FontBookMetrics::from_bytes(&data).unwrap();
        let c = m.math_constants();

        // Valores devem diferir do fallback
        // (a menos que a fonte use exactamente os mesmos valores)
        assert!(c.upem > 0.0);
        assert!(c.fraction_rule_thickness > 0.0);
        assert!(c.script_percent_scale_down > 0.0);
        assert!(c.script_percent_scale_down <= 1.0);

        // Sanidade: FractionRuleThickness em STIX Two Math é ~66 design units
        // com upem=1000. Valor entre 40 e 120 é razoável.
        assert!(c.fraction_rule_thickness > 20.0,
            "rule_thickness={} muito fino", c.fraction_rule_thickness);
        assert!(c.fraction_rule_thickness < 200.0,
            "rule_thickness={} muito grosso", c.fraction_rule_thickness);
    }

    #[test]
    #[ignore = "requer fonte com tabela MATH"]
    fn font_sem_tabela_math_retorna_fallback() {
        // Helvetica não tem tabela MATH
        // Usar Liberation Sans ou outra fonte sem MATH
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/liberation-sans-regular.ttf")
        ).expect("fixture necessária");

        let m = FontBookMetrics::from_bytes(&data).unwrap();
        let c = m.math_constants();

        // Deve retornar fallback (upem=1000 do fallback, ou upem real da fonte)
        assert!(c.fraction_rule_thickness > 0.0);
    }

    #[test]
    fn pdf_frac_com_constants() {
        // Pipeline completo — confirmar que não panic após refactoring
        let pdf = compile_to_pdf("$frac(a, b)$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_sqrt_com_constants() {
        let pdf = compile_to_pdf("$sqrt(x)$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_attach_com_constants() {
        let pdf = compile_to_pdf("$x^2_i$");
        assert!(!pdf.is_empty());
    }
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão:
- [ ] `MathConstants` existe em L1 com 13 campos + `fallback()` + `to_pt()`
- [ ] `FontMetrics::math_constants()` tem default que retorna fallback
- [ ] `FontBookMetrics` lê tabela MATH quando disponível na fonte
- [ ] `FontBookMetrics` retorna fallback quando fonte não tem tabela MATH
- [ ] `MathLayouter` usa `self.constants` em vez de valores hardcoded
- [ ] `layout_frac` usa `fraction_rule_thickness` e `script_percent_scale_down`
- [ ] `layout_attach` usa `superscript_shift_up` e `subscript_shift_down`
- [ ] `layout_root` usa `radical_rule_thickness` e `radical_vertical_gap`
- [ ] Testes existentes de frac/attach/sqrt continuam a passar (regressão)
- [ ] `ttf-parser` não aparece em `01_core/Cargo.toml`
- [ ] Zero violations no linter

---

## Ao terminar, reportar

**Do diagnóstico:**
- Versão de `ttf-parser` e se suporta `face.tables().math`
- API exacta usada para ler constantes MATH (nomes dos métodos)
- Se foi necessário parser manual da tabela MATH (alternativa)

**Da implementação:**
- Quantos valores hardcoded foram substituídos
- Se algum teste de layout mudou de resultado (regressão)
- Se a fonte de fixture (se usada) tem tabela MATH

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 42:**
- **GO — Glifos extensíveis**: constantes MATH funcionais; Passo 42
  implementa substituição de glifos (radical extensível, delimitadores
  extensíveis) usando a tabela MathGlyphConstruction
- **GO — Kern matemático**: constantes MATH funcionais; Passo 42
  implementa MathKernInfo para ajuste fino entre símbolos adjacentes
- **NO-GO — ttf-parser sem MATH**: tabela não acessível nem por API
  nem por bytes raw; avaliar crate alternativa ou parser manual antes
  de avançar

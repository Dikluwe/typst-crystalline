# Passo 44 — AxisHeight e MathKernInfo

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/math_constants.rs` — `MathConstants` com todos os campos actuais e `to_pt()`
- `01_core/src/entities/glyph_variants.rs` — `GlyphVariants`, `GlyphAssembly`, `GlyphPart`
- `01_core/src/rules/layout.rs` — trait `FontMetrics`, métodos actuais
- `01_core/src/rules/math/layout.rs` — `MathLayouter`, `layout_frac`, `layout_delimited`, `layout_attach`, `layout_root`, `offset_item`
- `03_infra/src/font_metrics.rs` — `FontBookMetrics`, o que já é lido de `math_table`

Pré-condição: `cargo test` — 504 L1 + 71 L3 + 50 parity, zero violations.

---

## Contexto

Dois problemas de posicionamento persistem no motor de equações:

**Problema A — AxisHeight**: fracções, delimitadores e raízes são actualmente
alinhados pela baseline do texto circundante. Em tipografia matemática, estes
elementos devem ser centrados no *eixo matemático* — uma linha horizontal
situada aproximadamente ao centro do sinal de mais (`+`), acima da baseline.
A tabela OpenType MATH define este valor como `axis_height`. Sem este
alinhamento, equações com fracções embutidas no texto ficam visivelmente
descentradas.

**Problema B — MathKernInfo**: quando um superscript ou subscript é colocado
junto a uma base com inclinação (ex: `f^2`, `V^n`), a distância horizontal
entre base e script é calculada de forma rectilínea. A tabela MATH define
kern específicos por quadrante (top-right, top-left, bottom-right,
bottom-left) para ajustar o espaçamento em função da altura do script.
Sem isso, scripts em letras inclinadas ficam demasiado afastados ou
sobrepostos.

Estes dois problemas são independentes e implementados em tarefas separadas.
A ordem é: AxisHeight primeiro (impacto visual maior), MathKernInfo depois.

Não há ADR nova — o padrão L1/L3 via structs simples está estabelecido.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. Confirmar campos actuais de MathConstants em L1
cat 01_core/src/entities/math_constants.rs

# 2. Confirmar como MathLayouter usa MathConstants actualmente
# (especialmente layout_frac, layout_delimited, layout_root)
grep -n "constants\.\|axis\|fraction_rule\|fraction_num\|fraction_den" \
  01_core/src/rules/math/layout.rs | head -30

# 3. Como offset_item e place() posicionam items actualmente
grep -n "fn offset_item\|fn place\|fn hconcat\|ascent\|descent" \
  01_core/src/rules/math/layout.rs | head -30

# 4. API de MathKernInfo no ttf-parser 0.25.1
find ~/.cargo/registry/src -path "*/ttf-parser-*/src" -type d 2>/dev/null \
  | head -1 | xargs -I{} grep -rn "MathKern\|kern_info\|KernInfo\|top_right\|top_left\|bottom_right\|bottom_left\|correction_height" {} | head -30

# 5. Acesso a MathKernInfo via math_table no ttf-parser
find ~/.cargo/registry/src -path "*/ttf-parser-*/src" -type d 2>/dev/null \
  | head -1 | xargs -I{} grep -rn "glyph_info\|kern_infos\|MathGlyphInfo\|math_kern" {} | head -20

# 6. Como layout_attach calcula posições de sup/sub actualmente
grep -n "fn layout_attach\|sup\|sub\|top\|bottom\|kern" \
  01_core/src/rules/math/layout.rs | head -30

# 7. Como MathConstants.axis_height é usada actualmente (se é que é)
grep -rn "axis_height\|axis" 01_core/src/ | head -10

# 8. Confirmar que axis_height existe em MathConstants e que to_pt() está disponível
grep -n "axis_height\|to_pt\|upem" 01_core/src/entities/math_constants.rs | head -10
```

**Reportar o output antes de continuar.**

Se `MathKernInfo` não estiver acessível na versão instalada de `ttf-parser`,
o sub-âmbito B (kern) fica fora do passo; implementar apenas AxisHeight.
Se `axis_height` já estiver em `MathConstants` mas não estiver a ser usada,
a Tarefa 2 é apenas adicionar o uso — não é necessário mudar a struct.

---

## Sub-âmbito A — AxisHeight

### Tarefa 1 — Confirmar/adicionar axis_height em MathConstants

`MathConstants` provavelmente já tem `axis_height` do Passo 41. Confirmar
no diagnóstico. Se existir: nenhuma alteração a `math_constants.rs`.
Se não existir: adicionar campo `axis_height: f64` com valor `fallback`
de `500.0` (design units — valor típico para fontes sem tabela MATH).

### Tarefa 2 — Usar AxisHeight no MathLayouter

O eixo matemático é a referência vertical para elementos centrados.
A regra: o centro vertical de fracções, delimitadores e raízes deve
coincidir com o eixo matemático em vez de com a baseline.

```rust
// Em MathLayouter — novo método auxiliar:

/// Deslocamento vertical para centrar um MathBox no eixo matemático.
///
/// Retorna o valor em pt que deve ser somado a `pos.y` para que o
/// centro do MathBox (meio entre ascent e descent) fique no eixo.
///
/// `axis` é `self.constants.axis_height` convertido para pt:
///   `axis_pt = self.size * (self.constants.axis_height / self.constants.upem)`
///
/// `center_offset = axis_pt - (box.ascent - box.descent) / 2.0`
///
/// Positivo = deslocar para cima; negativo = deslocar para baixo.
fn axis_offset(&self, math_box: &MathBox) -> Pt {
    let axis_pt = self.size * Pt(self.constants.axis_height / self.constants.upem);
    let box_center = (math_box.ascent - math_box.descent) / 2.0;
    axis_pt - box_center
}
```

Aplicar `axis_offset` nos seguintes sítios:

**Em `layout_frac`**: após calcular o `MathBox` final da fracção, aplicar
`axis_offset` ao resultado para que a linha de fracção fique centrada no eixo.
A linha de fracção já está posicionada relativamente ao numerador/denominador
— o que muda é a posição do bloco inteiro relativamente à baseline exterior.

**Em `layout_delimited`**: após `hconcat`, aplicar `axis_offset` ao bloco
resultante para centrar o conteúdo no eixo.

**Em `layout_root`**: após calcular o `MathBox` final, aplicar `axis_offset`.

**Não aplicar** em `layout_attach` nem em elementos inline simples
(`MathIdent`, `MathText`) — estes seguem a baseline normal.

**Implementação**: `axis_offset` retorna um `Pt`. O `MathLayouter` deve
somar este valor ao `y` de todos os `FrameItem` dentro do `MathBox`
resultante. O método `offset_item` já existe para este fim — verificar
no diagnóstico se aceita deslocamento em y ou apenas em x.
Se necessário, generalizar `offset_item` para aceitar `(dx: Pt, dy: Pt)`.

---

## Sub-âmbito B — MathKernInfo

### Tarefa 3 — MathKernTable em L1

Nova struct de domínio puro. Criar em `01_core/src/entities/glyph_variants.rs`
(junto às restantes structs de métricas matemáticas).

```rust
/// Um par (altura_de_correcção, valor_de_kern) da tabela MathKern.
///
/// A tabela define kern por intervalos de altura: para um script cuja
/// conexão está abaixo de `correction_height`, o kern aplicável é o
/// `kern_value` desse registo. O último registo não tem `correction_height`
/// — aplica-se a todas as alturas acima do penúltimo limiar.
///
/// Ambos os valores estão em design units.
#[derive(Debug, Clone)]
pub struct MathKernRecord {
    /// Altura máxima (em design units) para a qual este kern se aplica.
    /// `None` no último registo (aplica-se a todas as alturas restantes).
    pub correction_height: Option<f64>,
    /// Valor de kern a aplicar (design units). Pode ser negativo.
    pub kern_value: f64,
}

/// Tabela de kern para um quadrante de um glifo matemático.
///
/// Quadrantes: top-right (base+superscript), top-left (base+superscript à esquerda),
/// bottom-right (base+subscript), bottom-left (base+subscript à esquerda).
#[derive(Debug, Clone, Default)]
pub struct MathKernTable {
    pub records: Vec<MathKernRecord>,
}

impl MathKernTable {
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Retorna o kern em design units para uma dada altura de conexão.
    ///
    /// `height` é a altura do ponto de conexão do script (design units).
    /// Percorre os registos em ordem até encontrar o primeiro cujo
    /// `correction_height >= height`, ou usa o último registo.
    pub fn kern_at(&self, height: f64) -> f64 {
        for record in &self.records {
            match record.correction_height {
                Some(h) if h >= height => return record.kern_value,
                Some(_) => continue,
                None => return record.kern_value,
            }
        }
        0.0
    }
}

/// Kern matemático para os quatro quadrantes de um glifo.
#[derive(Debug, Clone, Default)]
pub struct MathGlyphKern {
    pub top_right: MathKernTable,
    pub top_left: MathKernTable,
    pub bottom_right: MathKernTable,
    pub bottom_left: MathKernTable,
}
```

### Tarefa 4 — FontMetrics::math_kern() em L1

Adicionar método ao trait `FontMetrics` em `01_core/src/rules/layout.rs`.
Default retorna `MathGlyphKern` vazio (sem kern — espaçamento rectilíneo).

```rust
use crate::entities::glyph_variants::MathGlyphKern;

/// Kern matemático por quadrante para um glifo.
///
/// `c` é o caractere base cujos scripts vão ser posicionados.
/// Default: sem kern (todos os quadrantes vazios).
fn math_kern(&self, c: char) -> MathGlyphKern {
    let _ = c;
    MathGlyphKern::default()
}
```

`FixedMetrics` herda o default — sem alteração.

### Tarefa 5 — FontBookMetrics implementa math_kern() em L3

```rust
// Em 03_infra/src/font_metrics.rs — impl FontMetrics for FontBookMetrics:

fn math_kern(&self, c: char) -> MathGlyphKern {
    let glyph_id = match self.face.glyph_index(c) {
        Some(id) => id,
        None => return MathGlyphKern::default(),
    };

    let math = match self.face.tables().math {
        Some(m) => m,
        None => return MathGlyphKern::default(),
    };

    // API exacta depende da versão do ttf-parser — confirmar no diagnóstico.
    // Esquema provável:
    //   math.glyph_info?.kern_infos?.get(glyph_id)
    //     → Option<MathKernInfoRecord>
    //   record.top_right / top_left / bottom_right / bottom_left
    //     → Option<MathKern>
    //   kern.count() → u16  (número de registos)
    //   kern.subtable(i) → (correction_height: MathValueRecord, kern_value: MathValueRecord)

    // Se a API diferir do esquema acima, adaptar antes de codificar (ver diagnóstico).

    let kern_infos = match math.glyph_info.and_then(|gi| gi.kern_infos) {
        Some(k) => k,
        None => return MathGlyphKern::default(),
    };

    let kern_record = match kern_infos.get(glyph_id) {
        Some(r) => r,
        None => return MathGlyphKern::default(),
    };

    fn read_kern_table(kern: Option<ttf_parser::math::MathKern>) -> MathKernTable {
        let kern = match kern {
            Some(k) => k,
            None => return MathKernTable::default(),
        };
        let count = kern.count() as usize;
        let mut records = Vec::with_capacity(count + 1);
        for i in 0..count {
            // correction_height[i] separa kern[i] de kern[i+1]
            let height = kern.correction_height(i).map(|v| v.value as f64);
            let kv = kern.kern_value(i).map(|v| v.value as f64).unwrap_or(0.0);
            records.push(MathKernRecord {
                correction_height: height,
                kern_value: kv,
            });
        }
        // Último registo: kern após o último correction_height
        if let Some(kv) = kern.kern_value(count).map(|v| v.value as f64) {
            records.push(MathKernRecord {
                correction_height: None,
                kern_value: kv,
            });
        }
        MathKernTable { records }
    }

    MathGlyphKern {
        top_right:    read_kern_table(kern_record.top_right),
        top_left:     read_kern_table(kern_record.top_left),
        bottom_right: read_kern_table(kern_record.bottom_right),
        bottom_left:  read_kern_table(kern_record.bottom_left),
    }
}
```

**Nota**: os nomes dos métodos (`correction_height`, `kern_value`, `count`,
`glyph_info`, `kern_infos`) devem ser confirmados no diagnóstico. Se
diferirem, adaptar antes de codificar.

### Tarefa 6 — layout_attach usa MathKernInfo

Em `01_core/src/rules/math/layout.rs`, modificar `layout_attach` para
consultar `math_kern` da base e ajustar o deslocamento horizontal dos scripts.

```rust
// Em layout_attach — após calcular as posições base de sup e sub:

// Kern para superscript: quadrante top-right da base.
// A altura de conexão do superscript é a sua descent (ponto inferior do script).
// Em design units: sup_descent_du = sup_box.descent.val() * upem / size
let base_kern = self.metrics.math_kern(base_char);

if let Some(sup_box) = &sup_box {
    let connection_height_du =
        sup_box.descent.val() * self.constants.upem / self.size.val();
    let kern_du = base_kern.top_right.kern_at(connection_height_du);
    let kern_pt = Pt(kern_du * self.size.val() / self.constants.upem);
    // Somar kern_pt ao x do superscript (pode ser negativo → aproximar)
    sup_x_offset = sup_x_offset + kern_pt;
}

if let Some(sub_box) = &sub_box {
    // Kern para subscript: quadrante bottom-right da base.
    // A altura de conexão é o ascent do subscript (ponto superior).
    let connection_height_du =
        sub_box.ascent.val() * self.constants.upem / self.size.val();
    let kern_du = base_kern.bottom_right.kern_at(connection_height_du);
    let kern_pt = Pt(kern_du * self.size.val() / self.constants.upem);
    sup_x_offset = sup_x_offset + kern_pt; // aplicar ao sub também por alinhamento
}
```

**Simplificação permitida**: usar apenas os quadrantes `top_right` e
`bottom_right` (scripts à direita da base). Quadrantes `top_left` e
`bottom_left` (scripts à esquerda, ex: limites de integral) ficam para
Passo 45+.

**`base_char`**: o caractere da base precisa de estar disponível em
`layout_attach`. Se actualmente a base é um `MathBox` sem referência ao
char original, adicionar um método que extraia o primeiro char do texto
dos items do `MathBox`, ou passar o char directamente. Confirmar no
diagnóstico como a base é representada.

---

## Tarefa 7 — Testes

### Testes unitários em L1

```rust
#[cfg(test)]
mod tests_math_kern {
    use super::*;
    use crate::entities::glyph_variants::{MathKernRecord, MathKernTable, MathGlyphKern};

    #[test]
    fn kern_at_abaixo_do_primeiro_limiar() {
        let t = MathKernTable {
            records: vec![
                MathKernRecord { correction_height: Some(300.0), kern_value: -50.0 },
                MathKernRecord { correction_height: Some(600.0), kern_value: -30.0 },
                MathKernRecord { correction_height: None,        kern_value: -10.0 },
            ],
        };
        assert_eq!(t.kern_at(200.0), -50.0);
    }

    #[test]
    fn kern_at_entre_limiares() {
        let t = MathKernTable {
            records: vec![
                MathKernRecord { correction_height: Some(300.0), kern_value: -50.0 },
                MathKernRecord { correction_height: Some(600.0), kern_value: -30.0 },
                MathKernRecord { correction_height: None,        kern_value: -10.0 },
            ],
        };
        assert_eq!(t.kern_at(450.0), -30.0);
    }

    #[test]
    fn kern_at_acima_de_todos_os_limiares() {
        let t = MathKernTable {
            records: vec![
                MathKernRecord { correction_height: Some(300.0), kern_value: -50.0 },
                MathKernRecord { correction_height: None,        kern_value: -10.0 },
            ],
        };
        assert_eq!(t.kern_at(999.0), -10.0);
    }

    #[test]
    fn kern_at_tabela_vazia_retorna_zero() {
        let t = MathKernTable::default();
        assert_eq!(t.kern_at(500.0), 0.0);
    }

    #[test]
    fn fixed_metrics_math_kern_vazio() {
        let m = FixedMetrics;
        let k = m.math_kern('f');
        assert!(k.top_right.is_empty());
        assert!(k.bottom_right.is_empty());
    }

    // ── AxisHeight ───────────────────────────────────────────────────────

    #[test]
    fn frac_com_axis_height_nao_regride() {
        // Fracção deve continuar a produzir conteúdo após AxisHeight
        let doc = layout_test("$frac(a, b)$");
        let text = doc.plain_text();
        assert!(text.contains('a'));
        assert!(text.contains('b'));
    }

    #[test]
    fn delimitado_com_axis_height_nao_regride() {
        let doc = layout_test("$(frac(a, b))$");
        let text = doc.plain_text();
        assert!(text.contains('a'));
        assert!(text.contains('b'));
    }

    #[test]
    fn sqrt_com_axis_height_nao_regride() {
        let doc = layout_test("$sqrt(x)$");
        let text = doc.plain_text();
        assert!(text.contains('√') || text.contains('x'));
    }

    #[test]
    fn attach_com_kern_nao_regride() {
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('2'));
    }

    #[test]
    fn attach_sub_com_kern_nao_regride() {
        let doc = layout_test("$x_i$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('i'));
    }
}
```

### Testes em L3

```rust
#[cfg(test)]
mod tests_kern_export {

    #[test]
    fn pdf_frac_inline_nao_vazio() {
        // Fracção inline com AxisHeight activo
        let pdf = compile_to_pdf("Valor: $frac(1, 2)$.");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_attach_sup_sub_nao_vazio() {
        let pdf = compile_to_pdf("$x^2 + y_i$");
        assert!(!pdf.is_empty());
    }

    #[test]
    #[ignore = "requer fonte com tabela MATH em tests/fixtures/"]
    fn font_math_tem_kern_para_f() {
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/stix-two-math.otf")
        ).expect("fixture necessária");
        let m = FontBookMetrics::from_bytes(&data).unwrap();
        // Apenas confirmar que não pânica — kern pode ser vazio para 'f'
        let k = m.math_kern('f');
        let _ = k;
    }

    #[test]
    fn fixed_metrics_math_kern_nao_panica() {
        let m = FixedMetrics;
        let k = m.math_kern('V');
        assert!(k.top_right.is_empty());
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

- [ ] `axis_height` está em `MathConstants` (já existente ou adicionado)
- [ ] `MathLayouter::axis_offset()` existe e calcula o deslocamento correcto
- [ ] `layout_frac` aplica `axis_offset` ao bloco resultante
- [ ] `layout_delimited` aplica `axis_offset` ao bloco resultante
- [ ] `layout_root` aplica `axis_offset` ao bloco resultante
- [ ] `layout_attach` não aplica `axis_offset` (elementos inline)
- [ ] `MathKernRecord`, `MathKernTable`, `MathGlyphKern` existem em `entities/glyph_variants.rs`
- [ ] `MathKernTable::kern_at()` retorna o kern correcto por intervalo de altura
- [ ] `FontMetrics::math_kern()` tem default com todos os quadrantes vazios
- [ ] `FontBookMetrics` lê `MathKernInfo` da tabela MATH quando disponível
- [ ] `FontBookMetrics` retorna kern vazio quando fonte não tem tabela MATH ou não tem kern para o glifo
- [ ] `layout_attach` consulta `math_kern` da base e aplica kern a sup e sub
- [ ] Com `FixedMetrics`, kern é zero — comportamento idêntico ao Passo 43
- [ ] Todos os testes de regressão de frac/attach/sqrt/delimited/assembly passam
- [ ] `ttf-parser` não aparece em `01_core/Cargo.toml`
- [ ] Zero violations no linter

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `axis_height` já estava em `MathConstants` ou foi necessário adicioná-lo
- API exacta de `MathKernInfo` no `ttf-parser` (nomes dos métodos, estrutura de acesso)
- Se `base_char` estava disponível em `layout_attach` ou foi necessário extraí-lo
- Se `MathKernInfo` estava acessível (sub-âmbito B executado ou não)

**Da implementação:**
- Se `axis_offset` mudou visualmente a posição de fracções (esperado: sim)
- Se `offset_item` foi generalizado para aceitar `dy` ou se foi usada outra abordagem
- Se o kern foi aplicado apenas a `top_right`/`bottom_right` ou também aos quadrantes esquerdos

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 45:**
- **GO — ToUnicode para FrameItem::Glyph (DEBT-9)**: se AxisHeight e kern funcionam, Passo 45 actualiza o mapa `ToUnicode` do PDF para incluir o `char` original de cada glifo de variante emitido, restaurando a extracção de texto
- **GO — Scripts à esquerda**: quadrantes `top_left`/`bottom_left` para limites de integrais e outros operadores com scripts à esquerda
- **NO-GO — ttf-parser sem MathKernInfo**: se a tabela não for acessível, registar como limitação e avançar para DEBT-9

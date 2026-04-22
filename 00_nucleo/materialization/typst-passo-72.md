# Passo 72 — Dimensões Reais de Imagem e Aspect Ratio (DEBT-24b)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/` — em particular ADR-0001 e os ADRs que definem
  `[l1_allowed_external]`. Qualquer crate nova em L1 que não esteja
  na whitelist dispara V14 e bloqueia `crystalline-lint`.
- `01_core/Cargo.toml` — lista actual de dependências de L1.
- `01_core/src/rules/layout/mod.rs` — braço `Content::Image` com
  placeholder 100×100 do Passo 71.
- `01_core/src/entities/value.rs` — para confirmar como `Value`
  representa medidas (`Length`, `Float`, ou outro).

Pré-condição: `cargo test` — 701 L1 + 125 L3, zero violations.
DEBT-24b e DEBT-26 registados.

---

## Contexto arquitectural — Porque `imagesize` não pode entrar em L1

A arquitectura cristalina declara em `crystalline.toml` a lista
`[l1_allowed_external]`. Actualmente contém apenas `thiserror` e
`comemo` (ADR-0001). Adicionar qualquer crate nova a L1 sem ADR
aprovado dispara **V14 — ExternalTypeInContract** e bloqueia o
linter. A regra é explícita no CLAUDE.md:

> "Qualquer outro crate que V14 sinalize em L1 → criar ADR antes
> de adicionar à whitelist. Não adicionar por conveniência."

A crate `imagesize` é infraestrutura de I/O (lê bytes de formatos
de ficheiro) — pertence a L3, não a L1. A solução correcta é:

1. L1 define um trait `ImageSizer` — contrato puro.
2. L3 implementa o trait usando `imagesize` (ou qualquer outra
   biblioteca).
3. O cálculo de aspect ratio (matemática pura) fica em L1.
4. O layouter recebe as dimensões já calculadas via injecção.

Este padrão é o mesmo que já existe para `FontMetrics` (Passo 20).

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar l1_allowed_external actual
grep -A 20 "l1_allowed_external" crystalline.toml | head -25

# 2. Confirmar o braço Content::Image no layouter do Passo 71
grep -n "Content::Image" 01_core/src/rules/layout/mod.rs -A 8 | head -20

# 3. Confirmar como Value representa medidas
grep -n "Length\|Float\|Pt\b" 01_core/src/entities/value.rs | head -20

# 4. Verificar se ImageSizer ou trait equivalente já existe
grep -rn "ImageSizer\|image_size\|blob_size" 01_core/src/ 2>/dev/null | head -10

# 5. Confirmar onde FontMetrics é injectado (para seguir o mesmo padrão)
grep -n "FontMetrics\|fn layout" 01_core/src/rules/layout/mod.rs | head -10
```

Reportar o output completo antes de continuar. O diagnóstico 3 é
crítico: a função `extract_pt` da Tarefa 2 depende de como `Value`
representa medidas — se for `Value::Float(f)`, `Value::Length(pt)`,
ou outro.

---

## Tarefa 0 — Actualizar DEBT.md

Antes de qualquer código:

```markdown
### DEBT-24b — Dimensões reais de imagem — ENCERRADO (Passo 72) ✓
Placeholder 100×100 substituído por leitura real de dimensões via
trait ImageSizer injectado no layouter. L3 implementa com imagesize
ou std::fs; L1 mantém-se puro.
```

---

## Tarefa 1 — Trait `ImageSizer` em L1

Criar `01_core/src/entities/image_sizer.rs`:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/image-sizer.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-04-19

/// Contrato para leitura das dimensões intrínsecas de uma imagem.
///
/// Implementado em L3 com imagesize ou equivalente.
/// L1 define apenas o contrato — zero dependências externas.
pub trait ImageSizer {
    /// Retorna (largura_px, altura_px) ou None se os bytes forem inválidos.
    fn size(&self, data: &[u8]) -> Option<(u32, u32)>;
}

/// Implementação nula — retorna sempre None.
/// Usada em testes L1 que não precisam de dimensões reais.
pub struct NullImageSizer;

impl ImageSizer for NullImageSizer {
    fn size(&self, _data: &[u8]) -> Option<(u32, u32)> {
        None
    }
}
```

Criar o prompt L0 `00_nucleo/prompts/entities/image-sizer.md` antes
de continuar:

```bash
git add 00_nucleo/prompts/entities/image-sizer.md
crystalline-lint --fix-hashes .
```

Registar em `entities/mod.rs`:

```rust
pub mod image_sizer;
```

---

## Tarefa 2 — Motor de cálculo de dimensões em L1

Criar `01_core/src/rules/layout/image.rs`:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout-image.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-04-19

use crate::entities::image_sizer::ImageSizer;
use crate::entities::value::Value;

/// Densidade padrão para conversão px → pt.
/// 96 DPI: 1 pt = 1/72 inch; 1 px = 1/96 inch → 1 px = 72/96 pt = 0.75 pt.
const PX_TO_PT: f64 = 0.75;

/// Dimensões finais de uma imagem para o layouter, em pontos.
pub struct ImageDimensions {
    pub width_pt:  f64,
    pub height_pt: f64,
}

/// Calcula as dimensões finais de uma imagem.
///
/// 1. Lê dimensões intrínsecas em píxeis via `sizer`.
/// 2. Converte para pontos (96 DPI).
/// 3. Aplica overrides do utilizador preservando o aspect ratio se apenas
///    um dos valores for fornecido.
///
/// Se `sizer` não conseguir ler os bytes (formato inválido ou desconhecido),
/// usa o fallback 100×100 pt para não bloquear o layout.
pub fn calculate_dimensions(
    data:        &[u8],
    user_width:  Option<&Value>,
    user_height: Option<&Value>,
    sizer:       &dyn ImageSizer,
) -> ImageDimensions {
    // 1. Dimensões intrínsecas
    let (intrinsic_w_pt, intrinsic_h_pt) = match sizer.size(data) {
        Some((pw, ph)) => (pw as f64 * PX_TO_PT, ph as f64 * PX_TO_PT),
        None           => (100.0, 100.0), // fallback documentado
    };

    let aspect = if intrinsic_h_pt > 0.0 {
        intrinsic_w_pt / intrinsic_h_pt
    } else {
        1.0
    };

    // 2. Overrides do utilizador
    let req_w = user_width.and_then(extract_pt);
    let req_h = user_height.and_then(extract_pt);

    let (width_pt, height_pt) = match (req_w, req_h) {
        (Some(w), Some(h)) => (w, h),                  // ambos forçados
        (Some(w), None)    => (w, w / aspect),          // preserva proporção
        (None, Some(h))    => (h * aspect, h),          // preserva proporção
        (None, None)       => (intrinsic_w_pt, intrinsic_h_pt), // tamanho original
    };

    ImageDimensions { width_pt, height_pt }
}

/// Extrai o valor em pontos de um Value.
///
/// Adaptar ao tipo real de `Value` conforme o diagnóstico 3:
/// - Se `Value::Float(f)` representa pontos directamente → `Some(*f)`
/// - Se `Value::Length(l)` → `Some(l.to_pt())`
/// Retorna None para tipos que não representam medidas lineares.
fn extract_pt(val: &Value) -> Option<f64> {
    match val {
        Value::Float(f) => Some(*f),
        // Value::Length(l) => Some(l.to_pt()),  // descomentar se existir
        _ => None,
    }
}
```

Criar o prompt L0 `00_nucleo/prompts/rules/layout-image.md` antes
de continuar.

Registar o módulo em `layout/mod.rs`:

```rust
pub mod image;
```

---

## Tarefa 3 — Implementação de `ImageSizer` em L3

Em `01_infra/src/` (ou onde L3 vive), criar
`01_infra/src/image_sizer.rs`:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/image-sizer.md
//! @prompt-hash <hash>
//! @layer L3
//! @updated 2026-04-19

use typst_core::entities::image_sizer::ImageSizer;

/// Implementação de ImageSizer usando a crate imagesize.
/// imagesize lê apenas o cabeçalho do ficheiro — não descodifica píxeis.
pub struct ImageSizeImageSizer;

impl ImageSizer for ImageSizeImageSizer {
    fn size(&self, data: &[u8]) -> Option<(u32, u32)> {
        imagesize::blob_size(data)
            .ok()
            .map(|s| (s.width as u32, s.height as u32))
    }
}
```

Adicionar `imagesize` ao `Cargo.toml` de L3 (não de L1):

```toml
# 01_infra/Cargo.toml  (ou 03_infra/Cargo.toml — confirmar estrutura)
[dependencies]
imagesize = "0.12"
```

`imagesize` nunca aparece em L1 — V14 não dispara.

---

## Tarefa 4 — Injectar `ImageSizer` no layouter (L1)

O layouter já recebe `FontMetrics` por injecção. Seguir o mesmo
padrão para `ImageSizer`.

Confirmar com o diagnóstico 5 como `FontMetrics` é passado ao
`Layouter`. Se for via campo da struct:

```rust
pub struct Layouter<'a, M: FontMetrics> {
    metrics: &'a M,
    // ... outros campos ...
}
```

Adicionar `ImageSizer` da mesma forma:

```rust
pub struct Layouter<'a, M: FontMetrics, S: ImageSizer> {
    metrics: &'a M,
    sizer:   &'a S,
    // ...
}
```

Se o layouter usa generics diferentes ou injecção por trait object,
seguir o padrão existente sem alterar a convenção.

**Efeito cascata nos testes existentes (esperado e necessário):**
ao adicionar o parâmetro `S: ImageSizer`, todos os testes de L1 que
instanciam `Layouter::new(...)` deixam de compilar — falta o argumento
para `sizer`. Isto é o comportamento correcto: o compilador está a
aplicar o contrato de injecção de dependências.

A correcção é mecânica: em cada teste existente que instancie o
`Layouter` directamente, passar `&NullImageSizer` no lugar do novo
parâmetro. A lista de testes afectados pode ser obtida com:

```bash
cargo test 2>&1 | grep "error\[E" | grep -i "layouter\|layout" | head -20
```

Não usar `_ = sizer` ou `PhantomData` para contornar o erro — a injecção
tem de ser explícita em todos os pontos de construção.

Actualizar o braço `Content::Image` no layouter:

```rust
Content::Image { path: _, data, width, height } => {
    // Verificar com o diagnóstico 3 se width/height são Option<Value>
    // ou Option<Box<Value>>. Se Value contém Content que contém Value,
    // o compilador exige Box para ter tamanho finito — e nesse caso
    // as_deref() é obrigatório (as_ref() daria Option<&Box<Value>>).
    let dims = image::calculate_dimensions(
        data,
        width.as_deref(),   // Option<Box<Value>> → Option<&Value>
        height.as_deref(),  // Option<Value>       → Option<&Value> (ambos funcionam)
        self.sizer,
    );

    self.flush_line();

    // Avançar o cursor pelo espaço da imagem.
    // Seguir o padrão dos outros nós de bloco (Figure, Heading, etc.).
    // Se existir FrameItem::Image, instanciar aqui.
    // Se não existir, reservar apenas o espaço vertical:
    self.cursor_y += Pt(dims.height_pt);

    // DEBT-24c: FrameItem::Image não implementado — imagem não aparece
    // no PDF. Passo 73 adiciona o stream XObject ao export.rs.
}
```

Registar em `DEBT.md`:

```markdown
### DEBT-24c — FrameItem::Image e export PDF (Passo 72)
O layouter reserva o espaço da imagem mas não emite FrameItem::Image.
A imagem não aparece no PDF — apenas o espaço em branco existe.
Resolução: Passo 73 adiciona FrameItem::Image e o stream XObject em export.rs.
```

---

## Tarefa 5 — Testes

### Testes L1 — cálculo de dimensões (sem imagesize)

Os testes L1 usam `NullImageSizer` — não dependem de `imagesize`.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::value::Value;

    #[test]
    fn dimensoes_fallback_quando_sizer_retorna_none() {
        // NullImageSizer retorna sempre None → fallback 100×100
        let dims = calculate_dimensions(&[], None, None, &NullImageSizer);
        assert_eq!(dims.width_pt,  100.0);
        assert_eq!(dims.height_pt, 100.0);
    }

    #[test]
    fn dimensoes_intrinsecas_sem_overrides() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((400, 300)) // 400×300 px
            }
        }
        // 400 * 0.75 = 300pt; 300 * 0.75 = 225pt
        let dims = calculate_dimensions(&[], None, None, &MockSizer);
        assert_eq!(dims.width_pt,  300.0);
        assert_eq!(dims.height_pt, 225.0);
    }

    #[test]
    fn override_width_preserva_aspect_ratio() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((400, 300)) // aspect ratio 4:3
            }
        }
        // Forçar width = 120pt → height = 120 / (4/3) = 90pt
        let w = Value::Float(120.0);
        let dims = calculate_dimensions(&[], Some(&w), None, &MockSizer);
        assert_eq!(dims.width_pt,  120.0);
        assert_eq!(dims.height_pt,  90.0);
    }

    #[test]
    fn override_height_preserva_aspect_ratio() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((400, 300)) // aspect ratio 4:3
            }
        }
        // Forçar height = 90pt → width = 90 * (4/3) = 120pt
        let h = Value::Float(90.0);
        let dims = calculate_dimensions(&[], None, Some(&h), &MockSizer);
        assert_eq!(dims.width_pt,  120.0);
        assert_eq!(dims.height_pt,  90.0);
    }

    #[test]
    fn ambos_overrides_forcam_dimensoes() {
        struct MockSizer;
        impl ImageSizer for MockSizer {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((400, 300))
            }
        }
        // Forçar 50×50 — ignora aspect ratio
        let w = Value::Float(50.0);
        let h = Value::Float(50.0);
        let dims = calculate_dimensions(&[], Some(&w), Some(&h), &MockSizer);
        assert_eq!(dims.width_pt, 50.0);
        assert_eq!(dims.height_pt, 50.0);
    }
}
```

### Teste L3 — `ImageSizeImageSizer` com bytes reais

```rust
#[test]
fn image_sizer_le_cabecalho_png_1x1() {
    use crate::image_sizer::ImageSizeImageSizer;
    use typst_core::entities::image_sizer::ImageSizer;

    // PNG 1×1 px transparente — bytes do cabeçalho suficientes para imagesize
    let png_1x1: &[u8] = &[
        137, 80, 78, 71, 13, 10, 26, 10,
        0, 0, 0, 13, 73, 72, 68, 82,
        0, 0, 0, 1, 0, 0, 0, 1,
        8, 6, 0, 0, 0, 31, 21, 196, 137,
        0, 0, 0, 11, 73, 68, 65, 84,
        8, 215, 99, 96, 0, 2, 0, 0,
        5, 0, 1, 226, 38, 5, 155,
        0, 0, 0, 0, 73, 69, 78, 68,
        174, 66, 96, 130,
    ];

    let sizer = ImageSizeImageSizer;
    let result = sizer.size(png_1x1);
    assert_eq!(result, Some((1, 1)),
        "imagesize deve ler cabeçalho PNG 1×1: {:?}", result);
}
```

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de conclusão:
- [ ] `imagesize` adicionado ao `Cargo.toml` de **L3**, não de L1.
  `crystalline-lint` não dispara V14.
- [ ] Prompt L0 `entities/image-sizer.md` criado e commitado antes
  de qualquer código.
- [ ] Prompt L0 `rules/layout-image.md` criado e commitado.
- [ ] `ImageSizer` trait e `NullImageSizer` em `entities/image_sizer.rs` (L1).
- [ ] `calculate_dimensions` em `layout/image.rs` (L1) — sem imports externos.
- [ ] `ImageSizeImageSizer` em L3 usando `imagesize`.
- [ ] `ImageSizer` injectado no `Layouter` seguindo o padrão de `FontMetrics`.
- [ ] Todos os testes existentes do layouter actualizados com `&NullImageSizer`
  — nenhum contorna o parâmetro com `PhantomData` ou `_`.
- [ ] Braço `Content::Image` no layouter usa `calculate_dimensions`.
- [ ] DEBT-24b marcado como **encerrado** em `00_nucleo/DEBT.md`.
- [ ] DEBT-24c registado em `00_nucleo/DEBT.md`.
- [ ] Testes L1 de aspect ratio passam sem `imagesize`.
- [ ] Teste L3 com bytes PNG reais passa.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Confirmação de que `imagesize` não aparece em L1 (V14 não dispara).
- Como `Value` representa medidas — se `extract_pt` precisou de ser
  adaptado para além de `Value::Float`.
- Se o `Layouter` já tinha dois parâmetros de tipo (`M: FontMetrics`)
  ou se a injecção de `ImageSizer` exigiu uma mudança de assinatura.

**Da implementação:**
- Se os testes de aspect ratio passaram à primeira.
- Se o teste L3 com bytes PNG reais passou — confirmar que `imagesize`
  consegue ler o cabeçalho mínimo.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 73:**
- **GO — FrameItem::Image e export PDF (DEBT-24c):** com as dimensões
  correctas no layouter, Passo 73 adiciona `FrameItem::Image` e emite
  o stream XObject no PDF.
- **NO-GO — V14 disparado:** se `imagesize` entrou em L1 por engano;
  Passo 73 move-o para L3 antes de avançar.

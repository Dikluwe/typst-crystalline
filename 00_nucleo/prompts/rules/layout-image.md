# Prompt L0 — rules/layout/image
Hash do Código: 6a9aa0a8

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/layout/image.rs`
**ADRs relevantes**: ADR-0029 (pureza física), ADR-0001 (l1_allowed_external)

## Contexto

Motor de cálculo de dimensões de imagem para o layouter. Lê dimensões em píxeis
via `ImageSizer` (injectado), converte para pontos (96 DPI padrão) e aplica
overrides do utilizador preservando o aspect ratio quando apenas um eixo é
especificado.

## Constantes e tipos públicos

```rust
const PX_TO_PT: f64 = 0.75;  // 96 DPI: 1px = 72/96 pt

pub struct ImageDimensions {
    pub width_pt:  f64,
    pub height_pt: f64,
}
```

## Função principal

```rust
pub fn calculate_dimensions(
    data:        &[u8],
    user_width:  Option<&Value>,
    user_height: Option<&Value>,
    sizer:       &dyn ImageSizer,
) -> ImageDimensions
```

**Lógica:**
1. Tenta ler dimensões intrínsecas via `sizer.size(data)`.
2. Se `None`, usa fallback 100×100 pt.
3. Calcula aspect ratio (`w / h`; fallback 1.0 se `h == 0`).
4. Aplica overrides:
   - ambos fornecidos → usa directamente (ignora aspect ratio).
   - só largura → `height = width / aspect`.
   - só altura → `width = height * aspect`.
   - nenhum → dimensões intrínsecas convertidas.

## Função auxiliar

```rust
fn extract_pt(val: &Value) -> Option<f64>
```

- `Value::Float(f)` → `Some(*f)`.
- `Value::Length(l)` → `Some(l.abs.to_pt())` (componente absoluta; ignora em).
- Outros → `None`.

## Invariantes

- Zero dependências externas em L1 — `imagesize` nunca importado aqui.
- Fallback 100×100 documentado: não bloqueia o layout em formato desconhecido.
- Aspect ratio sempre preservado quando apenas um override é fornecido.

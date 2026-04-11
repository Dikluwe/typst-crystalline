# Passo 48 — Baselines em Equações Inline (Alinhamento do Eixo Matemático)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/layout_types.rs` — struct `Frame` e as suas métricas
- `01_core/src/entities/content.rs` — variante `Content::Equation { body, block }`
- `01_core/src/entities/math_constants.rs` — `MathConstants`, campo `axis_height`
- `01_core/src/rules/layout.rs` — onde `Content::Equation` é processada
- `01_core/src/rules/math/layout.rs` — `MathLayouter`, `apply_axis_offset`

Pré-condição: `cargo test` — 530 L1 + 87 L3 + 50 parity, zero violations.

---

## Contexto

Equações inline (`$x/2$`, `block == false`) são actualmente tratadas como
caixas opacas que assentam a baseline na linha do texto. Em tipografia
matemática correcta, o eixo matemático da equação (o meio da fracção,
definido por `axis_height`) deve alinhar-se com o eixo matemático do texto
circundante.

O `axis_height` já é lido desde o Passo 44 e está em `MathConstants`.
`apply_axis_offset` já desloca o conteúdo interno das fracções/raízes.
O que falta é ajustar a **baseline da `Frame`** da equação inline em relação
ao texto que a envolve — de modo a que o motor de parágrafo a posicione
correctamente na linha.

A distinção é:
- `apply_axis_offset` (Passo 44): desloca items dentro do `MathBox` para
  que o centro do bloco matemático coincida com o eixo. Afecta posições
  internas.
- Este passo: ajusta as métricas externas da `Frame` (ascent/descent ou
  baseline) para que o motor de parágrafo saiba onde posicionar a equação
  em relação à linha de texto. Afecta o posicionamento externo.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. Confirmar API da struct Frame — campos ascent, descent, baseline
grep -A 15 "pub struct Frame" 01_core/src/entities/layout_types.rs

# 2. Confirmar se Frame tem campo baseline separado ou se usa ascent
grep -n "baseline\|ascent\|descent" 01_core/src/entities/layout_types.rs | head -20

# 3. Confirmar onde Content::Equation é processada no layout
grep -n "Content::Equation\|Equation {" \
  01_core/src/rules/layout.rs \
  01_core/src/rules/math/layout.rs | head -10

# 4. Confirmar como o MathBox resultante é convertido em Frame
grep -n "fn.*to_frame\|Frame {.*items\|MathBox.*Frame\|into.*Frame" \
  01_core/src/rules/math/layout.rs \
  01_core/src/rules/layout.rs | head -15

# 5. Confirmar se PagedDocument / Frame expõe ascent/descent nos testes
grep -n "fn plain_text\|fn ascent\|fn descent\|fn baseline" \
  01_core/src/entities/layout_types.rs | head -10

# 6. Confirmar como axis_height está em MathConstants e a conversão to_pt
grep -n "axis_height\|to_pt\|upem" 01_core/src/entities/math_constants.rs
```

**Reportar o output antes de continuar.**

O diagnóstico é obrigatório antes de codificar a Tarefa 1 porque a fórmula
exacta de ajuste depende da API da `Frame`:

- Se `Frame` tem campo `baseline: Pt` separado de `ascent`:
  o ajuste é `frame.baseline += axis_pt` (deslocar o ponto de referência
  sem mover o conteúdo).
- Se `Frame` representa a baseline implicitamente como `ascent` (origem no
  topo, baseline = ascent):
  o ajuste é `frame.ascent -= axis_pt` (reduzir o ascent desloca a baseline
  para cima, aproximando o eixo matemático da linha de texto).

Ambos os casos produzem o mesmo efeito visual: a equação inline sobe
`axis_pt` em relação à baseline do texto circundante.

---

## Tarefa 1 — Ajuste da baseline da Frame para equações inline (L1)

No sítio onde `Content::Equation` produz uma `Frame` (confirmar no
diagnóstico — provavelmente em `layout_content` em `rules/layout.rs` ou
no final do `MathLayouter`):

```rust
// Após obter a frame da equação (math_frame):

if !block {
    // axis_height em design units → pt
    let axis_pt: Pt = size * Pt(constants.axis_height / constants.upem);

    // Ajustar baseline conforme API da Frame (confirmar no diagnóstico):
    //
    // Opção A — Frame com campo baseline separado:
    //   math_frame.baseline = math_frame.baseline + axis_pt;
    //
    // Opção B — Frame sem baseline separado (baseline = ascent implícito):
    //   math_frame.ascent = math_frame.ascent - axis_pt;
    //   math_frame.descent = math_frame.descent + axis_pt;
    //   (o conteúdo não se move — apenas as métricas externas mudam)
    //
    // Escolher a opção correcta com base no diagnóstico 1 e 2.
}
```

Se `FixedMetrics` não tem `axis_height` (porque `MathConstants::fallback()`
retorna um valor de fallback), o ajuste será aplicado com o valor de fallback
— o comportamento é previsível e não causa panic.

**Equações block** (`block == true`): não aplicar ajuste de baseline — estas
equações são posicionadas pelo motor de parágrafo como blocos separados, não
inline na linha de texto.

---

## Tarefa 2 — Testes

### Testes unitários em L1

```rust
#[cfg(test)]
mod tests_inline_baseline {
    use super::*;

    #[test]
    fn equacao_inline_nao_regride_conteudo() {
        // O conteúdo continua presente após o ajuste de baseline
        let doc = layout_test("$frac(1, 2)$");
        let text = doc.plain_text();
        assert!(text.contains('1'), "numerador: {}", text);
        assert!(text.contains('2'), "denominador: {}", text);
    }

    #[test]
    fn equacao_inline_simples_nao_regride() {
        let doc = layout_test("$x + 1$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('1'));
    }

    #[test]
    fn equacao_inline_com_attach_nao_regride() {
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('2'));
    }

    #[test]
    fn equacao_inline_com_prime_nao_regride() {
        let doc = layout_test("$x'$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('′'));
    }

    #[test]
    fn pagina_nao_vazia_com_equacao_inline() {
        // Motor de parágrafo não entra em panic com baseline ajustada
        let doc = layout_test("$frac(1, 2)$");
        assert!(!doc.pages.is_empty());
        assert!(!doc.pages[0].items.is_empty());
    }
}
```

### Testes em L3

```rust
#[cfg(test)]
mod tests_inline_baseline_pdf {

    #[test]
    fn pdf_equacao_inline_frac_nao_vazio() {
        let pdf = compile_to_pdf("$frac(1, 2)$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_equacao_inline_com_texto_nao_vazio() {
        // Texto misto: parágrafo com equação inline
        let pdf = compile_to_pdf("Valor: $frac(1, 2)$ calculado.");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_equacao_inline_contem_bt_et() {
        let pdf = compile_to_pdf("$x^2 + 1$");
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("BT"));
        assert!(s.contains("ET"));
    }

    #[test]
    fn pdf_equacao_inline_com_sqrt_nao_vazio() {
        let pdf = compile_to_pdf("$sqrt(x)$");
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

- [ ] O sítio de processamento de `Content::Equation` foi identificado
- [ ] `block == false` activa o ajuste de baseline
- [ ] `block == true` não altera baseline (equações de bloco)
- [ ] A fórmula de ajuste usa `axis_height` de `MathConstants` convertido para pt
- [ ] Com `FixedMetrics` (fallback), o ajuste usa o valor de fallback sem panic
- [ ] Todos os testes de regressão de frac/sqrt/attach/prime/delimited passam
- [ ] Motor de parágrafo não entra em panic com baseline modificada
- [ ] `ttf-parser` não aparece em `01_core/Cargo.toml`
- [ ] Zero violations no linter

---

## Ao terminar, reportar

**Do diagnóstico:**
- API exacta da `Frame` — tem campo `baseline` separado ou usa `ascent` implícito
- Sítio exacto onde `Content::Equation` produz a `Frame` (ficheiro e função)
- Qual das duas opções (A ou B) foi usada

**Da implementação:**
- Se o ajuste de baseline mudou visualmente a posição de fracções inline
  (esperado: sim — a equação sobe ligeiramente em relação à linha de texto)
- Se foi necessário propagar `block` até ao sítio de ajuste ou se já estava
  acessível no contexto

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 49:**
- **GO — Kern diferenciado por quadrante para left-scripts**: se inline baseline funciona, Passo 49 refina kern de `tl`/`bl` independentes (simplificação do Passo 46)
- **GO — MathAlignPoint**: suporte a `&` em equações alinhadas
- **NO-GO — Frame sem API de baseline**: se a struct `Frame` não tiver forma de ajustar a baseline sem mover o conteúdo, reportar e redesenhar antes de avançar

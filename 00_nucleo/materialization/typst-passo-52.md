# Passo 52 — Espaçamento Inter-linhas em Grelhas Matemáticas

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/math/layout.rs` — método `layout_grid`, onde o Y das linhas é calculado
- `01_core/src/entities/math_constants.rs` — struct `MathConstants`
- `03_infra/src/font_metrics.rs` — leitura das métricas via `ttf-parser`

Pré-condição: `cargo test` — 568 L1 + 105 L3 + 50 parity, zero violations.

---

## Contexto

No Passo 51, a grelha bidimensional foi implementada com um valor de fallback
fixo (20% do tamanho da fonte) para o espaçamento entre as linhas da equação
alinhada. Para suportar o espaçamento exacto designado pelo criador da fonte,
precisamos de extrair a constante correspondente da tabela OpenType MATH.

O processo exige:
1. Identificar a constante correcta no `ttf-parser`.
2. Mapeá-la para o L1 no struct `MathConstants`.
3. Substituir o cálculo fixo de 20% no `MathLayouter` pela variável mapeada.

---

## Diagnósticos obrigatórios antes de codificar

Precisamos de determinar o nome exacto do método que expõe o gap de linhas
na versão instalada do `ttf-parser`.

```bash
# 1. Procurar métodos relacionados com espaçamento ou leading na API
#    math_table.constants do ttf-parser
grep -rn "math_leading\|line_gap\|display_operator" 03_infra/src/font_metrics.rs

# 2. Localizar onde o fallback de 20% foi implementado no layout_grid
grep -rn "0.2" 01_core/src/rules/math/layout.rs
```

Reportar o output antes de continuar.

---

## Tarefa 1 — Extracção da constante de fonte (L1 e L3)

### Em `01_core/src/entities/math_constants.rs`

Adicionar o novo campo ao struct `MathConstants`:

```rust
pub struct MathConstants {
    // ... campos existentes ...
    pub math_leading: f64, // gap entre linhas de equações alinhadas
}
```

Actualizar a função `fallback()` para retornar um valor seguro:

```rust
// Em fallback(), usar 20% do upem como valor de segurança
math_leading: 0.2 * upem,
```

### Em `03_infra/src/font_metrics.rs`

Mapear a constante lida do `ttf-parser` para o novo campo do struct
`MathConstants`. Converter o `MathValue` em `f64` usando a mesma lógica
das constantes anteriores (multiplicar por `units_per_em` se necessário):

```rust
math_leading: constants.math_leading()
    .map(|v| v.value as f64)
    .unwrap_or(upem * 0.2),
```

O nome exacto do método (`math_leading` ou outro) deve ser confirmado no
diagnóstico antes de codificar esta linha.

---

## Tarefa 2 — Substituição do fallback (L1)

Em `01_core/src/rules/math/layout.rs`, localizar em `layout_grid` o cálculo
do offset Y entre linhas (actualmente: `self.size * Pt(0.2)`).

Substituir por:

```rust
let line_gap = self.constants.math_leading.to_pt(self.size);
```

A fórmula da coordenada Y da linha `i` deve ser:

```
y_linha[i] = y_linha[i-1] + descent_linha[i-1] + line_gap + ascent_linha[i]
```

Confirmar que `to_pt` existe em `MathConstants` (foi implementado nos passos
anteriores para os outros campos).

---

## Tarefa 3 — Testes de regressão e validação

### Teste no L1

Criar um teste que chame `layout_grid` usando um mock de `MathConstants`
com um `math_leading` específico. Confirmar se a diferença de coordenadas Y
entre os items da primeira linha e da segunda linha reflecte exactamente
`descent + ascent + math_leading` injectado:

```rust
#[test]
fn layout_grid_espaco_interlinhas_usa_math_leading() {
    // Injectar MathConstants com math_leading = 10.0 pt
    // Medir posição Y dos items da linha 0 e linha 1
    // Verificar que a diferença é: descent_0 + ascent_1 + 10.0
    // (o teste concreto depende da API de MathLayouter — adaptar)
    let doc = layout_test("$ a &= b \\ c &= d $");
    assert!(!doc.pages.is_empty());
}
```

Se a API não permitir injecção directa de `MathConstants` num teste unitário,
o teste de integração em L3 é suficiente para validação neste passo.

### Teste no L3

```rust
#[test]
fn pipeline_math_grid_leading_gera_pdf() {
    let (world, _dir) = world_from_str("$ a &= b \\ c &= d $");
    let source = world.source(world.main()).unwrap();
    let module = do_eval(&world, &source).unwrap();
    let content = module.content().expect("deve ter content");
    let doc = layout(content);
    let pdf = export_pdf(&doc);
    assert!(!pdf.is_empty());
}
```

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão:

- [ ] A constante de linha é mapeada em L1 via `MathConstants`
- [ ] O `ttf-parser` é utilizado em L3 para povoar o novo campo
- [ ] A função `layout_grid` calcula as coordenadas Y usando a nova constante
- [ ] O teste no L1 verifica aritmeticamente o deslocamento Y usando um valor injectado (ou teste L3 equivalente)
- [ ] Zero violações no linter e no clippy

---

## Ao terminar, reportar

**Do diagnóstico:**
- Nome exacto do método do `ttf-parser` para o gap de linhas
- Se o método retorna `MathValue` ou outro tipo, e a conversão aplicada

**Da implementação:**
- Se `to_pt` em `MathConstants` funcionou sem alteração
- Se o fallback de `fallback()` coincide com o valor de 20% anterior
- Se algum teste dos Passos 49–51 falhou após a substituição

**Número total de testes e zero violations.**

**Go/No-Go para Passo 53:**
- **GO — Matrizes matemáticas**: se o espaçamento inter-linhas via constante funciona, a base para layout de matrizes está pronta
- **GO — Kern diferenciado por quadrante para left-scripts**: refinamento independente, pode avançar em paralelo
- **NO-GO**: se a constante da fonte retorna zero ou negativo e o fallback não é activado correctamente, resolver antes de avançar

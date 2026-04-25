# ⚖️ ADR-0028: Representação simplificada dos tipos tipográficos em Value

**Status**: `REVOGADO`
**Revogado por**: ADR-0029
**Data**: 2026-03-29

## Contexto

DEBT-4 exige adicionar tipos tipográficos ao `Value` para desbloquear funções
nativas que actualmente retornam `Value::None`. Os tipos no original (`typst-library`)
são altamente complexos (polimorfismo de cor, comprimentos relativos compostos,
StyleChain). Replicar essa complexidade agora bloquearia o progresso sem benefício.

## Decisão

Tipos tipográficos usam representações simplificadas em L1 para o Passo 25.
A fidelidade ao original é adiada até que StyleChain (DEBT-1) e o sistema de
unidades completo sejam necessários.

| Tipo | Representação L1 | Original |
|------|-----------------|---------|
| Length | `enum { Pt(f64), Em(f64) }` | struct `{ abs: Abs, em: Em }` |
| Ratio | `newtype f64` (0.0–1.0 = 0%–100%) | `Ratio(Scalar)` |
| Angle | `newtype f64` (radianos internamente) | `Angle(Scalar)` em radianos |
| Color | `enum { Rgb{r,g,b:u8}, Rgba{r,g,b,a:u8} }` | enum com 8 espaços de cor |
| Auto | unit variant sem dados | `Smart<T>` genérico |

## Regras de implementação

**PartialEq**: `derive` exacto em produção — sem tolerância embutida.
A conversão `deg → rad → deg` pode introduzir ruído IEEE 754. Esse ruído é correcto
e observável — não escondê-lo com tolerância em `PartialEq`.
Tolerância apenas em testes, via `assert_approx_eq!` (apenas `#[cfg(test)]`).

**Somas mistas Pt+Em**: `Err` explícito e propagável — nunca panic, nunca `Value::None`,
nunca resolução com font-size hardcoded.

**`luma(l)`**: implementado como `Color::Rgb { r: l, g: l, b: l }` (escala de cinzentos).

## Consequências

- `rgb(r,g,b)` e `luma(l)` funcionam no eval
- Aritmética `Ratio * Int`, `Length::Pt + Length::Pt`, `Length::Em + Length::Em`
- `Length::Pt + Length::Em` → `Err` com mensagem identificando as unidades
- Espaços de cor avançados (Oklab, CMYK, HSL) retornam `Value::None` com comentário DEBT-4
- `Length::Mm`, `Length::Cm`, `Length::In`, `Length::Fr` adiados para quando StyleChain for migrado
- `Relative` (comprimento abs+ratio composto) adiado — DEBT-1

## Próxima revisão

ADR-0029: quando StyleChain (DEBT-1) for implementado no Passo 30, `Length` passa
a ter a representação composta do original.

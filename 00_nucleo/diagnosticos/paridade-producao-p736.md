# P736 — Paridade de produção: `color`/`gradient` como valores-tipo

## Proveniência da medição (regra 2026-07-05)

- **Commit base:** `9291b43be8df7e537f6e6b0aa49d5cf6976ebf50` ("P735: preenche hash do commit no relatório")
- **Commit da implementação:** `17f854beced01c3a6a2fecb91a1a791b04de2ee8` ("P736: color/gradient como valores-tipo (paridade vanilla)")
- **Estado na medição final:** working tree não commitado; `git diff HEAD --stat`: 10 ficheiros — `00_nucleo/prompts/rules/stdlib/{color,foundations,gradients}.md`, `01_core/src/rules/eval/{bindings,mod,tests}.rs`, `01_core/src/rules/stdlib/{color,foundations,gradients,mod}.rs` (+320/−90).
- **Hora da validação final:** 2026-07-13 ~20:58 (-03)
- **Binário vanilla de referência:** `lab/typst-original/target/release/typst`
- **Binário cristalino:** `./target/release/typst` (rebuild 16.7s após a implementação)

## Sonda (medida antes de decidir — ADR-0108)

| Sondagem | Vanilla | Cristalino (pré-P736) |
|---|---|---|
| `#type(color)` / `#type(gradient)` | `type` / `type` | `dictionary` / `dictionary` |
| `#(type(red) == color)` | `true` | `false` (tipos distintos) |
| `#(type(gradient.linear(red, blue)) == gradient)` | `true` | `false` |
| `#color.rgb(255, 0, 0)` | `rgb("#ff0000")` | erro "campo 'rgb' não existe" |
| `#color.linear-rgb(50%, 50%, 50%)` | OK (50%) | campo inexistente |
| `#type(color.<f>)`, f ∈ {rgb, linear-rgb, luma, cmyk, hsl, hsv, oklab, oklch, lighten, darken, mix, negate, saturate, desaturate, rotate, components, space} | `function` (17 fields) | só os 6 operadores P476/P477 existiam no dict |
| `#type(gradient.<f>)`, f ∈ {linear, radial, conic} | `function` | OK (dict P262/264/267) |
| `#repr(color)` / `#repr(gradient)` | `color` / `gradient` | dict dump |
| `#color("#ff0000")` | erro "type color does not have a constructor" | — |
| `#color.foo` | erro "type color does not contain field `foo`" | — |

### Descoberta colateral medida — `linear-rgb` com dupla divergência (pré-existente)

| Caso | Vanilla | Cristalino pré-P736 (global `linear_rgb`) |
|---|---|---|
| `color.linear-rgb(50%, 50%, 50%)` | ✓ (50%) | erro "espera Float/Int, recebeu relative length" |
| `color.linear-rgb(128, 128, 128)` | ✓ (50.2% = 128/255) | aceita, mas interpreta 128.0 linear (sem sentido) |
| `color.linear-rgb(0.5, 0.5, 0.5)` | **erro** "expected integer or ratio, found float" | aceita (falso-aceite) |

O caso `color.linear-rgb(50%, 50%, 50%)` está literalmente no ficheiro do passo — corrigido neste passo. `rgb(50%, 0%, 0%)` (vanilla aceita → `rgb("#800000")`; cristalino rejeita) **não** está no passo — registado como achado adiado.

## Decisão

- `color`/`gradient` passam de `Value::Dict` a `Value::Type(Type::Color/Type::Gradient)` no scope global (padrão P685, como `int`/`str`).
- Fields resolvem-se por field access em `Value::Type` (braço em `eval/bindings.rs`), delegando em `color_type_field(field) -> Option<Value>` (14 fields: 8 constructors globais + 6 operadores) e `gradient_type_field` (3 fields). Campo inexistente → mensagem verbatim do vanilla "type color does not contain field `<f>`".
- `make_color_module`/`make_gradient_module` removidos; testes P476/P477 atualizados para o novo mecanismo.
- `linear-rgb`: Int ∈ [0,255] ÷ 255, Ratio → valor direto, Float → erro verbatim do vanilla.

L0 atualizados: `color.md`, `gradients.md`, `foundations.md` (hashes via fix-hashes).

## Testes (fail-first confirmado: 6/8 falhavam; os 2 que passavam cobriam não-regressão dos operadores/constructors de gradient via dict)

8 testes `p736_*` em `eval/tests.rs`: tipo `type`, igualdade `type(x) == color`, constructors acessíveis (8 color + 3 gradient), operadores mantidos, mensagem verbatim de campo inexistente, tipo não chamável, repr.

## Validação

| Verificação | Resultado |
|---|---|
| `cargo test -p typst-core p736` | **8 passed**, 0 failed |
| `cargo test --workspace` | **4750 passed**, 0 failed (pré-P736: 4742; +8) |
| `crystalline-lint .` | 0 violations, 0 drift |
| `cargo build --release` | OK (16.7s) |
| E2E (ficheiro do passo) | `type type true true rgb("#ff0000") color.linear-rgb(50%, 50%, 50%) ... color` — idêntico ao vanilla, exceto repr de `Gradient` (ver nota) |
| cetz (150 dpi, mesmo ficheiro de P734) | diff **0.1477%** (vanilla 1478 px, cristalino 1535 px) — **B−A = +57 px, idêntico a P734** (1451/1508); os +27 px em ambos os lados vêm de variação externa (não do código), o diff estrutural é constante → sem regressão |

Nota: o repr de `Value::Gradient` (`gradient(...)` vs expansão vanilla de stops) é limitação pré-existente documentada em `gradients.md` ("repr básico") — fora do passo.

## Achados

- Item "`color` e `gradient` expostos como `dictionary`" (aberto desde P731) **fechado**.
- Novo achado adiado: `rgb(50%, 0%, 0%)` rejeitado (vanilla aceita percentagens por componente) — pré-existente, medido nesta sonda.
- Scope-outs registados no L0 `color.md`: `color.rotate`/`color.components`/`color.space` (ausentes, também como métodos de instância); métodos de instância de cor (`red.lighten(20%)`) ausentes desde P476.

# Paridade Produção — P703 — `rgb()` aceita hex/string de 1 argumento

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-703.md`
**Hash do commit (implementação):** `6b5bb3f5c`.
**HEAD base:** `d65d7eae0` (fim de P702, detached HEAD).
**Estado:** ÂMBITO DE P703 FECHADO — `rgb(hex)` implementado, testado, paridade vanilla confirmada (incluindo casos de borda: 3/4/6/8 dígitos, com/sem `#`, erros de conteúdo e comprimento). `cetz` progride ainda mais fundo (mesmo tempo de compilação ~7s), mas bloqueia num gap novo e não relacionado (`range()` não aceita `step:`).

---

## 1. Proveniência das medições

- Working tree no momento da medição: ficheiros de código+L0 de P703 (ver
  §4) + este relatório. `cargo test --workspace` corrido antes (herdado de
  P702: 3747 passed) e depois (3756 passed — +9 testes novos) da
  implementação.

---

## 2. Sonda — confirmado contra o vanilla, com `file:line`

- **Algoritmo exacto**: `visualize/color.rs:2072-2107` do vanilla
  (`impl FromStr for Rgb`) — remove `#` opcional, valida hexadecimal,
  valida comprimento ∈{3,4,6,8}, expande dígitos curtos (`v + v*16`),
  default de alpha `255` quando ausente.
- **Formas confirmadas com documento `.typ` real** (`lab/typst-original`):
  - `rgb("#FF0000")`, `rgb("FF0000")` (sem `#`) → mesma cor.
  - `rgb("#FF0000FF")` (8 dígitos, alpha explícito opaco) → mesma cor.
  - `rgb("F00")` (3 dígitos, curto) → expande para `#ff0000`.
  - `rgb("FF00")` (4 dígitos) → **o 4º dígito é alpha**, não mais um
    componente de cor: `r=255,g=255,b=0,a=0` (confirmado: `rgb("#ffff0000")`).
  - `rgb("red")` (nome de cor) → **erro no próprio vanilla**
    (`color string contains non-hexadecimal letters`) — não suportado,
    não é scope-out do cristalino.
  - Comprimento inválido (5 dígitos, `"FFFFF"`) → `error: color string has
    wrong length` (medido directamente, não só inferido do código-fonte).
- **Estado cristalino antes do fix**: `error: rgb() requer 3 ou 4 Int,
  recebeu 1 args` para todas as formas de string (reprodução exacta do
  bloqueio isolado por P702).

---

## 3. Implementação

### 3.1 `native_rgb` — novo braço para `[Value::Str(s)]`

`01_core/src/engine/stdlib/foundations.rs`: novo braço no `match
args.items.as_slice()`, antes dos braços de 3/4 `Int`, delega a
`parse_hex_color`.

### 3.2 `parse_hex_color` — algoritmo verbatim do vanilla

Nova função privada, replica exactamente `visualize/color.rs:2072-2107`:
strip `#` opcional → valida hex → valida comprimento (3/4/6/8, senão erro
"color string has wrong length") → expande curtos → `Color::rgba(...)`.
Mensagens de erro copiadas verbatim (`"color string contains
non-hexadecimal letters"`, `"color string has wrong length"`).

### 3.3 Scope-out explícito (decisão, não omissão)

- Componentes `Ratio` (`rgb(25%, 13%, 65%)`) — não implementado.
- `rgb(color)` — conversão de uma `Color` existente — não implementado.
- Nenhum consumidor real medido (`cetz`) precisa destas formas — só usa
  `.map(rgb)` sobre strings hex em `palette.typ`.

### 3.4 Testes (9 novos, `foundations.rs::tests_p703_rgb_hex`)

6/8/3/4 dígitos, com/sem `#`, hex inválido (mensagem verbatim), nome de cor
(erro igual ao vanilla), comprimento errado (mensagem verbatim), forma
numérica de 3 `Int` sem regressão.

---

## 4. Ficheiros tocados

- **L0**: `00_nucleo/prompts/engine/stdlib/foundations.md` — secção
  `native_rgb` estendida com a forma hex, algoritmo, scope-out, testes
  canónicos.
- **Código**: `01_core/src/engine/stdlib/foundations.rs` (novo braço +
  `parse_hex_color` + 9 testes).

---

## 5. Validação — âmbito de P703 confirmado

Reexecutados os 5 documentos `.typ` da sonda + 3 casos de borda extra
(release build): todos os valores RGBA computados **idênticos ao vanilla**
(`r=255,g=0,b=0,a=255` para os hex de 6/8 dígitos e o de 3 dígitos;
`r=255,g=255,b=0,a=0` para o de 4 dígitos; erro igual para nome de cor).

Nota: a **representação textual** do valor (`repr`/`Debug`) já divergia do
vanilla antes de P703 — cristalino mostra `Srgb { r: 1.0, g: 0.0, ... }`
(Rust `Debug`), vanilla mostra `rgb("#ff0000")`. Isto é uma divergência de
**mecanismo de representação**, pré-existente e fora do âmbito de P703 (que
é sobre aceitar a forma hex como **entrada**, não sobre o formato de
saída) — ADR-0107, mecânica diverge de propósito.

- `cargo test --workspace` → **3756 passed**, 0 failed (3747 de P702 + 9
  novos de P703); `typst-infra` inalterado (626/5).
- `crystalline-lint .` → 0 violations (hash de `foundations.rs`
  realinhado com `--fix-hashes`).

---

## 6. Repetição da reprodução de P700-702 — mais progresso, novo bloqueio

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

Tempo de compilação: **~7.1s** — na mesma ordem de grandeza de P702 (~7.3s),
sinal de que o eval continua a avançar tão fundo quanto antes (não houve
regressão de profundidade, mas também não há sinal claro de que este passo
tenha desbloqueado uma fase inteira nova — o bloqueio seguinte está muito
perto do de `rgb`, na mesma função `palette.typ`).

**Novo bloqueio**: `error: range() argumento nomeado desconhecido: 'step'`.
Isolado à mesma `palette.typ` que motivou P703:
```
range(90, 40, step: -12).map(v => luma(v * 1%))
```
`native_range` (`01_core/src/engine/stdlib/foundations.rs`) só aceita
`range(n)` / `range(start, end)`, sem `step:`. Confirmado standalone contra
o vanilla:
```
#range(90, 40, step: -12)
#range(0, 10, step: 2)
```
→ vanilla: `(90, 78, 66, 54, 42) (0, 2, 4, 6, 8)`, exit 0. Cristalino:
`error: range() argumento nomeado desconhecido: 'step'`.

Mesma disciplina: registar, não forçar correcção improvisada fora do âmbito
de P703.

### Próximo passo sugerido (P704, não iniciado)

1. Confirmar no vanilla a semântica completa de `range(start, end, step:)`
   — sinal do passo, passo zero, direcção de paragem (`< end` vs `> end`
   conforme sinal do step), erro se `step == 0`.
2. Estender `native_range` (mesmo ficheiro, `foundations.rs`) e o L0
   correspondente.
3. Reexecutar a reprodução deste §6 como critério de fecho; se `cetz`
   avançar mais, repetir de novo (mesmo padrão desta cadeia).

---

## 7. Estado da cadeia P678–703

Progresso real e cumulativo: plugin WASM real (P699/P699b/P700),
`cbor.encode`/`cbor(bytes)` (P701), `.with()` (P702), e agora `rgb(hex)`
(P703) — todos fechados e testados. `cetz` ainda não renderiza; o bloqueio
mais recente está na mesma zona de código (`palette.typ`) do bloqueio
anterior, o que sugere que os próximos 1-2 passos podem esgotar rapidamente
os gaps desse ficheiro específico. Pausada aqui, com P704 sugerido e a
causa exacta já isolada (`range(..., step:)`).

## 8. Critério de fecho do passo

- [x] Sonda completa, formas de `rgb()` confirmadas contra o vanilla
      (`file:line` + medições directas de casos de borda).
- [x] Implementado e testado.
- [x] Forma de inteiros (3/4 `Int`) sem regressão.
- [x] `cetz` re-testado — **progresso mantido, próximo bloqueio
      identificado e isolado** (`range(step:)`), não sucesso completo.
- [x] Sem regressão em `cargo test --workspace` (3756 passed).
- [x] `crystalline-lint .` limpo.
- [x] Relatório com resultado exacto (este ficheiro).

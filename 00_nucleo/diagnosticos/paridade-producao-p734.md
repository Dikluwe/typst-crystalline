# Relatório P734 — `polygon` restringido a `Length` (rejeita Int/Float como o vanilla)

**Data:** 2026-07-13
**Passo:** `00_nucleo/materialization/typst-passo-734.md`
**ADRs em vigor:** ADR-0107 (paridade é com a linguagem), ADR-0108 (medir antes de decidir).
**Commit:** A PREENCHER
**Proveniência das medições (regra de proveniência):** commit base `b3421dc22e55206dc0fa941fc455a61b4374dd86` ("P733: preenche hash do commit no relatório"), branch `Tekt`, working tree com as alterações deste passo (`git diff HEAD --stat`: `00_nucleo/prompts/rules/stdlib/shapes.md`, `01_core/src/rules/stdlib/shapes.rs`, `01_core/src/rules/stdlib/mod.rs`, `01_core/src/rules/eval/tests.rs`). Medições vanilla: `lab/typst-original/target/release/typst`; medições cristalino: `./target/release/typst` (release build de 2026-07-13T23:14Z); compilação cetz medida 23:15:12–23:15:57Z.

---

## Sonda — medições antes de decidir (ADR-0108)

P732 encontrou (e registou como achado) que `polygon` aceitava Int/Float — o inverso do vanilla. Este passo confirma o tipo exacto aceite pelo vanilla e se o cetz depende da aceitação de números nus.

### Tipo aceite pelo vanilla (medido)

| Caso | Vanilla | Observação |
|---|---|---|
| `polygon((0pt, 0pt), ...)` | exit 0 | `Length` aceite |
| `polygon((0, 0), ...)` | erro "expected relative length, found integer" | Int rejeitado |
| `polygon((0.0, 0.0), ...)` | erro "expected relative length, found float" | Float rejeitado |
| `polygon((50%, 0pt), ...)` | exit 0, **2923 px não-brancos** | ratio aceite, resolvido no layout contra o contentor |
| `curve(("move", (0pt, 0pt)), ...)` | erro "expected content, found array" | o vanilla **rejeita tuples** — usa `curve.move`/etc. |

**Tipo confirmado: `Rel<Length>`** (length + ratio). Ratio fica scope-out no cristalino (constructor em tempo de eval, sem dimensão de referência — mesmo precedente de P513).

### Consumidores no cetz (medido, não estimado)

- `cetz` define a sua **própria** função `polygon(origin, sides, ...)` (`draw/shapes.typ:685`) — não chama o builtin.
- O único uso do builtin de path é `std.curve(..vertices)` (`canvas.typ:173`) com vértices de **conteúdo** (`curve.move`/`curve.line`/`curve.cubic`), construídos por `transform-point` que aplica `* length` (`canvas.typ:141-156`) — chegam como `Length`.
- **Nenhum consumidor de números nus** no builtin `polygon`/`curve` — a restrição é segura para a cadeia fechada.

### Decisão de desenho

A restrição aplica-se **só a `polygon`**. `curve` mantém a aceitação de números na interface por tuples: é divergência intencional documentada no L0 (o vanilla rejeita tuples — "expected content, found array", medido), e o caminho de paridade real do vanilla (`curve.move` etc., via `extract_curve_point`) não é tocado. O passo autoriza explicitamente esta separação.

### Critério de fecho da sonda

- [x] Tipo aceite pelo vanilla confirmado: `Rel<Length>` (ratio scope-out com razão medida).
- [x] Confirmado que o cetz **não** depende da aceitação de números nus (file:line acima).

## L0 (Prompt)

`00_nucleo/prompts/rules/stdlib/shapes.md` — secção `native_polygon`: assinatura passa a `Array[Length, Length]`; rejeição de Int/Float com mensagem verbatim do vanilla; scope-out de ratio registado com a medição (2923 px); nota explícita de que `curve` mantém números (divergência intencional) com a evidência do cetz; testes canónicos actualizados; marco P734 no cabeçalho. `crystalline-lint .` → **0 violations** (hashes sincronizados).

## Implementação

`01_core/src/rules/stdlib/shapes.rs`:

1. `extract_vertex(val, index) -> SourceResult<(f64, f64)>` + `vertex_component(val) -> SourceResult<f64>` — helpers próprios de `polygon`: só `Value::Length` (`abs.to_pt()`); `Int` → erro verbatim "expected relative length, found integer"; `Float` e resto → `format!("expected relative length, found {}", type_name())`; `Ratio` → erro explícito de scope-out (o vanilla aceita — a mensagem não finge paridade).
2. `native_polygon` passa a usar `extract_vertex` (era `extract_coordinate`).
3. `extract_coordinate`/`coord_component` **inalterados** — continuam a servir a interface por tuples de `curve`.
4. Dois testes antigos (`polygon_com_um_ponto...`, `polygon_triangulo...`) actualizados de Float para Length — manutenção de interface, não de lógica.

## Validação

- Fail-first confirmado: `cargo test -p typst-core p734` antes da implementação → **3 failed / 2 passed**.
- Depois: **5 passed, 0 failed** — `p734_polygon_rejeita_int_com_mensagem_vanilla`, `p734_polygon_rejeita_float_com_mensagem_vanilla`, `p734_curve_tuplos_com_numeros_sem_regressao` (unitários) + `p734_polygon_int_rejeitado_e2e`, `p734_polygon_length_sem_regressao_e2e` (`eval/tests.rs`).
- `cargo test --workspace` — **4737 passed, 0 failed** (4042 + 631 + 33 + 2 + 27 + 2; 8 ignored pré-existentes). Pré-P734: 4732; +5 = os novos testes do passo.
- `crystalline-lint .` — **0 violations**.

### cetz (cadeia P678-731 fechada) — sem regressão

Reprodução exacta do passo (`line((0,0),(2,1))` + `circle((0,0))`): exit 0; **diff 0.1419%** — número idêntico ao de P731 (1451 vs 1508 px não-brancos, 9264 bytes de diff). A cadeia permanece fechada.

## Critério de fecho do passo

- [x] Sonda completa, tipo aceite confirmado (`Rel<Length>`; ratio scope-out com razão medida).
- [x] Implementado e testado: Int/Float produzem erro igual ao vanilla; Length continua a funcionar.
- [x] Sem regressão em `cargo test --workspace` (4737 passed, 0 failed).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` sem regressão, confirmado com diff de pixels (0.1419%, inalterado).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p734.md`, com hash do commit.
- [x] Item actualizado em `achados-adiados-cetz.md` (Int/Float fechado; ratio permanece aberto como scope-out).

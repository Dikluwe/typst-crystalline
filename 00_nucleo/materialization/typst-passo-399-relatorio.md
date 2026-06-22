# Passo 399 — relatório: `Value::Decimal`

**Tipo:** modelagem de tipo primitivo L1 (S puro; zero consumer; zero I/O).  
**Data:** 2026-06-22. **HEAD:** pós-`d4315a322`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Modelou-se o tipo `Decimal` como novo variant de `Value`, o primeiro da série de tipos S puros (Decimal → Duration → Version).

- L0:
  - `00_nucleo/prompts/entities/decimal.md` — tipo `Decimal` e variant `Value::Decimal`.
- ADR:
  - `00_nucleo/adr/typst-adr-0112-rust-decimal-l1.md` — autorização de `rust_decimal` em L1.
- Dependências:
  - `rust_decimal = "1"` adicionado ao `[workspace.dependencies]` do `Cargo.toml` raiz.
  - `rust_decimal = { workspace = true }` adicionado a `01_core/Cargo.toml`.
  - `rust_decimal` registado em `[l1_allowed_external]` de `crystalline.toml` (ADR-0112).
- `01_core/src/entities/decimal.rs`:
  - Tipo L1 puro `Decimal(pub InnerDecimal)` com `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`.
  - Construtores: `new`, `from_str`, `from_f64`, `from_i64`; `Default`.
  - Testes unitários: 8 testes.
- `01_core/src/entities/value.rs`:
  - Novo variant `Value::Decimal(Decimal)`.
  - `type_name()` retorna `"decimal"`.
  - `cast_decimal()` converte `Decimal`, `Int`, `Float` (rejeita NaN/Inf) e `Str` (parse).
  - `From<Decimal> for Value`.
  - Testes unitários: 8 testes.
- `01_core/src/entities/mod.rs`:
  - `pub mod decimal;` registado com nota P399.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:
  - Tabela B.1: `Decimal` reclassificado de `ausente` para `implementado`; variants 19 → **20**, total 32 → **33**.
  - Tabela B resumo: implementado 75 → **76**, total arquitectural 107 → **108**.
  - Contagem user-facing total mantida em **141** (`Decimal` é tipo arquitectural; `decimal()` stdlib continua `ausente` — P400).
  - Nota de rodapé ⁸³ para P399.

`cargo test --workspace` verde; `crystalline-lint .` — `✓ No violations found`; hashes propagados via `crystalline-lint --fix-hashes`.

## Protocolo de Nucleação cumprido

1. L0 (`decimal.md`) escrito e hash propagado.
2. ADR-0112 registada antes de adicionar a dependência externa a L1.
3. TDD: testes unitários escritos antes/paralelamente à implementação.
4. Novo `Value` variant justificado por ADR-0017 (portão aberto) e necessidade de paridade linguagem.
5. Zero I/O; zero consumer; zero func stdlib nova — S puro.

## Decisão de engenharia

`Decimal` usa `rust_decimal::Decimal` (precisão fixa 28 dígitos, `Copy`, puro-Rust) em vez de `bigdecimal` (precisão arbitrária com heap). A paridade linguagem (ADR-0107) não exige mecânica interna idêntica ao vanilla; `rust_decimal` é suficiente e evita `Arc`/`Vec` em L1.

`cast_decimal` é intencionalmente generoso para os tipos-fonte mais comuns (`Int`, `Float`, `Str`), alinhando com futuro constructor `decimal(...)` e conversões implícitas de script. `Float` para `Decimal` rejeita NaN/Inf — valores sem representação decimal.

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `Decimal::new(12345, 2)` | `"123.45"` | ✓ |
| `Decimal::from_str("1.2345678901234567890123456789")` | parse ok | ✓ |
| `Decimal::from_str("abc")` | `None` | ✓ |
| `Decimal::from_f64(f64::NAN)` | `None` | ✓ |
| `Value::Decimal(Decimal::new(12345,2)).type_name()` | `"decimal"` | ✓ |
| `Value::Int(42).cast_decimal()` | `Some(Decimal(42))` | ✓ |
| `Value::Float(1.5).cast_decimal()` | `Some(Decimal(1.5))` | ✓ |
| `Value::Str("3.14").cast_decimal()` | `Some(Decimal(3.14))` | ✓ |
| `Value::Str("abc").cast_decimal()` | `None` | ✓ |
| `Value::Decimal(1.00) == Value::Decimal(1.0)` | `true` | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Value::Decimal(Decimal)` compila e integra-se no enum | ✓ |
| 2 | `Decimal` é `Copy`, puro-Rust, zero alloc | ✓ |
| 3 | Cast: `Int → Decimal`, `Float → Decimal` (com perda), `Str → Decimal` | ✓ |
| 4 | Repr: string literal decimal via `to_string()` | ✓ |
| 5 | Zero consumer complexo; zero I/O; zero func stdlib nova | ✓ |
| 6 | Testes verdes; lint zero; hashes propagados | ✓ 16 unit novos |
| 7 | Inventário 148 actualizado | ✓ |
| 8 | L0 salvo e hashado | ✓ `decimal.md` |
| 9 | ADR-0112 registada para dependência externa L1 | ✓ |

## Artefactos

- Código:
  - `01_core/src/entities/decimal.rs`
  - `01_core/src/entities/value.rs`
  - `01_core/src/entities/mod.rs`
  - `Cargo.toml`
  - `01_core/Cargo.toml`
  - `crystalline.toml`
- L0:
  - `00_nucleo/prompts/entities/decimal.md`
- ADR:
  - `00_nucleo/adr/typst-adr-0112-rust-decimal-l1.md`
- Inventário 148: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- Plano: `00_nucleo/materialization/typst-passo-399.md`.
- Este relatório.

## Nota sobre o Tekt

P399 é o **baseline de tipo S puro**: modelagem L1 sem consumer, sem I/O, sem stdlib. O ritmo deve ser comparado com P400 (`Duration`) e P401 (`Version`) para validar que o portão ADR-0017 está a funcionar como previsto.

# ADR-0112 — Autorização de `rust_decimal` em L1 para o tipo `Decimal`

**Estado:** `IMPLEMENTADO` (Passo 399 — `Value::Decimal` materializado; `cargo test --workspace` + `crystalline-lint .` verdes).
**Decisão do dono (registada):** usar `rust_decimal::Decimal` (precisão fixa 28 dígitos, puro-Rust, `Copy`, zero alloc) como substrato do tipo L1 `Decimal`; alternativa `bigdecimal` (heap, precisão arbitrária) rejeitada por não ser `Copy` e por exceder a paridade linguagem do vanilla.
**ADRs relacionadas:** ADR-0017 (portão aberto para variants em `Value`), ADR-0029 (pureza L1), ADR-0107 (paridade linguagem vs mecânica), ADR-0054 (operações aritméticas ricas scope-out).

---

## Contexto

O Typst vanilla expõe o tipo `decimal` (precisão arbitrária na documentação, mas na prática `rust_decimal::Decimal` de 28 dígitos). O cristalino não tinha o variant `Value::Decimal` porque o enum `Value` era fechado (ADR-0017). Após P395 (`Value::Tiling`) e P398 (`Value::Bytes`), o portão está aberto para tipos S puros.

## Decisão

Autorizar a crate `rust_decimal` em `[l1_allowed_external].rust` de `crystalline.toml`, exclusivamente como substrato do tipo L1 `Decimal`.

| Propriedade | `rust_decimal` | `bigdecimal` |
|-------------|----------------|--------------|
| Precisão | 28 dígitos fixos | arbitrária (heap) |
| `Copy` | ✓ | ✗ |
| Pureza | puro-Rust, zero alloc | alloc dinâmico |
| Paridade vanilla | suficiente (ADR-0107) | excesso de mecânica |

**Critério de escolha:** a paridade linguagem exige um tipo decimal fixo, não necessariamente precisão arbitrária. `rust_decimal` é `Copy` (128-bit), o que evita `Arc` em `Value::Decimal` e mantém L1 puro.

**Regra de fronteira (V14):** o tipo `rust_decimal::Decimal` **não aparece em contratos públicos L1**. O tipo de domínio é `entities::decimal::Decimal`, wrapper opaco; `Value::Decimal(Decimal)` é a única superfície pública.

## Consequências

- **Positivas.** Tipo `Decimal` em L1 sem alocação; `Value::Decimal` é `Copy`; abre caminho para `native_decimal` e operações aritméticas em passos futuros.
- **Custos.** +1 crate em L1; precisão fixa de 28 dígitos (limite documentado no L0).
- **Riscos.** Se no futuro a paridade exigir precisão arbitrária real, será necessário migrar para `bigdecimal` — mas isso violaria a decisão ADR-0107 de não perseguir mecânica interna do vanilla quando a linguagem já é equivalente.

## Alternativas consideradas

- **`bigdecimal`**: rejeitada porque usa `Vec` interno, não é `Copy`, e introduz alloc em L1 sem ganho de paridade linguagem.
- **Decimal próprio em L1**: rejeitado — reimplementar aritmética decimal correta é M+ e não agrega paridade face a uma crate puro-Rust madura.

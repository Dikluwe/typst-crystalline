# Prompt L0 — `entities/content_hash`
Hash do Código: e55b46d6

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/content_hash.rs`
**Criado em**: 2026-04-30 (P162 sub-passo .B — resolve pendência herdada de P161)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`hash_content` é a função pura que produz um identificador `u128` determinístico para qualquer `Content`. P162 materializa-a para ser consumida por `extract_payload` (preenche `ElementPayload::Heading.body_hash`) e por `walk` (preenche `Tag::End(loc, content_hash)`).

Vanilla deriva `Hash` em `Content` via proc-macro `#[elem]` + sub-types. Cristalino não usa proc-macro; `Content` é enum fechado mas contém `f64` (em `Pt`/`Length`/`Ratio`) que não implementa `Hash` automaticamente. Função manual é a alternativa.

---

## Restrições Estruturais

- Camada **L1**: função pura, sem I/O, sem estado global.
- Determinismo dentro da mesma versão do compilador: dois `Content` que sejam `==` produzem o mesmo `u128`.
- Sem dependência em ordem de iteração de `HashMap` (`Content` não usa `HashMap` directamente em variantes — fan-out apenas para tipos que também não usam).
- Floats são serializados via `Debug` (que mostra a representação decimal estável); valores f64 com bit-pattern diferente mas decimal igual produzem hashes iguais — comportamento aceite.
- Sem dependência externa nova (não autorizar `siphasher` em L1; usar `std::collections::hash_map::DefaultHasher` que existe em L1 sem precisar de allowed-list).

## Implementação escolhida (P162)

Forma minimalista: `format!("{:?}", content)` produz uma string estável recursiva (Debug derive em Content é estrutural). Hashar a string duas vezes com sementes distintas e concatenar os u64 resultantes para obter u128.

```rust
fn hash_content(content: &Content) -> u128 {
    let serialized = format!("{:?}", content);
    let lo = hash_with_seed(&serialized, 0);
    let hi = hash_with_seed(&serialized, 1);
    ((hi as u128) << 64) | (lo as u128)
}
```

**Fragilidade declarada**: a estabilidade do hash depende da estabilidade do output de `Debug` derive em `Content`. Mudanças cosméticas no Debug formatter de qualquer field-type (e.g. mudança de formatação f64 em Rust stdlib) podem alterar o hash. Aceitável para M1 (tags são descartadas). Quando M2/M3 começarem a consumir tags entre iterações do fixpoint, esta função pode precisar de ser substituída por hash recursivo manual sobre cada variante de Content. Decisão diferida — sem ADR nova.

Refino futuro candidato: hash recursivo manual com `std::mem::discriminant` por variante + hash de cada field directamente (com `f64::to_bits()` para floats). Aceitável quando consumidor real exigir.

---

## Interface pública

```rust
use crate::entities::content::Content;

pub fn hash_content(content: &Content) -> u128;
```

Sem trait, sem método em `Content` (manter Content sem imports adicionais). Função top-level no módulo.

---

## Semântica

- `hash_content(c)` é determinístico dentro da mesma execução e dentro da mesma versão de Rust+std.
- Dois `Content` que são `==` produzem o mesmo `u128` (porque Debug derive é determinístico para estruturas iguais).
- Dois `Content` distintos produzem `u128` distintos com altíssima probabilidade (colisões teóricas mas extremamente improváveis em qualquer prática realista — DefaultHasher é SipHash 1-3 com 64 bits cada).
- Função é pura — sem leitura de ambiente, sem alocação além do `format!`.

---

## Invariantes

- `hash_content(&c) == hash_content(&c.clone())` para qualquer `c: Content`.
- Determinismo entre runs (verificado por test "100 chamadas, todos iguais").
- Não há "hash zero" reservado; `0u128` é resultado válido se a entrada produzir esse Debug.

---

## Tests obrigatórios (sub-passo .B P162)

- **Igualdade**: `Content::text("hello") == Content::text("hello")` produz mesmo hash.
- **Distinguibilidade**: 5 Contents construídos manualmente produzem 5 hashes distintos.
- **Determinismo**: 100 chamadas sobre o mesmo Content produzem o mesmo u128.

---

## Consumers actuais

Nenhum no momento da criação. Imediatamente consumido em P162 sub-passos .D e .E.

## Consumers planeados

- `rules/introspect/extract_payload.rs` (P162 .D) — popula `ElementPayload::Heading.body_hash` via `hash_content(body)`.
- `rules/introspect.rs` walk (P162 .E) — popula `Tag::End(loc, content_hash)` via `hash_content(content)`.
- `Introspector` em M3 — pode usar para detectar mudanças cross-iteration.

---

## Sobre paridade

Vanilla obtém hash via derive `#[derive(Hash)]` automático em `Content` (proc-macro `#[elem]` injecta o derive). Hash output é u64. Cristalino não tem proc-macro nem hash automático; produz u128 (paridade com `Tag::End(Location, u128)` vanilla via design choice).

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` (2026-04-30) — vanilla `Tag::End(Location, u128)` linha 13 confirma o tipo de output.

---

## Resultado Esperado

- `01_core/src/entities/content_hash.rs` — função + 2 helpers + 3 tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P162 sub-passo .B: resolve pendência P161 — função de hash determinística sobre Content | `content_hash.rs`, `content_hash.md` |

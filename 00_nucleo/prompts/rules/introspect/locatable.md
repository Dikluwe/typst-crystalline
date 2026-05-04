# Prompt L0 — `rules/introspect/locatable`
Hash do Código: 4b2a29e5

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/introspect/locatable.rs`
**Criado em**: 2026-04-30 (P164 — passo único de M2 Introspection)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`is_locatable` é a função pura `&Content → bool` que classifica se um nó de `Content` é uma variante locatable (queryable pelo motor de introspecção). Em M1, "locatable" significa um dos 3 kinds que produzem `ElementPayload` em `extract_payload`: `Heading`, `Figure`, `Cite`.

Função extraída em M2 (P164) como utilitária para consumidores futuros — tipicamente `Introspector::from_tags` em M3 quando precisar de classificar tags por kind sem reusar a lógica de `extract_payload`.

Vanilla resolve via marker traits (`Locatable`, `Unqueriable`, `Tagged`); cristalino prefere função pura com match exaustivo top-level. Lista única auditável vs atributos dispersos por dezenas de elementos. Coerente com a topologia "Content é enum fechado" (ADR-0026) e "extract_payload é match exaustivo" (P162).

---

## Restrições Estruturais

- Camada **L1**: função pura, sem efeitos secundários, sem I/O.
- `pub fn is_locatable(content: &Content) -> bool`.
- **Match exaustivo** sobre `Content` — sem `_ => false` fall-through.
  Razão: compilador força revisão quando variant novo é adicionado a
  `Content`. Se usar `_ => false`, novos variants são silenciosamente
  classificados como não-locatable, perdendo a invariante.
- **Invariante**: para todo `c: Content`,
  `is_locatable(c) == extract_payload(c).is_some()`.

## Cobertura (P164 baseline; expandido em P169/P171/P178/P181D)

`Content` tem **56 variants**. Distribuição actual:

- **Locatable (10)** → `true`: `Heading`, `Figure`, `Cite`, `Metadata`
  (P169 M9), `State` + `StateUpdate` (P171 M9), `Outline` (P178),
  `Bibliography` (P181D — lacuna #6 fechada),
  **`SetHeadingNumbering`** (P182C — emite `StateUpdate` com chave
  `numbering_active:heading`; suporta lacuna #4),
  **`Equation`** (P186D — eixo 2 do bloqueio P183C C2 desbloqueado;
  emite `ElementPayload::Equation { block, counter_update }` via
  `extract_payload` P186C).
- **Não-locatable (46)** → `false`: todos os outros.

---

## Interface pública

```rust
use crate::entities::content::Content;

pub fn is_locatable(content: &Content) -> bool;
```

---

## Semântica

- `is_locatable(c)` é determinístico — mesma entrada produz mesmo output.
- Não tem state — função pura sobre o discriminante do Content.
- Equivalente a `extract_payload(c).is_some()` mas mais barato (não constrói `ElementPayload` nem chama `hash_content`).

---

## Invariantes

- Função pura sem side-effects.
- Match exaustivo — qualquer variant novo de `Content` força edição
  explícita aqui (compile error). Esta é a propriedade arquitectural.
- `is_locatable(c) == extract_payload(c).is_some()` para todo c. Test
  exaustivo verifica.
- Adicionar variant novo a `Content` exige decisão coordenada: editar
  ambos `extract_payload` (escolher `Some(...)` ou continuar em
  `_ => None`) e `is_locatable` (escolher `true` ou `false`). Os dois
  ficheiros são propositadamente paralelos.

---

## Tests obrigatórios (sub-passo .B P164)

- **Cobertura locatable**: `is_locatable(&Content::Heading{..}) == true`,
  `Content::Figure{..} == true`, `Content::Cite{..} == true`.
- **Cobertura não-locatable**: pelo menos 3 variants não-locatable
  retornam `false` (e.g. `Content::Empty`, `Content::Text(..)`,
  `Content::Sequence(..)`).
- **Invariante exaustivo**: para cada variant de `Content` (ou pelo
  menos um representante de cada bucket — locatable e não-locatable),
  construir instância mínima e verificar
  `is_locatable(&c) == extract_payload(&c).is_some()`.

---

## Consumers actuais

Nenhum no momento da criação. Walk em `rules/introspect.rs` **não** é
modificado em M2 — walk continua a chamar `extract_payload` directamente.
`is_locatable` está disponível como utilitária para consumers futuros.

## Consumers planeados

- `Introspector::from_tags` em M3 — consulta `is_locatable` quando
  classificar tags por kind sem reconstruir o payload.
- M9 features novas que precisem de classificar Content sem extrair
  payload (e.g. um `query` que filtra por "qualquer locatable").

---

## Sobre paridade

Vanilla `Locatable` / `Unqueriable` / `Tagged` são marker traits sobre
`#[elem]` structs. Cada elemento "anuncia" o seu estado via implementação
de trait. Cristalino: lista única em função top-level — quando alguém
quer saber "este nó é locatable?" pergunta-se à função, não ao tipo.

Vantagens cristalinas:
- Lista única e auditável (uma função, um ficheiro).
- Compilador força exaustividade quando variant novo é adicionado.
- Sem dispersion de "atributos arquitecturais" por 56 variants.

Desvantagens reconhecidas:
- Acrescentar variant locatable nova requer 2 edições (extract_payload
  + is_locatable). Vanilla acrescenta 1 marker trait impl no `#[elem]`.

Trade-off aceite per ADR-0026 (Content como enum fechado) e topologia
"sem proc-macros vtable" do cristalino.

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md`
(2026-04-30) §3 para classificação de marker traits vanilla como
vtable-driven (scope-out cristalino).

---

## Resultado Esperado

- `01_core/src/rules/introspect/locatable.rs` — função + tests.
- `01_core/src/rules/introspect.rs` — adicionar `pub mod locatable;`
  (parallel to `pub mod extract_payload;` existente).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P164: extracção de classificação locatable como função pura — primeiro passo de M2 | `locatable.rs`, `locatable.md`, `rules/introspect.rs` (declaração `pub mod`) |
| 2026-05-01 | P181D: `Content::Bibliography` move de não-locatable para locatable; suporte ao plano P181 (decisão P181A cláusula 4 = Opção β; lacuna #6) | `locatable.rs`, `locatable.md` |
| 2026-05-02 | P182C: `Content::SetHeadingNumbering` move de não-locatable para locatable; emite `StateUpdate { key: "numbering_active:heading", update: Set(Bool(active)) }` em `extract_payload`. Suporte ao plano P182 (lacuna #4). | `locatable.rs`, `locatable.md` |
| 2026-05-03 | P186D: `Content::Equation` move de não-locatable para locatable. Combinado com arm em `extract_payload` (P186C, ordem invertida pragmaticamente para evitar janela de invariante quebrada — embora a inversão tenha apenas invertido o sentido da quebra, vide P186C `.A.6` empírico). Repõe invariante `is_locatable ↔ extract_payload.is_some()` para Equation; sincronização Locator Layouter ↔ walk reposta. Cobertura `build_minimal_for_each_variant` em test de invariante estendida (lacuna pré-existente fechada). Suporte ao plano P186 (eixo 2 do bloqueio P183C C2). | `locatable.rs`, `locatable.md` |

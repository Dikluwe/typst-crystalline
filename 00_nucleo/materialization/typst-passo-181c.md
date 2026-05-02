# Passo P181C — `ElementKind::Bibliography` + `ElementPayload::Bibliography`

Segundo passo de materialização P181 (após P181B). Magnitude **S**.

Adiciona variants Bibliography aos enums `ElementKind` e
`ElementPayload`. Replica padrão estabelecido por P169 (Metadata),
P171 (State, StateUpdate), P178 (Outline). Sem cascade em arms
exhaustive de `Content` — `Content::Bibliography` já existia.

**Pré-condição**: P181B concluído. `BibStore` em `entities/`,
field público em `TagIntrospector`. Ambos vazios em produção.

**Restrições**:
- **Não** modificar `Content` enum.
- **Não** modificar `is_locatable` (P181D).
- **Não** modificar `extract_payload` (P181D).
- **Não** modificar `from_tags` (P181E).
- **Não** modificar `Introspector` trait (P181F).
- **Não** modificar walk (P181H).
- **Não** modificar Layouter (P181G).
- API pública preservada.
- Output observable não muda.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar variants existentes:
   - `01_core/src/entities/element_kind.rs`: 8 variants
     (Heading, Figure, Citation, Metadata, State,
     StateUpdate, Outline) + qualquer outro adicionado.
     Adicionar Bibliography como variant 9.
   - `01_core/src/entities/element_payload.rs`: 8
     variants paralelos.

2. Confirmar localização do helper `from_name`:
   - `ElementKind::from_name(&str) -> Option<ElementKind>`
     em `element_kind.rs` (criado P175 `.A`).
   - Arm `"outline" => Some(Outline)` adicionada P178.
   - Adicionar arm `"bibliography" => Some(Bibliography)`.

3. Confirmar `BibEntry`:
   - `01_core/src/entities/bib_entry.rs:82-100`.
   - Deriva `Debug, Clone, PartialEq, Eq` (P181B
     confirmou).
   - **Não** deriva `Hash` provavelmente (16 fields,
     alguns podem ser tipos sem Hash). Verificar.
   - Se `BibEntry` deriva `Hash`: payload pode usar
     derive automático.
   - Se não: payload usa workaround `format!("{:?}",
     self).hash()` (P169 padrão).

4. Confirmar L0s actuais:
   - `00_nucleo/prompts/entities/element_kind.md`.
   - `00_nucleo/prompts/entities/element_payload.md`.
   - Identificar onde inserir entradas Bibliography.

Output: notas internas; sem ficheiro novo.

**Critério de saída**:
- Auditoria limpa.
- Decisão sobre Hash automatic vs workaround.

### .B Update L0 `entities/element_kind.md`

Adicionar entrada para `Bibliography`:

- Variant `Bibliography` adicionada à enumeração.
- Documentar que mapeia para `Content::Bibliography`
  (locatable a partir de P181D).
- `from_name("bibliography")` retorna
  `Some(ElementKind::Bibliography)`.

Critérios verificáveis:
- `ElementKind::Bibliography` existe e é distinto.
- `from_name("bibliography")` round-trip.
- `Bibliography != Outline`.

**Critério de saída**:
- L0 actualizado.
- Hash recalculado quando humano confirmar (`.E`).

### .C Update L0 `entities/element_payload.md`

Adicionar entrada para `Bibliography`:

- Variant `Bibliography { entries: Vec<BibEntry> }`.
- Documentar que carrega entries completos (P181A
  cláusula 2 — captura full).
- Hash via `format!` se `BibEntry` não deriva Hash
  (decisão `.A`).

Critérios verificáveis:
- `ElementPayload::Bibliography { entries }` constructível.
- Igualdade estrutural funciona.
- Hash determinístico (ou via workaround).

**Critério de saída**:
- L0 actualizado.
- Hash recalculado em `.E`.

### .D Tests primeiro (devem falhar)

Em `01_core/src/entities/element_kind.rs::tests`:

```rust
#[test]
fn bibliography_existe_e_distinto() {
    let k = ElementKind::Bibliography;
    assert_ne!(k, ElementKind::Outline);
    assert_ne!(k, ElementKind::Heading);
}

#[test]
fn from_name_bibliography() {
    assert_eq!(
        ElementKind::from_name("bibliography"),
        Some(ElementKind::Bibliography)
    );
}

#[test]
fn bibliography_as_str() {
    // Se ElementKind::as_str() existe (P175):
    assert_eq!(ElementKind::Bibliography.as_str(), "bibliography");
}
```

Em `01_core/src/entities/element_payload.rs::tests`:

```rust
#[test]
fn bibliography_construi_e_compara() {
    use crate::entities::BibEntry;
    let entries = vec![/* mock entry */];
    let p1 = ElementPayload::Bibliography { entries: entries.clone() };
    let p2 = ElementPayload::Bibliography { entries };
    assert_eq!(p1, p2);
}

#[test]
fn bibliography_e_distinto_de_outras() {
    let bib = ElementPayload::Bibliography { entries: vec![] };
    let meta = ElementPayload::Metadata { value: Box::new(Value::None) };
    // (forma exacta de Metadata depende de P169)
    assert_ne!(bib, meta);
}
```

Confirmar que `cargo test` falha (variants ainda não
existem em código).

**Critério de saída**:
- Tests escritos.
- Compilação falha como esperado.

### .E Humano calcula `@prompt-hash`

Marco humano. Após `.B` e `.C`:
- Calcular hash de `element_kind.md` actualizado.
- Calcular hash de `element_payload.md` actualizado.
- `crystalline-lint --fix-hashes` automatiza.
- L1 linhagens em `element_kind.rs` e
  `element_payload.rs` atualizadas.

**Critério de saída**:
- L0 hashes preenchidos.
- L1 `@prompt-hash` correspondentes.

### .F Implementar variants

1. Em `01_core/src/entities/element_kind.rs`:
   - Adicionar variant:
     ```rust
     pub enum ElementKind {
         // ... existentes
         Bibliography,
     }
     ```
   - Update `as_str` (se existe):
     ```rust
     ElementKind::Bibliography => "bibliography",
     ```
   - Update `from_name`:
     ```rust
     "bibliography" => Some(ElementKind::Bibliography),
     ```

2. Em `01_core/src/entities/element_payload.rs`:
   - Adicionar variant:
     ```rust
     pub enum ElementPayload {
         // ... existentes
         Bibliography { entries: Vec<BibEntry> },
     }
     ```
   - Adaptar Hash manual se aplicável:
     - Se `BibEntry: Hash`: derive automático cobre.
     - Se não: workaround P169 — `impl Hash for ElementPayload`
       via `format!("{:?}", self).hash()`. Provavelmente
       já existe se outros variants têm `Box<Value>` ou
       `Func`. Adicionar arm Bibliography ao Hash manual
       se necessário.

3. Confirmar tests `.D` agora passam.

**Critério de saída**:
- `cargo check` passa.
- Tests `.D` passam.
- Linter passa.

### .G Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` — todos os tests
   passam. Δ vs P181B baseline (1708). Estimativa: +5
   (3 element_kind + 2 element_payload).
3. `crystalline-lint .`: zero violations.
4. L0 `element_kind.md` actualizado com hash.
5. L0 `element_payload.md` actualizado com hash.
6. L1 `element_kind.rs` linhagem actualizada.
7. L1 `element_payload.rs` linhagem actualizada.
8. `ElementKind::Bibliography` existe.
9. `ElementPayload::Bibliography { entries: Vec<BibEntry> }`
   existe.
10. `ElementKind::from_name("bibliography")` retorna
    `Some(Bibliography)`.
11. Walk **NÃO modificado**.
12. `is_locatable`, `extract_payload`, `from_tags`
    **NÃO modificados** — `Content::Bibliography`
    continua a ser tratado como **não-locatable** até
    P181D inverter.
13. `BibStore` (P181B) **NÃO populado** ainda — campo
    permanece vazio.
14. Layouter **NÃO modificado**.
15. Snapshot tests ADR-0033 verdes.
16. Linter passa final.

### .H Encerramento

Escrever
`00_nucleo/materialization/typst-passo-181c-relatorio.md`
com:

- Resumo: variants Bibliography adicionadas a
  `ElementKind` + `ElementPayload`. `from_name`
  estendido. Sem alteração comportamental — preparação
  para P181D.
- Confirmação de cada verificação `.G`.
- Hashes finais de L0s modificados.
- Decisões registadas em `.A`:
  - `BibEntry` Hash derivações (automático ou
    workaround).
  - Forma exacta do variant payload.
- Δ tests vs baseline P181B (esperado ~+5).
- **Estado de P181**: A, B, C concluídos; D-J pendentes.
- **M9**: 9/11 features (sem alteração).
- Pendências cumulativas inalteradas.
- Estado pós-passo: P181C concluído. P181D
  desbloqueado (`is_locatable` + `extract_payload`).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria L0 sem disparar gate.
2. L0 `element_kind.md` actualizado com Bibliography.
3. L0 `element_payload.md` actualizado com Bibliography.
4. Tests escritos primeiro (`.D`) e confirmaram falhar.
5. Hashes calculados (`.E`).
6. Variants implementados em L1 (`.F`).
7. Verificações `.G` 1-16 passam.
8. Relatório `.H` escrito.
9. Output observable não muda.
10. `Content::Bibliography` continua a ser tratado como
    não-locatable até P181D — `is_locatable` retorna
    `false`, `extract_payload` retorna `None`.

---

## O que pode sair errado

- **`BibEntry` não deriva `Hash`**: workaround via
  `format!("{:?}", self)` em impl manual de Hash do
  `ElementPayload`. Padrão P169 estabelecido. Verificar
  se `ElementPayload` já tem Hash manual (P171
  StateUpdate adicionou).
- **`as_str` ausente**: P175 mencionou. Se ausente,
  ignorar test `bibliography_as_str`.
- **`from_name` não existe**: P175 .A reportou criação.
  Se ausente, criar trivialmente.
- **Variant novo em `ElementKind`/`ElementPayload`
  força exhaustive matches**: provavelmente sim em
  vários sítios (decisão da equipa P165 sobre
  exhaustivity). Compilador guia. Cada arm novo
  retorna defensive ou reuso. Aceitável.
- **`BibEntry` clone caro em payload**: 16 fields ×
  Vec<BibEntry> em payload pode ser pesado em hash.
  Refino futuro pode capturar apenas keys (campo único).
  Adiar.
- **L0 hash divergence pós-update**: ajustar via
  `crystalline-lint --fix-hashes`.

---

## Notas operacionais

- **Tamanho**: S. Trabalho mecânico — adicionar variant
  a 2 enums + 2 arms em helpers + 5 tests.
- **Pré-condição P181D**: `ElementKind::Bibliography`
  e `ElementPayload::Bibliography` existem mas ninguém
  os produz. P181D adiciona `extract_payload` arm que
  os produz; `is_locatable` arm que retorna `true`.
- **Cláusula gate trivial**: aplicável a workaround
  Hash, ausência de `as_str`, etc.
- **Output observable inalterado**: nada usa os
  variants ainda. P181D começa a usar.
- **Walk arm cascade**: pode haver matches sobre
  `ElementPayload` em sítios além de `from_tags`.
  Compilador guia; cada arm novo retorna `_ => ...`
  defensive.
- **Padrão Hash manual**: se `ElementPayload` tem Hash
  manual (provável desde P171 com `StateUpdate::Func`),
  adicionar arm Bibliography ao match interno.

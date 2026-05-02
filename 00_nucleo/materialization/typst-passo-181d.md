# Passo P181D — `is_locatable` + `extract_payload` arms para Bibliography

Terceiro passo de materialização P181 (após P181B, P181C).
Magnitude **S**.

Activa `Content::Bibliography` como kind locatable. Walk começa
a emitir tag automaticamente; `from_tags` recebe tag mas tem
arm no-op (P181C) — `BibStore` continua vazio até P181E.

Replica padrão estabelecido por P178 (Outline). Sem variant
Content novo; apenas modifica comportamento de funções que
matcham sobre `Content::Bibliography`.

**Pré-condição**: P181C concluído. `ElementKind::Bibliography`
e `ElementPayload::Bibliography { entries: Vec<BibEntry> }`
existem.

**Restrições**:
- **Não** modificar `Content` enum.
- **Não** modificar walk em `rules/introspect.rs::walk` (P181H).
- **Não** modificar `from_tags` arm Bibliography (continua no-op
  defensivo desde P181C; P181E liga população).
- **Não** modificar `Introspector` trait (P181F).
- **Não** modificar Layouter (P181G).
- API pública preservada.
- Output observable não muda — tags emitidas mas descartadas
  por consumers actuais.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar estado actual de `is_locatable`:
   - `01_core/src/rules/introspect/locatable.rs`.
   - `Content::Bibliography { .. }` está em bloco
     non-locatable (or-pattern com outros) ou arm
     dedicado `=> false`.
   - Localizar onde mover.

2. Confirmar estado actual de `extract_payload`:
   - `01_core/src/rules/introspect/extract_payload.rs`.
   - Match com `_ => None` fall-through (P164 confirmou).
   - Identificar onde inserir arm novo (antes do
     fall-through, agrupado com outros arms locatable).

3. Confirmar campos de `Content::Bibliography`:
   - `01_core/src/entities/content.rs`.
   - `Content::Bibliography { entries, .. }` — confirmar
     nome exacto do field.
   - Vanilla cristalino: `entries: Vec<BibEntry>` (P181A
     confirmou). Outros fields possíveis: `path`, `style`,
     `title`, `full`. Usar `..` para ignorar.

4. Confirmar L0s actuais:
   - `00_nucleo/prompts/rules/introspect/locatable.md`.
   - `00_nucleo/prompts/rules/introspect/extract_payload.md`.
   - Identificar entradas existentes para Outline (P178
     padrão) — Bibliography vai entrar paralelamente.

Output: notas internas; sem ficheiro novo.

**Critério de saída**:
- Auditoria limpa.
- Confirmação que P178 padrão é replicável literalmente.

### .B Tests primeiro (devem falhar)

Em `01_core/src/rules/introspect/locatable.rs::tests`:

```rust
#[test]
fn bibliography_is_locatable_p181d() {
    let content = Content::Bibliography {
        // campos mínimos para construir
        entries: vec![],
        // outros fields conforme Content::Bibliography real
        ..
    };
    assert!(is_locatable(&content));
}
```

Em `01_core/src/rules/introspect/extract_payload.rs::tests`:

```rust
#[test]
fn extract_payload_bibliography_devolve_some_p181d() {
    let entry = make_bib_entry("intro");
    let content = Content::Bibliography {
        entries: vec![entry.clone()],
        ..
    };
    let payload = extract_payload(&content);
    assert!(matches!(
        payload,
        Some(ElementPayload::Bibliography { entries }) if entries.len() == 1
    ));
}

#[test]
fn extract_payload_bibliography_clones_entries_p181d() {
    let entry = make_bib_entry("test");
    let content = Content::Bibliography {
        entries: vec![entry.clone(), entry.clone()],
        ..
    };
    let payload = extract_payload(&content).unwrap();
    if let ElementPayload::Bibliography { entries } = payload {
        assert_eq!(entries.len(), 2);
    } else {
        panic!("expected Bibliography");
    }
}

#[test]
fn invariante_is_locatable_extract_payload_para_bibliography_p181d() {
    let content = Content::Bibliography { entries: vec![], .. };
    assert_eq!(is_locatable(&content), extract_payload(&content).is_some());
}
```

Confirmar que `cargo test` falha — `is_locatable` retorna
`false` actualmente (P164 baseline), `extract_payload`
retorna `None`.

**Critério de saída**:
- Tests escritos.
- Falham conforme esperado.

### .C Update L0 `locatable.md`

Documentar `Content::Bibliography` agora locatable.

Forma:
- Mover Bibliography do bloco "não-locatable" para o bloco
  "locatable".
- Documentar racional: P181D activa Bibliography como kind
  queryável via Introspector.
- Match continua exaustivo (P164 invariante preservado).

**Critério de saída**:
- L0 actualizado.
- Hash recalculado em `.E`.

### .D Update L0 `extract_payload.md`

Documentar arm novo para Bibliography.

Forma:
- Adicionar entrada para `Content::Bibliography { entries }`
  → `Some(ElementPayload::Bibliography { entries: entries.clone() })`.
- Posição: junto com outros arms locatable (Heading, Figure,
  Cite, Metadata, State, StateUpdate, Outline).

**Critério de saída**:
- L0 actualizado.
- Hash recalculado em `.E`.

### .E Humano calcula `@prompt-hash`

Marco humano. Após `.C` e `.D`:
- `crystalline-lint --fix-hashes` automatiza.
- L1 linhagens em `locatable.rs` e `extract_payload.rs`
  actualizadas.

**Critério de saída**:
- L0 hashes preenchidos.
- L1 `@prompt-hash` correspondentes.

### .F Implementar `is_locatable` arm

Em `01_core/src/rules/introspect/locatable.rs`:

- Mover `Content::Bibliography { .. }` do bloco
  non-locatable para arm `=> true`.
- Forma idêntica a P178 (Outline):
  ```rust
  Content::Bibliography { .. } => true,
  ```
  (na secção locatable do match).
- Match continua exaustivo (sem `_ => false`).

**Critério de saída**:
- `cargo check` passa.
- Test `.B` `bibliography_is_locatable_p181d` passa.
- Linter passa.

### .G Implementar `extract_payload` arm

Em `01_core/src/rules/introspect/extract_payload.rs`:

- Adicionar arm antes do `_ => None`:
  ```rust
  Content::Bibliography { entries, .. } => Some(
      ElementPayload::Bibliography { entries: entries.clone() }
  ),
  ```
- Adaptar campo exacto conforme `Content::Bibliography`
  real.

**Critério de saída**:
- `cargo check` passa.
- Tests `.B` extract_payload + invariante passam.
- Linter passa.

### .H Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` — todos os tests
   passam. Δ vs P181C baseline (1714). Estimativa: +4
   (1 locatable + 3 extract_payload).
3. `crystalline-lint .`: zero violations.
4. L0 `locatable.md` actualizado com hash.
5. L0 `extract_payload.md` actualizado com hash.
6. L1 `locatable.rs` linhagem actualizada.
7. L1 `extract_payload.rs` linhagem actualizada.
8. `is_locatable(&Content::Bibliography {..}) == true`.
9. `extract_payload(&Content::Bibliography {..})` retorna
   `Some(ElementPayload::Bibliography { entries })`.
10. Invariante `is_locatable == extract_payload.is_some()`
    para Bibliography.
11. Walk **NÃO modificado** estruturalmente. **Mas** walk
    agora chama `extract_payload` que retorna `Some` para
    Bibliography — tag emitida automaticamente. Isto NÃO
    é modificação de walk; é consequência directa de
    `extract_payload` mudar.
12. `from_tags` arm Bibliography continua **no-op**
    (P181C). `BibStore` continua **vazio em produção**.
13. `Content::Bibliography` walk arm em `walk()`
    inalterado — continua a popular `state.bib_*`
    legacy.
14. Layouter **NÃO modificado**.
15. Snapshot tests ADR-0033 verdes.
16. Linter passa final.

### .I Encerramento

Escrever
`00_nucleo/materialization/typst-passo-181d-relatorio.md`
com:

- Resumo: `Content::Bibliography` agora locatable;
  walk emite tag automaticamente; `from_tags` arm
  continua no-op (até P181E).
- Confirmação de cada verificação `.H`.
- Hashes finais de L0s modificados.
- Decisões registadas em `.A`:
  - Campos exactos de `Content::Bibliography`.
  - Padrão P178 (Outline) replicado literalmente.
- Δ tests vs baseline P181C (esperado +4).
- **Estado de P181**: A, B, C, D concluídos; E-J
  pendentes.
- **M9**: 9/11 features (sem alteração — feature bib
  não conta até P181I).
- Pendências cumulativas inalteradas.
- Estado pós-passo: P181D concluído. P181E
  desbloqueado (`from_tags` arm popula `BibStore`).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate.
2. Tests escritos primeiro (`.B`) e confirmaram falhar.
3. L0 `locatable.md` actualizado.
4. L0 `extract_payload.md` actualizado.
5. Hashes calculados (`.E`).
6. `is_locatable` arm modificado (`Bibliography => true`).
7. `extract_payload` arm adicionado.
8. Verificações `.H` 1-16 passam.
9. Relatório `.I` escrito.
10. Output observable não muda — tags fluem mas ninguém
    as consome ainda.
11. `BibStore` continua vazio em produção.
12. Walk e Layouter inalterados.

---

## O que pode sair errado

- **`Content::Bibliography` tem mais campos do que
  esperado**: usar `..` ignora-os. Apenas `entries` é
  capturado.
- **`Content::Bibliography` tem field com nome
  diferente**: `.A` confirma. Adaptar.
- **`is_locatable` em or-pattern**: P164 P178 padrão.
  Mover para arm dedicado.
- **Test stubs precisam helper `make_bib_entry`**:
  `BibEntry` constructor existe ou criar trivialmente
  em test.
- **Walk emite tag para Bibliography mas
  `state.bib_entries` também é populado**: redundância
  intencional em P181D — `from_tags` arm continua no-op,
  e walk arm continua a mutar state legacy. Output
  observable inalterado. P181H limpa walk arm depois
  de P181E ligar `BibStore` população.
- **Snapshot tests detectam mudança**: improvável.
  Tags são consumidas só por `Introspector` que ignora
  Bibliography (no-op). State legacy continua a ser
  fonte para Layouter.
- **Linter divergência**: ajustar.

---

## Notas operacionais

- **Tamanho**: S. Trabalho mecânico — 2 arms modificados,
  4 tests, 2 L0s.
- **Pré-condição P181E**: `Content::Bibliography` é
  locatable; walk emite tag; `from_tags` recebe tag mas
  ignora-a defensivamente. P181E muda arm para popular
  `BibStore`.
- **Cláusula gate trivial**: aplicável a campos exactos
  de `Content::Bibliography`, helper `make_bib_entry`.
- **Padrão P178 replicado literalmente**: sem decisões
  novas. Mecânica idêntica a Outline.
- **Output observable garantido inalterado**: walk
  continua a popular state legacy; tags emitidas mas
  ignoradas; Layouter lê de state legacy.
- **Redundância intencional em P181D-E-H**: walk arm
  popula state legacy; tag emitida via extract_payload;
  `from_tags` arm popula `BibStore` (P181E). Walk arm
  só remove mutação directa em P181H, depois de Layouter
  migrar (P181G).
- **Cascade compilador zero**: nenhum match exhaustive
  novo é forçado por P181D — apenas modifica
  comportamento de matches existentes (`is_locatable`,
  `extract_payload`).

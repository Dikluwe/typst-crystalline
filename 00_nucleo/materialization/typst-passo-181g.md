# Passo P181G — Layouter cite-arm migra para `Introspector`

Sexto passo de materialização P181 (após P181B–P181F).
Magnitude **M** — único M no plano.

Migra Layouter cite-arm de leitura directa de
`self.counter.bib_entries`/`bib_numbers` para
`self.introspector.bib_entry_for_key`/`bib_number_for_key`.
Replica padrão P168 (figure-ref) com substitution-with-fallback.

Após P181G:
- Cite-arm consome via `Introspector` (primeiro consumer M5
  bib).
- Walk arm legacy continua a popular `state.bib_*` (até P181H).
- `from_tags` continua a popular `BibStore` (desde P181E).
- Fallback defensivo a state legacy preservado durante janela
  compat — eliminado em M6.

**Pré-condição**: P181F concluído. Trait `Introspector`
expõe `bib_entry_for_key` + `bib_number_for_key`.
`BibStore` populado em produção (P181E) sempre que walk
processa Bibliography. Paridade `BibStore` vs `state.bib_*`
garantida por construção (P181E §6).

**Restrições**:
- **Não** modificar walk em `rules/introspect.rs::walk` (P181H).
- **Não** modificar walk arm `Content::Bibliography`
  (linha 567-573) — continua a popular state legacy.
- **Não** eliminar copy-sites `state→Layouter` (1385-1388
  e 1413-1416) — necessários para fallback funcionar
  durante compat.
- API pública preservada — `Layouter` recebe `introspector`
  via `layout_with_introspector` desde P168; sem nova
  signature.
- Output observable não muda — paridade `BibStore` vs state
  garante mesmo resultado.

---

## Sub-passos

### .A Auditoria L0 + decisão fallback

1. Confirmar cite-arm actual:
   - `01_core/src/rules/layout/mod.rs:584-597`.
   - Identificar exactamente:
     - Variável local que recebe leitura de bib_entries.
     - Variável local que recebe leitura de bib_numbers.
     - Como entry e number são usados nas 4 cite forms.

2. Confirmar 4 cite forms:
   - Normal: usa `number`.
   - Prose: usa `entry` (autor + ano).
   - Author: usa `entry` (autor).
   - Year: usa `entry` (ano).
   - Confirmar nomes exactos dos forms em
     `entities/cite.rs` ou similar.

3. Confirmar copy-sites:
   - `mod.rs:1385-1388` (`pub fn layout`).
   - `mod.rs:1413-1416` (`pub fn layout_with_introspector`).
   - Documentar que são mantidos como fallback.

4. Decisão sobre fallback:

   **Opção F1** — substitution-with-fallback (padrão P168):
   ```rust
   let entry = self.introspector
       .bib_entry_for_key(key)
       .or_else(|| self.counter.bib_entries.iter().find(|e| e.key == key));
   ```
   - Defensive — se Introspector não for reconstruído,
     state legacy salva.
   - Replica P168.

   **Opção F2** — substitution sem fallback:
   ```rust
   let entry = self.introspector.bib_entry_for_key(key);
   ```
   - Mais limpo.
   - Confia que P181E garante paridade.
   - Se introspector vazio, cite-arm falha graciosamente
     (None propagado).

   Sugestão: **F1** durante janela compat. F2 quando state
   legacy desaparecer (M6). Justificação: paridade é
   garantida por construção mas sem fallback é regressão
   silenciosa em caso de bug futuro de reconstrução.

5. Confirmar acesso a `self.introspector` no contexto
   cite-arm:
   - Layouter ganhou field `introspector` em P168.
   - Confirmar que `&self.introspector` é acessível dentro
     de `layout_ref` ou método equivalente.

Output: notas internas; sem ficheiro novo.

**Critério de saída**:
- Auditoria limpa.
- Decisão fallback registada (F1 sugerido).
- 4 cite forms identificados.

### .B Tests primeiro (devem falhar parcialmente)

Em `01_core/src/rules/layout/tests/` ou módulo
equivalente:

```rust
#[test]
fn cite_normal_renderiza_via_introspector_p181g() {
    // Construir Content com Bibliography + Cite Normal.
    // Layouter usa introspector populado.
    // Verificar render output contém número.
    // ...
}

#[test]
fn cite_prose_renderiza_via_introspector_p181g() {
    // Construir Content com Bibliography + Cite Prose.
    // Verificar render contém autor + ano.
    // ...
}

#[test]
fn cite_author_renderiza_via_introspector_p181g() {
    // Verificar render contém autor.
    // ...
}

#[test]
fn cite_year_renderiza_via_introspector_p181g() {
    // Verificar render contém ano.
    // ...
}

#[test]
fn cite_paridade_state_legacy_vs_introspector_p181g() {
    // Mesmo Content renderizado via:
    //   1. layout(content) — state legacy.
    //   2. layout_with_introspector(content) — introspector.
    // Output observable idêntico.
}
```

Confirmar que tests novos compilam mas falham (cite-arm
ainda lê state directamente).

Tests existentes de cite (P162+) **continuam a passar**
porque path legacy preservado.

**Critério de saída**:
- Tests escritos.
- Tests novos falham conforme esperado (paridade não
  ainda garantida via Introspector porque cite-arm não
  usa).
- Tests existentes inalterados.

### .C Update L0 `layout/mod.md`

Documentar migração cite-arm:
- Cite-arm consulta `Introspector` primeiro (`bib_entry_for_key`,
  `bib_number_for_key`).
- Fallback a `self.counter.bib_*` durante janela compat
  (M6 elimina).
- 4 cite forms (Normal/Prose/Author/Year) afectados.
- Output observable preservado por paridade `BibStore` vs
  state.

**Critério de saída**:
- L0 actualizado.
- Hash recalculado em `.D`.

### .D Humano calcula `@prompt-hash`

Marco humano. Após `.C`:
- `crystalline-lint --fix-hashes`.
- L1 linhagem `mod.rs` actualizada.

**Critério de saída**:
- L0 hash preenchido.
- L1 `@prompt-hash` correspondente.

### .E Implementar migração cite-arm

Em `01_core/src/rules/layout/mod.rs` linhas 584-597:

Substituir leitura de bib_entries:
```rust
// ANTES:
let entry = self.counter.bib_entries.iter().find(|e| e.key == key);

// DEPOIS (Opção F1):
let entry = self.introspector
    .bib_entry_for_key(key)
    .or_else(|| self.counter.bib_entries.iter().find(|e| e.key == key));
```

Substituir leitura de bib_numbers:
```rust
// ANTES:
let number = self.counter.bib_numbers.get(key);

// DEPOIS (Opção F1):
let number = self.introspector
    .bib_number_for_key(key)
    .or_else(|| self.counter.bib_numbers.get(key).copied());
```

Adaptar tipos exactos (`Option<&BibEntry>` vs `Option<u32>`).
Adaptar uso downstream nas 4 cite forms.

**Critério de saída**:
- `cargo check` passa.
- Tests `.B` passam (paridade verificada).
- Tests existentes continuam a passar.
- Linter passa.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` — todos os tests passam.
   Δ vs P181F baseline (1725). Estimativa: +5 (5 tests
   `.B`).
3. `crystalline-lint .`: zero violations.
4. L0 `layout/mod.md` actualizado com hash.
5. L1 `mod.rs` linhagem actualizada.
6. Cite-arm consulta `Introspector` primeiro.
7. Fallback a state legacy preservado.
8. 4 cite forms (Normal/Prose/Author/Year) renderizam
   correctamente.
9. Paridade pre/post-migração confirmada via test
   dedicado.
10. Walk **NÃO modificado**.
11. Walk arm `Content::Bibliography` (linha 567-573)
    inalterado.
12. Copy-sites `state→Layouter` (1385-1388, 1413-1416)
    preservados.
13. `BibStore` (`from_tags`) populado paralelamente.
14. State legacy (`walk arm`) populado paralelamente.
15. Snapshot tests ADR-0033 verdes — output observável
    inalterado.
16. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-181g-relatorio.md`
com:

- Resumo: cite-arm migrada para `Introspector` com
  fallback. 4 cite forms verificados; paridade confirmada.
- Confirmação de cada verificação `.F`.
- Hash final de `layout/mod.md`.
- Decisões registadas em `.A`:
  - Fallback F1 vs F2 (F1 escolhido).
  - 4 cite forms identificados.
  - Localização exacta das mudanças.
- Δ tests vs baseline P181F (esperado +5).
- **Estado de P181**: A, B, C, D, E, F, G concluídos;
  H-J pendentes.
- **Estado de M5**: 2/6 consumers migrados (figure-ref
  P168 + cite-arm P181G).
- **Padrão P168 replicado** com sucesso pela 2ª vez.
- Pendências cumulativas inalteradas.
- Estado pós-passo: P181G concluído. P181H desbloqueado
  (walk arm puro: remove mutação directa
  `state.bib_*`).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria + decisão fallback registada.
2. Tests escritos primeiro (`.B`); novos falharam, antigos
   inalterados.
3. L0 `layout/mod.md` actualizado.
4. Hash calculado (`.D`).
5. Cite-arm migrada com fallback F1.
6. Verificações `.F` 1-16 passam.
7. Relatório `.G` escrito.
8. Output observable não muda — paridade verificada.
9. M5 progresso: 2/6 consumers migrados.
10. Walk e walk arm inalterados.

---

## O que pode sair errado

- **`self.introspector` não acessível em cite-arm
  context**: P168 deveria ter exposto. Verificar; se
  ausente, gate substancial.
- **Cite forms têm semânticas diferentes do esperado**:
  P181A diagnóstico reportou 4 hardcoded; verificar
  `entities/cite.rs` para forma exacta. Adaptar.
- **`bib_entry_for_key` retorna referência mas
  cite-arm precisa value**: clone ou referência
  consistente. Adaptar.
- **Tests E2E de cite são complexos**: precisam construir
  Bibliography + Cite + render. Pode exigir helpers
  novos. Adiar tests E2E ricos para P181I se necessário;
  P181G mantém tests minimais focados em paridade.
- **Paridade falha**: divergência entre
  `BibStore` e `state.bib_*`. Investigar causa raiz —
  provavelmente bug em P181E (numeração) ou copy-site
  desincronizado.
- **`bib_numbers.get(key)` retorna `Option<&u32>`,
  `bib_number_for_key` retorna `Option<u32>`**:
  conversão `.copied()` pode ser necessária no fallback.
  Cláusula gate trivial.
- **Fallback F1 induz dupla pesquisa**: aceitar custo
  de leitura redundante durante janela compat. M6
  elimina.
- **Linter divergência**: ajustar.

---

## Notas operacionais

- **Tamanho**: M. Toca cite-arm (substancial — 4 cite forms
  + ~13 linhas de leitura) + L0 + 5 tests E2E. Único M
  no plano P181.
- **Pré-condição P181H**: cite-arm consome via
  Introspector. Walk arm legacy pode tornar-se puro sem
  quebrar consumer (cite-arm já lê de outro sítio).
- **Cláusula gate trivial**: aplicável a `.copied()`,
  4 cite forms exactos, helpers de test.
- **Padrão P168 replicado**: figure-ref foi 1º consumer
  M5; cite-arm é 2º. Mecânica idêntica
  (substitution-with-fallback). Confirma que padrão é
  reutilizável para outros consumers M5 futuros.
- **Janela compat**: walk arm legacy + copy-sites + state
  legacy + fallback são todos componentes da janela.
  Eliminados gradualmente em M6 quando lacuna #6 fechar
  e M5 ficar saturado.
- **M5 progresso**: 2/6 consumers migrados após P181G.
  Restantes 4 bloqueados por outras lacunas (#3 outline
  body, ou padrões mutação inerentes).
- **Output observable garantido inalterado**: paridade
  `BibStore` vs state assegura mesmo resultado, fallback
  defende caso introspector vazio.
- **Magnitude M comparável a P168**: figure-ref
  migration foi M também. Consistência.

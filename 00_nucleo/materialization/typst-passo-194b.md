# Passo P194B — Migrar C4 resolved label + tests E2E

Único passo de implementação P194 (após P194A diagnóstico).
Magnitude **S** agregada — passo único combinando migração
consumer C4, comentário inline curto, 4 tests E2E,
actualização de nota DEBT M5-residual, e relatório
consolidado.

**Passo 2 da sequência §9 P189 consolidado.**

Migra consumer C4 em
`01_core/src/rules/layout/references.rs:53-57` para forma
Opção C fixada em P194A §3:

```rust
let display_text = match layouter.introspector
    .resolved_label_for(target)
    .or_else(|| layouter.counter.resolved_labels.get(target).map(String::as_str))
{
    Some(text) => text.to_string(),
    None       => format!("@{}", target.0),
};
```

Após P194B:
- Consumer C4 consulta Introspector primeiro; fallback
  legacy quando `None`.
- Em produção real até P195+, sub-store P193B fica vazio
  → Introspector path retorna `None` → fallback legacy é
  caminho funcional. Output observable **inalterado por
  construção**.
- Após P195 + P196, walks Labelled/Heading populam
  sub-store via Tag → Introspector path activa →
  fallback torna-se redundante (mas inofensivo até M6
  cleanup).
- DEBT M5-residual: 3 → 2 pré-requisitos pendentes.
- **Excepções E2-E6 continuam activas** (P195+
  destranca; P194 desbloqueia consumer apenas).

**Pré-condição**: P194A concluído. Tests workspace 1.821
verdes; zero violations. Forma da expressão fixada
(Opção C). 6 cláusulas P194A fechadas.

**Restrições**:
- **Não** modificar trait `Introspector` (P185B fechou).
- **Não** modificar `TagIntrospector` (P193B fechou).
- **Não** tocar copy-sites Layouter
  (`mod.rs:1481, 1512`) — decisão fixada P194A `.D`
  (manter durante janela compat M5).
- **Não** modificar walk arms — P195+.
- **Não** modificar `from_tags` — P195+.
- **Não** popular sub-store via Tag — P195.
- **Não** abrir DEBT M5-residual formal (Cenário B
  continua).
- API pública preservada.
- Output observable em produção **inalterado** —
  sub-store vazio; fallback legacy chamado
  consistentemente; resultado idêntico ao actual.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar consumer C4 em `references.rs:53-57`:
   - Per P194A §2.1, site real é 53-57. Re-verificar
     empiricamente — se localização mudou entre P194A e
     P194B, ajustar.
   - Localizar match legacy (forma per P194A §11.5).

2. Confirmar acesso a `layouter.introspector`:
   - Per P194A §11.2, Cenário α directo confirmado por
     linha 44 (P168 padrão).
   - Re-verificar: `layouter.introspector.figure_number_for_label(...)`
     usa pattern directo.

3. Confirmar L0 `rules/layout.md`:
   - Localizar entradas existentes sobre layout_ref
     (P168 introduziu).
   - Identificar onde adicionar nota sobre migração C4
     com comentário curto sobre estado temporário.

4. Confirmar tests existentes que cobrem `layout_ref`:
   - `grep -rn "layout_ref\|references.*test\|cross.ref"
     01_core/src/`.
   - Per P194A §2.6, esperado:
     `layout_resolved_labels_nao_interfere_entre_documentos`
     ou similar.
   - Identificar quais devem manter-se inalterados após
     P194B (paridade observable preservada por construção).

5. Confirmar copy-sites Layouter:
   - `mod.rs:1481, 1512` — **não tocar** (decisão P194A
     `.D`).
   - Verificar empiricamente que ainda existem.

6. Confirmar nota DEBT M5-residual em relatórios:
   - Per P194A §8: notas em
     P189-consolidado/P193-consolidado.
   - Localizar para actualização (3 → 2 pré-requisitos).

Output: tabela com item + estado + linhas exactas para
edits.

**Critério de saída**:
- Site C4 confirmado.
- Tests existentes inventariados.
- Copy-sites localizados (não tocar).

### .B Migrar consumer C4

1. Em `01_core/src/rules/layout/references.rs:53-57`
   (ou linhas reais per `.A.1`):
   - Substituir match legacy pela forma Opção C fixada
     em P194A §3:
     ```
     let display_text = match layouter.introspector
         .resolved_label_for(target)
         .or_else(|| layouter.counter.resolved_labels
             .get(target).map(String::as_str))
     {
         Some(text) => text.to_string(),
         None       => format!("@{}", target.0),
     };
     ```
   - Forma exacta fica para Claude Code conforme
     convenção do projecto.

2. **Adicionar comentário inline curto** (per P194A §3
   cláusula 5 Opção B):
   - Texto sugerido (curto, factual):
     ```
     // Introspector path activa após P195 (walk
     // Labelled migrated). Durante janela compat,
     // fallback legacy é caminho funcional.
     ```
   - Localização: imediatamente antes da expressão.

3. Confirmar `@prompt-hash` actualiza após edit do L0
   (em `.C`).

**Critério de saída**:
- `cargo check --workspace` passa.
- Tests existentes não regridem (paridade observable
  preservada por construção — sub-store vazio; fallback
  legacy fornece valor idêntico).
- Linter passa (após `--fix-hashes` em `.D`).

### .C Actualizar L0 `rules/layout.md`

1. Adicionar entrada para C4 migration:
   - Consumer C4 (resolved-label resolution) consulta
     Introspector via `resolved_label_for(target)`
     primeiro; fallback legacy
     `counter.resolved_labels.get(target)` activo durante
     janela compat M5.
   - **Estado temporário** (não permanente face a P188B):
     Introspector activa após P195 + P196 (walk arms
     Labelled/Heading migrados).
   - Cross-references: P193A §11.3 (consumer simples),
     P194A §11.4 (tabela diferença P188B vs P194), P189
     §9 sequência.

2. Hash em branco aguarda recálculo manual (`.D`).

**Critério de saída**:
- L0 contém entrada para C4 migration com estado
  temporário documentado.
- Coerente com entradas P168 (figure-ref) existentes.

### .D Tests E2E em submódulo `p194b_c4_resolved_label`

Submódulo novo em `01_core/src/rules/layout/tests.rs`
(ou ficheiro de tests do `references.rs` se separado).
Irmão de `p184e_figure_per_kind`, `p185d_locator_sync`,
`p186f_equation_locatable`, `p187b_c1_heading_prefix`,
`p188b_c2_equation_counter` (se este existir),
`p189b_walk_puro_m5`.

4 tests obrigatórios (per P194A §13).

#### Test 1 — `c4_resolved_label_via_introspector_path_quando_populated`

1. Construir cenário com sub-store P193B
   pre-populated:
   ```
   let mut intr = TagIntrospector::empty();
   intr.resolved_labels.insert(
       Label("intro".to_string()),
       "Capítulo 1".to_string(),
   );
   ```

2. Layouter recebe esse Introspector via
   `layout_with_introspector(content, &intr)`.

3. Documento contém referência:
   ```
   Content::Sequence(vec![
       /* setup */,
       Content::Ref(Label("intro".to_string())),
   ])
   ```

4. Asserções:
   - `plain_text` contém "Capítulo 1" (não `@intro`).
   - Validação intermédia:
     `intr.resolved_label_for(&Label("intro".into()))`
     retorna `Some("Capítulo 1")`.

#### Test 2 — `c4_resolved_label_via_fallback_legacy_caso_atual`

**Caso central da produção** (per P194A §11.6).

1. Documento com Heading + Ref:
   ```
   Content::Sequence(vec![
       Content::SetHeadingNumbering { active: true },
       Content::Heading {
           level: 1,
           body: Box::new(Content::Text("Intro".into())),
           label: Some(Label("intro".into())),
       },
       Content::Ref(Label("intro".into())),
   ])
   ```

2. Pipeline normal: walk + from_tags + layout (sem
   inject directo no sub-store).

3. Asserções:
   - `intr.resolved_label_for(&Label("intro".into()))`
     retorna `None` (sub-store vazio em produção até
     P195).
   - `or_else` cai em legacy
     `counter.resolved_labels.get(&Label("intro".into()))`
     que retorna `Some("Secção 1.")` (ou texto
     equivalente per walk legacy).
   - `plain_text` contém "Secção 1." (ou equivalente).
   - **Confirma cenário de produção real** coberto por
     fallback legacy.

#### Test 3 — `c4_resolved_label_paridade_legacy_vs_introspector`

1. Mesmo documento.
2. Path A: pipeline normal (sub-store vazio, legacy
   activo).
3. Path B: pipeline com state injectado no sub-store
   pre-walk.
4. Asserção: `plain_text(A) == plain_text(B)` quando
   ambos paths produzem o mesmo texto resolvido.
5. Confirma paridade observable entre os dois caminhos.

#### Test 4 — `c4_resolved_label_fallback_at_arrobado_quando_ausente`

1. Documento com Ref a label que **não existe**:
   ```
   Content::Sequence(vec![
       Content::Ref(Label("missing".into())),
   ])
   ```

2. Pipeline normal.

3. Asserções:
   - `intr.resolved_label_for(&Label("missing".into()))`
     retorna `None`.
   - `counter.resolved_labels.get(&Label("missing".into()))`
     também retorna `None`.
   - `plain_text` contém literal `@missing`.
   - Confirma fallback final do match.

**Critério de saída**:
- 4 tests novos passam.
- Tests existentes não regridem.

### .E Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P194A
   baseline (1.821): **+4**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Tests `p194b_c4_resolved_label::*` passam isoladamente
   (`cargo test --workspace --lib p194b`).
5. Tests existentes (incluindo
   `layout_resolved_labels_nao_interfere_entre_documentos`)
   **não regridem** — paridade observable preservada por
   construção.
6. Consumer C4 (`references.rs:53-57`) consulta
   Introspector primeiro; fallback legacy.
7. **Comentário inline obrigatório presente** em
   `references.rs`.
8. Walks legacy E2/E4 **NÃO modificados** — continuam a
   popular `state.resolved_labels`.
9. Trait `Introspector` **NÃO modificado**.
10. `TagIntrospector` **NÃO modificado**.
11. Copy-sites Layouter (`mod.rs:1481, 1512`) **NÃO
    modificados**.
12. P168 path (figure-ref linhas 44-49) **NÃO afectado**.
13. Snapshot tests ADR-0033 verdes.
14. Linter passa final.

### .F Actualizar nota DEBT M5-residual

P194 avança 1 dos 3 pré-requisitos pendentes (após P193B
ter avançado 1 dos 4).

1. **Não editar** relatórios anteriores (preservação
   histórica per P187B/P188B/P189B/P193B).

2. Adicionar nota nova no relatório consolidado P194
   (`.G`) que actualiza estado:
   - **Antes P194**: 3 pré-requisitos pendentes.
   - **Após P194B**: **2 pré-requisitos restantes**:
     1. ~~Sub-store `resolved_labels`~~ ✅ P193B.
     2. ~~C4 migration~~ ✅ P194B.
     3. Sub-store `headings_for_toc` — passo dedicado.
     4. `Content::SetEquationNumbering` — passo
        independente.
   - **Excepções E2-E6 continuam activas** — só fecham
     com P195+ quando walk arms migrarem para popular
     sub-store via Tag.

**Critério de saída**:
- Nota actualizada no relatório consolidado P194 (`.G`).
- Sem edits em relatórios anteriores.

### .G Escrever relatório consolidado P194

1. Criar
   `00_nucleo/materialization/typst-passo-194-relatorio-consolidado.md`
   com 9 secções (padrão P181J / P184F / P185 / P186 /
   P187 / P188 / P189 / P193):

   - §1 Resumo executivo + C4 migrado + 2º passo da
     sequência §9 P189.
   - §2 Sub-passos materializados (tabela métricas A–G
     dentro de P194B único).
   - §3 Decisões arquiteturais (6 cláusulas P194A
     fechadas).
   - §4 Achados não-triviais durante execução:
     - P194A §11.1 — site C4 está depois de figure-ref
       early-returns.
     - P194A §11.3 — Layouters secundários têm
       Introspector próprio (clonado).
     - P194A §11.4 — diferença chave face a P188B
       (estado temporário vs permanente).
     - P194A §11.5 — forma Opção C idiomática.
   - §5 **Estado temporário** (secção dedicada paralela
     a §5 P188 "Estado dormente"):
     - Diferença observable: Introspector path migra
       mas fica vazio em produção até P195+.
     - Razão: walks Labelled/Heading não migrados (E2/E4
       activos).
     - Activação: P195 + P196 sequenciais.
     - Janela compat M6 abre quando excepções E2-E6
       todas fecharem.
   - §6 Estado final M9 (inalterado 11/11) e M5
     (inalterado: 1 arm migrado + 6 excepções; 2 dos 4
     pré-requisitos avançados).
   - §7 Estado final lacunas (#3 inalterada).
   - §8 Pendências cumulativas + DEBT M5-residual
     (3 → 2 pré-requisitos) + nota actualizada (vide
     `.F`).
   - §9 Próximos passos sugeridos:
     - **P195 — migrar walk arm Labelled** — agora
       desbloqueado; magnitude S–M (depende de decisão
       sobre payload).
     - Sequência continua até M5 universal fechar.

2. Sem L0 novo (apenas edit a `rules/layout.md` em
   P194B `.C`).

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções presentes (com §5 dedicada a estado
  temporário).
- Nota DEBT M5-residual actualizada.

### .H Encerramento

P194B é o passo único de implementação. Após `.G`
concluído, série P194 está fechada.

Estado projectado pós-P194B:

- **P194 série**: A ✅ B ✅. Fechada.
- **Consumer C4 migrado** com Introspector como caminho
  preferido; fallback legacy durante janela compat.
- **2/4 pré-requisitos cumpridos** para fechar excepções
  E2-E6.
- **DEBT M5-residual**: 3 → 2 pré-requisitos restantes.
- **M9**: 11/11 (inalterado).
- **M5/M4 progresso (read-sites)**: 8/12 (inalterado —
  C4 não estava entre C1-C3 do M4-residual; é trabalho
  M5).

   **Nota**: per P189A §11.4-§11.6, C4 era originalmente
   contado como parte de M4-residual mas ficou pendente
   em P183E não corrido. Alguns relatórios contam 9/12
   após P194 (incluindo C4); outros 8/12 (excluindo).
   Auditor decide nuance da contagem em `.G`.

- **Tests workspace**: 1.821 → **1.825** (+4).
- **64 passos executados** (per P194A §12: P194A = 63 +
  P194B = 64).
- **Padrão diagnóstico-primeiro**: 16ª aplicação
  consecutiva (16/16 acertaram a magnitude planeada
  ±1 nível).
- **Próximo: P195 (migrar walk arm Labelled)** —
  desbloqueado.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. Consumer C4 migrado (`references.rs:53-57`).
3. **Comentário inline curto presente** em
   `references.rs`.
4. L0 `rules/layout.md` actualizado com **estado
   temporário** explicitamente documentado.
5. 4 tests E2E novos passam.
6. Tests existentes não regridem (paridade observable).
7. Verificações `.E` passam (14/14).
8. Nota DEBT M5-residual actualizada (3 → 2).
9. Relatório consolidado P194 (9 secções com §5
   dedicada a estado temporário) escrito.
10. Output observable em produção **inalterado** —
    sub-store vazio; fallback funcional.
11. Walks legacy E2/E4 **NÃO modificados**.

---

## O que pode sair errado

- **Site C4 mudou** entre P194A e P194B (improvável):
  cláusula gate trivial — ajustar referência em `.B`.
- **Test 1 (introspector path) falha** porque sub-store
  P193B `insert` não está acessível para testes:
  cláusula gate trivial — usar API pública (P193B
  decidiu `pub(crate)` para insert; tests no mesmo
  crate acedem).
- **Test 2 (fallback legacy) falha** porque legacy não
  popula `resolved_labels` para Heading auto-toc nos
  testes: cláusula gate substancial — investigar walk
  arms E2 (Heading auto-toc) ou usar Labelled explicit.
- **Test 4 (`@missing` literal) falha** porque o formato
  exacto do fallback difere: cláusula gate trivial —
  ajustar asserção.
- **Tests existentes regridem**: indica que migração
  altera output observable. Investigar — pode ser que
  Introspector retorna valor diferente do legacy.
  Cláusula gate substancial. **Risco baixo** —
  sub-store vazio em produção; fallback chamado
  consistentemente.
- **Snapshot tests divergem**: improvável (output
  preservado por construção). Se acontecer, investigar.
- **Linter divergência V13/V14**: cláusula gate trivial
  — `--fix-hashes`.

---

## Notas operacionais

- **Tamanho**: S agregado. ~10 LOC consumer + comentário
  inline + ~120 LOC tests + edits L0 + relatório
  consolidado.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **Padrão replicado**: P184D Figure / P187B C1 / P188B
  C2 substitution-with-fallback + adaptação para Opção
  C (`Option<&str>` propagado).
- **Cláusula gate trivial**: aplicável a forma exacta da
  expressão, localização do site, recálculo de hashes.
- **Cláusula gate substancial**: aplicável apenas se
  test 2 (fallback legacy) falhar (indica que walks
  legacy não populam `state.resolved_labels` no setup
  do teste como esperado) ou se tests existentes
  regridirem.
- **Test 2 é caso central** — empiricamente valida que
  cenário de produção real é coberto pelo fallback
  legacy. Replica padrão P188B `.D.2`.
- **Estado temporário documentado em 2 sítios** (não 4
  como P188B): comentário inline + secção §5 do
  consolidado. Mais leve que P188B porque o estado é
  resolvido em P195+, não permanente.
- **Próximo passo após P194B**: **P195** — migrar walk
  arm Labelled. Magnitude S–M (depende de decisão
  arquitectural sobre payload Labelled — pode exigir
  nova variant `ElementPayload::Labelled` similar a P186
  Equation, ou pode aproveitar mecanismo P171
  StateUpdate). Cadeia E2 + E4 fecham juntos.

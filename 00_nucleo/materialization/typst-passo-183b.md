# Passo P183B — C1 heading prefix via `formatted_counter`

Primeiro passo de implementação P183 (após P183A diagnóstico).
Magnitude **trivial (S)**.

Migra consumer C1 (`Layouter::layout_content` arm
`Content::Heading` em `01_core/src/rules/layout/mod.rs:310`)
de `self.counter.format_hierarchical("heading")` legacy para
`self.introspector.formatted_counter("heading")` com fallback
legacy. Padrão substitution-with-fallback P168/P181G/P182D
replicado.

`formatted_counter` existe no trait `Introspector` desde P170
— sem método trait novo. Sem sub-store novo. Sem L0 novo.

Após P183B:
- Consumer heading prefix consulta Introspector primeiro;
  fallback legacy se `formatted_counter("heading")` retorna
  `None`.
- Walk arm canonical, write paralelo, copy-sites legacy
  intocados (M6 elimina).
- Output observable em produção inalterado — fallback
  preserva paridade.

**Pré-condição**: P183A concluído. Tests workspace 1.756
verdes; zero violations. 6 cláusulas P183A fixadas:
substitution-with-fallback; Opção 3 fecho M4; C1 reutiliza
`formatted_counter` existente; ordem triviais primeiro.

**Restrições**:
- **Não** modificar walk arm canonical legacy.
- **Não** modificar write paralelo `layout/counters.rs`.
- **Não** modificar copy-sites em `mod.rs:1414, 1442`.
- **Não** modificar trait `Introspector` (sem método novo).
- **Não** modificar `CounterRegistry` (`formatted_counter`
  já existe).
- **Não** migrar outros consumers (C2-C5 ficam para P183C-E).
- API pública preservada.
- Output observable em produção inalterado — fallback
  garante paridade.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar consumer C1 actual:
   - `01_core/src/rules/layout/mod.rs:310` (per P183A §2).
   - Localizar leitura: padrão esperado
     `self.counter.format_hierarchical("heading")` ou
     similar.
   - Identificar contexto exacto (arm `Content::Heading`,
     função/método, escopo das variáveis locais).
   - Confirmar que está no **mesmo arm** ou **arm
     vizinho** ao consumer P182D heading-arm
     (`mod.rs:301`). P183A §2 listou os 2 read-sites em
     linhas próximas.

2. Confirmar acesso a `self.introspector`:
   - P181G/P182D estabeleceram acesso. P183A §3 confirmou
     `formatted_counter` existe no trait.
   - Confirmar empiricamente assinatura:
     `formatted_counter(&self, key: &str) -> Option<String>`
     (per P170).

3. Confirmar trait import:
   - P181G estabeleceu `use crate::entities::introspector::Introspector;`
     local em arm. P182D replicou. P183B replica também.

4. Confirmar `format_hierarchical` legacy:
   - `01_core/src/entities/counter_state_legacy.rs` (ou
     similar).
   - Assinatura: `format_hierarchical(&self, key: &str) -> String`
     ou `Option<String>` (verificar empiricamente).
   - Se retorna `String` (não-Option): fallback é
     directo. Se `Option<String>`: fallback usa
     `or_else` ou `unwrap_or_else`.

5. Confirmar L0 actual `rules/layout.md` (ou nome real):
   - Localizar entrada que documenta heading-arm.
   - Verificar se já cobre P182D (deveria).
   - Adicionar entrada para P183B (heading prefix path).

Output: tabela com item + estado confirmado / linha
actual / observação.

**Critério de saída e gate de decisão**:
- Se `formatted_counter` tem assinatura diferente de
  `Option<String>`: cláusula gate trivial — adaptar
  fallback.
- Se `format_hierarchical` legacy tem comportamento
  diferente do esperado (e.g. retorna string vazia em vez
  de `None` para chave ausente): cláusula gate trivial —
  adaptar lógica de fallback.
- Se trait import já está top-level no file (P181G/P182D
  consolidados): trivial — não duplicar import.
- Senão prosseguir.

### .B Actualizar L0 `rules/layout.md`

1. Adicionar entrada para C1 migration:
   - Heading prefix consultado via Introspector
     `formatted_counter("heading")` primeiro.
   - Fallback legacy `format_hierarchical("heading")`
     activo durante janela compat M6.
   - Justificação: padrão P168/P181G/P182D
     (substitution-with-fallback).

2. Hash em branco aguarda recálculo manual após
   confirmação humana.

**Critério de saída**:
- L0 contém entrada para C1.
- Coerente com entrada P182D (heading-arm
  `is_numbering_active`).

### .C Migrar consumer C1 heading prefix

1. Em `01_core/src/rules/layout/mod.rs:310`:
   - Substituir leitura legacy por padrão
     substitution-with-fallback.
   - Forma exacta fica para Claude Code conforme
     convenção do projecto:
     - Variável intermédia (`let prefix = ...`) vs inline.
     - `or_else(|| self.counter.format_hierarchical(...))`
       vs `match` explícito.
     - `Option<String>` propagado vs `unwrap_or_else`.
   - Convenção sugerida: replica P181G/P182D (variável
     intermédia + chamada `||` ou `or_else`).

2. Confirmar trait import local se necessário (replica
   P181G/P182D padrão).

3. Confirmar cabeçalho de linhagem `@prompt-hash`
   actualiza após edit do L0.

**Critério de saída**:
- `cargo check --workspace` passa.
- Output observable inalterado em tests existentes
  (Introspector retorna real ou `None`; fallback
  compensa caso `None`).
- Linter passa.

### .D Tests unitários ou E2E

1. Em `01_core/src/rules/layout/tests.rs`, submódulo
   `p183b_heading_prefix` (irmão de `p182d_heading_numbering`
   e `p182e_e2e_heading_numbering`):

2. Tests sugeridos (3 cenários):
   - **Test A** — `pipeline_completo_heading_prefix_via_introspector`:
     - Documento típico (`SetHeadingNumbering(true)` + 3
       headings em nesting [1, 2, 1]).
     - Pipeline `walk → from_tags → layout_with_introspector`.
     - Confirmar prefixos `"1."`, `"1.1"`, `"2."` no
       `plain_text`.
     - Validação intermédia: `intr.formatted_counter("heading")`
       retorna `Some(...)` antes de chamar layout.
   - **Test B** — `heading_prefix_via_fallback_legacy`:
     - Introspector vazio (`TagIntrospector::empty()`);
       legacy state pré-populado com contadores.
     - `layout_with_introspector` cai em fallback;
       prefixos correctos.
   - **Test C** — `paridade_heading_prefix_legacy_vs_migrated`:
     - Documento típico processado em ambos paths
       (`layout()` legacy + `layout_with_introspector`
       directo).
     - `plain_text` idêntico.

3. Tests existentes não regridem.

**Critério de saída**:
- 3 tests novos passam.
- Tests existentes não regridem.

### .E Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P183A
   baseline (1.756): +3.
3. `crystalline-lint .` zero violations.
4. Consumer C1 (`mod.rs:310`) consulta
   `self.introspector.formatted_counter("heading")`
   primeiro; fallback legacy.
5. Walk arm canonical legacy **NÃO modificado**.
6. Write paralelo legacy **NÃO modificado**.
7. Copy-sites legacy **NÃO modificados**.
8. Trait `Introspector` **NÃO modificado** (`formatted_counter`
   já existia desde P170).
9. Snapshot tests ADR-0033 verdes (output observable
   inalterado).
10. Linter passa final.

### .F Encerramento

Escrever
`00_nucleo/materialization/typst-passo-183b-relatorio.md`
com:

- Resumo: C1 heading prefix migrado; substitution-with-fallback
  padrão; sem método trait novo; sem L0 novo significativo.
- Confirmação `.E` (10 verificações).
- Δ tests vs baseline P183A (esperado +3).
- Hashes finais de L0s modificados (`rules/layout.md`).
- Decisões de execução notáveis (se houver).
- Estado actual:
  - P183 série: A ✅ B ✅ | C-F pendentes.
  - M9: 11/11 (inalterado — P182 fechou).
  - **M5/M4 progresso**: 5+1 = 6 read-sites migrados
    (P168 + P181G ×2 + P182D ×2 + P183B ×1 = 6/12).
    C2-C5 pendentes.
  - 36 passos executados.
- Pendências cumulativas: inalteradas (legacy continua;
  fallback `||` em C1 elimina-se em M6).
- Próximo passo: P183C (C2 equation counter via
  `flat_counter` — método trait novo).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. L0 `rules/layout.md` actualizado.
3. Consumer C1 migrado.
4. 3 tests novos passam.
5. Tests existentes não regridem.
6. Verificações `.E` passam (10/10).
7. Relatório `.F` escrito.
8. Output observable em produção inalterado.

---

## O que pode sair errado

- **`formatted_counter` retorna assinatura diferente**:
  cláusula gate trivial — adaptar.
- **`format_hierarchical` legacy retorna `String` vazia
  em vez de `None`** para chave ausente: cláusula gate
  trivial — adaptar lógica de fallback (não há `None`
  para `or_else` accionar).
- **Consumer C1 lê outros fields além de
  `format_hierarchical`** (e.g. lê também
  `numbering_pattern` para formatar): cláusula gate
  trivial — migrar apenas `format_hierarchical`; resto
  legacy.
- **Tests existentes regridem**: não esperado se fallback
  está correcto. Se regridir, fallback tem bug —
  investigar antes de prosseguir.
- **Snapshot tests divergem**: indica que Introspector
  retorna prefixo diferente do legacy. Investigar —
  divergência genuína bloqueia P183B até resolução.
  Causa provável (em re-update análoga a P182E §5.2):
  `formatted_counter` no Introspector pode ter semântica
  diferente do `format_hierarchical` legacy mutável
  durante walk. Cláusula gate substancial.
- **`formatted_counter` em Introspector ainda não está
  populado para a chave "heading"** (depende de
  `from_tags` arm para `Content::Heading`): se sim,
  fallback é o caminho activo até esse arm ser também
  migrado. Não bloqueia P183B mas ajusta expectativa
  sobre quando o path Introspector está activo.
- **Linter divergência V13/V14**: cláusula gate trivial
  — `--fix-hashes`.

---

## Notas operacionais

- **Tamanho**: trivial (S). ~30-50 LOC (1 edit inline + 3
  tests + edit L0).
- **Sem dependências externas novas**.
- **Sem método trait novo** (replica P170 existente).
- **Sem sub-store novo**.
- **Pré-condição P183C**: este passo concluído.
- **Padrão replicado**: P168 figure-ref + P181G cite-arm
  + P182D heading numbering (substitution-with-fallback).
- **Cláusula gate trivial**: aplicável a forma exacta da
  expressão, assinatura legacy, semântica de `None` vs
  string vazia.
- **Cláusula gate substancial**: aplicável apenas se
  snapshot tests divergirem (indica que `formatted_counter`
  Introspector tem semântica diferente do legacy
  mutável).
- **Pendência paralela P182E §5.2** aplicável: se
  Introspector usa `state_final_value` ou similar para
  popular `formatted_counter`, casos de re-update podem
  divergir. **Não bloqueia P183B** porque fallback `||`
  garante paridade observável; mas confirma que M6
  cleanup vai precisar de location-aware Introspector
  para C1 também (não só `numbering_active`).

# Passo P184D — Migrar consumer C3 (figure auto-number per kind)

Terceiro passo de implementação P184 (após P184A diagnóstico,
P184B refinamento arm, P184C trait method + helper).
Magnitude **S**.

Migra consumer C3 em `01_core/src/rules/layout/mod.rs:435–439`
de leitura legacy directa
(`state.counter.figure_numbers.get(kind_key).and_then(|v|
v.get(idx)).copied().unwrap_or(idx + 1)`) para
`Introspector::figure_number_at_index(kind_key, idx)` com
fallback legacy. Padrão substitution-with-fallback
P168/P181G/P182D replicado.

Após P184D:
- Consumer C3 consulta Introspector primeiro; fallback
  legacy se `figure_number_at_index` retorna `None`.
- Walk arm canonical, write-sites legacy, copy-sites
  intocados (M6 elimina).
- Output observable em produção inalterado — fallback
  preserva paridade. Achado P184A §3.6 / P184B §1
  ("dead code em produção" — `figure_numbers` legacy nunca
  copiado ao Layouter): fallback legacy retorna sempre
  `None` em produção real; `unwrap_or(idx + 1)` continua
  a ser o caminho activo. Migração é estrutural, não
  funcional, mas Introspector agora **tem** dados para a
  chave `figure:{kind}` (P184B+P184C populados), pelo que
  `figure_number_at_index` retorna `Some(N)` em produção
  e o Introspector path fica activo.

**Pré-condição**: P184C concluído. Tests workspace 1.764
verdes; zero violations. Trait `Introspector` 16 métodos
(`figure_number_at_index` adicionado); `CounterRegistry`
expõe `value_at_index`.

**Restrições**:
- **Não** modificar walk arm Figure em `introspect.rs`.
- **Não** modificar `from_tags` arm Figure (P184B
  fechou).
- **Não** modificar trait `Introspector` (P184C fechou).
- **Não** modificar `CounterRegistry` (P184C fechou).
- **Não** remover fallback (M6).
- API pública preservada.
- Output observable em produção inalterado — fallback
  garante paridade mesmo no path Introspector ainda não
  preferido.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar consumer C3 actual:
   - `01_core/src/rules/layout/mod.rs:435–439` (per
     P183A §2 e P184A §3.5).
   - Localizar leitura: padrão exacto
     `state.counter.figure_numbers.get(kind_key).and_then(|v|
     v.get(idx)).copied().unwrap_or(idx + 1)`.
   - Identificar contexto (arm `Content::Figure` em
     Layouter, função/método, escopo das variáveis
     `kind_key` e `idx`).
   - Confirmar acesso a `self.introspector` no escopo
     (P181G/P182D estabeleceram).

2. Confirmar variáveis locais:
   - `kind_key`: já é `&str` resolvido com
     `.unwrap_or("image")` (per P184A §3.5).
   - `idx`: `usize`, posição da figure no kind
     correspondente.
   - Se `kind_key` é construído inline (não variável
     local): verificar se Introspector path pode usar
     directamente.

3. Confirmar trait import:
   - P181G/P182D estabeleceram padrão `use
     crate::entities::introspector::Introspector;`
     local em arm.
   - Confirmar empiricamente se já existe import
     top-level no file (consolidação anterior pode ter
     ocorrido).

4. Confirmar L0 actual `rules/layout.md`:
   - Localizar entrada que documenta arm `Content::Figure`.
   - Identificar onde adicionar nota sobre Introspector
     path (se aplicável).

5. Confirmar tests existentes que cobrem o consumer:
   - `grep -rn "figure_numbers\|figure_number_at_index"
     01_core/src/rules/layout/`.
   - Se tests existentes usam apenas state legacy:
     continuam a passar (fallback cobre ou Introspector
     path retorna mesmo valor).
   - Se tests usam pipeline completo (walk + Introspector
     + Layouter): podem ganhar paridade automática.

Output: tabela com item + estado confirmado / linha
actual / observação.

**Critério de saída e gate de decisão**:
- Se `kind_key` é construído de forma diferente do
  esperado: cláusula gate trivial — adaptar.
- Se `idx` tem semântica diferente entre legacy e
  Introspector (por exemplo, idx 0-indexed no Introspector
  vs 1-indexed no legacy): cláusula gate substancial —
  recuar e investigar.
- Senão prosseguir.

### .B Actualizar L0 `rules/layout.md` (se necessário)

1. Em L0 do Layouter (ou ficheiro equivalente):
   - Documentar que arm `Content::Figure` consulta
     Introspector com fallback legacy.
   - Justificação: padrão P168/P181G/P182D
     (substitution-with-fallback).
   - Cross-reference: P184B (arm `from_tags` Figure)
     + P184C (método trait).

2. Se entrada P182D (heading-arm) já existe em L0,
   adicionar entrada paralela para figure-arm.

3. Hash em branco aguarda recálculo manual.

**Critério de saída**:
- L0 contém entrada para C3 migration (se aplicável).
- Coerente com entrada P182D.

### .C Migrar consumer C3

1. Em `01_core/src/rules/layout/mod.rs:435–439`:
   - Substituir leitura legacy por padrão
     substitution-with-fallback:
     - Consultar Introspector primeiro:
       `figure_number_at_index(kind_key, idx)`.
     - Fallback legacy via `or_else` se Introspector
       retorna `None`.
     - `unwrap_or(idx + 1)` final preserva semântica
       actual (defensivo + dead code path).
   - Forma exacta fica para Claude Code conforme
     convenção do projecto. P184C §8 sugeriu forma
     concreta; usar como referência.

2. Confirmar trait import local se necessário (replica
   P181G/P182D padrão).

3. Confirmar cabeçalho `@prompt-hash` actualiza após edit
   do L0 (se aplicável).

**Critério de saída**:
- `cargo check --workspace` passa.
- Output observable inalterado em tests existentes.
- Linter passa.

### .D Tests unitários ou E2E

Tests E2E completos ficam para P184E (passo dedicado).
Em P184D, apenas confirmar que tests existentes não
regridem.

1. `cargo test --workspace --lib` antes do passo:
   1.764 verdes (baseline P184C).

2. `cargo test --workspace --lib` após `.C`:
   - Se Δ = 0: sem regressão (esperado).
   - Se Δ < 0: regressão. Investigar.

3. Tests específicos a verificar (não modificar):
   - `from_tags::tests::figuras_numeradas_recebem_numeros_sequenciais`
     ou similar (verificar empiricamente).
   - Tests de Layouter sobre figures.

**Critério de saída**:
- Δ tests = 0.
- Tests existentes não regridem.

### .E Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P184C
   baseline (1.764): **0** (sem tests novos em P184D).
3. `crystalline-lint .` zero violations.
4. Consumer C3 (`mod.rs:435–439`) consulta
   `self.introspector.figure_number_at_index(kind_key,
   idx)` primeiro; fallback legacy.
5. Walk arm canonical legacy **NÃO modificado**.
6. Write paralelo legacy **NÃO modificado**.
7. Copy-sites legacy **NÃO modificados**.
8. Trait `Introspector` **NÃO modificado** (P184C fechou).
9. `CounterRegistry` **NÃO modificado** (P184C fechou).
10. Snapshot tests ADR-0033 verdes (output observable
    inalterado — fallback preserva paridade; Introspector
    populado via P184B+C retorna mesmo valor).
11. Linter passa final.

### .F Encerramento

Escrever
`00_nucleo/materialization/typst-passo-184d-relatorio.md`
com:

- Resumo: consumer C3 migrado; substitution-with-fallback;
  Introspector path activo em produção (P184B+C populou
  dados); achado P184A §3.6 ("dead code em produção"
  ratificado pela vez n) registado.
- Confirmação `.E` (11 verificações).
- Δ tests vs baseline P184C (esperado 0).
- Hashes finais de L0s modificados (se aplicável —
  `rules/layout.md`).
- Decisões de execução notáveis.
- Estado actual:
  - P184 série: A ✅ B ✅ C ✅ D ✅ | E-F pendentes.
  - **C3 desbloqueado**: eixos 1 e 2 atendidos; consumer
    migrado.
  - **M5/M4 progresso**: 5+1 = 6 read-sites migrados
    (P168 + P181G ×2 + P182D ×2 + P184D ×1 = 6/12).
    C1 e C2 continuam bloqueados (esperam P185+).
  - 41 passos executados.
- Pendências cumulativas: inalteradas (legacy continua;
  fallback `||` em C3 elimina-se em M6).
- Próximo passo: P184E (tests E2E paridade C3).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. L0 `rules/layout.md` actualizado (se aplicável).
3. Consumer C3 migrado em `mod.rs:435–439`.
4. Tests existentes não regridem (Δ 0).
5. Verificações `.E` passam (11/11).
6. Relatório `.F` escrito.
7. Output observable em produção inalterado.

---

## O que pode sair errado

- **`figure_number_at_index` retorna assinatura diferente
  do esperado**: improvável (P184C fechou); cláusula
  gate trivial.
- **`kind_key` tem semântica diferente**: cláusula gate
  trivial — adaptar.
- **Idx 0-indexed vs 1-indexed entre legacy e
  Introspector**: cláusula gate substancial. Verificar
  empiricamente em `.A.2` antes de migrar. P184B/C usaram
  idx 0-indexed (per `value_at_index(key, idx)` semântica
  Vec); legacy usa idx do Layouter (provavelmente 0-indexed
  também, mas verificar). Se 1-indexed, ajustar.
- **Tests existentes regridem**: indica que Introspector
  retorna valor diferente do legacy. Causas possíveis:
  - Introspector path activo retorna `Some(N)` enquanto
    legacy retornava `None` (caminho dead code) →
    `unwrap_or(idx + 1)` deixa de disparar. Se
    Introspector path retorna mesmo `N` que `idx + 1`,
    sem regressão. Se diferente, regressão.
  - Investigar test específico antes de prosseguir.
- **Snapshot tests divergem**: similar ao acima. Se
  Introspector populado correctamente em P184B/C, Δ deve
  ser zero. Se diverge, lacuna em P184B ou P184C.
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: S puro. ~5 LOC consumer + edits L0 (~5
  linhas) se aplicável.
- **Sem dependências externas novas**.
- **Sem método trait novo** (P184C fechou).
- **Sem sub-store novo**.
- **Pré-condição P184E**: este passo concluído.
- **Padrão replicado**: P168 figure-ref + P181G cite-arm
  + P182D heading numbering / equation
  (substitution-with-fallback).
- **Cláusula gate trivial**: aplicável a forma exacta da
  expressão, assinatura, semântica de idx.
- **Cláusula gate substancial**: aplicável apenas se idx
  diverge entre legacy e Introspector (improvável).
- **Achado P184A §3.6 / P184B §1 — "dead code em
  produção"**: o legacy `figure_numbers` nunca chega ao
  Layouter (copy-sites não copiam). Em produção,
  `state.counter.figure_numbers.get(kind_key)` retornava
  sempre `None`, e `unwrap_or(idx + 1)` era o caminho
  real. Após P184B+C, Introspector tem dados — path
  Introspector retorna `Some(N)`. **Output muda**:
  números retornados passam a vir do Introspector real,
  não do `idx + 1` heurístico. Para ser observable, é
  necessário que `figure_number_at_index` retorne mesmo
  valor que `idx + 1` em casos típicos. Verificar
  empiricamente: figure 0 do kind X tem número 1; figure
  1 tem número 2; etc. Se sim, output idêntico. Se
  Introspector retorna outros valores (offset diferente,
  sequência diferente), há divergência observable que
  exige investigação.
- **Não bloqueia P184D** se valores forem idênticos
  (esperado). Mas é mais que migração estrutural — é
  primeiro consumer onde Introspector populado **é** o
  caminho activo, não redundância.

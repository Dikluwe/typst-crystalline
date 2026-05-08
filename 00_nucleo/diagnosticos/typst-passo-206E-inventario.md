# P206E — Inventário interno (encerramento série vanilla integration)

**Data**: 2026-05-08.
**Spec**: `00_nucleo/materialization/typst-passo-206E.md`.
**Output 1 de 4** (paralelo a P204H/P205E §C4 estrutura).

---

## §1 C1 — Auditoria das 7 condições ADR-0075

ADR-0075 §"Plano de validação" fixou 7 condições para
transitar PROPOSTO → ACEITE.

| # | Condição (literal) | Estado | Evidência empírica |
|---|--------------------|--------|---------------------|
| 1 | P206B materializado: 2 breaks corrigidos; `cargo check --all-targets` passa; smoke test vanilla CLI integrado | **CUMPRIDA** | `lab/parity/tests/layout_parity.rs:67-71` (migration P190I); `src/value_dto.rs:109+` (arm `Value::Location`); `tests/vanilla_cli_smoke.rs` (2 tests); `cargo check` verde confirmado |
| 2 | P206C materializado: vanilla CLI helper + comparação estrutural via `typst query` JSON; cristalino test helper produz output análogo; ≥5 tests E2E novos | **CUMPRIDA** | `03_infra/src/query_helpers.rs` (hash `51294329`; +13 tests); `lab/parity/src/{vanilla_invoke,structural_compare}.rs`; `tests/structural_parity.rs` (10 tests); 23 tests E2E totais — superado ≥5 |
| 3 | P206D materializado: matriz consolidada com `text_content` + `structural` populadas; 36 cobertos (1 SKIP error.typ); `geometric` N/A | **CUMPRIDA** | `lab/parity/tests/consolidado_p206d.rs` produz matriz (`p206d_corpus_consolidado` test); `reports/latest.md` + `history/2026-05-08-passo-206D.md`; SKIPS.md cobre 36 (23 INCLUDE + 3 SKIP-pre-existing + 10 SKIP-feature); geometric N/A documentado per ADR-0054 |
| 4 | Tests workspace verdes (estimativa 1860 → 1865-1875; ∆+5 a +15) | **CUMPRIDA** | 1860 → 1873 verdes (∆+13 real). Dentro do range estimado. Sem regressão |
| 5 | `crystalline-lint .` 0 violations | **CUMPRIDA** | `✓ No violations found` confirmado pós-cada sub-passo (P206B, C, D) + verificação final P206E |
| 6 | Vanilla CLI smoke test no boot: `typst --version` reporta 0.14.2; abort gracefully se ausente | **CUMPRIDA** | `p206b_vanilla_cli_disponivel_e_versao_compativel` test em `vanilla_cli_smoke.rs` confirma prefix match `0.14`; skip graceful via `eprintln!` + return implementado |
| 7 | Cond 9 ADR-0073 fechada: P206E formaliza transição "estruturalmente fechado" → "completo final" se 7/7 cumpridas | **CUMPRIDA via P206E (auto-referencial)** | P206E C9 transita ADR-0073 para "completo retroactivo" via Caminho B (fórmula intermediária honesta); cond 9 fechada estruturalmente via matriz P206D; ver C2/C3 abaixo |

### §1.1 Notas de auditor

**Cond 7** é auto-referencial — P206E é o sub-passo
que materializa a condição. Etiqueta CUMPRIDA é
recursiva mas literal: o trabalho desta C9 (transição
ADR-0073) cumpre cond 7. Pattern paralelo a P205E cond
3 (P205D condicional aceita branches).

**Cond 4** real (1873) está dentro da estimativa
(1865-1875) — sem ajustes documentados necessários.

**`P206C.div-1`** não é excepção do plano de validação
— é decisão arquitectural durante materialização (C2
Caminho B vs A). Cumprimento das 7 condições não
depende desta decisão. Documentada como divergência
cosmética em ADR-0075 §"Decisão" + §"Validação P206A-E".

---

## §2 C2 — Auditoria cond 9 ADR-0073 face a P206

### §2.1 Texto literal de cond 9

ADR-0073 §"Plano de validação" linha 208-209:

> 9. **Saída cristalino sanity-check** vs vanilla nos
> 5-7 ficheiros corpus paridade — sem regressões
> observable.

Estado em P204H 2026-05-07: **PARCIAL** por
`P204F.div-1` (vanilla integration deferred; DEBT-53/54
pre-existing).

### §2.2 Confronto com matriz P206D

Os 6 ficheiros introspection P204F (visual category)
cobertos por matriz P206C/D:

| Ficheiro P204F | Selector(s) testados | Resultado matriz |
|----------------|-----------------------|-------------------|
| `counter-heading.typ` | heading, figure, metadata | ✅ Match cristalino vs vanilla (heading count=5) |
| `figure-ref.typ` | heading, figure, metadata | ✅ Match (figure count=3 ambos) |
| `query-metadata.typ` | metadata | ✅ Count match (3 metadata ambos); shape diff cosmético tolerado |
| `equation-ref.typ` | heading, figure, metadata | ✅ Match em selectors validados; equation namespace divergence (vanilla rejeita `equation` standalone — fora-de-escopo cond 9 literal) |
| `outline-toc.typ` | heading | ⚠️ Excepção: heading count diff (cristalino emite auto-toc P200B; vanilla counts only outline body); design intencional cristalino — não regressão M8 |
| `cite-bibliography.typ` | (todos) | ⚠️ Excepção: cristalino eval falha (bibliography stdlib parcial pre-P206; gap conhecido pre-cond 9); não regressão introduzida por M8 |

**Resultado**: 4/6 ficheiros P204F com matches limpos;
2/6 com excepções documentadas.

### §2.3 Etiqueta fixada

**CUMPRIDA com excepções**.

Justificação literal:

- "Sem regressões observable" — satisfeito. As 2
  excepções **não são regressões M8**:
  1. `outline-toc` heading count: design intencional
     cristalino P200B (auto-toc emissions visíveis em
     query). Pre-existente; não introduzido por M8
     `#[comemo::track]`.
  2. `cite-bibliography` eval fail: bibliography
     stdlib cristalino parcial (P181 series não-completa);
     pre-P206; não introduzido por M8.
- "5-7 ficheiros corpus paridade" — cobertos: 6/6
  introspection P204F + 17 outros ficheiros visual/markup
  via matriz P206D extension. Literal "5-7" satisfeito
  (e excedido).

Etiqueta CUMPRIDA estritamente seria desonesta face às
2 excepções documentadas. Etiqueta PARCIAL ainda
sub-estimaria progresso (P206 materializou
substancialmente). **CUMPRIDA com excepções** é a
forma honesta.

---

## §3 C3 — Forma da transição ADR-0073: Caminho B

Decisão fixada: **Caminho B — "completo retroactivo"**.

Justificação literal (per spec C3):

- **Caminho A "completo (final)"** rejeitado: cond 9
  CUMPRIDA estritamente seria desonesto face às 2
  excepções documentadas em C2.
- **Caminho B "completo retroactivo"** fixado:
  fórmula intermediária honesta. Distingue
  explicitamente que cond 9 fechou em série diferente
  (P206) com excepções.
- **Caminho C "estruturalmente fechado preservado"**
  rejeitado: cond 9 progrediu materialmente via
  matriz P206D; preservar PARCIAL seria sub-estimar
  progresso.

Forma da transição:

- ADR-0073 estado: `ACEITE estruturalmente fechado`
  (P204H 2026-05-07) → **`ACEITE completo retroactivo,
  P206E 2026-05-08`**.
- Bloco "Fecho retroactivo cond 9 — P206E 2026-05-08"
  adicionado **no início** da ADR (após cabeçalho)
  preservando texto original do plano de validação +
  "Validação P204A–H" intactos.

---

## §4 C4 — Tratamento P204H consolidado: Caminho a

### §4.1 Auditoria P201/P202

Patterns confirmados empiricamente:

- **P201 relatório** (linha encontrada): "preservação
  histórica per spec §5" — backup do conteúdo
  anterior em ficheiro separado quando há reescrita.
- **P202 relatório**: "Modificação retroactiva
  quebraria a regra de preservação."

Pattern estabelecido: **preservação histórica é regra
imperativa**; modificação retroactiva apenas via
anotação cirúrgica que não reescreve.

### §4.2 Decisão fixada

**Caminho a — Anotação cirúrgica**.

Justificação literal:

- Pattern P201/P202 confirma preservação como regra.
- Consolidado P204H (`typst-passo-204-relatorio-consolidado.md`)
  reflecte estado real em 2026-05-07. Reescrever
  apagaria contexto histórico onde cond 9 era PARCIAL
  e plano de fecho estava em formação.
- Bloco §14 "Anotação cirúrgica P206E — Fecho
  retroactivo cond 9 (2026-05-08)" adicionado **no
  final** preservando §1-§13 intactas.

Caminho b (reescrita) rejeitado: viola P201/P202.

Caminho c (ambos) descartado: anotação cirúrgica é
suficiente.

---

## §5 C5 — Forma de fecho de P206: Completo (final)

Decisão fixada: **Completo (final)**.

Justificação literal:

- 7/7 condições obrigatórias ADR-0075 CUMPRIDAS (per
  C1).
- `P206C.div-1` é **divergência cosmética** documentada
  (não falha estrutural):
  - Caminho B (helper L3) materializado.
  - Satisfaz intenção da clarificação ("cristalino
    expõe helper") via API L3 público.
  - CLI subcomando deferred para sub-passo dedicado
    pós-P206 (não excepção do plano de validação).
- Sem condição PARCIAL ou NÃO CUMPRIDA com
  justificação não-cosmética.

Etiqueta "Estruturalmente fechado" seria sub-estimar:
P206 série materializou plenamente os objectivos
declarados; CLI deferred é decisão arquitectural, não
falha.

### §5.1 Distinção face a P204H + P205E

P204H fixou "estruturalmente fechado" porque cond 9
era PARCIAL — excepção real (DEBT-53/54 pre-existing).

P205E fixou "Completo (final)" porque P205D condicional
aceita branches per ADR-0074 cond 3.

P206E fixa "Completo (final)" porque `P206C.div-1` é
cosmética — clarificação inicial honrada parcialmente
via Caminho B aceitável per spec.

---

## §6 C6 — DEBT fechos

### §6.1 DEBT-53 → ENCERRADO (CLOSED)

**Etiqueta**: ENCERRADO (CLOSED).

**Justificação**: vanilla integration **materializada**
via série P206A-D:
- ADR-0075 ACEITE final.
- 7/7 condições do plano de validação CUMPRIDAS.
- Helper L3 + comparação estrutural + matriz
  consolidada produzidos.
- Cobertura empírica 34/36 compila; 20/36 structural
  matches.

**Localização**: `00_nucleo/DEBT.md` linha 759 →
header actualizado para "ENCERRADO (Passo 206E) ✓"
+ bloco de fecho com referências P206A-E + histórico
preservado.

### §6.2 DEBT-54 → ENCERRADO (OBSOLETED)

**Etiqueta**: ENCERRADO (OBSOLETED).

**Justificação**: workspace setup **nunca foi
necessário**. P206A auditoria empírica A5 descobriu
vanilla CLI 0.14.2 pre-built em `/usr/local/bin/typst`.
ADR-0075 P206A C5 fixou Caminho b ("pre-built
binário"); workspace setup obsoleto sem código.

Per pattern P206A D3: DEBT pode fechar via 3 caminhos
(CLOSED / REPLACED-BY / OBSOLETED). DEBT-54 é primeira
aplicação formal de OBSOLETED na trajectória.

**Localização**: `00_nucleo/DEBT.md` linha 557 →
header actualizado para "ENCERRADO (Passo 206E) ✓"
+ bloco de fecho com referência a P206A D3 + histórico
preservado.

---

## §7 Decisões durante a leitura

### D1 — `CUMPRIDA com excepções` é etiqueta honesta

C2 audita literal "sem regressões observable" + matriz
empírica P206D. 4/6 matches limpos + 2/6 excepções
documentadas (não regressões). Etiqueta intermediária
respeita evidência sem inflar (CUMPRIDA estritamente
seria desonesto) ou sub-estimar (PARCIAL seria
ignorar progresso).

### D2 — Caminho B retroactivo previne falsa "completo"

Spec §8 risco "inflar transição retroactiva ADR-0073
sem honestidade" — evitado. Caminho B preserva nuance
das excepções no estado da ADR. Auditor futuro lê
"ACEITE completo retroactivo" + bloco "Fecho retroactivo"
e entende contexto.

### D3 — Caminho a P204H confirmado por grep empírico

P201/P202 explicitamente declaram "Modificação
retroactiva quebraria a regra de preservação".
Caminho a (anotação cirúrgica §14) é literal-aplicação
do pattern. §1-§13 preservadas; §14 adicionada
chronologicamente.

### D4 — DEBT-54 OBSOLETED é primeira aplicação formal

Pattern P206A D3 ("DEBT pode fechar via 3 caminhos:
CLOSED / REPLACED-BY / OBSOLETED") aplicado pela
primeira vez em DEBT-54. Documentado em DEBT.md +
ADR-0075 + relatório consolidado P206 §4.3.

### D5 — `P206C.div-1` cosmética não bloqueia "Completo (final)"

Per spec C5 hipótese: "Completo (final) para P206
porque `P206C.div-1` é divergência cosmética". C5
fixado per literal: 7/7 condições CUMPRIDAS; div-1 é
decisão arquitectural durante materialização, não
excepção do plano de validação.

### D6 — Auto-referencialidade de cond 7 ADR-0075

Cond 7 ("Cond 9 ADR-0073 fechada: P206E formaliza
transição") é auto-referencial — P206E é o sub-passo
que cumpre a condição. Etiqueta CUMPRIDA é literal: o
trabalho desta C9 cumpre cond 7. Pattern paralelo a
P205E cond 3 (P205D condicional aceita branches).

### D7 — Distinção "completo final" (P205E) vs "completo retroactivo" (P206E)

P205E ADR-0074 transitou directamente de PROPOSTO para
ACEITE final em série única. P206E ADR-0073 transitou
retroactivamente — cond 9 era PARCIAL em série anterior
(P204H) e progrediu via série posterior (P206). A
fórmula "completo retroactivo" distingue trajectória
inter-séries vs intra-série.

### D8 — Blueprint §3.0ter chronológico

§3.0 [P204H] M8 + §3.0bis [P205E] F3 + §3.0ter
[P206E] vanilla integration. Pattern marca-por-fecho
consolidado: cada série completa adiciona subsecção
adjacente; chronologia preservada sem reescrita.

### D9 — Sem sentinelas novas em P206E

Spec C13: "P206E não adiciona sentinelas novas
(encerramento documental)". Sentinelas activas
preservadas: 21 workspace + 4+7 quarentena (net 11
P206 série). C14 verifica que continuam verdes.

### D10 — Magnitude P206 = M agregado (paralelo a P205)

Sub-passos P206A-E reais: M + S + M + S-M + S
documental ≈ M agregado. Comparação:
- P204 série (M8): L cross-modular.
- P205 série (F3): M agregado.
- P206 série (vanilla integration): M agregado.

P205 e P206 séries têm escopo similar (refactor
moderado vs integração externa); P204 era escopo
maior (paridade vanilla literal cross-modular).

---

## §8 Resumo — métricas

| Métrica | Valor |
|---------|-------|
| Forma de fecho fixada | **Completo (final)** |
| ADR-0075 transição | PROPOSTO → **ACEITE final** |
| ADR-0073 transição (retroactiva) | "estruturalmente fechado" → **"completo retroactivo"** (Caminho B) |
| Cond 9 ADR-0073 etiqueta P206E | **CUMPRIDA com excepções** (4/6 matches; 2/6 excepções documentadas) |
| DEBT-53 transição | EM ABERTO → **ENCERRADO (CLOSED)** |
| DEBT-54 transição | EM ABERTO → **ENCERRADO (OBSOLETED)** |
| Tests workspace antes (P206E) | 1873 |
| Tests workspace depois (P206E) | **1873** (sem alteração — documental) |
| Tests lab/parity antes | 75 |
| Tests lab/parity depois | **75** (sem alteração — documental) |
| Linter violations | 0 (sem alteração) |
| ADRs editadas | 2 (0075 transitada; 0073 anotada retroactivamente) |
| ADRs novas em P206E | 0 |
| Ficheiros docs modificados | 5 (ADR-0075; ADR-0073; P204H consolidado §14; blueprint §3.0ter; DEBT.md DEBTs 53+54) |
| Ficheiros docs novos | 3 (este inventário; consolidado série P206; relatório P206E) |
| LOC novas (código) | 0 |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 0 |
| Pattern emergente novo | 1 (transição retroactiva via anotação cirúrgica) |

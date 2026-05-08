# Passo 206C — Cristalino CLI/test helper de query + comparação estrutural

**Série**: 206 (sub-passo `C` = vanilla CLI runtime +
comparação estrutural após P206B foundational).
**Tipo**: implementação cross-modular (helper cristalino
+ comparação estrutural).
**Magnitude planeada**: **M** (com ressalva L se custo
de "novo CLI cristalino" for desproporcional —
auditoria empírica decide).
**Pré-condição**: P206B concluído; harness `lab/parity/`
compila com `cargo check --all-targets`; vanilla CLI
0.14.x detectável + subcomando `query` confirmado;
`parity-runner` smoke preservado funcional; tests
workspace cristalino 1860 verdes; 0 violations; ADR-0075
PROPOSTO em vigor com §P206B anotado ✅ MATERIALIZADO.
**Output**: 3 ficheiros (inventário + relatório +
alterações de código).

---

## §1 Propósito

Materializar comparação estrutural cristalino vs vanilla
via JSON shape compatível com `typst query`:

- **Cristalino expõe novo CLI/test helper** que aceita
  selector e retorna JSON compatível com vanilla
  (decisão fixada pela clarificação inicial).
- **Helper de invocação vanilla** em `lab/parity/src/`
  que executa `typst query` e captura output JSON.
- **Helper de comparação JSON** que valida equivalência
  semântica (não literal — JSON pode diferir em
  formatting).
- **Tests cobrindo todos os 36 ficheiros aplicáveis**
  (per P206A C2 + escopo fixado pela clarificação
  inicial).

P206C respeita o padrão: começa com inventário empírico
antes de qualquer alteração.

---

## §2 Tensão consciente entre os dois inputs

A clarificação inicial fixou:

- **Forma do helper**: novo CLI/test helper cristalino
  (não helper interno em `lab/parity/`).
- **Escopo**: 36 ficheiros corpus aplicáveis.

A combinação inflaciona magnitude:

- "Novo CLI cristalino" é trabalho mais pesado que
  "test helper interno". Cristalino actualmente não
  expõe CLI de query (per P204A A3 — `position_of` e
  outros métodos de `Introspector` têm zero consumers
  em produção; CLI vanilla `typst query` não tem
  equivalente cristalino).
- "36 ficheiros" multiplica esforço de teste.
- P206A C10 fixou magnitude M agregada para a série
  inteira (~3.5–5h). P206C sozinho pode aproximar-se ou
  exceder esse orçamento se ambas as decisões inflarem.

P206C resolve assim:

- C1 audita empíricamente o custo real de "novo CLI
  cristalino" (qual o ponto de extensão? quais os
  consumers? que formato JSON adoptar?).
- C2 fixa forma concreta com base em C1.
- **Caso C1 mostrar custo XL para "novo CLI"**, P206C
  tem 2 caminhos possíveis (decididos por C3):
  - **Caminho R — Recuar** — registar `P206C.div-N`
    sugerindo que "novo CLI" fica para sub-passo
    dedicado pós-P206; P206C usa caminho test helper
    interno.
  - **Caminho A — Aceitar** — manter "novo CLI"
    materializado, aceitar que P206C é magnitude L (não
    M), e ajustar P206A C10 retroactivamente.

A clarificação inicial fixou "novo CLI" — Caminho R só
é legítimo se evidência empírica mostrar inviabilidade
sem inflação retroactiva. Pré-fixação não é absoluta;
honestidade empírica prevalece.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código, listar literalmente:

1. **Vanilla `typst query` CLI** — confirmar:
   - Output formato exacto (`typst query <doc.typ>
     '<heading>' --format json`).
   - Que selectors aceita (`<heading>`, `<figure>`,
     `<my-label>`, etc.).
   - Estrutura JSON dos elementos retornados (campos
     standard).
2. **Cristalino consumers de `Introspector`** —
   confirmar:
   - `Introspector::query` retorna `Vec<Location>` (per
     P204B A1).
   - Pipeline: `Selector` → `query` → `Vec<Location>` →
     consumer extrai `Position`/`Content` per location.
   - Não há serialização JSON existente.
3. **Cristalino CLI actual** — confirmar:
   - `04_wiring/` é binary-only com `main.rs` (per
     P204E inventário).
   - CLI actual aceita `<file.typ>` e produz PDF.
   - Não há subcomandos (sem `query`, sem `compile`
     explícito).
   - Adicionar subcomando exige refactor de `main.rs`?
4. **Tipos cristalinos para serialização** — confirmar:
   - `Selector` parsing existe? Cristalino aceita
     `<heading>` ou `<my-label>` em syntax `.typ`?
   - `Content` tem variants serializáveis? P204B
     adicionou Hash impl manual; serde opcional?
   - `Location` (P179) tem repr serializável? Hash
     manual via `to_bits()` (per P204D).
5. **Lab/parity actual** — confirmar:
   - `lab/parity/Cargo.toml` declara `serde_json`
     dependency? Se não, adicionar.
   - Pattern existente para chamar binário externo
     (`Command::new`)? Per P206B C1.5 — sem helper
     pre-existente.
6. **Custo estimado "novo CLI cristalino"** — análise:
   - Subcomando `query` adicionado a `04_wiring/`.
   - Parsing de selector (delegate a stdlib cristalino
     ou parsing simples em CLI).
   - Serialização JSON do output.
   - Magnitude estimada: S / M / L / XL.
7. **Custo estimado "test helper interno"** — análise
   contraste:
   - Helper em `lab/parity/src/cristalino_query.rs` que
     invoca trait `Introspector` directamente e
     serializa.
   - Sem refactor de `04_wiring/`.
   - Magnitude estimada: S / M.

Output: 7 sub-secções com etiqueta CONFIRMADO ou
**AJUSTE NECESSÁRIO**.

Se C1.6 mostrar que "novo CLI" é XL (ex: parsing de
selector exige novo stdlib + refactor de `04_wiring/`
substancial), C2/C3 endereçam.

### C2 — Forma do helper cristalino

Decisão fixada com base em C1.6 + C1.7:

- **Caminho A — novo subcomando CLI** —
  `04_wiring/src/main.rs` ganha subcomando `query`;
  `cristalino query <doc.typ> '<heading>' --format
  json` produz JSON compatível com `typst query`.
- **Caminho B — test helper em workspace cristalino**
  — função `pub fn query_to_json` em
  `03_infra/src/query_helpers.rs` ou similar; invocada
  de tests cristalinos e de lab/parity. Sem CLI
  exposto.
- **Caminho C — degradado a test helper interno em
  lab/parity** — só se Caminho A e B forem ambos XL;
  abandona "novo CLI cristalino" da clarificação
  inicial com fundamento empírico.

A clarificação inicial fixou Caminho A como preferido.
Caminho B é compromisso aceitável (cristalino expõe
helper, mas não como CLI). Caminho C contradiz
clarificação — só legítimo se evidência empírica
mostrar inflação XL.

C2 fixa **uma** alternativa.

### C3 — Resolução da tensão (se C2 != A)

Se C2 = A: tensão resolvida; prosseguir.

Se C2 = B: tensão parcialmente resolvida (helper
cristalino existe; CLI exposto fica para sub-passo
dedicado pós-P206). Documentar em `P206C.div-N` que
clarificação inicial foi parcialmente honrada.

Se C2 = C: tensão não resolvida; registar
`P206C.div-N` com fundamento empírico e solicitar
decisão ao humano antes de prosseguir.

C3 fixa decisão pós-C2.

### C4 — Helper de invocação vanilla

Edição literal em `lab/parity/src/vanilla_invoke.rs`
(novo ficheiro):

```text
pub fn run_typst_query(
    typ_path: &Path,
    selector: &str,
) -> Result<serde_json::Value, VanillaInvokeError>;
```

- Executa `typst query <typ_path> '<selector>' --format
  json` via `Command`.
- Captura stdout; parse via `serde_json::from_slice`.
- Skip graceful se vanilla CLI ausente — retorna
  `VanillaInvokeError::NotInstalled`.
- Timeout reasonable (5–10s) para evitar hang.

C4 fixa estrutura do helper.

### C5 — Helper de comparação JSON

Edição literal em
`lab/parity/src/structural_compare.rs` (novo ficheiro):

```text
pub fn compare_query_outputs(
    cristalino_json: &serde_json::Value,
    vanilla_json: &serde_json::Value,
) -> CompareResult;
```

`CompareResult`:
- `Match` — equivalência semântica.
- `Diff(details)` — divergência (lista campos diferentes).
- `Skip(reason)` — feature não comparável.

Comparação:
- Tolerante a ordem de campos.
- Tolerante a campos não-essenciais (ex: `pos.x` com
  precisão de ponto flutuante).
- Estrita em campos essenciais (label, kind, page).

Critérios de tolerância fixam-se com base em C1.1 (formato
vanilla). Decisão dentro de P206C.

### C6 — Cobertura dos 36 ficheiros

Per clarificação inicial: todos os 36 ficheiros
aplicáveis.

C6 produz tabela 36 linhas com etiqueta:
- **INCLUDE** — comparação cristalino vs vanilla
  executável.
- **SKIP-feature** — ficheiro usa feature não suportada
  por cristalino ou vanilla na versão actual (ex:
  `here-locate.typ` per P204F SKIP, se ainda
  aplicável).
- **SKIP-pre-existing** — ficheiros já com SKIP
  documentado em `lab/parity/SKIPS.md` (per P206A C7;
  ex: `markup/error.typ`).

Cada SKIP documentado com razão literal. Lista finalizada
em P206C runtime.

### C7 — Tests dedicados

Estrutura proposta:

- `lab/parity/tests/structural_parity.rs` (novo):
  - 1 test parameterizado sobre os 36 ficheiros (via
    macro ou loop).
  - Para cada ficheiro INCLUDE: invoca cristalino
    helper + vanilla CLI; compara JSON; assert Match.
  - Para cada ficheiro SKIP: skip silencioso com
    `eprintln!`.
  - Skip graceful global se vanilla CLI ausente (per
    P206B pattern).

Critério: tests verdes; 54 → 54+N onde N reflecte
INCLUDE count (variável; depende de C6).

### C8 — Compilação

```
cargo check --manifest-path lab/parity --all-targets
cargo build --workspace
```

Critério: ambos verdes. Workspace cristalino afectado se
C2 = A (subcomando novo); pode ser afectado se C2 = B
(helper em workspace).

### C9 — Tests workspace cristalino

```
cargo test --workspace 2>&1 | tail -10
```

Critério: 1860 mantém-se ou ajusta-se conforme C2.

Se C2 = A: pode haver tests novos para subcomando query
em `04_wiring/`.

Se C2 = B: tests novos em `03_infra/`.

Se C2 = C: 1860 invariante (tudo em `lab/parity/`).

### C10 — Tests `lab/parity`

```
cargo test --manifest-path lab/parity --all-targets
```

Critério: 54 + N tests verdes (N = INCLUDE count de
C6).

### C11 — Linter

```
crystalline-lint .
```

Critério: 0 violations.

Se C2 = A ou B: pode haver L0 prompts novos exigidos
(per Protocolo de Nucleação). `--fix-hashes` aplicado.

### C12 — Documentação ADR-0075

ADR-0075 mantém PROPOSTO. Anotação cirúrgica em §P206C
com `✅ MATERIALIZADO` + sumário (1–2 linhas).

### C13 — Critério de fecho de P206C

P206C concluído quando:

- C1 inventário completo (7 sub-secções).
- C2 forma do helper fixada com justificação.
- C3 tensão resolvida (ou divergência registada se C2
  ≠ A).
- C4 helper de invocação vanilla aplicado.
- C5 helper de comparação JSON aplicado.
- C6 tabela de cobertura 36 ficheiros.
- C7 tests parameterizados.
- C8 compilação verde.
- C9 tests workspace mantidos.
- C10 tests lab/parity verdes.
- C11 linter 0 violations.
- C12 ADR-0075 anotada.
- Inventário registado.
- Relatório escrito.

### C14 — Possível `P206C.div-N` sobre custo

Se C1.6 mostrar que "novo CLI cristalino" exige refactor
substancial de `04_wiring/` ou parsing de selector
não-trivial (ex: stdlib cristalino não suporta sintaxe
de selector vanilla):

- Registar `P206C.div-N` com estimativa empírica.
- C2 fixa Caminho B ou C com fundamento.
- Se Caminho C, solicitar decisão ao humano antes de
  prosseguir P206D.

A clarificação inicial fixou "novo CLI" — divergência
legítima exige fundamento empírico não-cosmético.

### C15 — Sem cláusulas condicionais

C1 produz dados. C2 fixa **uma** alternativa. C3
resolve tensão. C4–C12 executam decisões fixas.

A possibilidade de C2 = B ou C **não é ramo na spec** —
é resposta empírica honesta a evidência de C1.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-206C-inventario.md`.

Conteúdo:
- §1 C1 — inventário (7 sub-secções).
- §2 C2 — forma do helper fixada.
- §3 C3 — resolução da tensão.
- §4 C4–C5 — helpers literais.
- §5 C6 — tabela 36 ficheiros com etiquetas.
- §6 Decisões durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-206C-relatorio.md`.

Conteúdo:
- O que foi feito.
- Caminho escolhido (A / B / C).
- Tempo de execução.
- Métricas (tests pre/post; LOC delta; cobertura
  INCLUDE/SKIP).
- Decisões.
- Sugestão para próximo sub-passo (P206D).

### Ficheiro 3 — Alterações em código

Não é ficheiro discreto. Conjunto de:

- (Caminho A) `04_wiring/src/main.rs` ganha subcomando
  `query`. L0 prompt eventualmente actualizado.
- (Caminho B) `03_infra/src/query_helpers.rs` (novo).
  L0 prompt novo se necessário.
- (Caminho C) `lab/parity/src/cristalino_query.rs`
  (novo).
- `lab/parity/src/vanilla_invoke.rs` (novo).
- `lab/parity/src/structural_compare.rs` (novo).
- `lab/parity/Cargo.toml` (`serde_json` se não estiver
  declarado).
- `lab/parity/tests/structural_parity.rs` (novo).
- Anotação cirúrgica em ADR-0075.

---

## §5 Critério de progressão para P206D

P206C fechado quando C13 cumprido.

Em caso de divergência empírica relevante, registar em
`P206C.div-N` e:

- Resolver dentro de P206C (preferido).
- Recuar para P206A re-fixar C2/C5 (se obstrução for
  estrutural).
- Solicitar decisão ao humano se C14 aplicar (custo
  XL).

P206D só começa quando P206C fechado.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 3 outputs.
- Inventário empírico antes de implementação.
- Localização canónica.
- Sem inflação retórica.

---

## §7 Não-objectivos

P206C não:

- Cobre features ainda não suportadas em cristalino
  (ex: `here()`/`locate()` per P204F SKIP) — esses
  ficam SKIP per C6.
- Implementa `Introspector` métodos novos.
- Implementa stdlib expansion.
- Transita ADR-0075 para ACEITE (P206E).
- Transita ADR-0073 para "completo final" (P206E).
- Materializa expectations vanilla nas companions
  `.typ.toml` (decisão dentro de P206C; possivelmente
  P206D).
- Compila vanilla typst (pre-built CLI usado).
- Modifica trait `Introspector` ou impl
  `TagIntrospector`.
- Endereça pre-existing breaks adicionais não
  identificados em P206B (já confirmado exhaustivo).

---

## §8 Erro a não repetir

Da série P204+P205+P206A/B — pattern empírico:
inventário antes de decisão; honestidade sobre
divergências.

Risco específico de P206C: **inflar para honrar "novo
CLI cristalino" pré-fixado quando empírico mostra que
exige refactor substancial de `04_wiring/`**. C14
antecipa isto e legitima divergência.

Outro risco: **subestimar custo de "novo CLI" porque
parece trivial**. Adicionar subcomando exige:

- Refactor de `main.rs` para subcomando dispatch.
- Parsing de selector (delegate a stdlib ou ad-hoc).
- Serialização JSON (vanilla output formato).
- Tests para o novo subcomando.

Magnitude pode ir de S (se refactor trivial) a L (se
parsing de selector exige stdlib).

Hipótese mais provável: C2 = B (test helper em
`03_infra/`) é compromisso pragmático que satisfaz a
intenção da clarificação ("cristalino expõe helper")
sem custo XL de adicionar subcomando CLI completo.
Caminho B mantém ponto de extensão para Caminho A
futuro.

Mas é hipótese, não decisão. C2 fixa-se com base em
C1.6 + C1.7.

Outro risco específico: **escopo "36 ficheiros"
inclui `lab/typst-original/` corpus**. Se vanilla
quarentena tem corpus separado, P206C deve confirmar
que "36 ficheiros" refere-se ao corpus `lab/parity/
corpus/` (per P204F + pre-existing) — não outros.
C6 confirma empíricamente.

---

## §9 Particularidade — execução

P206C é trabalho cross-modular:

- 1–3 ficheiros novos em `lab/parity/src/` (helpers).
- Possível ficheiro novo em `03_infra/` ou refactor em
  `04_wiring/` (per C2).
- 1 ficheiro novo de tests parameterizados.
- Cobertura de 36 ficheiros (cada SKIP/INCLUDE
  decidido com base em C6).
- Verificação compilação + tests + linter.

Volume médio. Magnitude **M** se C2 = B; **L** se C2 =
A com refactor substancial.

Recomendado Claude Code dado:

- Volume de leitura para C1 (vanilla output formato +
  cristalino consumers + arquitectura `04_wiring/`).
- Iteração rápida com cargo build em caso de C2 = A
  (refactor `main.rs`).
- Decisão arquitectural em C2 que beneficia de
  comparação detalhada de custos.

Sessão actual viável apenas se houver tempo
significativo. Caso contrário, Claude Code é mais
apropriado.

# Passo 206B — Reactivar harness + smoke vanilla CLI

**Série**: 206 (sub-passo `B` = implementação foundational
após diagnóstico P206A).
**Tipo**: implementação focada (fix breaks + smoke
sentinel).
**Magnitude planeada**: S.
**Pré-condição**: P206A concluído; ADR-0075 PROPOSTO em
`00_nucleo/adr/typst-adr-0075-vanilla-integration.md`;
auditoria em `typst-passo-206A-auditoria-vanilla.md`;
diagnóstico em `typst-passo-206A-diagnostico.md`; tests
1860 verdes; 0 violations; vanilla typst CLI 0.14.2
disponível em `/usr/local/bin/typst` (per A5 P206A);
21 sentinelas activas.
**Output**: 3 ficheiros (inventário + relatório +
alterações de código).

---

## §1 Propósito

Reactivar o harness `lab/parity/` per Caminho A fixado
em P206A C1:

- Fixar 2 pre-existing breaks identificados em P204F.div-1
  e confirmados em P206A A2:
  - `lab/parity/tests/layout_parity.rs:69` — signature
    outdated `layout(content, state)` → `layout(content)`.
  - `lab/parity/src/value_dto.rs:83` — missing
    `Value::Location(_)` arm.
- Adicionar smoke sentinel para detecção runtime de
  vanilla CLI (`typst --version`).
- Confirmar que `cargo check --manifest-path lab/parity
  --all-targets` passa.
- Confirmar que `parity-runner` continua funcional.

P206B **não** invoca vanilla CLI em runtime para
comparação — esse trabalho é P206C. P206B materializa
apenas a pré-condição (harness compila + vanilla CLI
detectável).

P206B respeita o padrão: começa com inventário empírico
antes de qualquer alteração.

---

## §2 Material de partida verificado em P206A

Antes de qualquer alteração, confirmar empíricamente:

- `lab/parity/Cargo.toml` declara binary `parity-runner`
  + bin `lab/parity/src/lib.rs` ou similar.
- `lab/parity/tests/layout_parity.rs` contém o break em
  linha ~69 (signature outdated).
- `lab/parity/src/value_dto.rs` contém o break em linha
  ~83 (missing `Value::Location(_)` arm).
- `cargo check --manifest-path lab/parity --all-targets`
  falha com erros relacionados com os 2 breaks (e
  apenas esses).
- `/usr/local/bin/typst --version` retorna `0.14.2` (per
  A5 P206A).

Sem isto, recuar para P206A.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código, listar literalmente:

1. **`tests/layout_parity.rs:69`** — confirmar:
   - Linha exacta com signature outdated.
   - Tipo de erro de compilação (extra arg, type
     mismatch).
   - Linhas adjacentes que possam precisar de ajuste
     (ex: `let state = introspect(...)` removido).
2. **`src/value_dto.rs:83`** — confirmar:
   - Local exacto onde `Value::Location(_)` arm deve
     ser adicionado (match ou impl?).
   - Padrão de outros arms para coerência.
   - Tipo de retorno esperado para `Location` arm.
3. **Outros breaks não documentados** — `cargo check`
   completo:
   - Lista exaustiva de todos os erros.
   - Confirmar que apenas os 2 breaks acima existem
     (per A2 P206A).
   - Se houver breaks adicionais, registar
     `P206B.div-N`.
4. **`parity-runner` smoke test** — antes de qualquer
   fix:
   - Conseguir invocar `cargo run --bin parity-runner`?
   - Output esperado.
5. **Vanilla CLI detection mechanism** — convenção
   cristalina para detectar binários externos:
   - Há helper existente (env var, `which`, `Command`)?
   - Padrão para skip graceful em CI/local sem vanilla?

Output: 5 sub-secções com etiqueta CONFIRMADO ou
**AJUSTE NECESSÁRIO**.

Se C1.3 detectar breaks adicionais não documentados,
registar `P206B.div-N` antes de prosseguir.

### C2 — Fix `layout_parity.rs:69`

Edição literal (notação ilustrativa baseada em A2
P206A — confirmar formato exacto em C1.1):

```text
- let layouter = layout(content, state);
- let state = introspect(content);
+ let layouter = layout(content);
```

Linha de `let state = introspect(content)` removida
(produzia o `state` que era passed); signature usa
apenas `content`.

### C3 — Fix `value_dto.rs:83`

Edição literal (depende do match exact em C1.2):

```text
  match value {
      Value::Auto => ...,
      Value::None => ...,
      ...
+     Value::Location(_) => ValueDto::Location,  // ou similar
      ...
  }
```

Padrão exacto fixa-se em C1.2 com base em outros arms.

### C4 — Smoke sentinel para vanilla CLI

Adicionar test (ou função helper) que:

- Executa `typst --version` via `std::process::Command`.
- Confirma exit status 0.
- Captura output e verifica que contém "0.14" (versão
  esperada).
- **Skip graceful** se vanilla CLI ausente — não falha
  o test, apenas marca como ignored com mensagem.

Localização preferida (decide em C1.5): novo ficheiro
`lab/parity/tests/vanilla_cli_smoke.rs` ou helper em
`lab/parity/src/vanilla.rs`.

C4 fixa local concreto.

### C5 — Compilação

```
cargo check --manifest-path lab/parity --all-targets
```

Critério: verde. Sem erros, sem warnings novos.

```
cargo build --workspace
```

Critério: verde (workspace cristalino não é afectado;
lab/parity é separado).

### C6 — `parity-runner` smoke

```
cargo run --manifest-path lab/parity --bin parity-runner
```

Critério: executa sem panic; output coerente com
funcionalidade existente cristalino-only baseline.

Se output divergir do esperado pré-P206B (per smoke
test em A3 P206A), registar `P206B.div-N`.

### C7 — Vanilla CLI smoke executa

```
cargo test --manifest-path lab/parity --test vanilla_cli_smoke
```

Critério: verde. Confirma que vanilla CLI está
acessível via `Command`.

### C8 — Tests workspace cristalino

```
cargo test --workspace 2>&1 | tail -10
```

Critério: 1860 mantém-se. P206B não toca em código
workspace cristalino — tests devem ser invariantes.

Se algum teste falhar, é regressão acidental — recuar.

### C9 — Linter

```
crystalline-lint .
```

Critério: 0 violations.

Cuidado: `lab/` é quarentena (per CLAUDE.md). Linter
pode ignorar `lab/` ou aplicar regras diferentes.
Confirmar política em C1.

### C10 — Documentação ADR-0075

ADR-0075 mantém PROPOSTO. Anotação cirúrgica em §P206B
do plano de materialização: `✅ MATERIALIZADO 2026-05-08`
+ sumário (1–2 linhas).

### C11 — Sentinelas

Sentinelas adicionadas em P206B vivem em `lab/parity/tests/`
(quarentena), não no workspace cristalino. Contagem das
21 sentinelas activas no workspace permanece inalterada.

Vanilla smoke test (C4) é sentinela de runtime
(detecção de ferramenta externa), não estrutural.
Documentado como tal.

### C12 — Critério de fecho de P206B

P206B concluído quando:

- C1 inventário completo (5 sub-secções).
- C2 `layout_parity.rs:69` fixado.
- C3 `value_dto.rs:83` fixado.
- C4 vanilla CLI smoke sentinel adicionado.
- C5 `cargo check lab/parity` verde.
- C6 `parity-runner` smoke verde.
- C7 vanilla CLI smoke verde.
- C8 tests workspace cristalino 1860 mantém-se.
- C9 linter 0 violations.
- C10 ADR-0075 anotada.
- Inventário registado.
- Relatório escrito.

### C13 — Sem cláusulas condicionais

C1 produz dados. C2–C4 executam decisões fixas com
base em P206A.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-206B-inventario.md`.

Conteúdo:
- §1 C1 — inventário (5 sub-secções).
- §2 C2 — fix `layout_parity.rs`.
- §3 C3 — fix `value_dto.rs`.
- §4 C4 — vanilla CLI smoke.
- §5 Decisões durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-206B-relatorio.md`.

Conteúdo:
- O que foi feito.
- Tempo de execução.
- Métricas.
- Decisões.
- Sugestão para próximo sub-passo (P206C).

### Ficheiro 3 — Alterações em código

Não é ficheiro discreto. Conjunto de:

- `lab/parity/tests/layout_parity.rs` (1 line + remoção
  trivial).
- `lab/parity/src/value_dto.rs` (1 line arm).
- `lab/parity/tests/vanilla_cli_smoke.rs` (novo) ou
  similar (per C4).
- Anotação cirúrgica em ADR-0075.

---

## §5 Critério de progressão para P206C

P206B fechado quando C12 cumprido.

Em caso de divergência empírica relevante (ex: breaks
adicionais não documentados, vanilla CLI não detectável
em ambiente local diferente), registar em `P206B.div-N`
e:

- Resolver dentro de P206B (preferido — fixes triviais).
- Recuar para P206A re-fixar C5 (vanilla acesso) se
  ambiente difere significativamente do auditado.

P206C só começa quando P206B fechado.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 3 outputs.
- Inventário empírico antes de implementação.
- Localização canónica:
  `00_nucleo/diagnosticos/` para inventário;
  `00_nucleo/materialization/` para relatório.
- Distinção fecho estrutural vs final mantida.
- Sem inflação retórica.
- `lab/parity/` é quarentena — não invade workspace
  cristalino.

---

## §7 Não-objectivos

P206B não:

- Invoca vanilla CLI em runtime para comparação (P206C).
- Adiciona expectations vanilla nas companions
  `.typ.toml` (P206C ou P206D).
- Cobre todos os 36 ficheiros corpus (P206D).
- Transita ADR-0075 para ACEITE (P206E).
- Transita ADR-0073 para "completo final" (P206E
  endereça cond 9).
- Cria ADR nova além de ADR-0075 já PROPOSTO.
- Modifica corpus existente (P204F já fechou).
- Toca em código produção workspace cristalino (P206B
  é puramente lab/parity).
- Compila vanilla typst (pre-built CLI usado).

---

## §8 Erro a não repetir

Da série P204+P205 — pattern empírico: inventário antes
de decisão; honestidade sobre divergências.

Risco específico de P206B: **assumir 2 breaks são os
únicos sem verificar empíricamente**. P206A A2 reportou
2 breaks; P206B C1.3 confirma exhaustivamente via
`cargo check` com lista completa. Se houver breaks
adicionais, P206B.div-N regista honestamente.

Outro risco: **vanilla CLI smoke escrito de forma
fragile** (ex: hardcoded path ou versão exacta). Smoke
deve ser robusto:
- Versão check via "0.14" (não "0.14.2 (b33de9de)") —
  permite micro-versões.
- Skip graceful se ausente (em vez de falha) — permite
  CI sem vanilla.
- Path resolvido via `which typst` ou `Command::new("typst")`,
  não hardcoded.

Hipótese específica: `lab/parity/tests/` pode ter mais
de 2 breaks (P206A reportou 2, mas auditoria não foi
exaustiva). Se houver, são geralmente triviais (1-line
fixes per pattern de drift histórico). C1.3 detecta
exaustivamente.

---

## §9 Particularidade — execução

P206B é trabalho de código focado:

- 2 fixes de 1-line cada (~2 LOC mudadas).
- 1 ficheiro novo smoke (~30–50 LOC).
- Verificação `cargo check` + tests + lint.
- Anotação ADR.

Volume baixo. Magnitude S.

Recomendado pela sessão actual (Opus, com bash_tool)
se houver disponibilidade — P206B é o sub-passo mais
simples de P206 e não exige iteração rápida com cargo.
Caso contrário, Claude Code segue padrão habitual.

# Passo 1302 — sanitização, selagem e commit da cadeia P1299–P1301r2

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Natureza:** fechamento mecânico e documental; nenhuma mudança semântica.
**Baseline:** `1f082370e59939de7b57992e137a9f74bfb6758f`.
**Estado inicial esperado:** 8 paths rastreados modificados, 68 não rastreados, índice
vazio; 76 paths no total.
**Commit autorizado, somente após todos os gates:**
`fix: seal color globals and module diagnostics through step 1302`.
**ADRs:** ADR-0107, ADR-0108, ADR-0127 e ADR-0129.

---

## 1. Objetivo

Fechar num único commit a cadeia causal iniciada pela auditoria P1299, continuada pela
materialização P1300 e reparada/certificada por P1301 revisão 2.

P1302 não corrige, refatora, formata, ressela ou reinterpreta produto. Ele apenas:

1. congela e revalida os 76 paths já existentes;
2. confirma a relação entre os vereditos históricos;
3. executa novamente os gates proporcionais e integrais necessários ao commit;
4. produz seis artefatos de fechamento;
5. monta uma allowlist literal de 83 paths;
6. faz staging somente dessa allowlist e cria um único commit.

A alegação máxima permitida é:

> A cadeia P1299–P1301r2 foi revalidada e selada no commit informado, preservando o
> bloqueio histórico P1300 e o certificado posterior que resolveu sua causa; não se
> alega paridade funcional geral.

---

## 2. Cadeia causal que deve ser preservada

### P1299

Veredito: `P1299_PASS_P1300_COHORT_SELECTED`. Selecionou
`color-global-constructors`: `hsl`, `hsv`, `linear_rgb`.

### P1300

Veredito histórico: `P1300_BLOCKED_IMPLEMENTATION`. Os aliases foram removidos, mas o
contrato encontrou 12 divergências determinísticas de mensagem/span em `std.*`. O owner
causal `compiler/eval/bindings/field_access` estava fora do allowlist. O recibo P1300 não
deve ser reescrito nem promovido retroativamente.

### P1301 revisão 2

Veredito: `P1301R2_CERTIFIED`. Reabriu o owner correto, matou 12/12 mutantes, passou os
casos obrigatórios em ordem normal e invertida, deixou somente os dois `Unknown` opacos
declarados e fez os testes P1300 passarem `10/10`.

A relação correta a registrar é:

```text
P1300_BLOCKED_IMPLEMENTATION
  + P1301R2_CERTIFIED(blocking_diagnostic_divergence_resolved=true)
  = cadeia atual fechável sem reescrever o veredito P1300
```

Não usar “P1300 certificado”. A certificação válida é a cadeia posterior P1301r2, com o
escopo limitado declarado em seu certificado.

---

## 3. Entradas terminais pinadas

Antes de qualquer gate, exigir:

| Artefato | identificador administrativo esperado |
|---|---|
| `p1299-certificate.json` | [dado administrativo removido] |
| relatório consolidado da limpeza | [dado administrativo removido] |
| `p1300-verification-receipt.json` | [dado administrativo removido] |
| `p1301r2-contract-seal.json` | [dado administrativo removido] |
| `p1301r2-verification-receipt.json` | [dado administrativo removido] |
| `p1301r2-certificate.json` | [dado administrativo removido] |
| relatório consolidado da limpeza | [dado administrativo removido] |

Os oito arquivos rastreados devem ter exatamente estas identidades no início:

| Path | identificador administrativo esperado |
|---|---|
| `00_nucleo/prompts/compiler/eval.md` | [dado administrativo removido] |
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | [dado administrativo removido] |
| `00_nucleo/prompts/compiler/eval/tests.md` | [dado administrativo removido] |
| `00_nucleo/prompts/compiler/stdlib/color.md` | [dado administrativo removido] |
| `01_core/src/compiler/eval/bindings/field_access.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/mod.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/tests.rs` | [dado administrativo removido] |
| `01_core/src/compiler/stdlib/color.rs` | [dado administrativo removido] |

Estado de autoria medido:

- `git status --short` identificador administrativo:
  [dado administrativo removido];
- `git diff --binary` identificador administrativo:
  [dado administrativo removido];
- diff rastreado: 8 arquivos, 618 inserções, 24 remoções;
- índice: vazio;
- `git diff --check`: PASS;
- `git diff --cached --check`: PASS.

Depois que este próprio passo existir, o hash do status naturalmente muda. O auditor deve
comparar a diferença nominal e aceitar somente a adição de
`00_nucleo/materialization/typst-passo-1302.md`. Qualquer outro delta antes dos outputs
P1302 termina em `P1302_BLOCKED_INPUT_DRIFT`.

---

## 4. Proibições absolutas

Durante todo o P1302 é proibido:

- editar os 76 paths de entrada;
- executar `cargo fmt` sem `--check`;
- executar `crystalline-lint --fix-hashes` sem `--dry-run`;
- regenerar matriz, oracle, mutantes, selo ou certificado anterior;
- alterar expectativas de testes;
- resolver warnings consultivos;
- adicionar uma correção “pequena” encontrada durante o fechamento;
- usar `git add .`, `git add -A`, glob ou diretório como alvo de staging;
- usar `git commit -a`, amend, rebase, reset, checkout destrutivo ou stash;
- apagar targets ou temporários preexistentes;
- fazer push.

Qualquer correção necessária bloqueia o P1302 e exige passo novo. O fechamento não pode
virar continuação semântica.

---

## 5. Papéis de fechamento

P1302 usa quatro autoridades sequenciais:

| Papel | Escrita permitida | Responsabilidade |
|---|---|---|
| P1 — auditor da cadeia | `p1302-baseline-manifest.json`, `p1302-chain-audit.json` | congelar 76 paths, revalidar hashes e vereditos |
| P2 — verificador | `p1302-verification-receipt.json` | executar gates sem corrigir falhas |
| P3 — selador | `p1302-staging-manifest.json`, `p1302-certificate.json`, relatório consolidado da limpeza | pinar allowlist e autorizar staging |
| P4 — stager/committer | índice Git e commit único | revalidar selo, fazer staging literal e commit |

P1 não altera inputs; P2 não corrige o que verifica; P3 não executa staging; P4 não edita
arquivos. Se o filesystem compartilhado permitir leitura cruzada, não alegar isolamento
técnico — a propriedade aqui é separação de capacidade de escrita e ordem causal.

---

## 6. Allowlist congelada de entrada — 76 paths

### 6.1 Rastreados modificados — 8

```text
00_nucleo/prompts/compiler/eval.md
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/tests.md
00_nucleo/prompts/compiler/stdlib/color.md
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/eval/tests.rs
01_core/src/compiler/stdlib/color.rs
```

### 6.2 P1299 — 14

```text
00_nucleo/diagnosticos/p1299-baseline-status.txt
00_nucleo/diagnosticos/p1299-certificate.json
00_nucleo/diagnosticos/p1299-crystalline-default.json
00_nucleo/diagnosticos/p1299-crystalline-html.json
[artefato protocolar removido]
00_nucleo/diagnosticos/p1299-feature-matrix.json
00_nucleo/diagnosticos/p1299-inventory-default.json
00_nucleo/diagnosticos/p1299-inventory-html.json
00_nucleo/diagnosticos/p1299-manifest.json
00_nucleo/diagnosticos/p1299-owner-ledger.tsv
00_nucleo/diagnosticos/p1299-probe-catalog.json
00_nucleo/diagnosticos/p1299-run-matrix.py
[artefato protocolar removido]
00_nucleo/materialization/typst-passo-1299.md
```

### 6.3 P1300 — 21

```text
[artefato protocolar removido]
[artefato protocolar removido]
00_nucleo/diagnosticos/p1300-contract-runner.py
00_nucleo/diagnosticos/p1300-contract-seal.json
00_nucleo/diagnosticos/p1300-contract.json
00_nucleo/diagnosticos/p1300-discrimination-receipt.json
00_nucleo/diagnosticos/p1300-feature-matrix.json
[artefato protocolar removido]
[artefato protocolar removido]
[artefato protocolar removido]
00_nucleo/diagnosticos/p1300-manifest.json
00_nucleo/diagnosticos/p1300-mutants.json
00_nucleo/diagnosticos/p1300-oracle-suite.json
[artefato protocolar removido]
[artefato protocolar removido]
[artefato protocolar removido]
00_nucleo/diagnosticos/p1300-surface-default.json
00_nucleo/diagnosticos/p1300-surface-html.json
[artefato protocolar removido]
00_nucleo/diagnosticos/p1300-verification-receipt.json
00_nucleo/materialization/typst-passo-1300.md
```

### 6.4 P1301 revisão 1 — 16

```text
[artefato protocolar removido]
[artefato protocolar removido]
00_nucleo/diagnosticos/p1301-candidate-runner.py
[artefato protocolar removido]
00_nucleo/diagnosticos/p1301-contract-runner.py
00_nucleo/diagnosticos/p1301-contract-seal.json
00_nucleo/diagnosticos/p1301-contract.json
00_nucleo/diagnosticos/p1301-discrimination-receipt.json
[artefato protocolar removido]
[artefato protocolar removido]
00_nucleo/diagnosticos/p1301-manifest.json
00_nucleo/diagnosticos/p1301-mutants.json
00_nucleo/diagnosticos/p1301-oracle-suite.json
[artefato protocolar removido]
[artefato protocolar removido]
[artefato protocolar removido]
```

### 6.5 P1301 revisão 2 — 17

```text
[artefato protocolar removido]
00_nucleo/diagnosticos/p1301r2-certificate.json
[artefato protocolar removido]
00_nucleo/diagnosticos/p1301r2-contract-runner.py
00_nucleo/diagnosticos/p1301r2-contract-seal.json
00_nucleo/diagnosticos/p1301r2-contract.json
00_nucleo/diagnosticos/p1301r2-discrimination-receipt.json
[artefato protocolar removido]
[artefato protocolar removido]
[artefato protocolar removido]
00_nucleo/diagnosticos/p1301r2-manifest.json
00_nucleo/diagnosticos/p1301r2-mutants.json
00_nucleo/diagnosticos/p1301r2-mutation-plan.json
[artefato protocolar removido]
00_nucleo/diagnosticos/p1301r2-oracle-suite.json
[artefato protocolar removido]
00_nucleo/diagnosticos/p1301r2-verification-receipt.json
```

P1 deve comparar essa lista ao status real como conjunto e por status. Ordem textual não
é prova suficiente. Ausência, path adicional ou status diferente bloqueia.

---

## 7. Artefatos novos P1302 — 7 paths

```text
00_nucleo/materialization/typst-passo-1302.md
00_nucleo/diagnosticos/p1302-baseline-manifest.json
00_nucleo/diagnosticos/p1302-chain-audit.json
00_nucleo/diagnosticos/p1302-verification-receipt.json
00_nucleo/diagnosticos/p1302-staging-manifest.json
00_nucleo/diagnosticos/p1302-certificate.json
[artefato protocolar removido]
```

Depois dos outputs, a allowlist terminal tem exatamente `76 + 7 = 83` paths. Nenhum outro
arquivo pode aparecer.

O manifesto baseline registra:

- HEAD, branch, data/hora e ferramentas;
- status integral, contagens e identificador administrativo;
- diff rastreado binário, diff stat e identificador administrativo;
- índice vazio;
- os 76 paths e seus identificador administrativo;
- os vereditos P1299/P1300/P1301r2;
- a lista fechada dos sete outputs P1302.

O audit receipt verifica referências internas dos certificados, hashes de consumers,
prompts, selos, receipts e a resolução explícita do blocker P1300.

---

## 8. Gates de verificação P2

Usar um único `CARGO_TARGET_DIR` novo, preferencialmente sob `/dev/shm`, sem alterar
`TMPDIR`, `TMP` ou `TEMP`. Não fazer cleanup adaptativo. Registrar início, fim, duração,
argv, exit, stdout e stderr.

### 8.1 Focais e contrato selado

```bash
cargo test -p typst-core p1300 -- --test-threads=1
cargo test -p typst-core p1301 -- --test-threads=1
```

Reexecutar o runner P1301r2 selado exatamente como documentado no receipt terminal:

- self-check;
- gate discriminatório dos 12 mutantes;
- candidato em ordem normal;
- candidato em ordem invertida;
- exigir o mesmo vetor canônico por `case_id`;
- exigir `20 Preserved / 2 Unknown declarados / 0 Violated / 0 FAIL`;
- exigir mutation score `12/12 = 1.0`.

Não substituir o runner por uma interpretação manual.

### 8.2 Integrais

```bash
cargo test --workspace
cargo build
cargo fmt --all -- --check
python3 -m unittest \
  lab/surface-inventory/test_merge.py \
  lab/surface-inventory/test_run_probes.py \
  [artefato protocolar removido]
crystalline-lint .
crystalline-lint --fail-on warning --checks v3,v4,v5,v13,v14,v15,v26 .
crystalline-lint --fix-hashes --dry-run .
git diff --check
git diff --cached --check
```

Resultados obrigatórios:

- todos os processos exit `0`;
- `crystalline-lint --fix-hashes --dry-run .` imprime `Nothing to fix`;
- checks estritos sem warning/violation;
- índice continua vazio;
- hashes dos 76 inputs continuam idênticos;
- status contém somente os 83 paths permitidos após os outputs.

Warnings consultivos fora do recorte estrito devem ser registrados, não corrigidos.

Qualquer falha termina P1302 como bloqueado. P2 não tenta reparar, repetir após edição ou
selecionar subset menor.

---

## 9. Selo pré-staging P3

Somente após P2 verde, P3 produz:

- `p1302-staging-manifest.json`, com os 83 paths em ordem lexicográfica, status esperado,
  tamanho e identificador administrativo;
- `p1302-certificate.json`, pinando manifesto, auditoria, verificação e staging manifest;
- relatório consolidado da limpeza, com veredito pré-commit
  `P1302_READY_FOR_EXACT_STAGING_AND_COMMIT`.

O certificado deve declarar:

- claim máxima literal;
- nenhuma mudança de bytes nos 76 inputs depois do baseline;
- índice vazio antes do staging;
- allowlist terminal de 83 paths;
- mensagem exata do commit;
- proibição de escrita de conteúdo pelo P4;
- ausência de staging/commit no momento da emissão;
- política de hash próprio: hash detached reportado depois da criação, sem recursão.

P3 não faz `git add` nem `git commit`.

---

## 10. Staging e commit P4

P4 revalida todos os hashes do certificado e exige
`P1302_READY_FOR_EXACT_STAGING_AND_COMMIT`. Depois:

1. confirmar índice vazio;
2. executar `git add --` com os 83 paths literais do staging manifest, sem glob;
3. comparar `git diff --cached --name-only` como conjunto exato com a allowlist;
4. exigir que `git diff --name-only` esteja vazio;
5. executar `git diff --cached --check`;
6. revalidar o hash do patch cached e a contagem de paths;
7. criar exatamente um commit:

```bash
git commit -m "fix: seal color globals and module diagnostics through step 1302"
```

8. confirmar que o commit possui exatamente os 83 paths;
9. confirmar working tree e índice vazios;
10. reportar ao dono o hash real do commit, sem escrever um artefato pós-commit que
    exigiria segundo commit.

Se qualquer condição mudar após o selo, não fazer commit. Não remover staging parcial por
comando destrutivo; parar e relatar o conjunto exato.

---

## 11. Vereditos permitidos

- `P1302_READY_FOR_EXACT_STAGING_AND_COMMIT` — certificado pré-commit válido;
- `P1302_COMMITTED` — commit único criado e árvore limpa;
- `P1302_BLOCKED_INPUT_DRIFT`;
- `P1302_BLOCKED_CHAIN_INCONSISTENCY`;
- `P1302_BLOCKED_FOCAL_GATE`;
- `P1302_BLOCKED_WORKSPACE_GATE`;
- `P1302_BLOCKED_ARCHITECTURAL_GATE`;
- `P1302_BLOCKED_STAGING_MISMATCH`;
- `P1302_BLOCKED_COMMIT_FAILURE`.

`P1302_COMMITTED` só pode ser comunicado após confirmar o hash real do commit e a árvore
limpa. O próximo trabalho de paridade — os spans gated de `pdf.table-summary`,
`pdf.header-cell` e `pdf.data-cell` — pertence a um passo posterior e não pode entrar
neste commit.

# Relatório P934 — Reconciliar discrepância de 27× entre medições de vanilla em P923 e P933

**Data de execução:** 2026-07-30  
**Ficheiro de passo:** `00_nucleo/materialization/typst-passo-934.md`  
**Commit base:** `acef3e88d0d5d7cc62c06dfbb741590aca2cc331`  
**Estado final da árvore:** sem alterações de código de produção relativamente a P933-fixed; `temp/p934*` removido.

---

## 1. Resumo executivo

A discrepância de 27× resolve-se: **P923 estava correto e P933 usou o binário errado como "vanilla"**.

| Medição | Vanilla reportado para `05-utf8.typ` | O que de facto se mediu |
|---|---|---|
| P923, Fase E.2 | ~0,3 s | `lab/typst-original/target/release/typst` — **vanilla real** |
| P933, secção 4.2 | ~8,2 s | `target-original/release/typst` — **cristalino P927**, não vanilla |

A remediação lado a lado, na mesma sessão, com o vanilla real, confirma:

| Cenário | Vanilla real | Cristalino P933-fixed | Rácio cristalino/vanilla |
|---|---:|---:|---:|
| `utf8-latin` | 274,7 ms | 93,8 ms | 0,34× |
| `utf8-greek` | 268,4 ms | 93,0 ms | 0,35× |
| `utf8-cjk` | 303,9 ms | 2371,4 ms | 7,80× |
| `utf8-emoji` | 283,9 ms | 7716,4 ms | 27,18× |
| `05-utf8` | 309,6 ms | 7744,4 ms | 25,02× |

O vanilla real é ~300 ms para CJK/emoji/05-utf8; o cristalino é ~2,4–7,7 s. O vanilla usa `fontdb-0.23.0`, que pré-computa a cobertura das fontes no arranque. O cristalino não faz essa pré-computação — é essa a diferença mecânica portável a investigar num passo futuro.

**Conclusão de P933 a corrigir:** a afirmação "não há nada a copiar do vanilla" estava baseada num binário errado. Há **muito** a aprender com a forma como o vanilla evita parsear fontes no caminho quente.

---

## 2. Proveniência

### Binários

| Papel | Caminho | SHA-256 | Versão reportada | Notas |
|---|---|---|---|---|
| Vanilla real | `lab/typst-original/target/release/typst` | `e73e4ac16f1d64843941b405a902f5f6952df4414d60b5a953efbe4952b858b5` | `typst 0.15.0 (969087ec)` | Usado em P923; strings confirmam `fontdb-0.23.0`. |
| "Vanilla" usado em P933 | `target-original/release/typst` | `8446552fa49a24220021e2ac2601a6b7c09322ee024507412b14be855dbb2072` | `typst 0.15.0 (da18ea9f)` | Na verdade **cristalino** P927; strings contêm `Typst compiler (crystalline)`, `CRYSTALLINE_DOCUMENT_ID`, caminhos `/tmp/typst-crystalline-da18ea9f3/...`. |
| Cristalino P933-fixed | `target/release/typst` | `63d8e4a585caf335d1ed8859cebb952ca6ab7ffcfed894c78fc1ab07f180c878` | `typst 0.15.0 (5a789380)` | Estado com a correcção de `face_covers_char` aplicada em `03_infra/src/shaper.rs`. |

### Ambiente de fontes

| Item | Valor |
|---|---|
| `fc-list \| wc -l` | 1407 famílias/entradas |
| Faces únicas abertas por ambos os binários | 2172 |
| Aberturas de ficheiro de fonte (vanilla real, `utf8-cjk`) | 4422 |

### Documentos de teste

Corpus: `tools/perf/corpus/p923/`. Os cinco documentos usados são os mesmos de P923/P933:

- `utf8-latin.typ`, `utf8-greek.typ`, `utf8-cjk.typ`, `utf8-emoji.typ`, `05-utf8.typ`.

Confirmou-se que o `05-utf8.typ` usado em P934 é o mesmo ficheiro referenciado em P923/P933 (mesmo caminho, mesmo conteúdo da frente).

### Scripts

- Script de remediação: `temp/p934-bench-suite.py` (apagado no final do passo).
- Resultados JSON intermédios: `temp/p934-*.json` (apagados).
- Logs `strace`: `temp/p934-strace-lab-vanilla.log`, `temp/p934-strace-target-original.log` (apagados).

---

## 3. Fase A — confirmação do cenário

### 3.1 Ficheiro de teste

O `05-utf8.typ` é o mesmo documento referenciado em P923 e P933 (caminho `tools/perf/corpus/p923/05-utf8.typ`). Não houve alteração de conteúdo entre os dois relatórios.

### 3.2 Binário vanilla

Aqui estava a raiz da discrepância. P933 registou o vanilla como `target-original/release/typst` (SHA `8446552f...`), mas esse binário é de facto o **cristalino no estado P927**. A confusão vem da convenção do script `tools/perf/benchmark-vanilla-vs-p927.py`, onde `target-original/release/typst` representa o cristalino *antes* das optimizações P927, não o Typst original.

O vanilla real é `lab/typst-original/target/release/typst` (SHA `e73e4ac1...`). Diferenças verificáveis:

| Propriedade | Vanilla real | `target-original/release/typst` |
|---|---|---|
| Versão CLI | `typst 0.15.0 (969087ec)` | `typst 0.15.0 (da18ea9f)` |
| Strings de crate | `fontdb-0.23.0` | `fontdb-0.23.0` + `CRYSTALLINE_*` |
| Identidade do binário | `The Typst compiler` | `Typst compiler (crystalline)` |
| Caminhos temporários | nenhum | `/tmp/typst-crystalline-da18ea9f3/...` |

### 3.3 Ambiente de fontes

Ambos os binários abrem 2172 faces únicas. O número de famílias listadas por `fc-list` (1407) e o número de faces únicas (2172) são consistentes com os valores registados em P933 — o ambiente de fontes não mudou significativamente entre P923 e P933. Logo, a diferença de 27× **não** é explicada pelo número de fontes instaladas.

---

## 4. Fase B — remediação lado a lado

### 4.1 Metodologia

- Correr o vanilla real (`lab/typst-original/target/release/typst`) e o cristalino P933-fixed (`target/release/typst`) sobre os mesmos documentos, na mesma sessão.
- `hyperfine --warmup 1 --min-runs 5` para cada par (vanilla vs. cristalino).
- Cada binário compilava o documento para `/dev/null` (formato PDF).

### 4.2 Resultados corrigidos

| Cenário | Vanilla real (ms) | Cristalino P933-fixed (ms) | Rácio |
|---|---:|---:|---:|
| `utf8-latin` | 274,7 | 93,8 | 0,34× |
| `utf8-greek` | 268,4 | 93,0 | 0,35× |
| `utf8-cjk` | 303,9 | 2371,4 | 7,80× |
| `utf8-emoji` | 283,9 | 7716,4 | 27,18× |
| `05-utf8` | 309,6 | 7744,4 | 25,02× |

### 4.3 Interpretação

- **Latim/grego:** o cristalino é ~3× mais rápido que o vanilla. Ambos são rápidos; a diferença não é decisiva para a frente de trabalho.
- **CJK/emoji/05-utf8:** o vanilla real é ~300 ms; o cristalino é ~2,4–7,7 s. **O vanilla é de facto muito mais rápido** para os cenários de fallback pesado.
- A explicação mecânica identificada é que o vanilla usa `fontdb-0.23.0`, que constrói a cobertura de todas as fontes no arranque. Quando o fallback dispara, o lookup de codepoint é O(1) sobre estruturas já materializadas. O cristalino, pelo contrário, paga o parse/scan da cmap no caminho quente.

---

## 5. Reavaliação da conclusão de P933

P933 concluiu (secção 4.3):

> "Nada. O vanilla paga o parse eager de todas as fontes, o que é pior do que a solução lazy do cristalino."

Essa conclusão está **errada** porque o "vanilla" medido em P933 era o cristalino P927. A remediação mostra que:

1. O vanilla real **paga** o parse eager, mas o faz **tão eficientemente** que o tempo total para CJK/emoji é ~300 ms — não ~8 s.
2. O cristalino P933-fixed ainda é ~3× mais rápido que o vanilla em CJK? **Não**. Com o vanilla real, o cristalino é **7,8× mais lento** em CJK e **27× mais lento** em emoji.
3. Há, portanto, **mecanismos no vanilla a investigar**: a forma como `fontdb-0.23.0` pré-computa e organiza a cobertura permite resolver fallback sem reparsear faces no caminho quente.

A correcção do shaper introduzida em P933 (reintroduzir `face_covers_char`) mantém-se válida e necessária — resolveu um bug real de falso positivo. O que muda é a interpretação estratégica: o vanilla é um alvo de paridade válido, e a frente P925-933 deve continuar procurando reduzir a diferença.

---

## 6. Lição metodológica

A discrepância só foi possível porque os relatórios anteriores não registaram identidade suficiente do binário medido. Daqui em diante, toda a medição desta frente deve incluir:

1. **SHA-256 do executável** (não apenas o caminho nem a versão CLI).
2. **Versão/commit reportado** pela CLI, mas tratado como secundário — a versão CLI pode ser igual para binários diferentes.
3. **Número de fontes do sistema** (`fc-list | wc -l`) e, quando relevante, número de faces únicas abertas.
4. **Identidade do binário** (vanilla vs. cristalino), verificada por strings distintivas quando ambos coexistem.
5. **Hash do conteúdo do documento de teste** (SHA-256), para garantir que "mesmo nome" = "mesmo conteúdo".

A regra geral do projecto (registar proveniência de cada medição) aplica-se em força máxima aqui: sem o SHA-256 do executável, o número de ~8 s de P933 era irreproduzível e, como se viu, incorrecto.

---

## 7. Nota sobre o código de produção

Neste passo **não houve alterações de código de produção**. A alteração em `03_infra/src/shaper.rs` (P933-fixed) mantém-se como estava ao início do passo. O trabalho de P934 foi puramente de investigação, remediação metodológica e documentação.

O próximo passo lógico (fora do escopo de P934) é investigar, com protótipos temporários e medição, como portar para o cristalino a estratégia do vanilla de pré-computar cobertura sem pagar o preço que o cristalino já demonstrou ser proibitivo no caso comum (lições de P930/P931).

---

## 8. Validação final

- `cargo build --workspace --release`: ok.
- `cargo test -p typst-infra --lib`: 743 passed; 0 failed.
- `crystalline-lint .`: 0 violations (apenas V7 pré-existente, `package_version_resolution.md`).
- `temp/p934*`: removido.

---

## 9. Entregáveis

1. Este relatório: `00_nucleo/diagnosticos/typst-passo-934-relatorio.md`.
2. Estado da árvore: idêntico a P933-fixed, com `temp/p934*` limpo.

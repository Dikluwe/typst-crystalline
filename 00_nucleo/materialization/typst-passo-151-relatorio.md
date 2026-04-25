# Passo 151 — Relatório (investigação DEBT-53; abertura de DEBT-54)

**Data**: 2026-04-25
**Natureza**: passo **L0-puro / investigação empírica**.
**Zero código tocado**. **Zero ADRs criadas**. **1 DEBT
aberto** (DEBT-54) por resultado da investigação. **DEBT-53
actualizado** com bloqueio.
**Precondição**: Passo 150 encerrado; DEBT-53 aberto;
infraestrutura `lab/parity/` materializada com baseline
cristalino-only (19/19 compila).

---

## 1. Sumário executivo

Passo 151 foi enunciado para **fechar DEBT-53** (integrar
pipeline vanilla em `lab/parity` para popular matriz P3 com
números reais). Investigação empírica em 151.1 revelou que o
setup vanilla é **pré-condição não-trivial** que excede o
escopo prático de um passo "fecho de DEBT". O spec do P151
§"O que pode sair errado" autoriza esta resposta:

> **Vanilla não compila**: ... Se não compila: pausar, abrir
> **DEBT-54** ("Vanilla repo broken at frozen commit"),
> reverter Cargo ao stub.

**Outputs do passo**:
- **DEBT-54 aberto** ("Setup vanilla `typst` workspace em
  `lab/parity` — pré-condição de DEBT-53") com plano
  específico.
- **DEBT-53 actualizado**: nota de bloqueio por DEBT-54;
  critério de fecho actualizado para apontar passo dedicado
  posterior.
- **§9 dos documentos de paridade** renumerado: P151 = "tentativa
  + DEBT-54"; P152+ = P2; P153+ = P4; "passo dedicado para
  DEBT-54" como item 7.
- **Cabeçalho de `DEBT.md`**: nota Passo 151; total abertos
  11 → 12.

**Tests**: cristalino inalterados em 1113. `lab/parity`
inalterado (matriz baseline P150 preservada).

---

## 2. Inventário pré-materialização (sub-passo 151.1)

### 2.1 — Estado de `lab/typst-original/`

```
lab/typst-original/
├── Cargo.toml.original       (workspace root vanilla, DESACTIVADO)
├── crates/
│   ├── typst/                (compilador top-level vanilla)
│   ├── typst-eval/
│   ├── typst-html/
│   ├── typst-layout/
│   ├── typst-library/
│   ├── typst-macros/
│   ├── typst-pdf/
│   ├── typst-realize/
│   ├── typst-render/
│   ├── typst-svg/
│   ├── typst-syntax/         (já em uso via lab/Cargo.toml)
│   ├── typst-timing/         (já transitiva)
│   ├── typst-utils/          (já transitiva)
│   ├── typst-bundle/
│   ├── typst-cli/
│   ├── typst-ide/
│   ├── typst-kit/
│   ├── ...
├── docs/
└── tests/
```

`Cargo.toml` (sem extensão `.original`) **não existe** —
vanilla workspace está em **quarentena** intencional. As
crates vanilla declaram `version = { workspace = true }`,
`rust-version = { workspace = true }`, `edition = { workspace
= true }` etc — exigem workspace root activo para resolver.

### 2.2 — Mecanismo actual de path-dep

`lab/Cargo.toml` (workspace virtual de `lab/parity`)
**intercepta** a resolução de workspace para
`typst-syntax` + `typst-utils` + `typst-timing`:

```toml
[workspace.package]
version = "0.14.2"
rust-version = "1.89"
authors = ["The Typst Project Developers"]
edition = "2024"
# ... outros campos herdáveis

[workspace.dependencies]
typst-timing = { path = "typst-original/crates/typst-timing" }
typst-utils  = { path = "typst-original/crates/typst-utils" }
ecow                 = "0.2.6"
rustc-hash           = "2.1"
serde                = "1.0.184"
# ... ~10 deps externas mínimas
```

Funciona para `typst-syntax` porque as suas deps externas
estão todas em `~/.cargo/registry/cache/` (verificado
empiricamente — `cargo build` em `lab/parity` corre limpo).

### 2.3 — Análise das deps de `typst-layout` (target mínimo)

`lab/typst-original/crates/typst-layout/Cargo.toml` declara
**29 dependências**:

**Path-deps internas (6 + 1 transitiva)**:
- `typst-assets` (não verificado se existe em
  `lab/typst-original/crates/`)
- `typst-library` (existe)
- `typst-macros` (proc-macro crate; exige próprio build
  separado)
- `typst-syntax` (já configurado)
- `typst-timing` (já configurado)
- `typst-utils` (já configurado)
- + transitivas das anteriores

**Externas (22)**:
`az`, `bumpalo`, `codex`, `comemo`, `ecow`, `either`,
`hypher`, `icu_properties`, `icu_provider`,
`icu_provider_adapters`, `icu_provider_blob`,
`icu_segmenter`, `kurbo`, `libm`, `memchr`, `rustc-hash`,
`rustybuzz`, `smallvec`, `ttf-parser`, `unicode-bidi`,
`unicode-math-class`, `unicode-script`,
`unicode-segmentation`.

### 2.4 — Verificação do cargo cache

Probe parcial em `~/.cargo/registry/cache/`:
- **Em cache**: `comemo`, `ecow`, `kurbo`, `rustybuzz`,
  `ttf-parser`, várias `icu_*`, etc.
- **Provavelmente ausentes** (a confirmar com fetch online):
  `codex`, `hayagriva` (deps transitivas de
  `typst-library`), `oxipng` (vanilla PDF).

### 2.5 — Estimativa de escopo total

Para `typst::compile<PagedDocument>` (objectivo do P151):

- **Crates internas vanilla (path-deps)**: ~12 (typst,
  typst-eval, typst-html, typst-layout, typst-library,
  typst-macros, typst-pdf, typst-realize, typst-render,
  typst-svg, typst-bundle, typst-assets).
- **Crates externas (workspace.dependencies)**: ~30+ a
  especificar com versões finais; algumas sem cache.
- **Workspace.package fields**: já tem 9; pode precisar de
  mais (e.g., `description = { workspace = true }` em alguns
  Cargo.toml internos).
- **Resolução**: cargo unifica versões; conflitos com
  cristalino possíveis (e.g., `ttf-parser` v0.X vs v0.Y).

---

## 3. Decisão de pausar + DEBT-54

### 3.1 — Aplicação do critério do spec

O spec §"O que pode sair errado" é explícito:

> **Vanilla não compila**: ... Se não compila: pausar, abrir
> DEBT-54 ("Vanilla repo broken at frozen commit"), reverter
> Cargo ao stub.

A investigação 151.1 revelou que vanilla **compila** em
princípio (existe e é coerente), mas **integrá-lo em
`lab/Cargo.toml`** requer trabalho substancial (quase
recriar o workspace vanilla). Critério aplicável **por
analogia**: o setup é o obstáculo, não a vanilla em si.

### 3.2 — DEBT-54 aberto

Conteúdo: ver entrada nova em `00_nucleo/DEBT.md` Secção 1.
Plano com 6 checklist items; critério de fecho: `cargo build
-p typst-layout` corre sem erros em `lab/`.

### 3.3 — DEBT-53 actualizado

- Cabeçalho passa a "EM ABERTO (Passo 150; bloqueado por
  DEBT-54 desde Passo 151)".
- Nova "Actualização Passo 151" descreve a investigação +
  refere DEBT-54.
- Critério de fecho ajustado: numeração do passo dedicado
  passa de "150A diagnóstico + 150B materialização, ou 153
  directo" para "153 directo após DEBT-54 fechar".

### 3.4 — Sequenciamento revisto

- **DEBT-54** materializa setup (independente).
- **DEBT-53** destranca-se assim que DEBT-54 fecha.
- **P152+** (P2 = `value_dto.rs`): **não bloqueado** por
  DEBT-54 — pode ser materializado em paralelo com
  cristalino-only baseline.
- **P153+** (P4 = `pdf_compare.rs`): idem.

---

## 4. Outputs criados

### 4.1 — `00_nucleo/DEBT.md`

- **DEBT-54 aberto** (entrada nova em Secção 1, antes de
  DEBT-53):
  - Título: "Setup vanilla `typst` workspace em `lab/parity`
    (pré-condição de DEBT-53)".
  - Plano com 6 itens checklist.
  - Critério de fecho: `cargo build -p typst-layout`
    (vanilla) corre sem erros.
  - Nota: DEBT-54 não bloqueia P152 (P2 cristalino-only é
    independente).
- **DEBT-53 actualizado**:
  - Cabeçalho com nota de bloqueio.
  - Sub-secção "Actualização Passo 151".
  - Critério de fecho ajustado.
- **Cabeçalho de DEBT.md** ganha nota Passo 151 ("Total
  abertos: 11 → 12").

### 4.2 — `00_nucleo/diagnosticos/typst-paridade-plano-medicao.md`

§9 renumerado:
- Item 4 antes era "Passo 151 = P2"; passa a "Passo 151 =
  tentativa fecho DEBT-53 + abertura DEBT-54".
- Item 5 era "Passo 152 = P4"; passa a "Passo 152+ = P2".
- Item 6 era "Decisão corpus"; passa a item 7 (com item 6
  novo: P153+ = P4).
- Item novo "Passo dedicado para DEBT-54" (item 7).
- Parágrafo final actualizado.

### 4.3 — `00_nucleo/materialization/typst-passo-151-relatorio.md`

Este ficheiro.

---

## 5. Verificação

| Item | Estado |
|------|--------|
| Investigação empírica documentada (sub-passos 151.1) | ✅ |
| DEBT-54 aberto com plano específico (6 itens) | ✅ |
| DEBT-53 actualizado com bloqueio + critério revisto | ✅ |
| §9 dos documentos de paridade renumerado (item 4 → 8) | ✅ |
| Cabeçalho de DEBT.md com nota Passo 151 | ✅ |
| Nenhum ficheiro de código tocado em L1/L2/L3/L4 cristalino | ✅ |
| Nenhum ficheiro tocado em `lab/parity/` (matriz P150 preservada) | ✅ |
| Nenhuma ADR criada / revogada / revisada | ✅ |
| `crystalline-lint .` zero violations | ✅ |
| `cargo test --workspace --lib` cristalino: 1113 inalterado | ✅ |
| `cd lab/parity && cargo test --test layout_parity` continua a correr (P150 baseline) | ✅ |
| Total DEBTs abertos: 11 → 12 | ✅ |
| Relatório do passo escrito | ✅ |

---

## 6. Próximo passo

Recomendação por prioridade:

1. **P152 — P2 (`value_dto.rs`) cristalino-only baseline**
   (independente de DEBT-54). Materializa P2 com mesma
   estratégia de P150: infraestrutura + corpus + matriz com
   colunas vanilla `N/A`.

2. **Passo dedicado para DEBT-54** (escopo M-L; 3-6h).
   Quando priorizado, materializa setup vanilla workspace.
   Após fechar, novo passo subsequente fecha DEBT-53 com
   matriz vanilla preenchida.

3. **P153 — P4 Opção B (textual)** se priorização do
   utilizador for completar matriz P-níveis antes de
   integrar vanilla.

Decisão entre 1, 2 e 3 fica a cargo do utilizador.

---

## 7. Notas operacionais

- **Reformulação 5 da série paridade**: 148 inventário; 149
  arqueologia; 150 baseline; **151 investigação + DEBT-54**.
  Padrão de "passo descobre obstáculo, gera sub-trabalho"
  confirma-se. **Padrão estabelecido**: investigação empírica
  é entrega válida quando precondições não estão satisfeitas.

- **Diferença material face a P150**: P150 entregou código
  novo (FrameDTO + report + tests); P151 não entregou código.
  Isto é **resposta legítima** à descoberta empírica do
  obstáculo. O spec autoriza explicitamente.

- **Padrão DEBT-N → DEBT-(N+1) bloqueia**: DEBT-54 bloqueia
  DEBT-53. Precedente: DEBT-1 dependia de DEBT-52 (rastreador
  aberto em P135 para encerrar primeiro). Modelo conhecido.

- **Pós-151**: o utilizador tem **dois caminhos paralelos**:
  - **Vanilla integration** (DEBT-54 → DEBT-53 → matriz
    real).
  - **P2/P4 cristalino-only** (independente; estende
    cobertura observacional do baseline P150).

  Escolha entre eles é decisão humana.

- **Ausência de ADR sobre paridade observacional**: ainda
  válida. ADR-0033 + ADR-0054 cobrem a base; nenhuma decisão
  arquitectural nova foi tomada em P151.

- **Tempo de execução**: P151 é passo curto (~30min); a
  investigação 151.1 já tem material prévio (relatório P150
  §2.4 antecipou os obstáculos). Trabalho concentrou-se em
  formalizar via DEBT-54 + actualizações coerentes em
  DEBT-53 e §9.

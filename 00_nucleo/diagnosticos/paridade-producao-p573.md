# P573 — Mapa e resolução do estado do directório de trabalho

**Data:** 2026-07-05  
**Passo:** 573  
**Tipo:** Sonda + reconciliação  
**ADR-0108:** em vigor

---

## 1. Objectivo

Após P572, restava no directório de trabalho um bloco de alterações pendentes
(extração de codepoints de `FrameItem::Text` / `collect_text_codepoints`) e
outros ficheiros alterados não explicados. Este passo mapeou tudo o que estava
por commitar, verificou a saúde do estado commitado e classificou cada bloco.

---

## 2. Sonda executada

### 2.1 Estado inicial (`git status` + `git diff HEAD --stat`)

Ficheiros modificados:

- `00_nucleo/prompts/wiring.md` (apenas hash)
- `03_infra/src/export/builder.rs`
- `03_infra/src/export/fonts.rs`
- `03_infra/src/export/mod.rs`
- `03_infra/src/shaper.rs`
- `tools/perf/results/hyperfine-*.json` (41 ficheiros)

Ficheiros não monitorados:

- `00_nucleo/materialization/typst-passo-572.md`
- `00_nucleo/materialization/typst-passo-573.md`

### 2.2 Build/test do estado commitado

```bash
git stash push -u
cargo build --workspace   # ok
cargo test --workspace    # ok
git stash pop
```

O estado commitado (sem nenhuma alteração pendente) compila e passa todos os
testes.

### 2.3 Build/test do estado com alterações pendentes

```bash
cargo build --workspace   # ok
cargo test --workspace    # 1 falha em p307b_07_multi_feature
```

A falha era esperada: P568 muda o subset de fontes, logo o PDF binário do
fixture `07-multi-feature` mudou. O snapshot foi regenerado com
`UPDATE_P307B_SNAPSHOTS=1`. Após regeneração, `cargo test --workspace` passa.

---

## 3. Classificação dos blocos pendentes

### Bloco A — P568: codepoints/glyphs de `FrameItem::Text`

**Ficheiros:**

- `03_infra/src/export/builder.rs`
- `03_infra/src/export/fonts.rs`
- `03_infra/src/export/mod.rs`
- `03_infra/src/shaper.rs`
- `03_infra/fixtures/p307b/reference/07-multi-feature.pdf` (snapshot regenerado)

**Descrição:**

- `fonts.rs`: adiciona `collect_text_codepoints`, que coleta codepoints usados
  em `FrameItem::Text` (caminho fallback), recursivamente através de `Group` e
  `Link`.
- `shaper.rs`: `try_shape` retorna `None` para texto que só contém whitespace,
  preservando espaços como `FrameItem::Text`.
- `builder.rs`: `build_cidfont` e `build_multifont` unem `collect_codepoints`
  com `collect_text_codepoints` tanto para o ToUnicode CMap como para o subset
  de glyphs.
- `mod.rs`: importa `collect_text_codepoints`.

**Decisão:** Resolve um problema real (espaços entre palavras precisam de
permanecer seleccionáveis/copiáveis no PDF) com origem documentada nos
comentários P568. **Manter e commitar.**

**Acção complementar:** os Prompts L0 correspondentes estavam desatualizados
(`fonts.md`, `builder.md`, `shaper.md`, `mod.md`). Foram actualizados com a
secção §P568 e os hashes foram corrigidos via `crystalline-lint --fix-hashes`.

### Bloco B — hash de `wiring.md`

**Ficheiro:** `00_nucleo/prompts/wiring.md`

**Descrição:** alteração do `Hash do Código` de `e5646088` (HEAD) para
`9bc6897d`, sem passo nem motivo identificável. Não correspondia ao hash do
ficheiro `04_wiring/src/main.rs`.

**Decisão:** órfão sem origem. **Descartado** (`git restore`).

### Bloco C — resultados de benchmark

**Ficheiros:** 41 ficheiros em `tools/perf/results/hyperfine-*.json`

**Descrição:** resultados de execuções do rig de performance. Alterações são
timings e caminhos temporários `/tmp/...` gerados automaticamente. Não estão
relacionados com P568 nem com qualquer passo documentado.

**Decisão:** não resolvem problema de código; são artefactos de medição. **Descartados**
(`git restore`).

### Bloco D — ficheiros de materialização

**Ficheiros:**

- `00_nucleo/materialization/typst-passo-572.md`
- `00_nucleo/materialization/typst-passo-573.md`

**Descrição:** documentos de processo (passos de execução), não são prompts L0.

**Decisão:** permanecem como ficheiros não monitorados. **Não commitar.**

### Bloco E — `04_wiring/Cargo.toml`

O passo P572 referia uma alteração "temporária" a `04_wiring/Cargo.toml` que
teria sido revertida. No estado actual do directório este ficheiro **não está
alterado**, e o build/test do estado commitado passa. A versão commitada é a
que fica; não há decisão a tomar.

---

## 4. Estado final do directório de trabalho

```text
 M 00_nucleo/prompts/infra/export/builder.md
 M 00_nucleo/prompts/infra/export/fonts.md
 M 00_nucleo/prompts/infra/export/mod.md
 M 00_nucleo/prompts/infra/shaper.md
 M 03_infra/fixtures/p307b/reference/07-multi-feature.pdf
 M 03_infra/src/export/builder.rs
 M 03_infra/src/export/fonts.rs
 M 03_infra/src/export/mod.rs
 M 03_infra/src/shaper.rs
?? 00_nucleo/materialization/typst-passo-572.md
?? 00_nucleo/materialization/typst-passo-573.md
```

Cada ficheiro modificado tem uma razão conhecida: implementação P568 + L0s
actualizados + snapshot regenerado.

---

## 5. Validação final

```bash
cargo build --workspace      # ok
cargo test --workspace       # ok
crystalline-lint .           # No violations found
```

---

## 6. Conclusão

- O único bloco de trabalho real pendente era P568.
- Os L0s foram actualizados e os hashes corrigidos.
- O snapshot `07-multi-feature.pdf` foi regenerado por mudança intencional do
  output PDF.
- Não foram encontrados outros blocos de código órfão sem L0.
- O directório de trabalho ficou limpo de alterações não explicadas.

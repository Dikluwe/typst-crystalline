# Passo 112 — Relatório: análise para CLI real em `04_wiring`

**Data**: 2026-04-23
**Precondição**: Passo 111 encerrado (formato rico de diagnósticos);
811 L1 + 189 L3 + 6 ignorados; zero violations.
**Natureza**: passo de **análise**. Zero código de produção, zero
ADRs novas, zero testes novos.

---

## Sumário executivo

**`04_wiring/` é stub hoje**: `main.rs` com 11 linhas (só
`println!("typst cristalino — em migração")`). Cargo.toml depende
de `typst-core`, `typst-shell`, `typst-infra`, `anyhow`. Sem
argparsing lib.

**L3 oferece o essencial**:
- `SystemWorld::new(root, main)` — World real, production-ready,
  lê filesystem, gere fontes.
- `export_pdf` / `export_pdf_with_font` — PDF serialization.
- `layout_with_font` — layout com métricas de fonte.
- `discover_fonts` + `build_font_book` — para `--font-path`.
- **Bloqueio organizational**: `do_eval_with_sink`,
  `format_diagnostic`, `drain_diagnostics_to_stderr` são
  test-only (em `#[cfg(test)] mod integration_tests`). CLI precisa
  de os promover a `pub` ou re-implementar inline.
- **Comemo em `[dev-dependencies]`** do `03_infra` — pode precisar
  de mover para `[dependencies]` regular se helpers forem expostos.

**Vanilla CLI tem 9 subcomandos** (4092 linhas, 18 ficheiros):
`compile`, `watch`, `init`, `query`, `eval`, `fonts`, `update`,
`completions`, `info`. Usa **clap derive** + `codespan-reporting`.

**Zero bloqueios arquitecturais** — CLI mínima é viável hoje.

---

## Recomendação primária

**Candidato 2 — "Mínimo com warnings"**.

```bash
typst input.typ output.pdf
```

Output exemplo:

```
input.typ:3:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
```

### Escolhas

| Decisão | Escolha | Razão factual |
|---------|---------|---------------|
| **Escopo** | Mínimo (Candidato 2) | Entrega valor visível (diagnostics gcc/clang via formatter do Passo 111) com zero deps novas. |
| **Argparsing** | Manual (`std::env::args`) | Positional `input output` é suficiente para MVP; clap adicionar-se-ia no Candidato 3. |
| **File loading** | SystemWorld real (L3) | Existe production-ready; sem necessidade de materializar mais L3 antes. |

### Tamanho esperado

- `04_wiring/src/main.rs`: ~120–150 linhas.
- `03_infra/`: ~80 linhas novas (promover `format_diagnostic` +
  `drain_diagnostics_to_stderr` de test-only para `pub`).
- Testes novos: 0 (os 189 L3 existentes cobrem o pipeline).

Comparável a Passo 111 em tamanho. **≤ Passo 109.** ✓

---

## Ranking completo de candidatos

| # | Candidato | Tamanho | Deps novas | Valor | ≤ 109? |
|---|-----------|---------|:---:|:---:|:---:|
| 1 | Micro positional | XS | 0 | Baixo | ✓ |
| 2 | **Mínimo com warnings** | S-M | 0 | **Médio** | **✓** |
| 3 | Subset compile (clap) | M | 1 (clap) | Alto | ✓ |
| 4 | compile + watch | L | 2 (clap, notify) | Alto | ✗ |

Detalhes em
`00_nucleo/diagnosticos/candidatos-cli-passo-112.md`.

---

## Avisos

### Candidato 2 (recomendado)

- **Promover helpers** de `integration_tests.rs` para módulos
  públicos exige decidir onde vivem. Propostas:
  - `03_infra/src/diagnostic_format.rs` (novo) — `format_diagnostic`,
    `drain_diagnostics_to_stderr`.
  - `03_infra/src/pipeline.rs` (novo) — `do_eval_with_sink`,
    `compile_to_pdf`.
  - Ou mantê-los em `lib.rs` se forem poucos.
  Decisão faz parte do Passo 113.
- **`comemo` em `03_infra/Cargo.toml`**: se `do_eval_with_sink`
  for promovido, `comemo` move de `[dev-dependencies]` para
  `[dependencies]`. Custo zero (workspace dep).
- **Alternativa**: manter helpers em test-only e re-implementar
  o boilerplate em `04_wiring/main.rs`. Custo: ~30 linhas de
  duplicação que serão limpas em 113+.

### Candidato 3 (se escolhido)

- `clap` precisa de entrada em `[workspace.dependencies]` e em
  `04_wiring/Cargo.toml`. Escolher versão alinhada com a usada
  por vanilla (visível no Cargo.lock vanilla se necessário).
- `clap_complete` (completions) pode ficar para um passo dedicado
  — não bloqueia compile básico.
- `codespan-reporting` **NÃO** necessário — o Passo 111 já
  cobre formato. Adicionar só se cores/snippets forem prioridade
  (passo dedicado).

### Candidato 4 (se escolhido)

**Não recomendado**. `notify` tem complexidade de threading +
debounce + cross-platform (inotify/FSEvents/ReadDirectoryChangesW).
Dividir em 2 passos (compile → watch).

### Geral

- **`export_pdf_with_font` exige bytes de fonte**. Se o CLI não
  carrega fonte, usa `export_pdf` (Helvetica Type1, Latin-1 only).
  Texto Unicode funciona parcialmente. Decisão: CLI recomendada
  deve tentar `discover_fonts` e cair para Helvetica se
  `--font-path` não passado. Passo 113 decide.
- **Warnings só aparecem se `Sink` drenar**. O padrão do Passo 111
  (`drain_diagnostics_to_stderr` com Source) é reutilizado
  directamente.
- **Errors do `eval()` → formatter**: o formatter do Passo 111
  aceita `&SourceDiagnostic` — cobre warnings e errors
  uniformemente (ADR-0045).

---

## Ficheiros de diagnóstico produzidos

1. `00_nucleo/diagnosticos/inventario-wiring-passo-112.md` —
   04_wiring actual (stub 11 linhas).
2. `00_nucleo/diagnosticos/inventario-l3-apis-passo-112.md` —
   APIs L3 públicas e privadas; zero bloqueios arquitecturais.
3. `00_nucleo/diagnosticos/vanilla-cli-perimetro-passo-112.md` —
   9 subcomandos vanilla; mapa de flags de `compile`.
4. `00_nucleo/diagnosticos/candidatos-cli-passo-112.md` —
   4 candidatos ranqueados com tamanho/deps/valor.

Este relatório agrega.

---

## Verificação

- **Zero** código de produção alterado.
- **Zero** ADRs novas.
- **Zero** testes novos ou removidos.
- `cargo test --workspace`: 811 L1 + 189 L3 + 6 ignorados (inalterado).
- `crystalline-lint .`: zero violations.

---

## Saída deste passo

Relatório **não toma decisão definitiva**. Inputs para conversa:

> **Pergunta directa para o próximo turno**:
> - Candidato 2 (Mínimo com warnings) como Passo 113?
> - Ou Candidato 1 (Micro) primeiro, depois 2 em 114?
> - Ou saltar directo para Candidato 3 (clap)?

Recomendação: **Candidato 2**. Razões detalhadas em
`candidatos-cli-passo-112.md`.

### Trabalho futuro registado

- **Cores ANSI** — passo dedicado depois da CLI básica.
- **PNG/SVG/HTML export** — passos dedicados por formato.
- **Watch** — Candidato 4, passo próprio.
- **Outros subcomandos vanilla** (init, query, eval, fonts,
  update, completions, info) — cada um em passo dedicado se
  aparecer necessidade.

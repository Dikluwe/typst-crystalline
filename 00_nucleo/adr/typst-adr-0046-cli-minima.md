# ADR-0046 — CLI mínima em L4 (compile com diagnostics)

**Estado**: EM VIGOR (Passo 113.E, 2026-04-23)
**Nota Passo 117 (ADR-0049)**: camada corrigida — CLI vive agora
em L2 (`02_shell/`), não em L4. Decisões funcionais deste ADR
(pipeline, exit codes, stderr/stdout discipline) mantêm-se.
**Data**: 2026-04-23
**Autor**: Passo 113
**Revoga**: nenhuma.
**Complementa**: ADR-0043 (canal Sink → L3), ADR-0045 (formato
de diagnósticos), ADR-0017 (L4 como composição).

---

## Contexto

`04_wiring/src/main.rs` era stub desde o Passo 0 — uma função
`main()` que só imprime `"typst cristalino — em migração"`. Até
ao Passo 112, não existia CLI real.

Passos anteriores construíram toda a infra necessária:

- **Passo 104** (ADR-0042) — Sink materializado.
- **Passo 106** (ADR-0043) — canal Sink → L3 com `TrackedMut` caller-managed.
- **Passo 107** — DEBT-49 encerrado; warnings reais emitidos.
- **Passo 109** (ADR-0044) — `Engine<'a>` consolida estado de eval.
- **Passo 111** (ADR-0045) — formato rico `path:linha:coluna: severity: mensagem`.
- **`SystemWorld`** em L3 desde há vários passos — World real,
  production-ready, lê filesystem.

Passo 112 fez análise e recomendou **Candidato 2 — Mínimo com
warnings** (entre 4 candidatos de escopo).

---

## Decisão

CLI em `04_wiring/src/main.rs` com invocação positional:

```bash
typst input.typ output.pdf
```

### Pipeline

```
1. parse args (std::env::args) → (input: PathBuf, output: PathBuf)
2. SystemWorld::new(root=input.parent, main=input)      [L3]
3. world.source(world.main()) -> Source                  [L3]
4. eval_to_module_with_sink(&world, &source)
     -> (SourceResult<Module>, Vec<SourceDiagnostic>)   [L3, promovido]
5. drain_diagnostics_to_stderr(&warnings, &source, path) [L3, promovido]
6. Se Ok(module):
     6a. module.content() → Content                      [L1]
     6b. introspect(content) → CounterState              [L1]
     6c. layout(content, state) → PagedDocument          [L1]
     6d. export_pdf(&doc) → Vec<u8>                      [L3]
     6e. fs::write(output, bytes)                        [L4]
   Senão Err(errors):
     drain_diagnostics_to_stderr(&errors, ...)
     exit 1
```

### Argparsing — manual

**Manual** (`std::env::args`). Razão: `clap` não está em
`[workspace.dependencies]` hoje. Adicionar dep nova para 2 args
positional é desproporcional. Migração para clap fica para o
passo que adicionar flags (`--root`, `--font-path`, etc.).

Formato de ajuda manual:

```
Usage: typst <input.typ> <output.pdf>
```

Sem `--help`, `--version`.

### Helpers promovidos (de test-only para `pub`)

Criar em `03_infra/`:

- **`src/pipeline.rs`** — `eval_to_module_with_sink(&World, &Source)
  -> (SourceResult<Module>, Vec<SourceDiagnostic>)`.
- **`src/diagnostic_format.rs`** — `format_diagnostic`,
  `drain_diagnostics_to_stderr` (ADR-0045 reutilizada).

`comemo` passa de `[dev-dependencies]` para `[dependencies]` em
`03_infra/Cargo.toml` (custo zero: já é workspace dep).

### Exit codes

- **0** — sucesso (output PDF escrito).
- **1** — erro de compilação (`eval` retornou `Err`, ou CLI não
  conseguiu extrair `content`).
- **2** — erro de argumentos ou I/O (argumentos errados, `SystemWorld::new`
  falha, `fs::write` falha).

### Stderr vs stdout

- **Tudo diagnóstico para stderr** (warnings, errors, mensagens
  de uso, erros de I/O).
- **Nada para stdout** — o PDF vai directo para o ficheiro; não
  há suporte a `-` (stdout) neste passo.

---

## Alternativas rejeitadas

### R-1 — Subcomandos desde já

Adicionar `compile`/`watch`/`query`/... cobre muito mais do que
MVP. Passo dedicado para cada (Candidato 3+ do Passo 112).

### R-2 — clap directo

Adicionaria `clap` ao workspace + `clap derive` + ~15 linhas de
setup. Ganho: `--help`/`--version` gratuitos. Custo: decisão
cross-cutting sobre versão/features. Adiado para quando flags
forem necessárias — justifica-se melhor lá.

### R-3 — Flags `--root`/`--output`/`--font-path`

Exigem argparsing declarativo (clap) ou match manual complexo.
Fora do MVP.

### R-4 — Watch (recompilação automática)

Requer `notify` + threading. Passo separado (Candidato 4 do
Passo 112).

### R-5 — Duplicar helpers em `main.rs`

Evitava promoções em L3. Custo: ~50 linhas duplicadas que
divergem no tempo. Passo 113.A confirmou que cadeia de
visibilidade não é um problema — promover é tecnicamente simples.

### R-6 — Cores ANSI / JSON / SARIF

Requerem detecção `isatty`, flags `--color`, deps adicionais
(`termcolor`, `serde`). Passo dedicado.

---

## Limitações aceites

1. **Sem flags**. Argparsing positional. Para compilar com
   paths absolutos ou paths de fontes, esperar por passo que
   adicione `--root` / `--font-path`.
2. **Fonte default**. CLI usa `export_pdf` (sem fonte) — Helvetica
   Type1, Latin-1. Texto Unicode rico parcialmente quebrado. Para
   Unicode completo, passo futuro adiciona `--font-path` +
   `export_pdf_with_font`.
3. **Cross-file diagnostics**. Continuam a mostrar `<detached>`
   para spans de outro ficheiro (limitação herdada de ADR-0045).
4. **Sem `sys.inputs`**. Mecanismo vanilla de injecção de
   variáveis não existe no cristalino.
5. **Sem PNG/SVG/HTML output**. Só PDF.
6. **Windows path separators**. `input.display()` produz `\` em
   Windows no formato `path:linha:coluna`. Editores em Windows
   podem ter dificuldade. Aceitar; passo futuro.

---

## Consequências

### Positivas

1. Utilizador externo compila ficheiros Typst reais:
   `cargo run --release --bin typst -- input.typ output.pdf`.
2. Feedback rico de warnings/errors via formato gcc/clang (Passo
   111). Editores podem parsear.
3. L3 ganha módulos públicos coesos (`pipeline`,
   `diagnostic_format`) — terreno preparado para CLI futura (clap,
   flags, cores, JSON).
4. Testes L3 ganham API pública que podem consumir em vez de
   helpers `#[cfg(test)]`.
5. Padrão estabelecido para L4 — próximos passos de CLI (watch,
   query, etc.) reusam `pipeline` + `diagnostic_format`.

### Negativas

1. 04_wiring/main.rs passa de 11 linhas a ~100–150 linhas.
   Aceitável; L4 é composição (ADR-0017).
2. `03_infra` ganha 2 módulos novos. Custo: ~80 linhas código
   movido + reorganização.
3. `comemo` muda de scope em `03_infra/Cargo.toml` —
   `[dev-dependencies]` → `[dependencies]`.

### Neutras

1. ADR-0043 (canal Sink) e ADR-0045 (formato) intactas.
2. API pública de `eval()`, `layout()`, `export_pdf()` inalterada.
3. Testes existentes continuam a passar (helpers em
   `integration_tests.rs` tornam-se thin wrappers sobre `pub`
   API, ou passam a chamar directamente).

---

## Aplicação

Implementado no Passo 113.C — ver
`00_nucleo/materialization/typst-passo-113-relatorio.md`.

ADR promovida a **EM VIGOR** em 113.E.

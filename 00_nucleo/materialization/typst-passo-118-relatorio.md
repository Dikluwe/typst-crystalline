# Passo 118 — Relatório (auditoria de atribuição de camadas)

**Data**: 2026-04-23
**Precondição**: Passo 117 encerrado; 811 L1 + 6 L2 + 201 L3 + 5 L4
+ 6 ignorados; 1023 total; zero violations.
**Natureza**: auditoria pura. Zero código de produção, zero ADRs
novas, zero testes novos.

---

## Sumário executivo

Auditoria sistemática dos 4 crates revelou:

| Camada | Estado | Candidatos de migração |
|--------|--------|:---:|
| **L1** (`01_core`) | Limpa | 0 |
| **L2** (`02_shell`) | Correcta pós-117 | 0 |
| **L3** (`03_infra`) | 14/18 funções OK; 1 óbvio, 3 fronteiriços | **1 óbvio** + 3 fronteiriços |
| **L4** (`04_wiring`) | Thin (85 linhas) | 0 directos; 1 dependente |

**Diagnóstico**: não há crise. Auditoria detectou apenas **1
correcção pendente** de tamanho XS-S: `format_diagnostic` (+ drain
acoplado) devia ter migrado para L2 no Passo 117 mas ficou para
este passo.

Os 2 fronteiriços em `pipeline.rs` (`eval_to_module_with_sink`,
`compile_to_pdf_bytes`) são composição que esconde boilerplate
comemo. **Recomendação: manter em L3** — padrão "pipeline braçal"
aceite.

---

## Resultados por camada

### L1 (auditoria-l1-passo-118.md)

- **Zero matches** para strings de CLI (`"warning:"`, `"\x1b["`,
  `eprintln!`, etc.).
- 8 `Display` impls encontrados — todos legítimos (PackageSpec,
  semver, tokens, erros de parser).
- **Veredicto**: L1 respeita "zero I/O, zero contexto" — sem
  contaminação.

### L2 (auditoria-l2-passo-118.md)

- Zero I/O (`std::fs`, `std::io::Write`, `print!`, etc.).
- 1 uso legítimo de `std::env::var_os("NO_COLOR")`.
- Zero imports de L3.
- Deps apropriadas: `clap`, `anyhow`, `typst-core` (preparado para uso futuro).
- **Veredicto**: L2 correctamente posicionado pós-117.

### L3 (auditoria-l3-passo-118.md)

Distribuição:

| Classificação | Contagem | Itens |
|---------------|---------:|-------|
| **L3 correcto** | 14 | export_pdf, export_pdf_with_font, process_png_for_pdf, discover_fonts, font_info_from_bytes, build_font_book, layout_with_font, SystemWorld, FontBookMetrics, ImageSizeImageSizer + tipos associados. |
| **Candidato L2** | **1** | `format_diagnostic`. |
| **Fronteiriço** | **3** | `drain_diagnostics_to_stderr`, `eval_to_module_with_sink`, `compile_to_pdf_bytes`. |
| **Candidato L4 puro** | 0 | — |

### L4 (auditoria-l4-passo-118.md)

- **85 linhas total** (~55 úteis). Thin mantido.
- Zero imports de `clap`, zero uso de `std::env::`.
- 4 mensagens `eprintln!` de errors de composição (I/O fails) —
  aceitáveis.
- Cria **zero tipos** — V12 respeitado.
- **Veredicto**: L4 limpo; único "candidato" é dependente de L3
  migrar `format_diagnostic` (Candidato 2 do ranking).

---

## Ranking de candidatos

| # | Candidato | Acção | Tamanho | Bloqueia |
|---|-----------|-------|:-:|:-:|
| 1 | `format_diagnostic` (L3 → L2) | **Migrar** | XS-S | Candidato 2 |
| 2 | `drain_diagnostics_to_stderr` (L3 → inline em L4) | **Migrar com 1** | XS | — |
| 3 | `eval_to_module_with_sink` | **Manter em L3** | S | — |
| 4 | `compile_to_pdf_bytes` | **Manter em L3** | S | — |

Detalhes em
`00_nucleo/diagnosticos/candidatos-migracao-camadas-passo-118.md`.

---

## Recomendação primária

### Passo 119: migrar par **Candidato 1 + Candidato 2**

**Escopo sugerido** (estimado M — comparável a Passo 117):

1. Mover `format_diagnostic` + paleta ANSI (6 const) + testes
   `format_diagnostic_*` de L3 para L2
   (`02_shell/src/cli.rs` ou novo módulo).
2. Remover `drain_diagnostics_to_stderr` de L3 (e a sua invocação
   por `compile_to_pdf_bytes`... wait, `compile_to_pdf_bytes` não
   chama `drain_*`; L4 chama). Inline em L4: 3 linhas de
   `for ... eprint!(format_diagnostic(...))`.
3. Actualizar imports: L4 passa a importar `format_diagnostic` de
   L2.
4. Prompts: `diagnostic_format.md` de L3 pode ser removido ou
   simplificado (fica só com descrição para quando outros
   formatters aparecerem em L3).
5. ADR: criar nova ou anotar ADR-0049 que "completa a migração
   do formatter, começada no 117".

**Testes**: redistribuição ~7 testes (format_diagnostic_*) de L3
para L2. Total workspace inalterado (1023).

### Candidatos 3 e 4: **não migrar**

Documentar em ADR futura (do passo 119 ou dedicada) que o padrão
"pipeline braçal em L3" é aceite — L3 tem dois papéis:
- **I/O bruto** (filesystem, bytes, fonts, export).
- **Composição que esconde infra-estrutura** (comemo, orquestração
  de eval+layout+export).

Move para L4 só se L4 crescer ainda mais thin for desejado — hoje
L4 tem folga (85/100).

---

## Pontos de atenção arquitectural

### Dependência L3 → L2 que surgiria se `drain_*` ficasse em L3

Se Candidato 1 migrar mas Candidato 2 não:
- L3 `drain_diagnostics_to_stderr` importaria
  `typst_shell::cli::format_diagnostic`.
- **Inversão**: L3 depende de L2.
- **Resposta**: tratar par 1+2 como unidade. Migrar juntos.

### Candidatos "grandes" não encontrados

Esperava-se possibilidade de descobrir candidato grande oculto.
**Não aconteceu**. L3 tem 14 funções legítimas; apenas 1 óbvia
misclassificada. L2 limpo, L4 thin. A migração do Passo 117 foi
quase completa — faltou apenas o formatter.

### ADR-0049 parcialmente incompleta

ADR-0049 moveu `ColorWhen` + `resolve_colored_with` para L2 mas
deixou `format_diagnostic` em L3. Esta auditoria revela essa
incompletude. Passo 119 completa a correcção começada em 117.

---

## Ficheiros de diagnóstico produzidos

1. `00_nucleo/diagnosticos/auditoria-l3-passo-118.md` — tabela
   completa de `pub fn` de L3 com classificação.
2. `00_nucleo/diagnosticos/auditoria-l1-passo-118.md` — greps
   de presentation keywords (zero matches).
3. `00_nucleo/diagnosticos/auditoria-l4-passo-118.md` — análise
   linha-a-linha de `main()`; confirmação de thin.
4. `00_nucleo/diagnosticos/auditoria-l2-passo-118.md` — sanity
   check; zero I/O, zero imports de L3.
5. `00_nucleo/diagnosticos/candidatos-migracao-camadas-passo-118.md`
   — ranking detalhado dos 4 candidatos.

Este relatório agrega.

---

## Verificação

- **Zero** código de produção alterado.
- **Zero** ADRs novas.
- **Zero** testes novos ou removidos.
- `cargo test --workspace`: 811 L1 + 6 L2 + 201 L3 + 5 L4 + 6
  ignorados = 1023 (inalterado).
- `crystalline-lint .`: zero violations.

---

## Conclusões

1. **Auditoria sistemática não revelou crise** — 4 passos (113-117)
   deixaram apenas 1 correcção pendente de tamanho pequeno.
2. **L1, L2, L4 estão limpos**. Só L3 tem trabalho.
3. **2 fronteiriços** (`eval_to_module_with_sink`, `compile_to_pdf_bytes`)
   confirmam padrão "pipeline braçal em L3" — não são erros.
4. **Recomendação primária**: Passo 119 migra par
   `format_diagnostic` + `drain_diagnostics_to_stderr`, tamanho
   M, zero mudança de UX, testes redistribuem sem variar total.

### Saída deste passo

Relatório **não toma decisão**. Serve como input para conversa
seguinte:

> **Pergunta para próximo turno**: confirmar Passo 119 migrar par
> Candidato 1+2, ou deixar como está (aceitar `format_diagnostic`
> em L3)?

Recomendação: **migrar**. Razões em
`candidatos-migracao-camadas-passo-118.md`.

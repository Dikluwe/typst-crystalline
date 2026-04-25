# ⚖️ ADR-0050: Formatter de diagnósticos em L2 (completa ADR-0049)

**Status**: `EM VIGOR`
**Completa**: ADR-0049 (CLI em L2) — ADR-0049 ganha nota
"completada por ADR-0050 no Passo 119".
**Não revoga**: nenhuma.
**Validado**: Passo 119.E.
**Data**: 2026-04-23
**Autor**: Passo 119

---

## Contexto

O Passo 117 (ADR-0049) corrigiu parcialmente a localização da CLI:
`ColorWhen`, `resolve_colored_with`, `Args`, `RunIntent` moveram
para L2. Mas `format_diagnostic` + `drain_diagnostics_to_stderr`
+ paleta ANSI ficaram em L3.

O Passo 118 fez auditoria sistemática e confirmou:

- `format_diagnostic` é **candidato óbvio para L2** (decide palavras
  user-facing, aplica escapes ANSI, escolhe indentação — toda
  apresentação).
- `drain_diagnostics_to_stderr` é **candidato dependente** — se
  o formatter migra, o drain fica órfão em L3 dependendo de L2.
- Migrar o par (candidatos 1+2 do ranking) é tamanho XS-S.
- A migração **não cria** crise — só completa um loop deixado
  aberto.

Este passo completa.

---

## Decisão

### L2 ganha `02_shell/src/diagnostic.rs`

Novo módulo com:

- Constantes ANSI (6): `ANSI_RED_BOLD`, `ANSI_YELLOW_BOLD`,
  `ANSI_CYAN_BOLD`, `ANSI_DIM`, `ANSI_BOLD`, `ANSI_RESET`.
- Função pública `format_diagnostic(diag: &SourceDiagnostic,
  source: &Source, source_path: &str, colored: bool) -> String`.
- 7 testes unitários cobrindo sem-cores e com-cores.

### L3 perde `03_infra/src/diagnostic_format.rs`

Ficheiro **removido completamente**:
- `format_diagnostic` → L2.
- `drain_diagnostics_to_stderr` → **eliminado** (inline em L4).
- 7 testes internos → L2.

L3 também perde, em `integration_tests.rs`, os 6 testes
duplicados que chamavam `format_diagnostic`:

- `sink_canal_formato_minimo`, `format_diagnostic_com_multiplos_hints`,
  `format_diagnostic_error_uniforme` — duplicados literais das L2
  unit tests.
- `format_diagnostic_warning_com_ficheiro_linha_coluna`,
  `format_diagnostic_span_detached_usa_fallback`,
  `format_diagnostic_pipeline_debt49` — redundantes com
  `debt49_*` / `sink_canal_*` que assevam `.message` e `.hints`
  directamente.

Total testes removidos em L3: **13** (7 internos + 6 integration).
Total ganhos em L2: **7**. Líquido: **−6** testes (duplicados
eliminados sem perder cobertura).

### L4 inline do drain

`04_wiring/src/main.rs` substitui:

```rust
use typst_infra::diagnostic_format::drain_diagnostics_to_stderr;
// ...
drain_diagnostics_to_stderr(&warnings, &source, &source_path, colored);
drain_diagnostics_to_stderr(&errors, &source, &source_path, colored);
```

Por:

```rust
use typst_shell::diagnostic::format_diagnostic;
// ...
fn drain_to_stderr(diags: &[SourceDiagnostic], source: &Source, path: &str, colored: bool) {
    for diag in diags {
        eprint!("{}", format_diagnostic(diag, source, path, colored));
    }
}

// em main:
drain_to_stderr(&warnings, &source, &source_path, colored);
drain_to_stderr(&errors, &source, &source_path, colored);
```

Helper local **`fn drain_to_stderr`** (3 linhas de corpo) evita
duplicação do loop. **Não cria tipo** (regra V12 OK). Função
privada de composição — aceitável em L4.

### Paleta ANSI em L2

As 6 constantes ANSI movem com o formatter (único consumidor).
Se algum dia L3 precisar de ANSI (improvável — L3 produz bytes
PDF, não terminal output), duplicar 5 linhas é custo desprezível.

---

## Relação com ADR-0049

ADR-0049 declarou migração de CLI para L2 mas não enumerou
todas as peças (o scope focou em `Args`, `ColorWhen`,
`RunIntent`). ADR-0050 **completa** o que 0049 começou:

| Peça | Passo que migrou | ADR |
|------|-----------------|-----|
| `ColorWhen` enum | 117 | 0049 |
| `resolve_colored_with` | 117 | 0049 |
| `Args` / `parse()` / `RunIntent` | 117 | 0049 |
| `format_diagnostic` + paleta | **119** | **0050** |
| `drain_diagnostics_to_stderr` | **119 (eliminado)** | **0050** |

ADR-0049 ganha nota:
> **Nota do Passo 119 (ADR-0050)**: migração completada.
> `format_diagnostic` e paleta ANSI movidos para L2; drain
> eliminado (inline em L4).

---

## Alternativas rejeitadas

### R-1 — Manter em L3

Rejeitada. Auditoria 118 classificou como Candidato L2 óbvio.
Manter contradiz ADR-0049 (L2 = apresentação user-facing) e
cria inversão lógica (L3 produz formato para terminal).

### R-2 — `drain_*` em L2 com `eprint!`

Rejeitada. L2 pós-117 não escreve I/O (só lê `std::env::var_os`
para `NO_COLOR`). Adicionar `eprint!` em L2 degrada princípio
"L2 não escreve".

### R-3 — Mover todas as 6 testes L3 integration para L2 ou L4

Considerada e **parcialmente rejeitada**. Os testes são
**duplicados** do que L2 unit tests já cobrem (format) + L3
`debt49_*` já cobre (message/hints). Mover sem dedup preserva
duplicação. Deletar é mais honesto.

### R-4 — Helper `drain_to_stderr` como `pub fn` em L2

Rejeitada. L2 `diagnostic::drain_to_stderr` faria `eprint!` em
L2. Helper privado em L4 é consistente com "L2 não escreve".

### R-5 — Manter paleta ANSI em L3 como "infra de formatação"

Rejeitada. A paleta só serve ao formatter; migrá-la para L2
junto com o formatter preserva coesão (constantes + função
juntas). Se L3 precisar de ANSI no futuro, decide pontualmente.

---

## Limitações aceites

1. **Duplicação de ANSI** se L3 futuro precisar. Custo: 5 linhas
   de `const`. Aceitável.
2. **Testes perdidos**: 6 duplicados removidos. Cobertura real
   inalterada (L2 tem 7 + 6 unit tests, L3 tem debt49_* que
   asseve `.message`).
3. **`drain_to_stderr` privado em L4**: função interna, não
   reutilizável por outros crates. Se algum dia outro caller
   precisar, promover para L2 ou aceitar cópia. Improvável.

---

## Consequências

### Positivas

1. Arquitectura completa: L2 = CLI (args + cores + diagnostics);
   L3 = I/O puro; L4 = composição.
2. `diagnostic_format.rs` em L3 **desaparece** — simplifica estrutura.
3. Paleta ANSI + formatter juntos (coesão).
4. Zero inversão de dependência (L3 já não importa L2).

### Negativas

1. Migração toca 5 ficheiros (novo L2 + remover L3 + 3 prompts).
2. Testes totais workspace caem de 1023 para 1017 (duplicados).
3. ADR-0049 ganha anotação (custo documental).

### Neutras

1. **Zero mudança de UX**. `typst` binário idêntico: mesmas
   mensagens, mesmos exit codes, mesmo comportamento.
2. Tests 114 passam sem modificação.
3. L4 cresce ~4 linhas (helper `drain_to_stderr`).

---

## Aplicação

Implementado no Passo 119.C — ver
`00_nucleo/materialization/typst-passo-119-relatorio.md`.

ADR promovida a **EM VIGOR** em 119.E.

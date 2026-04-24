# Passo 119 — Completar migração do formatter para L2

**Série**: 119 (correcção arquitectural; completa a migração
iniciada no Passo 117).
**Precondição**: Passo 118 encerrado (auditoria completa); 811 L1
+ 6 L2 + 201 L3 + 5 L4 + 6 ignorados; zero violations. Total 1023.
**ADRs aplicáveis**: ADR-0049 (CLI em L2).
**ADR nova**: ADR-00NN "Migração do formatter para L2 — completa
o Passo 117" — `PROPOSTO` em 119.B, `EM VIGOR` em 119.E.

---

## Natureza deste passo

Correcção arquitectural que **completa** o Passo 117. O 117
migrou `ColorWhen` + `resolve_colored_with` + `Args` para L2; mas
deixou `format_diagnostic` + paleta ANSI + `drain_diagnostics_to_stderr`
em L3. O Passo 118 confirmou factualmente que isso foi
inacabado — `format_diagnostic` é candidato **óbvio** para L2, e
`drain_*` é candidato dependente.

Este passo fecha o loop. Zero funcionalidade nova, zero mudança
de UX observável, redistribuição de código entre camadas.

---

## Objectivo

Ao fim do passo:

| Camada | Ganha | Perde |
|--------|-------|-------|
| **L2** | `02_shell/src/diagnostic.rs` novo (formatter + paleta ANSI + 7 testes) | — |
| **L3** | — | `format_diagnostic`, paleta ANSI, `drain_diagnostics_to_stderr`, 7 testes. O ficheiro `diagnostic_format.rs` desaparece ou fica vazio. |
| **L4** | Inline do drain loop (3-4 linhas em `main`) | Import de `drain_diagnostics_to_stderr` |

L3 elimina função que criava fronteira errada (apresentação
escondida como "infra"). L4 ganha 3 linhas de loop trivial —
aceitável como composição.

---

## Decisões já tomadas

1. **Novo ficheiro `02_shell/src/diagnostic.rs`** — separa
   concerns de argparsing (`cli.rs`) e formatação (`diagnostic.rs`).
2. **Paleta ANSI migra junto** — 6 constantes em L2, junto ao
   único consumidor. Se outra parte de L3 precisar de ANSI no
   futuro, cada caso decidirá (duplicar 5 linhas ou re-exportar).
3. **ADR nova** — marca fecho da migração. ADR-0049 fica anotada
   como "completada por ADR-00NN no Passo 119".
4. **Drain inline em L4** — `for diag in ... { eprint!("{}",
   format_diagnostic(...)); }`. 3 linhas, zero cerimónia.
5. **L3 perde dep de `diagnostic_format`** — re-exports em
   `03_infra/src/lib.rs` removidos.

---

## Escopo

**Dentro**:
- Criar `02_shell/src/diagnostic.rs` (copiar conteúdo de L3).
- Actualizar `02_shell/src/lib.rs` (`pub mod diagnostic;`).
- `02_shell/Cargo.toml` — verificar que `typst-core` é
  realmente usado (para `SourceDiagnostic`, `Severity`,
  `Source`, `Span`). Se sim, já declarado. Se não, adicionar.
- Remover `03_infra/src/diagnostic_format.rs`.
- `03_infra/src/lib.rs` — remover `pub mod diagnostic_format;`.
- `03_infra/src/integration_tests.rs` — actualizar imports se
  os testes chamavam `format_diagnostic` directamente.
- `04_wiring/src/main.rs` — substituir `use typst_infra::diagnostic_format::drain_diagnostics_to_stderr`
  por `use typst_shell::diagnostic::format_diagnostic` (ou
  equivalente); inline o loop.
- Prompts L0: criar `00_nucleo/prompts/shell/diagnostic.md`;
  remover/arquivar `00_nucleo/prompts/infra/diagnostic_format.md`;
  actualizar `shell.md`, `wiring.md`.
- ADR-0049 ganha nota "completada por ADR-00NN".

**Fora**:
- Candidatos 3 e 4 do Passo 118 (`eval_to_module_with_sink`,
  `compile_to_pdf_bytes`) — ficam em L3 conforme recomendação.
- Qualquer funcionalidade nova.
- Mudança em L1.
- Mudança no comportamento observável.

---

## Sub-passos

### 119.A — Inventário rápido

**Parte 1 — Conteúdo exacto a mover**:

1. `view` em `03_infra/src/diagnostic_format.rs`. Registar:
   - Número total de linhas.
   - Imports usados (`use ...`).
   - Funções públicas exactas.
   - Constantes (paleta ANSI).
   - Testes `#[cfg(test)]`.
2. `grep` por `diagnostic_format::` em `03_infra/src/`,
   `04_wiring/src/`, `04_wiring/tests/`. Listar call sites a
   actualizar.

**Parte 2 — Deps de L2**:

1. `view` em `02_shell/Cargo.toml`. Confirmar `typst-core` já
   declarado.
2. Se não, adicionar.
3. Registar se `typst-core` é activo ou declaração dormente
   (na auditoria 118 era declarado mas não usado).

**Parte 3 — Integration tests em L3**:

1. `view` em `03_infra/src/integration_tests.rs`. Se `format_diagnostic`
   é chamado lá, call sites têm de migrar para import de L2 ou
   os testes movem para L2.
2. Decidir:
   - **(a)** Call sites em L3 integration tests passam a
     importar de L2. Mas L3 importar L2 é inversão — **rejeitar**.
   - **(b)** Testes que usam `format_diagnostic` movem para L2.
     Mas são testes de integração end-to-end que também
     exercitam pipeline — muito invasivo.
   - **(c)** Re-escrever esses testes em L3 sem usar
     `format_diagnostic` — testar só valores (warnings contados,
     não formato).
   - **(d)** Mover testes para L4 (`04_wiring/tests/`) onde já há
     access a L2 + L3 — cada um fica na camada certa.
3. Recomendação: **(c) ou (d)** conforme o que os testes
   asseveram. Se asserem "warning tem mensagem X", (c) funciona
   (asserir `warning.message`). Se asserem "output contém X",
   (d) com binário.

**Escrever** em
`00_nucleo/diagnosticos/inventario-migracao-formatter-passo-119.md`:

```
diagnostic_format.rs: N linhas
  pub fn / pub const: [...]
  testes: [...]
  
Call sites:
  04_wiring/src/main.rs:N — drain_diagnostics_to_stderr
  03_infra/src/integration_tests.rs:M — format_diagnostic direct
  ...

Decisão testes L3 integration: (c) ou (d)
```

**Gate 119.A**: se os testes em L3 integration usam
`format_diagnostic` em ≥ 5 sítios e (c) é invasivo (exige
re-escrever asserts), considerar mover o `diagnostic.rs` para L2
**mas deixar testes de formato em L3 via re-importação**.
Isto é inversão de dependência, mas só em testes — fronteira
aceitável. **Documentar decisão**.

### 119.B — ADR nova

Criar `00_nucleo/adr/typst-adr-00NN-formatter-em-l2.md` com
`PROPOSTO`.

Conteúdo:

- **Contexto**: Passo 117 migrou parte da CLI para L2 mas
  deixou `format_diagnostic` em L3. Passo 118 auditoria
  confirmou que `format_diagnostic` é candidato claro L2 e
  que `drain_*` é dependente. Este passo completa a migração.
- **Decisão**:
  - `format_diagnostic` + paleta ANSI + `drain_diagnostics_to_stderr`
    são movidos de L3 para L2 (novo `02_shell/src/diagnostic.rs`).
  - L3 perde o ficheiro `diagnostic_format.rs` completamente.
  - L4 inline o drain loop (3-4 linhas) — evita criar `drain_*`
    em L2 que faria I/O.
- **Relação com ADR-0049**: ADR-0049 declarou "CLI em L2 —
  migração parcial". Este passo **completa**. ADR-0049 ganha
  nota "completada por ADR-00NN no Passo 119".
- **Alternativas rejeitadas**:
  - **Manter em L3**: contradiz auditoria 118 (candidato óbvio).
  - **`drain_*` em L2 com `eprint!`**: L2 não deve fazer I/O
    de escrita (regra pós-117). Drain em L4 é consistente.
  - **Formatter + drain em L4**: concentra apresentação em L4;
    perde testabilidade; L4 cresce.
- **Nova estrutura L2**:
  ```
  02_shell/src/
    lib.rs        (pub mod cli; pub mod diagnostic;)
    cli.rs        (Args, ColorWhen, RunIntent, parse, resolve_colored_with)
    diagnostic.rs (ANSI_*, format_diagnostic, 7 testes)
  ```
- **Limitações documentadas**:
  - Se L3 alguma vez precisar de ANSI fora do contexto CLI
    (improvável), cada caso decidirá (duplicar 5 linhas vs
    dep L3 → L2 em traits).

Promover em 119.E.

### 119.C — Implementação

Ordem obrigatória.

**119.C.1 — Criar `02_shell/src/diagnostic.rs`**:

Copiar conteúdo relevante de `03_infra/src/diagnostic_format.rs`:

- Imports: `Source`, `SourceDiagnostic`, `Severity`, `Span`
  (de `typst-core`).
- Constantes ANSI (6 `const &str`).
- Função pública `format_diagnostic(diag, source, path, colored) -> String`.
- Bloco `#[cfg(test)] mod tests` com 7 testes `format_diagnostic_*`.

**Não copiar** `drain_diagnostics_to_stderr` — será inline em L4.

Header cristalino:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/diagnostic.md
//! @prompt-hash <novo>
//! @layer L2
//! @updated 2026-04-23
```

**119.C.2 — Actualizar `02_shell/src/lib.rs`**:

```rust
pub mod cli;
pub mod diagnostic;
```

**119.C.3 — Verificar `02_shell/Cargo.toml`**:

Confirmar `typst-core = { path = "../01_core" }`. Se declarado
mas não activado (auditoria 118 diz que não era consumido), o
novo `diagnostic.rs` activa. Zero mudança no `Cargo.toml`.

**119.C.4 — Remover `03_infra/src/diagnostic_format.rs`**:

1. `rm` do ficheiro.
2. Em `03_infra/src/lib.rs`, remover `pub mod diagnostic_format;`.
3. Em `03_infra/src/integration_tests.rs`, actualizar imports
   conforme decidido em 119.A Parte 3.

**119.C.5 — Actualizar `04_wiring/src/main.rs`**:

Antes:

```rust
use typst_infra::diagnostic_format::drain_diagnostics_to_stderr;
// ...
drain_diagnostics_to_stderr(&warnings, &source, &source_path, colored);
// ...
drain_diagnostics_to_stderr(&errors, &source, &source_path, colored);
```

Depois:

```rust
use typst_shell::diagnostic::format_diagnostic;
// ...
for diag in &warnings {
    eprint!("{}", format_diagnostic(diag, &source, &source_path, colored));
}
// ...
for diag in &errors {
    eprint!("{}", format_diagnostic(diag, &source, &source_path, colored));
}
```

L4 cresce ~4 linhas (2 loops × 2 linhas cada — ou helper local
de 4 linhas se quiser evitar duplicação do loop).

**Alternativa helper local em L4** (se o loop for duplicado):

```rust
fn drain_to_stderr(diagnostics: &[SourceDiagnostic], source: &Source, path: &str, colored: bool) {
    for diag in diagnostics {
        eprint!("{}", format_diagnostic(diag, source, path, colored));
    }
}
```

Função privada em L4 é aceitável — 3 linhas de composição, não
cria tipo (V12 OK).

**119.C.6 — Prompts L0**:

1. Criar `00_nucleo/prompts/shell/diagnostic.md` com descrição:
   - Formatter de `SourceDiagnostic` para saída em terminal.
   - Aceita `colored: bool`.
   - Paleta ANSI embutida.
   - Testes determinísticos.
2. Remover ou arquivar `00_nucleo/prompts/infra/diagnostic_format.md`.
3. Actualizar `00_nucleo/prompts/shell.md` — menciona
   `diagnostic` submodule junto de `cli`.
4. Actualizar `00_nucleo/prompts/wiring.md` — descreve drain inline.

`crystalline-lint --fix-hashes .` regenera hashes.

**119.C.7 — Compilação incremental**:

Após 119.C.1–119.C.5:

```bash
cargo check -p typst-shell
cargo check -p typst-infra
cargo check -p typst-wiring
cargo build --release
```

Cada comando deve passar antes do próximo.

### 119.D — Testes

**Redistribuição**:

- L2 ganha 7 testes `format_diagnostic_*` (de L3).
- L3 perde 7 testes.
- L1/L4 inalterado (5 testes L4 usam `typst` binário, que
  internamente passa pela migração mas não testa formatter
  directamente).

Contagem esperada:

| Crate | Antes | Depois | Δ |
|-------|------:|-------:|---|
| L1 | 811 | 811 | 0 |
| L2 | 6 | **13** | +7 |
| L3 | 201 | **194** | −7 |
| L4 | 5 | 5 | 0 |
| **Total** | **1023** | **1023** | 0 |

Se 119.A Parte 3 decidir (c) ou (d), alguns testes de L3 podem
mudar de forma (asserem valores em vez de formato) — nesse caso
contagem L3 pode ter variação pequena. **Mas total workspace ≥
1023**.

### 119.E — Encerramento

1. `cargo build --release` passa.
2. `cargo test --workspace` — total ≥ 1023, distribuição
   esperada.
3. `crystalline-lint` zero violations. Confirmar que V12 **não
   dispara** em L2 nem L4.
4. ADR-00NN promovida a `EM VIGOR`.
5. ADR-0049 anotada com "completada por ADR-00NN no Passo 119".
6. Validação manual:
   - `typst input.typ out.pdf` produz warnings/errors no mesmo
     formato do Passo 117.
   - `--color=always|never` funciona.
   - `NO_COLOR` funciona.
   - Tests 114 passam sem modificação.
7. Relatório `typst-passo-119-relatorio.md`:
   - Decisão sobre testes L3 (119.A Parte 3).
   - Tamanho real de cada ficheiro.
   - Diff das deps.
   - L4 drain: inline directo ou helper local.
   - Limitações aceites.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 119.A escrito.
2. ADR-00NN criada e promovida.
3. `02_shell/src/diagnostic.rs` criado e funcional.
4. `03_infra/src/diagnostic_format.rs` removido.
5. `04_wiring/src/main.rs` faz drain inline (ou helper local).
6. Prompts L0 actualizados com hashes correctos.
7. `cargo build --release` passa.
8. `cargo test --workspace` ≥ 1023 testes, distribuição correcta.
9. `crystalline-lint` zero violations.
10. Validação manual: binário comporta-se idêntico ao Passo 117.
11. Relatório 119.E escrito.

---

## O que pode sair errado

- **Integration tests em L3 tocam `format_diagnostic`**: gate
  119.A Parte 3. Opções (c) re-escrever ou (d) mover para L4.
  Se qualquer delas for > 10 linhas de mudança, passo inflat —
  considerar adiar decisão e documentar como dívida.
- **L3 fica sem re-exports esperados**: call sites fora de L3
  (L4, tests) podem ter imports `typst_infra::diagnostic_format::...`
  que já não existem. Grep exaustivo em 119.A.
- **L4 cresce mais que esperado**: drain inline é 3-4 linhas; se
  ficar ≥ 10 linhas, algo escapou do escopo — reavaliar.
- **`02_shell/Cargo.toml` precisa de ajuste**: auditoria 118 diz
  que `typst-core` estava declarado mas não usado. Activar pelo
  `diagnostic.rs` deve funcionar, mas se houver conflito de
  versões/features, ajustar.
- **Prompts L0 ficam órfãos**: se `00_nucleo/prompts/infra/diagnostic_format.md`
  for apagado, algum ficheiro em L3 pode ter `@prompt-hash`
  apontando para ele — V7 do linter dispara. Grep antes de
  apagar.
- **Regressão visual**: se o formato muda por engano (ex:
  troca ordem de campos), testes 114 podem falhar. Output é
  testado em `cli_sucesso_com_warning` via `.contains("warning:")`
  + `.contains("font")` — asserts resistentes a mudanças
  pequenas de formato mas não a mudanças radicais.
- **`Severity` visibility**: se `SourceDiagnostic.severity` é
  `pub(crate)` em L1 (em vez de `pub`), o formatter em L2 não
  consegue fazer match. Auditoria 118 não confirmou explicitamente
  — verificar em 119.A.

---

## Notas operacionais

- Este passo **fecha** uma lacuna deixada no Passo 117. Padrão
  importante: correcções arquitecturais podem vir em múltiplos
  passos, cada um com âmbito honesto.
- ADR nova é preferível a "estender ADR-0049" porque o âmbito é
  distinto (117 = CLI args; 119 = formatter). Cada ADR tem
  contexto factual próprio.
- L4 inline vs helper local: ambos aceitáveis. Se o loop é
  usado em 2 sítios (warnings e errors), helper local evita
  duplicação. Decisão menor; escolher o que lê melhor em contexto.
- Paleta ANSI em L2 junto do formatter: se algum dia L3 precisar
  de ANSI (por exemplo, para export de terminal-friendly output),
  paleta duplica-se. Custo: 5 linhas. Aceitável.
- O Passo 118 estabeleceu que os 2 fronteiriços (`eval_to_module_with_sink`,
  `compile_to_pdf_bytes`) ficam em L3. Este passo não os toca —
  manter disciplina.
- Se depois de 119 surgir necessidade de refactor adicional em L3
  (ex: o módulo `pipeline.rs` ficar grande demais), passo dedicado.
  Este passo é só sobre o formatter.

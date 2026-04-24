# Passo 118.B — Auditoria de L1

**Data**: 2026-04-23
**Objectivo**: verificar que L1 não tem contaminação de
apresentação (CLI, cores, stdio).

---

## Greps executados

### Strings de formatação CLI

```
grep '"warning:"|"error:"|"hint:"|\\x1b\[|ANSI|stderr|stdout'
  em 01_core/src/
```

**Resultado**: **zero matches**.

### I/O direct calls

```
grep 'eprintln|println|eprint!|print!' em 01_core/src/
```

**Resultado**: **zero matches**.

### `Display` impls

```
grep 'impl.*Display.*for' em 01_core/src/
```

**Resultado**: 8 matches em 2 ficheiros:

| Ficheiro:linha | Tipo | Legitimidade |
|----------------|------|--------------|
| `package_spec.rs:31` | `PackageSpec` | **Legítima** — representação textual de tipo de domínio (`@preview/package:1.0.0`). |
| `package_spec.rs:51` | `VersionlessPackageSpec` | **Legítima** — idem. |
| `package_spec.rs:65` | `PackageVersion` | **Legítima** — semver display. |
| `package_spec.rs:75` | `PackageSpecError` | **Legítima** — erro do parser, usado em `thiserror` derive. |
| `package_spec.rs:136` | `PackageVersionError` | **Legítima** — idem. |
| `package_spec.rs:192` | `VersionBound` | **Legítima** — operador `^`, `~`, etc. |
| `syntax_node.rs:962` | `Unnumberable` | **Legítima** — erro do parser/numberer. |
| `syntax_text.rs:63` | `SyntaxText` | **Legítima** — representação textual de token. |

Todos são `Display` para tipos de **domínio** (semver, tokens,
erros de parser). Nenhum produz formato user-facing de CLI.

---

## Análise

### Mensagens de diagnóstico (`SourceDiagnostic::warning(...)`)

Strings literais como `"propriedade 'font' ainda não suportada"`
em L1 são **conteúdo** do `SourceDiagnostic`, não formatação.
`format_diagnostic` em L3/L2 é quem aplica `"warning:"` em volta.
Legítimo que L1 produza conteúdo semântico (sabe qual o problema);
formatação é camada externa.

### Trace via `Debug`

L1 usa `#[derive(Debug)]` em muitos tipos — é representação de
desenvolvedor, não user-facing. Legítima.

### Falsos positivos candidatos

- Strings `"unknown variable"`, `"cannot find"`, etc. em
  `rules/eval/` — **conteúdo de `SourceDiagnostic`**, não
  formatação CLI.
- `Span::detached()` — sentinel semântico, não "detached" como
  palavra de UI.
- `impl Display for SyntaxText` — representação textual do token.
  Se usada em CLI, a camada CLI escolhe usar; L1 só disponibiliza.

---

## Conclusão

**L1 limpo — zero candidatos de migração para L2.**

A ausência de `eprintln!`, `println!`, strings como `"warning:"`,
escapes ANSI, ou qualquer referência a stderr/stdout confirma que
L1 respeita a regra "zero I/O, zero contexto user-facing".

Os 8 `Display` impls encontrados são todos para tipos de domínio
(legítimos em L1 per ADR-0033 — paridade funcional com vanilla).

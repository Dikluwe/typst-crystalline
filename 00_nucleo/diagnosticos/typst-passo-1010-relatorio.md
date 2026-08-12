# Passo 1010 — Relatório final

**Data**: 2026-08-12  
**Commit de base**: `d0a0063f0` (fix(P1009): remove if/else redundante na criação do history de full_error)  
**Ficheiro alterado**:

- `00_nucleo/diagnosticos/typst-passo-1008-relatorio.md` — adenda corrigindo a evidência de fan-in de `compiler::eval::bindings`

---

## Resumo

O Passo 1008 citava `long_type_name` como o símbolo mais referenciado de `compiler::eval::bindings`, com "36 usos" em 7 ficheiros. A contagem de "36 usos" não foi reproduzida, e a distinção entre ocorrências nominais do nome e usos reais de `bindings::long_type_name` não estava clara.

Este passo mediou os usos reais e actualizou o relatório do P1008 com:

- `long_type_name` é o **único** símbolo `pub(crate)` exportado por `bindings.rs`.
- O nome `long_type_name` ocorre em **7 ficheiros** fora de `bindings.rs`, mas dois deles (`compiler/eval/operators/join.rs` e `compiler/stdlib/foundations.rs`) definem a sua própria função privada homónima; essas ocorrências não são usos de `bindings::long_type_name`.
- Os **usos reais** de `bindings::long_type_name` (chamadas ou reexportação/importação provenientes de `bindings.rs`) são **11 chamadas** distribuídas por **5 ficheiros**:
  - `compiler/eval/mod.rs` — reexporta e faz 2 chamadas qualificadas;
  - `compiler/eval/closures.rs` — 2 chamadas qualificadas;
  - `compiler/stdlib/eval.rs` — importa e faz 3 chamadas;
  - `compiler/stdlib/figure_image.rs` — 1 chamada qualificada;
  - `compiler/stdlib/plugin.rs` — importa e faz 3 chamadas.

## Conclusão

A conclusão qualitativa do P1008 mantém-se: `bindings.rs` é um agregador real de acesso/destruturação usado internamente por `eval`, e o fan-in não vem de uma interface (`trait`/tipo). O veredicto **Prosseguir para P1002-completo** permanece; apenas a célula de evidência numérica foi corrigida.

---

## Validação

Não houve alterações de código. O relatório foi actualizado e a contagem foi verificada com:

```bash
grep -rn "\blong_type_name\b" 01_core 02_shell 03_infra 04_wiring --include=*.rs | grep -v "compiler/eval/bindings.rs"
```

Seguido de inspecção manual para distinguir definições locais homónimas de usos qualificados de `bindings::long_type_name`.

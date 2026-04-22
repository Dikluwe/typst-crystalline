# Diagnóstico prévio à ADR-0023

**Contexto**: registo do diagnóstico executado antes da decisão
formalizada em `00_nucleo/adr/typst-adr-0023-indexmap.md`.

**Data do diagnóstico**: 2026-03-28 (obtida por `git blame` ao
cabeçalho original da secção movida).

**Natureza**: este ficheiro é registo histórico. Os comandos
abaixo foram executados antes da decisão do ADR-0023 ser tomada;
o estado do código pode ter mudado desde então. Consultar este
ficheiro para entender o contexto factual da decisão, não para
re-executar.

---

```bash
# Dependências externas de scope.rs
grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/foundations/scope.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Confirmar que IndexMap usa FxBuildHasher
grep -n "IndexMap\|FxBuildHasher\|rustc_hash" \
  lab/typst-original/crates/typst-library/src/foundations/scope.rs \
  | head -10

# Versão de indexmap usada no original
grep "indexmap" lab/typst-original/Cargo.toml
grep "indexmap" lab/typst-original/crates/typst-library/Cargo.toml
```

**Reportar output antes de continuar.**

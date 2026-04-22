# Diagnóstico prévio à ADR-0022

**Contexto**: registo do diagnóstico executado antes da decisão
formalizada em `00_nucleo/adr/typst-adr-0022-fontbook.md`.

**Data do diagnóstico**: 2026-03-28 (obtida por `git blame` ao
cabeçalho original da secção movida).

**Natureza**: este ficheiro é registo histórico. Os comandos
abaixo foram executados antes da decisão do ADR-0022 ser tomada;
o estado do código pode ter mudado desde então. Consultar este
ficheiro para entender o contexto factual da decisão, não para
re-executar.

---

```bash
# Estrutura de FontBook e FontInfo
grep -n "^pub struct\|^pub enum\|^pub fn\|^impl " \
  lab/typst-original/crates/typst-library/src/text/font/book.rs \
  | head -40

# Campos de FontInfo — são primitivos?
grep -A 30 "pub struct FontInfo" \
  lab/typst-original/crates/typst-library/src/text/font/book.rs

# Dependências externas de book.rs
grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/text/font/book.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Como FontBook é construído a partir de uma face ttf_parser
grep -n "push\|insert\|from_face\|ttf_parser\|Face" \
  lab/typst-original/crates/typst-library/src/text/font/book.rs \
  | head -20

# Métodos de pesquisa usados pelo engine
grep -rn "\.book()\.\|FontBook::\|select_font\|select_fallback" \
  lab/typst-original/crates/typst-library/src/text/ | head -30
```

**Reportar output antes de continuar.**

---

# P517 — Activar System Fonts por Defeito na CLI + Marcação de Subset no PDF

> **Passo:** 517
> **Data:** 2026-06-30
> **Foco:** Activar a descoberta de fontes do sistema (`with_system_fonts`) por defeito na CLI do cristalino, e adicionar a marcação de subset (`AAAAAA+` prefix) aos nomes de fontes no PDF gerado. Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação S/XS-size com validação via CLI e PDF.
> **Tamanho:** S (~30 min de implementação + 15 min de validação).
> **ADR-0107 ACEITE** — paridade é de linguagem, não de mecânica; estas são melhorias de produção.
> **ADR-0108 ACEITE** — medir antes de decidir.
> **ADR-0109 ACEITE** — atomização de código.
> **ADR-0115 ACEITE** — infra de benchmark.
> **Dependências:** P516 (subsetting TrueType implementado), P515 (fontdb + font fallback implementados).

---

## 1. Contexto

O P516 fechou o subsetting de fontes TrueType no PDF. No entanto, a CLI do cristalino **não activa `with_system_fonts` por defeito** — o usuário precisa passar `--with-system-fonts` explicitamente. No Typst vanilla 0.15.0, a descoberta de fontes do sistema é **sempre activa**.

Além disso, o PDF gerado pelo cristalino **não marca fontes subsetadas** com o prefixo `AAAAAA+` (ex: `AAAAAA+DejaVuSans`). No Typst vanilla, este prefixo é adicionado automaticamente para indicar que a fonte é um subset.

Este passo fecha estas duas lacunas de usabilidade/produção.

---

## 2. Sub-tarefa 517a — Activar `with_system_fonts` por Defeito

### 2.1 Estado Baseline

```bash
# Cristalino (pré-517):
target/release/typst compile doc.typ out.pdf
# Se o documento usar uma fonte do sistema (ex: "DejaVu Sans"):
# ERRO: "font family 'DejaVu Sans' not found"
# Precisa: target/release/typst --with-system-fonts compile doc.typ out.pdf
```

### 2.2 Implementação

**Arquivo alvo:** `src/infra/cli.rs` (ou equivalente onde a CLI é definida)

```rust
// Antes (pré-517):
fn compile(args: CompileArgs) -> Result<(), Error> {
    let mut ctx = CompileContext::new();
    if args.with_system_fonts {
        ctx.font_db.load_system_fonts();
    }
    // ...
}

// Depois (pós-517):
fn compile(args: CompileArgs) -> Result<(), Error> {
    let mut ctx = CompileContext::new();
    // Sempre carregar fontes do sistema (comportamento vanilla)
    ctx.font_db.load_system_fonts();
    // ...
}
```

**Nota:** A flag `--with-system-fonts` pode ser mantida para compatibilidade (no-op), ou removida. Recomenda-se mantê-la como no-op para não quebrar scripts existentes.

### 2.3 Validação

```bash
# Teste 1: Documento com fonte do sistema (sem flag explícita)
echo '#set text(font: "DejaVu Sans")
Hello World' > /tmp/test-system-font.typ

# Antes (pré-517):
target/release/typst compile /tmp/test-system-font.typ /tmp/out.pdf
# Esperado: ERRO (fonte não encontrada)

# Depois (pós-517):
target/release/typst compile /tmp/test-system-font.typ /tmp/out.pdf
# Esperado: OK (fonte encontrada automaticamente)
```

---

## 3. Sub-tarefa 517b — Marcação de Subset no PDF (`AAAAAA+` Prefix)

### 3.1 Estado Baseline

```bash
# PDF gerado pelo cristalino (pré-517):
pdffonts /tmp/out.pdf
# name                                 type              emb sub uni object ID
# ------------------------------------ ----------------- --- --- --- ---------
# DejaVuSans                           TrueType          yes yes yes      5  0

# PDF gerado pelo vanilla 0.15.0:
pdffonts /tmp/vanilla.pdf
# name                                 type              emb sub uni object ID
# ------------------------------------ ----------------- --- --- --- ---------
# AAAAAA+DejaVuSans                    TrueType          yes yes yes      5  0
# ^^^^^^^^ prefixo de subset
```

### 3.2 Implementação

**Arquivo alvo:** `src/infra/pdf/font_embed.rs` (ou onde o nome da fonte é definido no PDF)

```rust
// Antes (pré-517):
fn embed_font_subset(
    writer: &mut PdfWriter,
    font_id: fontdb::ID,
    font_name: &str,
    subset_data: &[u8],
) -> Ref {
    let font_ref = writer.alloc_ref();
    let mut font = writer.indirect(font_ref).start::<Dict>();
    font.pair(Name(b"Type"), Name(b"Font"));
    font.pair(Name(b"Subtype"), Name(b"TrueType"));
    font.pair(Name(b"BaseFont"), Name(font_name.as_bytes())); // "DejaVuSans"
    // ...
}

// Depois (pós-517):
fn embed_font_subset(
    writer: &mut PdfWriter,
    font_id: fontdb::ID,
    font_name: &str,
    subset_data: &[u8],
) -> Ref {
    let font_ref = writer.alloc_ref();
    let mut font = writer.indirect(font_ref).start::<Dict>();
    font.pair(Name(b"Type"), Name(b"Font"));
    font.pair(Name(b"Subtype"), Name(b"TrueType"));

    // Adicionar prefixo de subset: AAAAAA+FontName
    let subset_font_name = format!("AAAAAA+{}", font_name);
    font.pair(Name(b"BaseFont"), Name(subset_font_name.as_bytes())); // "AAAAAA+DejaVuSans"
    // ...
}
```

**Nota:** O prefixo `AAAAAA+` é uma convenção PDF para indicar que a fonte é um subset. O prefixo é composto por 6 caracteres hexadecimais aleatórios (ou um hash do subset) seguido de `+`. No Typst vanilla, o prefixo parece ser fixo (`AAAAAA+`) ou derivado de um hash. Para simplificar, usar `AAAAAA+` fixo (ou gerar um hash curto do subset).

**Alternativa (hash do subset):**

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn generate_subset_prefix(subset_data: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    subset_data.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:06X}", hash & 0xFFFFFF) // 6 hex chars
}

// Uso:
let prefix = generate_subset_prefix(&subset_data);
let subset_font_name = format!("{}+{}", prefix, font_name);
```

### 3.3 Validação

```bash
# Gerar PDF com subsetting
target/release/typst compile /tmp/test-system-font.typ /tmp/out.pdf

# Verificar marcação de subset
pdffonts /tmp/out.pdf
# Esperado: AAAAAA+DejaVuSans (ou similar) com "yes" em "sub"
```

---

## 4. Formato do Relatório de Resultados

```
| 517a system fonts default | Vanilla: OK | Cristalino: OK | MATCH | fonte do sistema encontrada sem flag |
| 517b subset prefix | Vanilla: AAAAAA+ | Cristalino: AAAAAA+ | MATCH | prefixo de subset no PDF |
```

---

## 5. Testes de Não-Regressão

```bash
# Corpus P490+P500
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1     && echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done
# Esperado: 37/37 OK

# Verificar fontes no PDF
pdffonts /tmp/out.pdf | grep -E "(name|AAAAAA\+)"
```

---

## 6. Critério de Fecho

- [ ] 517a implementado: `with_system_fonts` activo por defeito na CLI.
- [ ] 517a validado: documento com fonte do sistema compila sem flag explícita.
- [ ] 517b implementado: prefixo `AAAAAA+` (ou hash) adicionado ao nome da fonte no PDF.
- [ ] 517b validado: `pdffonts` mostra prefixo de subset.
- [ ] Corpus P490+P500: 37/37 OK (não-regressão).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (L0 prompts + CLI docs).
- [ ] Sentinela `p517_cli_defaults` adicionada.
- [ ] `00_nucleo/diagnosticos/paridade-producao-p517.md` produzido.

---

## 7. Próximo Passo (P518)

Com P517 fechado, o cristalino está **pronto para uso real**:

- ✅ Paridade de linguagem completa
- ✅ Paridade de produção completa (shaping + fontdb + subsetting + system fonts + marcação)
- ✅ CLI amigável (fontes do sistema por defeito)

**Recomendação:** P518 = **DEBT-42 Benchmark Revalidado** — medir performance com shaping + subsetting + system fonts ativos, comparar com vanilla 0.15.0 em corpus representativo. Este é o benchmark "real" que valida o projeto como um todo.

Alternativa: P518 = **Lookahead Layout Engine** — inovação arquitetural (não paridade).

---

## A. Apêndice — Referência Rápida

```bash
# Compilar
cargo build --release -p typst-wiring

# Testar system fonts por defeito
echo '#set text(font: "DejaVu Sans")
Hello World' > /tmp/test.typ
target/release/typst compile /tmp/test.typ /tmp/out.pdf

# Verificar subset prefix
pdffonts /tmp/out.pdf

# Corpus
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1     && echo "OK" || echo "FAIL: $(basename $f)"
done
```

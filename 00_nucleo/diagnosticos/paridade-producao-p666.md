# Relatório de Paridade de Produção — Passo 666

| Campo | Valor |
|-------|-------|
| Passo | P666 |
| Foco | Retomar pendência de P525: fontes variáveis embutem só a instância default |
| Data | 2026-07-10 |
| Autor | IA (Kimi Code CLI) sob direção do utilizador |
| Status | Fechado sem alterações de código — pendência já resolvida |
| Commit de medição | `0e51b31f23d4d24b286e3c78de3073d3345a40b6` |

---

## Sonda

### Objetivo

Confirmar se a regressão documentada por P525 ainda se aplicava: numa fonte variável, o export PDF embutia sempre a instância default dos contornos, fazendo com que `text(weight: 700)` tivesse avanços de negrito mas contornos de regular.

### Documento de teste

```bash
cat > /tmp/p666-vf-pesos.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.

#set text(weight: 100)
Thin hello.
EOF
```

### Execução

```bash
TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python ./target/release/typst /tmp/p666-vf-pesos.typ /tmp/p666.pdf
pdffonts /tmp/p666.pdf
mutool draw -o /tmp/p666.png -r 150 /tmp/p666.pdf
```

### Resultado

| Propriedade | Cristalino (pós-P530/P666) |
|-------------|----------------------------|
| `pdffonts` | 3× `CID TrueType` (`CrystallineFont1`, `CrystallineFont2`, `CrystallineFont3`) |
| Instâncias | Regular (400), Bold (700), Thin (100) |
| Visual | Três pesos distintos e correctos (ver `/tmp/p666.png`) |

A saída de debug do builder mostra que cada instância recebe a sua `FontVariant`:

```text
font_variant=FontVariant { style: Normal, weight: FontWeight(400), ... } axis_vars=[]
font_variant=FontVariant { style: Normal, weight: FontWeight(700), ... } axis_vars=[(Tag(wght), 700.0)]
font_variant=FontVariant { style: Normal, weight: FontWeight(100), ... } axis_vars=[(Tag(wght), 100.0)]
```

### Inspecção de código

- `collect_fonts_from_doc` em `03_infra/src/pipeline.rs:493` já agrupa por `(FontList, FontVariant)`.
- `resolve_fonts` em `03_infra/src/pipeline.rs:536` preserva essa chave.
- `03_infra/src/export/builder.rs:644` instancia estaticamente a VF depois do subsetting, usando `axis_variations_for_font_variant`.
- `instantiate_variable_font` em `03_infra/src/font_variant.rs:142` invoca `fontTools.varLib.instancer` via Python.

Conclusão da sonda: a pendência de P525 **já não existe**. O trabalho foi realizado em P530 (instanciação estática) e integrado na pipeline de recolha/resolução de fontes sem que P525 fosse explicitamente reaberto.

---

## Comparação com Vanilla Typst

```bash
lab/typst-original/target/release/typst compile /tmp/p666-vf-vanilla.typ /tmp/p666-vanilla.pdf
pdffonts /tmp/p666-vanilla.pdf
```

| Propriedade | Cristalino | Vanilla |
|-------------|------------|---------|
| Nº de fontes embutidas | 3 | 3 |
| Pesos visuais distintos | Sim | Sim |
| Tamanho do PDF | ~223 KB | ~11 KB |

Nota: o vanilla produz PDFs muito menores porque o seu subsetter é mais agressivo; o cristalino ainda embute dados de variação residuais. Isto é uma questão de eficiência de subsetting, não de correcção de instância, e está fora do âmbito deste passo.

---

## Limitação observada: `style: "italic"` ainda não suportado

```bash
cat > /tmp/p666-vf-italic.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(style: "italic")
Italic hello.

#set text(weight: 700, style: "italic")
Bold italic hello.
EOF
```

Resultado: o cristalino emite `warning: text: propriedade 'style' ainda não suportada`. As linhas com `style: "italic"` caem para `Normal`. Isto é uma limitação de linguagem já documentada em P525 e agravada pela reversão de P665 (`italic: true` como arg nomeado foi removido). Não é parte da pendência de P525, que dizia especificamente sobre **peso**.

---

## Sanity checks

| Check | Comando | Resultado |
|-------|---------|-----------|
| Testes workspace | `cargo test --workspace` | 606 passed; 0 failed; 5 ignored |
| Linter | `crystalline-lint .` | ✓ No violations found |

Não foram introduzidas alterações de código.

---

## Tabela final de classificação

| Item | Resultado |
|------|-----------|
| Problema de P525 ainda existe? | ❌ Não — já resolvido |
| `collect_fonts_from_doc` agrupa por `(FontList, FontVariant)` | ✅ Sim |
| `resolve_fonts` preserva a chave | ✅ Sim |
| Instanciação estática por peso/estilo no export | ✅ Sim (`03_infra/src/export/builder.rs:644`) |
| Três pesos visuais distintos no PDF | ✅ Sim |
| Subsetting funciona com múltiplas instâncias | ✅ Sim (tamanho ~223 KB para 3 instâncias) |
| Sem regressão | ✅ Sim |
| Relatório com hash do commit | ✅ Sim |

---

## Fecho da pendência de P525

A pendência 1 do relatório de P525 — *Export PDF com variações visuais* — está **fechada por P666**. A instanciação estática por peso já funciona; a eficiência do subsetting de VF permanece como trabalho futuro.

---

## Reprodução

```bash
# Sonda P666
cat > /tmp/p666-vf-pesos.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.

#set text(weight: 100)
Thin hello.
EOF
TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python ./target/release/typst /tmp/p666-vf-pesos.typ /tmp/p666.pdf
pdffonts /tmp/p666.pdf

# Sanity checks
cargo test --workspace
crystalline-lint .
```

---

## Linhagem

- L0: `00_nucleo/prompts/infra/shaper.md` (actualizado em P525)
- Código: `03_infra/src/pipeline.rs`, `03_infra/src/export/builder.rs`, `03_infra/src/font_variant.rs`
- Pendência original: `00_nucleo/diagnosticos/paridade-producao-p525.md`
- ADR-0107: paridade é com a linguagem, não com a mecânica/igualdade do Rust.
- ADR-0108: medir antes de decidir; hash do commit registado.

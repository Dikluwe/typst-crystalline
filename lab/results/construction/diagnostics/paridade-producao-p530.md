# Relatório de Paridade de Produção — Passo 530

| Campo | Valor |
|-------|-------|
| Passo | P530 |
| Foco | Implementar fix real de Variation Fonts (instanciação estática) |
| Data | 2026-07-01 |
| Autor | IA (Kimi Code CLI) sob direção do utilizador |
| Status | Concluído |

---

## Contexto

O P525 fez o shaper aplicar coordenadas de eixo (`wght`, `ital`) correctamente nos avanços. O P527 confirmou que o export PDF embutia sempre a **instância default**, pelo que `text(weight: 700)` numa fonte VF não era visualmente bold. O P529 validou a pipeline de instanciação depois do subsetting. Este passo implementou essa pipeline.

## Implementação

### Ficheiros criados

- `03_infra/src/font_variant.rs` — helpers partilhados:
  - `text_style_to_font_variant`
  - `axis_variations_for_font_variant`
  - `is_variable_font`
  - `instantiate_variable_font` (invoca `fontTools` via subprocess)
- `00_nucleo/prompts/infra/font_variant.md` — Prompt L0 correspondente.
- Script Python embebido em `font_variant.rs` via raw string.

### Ficheiros alterados

- `03_infra/src/lib.rs` — adicionado `pub mod font_variant`.
- `03_infra/src/shaper.rs` — moveu funções de mapeamento para `font_variant.rs`.
- `03_infra/src/pipeline.rs`:
  - `collect_fonts_from_doc` devolve `Vec<(FontList, FontVariant)>`.
  - `resolve_font` recebe `&FontVariant`.
  - `resolve_fonts` devolve `Vec<((FontList, FontVariant), Vec<u8>)>`.
- `03_infra/src/export/mod.rs`, `export/builder.rs`, `export/stream.rs`:
  - `export_pdf_multifont`/`build_multifont` indexam por `(FontList, FontVariant)`.
  - Após subsetar, se a fonte for VF e a variante pedir eixos existentes, instancia estaticamente.
  - `emit_text_pdf`/`emit_shaped_pdf` seleccionam a fonte correcta por `(FontList, FontVariant)`.
- `03_infra/src/export/tests.rs` — testes actualizados para a nova assinatura.
- `03_infra/src/pipeline.rs` (tests) — testes actualizados.
- `00_nucleo/diagnosticos/cristalino-contexto-handoff.md` — estado de VF actualizado.

### Pipeline de instanciação

```text
oxifont-subset (Rust)
  → fonte VF pequena (apenas glifos usados)
  → remover GPOS/GSUB/GDEF
  → fontTools.varLib.instancer (Python, rápido sobre fonte pequena)
  → fonte estática por (FontList, FontVariant)
  → embutir no PDF
```

## Sub-tarefa 0 — Dependência Python runtime

- Custo de arranque do Python + import `fontTools`: **~3.2s**.
- Instanciação de 4 combinações numa única chamada: **~0.55s** (depois do arranque).
- Custo total para documento com 3 pesos + itálico: **~10s** na primeira compilação (arranque do Python para cada combinação; optimizável com cache/serviço no futuro).
- Documentos sem VF não pagam este custo.
- Decisão: Python + fontTools é dependência de runtime. Se não estiver disponível, o export faz fallback para a instância default com aviso.

## Sub-tarefas 1–4 — Colecta, subset, instanciação, export

Todas implementadas. O export PDF agora:

- Colecta combinações `(FontList, FontVariant)` distintas.
- Resolve cada combinação com `FontVariant` real (permitindo usar instâncias estáticas do sistema se existirem).
- Subseta cada fonte com `oxifont-subset`.
- Instancia estaticamente quando necessário.
- Embute cada instância como fonte separada e referencia-a no stream.

## Sub-tarefa 5 — Validação empírica

Documento de teste:

```typst
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.

#set text(weight: 100)
Thin hello.

#set text(weight: 400, italic: true)
Italic hello.
```

| Critério | Resultado |
|----------|-----------|
| Compilação sem erro | ✅ Exit code 0 |
| Fontes embutidas | ✅ 4 entradas (`pdffonts`) |
| Bold visivelmente mais grosso | ✅ `/W` 'H': 697 → 725; 'e': 552 → 576 |
| Thin visivelmente mais fino | ✅ `/W` 'H': 697 → 680; 'e': 552 → 541 |
| Itálico inclinado | ⚠️ Não aplicado — Ubuntu Sans VF não tem eixo `ital` |
| Texto extraível | ✅ `pdftotext` correcto |
| Tempo total | ~10s (inclui 3 arranques Python) |

Comparação com vanilla 0.15.0: vanilla também gera 4 entradas de fonte (3 pesos + itálico). O cristalino agora embebe instâncias estáticas para bold/thin, tal como vanilla.

## Sub-tarefa 6 — Sanity checks

| Check | Comando | Resultado |
|-------|---------|-----------|
| Testes `typst-infra` | `cargo test -p typst-infra` | 565 passed; 0 failed |
| Linter | `crystalline-lint .` | ✅ No violations |
| Corpus sem VF | loop sobre `lab/parity/corpus/` | 39/39 OK |
| Corpus CFF P523 | `test-cff-nimbus.typ` | OK |

## Limitações conhecidas

1. **Itálico sem eixo `ital`:** se a VF não expuser um eixo `ital`, o itálico não é aplicado visualmente. O shaper ainda calcula avanços com `ital=1`, mas o PDF usa a instância default. Alternativas futuras: seleccionar instância estática itálica do sistema, aplicar faux-italic no PDF, ou usar uma VF com eixo `ital`.
2. **Dependência Python + fontTools:** o binário `typst` requer `python3` com `fontTools` no PATH (ou `TYPST_CRYSTALLINE_PYTHON` apontando para o interpretador). A primeira compilação com VF tem custo de arranque do Python.
3. **Performance:** ~10s para 3 pesos + itálico. Pode ser melhorado com:
   - Cache persistente de instâncias por `(hash fonte + variant)`.
   - Serviço Python reutilizável entre compilações.
   - Substituição por crate Rust-native (Fontations/skrifa) quando madura.

## Decisão

Variation Fonts está **fechado em P530** para peso (`wght`). Itálico (`ital`) funciona quando a VF tem o eixo; caso contrário, é limitação documentada.

---

## Reprodução

```bash
# Compilar
cargo build --release --bin typst

# Definir Python com fontTools (se necessário)
export TYPST_CRYSTALLINE_PYTHON=/caminho/para/lab/.venv/bin/python3

# Documento de teste
cat > /tmp/test-vf-p530.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.
#set text(weight: 700)
Bold hello.
#set text(weight: 100)
Thin hello.
EOF
./target/release/typst /tmp/test-vf-p530.typ /tmp/vf-p530-cristalino.pdf
pdffonts /tmp/vf-p530-cristalino.pdf

# Testes e linter
cargo test -p typst-infra
crystalline-lint .
```

---

## Linhagem

- L0: `00_nucleo/prompts/infra/font_variant.md`
- Código: `03_infra/src/font_variant.rs`, `03_infra/src/pipeline.rs`, `03_infra/src/export/builder.rs`, `03_infra/src/export/stream.rs`
- ADR-0107: paridade é com a linguagem, não com a mecânica.
- ADR-0108: medir antes de decidir.
- ADR-0109: atomização.

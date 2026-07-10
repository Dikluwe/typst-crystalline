# Relatório de Paridade de Produção — Passo 669

| Campo | Valor |
|-------|-------|
| Passo | P669 |
| Foco | Confirmar o ramo vazio do dispatch de fontes |
| Data | 2026-07-10 |
| Autor | IA (Kimi Code CLI) sob direção do utilizador |
| Status | Confirmado — sem alterações de código |
| Commit de medição | `b9c6d9c04f17a7680e9fb09e8c868e263288bebe` |

---

## Verificação

### Objetivo

P668 corrigiu o ramo "uma fonte" do dispatch de `pipeline.rs`. Restava confirmar o terceiro ramo, `[]` (nenhuma fonte resolvida), para garantir que não é possível uma fonte variável com eixos não-default acabar ali sem ser instanciada.

### Quando `resolved` fica vazio

`resolved` resulta de `resolve_fonts(&collect_fonts_from_doc(&doc), ...)`. A função `collect_fonts_from_doc` (`03_infra/src/pipeline.rs:530`) só recolhe `FontList` a partir de `FrameItem::Text` ou `FrameItem::TextShaped`. Logo, `resolved` é vazio **se e só se** o documento não contiver texto — por exemplo, apenas formas, imagens ou páginas em branco.

### Teste directo

```bash
export TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python
cat > /tmp/p669-vazio.typ <<'EOF'
#set text(font: "Ubuntu Sans", weight: 700)
#rect(width: 10pt, height: 10pt)
EOF
./target/release/typst /tmp/p669-vazio.typ /tmp/p669.pdf
```

Resultado:

- Exit code: `0`.
- `pdffonts` mostra apenas as três fontes Helvetica Type1 de fallback.
- Nenhuma fonte Ubuntu Sans embutida.

A declaração `#set text(font: "Ubuntu Sans", weight: 700)` existe no source, mas como não há texto no documento, nenhum `FrameItem::Text/TextShaped` é criado, logo `collect_fonts_from_doc` devolve vazio e o ramo `[]` é atingido.

### Conclusão

O ramo vazio só é atingido quando não há texto. Por definição, não há fonte variável para instanciar neste caso. Não há código a alterar.

---

## Tabela final de classificação

| Item | Resultado |
|------|-----------|
| Situação do ramo vazio confirmada | ✅ Atingido quando não há texto |
| VF com eixos não-default pode cair no ramo vazio? | ❌ Não — sem texto não há `FontList` colectada |
| Código alterado? | ✅ Não necessário |
| Relatório com hash do commit | ✅ |

---

## Fecho da pendência de P525

Com P666 (caminho multi-font), P668 (caminho single-font) e P669 (ramo vazio), os três ramos do dispatch de fontes em `pipeline.rs` foram verificados:

| Ramo | Estado |
|------|--------|
| `[]` — nenhuma fonte | ✅ Confirmado em P669: não envolve VF |
| `[single]` — uma fonte | ✅ Corrigido em P668: redireccionado para multi-font quando VF com eixos não-default |
| `many` — várias fontes | ✅ Confirmado em P666: já instanciava correctamente |

A pendência de P525 sobre variações visuais de fontes variáveis está agora fechada nos três caminhos.

---

## Reprodução

```bash
export TYPST_CRYSTALLINE_PYTHON=lab/.venv/bin/python
cat > /tmp/p669-vazio.typ <<'EOF'
#set text(font: "Ubuntu Sans", weight: 700)
#rect(width: 10pt, height: 10pt)
EOF
./target/release/typst /tmp/p669-vazio.typ /tmp/p669.pdf
pdffonts /tmp/p669.pdf
```

---

## Linhagem

- L0: `00_nucleo/prompts/infra/pipeline.md`
- Código: `03_infra/src/pipeline.rs`
- Relacionados: `00_nucleo/diagnosticos/paridade-producao-p525.md`, `00_nucleo/diagnosticos/paridade-producao-p666.md`, `00_nucleo/diagnosticos/paridade-producao-p668.md`
- ADR-0108: medir antes de decidir; o ramo vazio foi testado directamente.

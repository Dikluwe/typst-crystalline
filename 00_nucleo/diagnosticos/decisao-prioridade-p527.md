# Decisão de Prioridade — Passo 527

| Campo | Valor |
|-------|-------|
| Passo | P527 |
| Foco | Correcção do estado de VF + decisão de prioridade pós-P526 |
| Data | 2026-07-01 |
| Autor | IA (Kimi Code CLI) sob direcção do utilizador |
| Status | Aguarda decisão do utilizador |

---

## Sub-tarefa 0 — Correcção do estado de Variation Fonts

### Diagnóstico confirmado

Sondagem do código real (`03_infra/src/pipeline.rs` e `03_infra/src/export/builder.rs`):

- `collect_fonts_from_doc` agrupa fontes por `FontList` (sem weight/style).
- `resolve_font` chama `font_book.select_pattern(&family.name, &FontVariant::default())`.
- `export_pdf_multifont` embute **uma instância default** por fonte, independentemente dos pesos/estilos usados no documento.

Resultado: o shaper (P525) calcula avanços correctos para cada peso, mas o PDF final contém sempre os contornos da instância default. `text(weight: 700)` numa fonte VF produz texto visualmente igual a `weight: 400`.

### Confirmação com vanilla 0.15.0

```bash
./lab/typst-original/target/release/typst compile /tmp/test-vf-p525.typ /tmp/vf-vanilla-p527.pdf
pdffonts /tmp/vf-vanilla-p527.pdf
```

Resultado: **4 entradas de fonte** (3 instâncias Regular para pesos 400/700/100 + 1 Italic), cada uma subsetada separadamente. O vanilla instancia a VF estaticamente por combinação peso/estilo.

### Documentação corrigida

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/diagnosticos/cristalino-contexto-handoff.md` | Linha VF alterada de "✅ Fechado em P525" para "⚠️ Parcial"; secção 6 adicionada opção A (P528 VF-fix). |
| `00_nucleo/diagnosticos/paridade-producao-p525.md` | Sub-tarefa 3 actualizada: avanços correctos, contornos sempre da instância default; classificado como regressão de linguagem. |
| `00_nucleo/prompts/infra/shaper.md` | Secção "Limitações do MVP" actualizada com a regressão de output visual e a necessidade de instanciação estática. |
| `03_infra/src/shaper.rs` | Hash actualizado via `crystalline-lint --fix-hashes`. |

---

## Sub-tarefa 1 — Decisão de prioridade

### Dados da sonda P526 + P527

| Funcionalidade | Estado | Esforço MVP | Impacto | Tipo de gap |
|---------------|--------|-------------|---------|-------------|
| **VF instanciação estática** | ⚠️ Regressão de linguagem | **S–M** | `weight:` funciona visualmente para fontes VF do sistema | Regressão |
| SVG export | ❌ AUSENTE | M–L | Novo formato de saída | Ausência declarada |
| PNG export | ❌ AUSENTE | S–M (pós-SVG) | Novo formato de saída | Ausência declarada |
| HTML export | ❌ AUSENTE | M–L | Novo formato de saída | Ausência declarada |
| IDE/LSP | ❌ AUSENTE | L–XL | Ferramenta de desenvolvimento | Ausência declarada |

### Argumentos

**VF-fix primeiro (recomendação do assistente):**
- É uma **regressão de linguagem**: `text(weight: 700)` deveria produzir texto visualmente bold, mas não produz para fontes VF.
- Esforço **S–M**: instanciar a VF para cada combinação peso/estilo usada, subsetar cada instância separadamente, e referenciar a instância correcta no `TextShaped`/PDF.
- Não requer novas crates (usa `oxifont-subset`, que já suporta instanciação implícita; ou `fontTools.varLib.instancer` via subprocess se necessário).
- Fecha uma brecha existente antes de abrir novas trilhas.

**SVG/PNG/HTML primeiro:**
- São ausências declaradas no handoff, não regressões.
- O utilizador estabeleceu igualdade de saída como pré-requisito para inovação — mas estas funcionalidades são grandes (M–L cada).
- SVG é base para PNG; faz sentido agrupar.

**LSP primeiro:**
- Menos prioritário — ferramenta de desenvolvimento, não formato de saída.
- XL de esforço; candidato a scope-out permanente se aceitável.

### Recomendação do assistente

> **P528: Fix de VF (instanciação estática) → P529: SVG export → P530: PNG export → P531: HTML export → P532+: LSP (scope-out se aceitável)**

Baseado em:
- ADR-0108: medir antes de decidir.
- ADR-0107: regressão de linguagem > ausência mecânica.
- P526: SVG/PNG/HTML são ausentes, mas não são regressões.

### Próximo passo dependente da decisão

| Decisão | Passo | Tamanho | Descrição |
|---------|-------|---------|-----------|
| **VF-fix** | P528 | S–M | Instanciar VF estaticamente por peso/estilo; subsetar cada instância; referenciar no PDF |
| **SVG export** | P528/P529 | M–L | Implementar SVG export directo do `Frame` |
| **PNG export** | P52x | S–M | Via SVG → resvg/tiny-skia (requer SVG primeiro) |
| **HTML export** | P53x | M–L | Mapear `Frame` → HTML DOM + CSS |
| **LSP** | P54x+ | XL | Servidor LSP com diagnostics básicos |

---

## Nota sobre a sub-tarefa 2 (especificação SVG)

A especificação da Trilha 8 (SVG export) **não foi escrita** neste passo porque a recomendação do assistente é **VF-fix primeiro**. Se o utilizador decidir que SVG é prioritário (ou que VF-fix pode esperar), a especificação será produzida no passo seguinte correspondente.

---

## Reprodução

```bash
# Confirmar agrupamento por FontList no export
grep -n "collect_fonts_from_doc\|FontVariant::default" 03_infra/src/pipeline.rs

# Confirmar instâncias estáticas no vanilla
./lab/typst-original/target/release/typst compile /tmp/test-vf-p525.typ /tmp/vf-vanilla-p527.pdf
pdffonts /tmp/vf-vanilla-p527.pdf

# Linter
crystalline-lint .
```

---

## Linhagem

- L0: `00_nucleo/prompts/infra/shaper.md` (actualizado em P527)
- Código: `03_infra/src/pipeline.rs`, `03_infra/src/export/builder.rs`
- ADR-0107: paridade é com a linguagem, não com a mecânica.
- ADR-0108: medir antes de decidir.
- ADR-0114: sonda A.0 antes de spec.

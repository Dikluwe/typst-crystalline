# Relatório de Paridade de Produção — Passo 528

| Campo | Valor |
|-------|-------|
| Passo | P528 |
| Foco | Fix real de Variation Fonts: diagnóstico de abordagens e decisão de não implementar |
| Data | 2026-07-01 |
| Autor | IA (Kimi Code CLI) sob direcção do utilizador |
| Status | Concluído (diagnóstico) |

---

## Contexto

O P527 confirmou que o cristalino embebe sempre a **instância default** de uma fonte VF no PDF, embora o shaper (P525) calcule avanços correctos para cada peso/estilo. Isto é uma **regressão de linguagem**: `text(weight: 700)` numa fonte VF produz texto visualmente igual a `weight: 400`. O vanilla resolve isto embebedando **instâncias estáticas separadas** (Regular, Bold, Thin, Italic) como fontes distintas no PDF.

Este passo tinha como objetivo implementar o fix real de VF. Foram avaliadas duas abordagens. A **Abordagem A** (instanciação estática via `fontTools`) foi testada e **descartada por performance**. A **Abordagem B** (faux-bold/faux-italic no PDF) é tecnicamente viável, mas não é paridade real com o vanilla. Foi decidido **não implementar nenhuma das duas neste passo** e produzir este relatório para fundamentar a pesquisa de alternativas.

---

## Diagnóstico confirmado (de P527)

Pontos de medição no código:

- `03_infra/src/pipeline.rs:394` — `collect_fonts_from_doc` agrupa fontes por `FontList`, sem considerar `weight`/`style`.
- `03_infra/src/pipeline.rs:496` — `resolve_font` chama `font_book.select_pattern(name, &FontVariant::default())`.
- `03_infra/src/export/builder.rs:383` — `build_multifont` recebe `&[(FontList, Vec<u8>)]` — uma fonte por `FontList`.
- `03_infra/src/export/stream.rs:244` — `emit_shaped_pdf` selecciona `/F{}` apenas por `style.font`.

Conclusão: o export PDF embebe **uma única instância default por família**, independentemente das variações usadas no documento.

### Confirmação com vanilla 0.15.0

```bash
./lab/typst-original/target/release/typst compile /tmp/test-vf-p525.typ /tmp/vf-vanilla-p528.pdf
pdffonts /tmp/vf-vanilla-p528.pdf
```

Resultado: **4 entradas de fonte** — 3 instâncias Regular (pesos 400, 700, 100) + 1 Italic. O vanilla instancia a VF estaticamente e subseta cada instância separadamente.

---

## Abordagem A — Instanciação estática via `fontTools`

### Ideia

Para cada combinação `(FontList, FontVariant)` usada no documento, gerar uma fonte TTF estática com `fontTools.varLib.instancer.instantiateVariableFont` e depois subsetá-la com `oxifont-subset`.

### Teste de performance

Fonte: `UbuntuSans[wdth,wght].ttf` (VF TrueType, 2 eixos, 16 named instances).

```bash
time lab/.venv/bin/python3 - <<'PY'
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont
import io

path = '/usr/share/fonts/truetype/ubuntu/UbuntuSans[wdth,wght].ttf'
f = TTFont(path)
inst = instantiateVariableFont(f, {'wght': 700, 'wdth': 100})
buf = io.BytesIO()
inst.save(buf)
print('size:', len(buf.getvalue()))
PY
```

Resultado:

```text
size: 418020
real    4m0,642s
user    3m57,694s
sys     0m2,327s
```

### Variações testadas

| Configuração | Tempo | Tamanho |
|--------------|-------|---------|
| `instantiateVariableFont(f, {'wght':700,'wdth':100})` | 4m0,6s | 418 KB |
| com `overlap=False, updateFontNames=False` | 3m55,3s | 418 KB |
| `fontTools.varLib.mutator.instantiateVariableFont` (deprecated) | 3m59,0s | 418 KB |

### Porque A é inviável

- **Tempo por instância:** ~4 minutos para uma única combinação de eixos.
- Um documento com 3 pesos + itálico demoraria **~16 minutos só em instanciação**.
- A latência é inaceitável para CLI interactiva, CI, ou qualquer fluxo de produção.
- O bottleneck parece ser o processamento de outlines `glyf`/`gvar` (remoção de overlaps e recalcular contornos), não a I/O.

### Conclusão da Abordagem A

**Descartada** para o MVP de VF-fix. Pode ser reconsiderada no futuro se houver uma alternativa de instanciação ordens de magnitude mais rápida.

---

## Abordagem B — Faux-bold / faux-italic no PDF

### Ideia

Em vez de instanciar a fonte, aplicar transformações PDF no operador de texto:

- **Bold:** `2 Tr` (text rendering mode stroke) com espessura proporcional ao tamanho, ou escala horizontal `Tz > 100%`.
- **Italic:** matriz de transformação `Tm` com shear (ângulo ~12°).
- **Weight intermédio:** mapear para stroke/escala proporcional.

### Vantagens

- Rápido — não requer instanciação.
- Não depende de `fontTools` nem de Python.
- Funciona para qualquer fonte (estática ou VF).

### Desvantagens

- **Não é paridade com vanilla.** O vanilla usa contornos reais da instância; B usa simulação.
- **Qualidade inferior:** faux-bold engrossa uniformemente o stroke, o que pode parecer "sujo"; faux-italic distorce as proporções.
- **Avanços incorrectos:** o shaper calcula avanços com a fonte regular; o faux-bold real ocupa mais espaço, pelo que o posicionamento pode ficar ligeiramente apertado.
- **Não funciona para eixos arbitrários** (ex.: `wdth`, `slnt` com ângulo específico).
- **Math e símbolos:** pode quebrar glyph complexos.

### Conclusão da Abordagem B

Tecnicamente viável como **fallback de emergência**, mas **não resolve a regressão de linguagem com fidelidade**. Não é aceitável como solução final se o objectivo for paridade com vanilla.

---

## Alternativas a pesquisar

Dado que A é lento e B é baixa fidelidade, recomenda-se investigar alternativas Rust-native ou mais eficientes:

| Alternativa | Descrição | Potencial | Notas |
|-------------|-----------|-----------|-------|
| **`skrifa` + `read-fonts`** | Crates do Google Fonts / Fontations para ler e rasterizar fontes variáveis. | Alto | Não expõe API pública de serialização de instância estática (até onde se sabe). |
| **`fontations` / `write-fonts`** | Eco-systema Rust para manipulação de fontes OpenType. | Alto | Pode permitir construir uma fonte estática a partir de outlines variados; requer investigação. |
| **`freetype-py` + extrair outlines** | Usar FreeType para obter outlines na instância desejada e reconstruir uma fonte. | Médio | Complexo; perde hinting e tabelas avançadas. |
| **`harfbuzz` + serialização custom** | Aplicar `hb_font_set_variations_coord` e extrair glyphs. | Médio | Não há API simples de serialização de fonte. |
| **Dependência binária pré-compilada** | Usar `fontTools` como serviço/cache persistente. | Baixo | Não resolve a latência inicial; aumenta complexidade de deploy. |
| **Seleccionar instâncias estáticas do sistema** | Usar `font_book.select(name, variant)` e confiar que o SO tem `UbuntuSans-Bold.ttf`, etc. | Baixo-Médio | No sistema de teste só existe a VF; noutros sistemas pode haver estáticas. Não resolve o caso geral. |

### Recomendação de pesquisa

Priorizar **Fontations (`read-fonts`/`write-fonts`)** e **`skrifa`**, pois são Rust-native e podem ser integrados sem subprocess. Verificar se `write-fonts` permite construir uma tabela `glyf`/`CFF` a partir de outlines variados de `skrifa`.

---

## Decisão

| Opção | Estado |
|-------|--------|
| Implementar Abordagem A agora | ❌ Rejeitada — inviável por performance |
| Implementar Abordagem B agora | ❌ Rejeitada — baixa fidelidade, não resolve paridade |
| Pesquisar alternativas (Fontations, skrifa, etc.) | ✅ Decidido pelo utilizador |

**Próximo passo:** o utilizador irá pesquisar alternativas de instanciação rápida. Só depois se retoma a implementação do fix real de VF.

---

## Impacto no handoff

O estado de Variation Fonts no handoff mantém-se como **parcial**. A secção de próximos passos deve reflectir que P528 foi um diagnóstico e que a implementação depende de pesquisa de alternativas.

---

## Reprodução

```bash
# Confirmar regressão no cristalino
grep -n "FontVariant::default" 03_infra/src/pipeline.rs

# Medir instanciação fontTools
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline
time lab/.venv/bin/python3 - <<'PY'
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont
import io
path = '/usr/share/fonts/truetype/ubuntu/UbuntuSans[wdth,wght].ttf'
f = TTFont(path)
inst = instantiateVariableFont(f, {'wght': 700, 'wdth': 100})
buf = io.BytesIO(); inst.save(buf); print(len(buf.getvalue()))
PY

# Comparar com vanilla
./lab/typst-original/target/release/typst compile /tmp/test-vf-p525.typ /tmp/vf-vanilla-p528.pdf
pdffonts /tmp/vf-vanilla-p528.pdf
```

---

## Linhagem

- L0: `00_nucleo/prompts/infra/shaper.md` (limitação P525/P527)
- Código: `03_infra/src/pipeline.rs`, `03_infra/src/export/builder.rs`, `03_infra/src/export/stream.rs`
- ADR-0107: paridade é com a linguagem, não com a mecânica.
- ADR-0108: medir antes de decidir.

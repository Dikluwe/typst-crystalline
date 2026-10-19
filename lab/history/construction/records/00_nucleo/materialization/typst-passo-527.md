---
# P527 — Correção do estado de VF no handoff + Decisão de prioridade pós-P526

> **Passo:** 527
> **Data:** 2026-07-01
> **Foco:** (0) Sondar e corrigir o estado de Variation Fonts no handoff e documentação do projecto — o P525 reportou MVP fechado mas o PDF final não renderiza pesos visualmente distintos (regressão de linguagem). (1) Decidir a ordem de prioridade entre: fix real de VF (instanciação estática por peso/estilo), SVG export, PNG export, HTML export, e IDE/LSP — com base nos resultados da sonda P526. (2) Se a decisão for SVG, escrever a especificação da Trilha 8 (SVG export). Zero código de produção na sub-tarefa 0 e 1; sub-tarefa 2 é especificação (L0 + decisões técnicas).
> **Tipo:** Diagnóstico + Documentação + Especificação
> **Tamanho:** M (~45 min)
> **ADR-0108 EM VIGOR** — medir antes de decidir; a correção do handoff requer sondagem do código real.
> **ADR-0114 EM VIGOR** — sonda A.0 antes de spec; a especificação de SVG só após decisão informada.
> **ADR-0107 EM VIGOR** — paridade é de linguagem, não de mecânica; VF instância default é regressão de linguagem (`weight: 700` sem efeito visual).
> **Dependências:** P525 (relatório com estado incorrecto de VF), P526 (sonda de igualdade de saída: HTML/SVG/PNG/IDE ausentes)
---

## Contexto

Durante a execução do P526, detectou-se que o handoff (`cristalino-contexto-handoff.md`) e o relatório P525 continham uma classificação incorrecta do estado de Variation Fonts:

- **P525 reportou:** "VF MVP fechado — shaper aplica `wght`/`ital` via `set_variations`"
- **Realidade:** O shaper calcula avanços correctamente para cada peso, mas o PDF final embebe apenas a **instância default** dos contornos da fonte. Um leitor de PDF não tem mecanismo para aplicar `wght=700` a contornos embutidos — o vanilla resolve isto embebedando **instâncias estáticas separadas** (Regular, Bold, Thin, Italic) como fontes distintas no PDF.
- **Impacto:** `text(weight: 700)` numa fonte VF produz texto com avanços de bold mas **contornos de regular** — visualmente indistinguível de `weight: 400`. Isto é **regressão de linguagem** (semântica `weight` ignorada no output visual), não mera mecânica PDF.

Esta sub-tarefa 0 sonda o código real para confirmar o diagnóstico, corrige o handoff e todos os documentos L0 afectados, e decide a prioridade entre VF-fix e os novos formatos de saída (SVG/PNG/HTML).

---

## Sub-tarefa 0 — Sonda e correção do estado de VF

### 0.1 Verificar o código real de export PDF

```bash
grep -n "subset_font\|build_cidfont\|build_multifont\|FontDescriptor\|Subtype.*CIDFont" 03_infra/src/export/builder.rs 03_infra/src/export/subset.rs | head -n 30
```

Perguntas a confirmar:
- O `build_cidfont` / `build_multifont` é chamado **uma vez por fonte** ou **uma vez por combinação de peso/estilo**?
- O `subset_font_with_mapping` recebe o `FontVariant` (peso/estilo) ou apenas o `font_data` e `glyph_ids`?
- O `glyph_mapping` e `to_unicode_mappings` são construídos por fonte ou por sub-run de texto?

Se o export PDF agrupa todos os `TextShaped` da mesma fonte numa só fonte CID — independentemente do peso — então a confirmação está feita: **uma só instância default é embutida para todos os pesos**.

### 0.2 Confirmar com o vanilla

```bash
./lab/typst-original/target/release/typst compile /tmp/test-vf.typ /tmp/vf-vanilla.pdf
pdffonts /tmp/vf-vanilla.pdf
```

O vanilla deve mostrar **múltiplas entradas** para a mesma família (ex.: `UbuntuSans-Regular`, `UbuntuSans-Bold`, `UbuntuSans-Thin`) — cada uma é uma instância estática separada, subsetada individualmente.

### 0.3 Classificação correcta

| Estado anterior (P525/handoff) | Estado real | Classificação ADR-0107 |
|-------------------------------|-------------|------------------------|
| "✅ Fechado em P525 (MVP)" | Shaper aplica variações nos avanços, mas export PDF embebe instância default dos contornos | **Regressão de linguagem** — `weight: 700` sem efeito visual |

### 0.4 Correção de documentos

Ficheiros a corrigir:

1. **`00_nucleo/diagnosticos/cristalino-contexto-handoff.md`**
   - Secção 5.2: alterar linha VF de "✅ Fechado em P525 (MVP)" para:
     ```
     Variation fonts (VF) | ⚠️ Parcial — pesos calculados nos avanços mas contornos sempre da instância default | Shaper aplica wght/ital correctamente; fix real requer instanciar VF estaticamente por combinação de peso/estilo usada no documento (como vanilla faz)
     ```
   - Secção 6 (Próximos Passos): adicionar opção B (Fix de VF) antes de SVG/PNG/HTML.

2. **`00_nucleo/diagnosticos/paridade-producao-p525.md`**
   - Sub-tarefa 3: alterar de "Visual: mantida" para "Visual: **parcial** — avanços correctos, contornos da instância default".
   - Adicionar nota: "Regressão de linguagem: `weight: 700` numa fonte VF não produz texto visualmente bold no PDF final."

3. **`00_nucleo/prompts/infra/shaper.md`** (ou L0 equivalente)
   - Adicionar secção P525-correção: "Limitação conhecida: o shaper aplica variações mas o export PDF não instancia a fonte estaticamente."

### 0.5 Critério de fecho

- [ ] Código de export PDF inspeccionado; confirmado que uma só instância default é embutida por fonte.
- [ ] `pdffonts` do vanilla confirmado com múltiplas instâncias estáticas.
- [ ] Handoff corrigido (secção 5.2, 6).
- [ ] Relatório P525 corrigido (secção Sub-tarefa 3).
- [ ] L0 shaper.md (ou equivalente) actualizado com limitação.
- [ ] `crystalline-lint .` passa (hashes actualizados se necessário).

---

## Sub-tarefa 1 — Decisão de prioridade: VF-fix vs SVG vs PNG vs HTML vs LSP

### 1.1 Dados da sonda P526

| Funcionalidade | Estado | Esforço MVP | Impacto |
|---------------|--------|-------------|---------|
| **VF instanciação estática** | ⚠️ Regressão de linguagem | **S–M** | `weight: 700` funciona visualmente para fontes VF |
| SVG export | ❌ AUSENTE | M–L | Novo formato de saída |
| PNG export | ❌ AUSENTE | S–M (pós-SVG) | Novo formato de saída |
| HTML export | ❌ AUSENTE | M–L | Novo formato de saída |
| IDE/LSP | ❌ AUSENTE | L–XL | Ferramenta de desenvolvimento |

### 1.2 Argumentos para cada ordem

**VF-fix primeiro (recomendado):**
- É uma **regressão de linguagem** — funcionalidade declarada como suportada (`weight:`) que não funciona para um caso real (fontes VF do sistema).
- É **S–M** em esforço — instanciar a fonte VF para cada combinação de peso/estilo usada no documento, subsetar cada instância separadamente, e referenciar a instância correcta no `TextShaped` do PDF.
- Não requer novas dependências de crates (usa `oxifont-subset` que já suporta instanciação implícita, ou `fontTools.varLib.instancer` via subprocess se necessário).
- Fecha uma brecha antes de abrir novas trilhas.

**SVG/PNG/HTML primeiro:**
- São **ausências declaradas** (scope-out no handoff), não regressões.
- O usuário estabeleceu que inovação só após igualdade de saída completa — mas "igualdade de saída" inclui formatos que o vanilla suporta e o cristalino não.
- SVG é a base para PNG; faz sentido agrupar.

**LSP primeiro:**
- Menos prioritário — é ferramenta de desenvolvimento, não formato de saída.
- XL de esforço; pode ser scope-out permanente se o utilizador aceitar.

### 1.3 Decisão

O passo P527 **não decide sozinho** — apresenta os dados e aguarda decisão do usuário. No entanto, a recomendação técnica (baseada em ADR-0108: medir antes de decidir, e ADR-0107: regressão de linguagem > ausência mecânica) é:

> **P528: Fix de VF (instanciação estática) → P529: SVG export → P530: PNG export → P531: HTML export → P532+: LSP (scope-out se aceitável)**

### 1.4 Critério de fecho

- [ ] Tabela de prioridade documentada em `00_nucleo/diagnosticos/decisao-prioridade-p527.md`.
- [ ] Recomendação do assistente registada com justificação (regressão de linguagem vs ausência).
- [ ] Aguardar decisão do usuário antes de escrever P528.

---

## Sub-tarefa 2 — Especificação de SVG export (se decidido pelo usuário)

Se o usuário decidir que SVG é o próximo passo (ignorando VF-fix ou depois dele), esta sub-tarefa escreve a especificação da Trilha 8.

### 2.1 Estratégia de SVG export

Duas abordagens possíveis:

**A. SVG direto do `Frame` (recomendado):**
- Iterar o `Document.pages` → `Page.frames` → `FrameItem`.
- Mapear cada `FrameItem` para elemento SVG:
  - `TextShaped` → `<text>` com `font-family` (referenciar fonte do sistema) ou `<path>` (converter contornos).
  - `Image` → `<image href="...">` (referenciar ficheiro externo) ou base64 embutido.
  - `Shape` (rect, ellipse, path) → `<rect>`, `<ellipse>`, `<path>`.
  - `Gradient`/`Pattern` → `<linearGradient>`, `<radialGradient>`, `<pattern>`.
- Cada página → `<svg viewBox="...">` separado ou `<g>` num só SVG.

**B. SVG via intermediário (over-engineering):**
- Converter `Frame` para um formato intermédio (ex.: scene graph) e depois para SVG.
- Não recomendado — o `Frame` já é uma representação de cena suficiente.

### 2.2 Fontes no SVG

| Opção | Prós | Contras | Esforço |
|-------|------|---------|---------|
| Referenciar fonte do sistema (`font-family: "Ubuntu Sans"`) | Simples, PDF-size-like | Não funciona se a fonte não estiver instalada no viewer | XS |
| Embeber fonte como `@font-face` com base64 | Self-contained | Base64 aumenta tamanho; subsetting requer extração | S–M |
| Converter glyphs para `<path>` | Self-contained, nenhuma dependência de fonte | Tamanho enorme; perde seleccionabilidade de texto | M |
| Usar `<text>` + referenciar fonte subsetada (como PDF) | Seleccionável, compacto | Requer subsetting + embedding; complexo | L |

**Recomendação para MVP:** Opção 1 (referenciar fonte do sistema) para MVP rápido; Opção 2 (`@font-face` base64) para fidelidade. A escolha depende do objetivo do usuário ("igualdade de saída" com vanilla — verificar como o vanilla faz).

### 2.3 Como o vanilla faz SVG?

```bash
./lab/typst-original/target/release/typst compile --format svg /tmp/test.typ /tmp/vanilla.svg
cat /tmp/vanilla.svg | head -n 50
```

Inspeccionar:
- Usa `<text>` ou `<path>` para texto?
- Referencia fontes externamente ou embute?
- Como lida com imagens (href externo, base64, ou paths)?
- Estrutura do SVG (uma página = um SVG, ou múltiplas `<g>`?)

### 2.4 Decisões técnicas a documentar

- [ ] Formato de texto: `<text>` vs `<path>`.
- [ ] Estratégia de fontes: referência externa / `@font-face` / paths.
- [ ] Estrutura de páginas: um SVG por página ou múltiplas `<g>`.
- [ ] Imagens: href externo / base64 / omitir.
- [ ] Coordenadas: o `Frame` usa coordenadas PDF-like (y cresce para cima? ou para baixo?); SVG usa y cresce para baixo. Necessário flip Y ou transform.
- [ ] ViewBox: `0 0 width height` por página.

### 2.5 Critério de fecho da especificação

- [ ] SVG do vanilla inspeccionado e documentado.
- [ ] Decisões técnicas tomadas (com justificação).
- [ ] Especificação escrita em `00_nucleo/prompts/infra/export/svg.md` (ou equivalente).
- [ ] Template de passos P528+ definido (quantos passos, tamanho de cada).

---

## Critério de fecho do passo P527

- [ ] Sub-tarefa 0: handoff e documentação corrigidos; estado de VF reflecte realidade.
- [ ] Sub-tarefa 1: decisão de prioridade documentada; aguarda input do usuário.
- [ ] Sub-tarefa 2: se aplicável, especificação de SVG escrita e pronta para implementação.
- [ ] `crystalline-lint .` passa.
- [ ] Commit atómico: `P527: correção estado VF no handoff + decisão de prioridade pós-P526`.

---

## Próximo passo (depende da decisão do usuário)

| Decisão | Passo | Tamanho | Descrição |
|---------|-------|---------|-----------|
| **VF-fix** | P528 | S–M | Instanciar VF estaticamente por peso/estilo; subsetar cada instância; referenciar no PDF |
| **SVG export** | P528 (ou P529) | M–L | Implementar SVG export direto do `Frame` |
| **PNG export** | P52x | S–M | Via SVG → resvg/tiny-skia (requer SVG primeiro) |
| **HTML export** | P53x | M–L | Mapear `Frame` → HTML DOM + CSS |
| **LSP** | P54x+ | XL | Servidor LSP com diagnostics básicos |

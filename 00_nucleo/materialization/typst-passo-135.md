# Passo 135 — Diagnóstico do estado actual de shaping

**Série**: 135 (passo **S** em L0).
**Precondição**: Passo 134 encerrado; 1084 total tests; zero
violations; 53 ADRs activas; 11 DEBTs abertos. Lista canónica
DEBT-1 capturada (`font`, `lang`, `par.leading`, `weight`).
**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional) — reinterpretada neste passo
  para incluir consumo integral.
- **ADR-0034** (diagnóstico obrigatório) — este passo é
  cumprimento preventivo antes de mover para fase de consumo.
- **ADR-0019** (TTF+RustyBuzz) — referência para shaping.
- **DEBT-48** (referenciado no DEBT.md linha 116) — `TextStyle`
  plano como ponte actual.

**Natureza**: passo L0-puro. **Sem código**. **Sem testes**.
Produz documento de inventário + potencial DEBT novo + roadmap
revisto.

---

## Mudança estratégica que este passo formaliza

O roadmap original previa 135 como "fechar DEBT-1 em documentação".
Decisão reavaliada: **DEBT-1 não fecha enquanto `StyleDelta` for
inerte**. O critério "paridade total com o sistema de styles do
original" (DEBT.md linha 48) inclui output observável, não só
captura.

Resolver shaping integral é pré-requisito para:
- Efeito visível no PDF para propriedades capturadas (weight,
  tracking, leading, lang, font).
- Paridade end-to-end com vanilla para os mesmos inputs.
- Resolução natural do font dict (shaping consome `covers`).

Este passo **não resolve shaping**. Este passo **inventaria o
que existe e identifica o que falta**, para que os passos
seguintes ataquem gaps concretos em vez de adivinhar.

---

## Objectivo

Ao fim do passo:

1. **Diagnóstico** em
   `00_nucleo/diagnosticos/diagnostico-shaping-passo-135.md`
   com:
   - Estado actual do consumer de `StyleDelta` em layout.
   - Estado de `FontBook`, `rustybuzz`, `TextStyle`.
   - DEBTs existentes relacionados com shaping.
   - Gap entre estado actual e paridade end-to-end.
   - Mapeamento "propriedade em `StyleDelta` → consumer em
     layout → efeito no PDF" para cada campo.

2. **DEBT novo** (se não existe equivalente) em `DEBT.md`:
   `DEBT-54 (ou próximo número) — Consumer integral de
   StyleDelta em layout` com lista de gaps.

3. **Roadmap revisto para fecho de DEBT-1** em relatório do
   passo, estimando número de passos e escopo.

4. **Decisão sobre ADR-0033**: reinterpretar ou criar ADR nova
   que formalize "DEBT-1 inclui consumo integral".
   Registada no diagnóstico.

Este passo **não**:

- Toca código de L1, L2, L3, L4.
- Toca testes.
- Implementa qualquer parte do shaping.
- Fecha DEBT-1.
- Fecha ADR-0053 (font dict pendente).

---

## Contexto estratégico

Último passo antes da fase de consumo:

- **135** (este): diagnóstico de shaping.
- **136+**: implementação de consumers conforme roadmap
  saído do 135.
- **Fecho DEBT-1**: quando todos os gaps identificados em
  135 forem resolvidos. Número total de passos desconhecido
  até 135 encerrar.

---

## Sub-passos

### 135.A — Inventário do consumer actual

**A.1 — Mapear `StyleDelta` → layout**:

Começar em `01_core/src/entities/style_chain.rs` (pós-134) e
rastrear cada campo:

```
StyleDelta campo → quem lê → como consome → efeito observável
```

Para cada campo (bold, italic, size, fill, weight, tracking,
leading, lang, font), registar:
- Sítio onde o valor é lido (ficheiro, linha).
- Transformação aplicada (se há).
- Efeito final (glyph selection, positioning, styling, rendering).
- Se é **inerte** (capturado sem uso) ou **activo** (afecta PDF).

**Expectativa baseada no DEBT.md**:
- `bold`, `italic`, `size`: activo (Passo 30 + `TextStyle::from`).
- `fill`: activo (Passo 102).
- `weight`: provavelmente **inerte**.
- `tracking`, `leading`: **inerte** (confirmado relatórios
  127, 128, 134).
- `lang`, `font`: **inerte** (confirmado relatórios 131B, 132B).

**A.2 — `TextStyle` como ponte actual**:

`grep -rn "TextStyle" 01_core/src/ 03_infra/src/`.

Registar:
- Campos de `TextStyle`.
- `TextStyle::from(&StyleChain)` — que campos transfere.
- Onde `TextStyle` é consumido (layout, export).
- DEBT-48 estado actual (DEBT.md).

**A.3 — `FontBook` em L1**:

`grep -rn "FontBook" 01_core/src/`.

Registar:
- Se o tipo existe.
- Se tem `select` ou equivalente.
- Se integra com `FontWeight`/`FontStretch` existentes em
  `font_book.rs`.
- Se usa `FontList` (pós-132B) de `font_list.rs`.
- Dependência em crates externas (ttf-parser, fontdb, etc.).

**A.4 — `rustybuzz` e shaping pipeline**:

`grep -rn "rustybuzz\|shape" 01_core/src/ 03_infra/src/`.

Registar:
- Onde `rustybuzz` é chamado.
- Se é L1 (processamento puro) ou L3 (I/O de fontes).
- Input esperado (font bytes, glyphs, features).
- Output (posicionamentos).
- Se processa `StyleChain`/`StyleDelta` directamente ou via
  intermediário.

**A.5 — Pipeline completo**:

Rastrear do Typst source até PDF:

```
texto Typst → parse → AST → eval → Content → layout → frames → PDF
```

Para cada etapa, registar:
- Ficheiro principal.
- Que informação sobre styles entra/sai.
- Onde `StyleChain` viaja e onde colapsa para `TextStyle`.

### 135.B — DEBTs existentes relacionados

**B.1 — Ler DEBT.md Secção 1 inteira**:

`view /mnt/user-data/uploads/DEBT.md` (ou caminho equivalente
no repo).

Para cada DEBT aberto, classificar:
- Relacionado com shaping (consumer de styles).
- Relacionado com infraestrutura de layout.
- Não relacionado.

**B.2 — DEBT-48 em particular**:

Segundo DEBT.md linha 116 ("Novo DEBT-48 aberto"), DEBT-48
existe para `TextStyle` como vista achatada. Ler secção
completa. Registar:
- Escopo exacto.
- Se cobre parte do que este passo identifica.
- Se precisa ser expandido ou substituído.

**B.3 — Candidatos registados nos relatórios 126-134**:

Resumos e relatórios mencionam:
- "Consumer `tracking` em layout" (pós-127).
- "Consumer `weight` em layout (M-L)" (pós-127).
- "Materializar `Region`" (pós-131B).
- "Materializar `Dir`" (pós-131B).
- "Consumer integral" (pós-132B).
- "Autorizar `regex` em L1" (pós-132B, ligado a font dict).

Registar lista completa. Cada um é candidato a passo futuro ou
a DEBT satélite.

### 135.C — Gap analysis

Para cada gap identificado em A/B, registar:

- **Gap concreto**: "propriedade X não tem consumer".
- **Dependências**: que infraestrutura falta.
- **Estimativa**: XS / S / M / L.
- **Bloqueios**: decisões arquitecturais pendentes.

Tabela final esperada:

```
| Gap | Deps | Estimativa | Bloqueios |
|-----|------|------------|-----------|
| Consumer weight (selecção variante) | FontBook::select | S | — |
| Consumer tracking | Layout positioning | S | — |
| Consumer leading | Layout vertical | S | — |
| Consumer font (string + array) | FontBook::select + fallback chain | M | — |
| Consumer font dict (covers) | Covers concreto + regex | M | ADR de regex |
| Consumer lang (shaping features) | rustybuzz integration | M | Integração L1 |
| Consumer lang (hyphenation) | Crate hifenização | M | Autorização crate |
| Remover TextStyle como ponte | DEBT-48 | L | Refactor extenso |
```

### 135.D — Decisão sobre ADR-0033

Três opções:

**Opção a — Reinterpretar ADR-0033 existente**:
Adicionar nota à ADR-0033 explicitando que "paridade funcional"
inclui consumo integral, não só captura. Decisão absorvida.

**Opção b — Criar ADR-0055 (ou próximo) explícita**:
"DEBT-1 fecho: critério inclui consumo integral de `StyleDelta`
em layout". ADR dedicada formaliza a mudança de critério.

**Opção c — Deixar ADR-0033 como está e documentar só no
DEBT novo**:
A decisão vive no DEBT-54; ADR-0033 permanece na sua forma
actual.

Recomendação pragmática do diagnóstico: **opção b** — ADR
dedicada porque a mudança é de critério de fecho de um DEBT
central, não de nota marginal. Precedente: ADR-0052 e ADR-0053
formalizaram materializações específicas como decisões próprias.

### 135.E — Escrever diagnóstico

Ficheiro: `00_nucleo/diagnosticos/diagnostico-shaping-passo-135.md`.

**Template**:

```markdown
# Diagnóstico do estado actual de shaping — Passo 135

**Data**: 2026-MM-DD
**Motivação**: reavaliação do critério de fecho de DEBT-1 —
`StyleDelta` capturado precisa de consumer integral para
paridade end-to-end com vanilla.

---

## 1. Estado do consumer actual

### 1.1 Mapa StyleDelta → efeito observável

[Tabela por campo com: sítio de leitura, transformação, efeito]

### 1.2 TextStyle como ponte

[Campos, onde é usado, DEBT-48 estado]

### 1.3 FontBook em L1

[Existe? Select implementado? Integração com FontList?]

### 1.4 rustybuzz e shaping pipeline

[Onde é invocado, camada, inputs/outputs]

### 1.5 Pipeline completo Typst → PDF

[Diagrama ou descrição passo a passo]

## 2. DEBTs existentes relacionados

### 2.1 DEBTs abertos relevantes

[Lista com classificação: shaping / layout / outro]

### 2.2 DEBT-48 análise

[Escopo actual, cobre parte do problema?]

### 2.3 Candidatos registados em relatórios anteriores

[Lista completa com referência a relatórios]

## 3. Gap analysis

[Tabela completa de gaps com deps, estimativas, bloqueios]

## 4. Roadmap revisto para fecho de DEBT-1

[Sequência de passos estimada]

## 5. Decisão sobre ADR-0033

[Opção escolhida e razão]

## 6. DEBT novo proposto

[Escopo do DEBT-54 (ou próximo número)]
```

### 135.F — Criar DEBT novo

Ficheiro: `00_nucleo/DEBT.md` (editar).

Adicionar à Secção 1:

```markdown
## DEBT-54 — Consumer integral de StyleDelta em layout

**Aberto em**: Passo 135 (2026-MM-DD).
**Relacionado com**: DEBT-1 (fecho depende deste), DEBT-48
(`TextStyle` como ponte), ADR-0033 (paridade), ADR-0053
(`font` dict deferido).

### Contexto

Passos 126-134 capturaram lista canónica DEBT-1 em `StyleDelta`.
Valores são inertes: layout actual usa `TextStyle` plano
(DEBT-48) que cobre apenas bold/italic/size/fill. Outras
propriedades (weight, tracking, leading, lang, font) são
capturadas sem efeito observável no PDF.

Para fechar DEBT-1 com paridade end-to-end (ADR-0033 lido
literal), cada propriedade precisa de consumer em layout.

### Gaps identificados (diagnóstico 135)

[Lista da tabela de 135.C]

### Âmbito

- [ ] Consumer weight (selecção variante).
- [ ] Consumer tracking (positioning).
- [ ] Consumer leading (vertical).
- [ ] Consumer font string + array.
- [ ] Consumer font dict (depende de regex + Covers).
- [ ] Consumer lang (shaping features OpenType).
- [ ] Consumer lang (hyphenation).
- [ ] Remover/substituir `TextStyle` plano (coordenar com DEBT-48).

### Dependências

- `FontBook` integrado em L1 (verificar 135.A.3).
- `rustybuzz` em L1 (verificar 135.A.4).
- Autorização de `regex` (para font dict).
- Crate de hifenização (se não existe).

### Roadmap estimado

[Sequência saída do 135.C]
```

Contagem topo do DEBT.md: `11 DEBTs abertos` → `12 DEBTs abertos`.

### 135.G — ADR (se opção b escolhida)

Se 135.D escolhe opção b:

Ficheiro: `00_nucleo/adr/typst-adr-NNNN-critério-fecho-debt-1.md`.

```markdown
# ⚖️ ADR-NNNN: Critério de fecho de DEBT-1 inclui consumo integral

**Status**: `EM VIGOR`
**Data**: 2026-MM-DD

## Contexto

DEBT-1 foi aberto no Passo 30 com âmbito "StyleChain +
propriedades adicionais". Interpretação inicial: captura.
Relatórios 126-134 revelaram que a lista canónica pode estar
capturada e ainda assim não haver efeito observável em PDF.

ADR-0033 (paridade funcional) lido literal inclui output, não
só input processing.

## Decisão

DEBT-1 só fecha quando:
1. Cada propriedade de `StyleDelta` tem consumer em layout.
2. Output PDF é observacionalmente idêntico ao vanilla para
   inputs equivalentes.

Captura sem consumer é **estado intermédio**, não fecho.

## Alternativas consideradas

[Tabela opções a, b, c de 135.D]

## Consequências

- DEBT-1 permanece aberto até DEBT-54 resolver.
- Roadmap de fecho de DEBT-1 expande significativamente.
- Paridade ADR-0033 ganha dimensão observável explícita.

## Referências

- DEBT-1 (DEBT.md).
- DEBT-54 (aberto em 135).
- ADR-0033.
- Passo 135 diagnóstico.
```

### 135.H — Relatório

Ficheiro: `typst-passo-135-relatorio.md`.

Incluir:
- Sumário do inventário (5 secções).
- Gaps identificados (tabela).
- DEBT-54 aberto.
- Decisão ADR (a/b/c + razão).
- Roadmap revisto para fecho de DEBT-1 em termos de número
  estimado de passos.
- Próximo passo sugerido (136) — provavelmente o gap mais
  pequeno da tabela.

---

## Verificação

1. Diagnóstico criado com 6 secções preenchidas com factos
   concretos do código (não placeholders).
2. DEBT-54 aberto em `DEBT.md`.
3. Contagem de DEBTs actualizada.
4. ADR nova criada (se opção b) ou ADR-0033 anotada (se opção a).
5. Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
   `04_wiring/` tocado.
6. `cargo test --workspace` continua em 1084, 6 ignorados.
7. `crystalline-lint` zero violations.
8. Relatório 135.H escrito.

---

## Critério de conclusão

1. Diagnóstico existe com 5 secções de inventário + 1 de
   gap analysis + 1 de decisão ADR + 1 de DEBT novo.
2. Tabela de gaps concreta com estimativas (XS/S/M/L).
3. DEBT-54 (ou próximo número) aberto.
4. Decisão ADR-0033 registada (a/b/c).
5. Roadmap revisto com número estimado de passos até fechar
   DEBT-1.
6. Zero código tocado.
7. `cargo test --workspace` inalterado.
8. `crystalline-lint` zero violations.
9. Relatório escrito com próximo passo sugerido.

---

## O que pode sair errado

- **Inventário revela que muito mais propriedades já têm
  consumer do que esperado**: óptimo — gap menor, roadmap
  mais curto. Registar e ajustar estimativas em baixo.

- **Inventário revela que muito menos infraestrutura existe
  do que esperado** (ex: `rustybuzz` não integrado, `FontBook`
  ausente, `TextStyle` muito mais superficial): gap maior,
  roadmap mais longo, DEBT-54 ganha escopo significativo.
  Registar honestamente — não minimizar.

- **DEBT já existe para shaping** (e não é DEBT-48): ajustar
  plano. Em vez de abrir DEBT-54, expandir DEBT existente.

- **Decisão ADR é mais complexa do que a/b/c**: se o executor
  encontrar quarta opção válida, registar e propor.

- **Inventário exige leitura de > 20 ficheiros**: aceitar,
  passo pode ser grande mas é L0-puro e o output justifica.
  Se ultrapassa 3h de leitura, pausar e reportar para decidir
  se continuar ou fragmentar.

- **Pipeline cristalino não segue pipeline vanilla de perto**:
  registar divergências estruturais. Podem afectar estratégia
  de consumer.

---

## Notas operacionais

- **Este é o passo de maior retorno por unidade de trabalho**
  em toda a série recente. Inventário honesto do shaping evita
  escrever enunciados baseados em estrutura imaginada.

- **Resistir ao impulso de sugerir implementação**: 135 é só
  diagnóstico. Decisões de "como implementar consumer X" ficam
  para 136+ onde haverá diagnóstico dedicado se necessário.

- **Padrão "só diagnóstico" tem agora três aplicações**
  planeadas: 131A (Lang), 132A (FontList), 135 (shaping).
  Pattern consolida-se como disciplina reusável. Candidato
  registado em 132A para formalizar no ADR-0034 continua a
  valer.

- **DEBT-54 é rastreador, não trabalho**: o DEBT lista gaps
  com referências aos passos futuros que os resolvem. Fecha
  quando todos os gaps são atacados.

- **ADR nova (se escolhida opção b)** formaliza mudança de
  critério. Precedente nesta série: ADR-0052 + 0053
  documentaram materializações como decisões próprias. Esta
  documenta mudança de critério como decisão própria.

- **Próximo passo (136) só se escreve depois deste encerrar**.
  Razão: sem o diagnóstico, o 136 seria adivinhação.

- **Candidato `eval_with_warnings`** continua pendente.
  Não é shaping. Passo independente.

# Passo 135 — Relatório (diagnóstico de shaping + DEBT-52 + ADR-0054)

**Data**: 2026-04-24
**Precondição**: Passo 134 encerrado; 1084 total tests; zero
violations; 53 ADRs activas; 11 DEBTs abertos. Lista canónica
DEBT-1 capturada em `StyleDelta`.
**Natureza**: passo L0-puro. **Sem código**. **Sem testes**.
Terceira aplicação do padrão "diagnóstico + ADR" (após 131A/B,
132A/B).
**ADR**: **ADR-0054** criada em status `EM VIGOR`.
**DEBT**: **DEBT-52** aberto.

---

## Sumário

Mudança estratégica formalizada: **roadmap original "135 fecha
DEBT-1 em documentação" foi reavaliado**. Diagnóstico empírico
revelou que 5 de 10 campos de `StyleDelta` são inertes — sem
consumer em layout. ADR-0033 (paridade) lido literal exige
output observacional equivalente ao vanilla.

**Três artefactos L0 produzidos**:

1. **Diagnóstico** `diagnostico-shaping-passo-135.md` (~220
   linhas) — 7 secções: estado consumer, TextStyle ponte,
   FontBook, rustybuzz (ausente), pipeline completo, gaps com
   tabela de dependências e estimativas, roadmap revisto,
   decisão ADR, DEBT novo.
2. **ADR-0054** `critério-fecho-debt-1.md` em `EM VIGOR` —
   9 decisões, 3 perfis de paridade, alternativas, plano de
   materialização em 5 fases.
3. **DEBT-52** aberto em `DEBT.md` como rastreador; contagem
   atualizada 11 → **12** DEBTs abertos.

**Zero código tocado**. **Tests 1084 preservado**. Lint clean.

---

## 135.A — Inventário confirmatório

### Findings críticos

1. **`TextStyle` tem 5 campos**: `bold, italic, size, fill,
   heading_level`. `StyleDelta` tem **10 campos**. Gap: 5
   campos inertes (`weight, tracking, leading, lang, font`).
2. **`From<&StyleChain>` mapeia apenas os 5**. Novos campos
   não atravessam para o frame.
3. **`FrameItem::Text.style: TextStyle`** — bloqueio físico
   para propriedades ausentes em TextStyle chegarem ao export.
4. **PDF export usa `(bold, italic)` para selecção `F1/F2/F3`
   Helvetica hardcoded**. Sem font embedding real.
5. **`rustybuzz` ausente** em todo o repo (L1, L3, workspace).
   Sem shaping engine.
6. **`FontBook::select` existe em L1** — integração com
   `FontList` (132B) é possível mas nunca feita.

### Mapa StyleDelta → efeito

- **Activo** (5): bold, italic, size, fill, heading_level.
- **Inerte** (5): weight, tracking, leading, lang, font.

---

## 135.B/C — DEBTs existentes + Gap analysis

### DEBTs relacionados

- **DEBT-1** (central): fecho depende de consumer integral.
- **DEBT-48** (ENCERRADO): `TextStyle` como ponte aceite.
  Extensão > substituição.
- Outros 10 DEBTs abertos: não relacionados com shaping.

### Gap summary

| Dificuldade | # de gaps | Exemplos |
|-------------|-----------|----------|
| XS | 2 | estender TextStyle, propagar via From |
| S | 3 | tracking, leading, weight faux-bold |
| M | 4-5 | font string, font array, lang hyphenation, PDF embedding |
| L | 1 | lang shaping features (requer rustybuzz) |
| XL | 1 | shaping engine completo |

**Paridade observacional graded alcançável em 4-8 passos**
(fase A + B + C). XL (rustybuzz) explicitamente fora de DEBT-1.

---

## 135.D — Decisão ADR-0033

**Opção (b) escolhida**: criar **ADR-0054** dedicada.

Razões:
- Mudança de critério de fecho de DEBT central merece ADR própria.
- Precedente: ADR-0052/0053 formalizaram materializações como
  decisões próprias.
- Opção (a) "nota em ADR-0033" enterraria decisão sob princípio
  geral.
- Opção (c) "só DEBT" perderia visibilidade em revisões futuras.

---

## 135.E — Diagnóstico escrito

`diagnostico-shaping-passo-135.md` cobre 7 secções:
1. Estado consumer (mapa + TextStyle + FontBook + rustybuzz +
   pipeline completo).
2. DEBTs existentes.
3. Gap analysis com tabela completa.
4. Roadmap revisto (fases A–E).
5. Decisão ADR (opção b).
6. DEBT novo proposto.
7. Resumo executivo.

---

## 135.F — DEBT-52 aberto

Entrada adicionada em DEBT.md Secção 1 (antes de "Secção 2 —
DEBTs encerrados"). Contagem atualizada no header: **11 → 12
abertos**.

Escopo: rastreador de 8 gaps de consumer (ver tabela). Não é
trabalho per se; lista items que passos futuros resolvem.

Dependências registadas:
- `FontBook::select` (existe em L1).
- PDF font embedding em L3 (infra nova).
- Crate `regex` (não autorizada; bloqueia font dict).
- Crate hifenização (não autorizada; bloqueia lang).

Critério de fecho: cada gap atacado OU explicitamente scope-out
com ADR.

---

## 135.G — ADR-0054 criada

**Status**: `EM VIGOR` imediatamente (formaliza decisão tomada
neste passo). Contém:
- 3 decisões-chave: consumer integral como critério, perfil
  observacional graded, DEBT-52 como gate.
- 3 perfis de paridade considerados (bit-perfect, visual,
  observacional graded) — escolha graded justificada.
- 5 alternativas em tabela com prós/contras.
- Plano de materialização em 5 fases (A-E).

**rustybuzz explicitamente fora de DEBT-1**: documentado como
DEBT-52 fase E opcional, possivelmente série dedicada.

---

## Verificação

1. ✓ Diagnóstico com 7 secções de factos concretos.
2. ✓ DEBT-52 aberto em DEBT.md com âmbito claro.
3. ✓ Contagem 12 DEBTs abertos registada.
4. ✓ ADR-0054 criada em `EM VIGOR`.
5. ✓ Zero ficheiros L1/L2/L3/L4 tocados (`git status`:
   `M DEBT.md` + 3 new L0 files).
6. ✓ `cargo test --workspace`: 853+186+24+21 = 1084.
7. ✓ `crystalline-lint` zero violations.

---

## Ficheiros produzidos

| Ficheiro | Natureza | Linhas |
|----------|----------|-------:|
| `00_nucleo/diagnosticos/diagnostico-shaping-passo-135.md` | L0 diagnóstico | ~230 |
| `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` | L0 ADR EM VIGOR | ~160 |
| `00_nucleo/DEBT.md` | L0 — DEBT-52 adicionado + header atualizado | +75 linhas |
| `00_nucleo/materialization/typst-passo-135-relatorio.md` | L0 relatório | ~180 |

Zero código/teste tocado.

### Números finais

| Métrica | Antes (134) | Depois |
|---------|------:|-------:|
| L1 tests | 853 | 853 |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1084** | **1084** |
| Violations | 0 | 0 |
| ADRs activas | 53 | **54** (ADR-0054 EM VIGOR) |
| **DEBTs abertos** | **11** | **12** (+DEBT-52) |

---

## Roadmap revisto para fecho de DEBT-1

### Fase A (1 passo, XS) — Extensão TextStyle

**136**: adicionar `weight/tracking/leading/lang/font` a
`TextStyle` + actualizar `From<&StyleChain>`. Nenhum efeito
visível, mas infra pronta para consumers.

### Fase B (3 passos, S cada) — Consumers simples

**137**: consumer `tracking`.
**138**: consumer `leading`.
**139**: consumer `weight` faux-bold (PDF stroke).

### Fase C (4-5 passos, M cada) — Consumers avançados

**140**: consumer `font` string (via FontBook::select + infra
PDF).
**141**: consumer `font` array (fallback chain).
**142**: consumer `lang` hyphenation (crate nova + ADR).
**143** (opcional): embedding PDF fonts reais.

### Fase D (opcional, M) — Font dict

**ADR-0054bis**: autorizar `regex` + `Covers` concreto.
1 passo dedicado.

### Fase E (opcional, XL) — Shaping engine

**rustybuzz integrado**: fora de DEBT-1; possivelmente série
100+ linhas dedicada. Não bloqueia fecho.

### Fecho DEBT-1

Após **Fase A + B + C** completos. Estimativa total:
**4-8 passos adicionais = 10-16h**.

---

## Lições

1. **"Diagnosticar antes de adivinhar"**: roadmap original
   assumia "fechar DEBT-1 em 135 como documentação". Um
   inventário de 10 minutos revelou que 5 de 10 campos eram
   inertes — transformando fecho em mais 4-8 passos. **135
   custou 1h, poupou semanas de confusão**.

2. **Terceira aplicação do padrão L0-puro**: 131A (Lang),
   132A (FontList), 135 (shaping). Padrão consolidado como
   disciplina reusável. Cada L0 passo tem retorno assimétrico:
   investimento pequeno (leitura + escrita), evita refactors
   não planeados.

3. **"Captura ≠ paridade"**: distinção explícita era implícita
   antes do 135. ADR-0054 formaliza. Futuros DEBTs podem
   beneficiar de clarificar este ponto desde o início.

4. **rustybuzz é tensão oculta**: o stack cristalino assumiu
   shaping integração em algum ponto, mas o ponto nunca chegou.
   Documentar como XL explícito em DEBT-52 fase E/Fase L
   evita que "um dia alguém começa" sem âmbito claro.

5. **Perfil observacional graded é honesto**: tentar "paridade
   visual" sem rustybuzz é receita para frustração. Perfil
   graded diz "estas métricas sim, shaping fica para depois"
   — utilizador informado, contracto claro.

6. **DEBT-52 como rastreador**: diferente dos DEBTs anteriores
   (1-50) que listam trabalho finito. DEBT-52 lista **gaps**
   que cada um é seu próprio trabalho. Pattern novo; útil
   quando área precisa de coordenação mas não de execução
   única.

---

## Estado pós-Passo 135

### DEBT-1 — estado

**Permanece aberto**. Critério de fecho revisto por ADR-0054.
Lista canónica **capturada** mas **não consumida**. Roadmap
claro para fechar em 4-8 passos.

### DEBT-52 — aberto

Rastreador de 8 gaps de consumer. Dependências documentadas.
Critério de fecho explícito.

### ADR-0054 — EM VIGOR

Formaliza critério. Perfil graded escolhido. Alternativas
registadas.

### Próximo passo sugerido: **Passo 136** (fase A, XS)

Estender `TextStyle` com 5 campos novos + atualizar
`From<&StyleChain>`. Zero efeito visível mas infra pronta.
Estimativa: < 1h.

### Candidatos pendentes continuam

- `eval_with_warnings` test harness helper (Passo 127, 132B).
  Custo cresce; priorizar após fecho DEBT-1 ou em passo
  dedicado intermédio.
- `Region`, `Dir` materialização (131B) — independentes.
- ADR-0054bis (autorizar `regex`) — quando font dict chegar.

---

## Mudança observável face ao roadmap original

| Esperado | Real |
|----------|------|
| 135 fecha DEBT-1 em documentação | 135 **abre DEBT-52** e revisa critério de fecho DEBT-1 via **ADR-0054** |
| 11 DEBTs abertos pós-135 | 12 DEBTs abertos (+DEBT-52) |
| Fecho DEBT-1 imediato | Fecho DEBT-1 em +4-8 passos (fase A+B+C) |

**Esta revisão é o produto do passo**. Sem ela, fecho prematuro
esconderia gap significativo; utilizadores finais veriam
warnings capturados "silenciosamente" sem efeito no PDF.

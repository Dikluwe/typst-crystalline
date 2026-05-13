# Passo 214 — Recálculo das categorias restantes pós-M9c

**Série**: 214 (passo único — administrativo XS
documental ampliado).
**Marco**: nenhum (segundo passo pós-M9c; estende
método P213 às restantes categorias).
**Tipo**: recálculo de cobertura paralelo + actualização
de documentos canónicos.
**Magnitude**: XS-S (~1h-1h30min documental — 5
categorias em paralelo vs 1 em P213).
**Pré-condição**: P213 concluído (Introspection
recalculada para 83%; método estabelecido; pattern
"diagnóstico-recálculo pós-marcos" N=1 documentado);
inventário 148 §A.1–§A.9 com footnotes 1-38; blueprint
§2.1 com 1 categoria recalculada (Introspection) +
8 desactualizadas; ADR-0076 ACEITE 2026-05-12; tests
1939 verdes; 0 violations.
**Output**: 1 ficheiro (relatório curto recálculo
ampliado) + 2 ficheiros canónicos editados.

---

## §1 Trabalho

P213 recalculou só a categoria Introspection. Auditoria
posterior detectou que **as outras 8 categorias do quadro
§2.1 também acumularam pendência documental** pelo mesmo
mecanismo (política "reescrita ampla fora-de-escopo"
pós-P204H/P205E/P206E/P207E/P208D/P209E/P210C/P211A/
P212). Recalcular as restantes 8 categorias replicando
exactamente o método P213 (reconta empírica + actualização
de Tabela A inventário 148 + actualização §2.1 blueprint
+ marca cirúrgica).

**Decisão central de P214**: aplicar método P213 a todas
as categorias com pendência detectada, escala "todas de
uma vez" vs P213 "uma categoria por passo". Justificação:
estimativa de magnitude paralela ~5× a P213 mas zero
overhead repetido (cada categoria carrega 5min de
auditoria + 5min de edição = 10min × 8 = ~80min total);
pattern emergente "diagnóstico-recálculo pós-marcos"
cresce de N=1 (P213) para **N=9** (P213 + P214 a 8
categorias) — saltando limiar formalização N=3-4. Sem
ADR meta nesta sessão (per política P158).

**Decisão alternativa rejeitada**: P214A/B/C/... uma
categoria por sub-passo. Rejeitada porque:
- Per spec do humano "o certo é recalcular tudo".
- Categorias são independentes — não há dependências
  hard.
- Cada recálculo é mecânica idêntica; replicação
  paralela é honesta.
- Inflar para N sub-passos viola anti-inflação
  (precedente 9 aplicações cumulativas pós-P205D).

Reuso de dados toda a trajectória pós-2026-04-25 (sem
recolha nova — séries P154A/B + P155 + P156B-L + P157A-C
+ P158A-C + P159A-G + P207-P212 + P213):

- Histórico cumulativo nas footnotes 1-38 já existentes.
- ADRs ACEITES/IMPLEMENTADAS preservadas (60-77).
- Estado factual confirmável via `grep` em
  `01_core/src/entities/`/`stdlib/`/etc.

---

## §2 Cláusulas (8)

### C1 — Reconta empírica por categoria

Auditoria empírica das 8 categorias restantes face ao
estado factual pós-P212. Para cada categoria, classificar:
- Pré-P214 (estado actual no quadro §2.1).
- Pós-P214 esperado (após contar entradas individuais
  na §A correspondente + cruzar com passos materializados
  pós-2026-04-25).

| Categoria | §2.1 actual | Distribuição actual (Tabela A) | Hipótese pós-P214 |
|-----------|-------------|---------------------------------|----------------------|
| Math | 92% | 6/6/1/0/0 = 13 | inalterada (sem trabalho pós-2026-04-25) |
| Foundations stdlib | 67% | 9/1/4/1/0 = 15 | provável inalterada ou +1pp |
| `#let`/`#set`/`#show` | 62% | 7/1/4/1/0 = 13 | provável inalterada (subset stable) |
| Markup syntactic | 61% | 8/3/3/4/0 = 18 | **provável alteração** — `quote`/`terms`/smart-quotes movidos em P154B+P155 |
| Visualize | 54% | 6/1/1/5/0 = 13 | provável inalterada ou +1pp |
| Text features | 52% | 7/5/1/8/2 = 23 | provável alteração — smart-quotes activado em P155 |
| Model | 45% | 7/4/7/4/0 = 22 | **alteração esperada** — várias entradas P157/P158/P159 reclassificadas (table/figure/cite/bibliography) |
| Layout | 38% | 13/1/3/1/0 = 18 | **alteração esperada** — Tabela A já reflecte ⁵⁶⁸ recálculos P156B-L mas §2.1 não actualizada |

Hipótese central: **3 categorias mudaram materialmente**
(Markup/Model/Layout); **2 podem mudar** (Text/Visualize);
**3 estáveis** (Math/Foundations/let-set-show).

Verificação per-categoria:
- Cruzar §A entradas marcadas com passos pós-P148.
- Contar reclassificações ausente→implementado e
  implementado→implementado⁺ desde 2026-04-25.
- Recalcular distribuição e percentagem
  `(impl + impl⁺) / total`.
- Se distribuição mudou: registar reclassificações.
- Se distribuição não mudou: registar "inalterada".

Se reconta produzir distribuição inesperada vs hipótese:
registar `P214.div-N` e ajustar antes de C2.

### C2 — Auditoria Tabela A vs §2.1 blueprint

P156B (Layout) e P154A/B/P155 (Model) já actualizaram a
Tabela A do inventário 148 (via footnotes ⁵⁶⁸¹⁰¹²¹³¹⁵¹⁷¹⁹²¹
em Layout e ¹²³²²²⁴²⁹ em Model). Mas **o §2.1 do blueprint
nunca foi sincronizado** desde 2026-04-25.

C2 audita literal:
- Calcular cobertura categoria via `(impl + impl⁺) / total`
  per linha da Tabela A actual.
- Comparar com valor declarado em §2.1.
- Se divergente: anotar Δ por categoria.

Esperado:

| Categoria | §2.1 declarado | Tabela A actual (calculado) | Δ |
|-----------|----------------|-----------------------------|---|
| Layout | 38% | 14/18 = 78% | +40pp |
| Model | 45% | 11/22 = 50% | +5pp |
| Markup syntactic | 61% | 11/18 = 61% | 0 (se sem alteração pós-P148) |
| Text features | 52% | 12/23 = 52% | 0 (se inalterado) |
| Visualize | 54% | 7/13 = 54% | 0 (se inalterado) |
| Foundations | 67% | 10/15 = 67% | 0 |
| `#let`/`#set`/`#show` | 62% | 8/13 = 62% | 0 |
| Math | 92% | 12/13 = 92% | 0 |

Diferença material esperada apenas em Layout (+40pp) e
Model (+5pp). Outras categorias provavelmente já alinhadas
ou pendentes de reclassificação interna (entradas que
mudaram dentro de uma categoria sem mover percentagem).

### C3 — Reclassificações internas detectáveis

C2 dá só agregado. C3 audita reclassificações **dentro**
de categorias que podem não mover a percentagem mas
modificam a distribuição. Foco em entradas vanilla
materializadas após 2026-04-25 que ainda figurem como
`ausente` na §A correspondente:

**§A.1 Markup syntactic** — verificar:
- `> blockquote` (linha 54): declarado `ausente`; vanilla
  `model/quote.rs`. **P155 materializou** `Content::Quote`
  e markup smart-quote insertion. Possível
  ausente→implementado.
- `/ term: definition` (linha 55): declarado `ausente`;
  vanilla `model/terms.rs`. **P154B materializou**
  `Content::Terms` + `Content::TermItem`. Possível
  ausente→implementado.
- Smart quotes (linha 60): declarado `ausente`;
  vanilla `text/smartquote.rs`. **P155 materializou**
  via `rules/lang/quotes.rs` + alternância em
  `eval_markup`. Possível ausente→implementado.

**§A.3 Text features** — verificar:
- Sem entradas obvias para reclassificar (todas as
  reclassificações pós-P148 já registadas em footnotes
  de Text features se aplicável).

**§A.5 Layout** — já reclassificada P156B; verificar se
distribuição local da §2.1 reflecte Tabela A.

**§A.6 Model** — já reclassificada P154A/B + P155 +
P157A/B/C + P158A-C + P159A-G; verificar
sincronização §2.1.

Se C3 detectar reclassificações novas em §A.1: registar
`P214.div-N` e propagar a Tabela A + total user-facing.

### C4 — Actualização Tabela A inventário 148

Editar
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- Para cada categoria com Δ detectado em C2 ou
  reclassificação em C3: actualizar distribuição na
  linha Tabela A.
- Recalcular total user-facing.
- Adicionar footnote 39+ por categoria recalculada
  (paridade pattern footnote 38 P213): per-entrada
  detalhe + sub-passo de fecho + deferreds.
- Reescrever §A correspondente onde houver
  reclassificação interna.

**Importante**: distinguir
- **Re-sincronização §2.1 ↔ Tabela A** (Layout +40pp,
  Model +5pp): Tabela A já correcta; só §2.1 precisa
  actualizar. Footnote 39/40 documenta sincronização,
  não recálculo material.
- **Reclassificação material** (e.g. §A.1 quote/terms/
  smart-quotes ausente→implementado): Tabela A precisa
  actualizar; footnote 41+ documenta a reclassificação
  material como P213 fez.

### C5 — Actualização blueprint §2.1

Editar `00_nucleo/diagnosticos/blueprint-projecto.md`:

Quadro §2.1 com 8 linhas pós-P214 (apenas linhas com
Δ ≠ 0 são alteradas):

Antes (post-P213, current):
```
| Math | 92% | quase total |
| Foundations stdlib | 67% | parcial |
| `#let`/`#set`/`#show` | 62% | parcial |
| Markup syntactic | 61% | parcial |
| Visualize | 54% | parcial |
| Text features | 52% | parcial |
| Model (structural) | 45% | em curso (Fase 1 fechada) |
| Layout | 38% | gap grande |
| Introspection ⁽ᴾ²¹³⁾ | 83% | quase total (paridade arquitectural pós-M9c) |
```

Depois (post-P214, esperado):
```
| Math | 92% | quase total |
| Introspection ⁽ᴾ²¹³⁾ | 83% | quase total (paridade arquitectural pós-M9c) |
| Layout ⁽ᴾ²¹⁴⁾ | 78% | quase total (Fase 1+2+3 sub-passo 1 fechadas) |
| Foundations stdlib | 67% | parcial |
| `#let`/`#set`/`#show` | 62% | parcial |
| Markup syntactic | 61% | parcial |
| Visualize | 54% | parcial |
| Text features | 52% | parcial |
| Model (structural) ⁽ᴾ²¹⁴⁾ | 50% | em curso (Fase 1 fechada; Fase 2 table+figure+bib em curso) |
```

Ordem reordenada por cobertura decrescente (Math 92% →
Text 52%) — paridade visual com prioridade. Markers
⁽ᴾ²¹⁴⁾ adicionados onde aplicável.

### C6 — Marca blueprint §3.0undecies

Adicionar marca cirúrgica nova após §3.0decies P213,
antes de §3.1.

**Decisão sobre forma**:
- **Opção α** — marca §3.0undecies idêntica a §3.0decies
  P213 (paridade pattern marca-por-fecho).
- **Opção β** — marca §3.0undecies expandida (porque
  cobre múltiplas categorias).
- **Opção γ** — Opção α com nota explícita "recálculo
  ampliado a 8 categorias paralelo a P213" + tabela
  Δ per categoria.

Hipótese provável: **Opção γ** — preserva pattern
minimal mas regista escala ampliada.

Conteúdo da marca:
- Δ por categoria (tabela 8 linhas).
- Causa: continuação da política "reescrita ampla
  fora-de-escopo" que P213 endereçou parcialmente.
- Pattern emergente "diagnóstico-recálculo pós-marcos"
  cresce N=1 → 9 (P213 + 8 categorias P214).
- Subpadrão "passo administrativo XS" cresce N=3 → 4
  (atinge limiar formalização N=3-4 ultrapassado;
  ADR meta candidata).
- Política "sem novas reservas" preservada.

### C7 — Verificação final

```
grep -n "P214" 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md
grep -n "P214" 00_nucleo/diagnosticos/blueprint-projecto.md
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
```

Critério:
- §2.1 blueprint mostra cobertura pós-recálculo em todas
  as 8 categorias (3-9 marcadas com ⁽ᴾ²¹⁴⁾ conforme C5).
- Tabela A inventário 148 actualizada com footnotes
  novas (39+ ou consolidada).
- Tests 1939 verdes (Δ 0 — sem código tocado).
- 0 violations.

### C8 — Decisão sobre próximo trabalho

P214 fecha pendência documental ampla. Próximo passo
fica em aberto para decisão humana entre:

- **Materialização** de candidatos identificados (Bloco
  A `measure`, Bloco B `position()`, Layout Fase 3
  columns/colbreak DEBT-56, Model bibliography hayagriva
  DEBT-55, etc.).
- **ADR meta** "passo administrativo XS recálculo
  cobertura" se N=4+ confirmado (P213 + 8 categorias
  P214 = N=9, largamente acima de N=3-4).
- **Outro módulo** ou refactor.

C8 documenta opções em §8 do relatório; não fixa
nenhuma.

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-214-relatorio.md`.

Estrutura (~7-9 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Reconta empírica per categoria (tabela 8 linhas:
  declarado §2.1 / Tabela A actual / Δ / fonte
  pós-2026-04-25).
- §3 Tabela A inventário 148 actualizada (antes/depois
  per categoria com Δ; total user-facing recalculado).
- §4 Blueprint §2.1 reordenado por cobertura
  decrescente + marca §3.0undecies.
- §5 Decisões substantivas (Opção γ marca; "tudo de
  uma vez" vs sub-passos; reclassificações internas §A.1
  se detectadas).
- §6 Métricas pós-P214 (cobertura user-facing total
  recalculada via aritmética honesta).
- §7 Pattern emergente "diagnóstico-recálculo
  pós-marcos" cresce N=1 → 9; subpadrão administrativo
  XS cresce N=3 → 4 ultrapassa limiar formalização.
- §8 Próximo passo (3-4 opções; decisão humana).

---

## §4 Não-objectivos

- Materializar trabalho deferred (`measure` stdlib
  expose, `position()` standalone, columns/colbreak,
  hayagriva, etc.) — todos com critério de reabertura
  documentado em ADRs 0061/0062/0066/0076.
- Promover subpadrão "passo administrativo XS" a ADR
  meta neste passo — diferida para passo dedicado
  futuro mesmo com N=9 ultrapassando limiar (per
  política "sem novas reservas").
- Reabrir P213 — recálculo Introspection preservado
  intacto.
- Reescrever §3.1 datado 2026-04-25 do blueprint —
  preservação histórica per padrão P204H+.
- Tocar em código `.rs` — passo documental puro.
- Tocar em hashes L0 — nenhum prompt L0 alterado.
- Abrir/fechar DEBTs — nenhum DEBT tocado.
- Criar ADR nova — passo sem nova decisão arquitectural.
- Auditar pré-2026-04-25 inventário — escopo restrito
  ao Δ pós-P148.

---

## §5 Riscos a evitar

1. **Adivinhar percentagens sem auditoria empírica**:
   C1-C3 exigem cruzamento real com §A entradas e passos
   pós-2026-04-25. Pattern paralelo a P213 que verificou
   cada reclassificação contra sub-passo de fecho real.
2. **Confundir re-sincronização vs reclassificação
   material**: Layout +40pp em §2.1 é sincronização (Tabela
   A já reflecte recálculos P156B-L); §A.1 Markup
   smart-quotes ausente→implementado seria
   reclassificação material nova. C4 trata os dois casos
   distintamente.
3. **Inflar §A.1 com auditoria de "tudo que mudou
   ever"**: foco em entradas pós-2026-04-25 que afectam
   cobertura. Smart quotes / quote / terms são óbvias;
   refinos internos a entradas já implementadas não
   precisam de reclassificação se não movem categoria.
4. **Quebrar política "sem novas reservas"**: P214 é
   diagnóstico-recálculo paralelo a P213, não compromisso
   de trabalho subsequente. Candidatos identificados em
   §8 são opções, não reservas.
5. **Marca §3.0undecies inflada**: preserva pattern
   minimal de marca-por-fecho. Não reescrever §3.1.
   Tabela Δ per categoria fica em §3.0undecies; detalhe
   completo em relatório §3.
6. **Reordenar §2.1 sem justificação clara**: ordenar
   por cobertura decrescente é melhoria visual mas pode
   confundir leitores que esperam ordem categórica
   alfabética. Decidir em C5 e documentar em §5 do
   relatório.
7. **Esquecer Δ user-facing total**: 8 categorias
   recalculadas podem produzir Δ maior que P213 +2.8pp
   (que recalculou só Introspection). Aritmética
   honesta: somar Δ per categoria × peso × pp.
   Provável Δ pós-P214 estendido: ~+4-6pp adicional
   (Layout +40pp × 13% peso = +5.2pp; Model +5pp ×
   16% peso = +0.8pp; total ~+6pp).
8. **Pattern emergente saltar de N=1 para N=9 sem
   formalização**: humano fixou "tudo recalculado".
   Cumprir literal mas registar honestamente que ADR
   meta agora tem N=9 evidência empírica; promoção a
   ADR fica para passo dedicado per política P158.

---

## §6 Hipótese provável

C1 confirmará distribuição esperada — 2 categorias com Δ
material (Layout +40pp re-sincronização; Model +5pp
re-sincronização); 1 categoria com reclassificações
internas (Markup §A.1 — quote/terms/smart-quotes); 5
categorias inalteradas (Math/Foundations/let-set-show/
Visualize/Text features).

C2 confirmará desvios sincronização §2.1 ↔ Tabela A em
Layout (38% → 78%) e Model (45% → 50%).

C3 detectará 3 reclassificações em §A.1 Markup syntactic.

C4 actualizará Tabela A com footnotes 39 (consolidada
P214) ou 39+40+41 (separadas).

C5 reordenará §2.1 por cobertura decrescente; marcadores
⁽ᴾ²¹⁴⁾ em Layout, Model, Markup (3 categorias).

C6 fixará Opção γ — §3.0undecies marca com tabela Δ
per categoria.

C7 reportará tests/lint preservados.

C8 listará 3-4 opções de próximo trabalho sem fixar
(incluindo ADR meta agora com N=9 evidência).

Custo real: XS-S (~1h-1h30min documental). Sem código
tocado.

Mas é hipótese, não decisão. C1-C8 fixam-se
empíricamente.

---

## §7 Particularidade P214

P214 é estruturalmente distinto na trajectória pós-M9c:

- **Segundo passo pós-M9c** — continuação de P213 mas
  com escopo ampliado (8 categorias vs 1).
- **Primeira aplicação paralela do pattern P213** —
  pattern "diagnóstico-recálculo pós-marcos" cresce de
  N=1 (P213) para N=9 (P214 cobre 8 categorias
  individualmente). Ultrapassa limiar formalização
  N=3-4 mas sem promoção a ADR meta (per política P158).
- **Quarto passo administrativo XS** — subpadrão
  "passo administrativo XS" cresce N=3 → 4. ADR meta
  candidata real mas diferida.
- **Re-sincronização vs reclassificação material**
  documentada distintamente: Layout +40pp é
  re-sincronização (Tabela A já correcta); §A.1 Markup
  reclassificações são material (Tabela A precisa
  actualizar). Distinção nova qualitativamente.
- **Reordenação §2.1 por cobertura** — primeira
  reorganização visual do quadro categorias desde
  P148.
- **Custo agregado P213+P214 documental**: XS + XS-S
  ≈ S total (~2h cumulativo). Compara com séries
  P156C-L (10 passos materialização ~20h).

Por isso §5 risco 6 ("reordenação sem justificação") é
relevante — reordenação por cobertura é melhoria visual
mas decisão deve estar explícita no relatório §5 com
trade-off documentado (paridade visual com prioridade
vs paridade categórica com vanilla).

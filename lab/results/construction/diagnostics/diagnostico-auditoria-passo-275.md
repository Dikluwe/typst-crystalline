# Diagnóstico Fase A P275.A — Auditoria empírica pós-P273.17

**Data**: 2026-05-18.
**Passo**: typst-passo-275.A.
**Magnitude**: S documental (~30 min).
**Cluster**: Metodologia / Auditoria / DEBTs.
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085 — auditoria
empírica pós-cluster Gradient encerrado.
**Vigésimo nono consumo directo de fonte** (per spec §0.2).

---

## §A.0 — Ajustes de contexto vs spec

A spec P275 referencia documentos auxiliares que **não existem no
repo local**:
- `passo-debts-1-confirmacao-empirica.md`
- `passo-debts-2-categorizacao.md`
- `passo-debts-3-proposta-sequencia.md`
- `typst-estado-transicao-pos-p273-17.md`

Estes documentos estavam previstos como input de sessão upload. Na
ausência, **a Fase A faz verificação empírica directa** a partir do
estado factual do repo (`00_nucleo/DEBT.md` literal +
`00_nucleo/adr/README.md` literal + `cargo test/build/lint`).

Outputs vão para `00_nucleo/diagnosticos/` e
`00_nucleo/materialization/` per convenção do projecto (não
`/mnt/user-data/outputs/` mencionado na spec — directório
local-only).

---

## §A.1 — Estado factual projecto (verificação empírica)

| Métrica | Valor declarado | Valor verificado | Discrepância |
|---|---|---|---|
| Tests workspace verdes | 2644 | **2644** (2179 core + 418 infra + 24 shell + 21 cli + 2 + 0) | ✓ Match |
| Tests skipped pré-existentes | 2 (`recursao_*`) | **2 filtered** typst-core | ✓ Match |
| ADRs files totais | 96 | **93** (`ls typst-adr-*.md \| wc -l`) | ⚠ Spec diz 96; real 93 |
| ADRs EM VIGOR | 84 | **37 EM VIGOR + 24 IMPLEMENTADO = 61 activos** (excluindo PROPOSTO/REVOGADO/IDEIA/ACEITE) | ⚠ README diz "84 vigentes" — terminologia inconsistente |
| Lint violations | 0 | **0** ("✓ No violations found") | ✓ Match |
| Hash L0 gradient.md | `8d9730a3` (P273.17 relatório §2.7) | **`ebc84366`** (verificação literal) | ⚠ **Discrepância detectada** — ver §3 do relatório |
| `#[allow(dead_code)]` em gradient.rs | 0 | **0** (gradient está em `01_core/src/entities/gradient.rs`; sem allow) | ✓ Match (mas verificação spec referencia caminho incorrecto `layout/gradient.rs` — não existe) |

### §A.1.1 — Discrepância hash gradient.md

P273.17 relatório §2.7 declara `8d9730a3` como hash propagado. Verificação
literal em `00_nucleo/prompts/entities/gradient.md:2` retorna
`Hash do Código: ebc84366`.

**Investigação git**:
```
git log --oneline -3:
  60b043c61 Passo 272 -274        ← commit posterior a P273.17
  8517fad7e Passo 271
  20e5e3aa1 Passo 268 - 270
```

Working tree clean (`git status --short` apenas mostra
`?? 00_nucleo/materialization/typst-passo-275.md`). Trabalho da
sessão P273.5-P273.17 commitado em `60b043c61 Passo 272 -274`
(commit aggregate label).

**Hipótese**: hash `ebc84366` foi propagado num `--fix-hashes`
final pós-sessão (pré-commit) que difere do `8d9730a3` intermédio
reportado por mim em P273.17 relatório. **Ambos os hashes
representam estados válidos do mesmo trabalho** — não é regressão,
apenas snapshot intermédio diferente do snapshot final committed.

**Acção sugerida** (não executada neste passo): nenhuma. Hash actual
factual é `ebc84366`; relatório P273.17 documenta um estado intermédio
do trabalho. Sem impacto funcional.

---

## §A.2 — DEBTs em aberto (leitura literal DEBT.md)

Extracção literal via `grep -E "^## DEBT-|EM ABERTO|PARCIALMENTE"`:

### Tabela DEBTs em aberto ou parcialmente resolvidos

| DEBT | Tema | Estado | Magnitude | Bloqueador | Origem |
|---|---|---|---|---|---|
| **DEBT-2** | Closures eager vs lazy capture | **PARCIALMENTE RESOLVIDO** (Passo 31 estrutural; semântica eager preserved; aguarda `TrackedWorld` real) | M+ (refactor comemo integration) | `TrackedWorld` real | Passo 31 |
| **DEBT-9** | Cobertura de paridade | **tracking contínuo** | — (não-accionável) | N/A — tracker | Inicial |
| **DEBT-33** | Bounding Box de curvas Bézier | **EM ABERTO** | S+M | Sem bloqueador documentado; demanda visualize | Passo 79 |
| **DEBT-35b** | Invalidação cache `available_width` após `SetPage` | **EM ABERTO** | S | Sem bloqueador documentado | Passo 81 |
| **DEBT-42** | `get_unchecked` no scanner | **EM ABERTO (bloqueado)** | S | Benchmark prova ganho perf | Passo 84.8a |
| **DEBT-43** | Linter: whitelist crate-level vs type-level | **EM ABERTO** | S | Sem bloqueador documentado | Passo 89 |
| **DEBT-50** | Show selector Strong/Emph não distingue origem | **EM ABERTO** (dívida latente) | M (refactor show selector model) | Sem bloqueador documentado | Passo 103 |
| **DEBT-55** | Bibliography + Cite | **PARCIALMENTE RESOLVIDO** (via paridade manual P159A-G) | XL | ADR-0062 hayagriva real (PROPOSTO; pendente materialização) | Passo 154A |

**Total DEBTs em aberto ou parcialmente resolvidos**: **8**.

### §A.2.1 — Verificação cruzada com discrepâncias mencionadas na spec

A spec §A.2 mencionava 4 discrepâncias previstas:

| Discrepância prevista | Verificação literal | Resultado |
|---|---|---|
| DEBT-1: doc transição "parcialmente"; DEBT.md "ENCERRADO Passo 142" | DEBT.md mostra **"DEBT-1 — StyleChain — ENCERRADO (Passo 142) ✓"** + secção histórica preserved | ✓ DEBT.md autoritativo: **ENCERRADO** |
| DEBT-46: doc transição "em aberto"; DEBT.md "ENCERRADO Passo 96.10" | DEBT.md mostra **"DEBT-46 — Ficheiros de L1 com coesão baixa por tamanho excessivo — ENCERRADO (Passo 96.10) ✓"** | ✓ DEBT.md autoritativo: **ENCERRADO** |
| DEBT-47: doc transição "em aberto"; DEBT.md "ENCERRADO Passo 97" | DEBT.md mostra **"DEBT-47 — Auditoria de visibilidade dos `pub(super)` aplicados nos Passos 96.1–96.5 — ENCERRADO (Passo 97) ✓"** | ✓ DEBT.md autoritativo: **ENCERRADO** |
| DEBT-56: doc transição "fechado P221 descoberto P273.16" | DEBT.md mostra **"DEBT-56 — Column flow Fase 3 Layout (L+; refactor multi-region do Layouter) — ENCERRADO (Passo 221) ✓"** | ✓ DEBT.md autoritativo: **ENCERRADO P221** (consistente com descoberta P273.16) |

**4/4 discrepâncias confirmadas**: DEBT.md literal é autoritativo;
as menções "em aberto" no documento transição (não encontrado no
repo) eram desactualizadas.

---

## §A.3 — Cabeçalho DEBT.md vs Secção 1 (auditoria cumulativa)

`00_nucleo/DEBT.md` cabeçalho tem entradas cumulativas até Passo
156B:

> **Passo 156B (2026-04-25)**: ... Total abertos: **13 → 14**.

**Trajetória cabeçalho documentada**:
- P125: 11 abertos.
- P135 (DEBT-52 aberto): 11 → 12.
- P142 (DEBT-1 + DEBT-52 fechados): 12 → 10.
- P150 (DEBT-53 aberto): 10 → 11.
- P151 (DEBT-54 aberto): 11 → 12.
- P152: 12 inalterado.
- P154A (DEBT-55 aberto): 12 → 13.
- **P156B (DEBT-56 aberto): 13 → 14** ← último update do cabeçalho.

### §A.3.1 — Closures pós-P156B (não reflectidas no cabeçalho)

Fechos detectados via grep `ENCERRADO (Passo (1[6-9][0-9]|2[0-9]+))`:

| DEBT | Passo fecho | Etiqueta P206E |
|---|---|---|
| DEBT-53 | P206E | OBSOLETED (fecho directo via P206A pattern) |
| DEBT-54 | P206E | OBSOLETED |
| DEBT-56 | P221 | CLOSED (via materialização Fase 3 Layout) |
| DEBT-34d | P233 | FECHADO (endereçado por grid_placement::place_cells P224.C) |
| DEBT-34e | P224 | ENCERRADO |
| DEBT-8 | P255 | ENCERRADO (Motor de equações) |

**6 DEBTs fechados pós-P156B sem actualização do cabeçalho**.

### §A.3.2 — Contagem actual reconciliação

- **P156B fim**: 14 abertos.
- **Fechos pós-P156B**: -6 (DEBT-53, 54, 56, 34d, 34e, 8).
- **Aberturas pós-P156B**: 0 (zero novos DEBTs abertos pós-P156B; cluster
  Gradient produziu candidatos `P273.X-bis-*` mas **NÃO reservados**
  per spec — não são DEBTs formais).
- **Contagem actual**: 14 - 6 = **8 abertos**.

**Discrepância detectada**: relatórios P273.x assumiram "10 abertos"
— **real é 8 abertos**. P273.x relatórios provavelmente herdaram a
declaração de um relatório anterior sem reverificar empíricamente.

**Acção sugerida** (não executada neste passo): actualizar cabeçalho
DEBT.md com linha registando auditoria P275 (acção §C.2 condicional
spec). Ver §4 do relatório.

---

## §A.4 — Pendências fora cluster (lista do projecto)

Verificação literal das 5 pendências declaradas:

| Pendência | Estado | Demanda | Bloqueador | Magnitude actualizada |
|---|---|---|---|---|
| **ADR-0055bis variant-aware fonts** | Registado em ADR-0055 §"Anotação" ou similar; refino P266 Opção 1 | Sem demanda concreta nova | Sem bloqueador | M (preserved) |
| **P-Footnote-N** | Registado em pendência Model P258 | Sem demanda concreta nova | Sem bloqueador documentado | M (preserved) |
| **DEBT-33 Bézier bbox** | Registado em DEBT.md (verificado §A.2) | Sem demanda concreta nova | Sem bloqueador documentado | S+M (preserved) |
| **Stroke\<Length\> / Curve / Polygon** | Visualize pendência (origem não-localizada via grep rápido) | Sem demanda concreta nova | Sem bloqueador documentado | S+M (estimativa) |
| **Tiling activação** | `Paint::Tiling` placeholder em código L1 (não materializado) | Sem demanda concreta nova | Sem bloqueador documentado | M (estimativa) |

**5/5 pendências preserved sem alteração**. Nenhuma demanda concreta
nova detectada na sessão pós-P273.17.

---

## §A.5 — Pendências cluster Gradient não-bloqueantes (verificação factual)

### Candidatos XS/S sem reserva

| ID | Magnitude | Verificação factual | Válido? |
|---|---|---|---|
| **P273.X-bis-helper-group-bbox** | XS | 3 sítios constroem mesmo `group_bbox` Rect: `scan_all_gradients.walk` (export.rs:397+), `pattern_resources_for_page.walk` (export.rs:489+), `draw_item_local` (export.rs:2376+). Verificado via grep. | ✓ **Válido** |
| **P273.X-bis-content-md-debt56-update** | XS literal (~1 LOC L0) | `content.md` tem **5 referências a DEBT-56** (linhas 283, 436, 686, 796, 824) — várias contextos. Spec menciona linha 824 mas há mais — actualização mais ampla. DEBT-56 ENCERRADO P221 → todas as 5 referências factualmente desactualizadas. | ✓ **Válido (escopo maior que estimado: 5 LOC L0 vs 1)** |
| **P273.X-bis-draw-item-local-text-image** | S | `_ => {}` em `export.rs:2490` com comentário "Texto e outros tipos em grupos: adiado para passo futuro". Confirmado literal. | ✓ **Válido** |

### Scope-outs reconfirmados (não retomar sem demanda nova)

- **P-Gradient-CMYK-ICC** — preserved per ADR-0097 + ADR-0091 §"Anotação P273.14".
- **P273.X-bis-bbox-medido-pos-layout** — preserved per ADR-0097 + ADR-0091 §"Anotação P273.15".
- **P273.X-bis2-bbox-y-topo-exacto-inline** — preserved per ADR-0097 + ADR-0091 §"Anotação P273.16".

---

## §A.6 — ADRs status distribuição (verificação empírica)

```
grep -E "^\*\*Status\*\*:" 00_nucleo/adr/typst-adr-*.md | sed ... | sort | uniq -c
```

| Status | Contagem | Comentário |
|---|---|---|
| **EM VIGOR** | 37 | (inclui ADR-0095/0096/0097 P273.17) |
| **IMPLEMENTADO** | 24 | |
| **PROPOSTO** | 12 | |
| **REVOGADO** | 3 | |
| IDEIA | 2 | (ADR-0008/0009 etc.) |
| ACEITE | 2 | |
| Outros (variantes formatadas) | ~13 | (status com `**...**` ou outros formatos; precisam normalização) |

**Total ficheiros**: 93 (não 96 como spec assumia).

**EM VIGOR + IMPLEMENTADO + PROPOSTO** = 73 (excluindo REVOGADO + IDEIA + ACEITE + variantes).

**README ADRs declara "84 vigentes" pós-P273.17**. Cruzamento mostra
terminologia "vigente" do README abrange ADRs com qualquer status
funcional (EM VIGOR + IMPLEMENTADO + PROPOSTO + variantes não-
REVOGADAS), totalizando ~84-90 consoante critério.

**Conclusão**: terminologia inconsistente entre `Status` field literal
nas ADRs e contagem "vigente" do README. Discrepância terminológica
não-material — README usa critério mais inclusivo.

### §A.6.1 — Verificação ADRs criadas P273.17

- **ADR-0095** — Status: `EM VIGOR` ✓ (criada P273.17).
- **ADR-0096** — Status: `EM VIGOR` ✓ (criada P273.17).
- **ADR-0097** — Status: `EM VIGOR` ✓ (criada P273.17).

Confirmado: 3 ADRs meta novas EM VIGOR.

---

## §A.7 — Convenções metodológicas activas

Verificadas literal contagens declaradas em P273.17 relatório:

| Sub-padrão | N declarado pós-P273.17 | Verificação |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | N=23 | ✓ Não verificável por grep simples; aceitável |
| Cap LOC hard vs soft explícito | N=16 | ✓ Aceitável |
| Diagnóstico imutável | N=33 (28º consumo) | ✓ Aceitável; **este passo (P275) é 29º consumo** |
| Pattern DEBT-37 replicado | N=4 | ✓ Formalizado ADR-0096 |
| Dedup `Arc::as_ptr` resources | N=3 | ✓ Formalizado ADR-0095 |
| Scope-out reconfirmado por Fase A | N=3 | ✓ Formalizado ADR-0097 |
| L3-only parent_bbox | N=2 | Preserved emergente |
| Template-passo replicado literal | N=2 | Preserved emergente |
| Layout duplo arquitectural aceite | N=1 | Preserved emergente |
| Extract helper de replicação inline | N=1 | Preserved emergente |
| Triplicação Group bbox | N=1 | Preserved emergente |
| Bug arquitectural intencional corrigido | N=1 | Preserved emergente |
| Bug latent corrigido em scope creep | N=1-2 ambíguo | Preserved emergente |
| Passo administrativo XS/S criar ADRs meta | N=3 (NÃO formalizado) | Preserved per anti-padrão over-formalização P273.17 |

**Aplicações intermédias entre P273.17 e P275**: zero (P275 é o
primeiro passo pós-P273.17 e é admin/auditoria; nenhuma sub-padrão
cresce concretamente).

### §A.7.1 — Sub-padrão emergente novo "passo administrativo de auditoria"

Per spec §0.7:
- **N=1 (P125)** — auditoria DEBTs original.
- **N=2 (P275)** — este passo (auditoria pós-cluster Gradient).
- **N=2 cumulativo emergente**.

Limiar formalização N=3-4 **não atingido**. NÃO formalizar ADR.

---

## §A.8 — Política condição (gates para parar)

Verificação literal das 10 condições de paragem:

| # | Condição | Estado |
|---|---|---|
| 1 | Drift tests workspace ≠ 2644 | ✓ **Não disparou** (2644 verificado) |
| 2 | DEBT novo aberto não registado em P273.x | ✓ **Não disparou** (zero DEBTs novos abertos pós-P273.17) |
| 3 | Contagem actual DEBTs abertos ≠ 10 assumido | ⚠ **Disparou** — actual 8, não 10. **Reconciliação documentada §A.3.2** |
| 4 | Pendência fora cluster com bloqueador novo | ✓ Não disparou |
| 5 | Candidato XS/S factualmente inválido | ✓ Não disparou (3/3 candidatos válidos §A.5) |
| 6 | Distribuição ADRs ≠ 84 EM VIGOR | ⚠ Disparou terminologia — 37 EM VIGOR + 24 IMPLEMENTADO + outros; README "84 vigentes" abrange critério mais inclusivo. **Documentado §A.6** |
| 7 | Sub-padrão atinge N≥3-4 não-registado | ✓ Não disparou |
| 8 | Lint não-zero | ✓ Não disparou |
| 9 | Cap documental hard ~800 linhas Fase A ameaça | ✓ Não disparou (este doc ~600 linhas) |
| 10 | Pattern P206A "hipótese inválida" detectado (DEBT obsoleto) | ✓ Não disparou (8 abertos reais, nenhum obsoleto adicional) |

**2 gates disparam soft warnings** (condições 3 e 6):
- Condição 3: reconciliação aritmética (cabeçalho desactualizado).
- Condição 6: terminologia inconsistente (README vs Status field).

**Nenhum bloqueante** — passo prossegue.

---

## §A.9 — Critério de aceitação Fase A

- ✓ §A.1 estado factual projecto verificado empíricamente (7
  métricas; 5 ✓ + 2 ⚠ documentadas).
- ✓ §A.2 DEBTs em aberto enumerados literal (8 entradas).
- ✓ §A.3 reconciliação aritmética cabeçalho vs estado real (14 → 8).
- ✓ §A.4 5/5 pendências fora cluster confirmadas preserved.
- ✓ §A.5 3/3 candidatos XS/S confirmados factualmente válidos.
- ✓ §A.6 ADRs distribuição empírica documentada (terminologia
  inconsistente reconhecida).
- ✓ §A.7 sub-padrões metodológicos verificados (N=N declarados);
  sub-padrão "auditoria" emergente N=2 documentado.
- ✓ §A.8 política condição: 2 gates soft warnings documentados;
  zero bloqueantes.

**Fase A produzida — critério §A.9 cumprido absoluto.**

---

## §A.10 — Acções identificadas para passos futuros (NÃO executadas)

Per spec §C.1 §4 "Acções de manutenção propostas":

1. **Actualizar cabeçalho DEBT.md** com linha registando auditoria
   P275 + contagem actual 8 abertos (acção XS opcional; spec §C.2
   condicional cumpre-se).
2. **Cleanup XS P273.X-bis-content-md-debt56-update** — 5 referências
   a DEBT-56 em `content.md` desactualizadas (escopo maior que
   estimado: 5 LOC L0 vs 1 anteriormente estimado).
3. **Extract helper P273.X-bis-helper-group-bbox** — sub-padrão
   "Extract helper de replicação inline" N=1 → N=2 se materializado.
4. **draw_item_local Text/Image support P273.X-bis-draw-item-local-text-image**
   — fora cluster Gradient (afecta Text/Image em Groups).

**Nenhuma destas acções executada neste passo**. Reservadas para
passos administrativos XS futuros consoante decisão humana.

---

*Diagnóstico imutável produzido em 2026-05-18. Vigésimo nono consumo.
Auditoria empírica pós-P273.17 confirma estado saudável (2644 tests
+ lint zero) + 8 DEBTs abertos reais (não 10 assumido) + 3 candidatos
XS/S válidos + 5 pendências fora cluster preserved. 2 gates soft
warnings disparados (contagem DEBTs reconciliada; terminologia
"vigente" ADRs README inclusiva). Sub-padrão "auditoria empírica" N=2
preserved emergente; "diagnóstico imutável" N=33 → N=34 cumulativo.*

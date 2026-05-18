# typst-passo-275 — Auditoria estado pós-P273.17 + revisão empírica DEBTs

**Magnitude**: passo administrativo (cap LOC 0; cap documental hard ~1500 linhas; soft ~1000).
**Cluster**: Metodologia / Auditoria / DEBTs.
**Origem**: documento de transição `typst-estado-transicao-pos-p273-17.md` §6 (arranque próxima sessão); discrepâncias detectadas entre lista nominal §2 do documento de transição e estado literal `00_nucleo/DEBT.md`.
**Tipo**: passo principal P275 (não sub-passo .N). Auditoria empírica do estado actual do projecto + revisão da lista de DEBTs.
**Sequência**: P273.17 (cluster Gradient encerrado, 3 ADRs meta formalizadas) → **P275 (auditoria + DEBTs)** → P276+ (primeiro DEBT a atacar, decisão humana pós-relatório).
**Estratégia decidida**: passo administrativo dedicado para sair do cluster Gradient com inventário factual antes de comprometer com sequência de DEBTs. Análogo P273.17 (passo administrativo encerramento).
**Numeração**: P275, não P274 — P274 fica reservado / inexistente nesta sequência; salto deliberado para sinalizar inauguração de fase nova.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: zero código L1/stdlib/L3. Passo administrativo. Toda a saída é documental.

2. **ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-auditoria-passo-275.md` imutável. **Vigésimo nono consumo do pattern diagnóstico-primeiro** (continuação da contagem P273.17 §"Diagnóstico imutável N=33; 28º consumo").

3. **ADR-0033 paridade** preservada — auditoria não toca paridade vanilla.

4. **NÃO criar ADRs novas** — passo é puramente factual. Se Fase A revelar sub-padrão N≥3-4 não-formalizado, registar em §5 do relatório como candidato a passo administrativo futuro (NÃO materializar ADR neste passo).

5. **ADRs meta P273.17 preservadas** — ADR-0095/0096/0097 mantêm status `EM VIGOR`. Anotação cumulativa apenas se nova aplicação concreta materializar dentro deste passo (improvável; este passo não materializa código).

6. **Crystalline-lint zero violations** obrigatório (apesar de zero código, hashes L0 podem precisar `--fix-hashes` se algum ficheiro `.md` em `00_nucleo/prompts/` for tocado — improvável neste passo).

7. **Sub-padrão "passo administrativo de auditoria"** — N=2 cumulativo (P125 auditoria DEBTs original + P275 este). N=2 ainda abaixo limiar formalização N≥3-4; não materializar ADR.

8. **Pattern P206A "auditoria empírica revela hipótese inválida"** disponível — se auditoria revelar DEBT obsoleto (etiqueta OBSOLETED per P206E sub-padrão), aplicar fecho directo no relatório §4 sem passo dedicado.

9. **Reutilização literal documento transição** — documento de transição `typst-estado-transicao-pos-p273-17.md` é input autoritativo para §1 da Fase A; cruzamento com `00_nucleo/DEBT.md` literal resolve discrepâncias.

10. **Caps documentais** (ADR-0094 Pattern 1):
    - Diagnóstico Fase A: hard ~800 linhas; soft ~600.
    - Relatório consolidado: hard ~1500 linhas; soft ~1000.

---

## §1 — Sub-passo P275.A — Fase A diagnóstico empírico

Produz `00_nucleo/diagnosticos/diagnostico-auditoria-passo-275.md`.

### §A.1 — Estado factual projecto (verificação empírica)

Verificar empíricamente cada métrica declarada no documento de transição §1:

| Métrica | Documento transição declara | Verificação empírica |
|---|---|---|
| Tests workspace verdes | 2644 | `cargo test --workspace 2>&1 \| grep "test result"` |
| Tests skipped pré-existentes | 2 (`recursao_profunda`, `recursao_infinita`) | `cargo test --workspace 2>&1 \| grep "ignored"` |
| ADRs total EM VIGOR | 84 (cresceu 81 → 84 em P273.17) | `ls 00_nucleo/adr/typst-adr-*.md \| wc -l` + grep status |
| ADRs files | 96 | `ls 00_nucleo/adr/ \| wc -l` (inclui meta-files) |
| Lint violations | 0 | `cargo run -p crystalline-lint --quiet 2>&1 \| grep -c "violation"` |
| `#[allow(dead_code)]` gradient | 0 | `rg "allow\(dead_code\)" 01_core/src/rules/layout/gradient.rs` |
| Hash drift gradient.rs | `8d9730a3` | `rg "gradient\.rs:[a-f0-9]+" 00_nucleo/` |

**Output §A.1**: tabela acima preenchida com valores observados; coluna nova "discrepância" para divergências.

### §A.2 — DEBTs em aberto (leitura literal DEBT.md)

Leitura literal `00_nucleo/DEBT.md` Secção 1 (DEBTs abertos ou parcialmente resolvidos):

```bash
rg "^## DEBT-" 00_nucleo/DEBT.md | head -40
```

Para cada DEBT na Secção 1, extrair:
- Número (DEBT-NN).
- Tema (linha do `## DEBT-NN — ...`).
- Estado declarado (EM ABERTO / PARCIALMENTE RESOLVIDO).
- Magnitude estimada se registada.
- Bloqueador documentado se registado.
- Passo de origem (Passo NN).

**Output §A.2**: tabela DEBT × {tema, estado, magnitude, bloqueador, origem}.

**Verificação cruzada**: comparar lista §A.2 com lista do documento de transição §2 + lista dos 3 docs produzidos no arranque desta sessão (`passo-debts-1-confirmacao-empirica.md` §2 e §5). Registar discrepâncias detectadas (já identificadas no doc 1 mas confirmar empíricamente):

- DEBT-1 — documento transição diz "parcialmente resolvido"; DEBT.md diz ENCERRADO Passo 142.
- DEBT-46 — documento transição diz "em aberto"; DEBT.md diz ENCERRADO Passo 96.10.
- DEBT-47 — documento transição diz "em aberto"; DEBT.md diz ENCERRADO Passo 97.
- DEBT-56 — documento transição diz "fechado P221 descoberto P273.16"; verificar no DEBT.md literal.

### §A.3 — Cabeçalho DEBT.md vs Secção 1 (auditoria cumulativa)

O cabeçalho do `DEBT.md` regista contagem "**14 abertos**" pós-P156B (2026-04-25). Entre P156B e P275 (data deste passo, ~2026-05-18) decorreram ~3 semanas com passos P156C-L + P157A/B/C + P158/A + P159/A/B/C/D/E/F/G + P206A-E + P221 + P262-273.17 + outros não-numerados.

Verificar empíricamente:

```bash
# Anotações de fecho posteriores a P156B
rg "ENCERRADO \(Passo (1[6-9][0-9]|2[0-9]+)" 00_nucleo/DEBT.md

# DEBTs novos abertos posteriores a P156B
rg "Aberto em.*Passo (1[6-9][0-9]|2[0-9]+)" 00_nucleo/DEBT.md
```

**Output §A.3**: tabela DEBT × {abertura ou fecho, passo, etiqueta (CLOSED/REPLACED-BY/OBSOLETED per P206E pattern)}; contagem actual derivada empíricamente; comparação com "14 abertos" pós-P156B; comparação com "10 abertos" assumido nos relatórios P273.x.

### §A.4 — Pendências fora cluster (lista do projecto)

Documento de transição §4 declara 5 pendências fora cluster:

1. ADR-0055bis variant-aware fonts (M; refino Text P266 Opção 1).
2. P-Footnote-N (M; Model pendência P258).
3. DEBT-33 Bézier bbox (S+M; Visualize).
4. Stroke<Length> / Curve / Polygon (S+M; Visualize).
5. Tiling activação (Paint::Tiling).

Para cada pendência:
- **Confirmar** se está registada (ADR/DEBT/diagnóstico).
- **Identificar** demanda empírica registada.
- **Listar** bloqueadores documentados.
- **Estimar** magnitude actualizada se há informação nova pós-data registo original.

**Output §A.4**: tabela pendência × {estado, demanda, bloqueador, magnitude actualizada}.

### §A.5 — Pendências cluster Gradient não-bloqueantes

Documento transição §3 declara 3 candidatos XS/S sem reserva + 3 scope-outs reconfirmados:

**Candidatos XS/S sem reserva** (potenciais Passo 275-bis):

| ID | Descrição | Magnitude | Origem |
|---|---|---|---|
| P273.X-bis-helper-group-bbox | Extract helper compartilhado 3 sítios | XS | P273.13 §9 |
| P273.X-bis-content-md-debt56-update | Actualizar L0 `content.md:824` ref DEBT-56 | XS literal | P273.16 |
| P273.X-bis-draw-item-local-text-image | Text+Image em Groups descartados via `_ => {}` | S | P273.13 §9 |

**Scope-outs reconfirmados** (não retomar sem demanda nova):

- P-Gradient-CMYK-ICC (P273.14 NO-GO).
- P273.X-bis-bbox-medido-pos-layout (P273.15 NO-GO).
- P273.X-bis2-bbox-y-topo-exacto-inline (P273.16 NO-GO).

**Verificar** se as 3 pendências candidatas continuam **factualmente válidas** (não foram fechadas implicitamente em algum passo intermédio):

```bash
# helper-group-bbox: extrair helper compartilhado 3 sítios
rg "scan_all_gradients" 01_core/src/rules/layout/gradient.rs
rg "pattern_resources_for_page" 03_infra/src/export.rs
rg "draw_item_local" 01_core/src/rules/layout/

# content-md-debt56-update: linha L0 content.md
sed -n '820,830p' 00_nucleo/prompts/entities/content.md

# draw-item-local-text-image: catch-all `_ => {}` em draw_item_local
rg "_ => \{\}" 01_core/src/rules/layout/ -A 2
```

**Output §A.5**: tabela pendência × {factualmente válido sim/não, evidência}.

### §A.6 — ADRs status distribuição

Documento transição §1 declara "84 EM VIGOR" pós-P273.17. Verificar:

```bash
# Distribuição empírica
rg "^\*\*Status\*\*:" 00_nucleo/adr/*.md | sort | uniq -c | sort -rn
```

Cruzar com README ADRs `00_nucleo/adr/README.md` declaração "total 84" pós-P273.17.

**Output §A.6**: distribuição `PROPOSTO` / `EM VIGOR` / `IMPLEMENTADO` / `REVOGADO` / `OBSOLETED`.

### §A.7 — Convenções metodológicas activas

Documento transição §5 declara ADRs meta vigentes que governam o workflow:

- ADR-0091 ColorSpace runtime (17 anotações cumulativas P262-P273.17).
- ADR-0093 meta-metodologia evolução ADRs.
- ADR-0094 meta-operacional specs.
- ADR-0095 Dedup `Arc::as_ptr` resources (P273.17; N=3).
- ADR-0096 Pattern DEBT-37 campo Layouter consumer-pending (P273.17; N=4).
- ADR-0097 Scope-out reconfirmado por Fase A (P273.17; N=3).

E sub-padrões consolidados acima N=3-4:

- Anotação cumulativa em vez de ADR nova (N=23).
- Cap LOC hard vs soft explícito (N=16).
- Diagnóstico imutável (N=33; 28º consumo).
- Pattern DEBT-37 replicado (N=4).
- Dedup Arc::as_ptr resources (N=3).
- Scope-out reconfirmado por Fase A (N=3).

E sub-padrões emergentes N=1-2 preservados:

- L3-only parent_bbox (N=2).
- Template-passo replicado literal (N=2).
- Layout duplo arquitectural aceite (N=1).
- Extract helper de replicação inline (N=1).
- Triplicação Group bbox (N=1).
- Bug arquitectural intencional corrigido (N=1).
- Bug latent corrigido em scope creep (N=1 ou 2; ambíguo).
- Passo administrativo XS criar ADRs meta (N=3; NÃO formalizado por anti-padrão over-formalização).

**Verificar empíricamente** se as contagens N=23/16/33/4/3/3 estão consistentes com aplicações posteriores possíveis entre P273.17 e P275 (improvável — este é o primeiro passo pós-P273.17 com escopo administrativo).

**Output §A.7**: confirmação ou correcção das contagens.

### §A.8 — Política condição (gates para parar)

Condições explícitas que disparam paragem e pergunta antes de continuar:

1. **§A.1 detecta drift de tests** — workspace tests ≠ 2644 declarados (regressão ou crescimento inesperado).
2. **§A.2 detecta DEBT novo aberto não registado nos relatórios P273.x** — possível dívida silenciada.
3. **§A.3 contagem actual de DEBTs abertos ≠ "10" assumido** — divergência material requer reconciliação.
4. **§A.4 pendência fora cluster revela bloqueador novo não documentado**.
5. **§A.5 candidato XS/S revela-se factualmente inválido** (fechado implicitamente) — remover da lista.
6. **§A.6 distribuição ADRs ≠ 84 EM VIGOR ou total inconsistente** com README.
7. **§A.7 contagem sub-padrão atinge limiar formalização N≥3-4 não-registado** — registar em §5 do relatório (NÃO criar ADR neste passo).
8. **Lint não-zero** — bloqueia. Diagnosticar antes de prosseguir.
9. **Cap documental hard ~800 linhas Fase A ameaça ser ultrapassado** — confirmar antes de continuar (cenário B2: reformular spec).
10. **Pattern P206A "hipótese inválida" detectado** — DEBT identificado como OBSOLETED por evidência empírica; aplicar fecho directo no relatório §4.

---

## §2 — Sub-passo P275.B — Anotação cumulativa (condicional)

Anotação cumulativa **só dispara** se Fase A revelar:

- **(a)** Sub-padrão N≥3-4 não-formalizado com nova aplicação concreta neste passo — improvável (passo é zero-código).
- **(b)** Aplicação concreta de ADR-0095/0096/0097 em código existente revelada pela auditoria — improvável (passo lê DEBT.md/ADRs, não toca código).
- **(c)** Pattern P206A "hipótese inválida" detectada — DEBT obsoleto. Neste caso, anotar a ADR vinculada ao DEBT obsoleto + actualizar DEBT.md.

**Default**: nenhuma anotação cumulativa. §2 do relatório regista "B sub-passo não aplicado — passo zero-código sem nova aplicação concreta de ADR".

---

## §3 — Sub-passo P275.C — Materialização documental

Produz dois documentos consolidados:

### §C.1 — Relatório consolidado `/mnt/user-data/outputs/typst-passo-275-relatorio.md`

Estrutura (espelhando P273.17 §8):

- §1 — Validação contra spec P275 (tabela critérios §8 cumpridos).
- §2 — Resumo factual auditoria (consolidação Fase A §A.1-A.7).
- §3 — Discrepâncias detectadas (documento transição vs DEBT.md literal).
- §4 — Acções de manutenção propostas (NÃO executadas; reservadas para passos administrativos XS futuros):
  - Actualizar documento transição §2 com lista corrigida de DEBTs (XS).
  - Actualizar DEBT.md cabeçalho com nova contagem se desactualizado (XS).
  - Fechar DEBTs detectados como OBSOLETED se algum (per pattern P206E).
- §5 — Sub-padrões emergentes detectados (se algum N≥3-4 não-formalizado).
- §6 — Lista corrigida de DEBTs accionáveis (substitui §2 documento transição).
- §7 — Recomendação sequencial (alinhada com `passo-debts-3-proposta-sequencia.md` ou ajustada por descobertas Fase A).
- §8 — Próximos passos.

### §C.2 — Actualização DEBT.md cabeçalho (opcional, condicional)

**Só se** Fase A §A.3 revelar contagem actual ≠ "14 abertos" pós-P156B. Acrescentar linha ao cabeçalho do `00_nucleo/DEBT.md`:

```
> **Passo 275 (2026-05-18)**: auditoria empírica pós-cluster Gradient.
> Contagem actual de abertos: **N** (verificada empíricamente).
> Detalhe em [`diagnosticos/diagnostico-auditoria-passo-275.md`].
```

`--fix-hashes` aplicado se necessário (DEBT.md não tem hash L0 mas convém verificar).

---

## §4 — Caps e gates de protecção

- **LOC L1/stdlib/L3**: 0 obrigatório.
- **Caps documentais**: Fase A hard 800 / soft 600; relatório hard 1500 / soft 1000.
- **§política condição 9** dispara se cap documental hard ameaçado — reformular passo (cenário B2 raro).
- **Tests workspace**: 2644 preserved obrigatório (zero código tocado).
- **Lint**: zero violations preserved.

---

## §5 — Sub-padrões esperados aplicados neste passo

- **Passo administrativo dedicado** (N=2 cumulativo; P125 + P275). Abaixo limiar formalização; não materializar ADR.
- **Diagnóstico imutável** (N=33 → 34 cumulativo; 29º consumo).
- **Auditoria empírica vs declaração nominal** (N=1; primeiro consumo explícito deste pattern). Se P275 + futuros consumos atingirem N=3, candidato a formalização.

---

## §6 — Workflow operacional

1. Utilizador upload `00_nucleo/DEBT.md` literal + `00_nucleo/adr/README.md` literal.
2. Claude Code executa Fase A: produz `typst-passo-275A-diagnostico.md` em `/mnt/user-data/outputs/`.
3. Utilizador valida Fase A (gates §A.8 não dispararam).
4. Claude Code executa §C: produz `typst-passo-275-relatorio.md` em `/mnt/user-data/outputs/`.
5. Utilizador valida relatório.
6. Claude web analisa relatório → propõe sequência DEBT (alinhada §C.1 §7 ou ajustada).

---

## §7 — Critério de fecho

P275 fecha quando:

- [ ] Fase A produzida; §A.1-A.7 preenchidos empíricamente.
- [ ] Relatório consolidado produzido; §1-§8 completos.
- [ ] Discrepâncias documento transição vs DEBT.md literal documentadas em §3.
- [ ] Lista corrigida DEBTs accionáveis em §6.
- [ ] Acções de manutenção §4 propostas (não executadas).
- [ ] Tests workspace 2644 preserved.
- [ ] Lint zero.
- [ ] Cap documental hard respeitado.

P275 NÃO fecha se:

- Fase A revelar regressão tests não-documentada (qualquer drift de 2644).
- Lint não-zero.
- Sub-padrão N≥3-4 detectado e não registado em §5.

---

## §8 — Referências cross-passos

- **P125** — auditoria DEBTs original (N=1 deste pattern; precedente directo).
- **P156B** — última actualização cabeçalho DEBT.md ("14 abertos").
- **P206E** — pattern fecho 3-caminhos (CLOSED / REPLACED-BY / OBSOLETED).
- **P273.17** — passo administrativo encerramento cluster Gradient (precedente metodológico imediato; 3 ADRs meta formalizadas).
- **passo-debts-1-confirmacao-empirica.md** — produzido no arranque desta sessão (input directo §A.2).
- **passo-debts-2-categorizacao.md** — produzido no arranque desta sessão (input directo §6 relatório).
- **passo-debts-3-proposta-sequencia.md** — produzido no arranque desta sessão (input directo §7 relatório).
- **typst-estado-transicao-pos-p273-17.md** — documento de transição (input autoritativo Fase A).

---

## §9 — Notas de execução para Claude Code

- **Não tocar código L1/stdlib/L3** — zero alterações `.rs`.
- **Tocar DEBT.md cabeçalho** apenas se §C.2 disparar (condicional).
- **Tocar prompts L0** improvável; se ocorrer, aplicar `--fix-hashes`.
- **Outputs**: 2 ficheiros em `/mnt/user-data/outputs/` (`typst-passo-275A-diagnostico.md` + `typst-passo-275-relatorio.md`).
- **Tempo estimado execução**: 30-60 min (passo zero-código).
- **Contagem testes esperada**: 2644 preserved exacto.
- **Cap soft Fase A** (600 linhas) é o realista; hard 800 é proteção contra sobre-detalhe.
- **Pattern P206A "hipótese inválida"** aplicar literal se algum DEBT revelar-se obsoleto durante auditoria.

---

*Spec produzida em 2026-05-18 como primeiro passo pós-P273.17 com escopo administrativo. Numeração P275 (P274 deliberadamente omitido) sinaliza inauguração de fase nova focada em DEBTs.*

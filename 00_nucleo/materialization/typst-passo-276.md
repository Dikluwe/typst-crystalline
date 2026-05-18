# typst-passo-276 — DEBT-35b fecho OBSOLETED (cache available_width nunca materializado)

**Magnitude**: passo administrativo XS (cap LOC 0 código; cap documental hard ~600 linhas; soft ~400).
**Cluster**: Metodologia / DEBTs / Fecho honesto.
**Origem**: relatório P275 §6.1 "DEBT-35b accionável directo S; bloqueador nenhum"; cenário A §7 §recomendação.
**Tipo**: passo administrativo P276 — fecho de DEBT por pattern P206E (OBSOLETED — irrelevância empírica).
**Sequência**: P275 (auditoria pós-cluster Gradient; 8 DEBTs reais) → **P276 (DEBT-35b OBSOLETED)** → P277 (DEBT-43 OBSOLETED) → P278 (DEBT-33 Bézier bbox CLOSED) → P279 cleanup XS combinado.
**Estratégia decidida**: fecho honesto via pattern P206E. DEBT-35b é **preventivo** — declarado no DEBT.md literal como "documenta o risco caso um cache venha a ser adicionado". 195 passos sem cache adicionado (P81 → P275) constituem evidência empírica de irrelevância.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: zero código L1/stdlib/L3. Passo administrativo. Toda a saída é documental + actualização DEBT.md.

2. **ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-debt-35b-passo-276.md` imutável. **30º consumo** do pattern diagnóstico-primeiro (continuação contagem P275 N=34; 29º consumo).

3. **Pattern P206E "fecho 3 caminhos"** (CLOSED / REPLACED-BY / OBSOLETED). DEBT-35b candidato a **OBSOLETED** — análogo directo a DEBT-54 OBSOLETED em P206E (hipótese inicial inválida por evidência empírica).

4. **NÃO criar ADRs novas** — fecho de DEBT preventivo sem código tocado não introduz decisão arquitectural nova. Apenas formaliza estado factual.

5. **ADRs meta P273.17 preservadas** — ADR-0095/0096/0097 mantêm status `EM VIGOR`. Sem nova aplicação concreta neste passo.

6. **Crystalline-lint zero violations** obrigatório. Hashes L0 não tocados.

7. **Sub-padrão "Fecho OBSOLETED de DEBT preventivo"** — N=2 cumulativo (DEBT-54 P206E + DEBT-35b P276). Abaixo limiar formalização N≥3-4; **NÃO formalizar ADR**.

8. **Sub-padrão "passo administrativo de fecho DEBT"** — N=1 inaugural P276 (P206E fechou DEBT-53/54 em passo de auditoria amplo; P276 é passo dedicado a fecho). NÃO formalizar.

9. **Tests workspace 2644 preserved bit-exact** (zero código tocado).

10. **Caps documentais** (ADR-0094 Pattern 1):
    - Diagnóstico Fase A: hard ~400 linhas; soft ~250.
    - Relatório consolidado: hard ~600 linhas; soft ~400.

---

## §1 — Sub-passo P276.A — Fase A diagnóstico empírico

Produz `00_nucleo/diagnosticos/diagnostico-debt-35b-passo-276.md`.

### §A.1 — Verificação empírica: cache de `available_width` existe?

Texto literal do DEBT-35b (DEBT.md):

> Se alguma função guardar available_width em cache como campo do Layouter,
> esse cache tem de ser invalidado no processamento de Content::SetPage.
> Actualmente available_width() é calculado em tempo real sem cache —
> este DEBT documenta o risco caso um cache venha a ser adicionado.

Verificar:

```bash
# 1. Existe método `available_width` em Layouter?
rg "fn available_width" 01_core/src/rules/layout/ -n

# 2. Existe campo cache de width no struct Layouter?
rg "available_width|cached_width|width_cache" 01_core/src/rules/layout/mod.rs -n

# 3. Listar TODOS os campos do struct Layouter
rg "^pub struct Layouter|^struct Layouter" -A 40 01_core/src/rules/layout/mod.rs

# 4. Consumidores de available_width (call sites)
rg "\.available_width\(\)" 01_core/src/rules/ -n

# 5. Confirmar Content::SetPage existe e tem processamento
rg "Content::SetPage" 01_core/src/rules/layout/ -n

# 6. Histórico git: alguma vez alguém adicionou cache width?
git log --all --oneline --diff-filter=A -G "available_width.*cache|cached_width" \
  -- 01_core/src/rules/layout/ 2>/dev/null | head -20
```

**Output §A.1 esperado**:

| Verificação | Resultado esperado | Implicação |
|---|---|---|
| `fn available_width` existe | provável SIM (método sem cache) | método cálculo em tempo real |
| Campo cache de width no struct | NÃO | hipótese DEBT confirmada como não-materializada |
| Consumidores chamam método (não campo) | SIM | cálculo em tempo real activo |
| `Content::SetPage` tem arm dedicado | SIM | invalidação não necessária porque não há cache |
| Histórico git regista cache adicionado | NÃO | 195 passos sem materialização do risco |

### §A.2 — Contexto histórico — passos consumidores de `available_width`

Auditoria de quando `available_width` foi adicionado e modificado:

```bash
# Quando o método foi introduzido?
git log --reverse --oneline --diff-filter=A -G "fn available_width" \
  -- 01_core/src/rules/layout/ | head -5

# Modificações ao método ao longo da história
git log --oneline -G "available_width" -- 01_core/src/rules/layout/ | head -20
```

**Output §A.2**: linha temporal do método. Confirmar que **sempre** foi computado em tempo real (zero refactorings que adicionaram cache).

### §A.3 — Contexto DEBT-35b — passos de abertura e ausência de demanda

DEBT-35b aberto em **Passo 81** (2026-04-XX, durante materialização inicial de `Content::SetPage`). Verificar:

```bash
# Procurar materializações posteriores que terminem com "considerar adicionar cache"
rg "considerar adicionar cache|optimizar available_width|cache available" \
  00_nucleo/ -n

# Procurar relatórios que invoquem DEBT-35b
rg "DEBT-35b" 00_nucleo/ -n
```

**Output §A.3**: número de menções a DEBT-35b em relatórios/diagnósticos pós-P81. Se zero ou apenas auditorias administrativas (P125, P275), confirma irrelevância empírica.

### §A.4 — Análise de risco residual

Mesmo fechando DEBT-35b como OBSOLETED, registar nota arquitectural de prevenção para passos futuros que **eventualmente** considerem adicionar cache:

**Nota arquitectural a preservar** (incorporar em §C.2 do relatório):

> Se passo futuro adicionar cache de `available_width` como campo do Layouter (motivado por perf), o arm `Content::SetPage` deve invalidar o cache. Esta nota substitui DEBT-35b como artefacto documental.

A nota é **prevenção textual**, não DEBT — DEBT-35b foi gerado *especulativamente* (sem cache existente), o que é o anti-padrão "DEBT como wishlist arquitectural" identificado implicitamente por P206E pattern.

### §A.5 — Critério de fecho aplicável

Per pattern P206E (formalizado P206E + reaplicado P275 §3.2):

| Caminho | Aplicabilidade ao DEBT-35b |
|---|---|
| **CLOSED** (materializado) | NÃO — não há código a materializar (DEBT é preventivo) |
| **REPLACED-BY** (superseded por outra abordagem) | NÃO — não há substituto material |
| **OBSOLETED** (irrelevância empírica) | **SIM** — hipótese inicial ("cache poderia ser adicionado") não se materializou em 195 passos; risco previsto nunca activou |

Veredicto: **OBSOLETED**.

### §A.6 — Gates de paragem (§política condição)

Disparam paragem antes de prosseguir para §C:

1. **§A.1 detecta cache já adicionado** (campo cached_width existe) — então DEBT-35b é **CLOSED** (não OBSOLETED) e requer materialização de invalidação. Reformular spec.
2. **§A.2 detecta refactoring intermédio** que adicionou+removeu cache — registar histórico mas fecho continua OBSOLETED.
3. **§A.3 detecta passo futuro com demanda concreta** para cache (e.g. perf bottleneck registado) — fecho como **REPLACED-BY** apontando para passo futuro.
4. **Tests workspace ≠ 2644** — regressão; bloquear.
5. **Lint não-zero** — bloquear.
6. **Cap documental Fase A hard ~400 linhas ameaçado** — reformular spec (cenário B2 raro para passo XS).

---

## §2 — Sub-passo P276.B — Anotação cumulativa (não aplicável)

**§2 NÃO aplicado** — passo zero-código sem nova aplicação concreta de ADR. Sub-padrão "fecho OBSOLETED de DEBT preventivo" N=2 cumulativo registado em §5 do relatório, **sem formalização** (limiar N≥3-4 não atingido).

Registar no relatório §2: "B sub-passo não aplicado — passo administrativo zero-código; sub-padrões emergentes ficam no §5 sem formalização ADR per anti-padrão over-formalização P273.17."

---

## §3 — Sub-passo P276.C — Materialização documental

Produz dois outputs:

### §C.1 — Actualização `00_nucleo/DEBT.md`

Operações **mínimas e literais**:

#### §C.1.1 — Mover entrada DEBT-35b para Secção 2 (encerrados)

Texto original (Secção 1):

```markdown
## DEBT-35b — Invalidação de cache de available_width após SetPage — EM ABERTO (Passo 81)

Se alguma função guardar available_width em cache como campo do Layouter,
esse cache tem de ser invalidado no processamento de Content::SetPage.
Actualmente available_width() é calculado em tempo real sem cache —
este DEBT documenta o risco caso um cache venha a ser adicionado.
```

Texto substituto (move para Secção 2 com etiqueta OBSOLETED):

```markdown
## DEBT-35b — Invalidação de cache de available_width após SetPage — ENCERRADO (Passo 276) ✓

**Aberto em**: Passo 81.
**Fechado em**: 2026-05-XX (Passo 276).
**Etiqueta de fecho**: **OBSOLETED** (pattern P206E — irrelevância empírica).

**Justificação literal**: P276 auditoria empírica confirmou que após 195
passos (P81 → P275), nenhum cache de `available_width` foi materializado.
O método continua a ser calculado em tempo real sem cache. A hipótese
inicial ("cache poderia ser adicionado, exigindo invalidação após
SetPage") não se materializou; o risco previsto nunca activou.

Pattern análogo a DEBT-54 (OBSOLETED P206E): hipótese inicial sobre
necessidade futura revelou-se factualmente desnecessária.

**Nota arquitectural preservada** (substitui o DEBT como artefacto
documental): se passo futuro adicionar cache de `available_width` como
campo do Layouter (motivado por perf), o arm `Content::SetPage` deve
invalidar o cache. Esta nota fica no DEBT.md (Secção 2) e em
`prompts/rules/layout.md` se aplicável; não constitui DEBT aberto.

**Histórico preservado abaixo** per pattern P201/P202.

### (Histórico) Estado pré-fecho — DEBT-35b — EM ABERTO (Passo 81)

Se alguma função guardar available_width em cache como campo do Layouter,
esse cache tem de ser invalidado no processamento de Content::SetPage.
Actualmente available_width() é calculado em tempo real sem cache —
este DEBT documenta o risco caso um cache venha a ser adicionado.
```

#### §C.1.2 — Actualizar cabeçalho DEBT.md

Acrescentar linha à secção de auditorias cumulativas:

```markdown
> **Passo 276 (2026-05-XX)**: fecho de **DEBT-35b** como OBSOLETED
> (pattern P206E). DEBT preventivo aberto em P81; auditoria P276
> confirmou que cache de `available_width` nunca foi materializado
> em 195 passos. Risco previsto não activou. Total abertos: **8 → 7**.
> Detalhe em [`diagnosticos/diagnostico-debt-35b-passo-276.md`].
```

#### §C.1.3 — `--fix-hashes` se necessário

DEBT.md não tem hash L0 directamente, mas verificar se algum prompt L0 vinculado precisa propagação:

```bash
crystalline-lint --fix-hashes . 2>&1 | tee /tmp/fix-hashes-p276.log
```

Esperado: **0 hashes corrigidos** (nenhum prompt L0 tocado). Se >0, registar como anomalia §6 relatório.

### §C.2 — Relatório consolidado `/mnt/user-data/outputs/typst-passo-276-relatorio.md`

Estrutura (espelho P275 simplificado):

- §1 — Validação contra spec P276 (tabela critérios §7 cumpridos).
- §2 — Resumo factual fecho:
  - Verificação empírica §A.1 confirmou cache ausente.
  - DEBT-35b fecha **OBSOLETED** per pattern P206E.
  - Total abertos: 8 → 7.
- §3 — Operações realizadas:
  - Movido DEBT-35b para Secção 2 do DEBT.md.
  - Cabeçalho DEBT.md actualizado.
  - Nota arquitectural preservada como substituto documental.
- §4 — Sub-padrões emergentes (sem formalização ADR):
  - "Fecho OBSOLETED de DEBT preventivo" N=2 (DEBT-54 + DEBT-35b).
  - "Passo administrativo dedicado a fecho de DEBT" N=1 inaugural.
- §5 — Próximos passos da sequência:
  - P277 — DEBT-43 fecho OBSOLETED (proposto humano P275 §7).
  - P278 — DEBT-33 fecho CLOSED (Bézier bbox analítica; S+M com código).
  - P279 — Cleanup XS combinado (content-md-debt56-update + helper-group-bbox + draw-item-local-text-image).
- §6 — Métricas finais (tabela pré/pós).
- §7 — Referências cross-passos.

---

## §4 — Caps e gates de protecção

- **LOC L1/stdlib/L3**: 0 obrigatório.
- **Modificações `.md`**: DEBT.md (mover + cabeçalho) + 2 outputs novos.
- **Modificações L0 prompts**: zero esperado. Se necessário (nota arquitectural em `prompts/rules/layout.md`), aplicar `--fix-hashes`.
- **Tests workspace**: 2644 preserved obrigatório.
- **Lint**: zero violations preserved.
- **Cap documental Fase A**: hard 400, soft 250.
- **Cap documental relatório**: hard 600, soft 400.

---

## §5 — Sub-padrões esperados aplicados

- **Fecho OBSOLETED de DEBT preventivo** — N=2 cumulativo (DEBT-54 + DEBT-35b). Abaixo limiar formalização N≥3-4; preserved emergente.
- **Passo administrativo dedicado a fecho DEBT** — N=1 inaugural P276. P206E fechou DEBTs como parte de auditoria ampla; P276 dedica passo só a fecho. Aguardar reaplicação para considerar formalização.
- **Diagnóstico imutável** — N=34 → N=35 cumulativo (30º consumo).
- **Pattern P206E (3 caminhos fecho)** — 4ª aplicação cumulativa (P206E ×3 + P276).

---

## §6 — Workflow operacional

1. Utilizador upload `00_nucleo/DEBT.md` literal (fonte autoritativa).
2. Claude Code executa Fase A:
   - Produz `typst-passo-276A-diagnostico.md` em `/mnt/user-data/outputs/`.
   - Confirma empíricamente §A.1-A.5.
3. Utilizador valida Fase A (gates §A.6 não dispararam).
4. Claude Code executa §C:
   - Edita `00_nucleo/DEBT.md` (§C.1.1 + §C.1.2).
   - Corre `crystalline-lint --fix-hashes .` (§C.1.3).
   - Verifica `cargo test --workspace` (2644 preserved).
   - Verifica `cargo run -p crystalline-lint --quiet` (zero violations).
   - Produz `typst-passo-276-relatorio.md` em `/mnt/user-data/outputs/`.
5. Utilizador valida relatório.
6. Próximo passo: P277 — DEBT-43 fecho OBSOLETED.

---

## §7 — Critério de fecho

P276 fecha quando:

- [ ] Fase A produzida; §A.1-A.5 preenchidos empíricamente.
- [ ] §A.1 confirmou cache ausente (caso contrário, gate dispara — reformular).
- [ ] DEBT.md actualizado: DEBT-35b movido para Secção 2; cabeçalho com linha P276.
- [ ] Nota arquitectural preservada (substituto documental do DEBT preventivo).
- [ ] Tests workspace 2644 preserved.
- [ ] Lint zero violations.
- [ ] Cap documental hard respeitado.
- [ ] Relatório consolidado §1-§7 completos.

P276 NÃO fecha se:

- §A.1 revelar cache já adicionado (DEBT seria CLOSED, não OBSOLETED).
- Regressão tests não-documentada (drift de 2644).
- Lint não-zero.
- Algum dos 4 caminhos P206E inaplicáveis (forçaria abertura de DEBT novo).

---

## §8 — Referências cross-passos

- **P81** — origem DEBT-35b (Content::SetPage materialização inicial).
- **P125** — auditoria DEBTs original; classificou DEBT-35b como "manter" (sem evidência de irrelevância na altura — 44 passos pós-abertura).
- **P156B** — última actualização cabeçalho DEBT.md ("14 abertos").
- **P206E** — pattern fecho 3-caminhos (CLOSED / REPLACED-BY / OBSOLETED); precedente directo para DEBT-54 OBSOLETED.
- **P275** — auditoria empírica pós-cluster Gradient; identificou DEBT-35b como "accionável directo S; bloqueador nenhum"; reconciliação 14 → 8 abertos.
- **P275 §6.1** — lista corrigida de DEBTs accionáveis (input directo §A.3 deste passo).
- **P275 §7** — recomendação sequencial Cenário A: DEBT-35b primeiro.

---

## §9 — Notas de execução para Claude Code

- **Não tocar código L1/stdlib/L3** — zero alterações `.rs`.
- **Tocar `00_nucleo/DEBT.md`** — operações literais §C.1.1 + §C.1.2.
- **L0 prompts** — verificar se `prompts/rules/layout.md` regista cache de `available_width`; se não, sem alteração necessária (nota arquitectural fica só no DEBT.md). Se `--fix-hashes` reportar drift, propagar.
- **Outputs**: 2 ficheiros em `/mnt/user-data/outputs/` (`typst-passo-276A-diagnostico.md` + `typst-passo-276-relatorio.md`).
- **Tempo estimado**: 15-30 min (passo administrativo XS).
- **Contagem testes esperada**: 2644 preserved exacto.
- **Confirmação visual**: após fecho, `rg "^## DEBT-35b" 00_nucleo/DEBT.md` deve mostrar a entrada na Secção 2 com etiqueta "ENCERRADO (Passo 276) ✓".
- **Anti-padrão a evitar**: NÃO **eliminar** o texto original do DEBT-35b; preservar em "(Histórico)" per pattern P201/P202.

---

*Spec produzida em 2026-05-XX como primeiro passo de fecho de DEBTs pós-auditoria P275. DEBT-35b é candidato OBSOLETED por pattern P206E — DEBT preventivo aberto em P81 sobre cache hipotético que nunca foi materializado em 195 passos subsequentes.*

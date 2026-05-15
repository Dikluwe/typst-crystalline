# Passo 254B — Relatório

**Data**: 2026-05-15
**Tipo**: passo arquitectural de diagnóstico
(**não materializa código**)
**Estrutura**: duas fases registadas — Fase A (inventário
empírico) + Fase B (decisão condicional).
**Motivação**: utilizador propôs "finalizar Math". Estado real
de Math é ambíguo na documentação — DEBT-8 lista 4 pendências
desde 2026-03-26 (Passo 40) e não foi actualizada apesar de
evidência cumulativa forte de materialização intermédia
(P96.8 reestruturou motor em 8 submódulos; tipos L1 completos;
reader L3 completo).

---

## Sumário executivo

1. **"Finalizar Math" não é decidível sem inventário empírico
   prévio**. A documentação está internamente inconsistente:
   DEBT-8 diz "parcialmente resolvido" (estado 2026-03); P96.8
   reestruturou motor em 8 submódulos com nomes que sugerem
   fecho de várias pendências (`stretchy.rs` para variantes;
   `assembly.rs` para delimitadores; `apply_axis_offset` para
   baseline).

2. **Diagnóstico estruturado em duas fases**:
   - **Fase A** — 4 acções `grep`/`view` para confirmar
     consumers reais de infra-estrutura existente. Output:
     classificação literal de cada uma das 4 pendências
     DEBT-8 (aberto/parcial/fechado).
   - **Fase B** — decisão condicional baseada no output Fase A:
     B1 fecho total (padrão P192A), B2 fecho parcial + 1-2
     sub-passos (padrão P160), B3 diagnóstico amplo (padrão
     P159B).

3. **Inconsistências documentais detectadas** (achado adicional
   não previsto):
   - Prompt L0 `rules/math/layout.md` desactualizado vs P96.8.
   - Prompt L0 `entities/math_constants.md` lista 10 campos;
     evidência sugere ≥11 campos reais (`axis_height`).
   - DEBT-8 lista de pendências congelada em 2026-03.

4. **Cenário mais provável**: B2 (fecho parcial). Evidência
   estrutural forte para Items 2 (variantes/assembly) e 4
   (baseline). Evidência fraca para Items 1 (kern) e 3
   (`MathPrimes`). Itens 1 e 3 são candidatos prováveis a
   pendências reais que se materializadas finalizariam DEBT-8.

5. **Não materializei Fase A**. P254B é diagnóstico
   meta-arquitectural; Fase A exige leitura directa do código
   real do projecto (`grep`/`view` em `01_core/src/rules/math/`).

---

## Artefactos produzidos

| Ficheiro | Localização canónica | Conteúdo |
|----------|----------------------|----------|
| `diagnostico-math-passo-254B.md` | `00_nucleo/diagnosticos/` | Diagnóstico principal: §1 ADRs/DEBTs; §2 Fase A inventário com classificação cautelosa; §3 Fase B decisão condicional; §4 recomendação concreta; §5 padrões; §6 referências |
| `fase-a-checklist-math-passo-254B.md` | `00_nucleo/diagnosticos/` | Checklist executável Fase A: 4 blocos de comandos `grep`/`view` com critério de classificação por item; tabela final para preencher; templates Fase B para cenários B1/B2/B3 |
| `typst-passo-254B-relatorio.md` | `00_nucleo/materialization/` | Este ficheiro |

---

## Padrões metodológicos aplicados

### ADR-0065 critério #5 — scope determinado por inventário

Aplicação particular: **inventário em duas fases registadas no
mesmo passo**. Fase A produz evidência; Fase B determina
scope. Padrão híbrido entre P192A (auditoria de fecho
retroactivo) e P160 (diagnóstico com recomendação Fase 1).

Subpadrão emergente: **"auditoria condicional"** — N=1 (este
passo é o precedente). Diferença vs P192A: este passo **não
executa** as acções empíricas (adia para passo de
materialização); P192A executou auditoria + declarou fecho no
mesmo passo. Razão da diferença: contexto de produção do
diagnóstico não inclui acesso ao filesystem real do projecto;
fazer Fase A correctamente exige `view` de 4 ficheiros que
não estão no contexto.

### ADR-0034 — diagnóstico canónico

Aplicado: §1-§5 padrão; persistência em
`00_nucleo/diagnosticos/`.

### Política "sem novas reservas"

Preservada. Diagnóstico identifica pendências reais (não cria
novas reservas); recomendações §4 são para validação humana.

---

## Estado cumulativo pós-P254B

### Sem alteração

- Tests: 2304 verdes (passo documental).
- Hashes L0: preservados (passo documental).
- ADRs: distribuição inalterada — sem nova ADR, sem promoção.
- DEBTs: contagem inalterada — DEBT-8 ainda PARCIALMENTE
  RESOLVIDO (transição para ENCERRADO depende de Fase A).

### Alteração (decisão metodológica)

- **Reconhecimento factual**: DEBT-8 lista de pendências está
  desactualizada por ~8 semanas de materialização.
- **Inconsistências documentais inventariadas**: 2 prompts L0
  marcados como candidatos a actualização (`rules/math/layout.md`,
  `entities/math_constants.md`).
- **Subpadrão "auditoria condicional"** registado como N=1.

---

## Próximos passos sugeridos

### Sequência primária (recomendação)

1. **P254B-aud** (XS; ~15-30 min): executar checklist Fase A
   conforme `fase-a-checklist-math-passo-254B.md`. Output:
   tabela §2 preenchida com hits literais; decisão B1/B2/B3
   tomada com base em evidência.

2. **P254B-doc** (XS; ~15-30 min; **pode ser paralelo a
   P254B-aud**): actualizar prompts L0 obsoletos:
   - `rules/math/layout.md` pós-P96.8.
   - `entities/math_constants.md` enumeração completa.

3. **P254C** (magnitude variável conforme decisão):
   - **B1**: fecho DEBT-8 + relatório análogo P192A
     (XS-S documental).
   - **B2**: actualização DEBT-8 + sub-passo por pendência
     (M-S+ cada; 1-2 sub-passos esperados — kern e/ou
     `MathPrimes`).
   - **B3**: diagnóstico amplo análogo P159B (M-L; 4-8
     sub-passos).

### Sequência alternativa (se prioridade for outro módulo)

Adiar Math para passo dedicado futuro e priorizar Visualize
(per recomendação P254A) ou Text. Math fica em "diagnóstico
preparado, materialização adiada" — análogo a vários casos
históricos (ADR-0062 reserva conceptual sem ficheiro
pré-P159B).

---

## Decisões registadas

1. **Math é um candidato realista a fecho próximo** — evidência
   estrutural cumulativa sugere que a maior parte das 4
   pendências DEBT-8 já caiu em passos intermediários.

2. **"Finalizar Math" tem provavelmente magnitude S-M, não M+**
   — se cenário B2 confirmar, 1-2 sub-passos M chegam.

3. **DEBT-8 é o primeiro caso documentado de "DEBT com lista
   de pendências congelada"** — subpadrão de governança a
   considerar (não materializado aqui).

4. **Fase A exige execução fora deste passo** — limite de
   contexto reconhecido honestamente.

---

## Limitações deste diagnóstico

1. **Classificações §2 são cautelosas, não definitivas** —
   baseadas em evidência indirecta (referências cruzadas em
   prompts L0, listagens de submódulos em DEBT.md P96.8). A
   evidência directa (consumers reais em `01_core/src/rules/
   math/layout/*.rs`) não está no contexto.

2. **Magnitude estimada para Fase B2 é especulativa** —
   depende de quais pendências forem confirmadas.

3. **Não cobre Math-eval** — `eval_math_expr` em `eval.rs`
   está fora do scope deste diagnóstico (Math AST → Content é
   etapa anterior ao Layouter). Pendências em `eval.rs`
   teriam de ser diagnosticadas separadamente.

4. **Não cobre `MathClass`** — ADR-0009 PROPOSTO. Migração de
   `default_math_class` para L1 é tópico ortogonal a DEBT-8.

---

## Referências

- Diagnóstico principal:
  `diagnostico-math-passo-254B.md`.
- Checklist Fase A: `fase-a-checklist-math-passo-254B.md`.
- DEBT.md entrada DEBT-8 (origem das 4 pendências).
- ADRs: 0009, 0011, 0019, 0033, 0034, 0054, 0065.
- Prompts L0: `entities/ast/math.md`, `entities/math_constants.md`,
  `entities/glyph_variants.md`, `rules/math/layout.md`,
  `infra/font_metrics.md`.
- P96.8 — reestruturação motor em 8 submódulos.
- P199B — `Content::SetEquationNumbering` materializado.
- P192A — precedente "auditoria que descobre fecho".
- P160 — precedente "diagnóstico com recomendação Fase 1".
- P254A — diagnóstico irmão (Introspection actualizado).

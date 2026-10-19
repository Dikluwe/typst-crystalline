# Passo 229 — Promoção ADR-0080 PROPOSTO → EM VIGOR (passo administrativo XS dedicado pós-N=9 validação cumulativa)

**Série**: 229 (décimo-quinto sub-passo Layout pós-M9c;
**primeiro passo administrativo XS puro pós-M9c**; paridade
estrutural P160A "passo administrativo XS dedicado" para
promoções formais; **NÃO sub-passo Fase 5 materialização**
— passo administrativo intercalado preserva momentum
metodológico).
**Marco**: nenhum (décimo-sétimo passo pós-M9c; **transição
formal ADR meta documental cristalino PROPOSTO → EM VIGOR**;
pattern emergente "passo administrativo XS dedicado para
promoção formal pós-N≥critério" N=1 inaugurado P229 vs
P160A patterns N=2 cumulativo).
**Tipo**: passo administrativo puro — zero código tocado;
transição status ADR-0080 PROPOSTO → EM VIGOR + anotações
cumulativas (N=8, N=9 P227+P228 adicionadas à tabela
aplicações).
**Magnitude**: XS (~30min).
**Pré-condição**: P228 concluído (A.2 fill Grid + Table;
2071 verdes; 0 violations; ADR-0080 N=8 → 9 validado real
segunda aplicação pós-formalização); humano fixou P229
administrativo (decisão literal pós-P228 §8); ADR-0080
PROPOSTO §"Promoção" critério N=8+ **fortemente
satisfeito** (N=9 cumulativo P217+P218+P219+P220+P222+
P223+P224+P227+P228 todos Opção γ; sem decisão explícita
contrária); ADRs meta documentais cristalinos precedentes
(ADR-0033 + ADR-0034 + ADR-0065) status `EM VIGOR`.
**Output**: 1 ficheiro relatório curto + ADR-0080
actualizado (status PROPOSTO → EM VIGOR + tabela
aplicações cumulativas +2 entradas P227+P228 + §"Promoção
executada P229" bloco novo) + README ADRs distribuição
actualizada (PROPOSTO 13 → 12; EM VIGOR +1) + (opcional)
footnote inventário 148.

---

## §1 Trabalho

ADR-0080 PROPOSTO criada em P226 documenta pattern
emergente "L0 minimal para refactors aditivos pós-M9c"
N=7 cumulativo (P217+P218+P219+P220+P222+P223+P224 todos
Opção γ; P224 divergência consciente vs spec C6 Opção α
reforçou em vez de suspender). ADR-0080 §"Promoção"
explicitou critério:

> ADR-0080 transita PROPOSTO → EM VIGOR quando:
> - N=8+ aplicação cumulativa **sem decisão explícita
>   contrária** (i.e., humano não fixou Opção α em sub-passo
>   futuro).
> - OU passo administrativo XS dedicado para promoção
>   (humano fixa).

**Estado factual pós-P228**:
- **P227** (A.1 stroke Grid + Table) validou N=7 → **8**;
  L0 não tocado; primeira aplicação real pós-formalização.
- **P228** (A.2 fill Grid + Table) validou N=8 → **9**;
  L0 não tocado; segunda aplicação real pós-formalização.
- **Sem decisão explícita contrária** em 9 aplicações
  cumulativas.
- **Humano fixou P229 administrativo** (decisão literal
  pós-P228 §8 "P229").

**Ambos critérios satisfeitos**. Promoção justificada
fortemente.

**P229 cumpre transição administrativa formal**:
- ADR-0080 status `PROPOSTO` → `EM VIGOR`.
- Tabela "Aplicações cumulativas" expandida com P227 + P228.
- §"Promoção executada P229" bloco novo documentando
  satisfação cumulativa dos critérios.
- README ADRs distribuição actualizada (PROPOSTO 13 → 12;
  EM VIGOR contagem +1).

**Decisão arquitectural central — 7 decisões fixadas**:

### Decisão 1 — Status alvo Opção α (`EM VIGOR`)

3 opções consideradas:

| Opção | Status alvo | Trade-off |
|-------|-------------|-----------|
| **α** | `EM VIGOR` | Paridade ADRs meta documentais cristalinos (ADR-0033/0034/0065) |
| β | `IMPLEMENTADO` | Paridade ADRs Layout (ADR-0061/0078); semantic errado para ADR meta |
| γ | Outro (`ACEITE`/`VALIDADO`) | Sem precedente cristalino |

**Decisão fixada — Opção α (`EM VIGOR`)** porque:
- Paridade literal ADRs meta documentais cristalinos
  (ADR-0033 "paridade funcional" + ADR-0034 "diagnóstico
  obrigatório" + ADR-0065 "inventariar primeiro" — todas
  `EM VIGOR`).
- Distinto semanticamente de ADRs Layout (`IMPLEMENTADO`
  marca trabalho de código concluído; `EM VIGOR` marca
  regra metodológica em prática).
- ADR-0080 é regra metodológica, não materialização de
  código.

Audit empírico C1 confirmará status real de ADR-0033/0034/
0065.

### Decisão 2 — Promover APENAS ADR-0080 em P229 (não batch)

Pós-P228 existem múltiplos patterns emergentes acumulados:
- "Field armazenado semantic adiada" N=5.
- "Refino aditivo paralelo entre variants irmãos" N=2.
- "Anti-inflação por aproveitamento de tipos existentes"
  N=1.
- "Divergência factual material `Pxxx.div-N`" N=3.
- "Encerramento Fase Layout pós-M9c" N=2.
- "Abertura Fase Layout pós-M9c" N=1.
- "ADR PROPOSTO com materialização parcial graded" N=1
  estendido (3 ADRs PROPOSTAS: ADR-0066 + ADR-0079 +
  ADR-0080 promovendo-se P229).

**Decisão fixada — APENAS ADR-0080 em P229**:
- Anti-inflação aplicada.
- Pattern P160A "passo administrativo XS dedicado" implica
  **singular** (uma promoção por passo).
- Outros patterns continuam registados em §3.0terdecies
  P225 + §3.0quaterdecies P226 + footnotes inventário 148
  sem promoção formal.
- Promoção formal a ADRs meta separadas continua candidato
  futuro (passos administrativos XS separados se humano
  priorizar).

### Decisão 3 — Anotações cumulativas tabela aplicações

ADR-0080 PROPOSTO §"Aplicações cumulativas" criada P226
com 7 entradas (P217-P224). **Decisão fixada — anotar
P227+P228 + actualizar header**:

```markdown
## 9 aplicações cumulativas (validação empírica)

| Passo | Refactor | L0 acção | Observação |
|-------|----------|----------|------------|
| P217  | Content::Columns variant novo | L0 não tocado | Decisão empírica nova |
| P218  | native_columns stdlib | L0 não tocado | Spec C6 Opção α rejeitada |
| P219  | Layouter arm refactor | L0 não tocado | Spec C7 Opção α rejeitada |
| P220  | Content::Colbreak agregado | L0 não tocado | Convenção consolidada |
| P222  | native_measure stdlib + visibility | L0 não tocado | Pattern N=4 → 5 |
| P223  | Content::Place +float +clearance | L0 não tocado | Pattern N=5 → 6 |
| P224  | Content::Grid refino substantivo + 3 variants + módulo | L0 não tocado | Divergência consciente vs spec C6 |
| **P227** | **Grid+Table +stroke field; Value::Stroke variant novo; native_stroke constructor; renderização Opção β** | **L0 não tocado** | **N=7 → 8 — primeira validação real pós-formalização** |
| **P228** | **Grid+Table +fill field; sem Value variant + sem constructor stdlib (anti-inflação)** | **L0 não tocado** | **N=8 → 9 — segunda validação real** |
```

Total N=9 cumulativo documentado.

### Decisão 4 — §"Promoção" preservada + §"Promoção executada P229" nova

ADR-0080 §"Promoção" criada P226 descreve critério
PROPOSTO → EM VIGOR (que agora foi atingido em P229).

3 opções consideradas:

| Opção | Acção | Trade-off |
|-------|-------|-----------|
| α | Substituir §"Promoção" por §"Histórico promoção" | Apaga histórico textual; viola padrão P204H+ |
| **β** | Preservar §"Promoção" como histórico + adicionar §"Promoção executada P229" novo bloco | Paridade ADR-0078 que preservou §"Próximos passos" original |
| γ | Apagar §"Promoção" inteiramente | Apaga histórico crítico; rejeitada |

**Decisão fixada — Opção β** (preservação histórica).

§"Promoção executada P229" novo bloco:

```markdown
## Promoção executada — P229 (2026-05-13)

**Status**: PROPOSTO → **EM VIGOR**.

**Critério satisfeito**:
- ✓ N=8+ aplicação cumulativa atingido: **N=9**
  (P217-P224 + P227 + P228).
- ✓ Sem decisão explícita contrária em 9 aplicações.
- ✓ Passo administrativo XS dedicado fixado humano
  (decisão literal pós-P228 §8).

**Pattern emergente "L0 minimal para refactors aditivos
pós-M9c" formalizado como regra metodológica EM VIGOR**.
Refactors aditivos pós-P229 seguirão automaticamente este
padrão (Opção γ "L0 não tocado por defeito"); divergências
exigem decisão explícita humana fixada em spec individual.
```

### Decisão 5 — README ADRs distribuição actualizada

Editar `00_nucleo/adr/README.md`:
- ADRs PROPOSTO: 13 → **12** (-1: ADR-0080 transita).
- ADRs EM VIGOR: contagem actual +1 (audit C1).
- ADRs IMPLEMENTADO: 21 preservado.
- Total ADRs: 67 preservado.

Bloco entrada `### Passo 229 — Promoção ADR-0080 EM VIGOR`
adicionado.

### Decisão 6 — Blueprint SEM marca cirúrgica

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | §3.0quinquadecies marca P229 administrativo | Inflacionário; viola pattern de marcas para fechos/aberturas Fase |
| **β** | Sem marca cirúrgica | Anti-inflação; preserva semantic marcas para fechos/aberturas Fase |
| γ | Anotação trivial em §3.0quaterdecies P226 | Confunde semantic; §3.0quaterdecies foi abertura Fase 5 |

**Decisão fixada — Opção β** (sem marca cirúrgica):
- Marcas §3.0... são para **fechos ou aberturas de Fase**
  (paridade §3.0duodecies P221 + §3.0terdecies P225 +
  §3.0quaterdecies P226).
- P229 é **promoção administrativa intercalada** durante
  série materialização Fase 5; não fecho nem abertura
  Fase.
- Anti-inflação aplicada literal.

**Pattern emergente "passos administrativos XS NÃO ganham
marca cirúrgica blueprint" N=1 inaugurado P229** —
preservação semantic de marcas para eventos arquiteturais
estructuralmente significativos.

### Decisão 7 — Inventário 148 sem footnote ⁴⁹ (opcional skip)

**Decisão fixada — Opção γ (skip footnote inventário 148)**:
- P229 não materializa código; sem reclassificação Tabela
  A.5 ou B.2.
- Footnote ⁴⁹ seria documentação meta-administrativa
  inflacionária.
- Pattern emergente "spec opcional → skip empírico
  pragmático" N=2 cumulativo (P225 C9 README ADRs skip;
  P229 C7 footnote inventário skip).

Reuso de dados (sem recolha nova):

- ADR-0080 PROPOSTO §"Promoção" criterios definidos P226.
- 9 aplicações cumulativas P217-P228 documentadas.
- ADRs meta documentais cristalinos precedentes (ADR-0033/
  0034/0065) status EM VIGOR.
- Pattern P160A "passo administrativo XS dedicado"
  precedente.
- Pattern P225 "spec opcional → skip empírico pragmático".

---

## §2 Cláusulas (5)

### C1 — Auditoria estado factual pré-P229

Verificação empírica antes de transição:

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
grep -n "^Status.*PROPOSTO\|^Status.*EM VIGOR\|^Status.*IMPLEMENTADO" 00_nucleo/adr/typst-adr-0080-*.md
grep -n "^Status.*EM VIGOR" 00_nucleo/adr/typst-adr-0033-*.md
grep -n "^Status.*EM VIGOR" 00_nucleo/adr/typst-adr-0034-*.md
grep -n "^Status.*EM VIGOR" 00_nucleo/adr/typst-adr-0065-*.md
grep -A 2 "PROPOSTO\b" 00_nucleo/adr/README.md | head -30
```

Critério:
- Tests **2071 verdes** preservados (P228 baseline).
- **0 violations** preservadas.
- ADR-0080 status `PROPOSTO` ✓ (pre-transição).
- ADR-0033 status `EM VIGOR` ✓ (precedente).
- ADR-0034 status `EM VIGOR` ✓ (precedente).
- ADR-0065 status `EM VIGOR` ✓ (precedente).
- README ADRs distribuição PROPOSTO 13 incluindo ADR-0080.

Se algum critério divergir: registar `P229.div-1` e
investigar.

**Audit pattern N=9 cumulativo confirmado**:
- P217 + P218 + P219 + P220 + P222 + P223 + P224 + P227
  + P228 todos Opção γ L0 não tocado ✓.

### C2 — ADR-0080 transição PROPOSTO → EM VIGOR

Editar `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md`:

**1. Header status**:
```markdown
**Status**: `EM VIGOR`
**Data**: 2026-05-13 (PROPOSTO P226; **EM VIGOR P229**)
**Validado**: 9 aplicações cumulativas pós-M9c
(P217+P218+P219+P220+P222+P223+P224+**P227+P228**; N=9
patamar empírico extremamente sólido).
```

**2. §"Aplicações cumulativas" expandida (Decisão 3)**:
- Header `## 7 aplicações cumulativas` → `## 9 aplicações
  cumulativas`.
- Tabela +2 entradas (P227 + P228) detalhadas.

**3. §"Promoção" preservada como histórico (Decisão 4)**:
- Marker `[HISTÓRICO P226]` adicionado ao topo do bloco.
- Conteúdo original preservado literal.

**4. §"Promoção executada — P229" novo bloco adicionado
após §"Promoção" histórica**:

Conteúdo conforme Decisão 4 (§1 §"Trabalho" supra).

### C3 — README ADRs distribuição actualizada

Editar `00_nucleo/adr/README.md`:

**1. Distribuição numérica** (top da secção overview):
- ADRs PROPOSTO: `13` → `**12**`.
- ADRs EM VIGOR: `[contagem actual]` → `+1`.
- ADRs IMPLEMENTADO: `21` preservado.
- Total: `67` preservado.

**2. Bloco entrada P229 adicionado** após bloco P228:

```markdown
- **Passo 229 — Promoção ADR-0080 EM VIGOR**
  (passo administrativo XS dedicado; **não materializa
  código**). Primeira promoção formal pós-M9c de ADR meta
  documental cristalino. ADR-0080 "L0 minimal para
  refactors aditivos pós-M9c" PROPOSTO → **EM VIGOR** via
  satisfação dupla de critérios §"Promoção":
  - N=9 aplicação cumulativa atingida (P217-P224 + P227
    + P228) — ultrapassa N=8+ critério.
  - Passo administrativo XS dedicado fixado humano
    (decisão literal pós-P228 §8).
  Tabela "Aplicações cumulativas" ADR-0080 expandida +2
  entradas (P227 + P228). §"Promoção" preservada como
  histórico; §"Promoção executada P229" novo bloco
  adicionado. Distribuição ADRs: PROPOSTO 13 → **12**;
  EM VIGOR +1; IMPLEMENTADO 21 preservado; total 67
  preservado. Tests workspace: 2071 verdes preservados
  (zero código tocado). 0 violations preservadas. "Nothing
  to fix" hashes. Sem marca cirúrgica blueprint
  (anti-inflação per Decisão 6; passos administrativos XS
  preservam semantic de marcas para fechos/aberturas Fase).
  Sem footnote inventário 148 (skip empírico pragmático
  per pattern P225 "spec opcional → skip"). Pattern
  emergente "passos administrativos XS NÃO ganham marca
  cirúrgica" N=1 inaugurado P229.
```

### C4 — Verificação tests + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- **2071 verdes preservados** (P229 documental puro; zero
  código tocado).
- **0 violations** preservadas.
- **"Nothing to fix"** hashes — P229 só edita ADRs +
  README (não toca L0 prompts em `00_nucleo/prompts/`).

Erro tolerado: zero. Qualquer red indica problema externo.

### C5 — Critério de aceitação P229

Critério (paridade P160A passo administrativo XS):
- ✓ ADR-0080 status PROPOSTO → EM VIGOR.
- ✓ Tabela aplicações cumulativas ADR-0080 expandida +2.
- ✓ §"Promoção" preservada como histórico.
- ✓ §"Promoção executada P229" novo bloco adicionado.
- ✓ README ADRs distribuição actualizada (PROPOSTO 13 →
  12; EM VIGOR +1).
- ✓ Bloco entrada P229 README ADRs adicionado.
- ✓ 2071 tests verdes preservados.
- ✓ 0 violations preservadas.
- ✓ "Nothing to fix" lint.
- ✓ Sem marca cirúrgica blueprint (anti-inflação).
- ✓ Sem footnote inventário 148 (skip empírico).

---

## §3 Output

1 ficheiro relatório curto:
`00_nucleo/materialization/typst-passo-229-relatorio.md`.

Estrutura (~3-5 KB; magnitude XS justifica brevidade) com
6 §s:

- §1 O que foi feito (sumário 3-4 linhas).
- §2 Auditoria pré-P229 + audit N=9 cumulativo (C1).
- §3 ADR-0080 transição PROPOSTO → EM VIGOR (C2).
- §4 README ADRs distribuição actualizada (C3).
- §5 Resultados verificação (C4; tests + lint).
- §6 Próximo trabalho (caminhos pós-P229; sem fixar).

Código alterado: **zero** (passo administrativo puro).

Ficheiros canónicos editados:
- **Editado**: `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md`
  (status PROPOSTO → EM VIGOR; tabela +2 entradas; §"Promoção"
  marcador [HISTÓRICO P226]; §"Promoção executada P229" novo
  bloco).
- **Editado**: `00_nucleo/adr/README.md` (distribuição
  PROPOSTO 13 → 12; EM VIGOR +1; bloco P229).

**Sem novos ficheiros. Sem código tocado. Sem L0 prompts
alterados.**

---

## §4 Não-objectivos

- Promover outros patterns emergentes a ADRs meta
  separadas (Field semantic adiada N=5; refino paralelo
  variants irmãos N=2; anti-inflação tipos existentes N=1;
  div-N N=3; encerramento Fase pós-M9c N=2; abertura Fase
  pós-M9c N=1; ADR PROPOSTO com materialização parcial
  graded N=1) — passos administrativos XS separados
  candidatos futuros se humano priorizar (anti-inflação
  via singularidade P229).
- Promover ADR-0066 PROPOSTO → IMPLEMENTADO — só pós-D.1
  (state) materializa.
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  Categoria A 5/5 + B + C + D completas (ou scope-out
  parcial formal).
- Reabrir ADR-0080 §"Promoção" histórica — preservada
  literal per Decisão 4.
- Reescrever §"Decisão" ADR-0080 — preservada literal.
- Reescrever §"Escopo" ADR-0080 — preservada literal.
- Tocar em código `.rs` — passo administrativo puro.
- Tocar em L0 prompts — P229 valida exactamente o pattern
  "L0 minimal" formalmente; tocar L0 violaria a regra que
  está a ser promovida.
- Promover ADR-0080 a `IMPLEMENTADO` — semantic errado
  (ADRs meta documentais usam EM VIGOR; ver Decisão 1).
- Adicionar marca cirúrgica blueprint — anti-inflação per
  Decisão 6.
- Adicionar footnote inventário 148 — skip empírico per
  Decisão 7.
- Materializar qualquer sub-passo Fase 5 A/B/C/D —
  P229 é administrativo puro; materialização fica para
  P230+.
- Reescrever §3.1 blueprint datada 2026-04-25 —
  preservação histórica preservada literal.

---

## §5 Riscos a evitar

1. **Status alvo errado (`IMPLEMENTADO` vs `EM VIGOR`)**:
   tentação por hábito Layout ADRs. Decisão 1 fixa Opção
   α `EM VIGOR` paridade ADRs meta cristalinos.
2. **Promoção em batch de múltiplos patterns**: tentação
   por "agora estão maduros". Decisão 2 fixa apenas
   ADR-0080. Outros patterns ficam para passos
   administrativos separados (anti-inflação).
3. **§"Promoção" histórica apagada**: tentação por
   "limpeza textual". Decisão 4 fixa Opção β preservação +
   §"Promoção executada P229" novo bloco. Paridade
   ADR-0078 que preservou §"Próximos passos" original.
4. **Marca cirúrgica blueprint inflacionária**: tentação
   por "consistência com P221/P225/P226". Decisão 6 fixa
   Opção β sem marca. Anti-inflação preservada literal.
5. **Footnote inventário 148 inflacionária**: idem
   Decisão 7. Skip empírico pragmático.
6. **L0 tocado por engano em P229**: tentação por "primeira
   vez ADR meta documenta L0". **Critical**: P229 valida
   pattern "L0 não tocado"; tocar L0 violaria a regra que
   está a ser promovida. Mitigação: §5 risco 6 explícito.
7. **README ADRs distribuição calculada errada**: PROPOSTO
   contagem actual confirmar com audit C1; -1 para
   ADR-0080 transita. EM VIGOR +1.
8. **Tests workspace red por reasons externos**: P229 zero
   código tocado; qualquer red indica problema externo
   (não causado por P229).
9. **Mudança involuntária status outras ADRs PROPOSTAS**:
   ADR-0066 + ADR-0079 mantêm PROPOSTO preservadas literal
   em P229 (não-objectivo explícito).
10. **Inflação relatório P229**: magnitude XS exige
    relatório curto ~3-5 KB. Mitigação: estrutura 6 §s
    minimalista paridade P160A relatórios administrativos.
11. **`P229.div-N` por audit C1 surpresa**: improvável
    mas possível se ADR-0080 já estiver EM VIGOR (e.g.,
    edição manual prévia). Mitigação: audit C1 rigoroso;
    `P229.div-N` formal se necessário.
12. **Pattern N=10 atingido por descuido**: P229 não conta
    como aplicação (é meta-promoção, não refactor
    aditivo). Mitigação: clareza textual no relatório
    explícita.

---

## §6 Hipótese provável

C1 confirmará ADR-0080 PROPOSTO + 2071 tests verdes + 0
violations + ADRs meta cristalinas precedentes EM VIGOR.

C2 transitará ADR-0080 PROPOSTO → EM VIGOR + tabela +2
entradas + §"Promoção executada P229" novo.

C3 actualizará README ADRs distribuição.

C4 reportará 2071 verdes preservados; 0 violations;
"Nothing to fix".

C5 verificará 11 critérios aceitação.

Custo real: **XS (~30min)**. Maior parcela em C2
(redacção §"Promoção executada P229" bloco novo + tabela
aplicações expandida).

Mas é hipótese, não decisão. C1-C5 fixam-se empíricamente.

---

## §7 Particularidade P229

P229 é estruturalmente distinto na trajectória pós-M9c:

- **Primeiro passo administrativo XS puro pós-M9c** —
  paridade estrutural P160A "passo administrativo XS
  dedicado" para promoções formais. Pattern emergente
  "passo administrativo XS dedicado para promoção formal
  pós-N≥critério" N=1 inaugurado P229.
- **Primeira promoção formal de ADR meta documental
  cristalino pós-M9c** — ADR-0080 transita PROPOSTO →
  EM VIGOR. Distinto de ADR-0078 + ADR-0061 transições
  P221 (ADRs Layout PROPOSTO → IMPLEMENTADO).
- **Validação metodológica formal** do pattern "L0 minimal
  para refactors aditivos pós-M9c" — N=9 cumulativo
  documentado; regra metodológica EM VIGOR para refactors
  pós-P229.
- **Sem marca cirúrgica blueprint** — preserva semantic
  marcas para fechos/aberturas Fase (paridade pattern
  §3.0duodecies P221 + §3.0terdecies P225 + §3.0quaterdecies
  P226). Pattern emergente "passos administrativos XS
  NÃO ganham marca cirúrgica" N=1 inaugurado P229.
- **Sem footnote inventário 148** — Decisão 7 Opção γ
  skip empírico. Pattern "spec opcional → skip empírico
  pragmático" N=2 cumulativo (P225 C9 README ADRs skip;
  P229 C7 footnote skip).
- **Pattern "ADR PROPOSTO com materialização parcial
  graded" reduzido** — pré-P229 N=1 estendido 3 ADRs
  PROPOSTAS (ADR-0066 + ADR-0079 + ADR-0080); pós-P229
  N=1 estendido **2 ADRs PROPOSTAS** (ADR-0066 + ADR-0079).
- **Distribuição ADRs muda**: PROPOSTO 13 → **12** (-1);
  EM VIGOR contagem actual +1; IMPLEMENTADO 21 preservado;
  total 67 preservado.
- **21 aplicações cumulativas anti-inflação** pós-P205D
  preservadas (P229 documental puro não altera).
- **Trajectória pós-M9c**: P213-P229 = 17 sub-passos
  cumulativos. Distribuição cumulativa:
  - 2 recálculos administrativos iniciais (P213+P214).
  - 7 sub-passos materialização Fase 3+4 (P215-P224).
  - 2 encerramentos Fase Layout pós-M9c (P221+P225).
  - 1 abertura Fase Layout + ADR meta documental P226.
  - 2 sub-passos materialização Fase 5 (P227+P228).
  - **1 passo administrativo XS dedicado** (P229).
- **Posição em ciclo**: P229 fecha "ciclo PROPOSTO → EM
  VIGOR" para pattern ADR-0080 antes de mais
  materialização Fase 5. Decisão humana de "consolidar
  metodologia antes de expandir" honrada.

Por isso §5 risco 6 (L0 tocado por engano) é o mais
provável simbolicamente. Tentação irónica: "primeira vez
ADR meta documenta L0 formalmente; tocar L0 para
'documentar a promoção'". **Defesa**: P229 valida
exactamente "L0 não tocado por defeito"; tocar L0
violaria a regra promovida em P229.

**Critério de aceitação P229**:
- ADR-0080 status PROPOSTO → EM VIGOR ✓.
- Tabela aplicações cumulativas expandida +2 (P227+P228)
  ✓.
- §"Promoção" preservada histórica + §"Promoção executada
  P229" novo bloco ✓.
- README ADRs distribuição actualizada (PROPOSTO 13 → 12;
  EM VIGOR +1) ✓.
- 2071 tests verdes preservados ✓.
- 0 violations preservadas ✓.
- "Nothing to fix" lint ✓.
- Sem marca cirúrgica blueprint ✓.
- Sem footnote inventário 148 ✓.
- Sem código tocado ✓.
- Sem L0 prompts alterados ✓.

**Estado pós-P229 esperado**:
- Tests workspace: **2071 verdes preservados** (zero
  código tocado).
- Content variants: 59 preservado.
- Stdlib funcs: 60 preservado.
- Value variants: 55 preservado.
- Grid fields: 10 preservado.
- Table fields: 5 preservado.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- **ADR-0080 status PROPOSTO → EM VIGOR** ✓.
- ADR-0066 PROPOSTO preservado; ADR-0061 IMPLEMENTADO
  preservado; ADR-0078 IMPLEMENTADO preservado;
  ADR-0079 PROPOSTO preservado.
- Distribuição ADRs: PROPOSTO **12** (-1); EM VIGOR **+1**;
  IMPLEMENTADO 21; total 67 preservado.
- Saldo DEBTs: 12 preservado.
- **21 aplicações cumulativas anti-inflação** pós-P205D
  preservadas.
- **Pattern "L0 minimal para refactors" N=9 EM VIGOR** —
  regra metodológica formal para refactors aditivos pós-M9c.
- **Pattern emergente "passo administrativo XS dedicado
  para promoção formal" N=1 inaugurado P229** (paridade
  P160A "passo administrativo XS" mas para promoção
  específica vs documentação geral).
- **Pattern emergente "passos administrativos XS NÃO
  ganham marca cirúrgica blueprint" N=1 inaugurado P229**.
- **Pattern "spec opcional → skip empírico pragmático"
  N=1 → 2 cumulativo** (P225 C9 + P229 C7).
- **Pattern "ADR PROPOSTO com materialização parcial
  graded" reduzido** — 3 ADRs PROPOSTAS pré-P229 → 2
  ADRs PROPOSTAS pós-P229 (ADR-0066 + ADR-0079).
- **Trajectória aberta pós-P229**: sub-passos Fase 5
  materialização caso-a-caso (A.3 per-cell candidato
  consolidação Categoria A; A.4 Block/Boxed; B.1 DEBT-34d;
  D.1 state; pivot outro módulo).

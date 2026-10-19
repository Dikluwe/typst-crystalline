# Relatório do passo P249 — Passo administrativo XS criar ADR meta "Promoções reais scope-outs ADR-0054 graded" PROPOSTO (paridade P156K/P160A/ADR-0062-create; formaliza pattern N=8 cumulativo granular)

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-249.md`.
**Tipo**: passo administrativo XS puramente documental
formalizando padrão metodológico empírico cumulativo. **Zero
código tocado.** **Zero variant Content.** **Zero entity novo.**
**Zero L0 prompts tocados.**
**Magnitude planeada**: XS (~30-60 min). **Magnitude real**:
**XS (~30 min)** — audit C1 directo + template ADR-0065/0080
reusável; `P249.div-2` formal activado (numeração ADR-0067 →
ADR-0082).
**Marco**: **sétima aplicação cumulativa do padrão "passo
administrativo XS"** N=6 → **N=7 cumulativo** (P156A
historiograma + P156K ADRs meta + ADR-0062-create + P160A +
P238 + P244 + **P249**); **terceira aplicação cumulativa do
padrão "ADR meta formalizar pattern N≥4 cumulativo"** N=2 →
**N=3 cumulativo** (P156K Smart→Option N=6 + P156K inventariar
primeiro N=5 + P234 L0 minimal N=7 + **P249 promoções reais
N=8 + ADR-0082**); décima segunda aplicação cumulativa "spec
C1 audit obrigatório bloqueante pós-P236.div-1" N=11 → 12
cumulativo (lição refinada P249: "ADR meta administrativo XS
exige audit empírico das N≥4 aplicações concretas antes de
formalizar pattern").

---

## §1 O que foi feito

P249 cria **ADR-0082 PROPOSTO** formalizando pattern empírico
"Promoções reais scope-outs ADR-0054 graded" que atingiu **N=8
cumulativo granular** pós-P248 (P242 ×2 + P247 ×3 + P248 ×3).

**Trabalho real**:

1. **Criou ADR-0082 nova** em
   `00_nucleo/adr/typst-adr-0082-promocoes-reais-scope-outs-graded.md`
   (~250 linhas; estrutura canónica paridade ADR-0065 + ADR-0080):
   - Status `PROPOSTO`.
   - Validado: 8 aplicações cumulativas granular pós-M9d.
   - Contexto: ADR-0054 perfil graded + pattern empírico
     P242/P247/P248 emergente.
   - Decisão: regra vinculativa + **4 critérios operacionais**
     (storage prévio + consumer Layouter graded + paridade
     vanilla referência + backward compat literal).
   - Justificação empírica: tabela cumulativa N=8 + contraste
     pré-P242 ad-hoc.
   - Alternativas: manter ad-hoc (rejeitada); revisão R1
     ADR-0054 (rejeitada); ADR meta única vs múltipla
     (decisão híbrida adoptada).
   - Implicações: positivas/neutras/negativas mitigadas.
   - Sub-padrões relacionados: A (promoção graded → real cross-
     passo N=2 P245+P248); B (P247 ortogonais aditivos N=1);
     C (P248 multi-consumer mecanismo comum N=1) — formalização
     meta-meta diferida (limiar N≥4 não satisfeito).
   - Plano de promoção: PROPOSTO → EM VIGOR pendente N=3
     aplicações consecutivas citantes (decisão humana).
   - Referências: ADR-0033/0054/0065/0080/0079 + passos P156G/
     H/P157B/P231/P242/P246/P247/P248/**P249**.
2. **Anotou ADR-0054** §"Promoções reais cumulativas" sub-secção
   nova (tabela N=8 + referência ADR-0082 PROPOSTO; status
   `EM VIGOR` preservado literal — refino interno).
3. **Anotou ADR-0080** §"Lição refinada P249" N=11 → N=12
   cumulativo ("ADR meta administrativo XS exige audit empírico
   das N≥4 aplicações concretas antes de formalizar pattern") +
   sub-padrão emergente "ADR meta formalizar pattern N≥4
   cumulativo" N=2 → N=3 cumulativo + sub-padrão "Passo
   administrativo XS" N=6 → N=7 cumulativo.
4. **Anotou README ADRs** entrada ADR-0082 PROPOSTO + entrada
   P249 nos passos-chave (paridade entradas P156K/ADR-0062-
   create/P160A precedentes).
5. **Relatório P249** (este ficheiro).

**2255 verdes preservados** (zero código tocado; zero
adaptações; paridade absoluta administrativos XS).
**Hashes L0 preservados** literal (zero L0 prompts tocados;
`crystalline-lint --fix-hashes` retorna "Nothing to fix").

**`P249.div-2` formal activado** — ADR-0067 já ocupada por
`attribute-grammar-scoping`; ADR-0082 escolhida como próximo
slot disponível após ADR-0081 (M7+ pipeline restructuring
scope).

---

## §2 Auditoria pré-P249 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=11 → 12 cumulativo

**Audit empírico** (lição refinada P248 N=11 → P249 N=12
cumulativo: "ADR meta administrativo XS exige audit empírico
das N≥4 aplicações concretas antes de formalizar pattern"):

| Aspecto | Hipótese Spec | Realidade Empírica | Implicação |
|---|---|---|---|
| Inventário 8 promoções pré-P249 | confirmado spec §1 | ✓ Confirmado P242×2 + P247×3 + P248×3 | OK |
| ADR-0054 §"Promoções reais" pré-existente | improvável | ✓ Confirmado **ausente** | Criar secção nova |
| ADR-0067 disponível | hipótese spec previa | **OCUPADA** (`attribute-grammar-scoping`) | `P249.div-2` formal → ADR-0082 |
| Template ADR-0065/0080 | hipotetizou reusável | ✓ Confirmado reuso directo | Sem refactor |
| Tests baseline pré-P249 | 2255 verdes | ✓ Confirmado | Baseline preservado |
| Hashes L0 baseline | "Nothing to fix" | ✓ Confirmado | Hashes preservados literal |

**Conclusão audit C1**: trabalho real ~280 LoC documental
(ADR-0082 ~250 LoC + anotações ADR-0054/ADR-0080/README ~30
LoC). Magnitude real **XS (~30 min)** face XS (~30-60 min)
hipotetizado.

**`P249.div-2` formal activado** (audit §2.3 confirmou ADR-0067
ocupada → ADR-0082 escolhida).

---

## §3 ADR-0082 PROPOSTO criada (C2)

**Conteúdo principal — 4 critérios operacionais**:

1. **Storage prévio**: scope-out já armazenado em campo de
   variant Content (não variant novo).
2. **Consumer Layouter pre-promoção é graded**: arm `field: _`
   ou "armazenado mas semantic adiada" ou "comportamento parcial
   via outro caminho".
3. **Paridade vanilla referência empírica**: confronto com
   `lab/typst-original/` em audit C1 obrigatório.
4. **Backward compat literal**: defaults pré-promoção preservados
   literais; N≈0 adaptações em tests pré-existentes.

**Tabela cumulativa N=8 (justificação empírica)** documentada
em ADR-0082 §"Justificação empírica" com matriz por critério
(P242×2 + P247×3 + P248×3 satisfazem todos os 4 critérios).

**Status PROPOSTO inicial** — promoção a EM VIGOR pendente:
1. Próxima aplicação N=9 cumulativa cita explícitamente
   ADR-0082.
2. N=3 aplicações consecutivas citantes atingidas (paridade
   ADR-0065 limiar interno).

---

## §4 Anotações cumulativas ADR-0054 + ADR-0080 + README (C3)

### ADR-0054 §"Promoções reais cumulativas" sub-secção nova

Tabela cumulativa N=8 + referência ADR-0082 PROPOSTO + nota
**status `EM VIGOR` preservado literal**.

### ADR-0080 §"Lição refinada P249" anotação

- Lição N=12 cumulativa: "ADR meta administrativo XS exige
  audit empírico das N≥4 aplicações concretas antes de
  formalizar pattern".
- Sub-padrão "ADR meta formalizar pattern N≥4 cumulativo"
  N=2 → N=3 cumulativo (P249).
- Sub-padrão "Passo administrativo XS" N=6 → N=7 cumulativo.
- Promoções reais scope-outs ADR-0054 graded granular N=8
  preservado P249.
- Cross-reference ADR-0082 PROPOSTO.

### README ADRs anotação

- Entrada ADR-0082 PROPOSTO em tabela "Estado por ADR".
- Entrada **P249 administrativo XS** em passos-chave (paridade
  entradas precedentes; ~30 linhas descritivas).

---

## §5 Critério aceitação P249 (C6+C7)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde preservado | ✓ verde (zero código) |
| `cargo test --workspace` | 2255 verdes preservado | ✓ 2255 verdes preservado |
| `crystalline-lint .` | 0 violations preservado | ✓ 0 violations |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | ✓ "Nothing to fix" |
| Content variants | 62 preservado | ✓ 62 |
| ShapeKind variants | 5 preservado | ✓ 5 |
| Block / Boxed / TableCell fields | preservados | ✓ preservados |
| Layouter / Regions fields | preservados | ✓ preservados |
| Stdlib funcs | 64 preservado | ✓ 64 |
| §A.5 distribuição | preservada literal | ✓ preservada |
| Cobertura Layout per metodologia | ~95-96% preservado | ✓ preservado |
| Cobertura user-facing total | ~75-76% preservado | ✓ preservado |
| **ADRs distribuição**: +ADR-0082 PROPOSTO | preservados restantes | ✓ ADR-0082 PROPOSTO criada |
| ADR-0054 status | EM VIGOR preservado | ✓ preservado (refino interno) |
| ADR-0080 anotação | §"Lição refinada P249" N=12 | ✓ |
| DEBT-34c / DEBT-34e / DEBT-30 | sentinelas preservadas | ✓ preservadas |
| L0 hashes propagados | 0 | ✓ 0 |
| Adaptações pre-existentes | N=0 | ✓ N=0 |
| Regressões reais | 0 mandatório | ✓ 0 |
| Patterns emergentes | 3 cumulativos esperados | ✓ todos |
| `P249.div-N` | possíveis 4 cenários | ✓ `P249.div-2` formal activado (ADR-0067 → ADR-0082) |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2255 verdes pré-P249 → **2255
   verdes** pós-P249 (paridade absoluta; zero código).
2. **Comemo memoization invariants ADR-0073/0074 preservados** —
   P249 não toca trait Introspector nem qualquer código L1.
3. **Backward compat literal**: zero código alterado; toda
   funcionalidade pré-P249 preservada literal.

**Promoções ADR**:
- **ADR-0082 PROPOSTO** criada (autorização arquitectural
  concedida em princípio; promoção a EM VIGOR pendente futura
  aplicação N=9 cumulativa citante).
- ADR-0054 §"Promoções reais cumulativas" sub-secção anotada
  (status `EM VIGOR` preservado).
- ADR-0080 §"Lição refinada P249" anotada N=12 cumulativo.
- **Sem outras ADRs criadas**.

---

## §6 Patterns emergentes inaugurados/consolidados P249 (3)

- **"Passo administrativo XS"** N=6 → **N=7 cumulativo P249**
  (P156A + P156K + ADR-0062-create + P160A + P238 + P244 +
  **P249**). Limiar formalização N=6 ultrapassado; pattern
  metodológico sólido reforçado.
- **"ADR meta formalizar pattern N≥4 cumulativo"** N=2 →
  **N=3 cumulativo P249** (P156K Smart→Option N=6 →
  ADR-0064; P156K inventariar primeiro N=5 → ADR-0065; P234
  L0 minimal N=7 → ADR-0080; **P249 promoções reais N=8 →
  ADR-0082**). **Limiar formalização interno N=3 atingido
  P249** (paridade ADR-0065 EM VIGOR pós-P156J + P157A + P157B
  sequente).
- **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"**
  N=11 → **N=12 cumulativo P249**. Lição refinada N=12: "ADR
  meta administrativo XS exige audit empírico das N≥4
  aplicações concretas antes de formalizar pattern".

**Padrão "Promoção real scope-out ADR-0054 graded" granular
N=8 preservado P249** (P249 administrativo XS não materializa
nova promoção; apenas formaliza pattern).

**Anti-inflação 41ª aplicação cumulativa** pós-P205D — Opção β
L0 minimal (zero L0 tocado; zero hashes propagados) + Opção α
ADR nova PROPOSTO (paridade ADR-0062-create + P160A) + Opção α
anotação cumulativa minimal ADR-0054 (secção nova refino
interno) + Opção α paridade administrativos XS precedentes
(N=6 → 7 cumulativo).

---

## §7 Próximo sub-passo pós-P249

P249 fecha formalização do pattern "Promoções reais scope-outs
ADR-0054 graded" via ADR-0082 PROPOSTO. Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 Block 4 scope-outs restantes** | spacing + above + below + sticky (paridade P247 agregada; **cita ADR-0082 PROPOSTO N=9** — primeira aplicação citante) | S-M | **alta** (Block A.4 completo 10/10; valida ADR-0082) |
| **A.4 Boxed 1 scope-out restante** | stroke-overhang (cita ADR-0082) | XS | baixa-média |
| **A.4 TableCell row break real** | Activação row break (refino P248 clip implícito; cita ADR-0082) | M-L | baixa-média |
| **ADR-0079 → IMPLEMENTADO graded** | Scope-out humano C.2 | XS-S | alta se humano decide fechamento |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo (colspan/rowspan real) | L+ | baixa (não-reservado P158) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| **Outro ADR meta admin XS** | Formalizar "Layouter consumer migration via API wrapper" N=1 (P246) ou "agregar multi-consumer mecanismo comum" N=1 (P248) — pendente N=3-4 limiar | XS | baixa (limiar não atingido) |

**Recomendação subjectiva pós-P249**: **A.4 Block 4 scope-outs
restantes** — sequente natural agregação P247/P248; **primeira
aplicação concreta a citar ADR-0082 PROPOSTO** (N=9 cumulativa);
se N=9 cita explicitamente, P249 valida-se empíricamente como
N=3 limiar formalização (paridade pattern ADR-0065 P156K
validado em P156J → P157A → P157B sequente).

**Decisão humana fica em aberto literal** pós-P249.

**Estado pós-P249**:
- Tests workspace: **2255 verdes preservado**.
- Content variants: **62 preservado**.
- Block / Boxed / TableCell fields: preservados.
- ShapeKind variants: **5 preservado**.
- Layouter / Regions fields: preservados.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: preservada literal.
- Cobertura Layout per metodologia: **~95-96% preservado**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição**: +ADR-0082 PROPOSTO; EM VIGOR preservado;
  IMPLEMENTADO preservado.
- **Saldo DEBTs: 11 preservado** (DEBT-34c+DEBT-34e+DEBT-30
  sentinelas preservadas; sem reabertura; sem novo DEBT).
- **41 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P249** (3):
  - "Passo administrativo XS" N=6 → **7 cumulativo**.
  - "ADR meta formalizar pattern N≥4 cumulativo" N=2 → **3
    cumulativo** (limiar interno atingido P249).
  - "Spec C1 audit obrigatório bloqueante" N=11 → **12
    cumulativo**.
- **Categoria A Fase 5 Layout**: A.4 muito reforçada cumulativa
  (5/9 Block + 5/6 Boxed + breakable real + height real + cell
  overflow real); **ADR-0082 formaliza pattern**.
- **Marco interno**: pattern "promoções reais scope-outs"
  formalizado em ADR-0082 PROPOSTO; padrão administrativo XS
  atinge N=7 (limiar sólido reforçado); padrão "ADR meta
  formalizar pattern N≥4 cumulativo" atinge N=3 (limiar interno
  atingido); lição C1 audit N=12 cumulativa refinada
  procedimentalmente; `P249.div-2` formal documentado
  (ADR-0067 ocupada → ADR-0082 escolhida); primeira aplicação
  cumulativa onde audit C1 é especificamente para "ADR meta
  administrativo XS formalizar pattern N≥4 cumulativo".

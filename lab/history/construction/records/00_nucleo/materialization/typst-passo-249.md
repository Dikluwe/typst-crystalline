# Spec do passo P249 — Passo administrativo XS criar ADR meta "Promoções reais scope-outs ADR-0054 graded" PROPOSTO (paridade P156K/P160A/ADR-0062-create; formaliza pattern N=8 cumulativo granular; sétima aplicação cumulativa "passo administrativo XS")

**Data**: 2026-05-14.
**Tipo**: passo administrativo XS puramente documental
formalizando padrão metodológico empírico cumulativo. **Zero
código tocado.** **Zero variant Content.** **Zero entity novo.**
Cria ficheiro ADR concreto novo a partir de pattern empírico
empíricamente atingido (N=8 cumulativo granular pós-P248).
**Magnitude planeada**: XS (~30-60 min documental; paridade
P156K + P160A + ADR-0062-create precedentes).
**Marco**: **sétima aplicação cumulativa do padrão "passo
administrativo XS"** N=6 → **7 cumulativo** (P156A historiograma
+ P156K ADRs meta + ADR-0062-create + P160A + P238 + P244 +
**P249**); **primeira aplicação cumulativa do padrão "ADR meta
formalizar pattern N≥4 cumulativo"** N=2 → **3 cumulativo**
(P156K critérios inventariar primeiro N=5; ADR meta admin
ADR-0080 N=2 já formalizada P234; **P249 N=3 cumulativo**
formaliza promoções reais scope-outs ADR-0054 graded N=8);
décima segunda aplicação cumulativa "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=11 → 12 cumulativo (refino
procedural P249: "ADR meta administrativo XS exige audit
empírico das N≥4 aplicações concretas antes de formalizar
pattern").

---

## §1 O que será feito

P249 cria **ADR meta nova** formalizando pattern empírico
"Promoções reais scope-outs ADR-0054 graded" que atingiu
**N=8 cumulativo granular** pós-P248. Análogo estrutural a:

- **ADR-0065** (P156K — formalizou pattern "Inventariar
  primeiro" N=5).
- **ADR-0064** (P156K — formalizou pattern "Smart→Option/default"
  N=6).
- **ADR-0066** (P160A — formalizou reserva conceptual
  Introspection runtime adiada).
- **ADR-0080** (P234 — formalizou pattern L0 minimal para
  refactors).

### Estado pré-P249 confirmado empíricamente (2026-05-14)

Padrão "Promoção real scope-out ADR-0054 graded" tem **N=8
aplicações concretas cumulativas granular** pós-P248:

| # | Passo | Scope-out promovido | Origem (graded) | Magnitude |
|---|-------|---------------------|-----------------|-----------|
| 1 | P242 | radius (Block + Boxed) | P156G + P156H scope-out | M |
| 2 | P242 | clip (Block + Boxed) | P156G + P156H scope-out | (incluído P242) |
| 3 | P247 | outset semantic real (Block + Boxed) | P156G + P156H + P231 graded | (incluído P247) |
| 4 | P247 | fill (Block + Boxed) | P156G + P156H scope-out | M |
| 5 | P247 | stroke (Block + Boxed) | P156G + P156H scope-out | (incluído P247) |
| 6 | P248 | Block.breakable semantic real | P156G "semantic adiada" | M |
| 7 | P248 | Boxed.height overflow real | P156H "semantic adiada" | (incluído P248) |
| 8 | P248 | TableCell.body overflow clip implícito | P157B "ignorados em layout" | (incluído P248) |

**N=8 cumulativo granular** ultrapassa **limiar formalização
N=4 sólido** declarado em ADR-0065 §"Justificação empírica"
(também N=5 do precedente ADR-0064/ADR-0065 P156K).

### Padrão "promoção graded → real semantic activação consumer"

Sub-padrão **N=2 cumulativo agregado** pós-P248:
- N=1 (P245 Place float real — primeiro storage P223 → semantic
  P245 cross-passo).
- N=2 (P248 agregado 3 sub-activações Block.breakable +
  Boxed.height + TableCell overflow via mecanismo comum
  `measure_content_constrained`).

**P249 NÃO formaliza este sub-padrão separadamente** — é
sub-pattern intrincado com "Promoção real scope-out" ADR-0054
graded; tratamento conjunto na ADR meta única.

### Padrão "agregar promoções via mecanismo comum"

Sub-padrão **N=1 inaugurado P248** ("activação semantic real
multi-consumer via mecanismo comum" — partilha
`measure_content_constrained`). Distinto N=1 P247 "agregar
cosméticos visuais ortogonais" (sem mecanismo comum;
ortogonalidade aditiva).

**P249 anota ambos como sub-patterns relacionados** na ADR
meta mas formalização meta-meta diferida.

### Trabalho a fazer P249

1. **Criar ficheiro ADR novo** em `00_nucleo/adr/typst-adr-0067-promocoes-reais-scope-outs-graded.md` (numeração próxima disponível pós-ADR-0066 P160A; confirmar pré-existência audit §2.4).
2. **Estrutura canónica** (paridade ADR-0065/ADR-0066/ADR-0080):
   - Status (`PROPOSTO`)
   - Data + Autor
   - Validado (P242/P245/P247/P248 evidência cumulativa)
   - Contexto (cita ADR-0054 perfil graded como fundamento)
   - Decisão (regra vinculativa "promoção real scope-out
     ADR-0054 graded segue 4 critérios operacionais")
   - Justificação empírica (tabela N=8 + tabela contraste
     pré-P242 manualidade ad-hoc)
   - Alternativas consideradas (manter ad-hoc; revisão R1
     ADR-0054; ADR meta única vs múltipla)
   - Implicações (sessões futuras citam ADR-0067; redução
     overhead enunciados)
   - Referências (ADR-0033/0054/0065/0080 + passos)
3. **Anotação ADR-0054** §"Promoções reais scope-outs"
   (secção nova): tabela cumulativa N=8 + referência ADR-0067
   PROPOSTO; **sem promoção ADR-0054 status** (permanece EM
   VIGOR; refino interno apenas).
4. **README ADRs anotação**:
   - Total **68 → 69 ADRs** (ADR-0067 adicionada).
   - Distribuição PROPOSTO **12 → 13** (+0067 PROPOSTO);
     EM VIGOR 29 preservado; IMPLEMENTADO 23 preservado.
5. **Relatório P249** estrutura canónica administrativos XS.

### Tests esperados

Tests P249 novos: **0** (paridade absoluta administrativos XS
precedentes P156A/P156K/ADR-0062-create/P160A/P238/P244).
Workspace pós-P249: **2255 verdes preservado**.

### Adaptações pre-existentes

**N=0** (paridade absoluta administrativos XS).

---

## §2 Verificação empírica pré-P249 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=11 → 12 cumulativo

Audit C1 obrigatório bloqueante pós-P236.div-1. Lição refinada
N=11 P248 ("mapear pontos de check overflow existentes antes
de adicionar novos checks duplicados") expande para **N=12
cumulativo**: "ADR meta administrativo XS exige audit empírico
das N≥4 aplicações concretas antes de formalizar pattern".

### §2.1 Inventário promoções reais cumulativas pré-P249 (factual)

Auditar empíricamente cada uma das 8 promoções listadas em §1:

```bash
grep -n "P242\|P247\|P248" 00_nucleo/adr/typst-adr-0079-*.md
grep -n "P242\|P247\|P248" 00_nucleo/adr/typst-adr-0080-*.md
grep -rn "Promoção real scope-out\|promoção real" 00_nucleo/adr/
```

Confirmar literal cada anotação documental que sustenta as 8
aplicações cumulativas.

### §2.2 ADR-0054 estado actual

```bash
grep -n "Promoções reais\|promoção real" 00_nucleo/adr/typst-adr-0054-*.md
head -30 00_nucleo/adr/typst-adr-0054-*.md
```

Identificar:
- Tem §"Promoções reais" pré-existente? (Provavelmente **não**
  — pesquisa anterior revelou que ADR-0054 não tem secção
  formal; só referências esparsas em anotações
  P242/P247/P248).
- Status ADR-0054 actual (esperado: EM VIGOR; P249 preserva).

### §2.3 Próximo número ADR disponível

```bash
ls 00_nucleo/adr/typst-adr-00*.md | sort | tail -5
```

Confirmar **ADR-0067 é próximo disponível** (pós-ADR-0066
P160A; ADR-0063 reservada column flow; outras consecutivas
ocupadas).

### §2.4 Estrutura canónica precedentes

`head -50 00_nucleo/adr/typst-adr-0065-*.md 2>/dev/null` —
template estrutural directo. Idem ADR-0080 e ADR-0066.

### §2.5 Tests pré-P249 baseline

```bash
cargo test --workspace
```

Esperado: **2255 verdes** (estado pós-P248).

### §2.6 Hashes L0 baseline

```bash
crystalline-lint --fix-hashes
```

Esperado: **"Nothing to fix"** (paridade P249 administrativo
XS; zero código L1 tocado; L0 prompts inalterados;
`entities/content.md` hash `9f03e1a8` pós-P248 preservado).

### §2.7 Decisão arquitectural pós-audit

Após §2.1-§2.6 completos, fixar empíricamente:
- **Decisão 1** numeração ADR confirmada empíricamente §2.3.
- **Decisão 2** secção ADR-0054 "Promoções reais" — criar
  nova ou anotar inline em §"Decisão" ou §"Perfil de paridade".

### `P249.div-N` antecipadas — possíveis

- **`P249.div-1`** se §2.1 revelar discrepância nas 8 aplicações
  (ex: anotação P247 não corresponde exactamente à descrição
  do spec) → reconciliação prévia.
- **`P249.div-2`** se §2.3 revelar ADR-0067 já ocupada → usar
  ADR-0068 disponível.
- **`P249.div-3`** se §2.5 baseline ≠ 2255 → reconciliação
  prévia tests.
- **`P249.div-4`** se ADR-0054 §"Promoções reais" já tiver
  secção formal pré-existente não conhecida → escopo reduzido
  (apenas anotar cumulativo em vez de criar nova).

---

## §3 Decisões fixadas P249 — 7 decisões

### Decisão 0 — Audit C1 lição N=11 → 12 cumulativo

Pattern "spec C1 audit obrigatório bloqueante pós-P236.div-1"
N=11 → **12 cumulativo**. Refino procedural P249: "ADR meta
administrativo XS exige audit empírico das N≥4 aplicações
concretas antes de formalizar pattern". Anotação em ADR-0080
§"Lição refinada P249".

### Decisão 1 — Numeração ADR-0067 (preliminar; confirmar pós-audit §2.3)

ADR-0067 é próximo número disponível pós-ADR-0066 P160A
(ADR-0063 reservada column flow; 0064 ADR-0064 P156K Smart→Option;
0065 ADR-0065 P156K inventariar primeiro; 0066 ADR-0066 P160A
Introspection runtime). **Confirmar empíricamente §2.3**; se
ocupada, usar próximo livre.

### Decisão 2 — Conteúdo ADR-0067: 4 critérios operacionais para promoção real

Conteúdo proposto para §"Decisão" da ADR-0067:

**Regra vinculativa**: Promoção real de scope-out ADR-0054
graded segue 4 critérios operacionais:

1. **Storage prévio**: o scope-out já está armazenado em campo
   de variant Content (não promoção a variant novo). Distingue
   "promoção real" de "materialização nova".

2. **Consumer Layouter pre-promoção é graded**:
   - "armazenado mas ignorado" (default `_` em arm).
   - "armazenado mas semantic adiada" (lido mas sem semantic
     completa).
   - "comportamento parcial via outro caminho" (ex: outset
     activo em Grid mas não Block isolado).
   Decisão de promoção é "activar/completar semantic real
   no consumer".

3. **Paridade vanilla referência empírica**: implementação real
   deve ser confrontada com `lab/typst-original/crates/typst-*/`
   antes de cristalizar. Divergências graded permitidas per
   ADR-0054 documentadas em "Limitações conscientes" do passo.

4. **Backward compat literal**: defaults pré-promoção
   preservados literais — output PDF bit-equivalente para casos
   default (paridade tests sentinela P248
   `*_default_preserva_*`). Adaptações em tests pré-existentes
   esperadas N≈0.

**Justificação cumulativa**: 8 aplicações concretas (P242 ×2 +
P247 ×3 + P248 ×3) com 0 reformulações arquiteturais; pattern
empírico sólido N≥4 ultrapassado largamente.

### Decisão 3 — ADR-0067 status `PROPOSTO` inicial

Paridade ADR-0065 P156K + ADR-0066 P160A. **Promoção a EM
VIGOR** ocorre quando:
1. Próxima aplicação N=9 cumulativa cita explícitamente
   ADR-0067 (em vez de re-justificar empíricamente).
2. **N=3 aplicações consecutivas citantes** atingidas (paridade
   limiar formalização interno).

**Decisão de promoção é humana** — não automática per passo
materialização.

### Decisão 4 — Anotação ADR-0054 secção nova "Promoções reais"

Adicionar secção nova em ADR-0054 §"Perfil de paridade"
sub-secção "§Promoções reais cumulativas":

> **Pós-P249**: o perfil graded permite **promoção real** de
> scope-outs declarados (refino futuro per ADR-0054 graded
> documentado em "Limitações conscientes" do passo de origem).
> Tabela cumulativa pós-P248:
>
> | # | Passo | Scope-out | Origem |
> |---|-------|-----------|--------|
> | 1-2 | P242 | radius + clip | P156G/H |
> | 3-5 | P247 | outset + fill + stroke | P156G/H + P231 |
> | 6-8 | P248 | breakable + height + cell overflow | P156G/H/P157B |
>
> **Padrão metodológico de promoção formalizado em ADR-0067
> PROPOSTO** (P249 administrativo XS).
>
> ADR-0054 **status EM VIGOR preservado**; refino interno
> apenas — não reaberta nem revogada.

### Decisão 5 — Sem nova reserva; sem reabertura ADR-0054

Política P158 "sem novas reservas" preservada. ADR-0067
formaliza pattern empírico pré-existente, não reserva
conceptual nova. ADR-0054 permanece EM VIGOR literal.

### Decisão 6 — Anti-inflação 41ª aplicação cumulativa

- Opção β L0 minimal: zero L0 prompts tocados; hashes
  preservados literal.
- Opção α ADR nova `PROPOSTO` (paridade ADR-0062-create +
  P160A): autorização arquitectural concedida sem materialização
  imediata.
- Opção α anotação cumulativa minimal ADR-0054 (secção nova
  refino interno).
- Opção α paridade administrativos XS precedentes (N=6 → 7
  cumulativo).

### Decisão 7 — Patterns emergentes cumulativos P249 (3)

- **"Passo administrativo XS"** N=6 → **7 cumulativo** (P156A
  + P156K + ADR-0062-create + P160A + P238 + P244 + **P249**).
- **"ADR meta formalizar pattern N≥4 cumulativo"** N=2 → **3
  cumulativo** (P156K Smart→Option N=6 + inventariar primeiro
  N=5; ADR-0080 L0 minimal P234 N=2; **P249 promoções reais
  scope-outs N=8**).
- **"Spec C1 audit obrigatório bloqueante"** N=11 → **12
  cumulativo**.

---

## §4 Ficheiros a editar (C2+C3+C4+C5)

| Categoria | Ficheiro | Trabalho |
|-----------|----------|----------|
| ADR nova | `00_nucleo/adr/typst-adr-0067-promocoes-reais-scope-outs-graded.md` | **Criar do zero**: estrutura canónica paridade ADR-0065 (~80-100 linhas: Status / Data / Validado / Contexto / Decisão / Justificação empírica tabela N=8 / Alternativas consideradas / Implicações / Referências / Próximos passos) |
| ADR-0054 anotação | `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` | Adicionar sub-secção "§Promoções reais cumulativas" em §"Perfil de paridade"; tabela N=8 + referência ADR-0067 PROPOSTO; **status EM VIGOR preservado literal** |
| README ADRs | `00_nucleo/adr/README.md` | Adicionar entrada P249 administrativo XS (paridade entradas P244 + P160A + ADR-0062-create); contagens ADRs 68 → 69; PROPOSTO 12 → 13 |
| ADR-0080 anotação | `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md` | §"Lição refinada P249" anotada N=12 cumulativo (refino procedural "ADR meta administrativo XS exige audit empírico das N≥4 aplicações concretas"); cross-reference a ADR-0067 PROPOSTO |
| Relatório P249 | `00_nucleo/materialization/typst-passo-249-relatorio.md` | Estrutura canónica administrativos XS (paridade P156K + P160A + P244 + ADR-0062-create) |

**Sem L0 prompts tocados.** **Sem entities tocadas.** **Sem
rules tocadas.** **Sem stdlib tocada.** **Sem inventário 148
tocado** (refino administrativo não-feature).

---

## §5 Critério aceitação P249 (C6+C7)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | **verde preservado** (zero código tocado) |
| `cargo test --workspace` | **2255 verdes preservado** (paridade absoluta administrativos XS) |
| `crystalline-lint .` | **0 violations preservado** |
| `crystalline-lint --fix-hashes` | **"Nothing to fix"** (paridade administrativos XS) |
| Content variants | **62 preservado** |
| ShapeKind variants | **5 preservado** |
| Block / Boxed / TableCell fields | preservados |
| Layouter / Regions fields | preservados |
| Stdlib funcs | **64 preservado** |
| §A.5 distribuição | preservada literal (refino administrativo) |
| Cobertura Layout per metodologia | **~95-96% preservado** |
| Cobertura user-facing total | **~75-76% preservado** |
| **ADRs distribuição**: PROPOSTO 12 → **13** | (+ADR-0067 PROPOSTO); EM VIGOR 29 preservado; IMPLEMENTADO 23 preservado; **total 68 → 69** |
| ADR-0054 status | **EM VIGOR preservado** (refino interno secção nova "Promoções reais" anotada) |
| ADR-0080 anotação | §"Lição refinada P249" N=12 cumulativo |
| DEBT-34c / DEBT-34e / DEBT-30 | sentinelas preservadas |
| L0 hashes propagados | **0** (paridade administrativos XS) |
| Adaptações pre-existentes | **N=0** (paridade absoluta administrativos XS) |
| Regressões reais | **0** mandatório |
| Patterns emergentes | "Passo administrativo XS" N=6 → 7 cumulativo; "ADR meta formalizar pattern N≥4" N=2 → 3 cumulativo; "Spec C1 audit" N=11 → 12 cumulativo |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2255 verdes pré-P249 →
   2255 verdes pós-P249 (paridade absoluta; zero código).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P249 não toca trait Introspector nem methods nem qualquer
   código L1; invariants preservados literal.
3. **Backward compat**: zero código alterado; toda funcionalidade
   pré-P249 preservada literal.

**Promoções ADR esperadas**:

- **ADR-0067 PROPOSTO** criada (autorização arquitectural
  concedida em princípio; promoção a EM VIGOR pendente futura
  aplicação N=9 cumulativa citante).
- ADR-0054 §"Promoções reais cumulativas" sub-secção anotada
  (status EM VIGOR preservado).
- ADR-0080 §"Lição refinada P249" anotada.
- **Sem outras ADRs criadas**.

---

## §6 Próximo sub-passo pós-P249

P249 fecha formalização do pattern "Promoções reais scope-outs
ADR-0054 graded". Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 Block 4 scope-outs restantes** | spacing + above + below + sticky (paridade P247 agregada; cita ADR-0067 PROPOSTO N=9 → EM VIGOR possível) | S-M | **alta** (Block A.4 completo 10/10) |
| **A.4 Boxed 1 scope-out restante** | stroke-overhang | XS | baixa |
| **A.4 TableCell row break real** | Activação row break (refino P248 clip implícito) | M-L | baixa-média |
| **ADR-0079 → IMPLEMENTADO graded** | Scope-out humano C.2 | XS-S | alta se humano decide fechamento |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo | L+ | baixa (não-reservado P158) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| **Outro ADR meta admin XS** | Formalizar "Layouter consumer migration via API wrapper" N=1 (P246) ou "agregar promoções multi-consumer mecanismo comum" N=1 (P248) — pendente N=3-4 limiar | XS | baixa (limiar não atingido) |

**Recomendação subjectiva pós-P249**: **A.4 Block 4 scope-outs
restantes** — sequente natural agregação P247/P248; **primeira
aplicação concreta a citar ADR-0067 PROPOSTO** (N=9 cumulativa);
se N=9 cita explicitamente, P249 valida-se empíricamente como
N=3 limiar formalização (paridade pattern ADR-0065 P156K que
ganhou validação em P156J → P157A → P157B sequente).

**Decisão humana fica em aberto literal** pós-P249.

**Estado esperado pós-P249**:
- Tests workspace: **2255 verdes preservado**.
- Content variants: **62 preservado**.
- Block / Boxed / TableCell fields: preservados.
- Cobertura Layout per metodologia: **~95-96% preservado**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição**: PROPOSTO 12 → **13** (+ADR-0067);
  EM VIGOR 29 preservado; IMPLEMENTADO 23 preservado; **total
  68 → 69**.
- **Saldo DEBTs: 11 preservado**.
- **41 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P249** (3):
  - "Passo administrativo XS" N=6 → **7 cumulativo**.
  - "ADR meta formalizar pattern N≥4 cumulativo" N=2 → **3
    cumulativo**.
  - "Spec C1 audit obrigatório bloqueante" N=11 → **12
    cumulativo**.
- **Categoria A Fase 5 Layout**: A.4 muito reforçada cumulativa
  (5/9 Block + 5/6 Boxed + breakable real + height real
  + cell overflow real); **ADR-0067 formaliza pattern**.
- **Marco interno**: pattern "promoções reais scope-outs"
  formalizado em ADR concreta PROPOSTO; padrão administrativo
  XS atinge N=7 (limiar sólido reforçado); padrão "ADR meta
  formalizar pattern N≥4 cumulativo" atinge N=3 (limiar interno
  atingido); lição C1 audit N=12 cumulativa refinada
  procedimentalmente ("ADR meta administrativo XS exige audit
  empírico das N≥4 aplicações concretas antes de formalizar
  pattern").

---

## §7 Notas operacionais para o executor

1. **Audit C1 BLOQUEANTE prioridade absoluta**. Não materializar
   antes de §2.1-§2.6 completos. **Lição N=12 cumulativa**:
   primeira aplicação onde audit C1 é especificamente para
   "ADR meta administrativo XS formalizar pattern N≥4
   cumulativo". Se §2.1 revelar discrepância nas 8 aplicações
   (ex: anotação P247 não corresponde) → `P249.div-1` formal
   + reconciliação prévia.

2. **Decisão 1 final fixa pós-audit §2.3**. ADR-0067 é número
   esperado mas confirmar empíricamente.

3. **Decisão 2 final fixa pós-audit §2.2**. Se ADR-0054 já
   tiver secção "Promoções reais" pré-existente (improvável
   mas possível), `P249.div-4` formal + escopo reduzido.

4. **Estrutura ADR-0067 paridade ADR-0065 P156K**. Não criar
   formato novo; reusar template empírico.

5. **Custo real esperado**: ~30-60 min (paridade P156K +
   P160A + ADR-0062-create + P244 administrativos XS
   precedentes). Maior parcela: redacção da ADR-0067 (~30-40
   min) + anotações ADR-0054/ADR-0080/README (~10-15 min) +
   relatório P249 (~10-15 min).

6. **Sem `P249.div-N` antecipado normal**. 4 cenários
   contingenciais em §2.7; pouco prováveis.

7. **Anti-inflação 41ª aplicação cumulativa** pós-P205D
   preservar: Opção β L0 minimal (zero L0 tocado) + Opção α
   ADR nova PROPOSTO + Opção α anotação cumulativa minimal
   ADR-0054 + Opção α paridade administrativos XS precedentes.

8. **Sem promoção ADR-0067 → EM VIGOR em P249**. Status
   PROPOSTO inicial paridade ADR-0065/ADR-0066/ADR-0080
   precedentes. **Promoção a EM VIGOR** é decisão humana
   posterior conforme N=3 aplicações consecutivas citantes
   (paridade ADR-0065 ganhou EM VIGOR pós-P156K via
   aplicações concretas P156J/P157A/P157B).

9. **ADR-0079 + ADR-0061 + ADR-0079 + ADR-0080 + DEBT.md
   intocados** salvo anotações descritas em §4. Decisão
   formal de promoção ADR-0079 → IMPLEMENTADO mantém-se em
   aberto humano (não-bloqueia P249).

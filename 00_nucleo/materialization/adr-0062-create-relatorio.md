# Relatório ADR-0062-create — Criar ADR-0062 PROPOSTO (administrativo XS)

Passo administrativo XS para formalizar reserva pré-existente
de ADR-0062 (autorização de crate `hayagriva`) como ficheiro
ADR concreto com status `PROPOSTO`. **Não materializa código**.
**Não promove** ADR-0062 a `EM VIGOR` ou `IMPLEMENTADO` —
apenas formaliza a reserva.

**Subpadrão emergente N=1**: "passo administrativo XS criar
ADR PROPOSTO a partir de reserva pré-existente" — primeiro
do tipo nesta sessão. Candidato a precedente para outras
reservas (e.g. ADR-0063 column flow se relevante).

---

## 1. Resumo do executado

### 1.1 Inventário (.1)

Confirmado:
- **ADR-0062 era reserva sem ficheiro** (apenas menção em
  README ADRs e relatórios — paridade P159 §1.2 + P159A).
- **Precedentes citáveis confirmados**: ADR-0023 (`indexmap`),
  ADR-0024 (`ecow`), ADR-0057 (`hypher`) — todos existem como
  ficheiros com estrutura canónica autorização crate L1.
- **Naming convention**: `typst-adr-NNNN-titulo-kebab-case.md`
  per padrão. Naming escolhido:
  `typst-adr-0062-hayagriva-bibliography-parsing.md`.
- **Estrutura canónica**: Status / Data / Contexto / Análise de
  pureza / Decisão / Precedentes / Crate informação técnica /
  Consequências / Alternativas / Plano de promoção / Referências.

### 1.2 Redacção ADR-0062 PROPOSTO (.2)

Ficheiro novo:
`00_nucleo/adr/typst-adr-0062-hayagriva-bibliography-parsing.md`.

**Conteúdo principal**:
- Status `PROPOSTO`, Data 2026-04-27.
- **Contexto**: vanilla integra hayagriva profundamente (1226
  linhas em `model/bibliography.rs`); P159A subset minimal
  cristalino sem hayagriva insuficiente para paridade ADR-0060
  ~68%; DEBT-55 4/10 itens pendentes.
- **Análise de pureza** (paridade ADR-0023): Zero I/O ✓; Zero
  estado global ✓; Determinismo total ✓; Dependências
  transitivas a verificar.
- **Decisão**: autorizar `hayagriva` em L1 para CSL parsing,
  bibliography parsing externo, citation strings.
- **Precedentes citáveis**: ADR-0023/0024/0057 com tabela
  comparativa.
- **Crate hayagriva info**: versão 0.9.1 (probe P152); MIT;
  mantida pela mesma org que typst.
- **Consequências**: positivas (paridade, reuso CSL, manutenção
  partilhada); negativas (dependência externa, tempo compilação,
  conflito versão); neutras (DEBT-55 desbloqueia, ADR-0017
  separada).
- **Alternativas consideradas** (5 opções tabuladas).
- **Plano de promoção futuro** (8 passos para PROPOSTO →
  IMPLEMENTADO).
- **Referências** (12 ADRs/passos/DEBTs).

### 1.3 README ADRs actualizado (.3)

- Total ADRs: 63 → **64**.
- Reservas: ADR-0062 substituída por entrada concreta com
  referência ao ficheiro novo + status PROPOSTO + plano de
  promoção.
- Tabela "Estado por ADR": entrada `0062` adicionada com
  status `PROPOSTO`.
- Distribuição status: PROPOSTO 11 → **12** (+0062); EM VIGOR
  28 inalterado; IMPLEMENTADO 19 inalterado.
- Entrada cronológica `ADR-0062-create` adicionada antes de
  P159B (ordem reversa).

### 1.4 ADR-0061 §"Aplicações cumulativas" actualizado (.4)

- Linha cronológica nova `ADR-0062-create` com slope "—" e
  tests Δ "0".
- Padrões metodológicos:
  - Inventariar primeiro N=15 → **16** (critério #1 naming +
    #5 inventário trivial).
  - §análise de risco N=15 → **16** (passo administrativo XS
    muito baixo risco).
- Estado pós-ADR-0062-create documentado.
- Hash content.rs preservado **11º passo consecutivo** (P156L
  → ADR-0062-create).

### 1.5 Verificação documental (.5)

- `crystalline-lint`: zero violations ✓.
- Links cruzados verificados: ADR-0023/0024/0057 + ADR-0033 +
  ADR-0034 + ADR-0054 + ADR-0060 + ADR-0017 + DEBT-55 +
  P152/P159/P159A/P159B + `ADR-0062-create`.

---

## 2. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | Ficheiro ADR-0062 criado em `00_nucleo/adr/typst-adr-0062-*.md` | **✓** `typst-adr-0062-hayagriva-bibliography-parsing.md` |
| 2 | Status `PROPOSTO` documentado | **✓** linha 3 do ficheiro |
| 3 | Estrutura canónica seguida | **✓** 11 secções (Status / Data / Contexto / Análise pureza / Decisão / Precedentes / Crate info / Consequências / Alternativas / Plano promoção / Referências) |
| 4 | ADRs precedentes (0024/0023/0057) citados como precedente "autorização crate externa" | **✓** §"Precedentes citáveis" com tabela comparativa |
| 5 | ADR-0060 + ADR-0033 + DEBT-55 + P159A/B referenciadas | **✓** §"Referências" com 12 entradas |
| 6 | README ADRs actualizado: entrada substituída; contagem **64** | **✓** entrada cronológica + tabela status + distribuição actualizadas |
| 7 | ADR-0061 §"Aplicações cumulativas" actualizada com linha `ADR-0062-create` | **✓** linha + padrões N=15→16 |
| 8 | Sem código alterado — `entities/content.rs` mantém `ec58d849` | **✓ 11º passo consecutivo** P156L → ADR-0062-create |
| 9 | Sem novas reservas criadas | **✓** política P158 preservada — formaliza pré-existente, não cria nova |
| 10 | ADR-0062 NÃO promovida a EM VIGOR/IMPLEMENTADO | **✓** status PROPOSTO; promoção em passo futuro materialização hayagriva |
| 11 | `crystalline-lint`: zero violations | **✓ No violations found** |

---

## 3. Análise de risco (padrão N=15 → 16; passo administrativo XS)

ADR-0062-create é **passo administrativo XS** sem alteração de
código. **Décima sexta aplicação consecutiva** de §análise de
risco preservando precedente.

### 3.1 Riscos identificados

| Risco | Avaliação | Mitigação aplicada |
|-------|-----------|---------------------|
| ADR-0024/0023/0057 não existirem ou ter scope diferente | Não materializou — todas confirmadas existentes com estrutura canónica | Verificação pré-execução em §1.1 |
| Convenção naming ter mudado | Não materializou — convenção `typst-adr-NNNN-titulo-kebab-case.md` confirmada | Verificação pré-execução |
| Conteúdo concreto da reserva já documentado algures não visto | Parcial — DEBT-55 documenta plano XL (re-citado); README documenta menção breve (re-citado) | Reuso de informação documentada |
| Outras reservas em estado "sem ficheiro" tentadora a criar simultaneamente | Não materializou — preservada atomicidade (uma reserva por passo XS) | ADR-0017/ADR-0063 mencionadas mas NÃO formalizadas neste passo |

### 3.2 Riscos avaliados mas não materializados

| Risco | Avaliação inicial | Razão de não-materialização |
|-------|-------------------|----------------------------|
| Decisão de Status ser ambígua (PROPOSTO vs IDEIA vs ADIADO) | Baixo | PROPOSTO é status correcto per ADR-0062 ter decisão tomada mas código não em vigor |
| Análise de pureza não-trivial para crate externa | Baixo | Padrão estabelecido em ADR-0023/0024/0057 — tabela 4 linhas suficiente |

### 3.3 Riscos não-aplicáveis

- **Refactor de código**: zero (passo puramente documental).
- **Quebra de contrato API**: zero.
- **Drift de hashes**: zero.
- **Promoção prematura de ADR-0062 a IMPLEMENTADO**: zero
  (deliberadamente PROPOSTO).

### 3.4 Conclusão de risco

**Risco residual: muito baixo**. Padrão "passo administrativo
XS criar ADR PROPOSTO a partir de reserva pré-existente"
estabelece precedente novo nesta sessão. Reuso de estrutura
canónica + precedentes citáveis + plano de promoção tornam o
passo replicável (e.g. para ADR-0063 column flow se prioritário).

---

## 4. Subpadrão emergente — passo administrativo XS criar ADR PROPOSTO

ADR-0062-create estabelece subpadrão **N=1** dentro da série
diagnóstico/administrativa cumulativa P156-P159B:

**Características**:
- Tamanho XS (passo único, ~30min trabalho).
- Sem código alterado.
- Cria ficheiro ADR concreto a partir de reserva pré-existente
  documentada em README + relatórios.
- Status `PROPOSTO` (não promove).
- Total ADRs incrementa em 1.

**Replicabilidade**:
- ADR-0017 Introspection runtime adiada (já tem ficheiro como
  `IMPLEMENTADO`; não aplicável).
- ADR-0063 column flow (reservada; aplicável se DEBT-56
  materializado e prioritário).
- Outras reservas futuras (preservar política "sem novas
  reservas" — só formalizar pré-existentes).

**Candidato a formalização** se aplicado N=2-3 vezes.

---

## 5. Estado pós-ADR-0062-create

- **Cobertura Layout**: **78%** (inalterada — passo
  administrativo XS).
- **Cobertura Model agregada**: ~50% (inalterada).
- **Cobertura arquitectural total**: **82%** (inalterada).
- **Variants Content**: **58** (inalterada).
- **Stdlib funcs**: **48** (inalterada).
- **Tests**: **1174** typst-core lib (inalterada). Workspace:
  **1434** (inalterada).
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados.
- **ADR-0061** §"Aplicações cumulativas": tabela slope ganha
  linha `ADR-0062-create`; padrões N=15 → 16.
- **README ADRs**: entrada cronológica adicionada; tabela
  Estado actualizada com 0062 PROPOSTO; distribuição status
  PROPOSTO 11 → 12.
- **Reservas pré-existentes**: ADR-0017 Introspection runtime
  adiada mantida (não formalizada — preserva atomicidade);
  ADR-0063 column flow mantida.
- **Hash `content.rs`**: `ec58d849` (preservado — **11º passo
  consecutivo** P156L → ADR-0062-create).
- **Total ADRs**: 63 → **64**.

### 5.1 Implicação para Bloco B do diagnóstico P159B

**Bloco B (refinos com hayagriva ADR-0062)** agora pode
iniciar com referência concreta a ADR-0062 PROPOSTO em vez de
referência a reserva sem ficheiro:

- **P159G** Cargo.toml + crystalline.toml hayagriva: pode
  citar ADR-0062 como autorização (ainda PROPOSTO; promoção
  ocorre quando código real for adicionado).
- **P159H** hayagriva integration minimal: idem.
- **P159I/J** CSL styles: idem.

ADR-0062 transita `PROPOSTO → IMPLEMENTADO` no primeiro passo
que adicionar real `hayagriva = "0.9.1"` ao Cargo.toml.

---

## 6. Decisão pós-ADR-0062-create

Per §"Pós-passo" + política "sem novas reservas":

**Bloco A (refinos puramente Model — recomendação primária P159B §6)**:
- **P158B** Supplement automático por lang em figure (M).
- **P159C** Cite.form variants (M).
- **P159D** BibEntry fields adicionais (S+).
- **P158C** Refactor `kind: String → Option<String>` (XS).
- **P159F** Numbering numérico simples Bibliography (M).

**Bloco B (refinos com hayagriva — DESBLOQUEADO)**:
- **P159G** Cargo.toml + crystalline.toml hayagriva (XS).
- **P159H** hayagriva integration minimal (M+).
- **P159I/J** CSL styles (M cada).

**Outras direcções** (sem reservas reforçadas):
- Continuar Fase 3 Layout (DEBT-56).
- Footnote area.
- Atacar Introspection (17%).
- Promover ADR-0061 a IMPLEMENTADO.
- Promover `extract_length` a helper público.
- Fechar DEBT-34e + DEBT-56.
- Promover ADR-0060 a R1.
- ADR meta XS de "ADR-0064 caso completion".
- ADR-0017 formalização análoga (se prioritário).
- Actualizar L0 prompt content.md (se prioritário).

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. **ADR-0062 PROPOSTO** (era reserva).

**Padrão granularidade N=14 NÃO desafiado** por este passo
(administrativo XS).

---

## 7. Fechamento

ADR-0062-create fecha como **passo administrativo XS** —
primeiro do tipo "criar ADR PROPOSTO a partir de reserva
pré-existente" nesta sessão. **Subpadrão emergente N=1**
estabelecido como precedente potencial.

**Decisões arquitecturais-chave**:
- **Status PROPOSTO deliberado** — promoção ocorre em passo
  futuro de materialização hayagriva real.
- **Estrutura canónica seguida** com paridade ADR-0023/0024/0057.
- **Plano de promoção documentado** em 8 passos.
- **Reuso de informação documentada** (DEBT-55, P152, P159A,
  P159B) sem duplicação.

**Política "sem novas reservas" preservada** — passo formaliza
reserva pré-existente, **não cria nova**. Reservas restantes
(ADR-0017, ADR-0063) mantêm-se documentadas mas não reforçadas.

**Padrões pós-ADR-0062-create**:
- Granularidade N=14 (inalterada — administrativo).
- Inventariar primeiro N=15 → **16** (critério #1 naming +
  #5 inventário trivial em passo administrativo).
- §análise risco N=15 → **16** (passo administrativo XS).
- **Hash content.rs preservado 11º passo consecutivo** (P156L
  → ADR-0062-create).

**ADR-0060 mantém `IMPLEMENTADO`**. **ADR-0061 mantém
`PROPOSTO`**. **ADR-0062 PROPOSTO** (era reserva sem ficheiro).

**Implicação concreta**: Bloco B do diagnóstico P159B desbloqueado
para iniciar com referência concreta a ADR-0062 (em vez de
reserva sem ficheiro).

**Pausa natural após ADR-0062-create — Bloco B desbloqueado;
política "sem novas reservas" preservada (formaliza pré-existente);
contagem ADRs cresce 63 → 64. Decisão humana sobre próxima
direcção tem máxima informação acumulada.**

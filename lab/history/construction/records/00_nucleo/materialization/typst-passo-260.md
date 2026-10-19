# Passo 260 — ADRs meta formalizando padrões "auditoria condicional" + "diagnóstico imutável" (administrativo XS)

**Data**: 2026-05-15
**Tipo**: passo arquitectural meta administrativo XS;
**não materializa código**.
**Análogo estrutural canónico**: **P156K** (criou ADR-0064 +
ADR-0065 simultaneamente, ambas `EM VIGOR`, formalizando
padrões N≥5 com zero reformulações).
**Pré-requisito leitura obrigatória**:
- `CLAUDE.md` (Protocolo de Nucleação).
- ADR-0034 (diagnóstico canónico; **a ser estendido** por
  ADR-0085 deste passo).
- ADR-0064 (precedente "Smart→Option"; formato canónico
  P156K).
- ADR-0065 (precedente "Inventariar primeiro"; formato
  canónico P156K).
- Relatórios precedentes: P255, P257, P258, P259 (evidência
  empírica das aplicações).

**Outputs canónicos esperados** ao fim do passo:
- `00_nucleo/adr/typst-adr-0084-auditoria-condicional.md`
  (status `EM VIGOR`; ficheiro novo).
- `00_nucleo/adr/typst-adr-0085-diagnostico-imutavel.md`
  (status `EM VIGOR`; ficheiro novo).
- `00_nucleo/adr/README.md` (entrada P260; distribuição
  actualizada; total 70 → 72).
- Relatório do passo em
  `00_nucleo/materialization/typst-passo-260-relatorio.md`.

---

## §0 — Princípios vinculativos para este passo

1. **Auto-aplicação P156K-style** — este passo cumpre ADR-0065
   critério #5 (inventariar primeiro): inventário das
   aplicações dos padrões antes de redigir as ADRs.
2. **Zero código alterado** — passo puramente documental.
3. **Tests workspace** preservados em **2334** (sem regressão;
   sem materialização).
4. **`crystalline-lint .`** zero violations (esperado:
   "Nothing to fix" pós-passo).
5. **Política "sem novas reservas" preservada** — passo
   formaliza padrões empíricos existentes, não cria reservas
   novas.
6. **Auditoria empírica antes da escrita** — cada ADR cita
   N concreto + lista literal de aplicações com passos +
   datas.

---

## §1 — Sub-passo P260.A: Inventário (auto-aplicação ADR-0065 critério #5)

**Objectivo**: documentar inventário literal das aplicações
de cada subpadrão antes de redigir as ADRs.

**Materialização**: zero código novo. Apenas leitura de
relatórios precedentes.

### Acções obrigatórias

#### A.1 — Inventário "Auditoria condicional"

Verificar contagem literal:

```bash
# Procurar aplicações cumulativas declaradas
grep -rn "auditoria condicional.*N=" 00_nucleo/materialization/
grep -rn "subpadrão.*auditoria condicional" 00_nucleo/adr/

# Confirmar 5 passos
ls 00_nucleo/materialization/typst-passo-192*.md \
   00_nucleo/materialization/typst-passo-255-relatorio.md \
   00_nucleo/materialization/typst-passo-257-relatorio.md \
   00_nucleo/materialization/typst-passo-258-relatorio.md \
   00_nucleo/materialization/typst-passo-259-relatorio.md
```

**Output esperado**: N=5 confirmado:
- N=1 P192A (audit M7 fixpoint estruturalmente fechado).
- N=2 P255 (audit DEBT-8 Math ENCERRADO).
- N=3 P257 (audit Color vanilla Fase A — 8/8 espaços).
- N=4 P258 (audit Model Fase A — cobertura 48% → 73%).
- N=5 P259 (audit Visualize Fase A — cobertura ~52% factual
  vs estimativa optimista pré).

#### A.2 — Inventário "Diagnóstico imutável precedente à acção"

```bash
# Diagnósticos imutáveis em 00_nucleo/diagnosticos/
ls 00_nucleo/diagnosticos/diagnostico-math-fase-a-passo-255.md \
   00_nucleo/diagnosticos/diagnostico-color-vanilla-passo-257.md \
   00_nucleo/diagnosticos/diagnostico-model-fase-a-passo-258.md \
   00_nucleo/diagnosticos/diagnostico-visualize-fase-a-passo-259.md

grep -l "Imutável.*ADR-0034\|imutável.*ADR-0034" \
  00_nucleo/diagnosticos/diagnostico-*-fase-a-*.md \
  00_nucleo/diagnosticos/diagnostico-*-vanilla-*.md
```

**Output esperado**: N=4 confirmado:
- N=1 P255 (`diagnostico-math-fase-a-passo-255.md`).
- N=2 P257 (`diagnostico-color-vanilla-passo-257.md`).
- N=3 P258 (`diagnostico-model-fase-a-passo-258.md`).
- N=4 P259 (`diagnostico-visualize-fase-a-passo-259.md`).

#### A.3 — Próximo número ADR livre

```bash
# Confirmar último número ADR usado
ls 00_nucleo/adr/typst-adr-*.md | sort | tail -5

# Confirmar slots livres
grep -E "^\| [0-9]{4}" 00_nucleo/adr/README.md | sort -k2 | tail -10
```

**Output esperado**: ADR-0083 último; **ADR-0084 e ADR-0085
livres** para este passo.

### Output exigido — registar inline

Inventário §A.1/A.2/A.3 fica inline no relatório §2 P260.A
(não exige ficheiro separado — magnitude trivial per ADR-0065
§"Neutras").

### Critério de aceitação P260.A

- Inventário N=5 + N=4 confirmado empíricamente.
- ADR-0084 + ADR-0085 confirmados livres.
- Zero ficheiros novos criados ainda (vem em P260.B/C).

---

## §2 — Sub-passo P260.B: Criar ADR-0084 "Auditoria condicional"

**Objectivo**: formalizar subpadrão "auditoria condicional" em
ADR `EM VIGOR` (paridade P156K).

### Estrutura canónica ADR-0084

Ficheiro novo
`00_nucleo/adr/typst-adr-0084-auditoria-condicional.md`:

```markdown
# ⚖️ ADR-0084: Auditoria condicional — audit empírico antes de decisão B1/B2/B3

**Status**: `EM VIGOR`
**Data**: 2026-05-15
**Autor**: Humano + IA
**Validado**: Passo 260 — formaliza padrão N=5 das aplicações
P192A/P255/P257/P258/P259.
**Diagnóstico prévio**: inline em P260.A (paridade P156K
auto-aplicação ADR-0065 critério #5).

---

## Contexto

Cinco passos consecutivos aplicaram o mesmo padrão metodológico
para decisões de scope sobre módulos com cobertura ambígua:

1. **Fase A empírica** — comandos `grep`/`view` produzem
   inventário literal do estado real (vs estado declarado em
   relatório/DEBT desactualizado).
2. **Decisão B1/B2/B3** — baseada em evidência factual:
   - **B1**: ≥75% cobertura → fecho conceptual (relatório
     documental).
   - **B2**: 55-70% → sub-passos prioritários materializáveis.
   - **B3**: ≤50% → re-classificação primeiro.
3. **Acções P260.B+** — actualização documental (L0 prompts
   obsoletos), eventual materialização condicional, fecho
   cumulativo.

**N=5 cumulativo** com zero reformulações:

| Passo | Módulo/alvo | Estado declarado pré | Estado factual pós | Cenário |
|-------|-------------|----------------------|---------------------|---------|
| P192A | M7 fixpoint runtime | ambíguo "intermédio" | estruturalmente fechado | B1 retroactivo |
| P255 | DEBT-8 Math (4 pendências) | "parcialmente resolvido" desde 2026-03 | 4/4 fechados cumulativamente | B1 fecho |
| P257 | Color paridade vanilla | 2/8 variants P25 | 8/8 estructural + 4 scope-outs ADR-0083 | Fase A literal |
| P258 | Model (22 entradas P154A) | ~48% declarado | ~73% empírico (Δ +25pp) | B1 fecho conceptual |
| P259 | Visualize (27 entradas) | ~60-65% estimativa optimista | ~52% factual (Δ -8 a -13pp) | B2 sub-passos adiados |

**Limiar formalização N=3-5 atingido** (paridade ADR-0064
N=6/ADR-0065 N=5/ADR-0080 N=9/ADR-0082 N=8/ADR-0064 N=8
saturação).

## Decisão

**Pattern "auditoria condicional" formalizado como metodologia
canónica para audits de módulo com cobertura ambígua**.

### Estrutura obrigatória

Para passos de audit empírico de módulo:

1. **Sub-passo `.A` Fase A audit** — checklist com blocos de
   comandos `grep`/`view` (mínimo 3 blocos; máximo conforme
   subsistemas).
2. **Output Fase A**: ficheiro imutável
   `diagnostico-<modulo>-fase-a-passo-NNN.md` em
   `00_nucleo/diagnosticos/` per **ADR-0085** (este passo).
3. **Decisão B1/B2/B3 explícita** em secção dedicada do
   diagnóstico imutável.
4. **Sub-passos `.B`/`.C`/`.D`** consequentes:
   - `.B` reconciliação documental L0 (paridade P258.B
     pattern "histórico cumulativo" ADR-0080 §"refactor
     aditivo").
   - `.C` materialização condicional (executa só se B2/B3).
   - `.D` fecho cumulativo + relatório.

### Critério "cobertura ambígua" (gatilho)

Quando aplicar audit empírico vs prosseguir sem audit:

| Sintoma | Acção |
|---------|-------|
| Número declarado >6 semanas sem actualização | Audit obrigatório |
| Materialização cumulativa entre declarações | Audit obrigatório |
| DEBT "parcialmente resolvido" sem actualização | Audit obrigatório |
| Resumo cita "~N%" sem referência cruzada | Audit recomendado |
| Modulo recém-implementado (<2 semanas) | Audit não-necessário |
| Cobertura estável documentada (saturação confirmada) | Audit não-necessário |

### Decisão Cenário B1 vs B2 vs B3

| Cobertura empírica | Cenário | Acção típica |
|--------------------|---------|---------------|
| ≥75% | B1 | Fecho conceptual; DEBT actualizada; relatório |
| 55-70% | B2 | 1-3 sub-passos prioritários; opções listadas |
| ≤50% | B3 | Re-classificação primeiro; sub-passos depois |

**Limiares são guidelines, não absolutos**. Decisão local
documenta justificação se desviar (precedente P259: 51.9%
literal → B2 vs 54.8% ponderado).

### Compatibilidade com ADR-0034 + ADR-0065

- **ADR-0034** (diagnóstico canónico): audit produz diagnóstico
  conforme estrutura canónica + secção "Decisão Fase B" nova.
- **ADR-0065** (inventariar primeiro): audit é forma específica
  de inventário (critério #5 — scope determinado).
- **ADR-0085** (diagnóstico imutável; este passo): ficheiro
  produzido por audit é imutável.

### Compatibilidade com política "sem novas reservas" (P158)

Audit empírico **revela** estado factual; **não cria reservas
novas**. Pendências descobertas registadas como achados
factuais no diagnóstico imutável, não como DEBTs/ADRs novas
(excepto se decisão arquitectural não-trivial justificar — e.g.
P257 ADR-0083 para Color scope-outs).

## Consequências

### Positivas

- **Reduz risco de obsolescência documental**: padrão exige
  audit periódico vs continuar a citar números antigos.
- **Acelera decisões pós-audit**: B1/B2/B3 fluxo pré-definido
  poupa tempo de re-justificação.
- **Cross-references explícitas**: cada audit produz
  diagnóstico imutável citável.
- **Honestidade documental**: descobertas factuais (subida ou
  descida de cobertura) registadas literais.

### Negativas

- Overhead documental por audit (típico ~30-45 min XS-S).
- Risco de audit superficial — mitigação: checklist com
  blocos `grep`/`view` literais.

### Neutras

- Padrão aplica-se preferencialmente a módulos com cobertura
  ambígua; aplicação a módulos novos é redundante.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Manter audits ad-hoc sem ADR | Menor overhead | Padrão N=5 sem rastreabilidade formal |
| Único ADR cobrindo "auditoria condicional" + "diagnóstico imutável" | Mais conciso | Mistura âmbitos (fluxo vs artefacto); dificulta citação |
| **ADR-0084 + ADR-0085 separadas (paridade P156K)** | **Foco preservado; citação precisa** | **+2 ADRs em vez de +1** |

**Decisão**: separação **ADR-0084 + ADR-0085** análoga a
P156K (que separou ADR-0064 Smart→Option de ADR-0065
Inventariar primeiro).

## Referências

- ADR-0034 — Diagnóstico canónico (estendido aqui).
- ADR-0065 — Inventariar primeiro (forma genérica).
- ADR-0085 — Diagnóstico imutável (artefacto produzido).
- ADR-0064 — Smart→Option (paridade formato canónico P156K).
- Aplicações:
  - P192A — `relatorio-passo-192a.md`.
  - P255 — `typst-passo-255-relatorio.md`.
  - P257 — `typst-passo-257-relatorio.md`.
  - P258 — `typst-passo-258-relatorio.md`.
  - P259 — `typst-passo-259-relatorio.md`.

---

## Auto-aplicação

P260 cumpre ADR-0065 critério #5 (inventário antes da decisão)
inline em P260.A — paridade auto-aplicação P156K.
```

### Critério de aceitação P260.B

- Ficheiro `typst-adr-0084-auditoria-condicional.md` criado.
- Status `EM VIGOR`.
- Tabela 5 aplicações com passos + módulos + estados.
- Critério "cobertura ambígua" documentado.
- Cross-references ADR-0034/0065/0085 explícitas.

---

## §3 — Sub-passo P260.C: Criar ADR-0085 "Diagnóstico imutável"

**Objectivo**: formalizar subpadrão "diagnóstico imutável
precedente à acção" em ADR `EM VIGOR`.

### Estrutura canónica ADR-0085

Ficheiro novo
`00_nucleo/adr/typst-adr-0085-diagnostico-imutavel.md`:

```markdown
# ⚖️ ADR-0085: Diagnóstico imutável — artefacto produzido por audit

**Status**: `EM VIGOR`
**Data**: 2026-05-15
**Autor**: Humano + IA
**Validado**: Passo 260 — formaliza padrão N=4 das aplicações
P255/P257/P258/P259.
**Estende**: ADR-0034 (diagnóstico canónico).

---

## Contexto

ADR-0034 (diagnóstico canónico) estabeleceu que diagnósticos
de tipos vanilla obrigatórios são imutáveis após criação.
Quatro passos consecutivos (P255/P257/P258/P259) produziram
**diagnósticos de audit Fase A** com a mesma propriedade —
ficheiros marcados explicitamente "Imutável após criação per
ADR-0034" em
`00_nucleo/diagnosticos/diagnostico-<modulo>-fase-a-passo-NNN.md`.

**N=4 cumulativo**:

| Passo | Ficheiro imutável | Função |
|-------|-------------------|--------|
| P255 | `diagnostico-math-fase-a-passo-255.md` | Audit DEBT-8 Math |
| P257 | `diagnostico-color-vanilla-passo-257.md` | Audit Color vanilla 8/8 |
| P258 | `diagnostico-model-fase-a-passo-258.md` | Audit Model 22 entradas |
| P259 | `diagnostico-visualize-fase-a-passo-259.md` | Audit Visualize 27 subsistemas |

**Limiar formalização N=3-4 atingido**. Próxima aplicação
audit cumprirá automaticamente.

## Decisão

**Diagnósticos de audit Fase A** (per ADR-0084) **são artefactos
imutáveis** em `00_nucleo/diagnosticos/` análogos a
diagnósticos de tipo vanilla (ADR-0034).

### Propriedades obrigatórias

1. **Localização canónica**: `00_nucleo/diagnosticos/`.
2. **Nome canónico**:
   `diagnostico-<modulo>-fase-a-passo-NNN.md` para audits
   estruturados (ADR-0084), ou
   `diagnostico-<tipo>-vanilla-passo-NNN.md` para audits
   de tipo vanilla (ADR-0029 §"Diagnosticar primeiro").
3. **Marcador explícito de imutabilidade**: "Imutável após
   criação per ADR-0034" (ou ADR-0085 a partir deste passo).
4. **Cabeçalho canónico**:
   - Data.
   - Executor (humano vs Claude Code).
   - Padrão (ADR-0034 / ADR-0084 / ADR-0065).
   - Diagnóstico pai (referência cruzada).
   - Análogo estrutural (passo precedente, se aplicável).
5. **Conteúdo literal**, não interpretativo (output `grep`/
   `view` colado; classificações em coluna separada da
   evidência).
6. **Tabelas estruturadas** (Tabela A entradas;
   Tabela B agregada).
7. **Secção "Decisão"** explícita (B1/B2/B3 conforme
   ADR-0084).

### Diferença vs diagnósticos transientes

| Tipo | Imutável? | Localização | Função |
|------|-----------|-------------|--------|
| Diagnóstico Fase A audit (ADR-0084) | **Sim** | `00_nucleo/diagnosticos/` | Evidência factual |
| Diagnóstico tipo vanilla (ADR-0029) | **Sim** | `00_nucleo/diagnosticos/` | Evidência tipo |
| Diagnóstico preparatório/planeamento | Não | `00_nucleo/diagnosticos/` | Pode ser actualizado |
| Inventário trivial inline | n/a | inline no passo | Auto-aplicação ADR-0065 |

### Distinção crítica vs ADR-0034

| Aspecto | ADR-0034 (precedente) | ADR-0085 (este passo) |
|---------|----------------------|------------------------|
| Âmbito | Tipo vanilla a materializar | Audit empírico de módulo |
| Gatilho | Novo tipo a entrar em L1 | Cobertura ambígua (ADR-0084) |
| Output | Estrutura/operadores tipo | Tabelas A/B + decisão B1/B2/B3 |
| Imutabilidade | Sim (per ADR-0034) | Sim (este ADR) |

ADR-0085 **estende** ADR-0034 cobrindo o novo âmbito (audit
empírico) sem alterar a regra original (tipo vanilla).

## Consequências

### Positivas

- **Evidência factual preservada**: histórico empírico não
  pode ser re-escrito retroactivamente.
- **Cross-references estáveis**: passos posteriores podem
  citar diagnósticos imutáveis com confiança.
- **Auditoria de processo**: futuro humano pode verificar se
  decisões pós-audit foram coerentes com evidência.

### Negativas

- **Sem actualização** se evidência factual mudar (audit
  futuro produz novo ficheiro com novo número).

### Neutras

- Diagnósticos preparatórios (não-imutáveis) podem coexistir
  na mesma directoria; distinção é por marcador explícito de
  imutabilidade.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Permitir actualização de diagnósticos Fase A | Mais flexível | Perde rastreabilidade; evidência factual pode ser revista |
| Revisão `-R1` de ADR-0034 em vez de ADR-0085 | Centraliza regras | Mistura âmbitos (tipo vanilla vs audit) |
| **Decisão adoptada: ADR-0085 nova estendendo ADR-0034** | **Foco preservado; novo âmbito tem ADR próprio** | **+1 ADR; aceitável dado padrão N=4** |

## Referências

- ADR-0034 — Diagnóstico canónico (estendido por este ADR).
- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório
  (cita diagnósticos imutáveis de tipo).
- ADR-0084 — Auditoria condicional (consumidor primário deste
  ADR).
- Aplicações:
  - P255 — `diagnostico-math-fase-a-passo-255.md`.
  - P257 — `diagnostico-color-vanilla-passo-257.md`.
  - P258 — `diagnostico-model-fase-a-passo-258.md`.
  - P259 — `diagnostico-visualize-fase-a-passo-259.md`.

---

## Auto-aplicação

P260 não produz diagnóstico Fase A imutável (passo
administrativo XS). Inventário §A inline no relatório cumpre
ADR-0065 critério #5 §"Neutras" (inline aceitável para
inventário trivial).
```

### Critério de aceitação P260.C

- Ficheiro `typst-adr-0085-diagnostico-imutavel.md` criado.
- Status `EM VIGOR`.
- Tabela 4 aplicações com passos + ficheiros.
- Propriedades obrigatórias listadas.
- Distinção vs ADR-0034 documentada.
- Cross-references ADR-0034/0029/0084 explícitas.

---

## §4 — Sub-passo P260.D: Actualizar README ADRs + relatório

### D.1 — Actualizar `00_nucleo/adr/README.md`

#### Tabela ADRs — adicionar 2 entradas

```markdown
| 0084 | Auditoria condicional — audit empírico antes de decisão B1/B2/B3 | `EM VIGOR` (P260; formaliza padrão N=5 dos audits P192A/P255/P257/P258/P259) |
| 0085 | Diagnóstico imutável — artefacto produzido por audit | `EM VIGOR` (P260; estende ADR-0034; formaliza padrão N=4 dos diagnósticos imutáveis P255/P257/P258/P259) |
```

#### Distribuição — actualizar contagens

```markdown
- `PROPOSTO`: 11 ADRs (inalterado).
- `EM VIGOR`: **30 → 32** ADRs (+0084, +0085).
- `IMPLEMENTADO`: 25 ADRs (inalterado).
- Total: **70 → 72** ADRs.
```

#### Passos-chave — adicionar P260

Entrada nova nos passos-chave com sumário (~30-50 linhas
descritivas; paridade entrada P156K):

```markdown
- **Passo 260 — ADRs meta formalizando padrões "auditoria
  condicional" + "diagnóstico imutável" cumulativos**
  (passo arquitectural meta administrativo XS; análogo
  estrutural canónico P156K). Dois ADRs novos `EM VIGOR`:
  **ADR-0084** (auditoria condicional — formaliza padrão N=5
  P192A/P255/P257/P258/P259; documenta critério cobertura
  ambígua e fluxo B1/B2/B3) e **ADR-0085** (diagnóstico
  imutável — estende ADR-0034 para audits Fase A; formaliza
  N=4 P255/P257/P258/P259). Auto-aplicação: P260 cumpre
  ADR-0065 critério #5 (inventário inline §A.1/A.2). Numeração
  escolhida: 0084 + 0085 (consecutivos; próximo livre pós
  ADR-0083 P257). Contagens: total 70 → **72** ADRs; EM VIGOR
  30 → **32**. Zero código alterado; tests preservados 2334;
  `crystalline-lint` zero violations (esperado "Nothing to
  fix"). **Benefício**: sessões futuras citam ADRs explicitamente
  em vez de re-justificar empiricamente cada audit — reduz
  overhead e garante rastreabilidade formal dos padrões
  consolidados. **Subpadrão emergente N=3 → cresce N=4**
  "passo administrativo XS criar/promover ADR" (ADR-0062-create
  + P156K + P160A + P260; atinge limiar formalização N=3-4
  consolidado).
```

### D.2 — Relatório do passo

`00_nucleo/materialization/typst-passo-260-relatorio.md`
estrutura canónica:

- **§1 Sumário executivo** — ADRs criadas (2); status (`EM
  VIGOR`); tests delta (zero); ADRs distribuição (70 → 72).
- **§2 Sub-passo P260.A** — inventário inline N=5 + N=4.
- **§3 Sub-passo P260.B** — ADR-0084 criada (sumário).
- **§4 Sub-passo P260.C** — ADR-0085 criada (sumário).
- **§5 Sub-passo P260.D** — README ADRs actualizado.
- **§6 Padrões metodológicos** — auto-aplicação P156K-style;
  subpadrão "passo administrativo XS criar ADR" cresce
  N=3 → 4.
- **§7 Cobertura** — inalterada (passo administrativo).
- **§8 Limitações e trabalho futuro** — padrões formalizados;
  audits futuros (Text P261? Layout final?) consumirão
  ADR-0084/0085.
- **§9 Critério de aceitação global P260 — Checklist final**.
- **§10 Referências**.

### Critério de aceitação P260.D

- README ADRs reflecte estado pós-passo.
- Relatório criado em `00_nucleo/materialization/`.
- Cross-references coerentes.

---

## §5 — Critério de aceitação global P260

- [ ] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found` (esperado "Nothing to fix" — passo
  administrativo).
- [ ] `cargo test --workspace` retorna **2334 verdes
  preservado** (sem regressão).
- [ ] `typst-adr-0084-auditoria-condicional.md` criado;
  status `EM VIGOR`.
- [ ] `typst-adr-0085-diagnostico-imutavel.md` criado; status
  `EM VIGOR`.
- [ ] `README.md` ADRs actualizado (distribuição 70 → 72;
  entrada P260).
- [ ] Relatório criado em `00_nucleo/materialization/`.
- [ ] Zero código L1/L2/L3/L4 tocado.
- [ ] Zero prompts L0 editados.
- [ ] Zero DEBTs criados/encerrados.
- [ ] Política "sem novas reservas" preservada (formaliza
  padrões empíricos; não cria reservas).

---

## §6 — Sequência operacional condensada

1. **Ler** `CLAUDE.md`, ADR-0034, ADR-0064 (formato canónico
   P156K), ADR-0065 (auto-aplicação), relatórios P255/P257/
   P258/P259 (evidência empírica).
2. **Reportar** estado inicial: tests 2334 + lint baseline +
   ADRs 70.
3. **P260.A** — Executar comandos inventário; confirmar N=5
   + N=4; confirmar slots 0084/0085 livres.
4. **P260.B** — Criar ADR-0084 conforme estrutura §2.
5. **P260.C** — Criar ADR-0085 conforme estrutura §3.
6. **P260.D** — Actualizar README ADRs; criar relatório.
7. **Verificação final**:
   - `crystalline-lint .` zero violations.
   - `cargo test` 2334 preservado.
   - Checklist §5 satisfeito.
8. **Reportar** ao utilizador: ADRs criadas, distribuição
   actualizada, recomendação P261+ pós-P260.

---

## §7 — Política de paragem

Claude Code **deve parar e perguntar ao utilizador** se:

- P260.A revela contagem diferente (e.g. N=4 cumulativa para
  "auditoria condicional" porque P192A não conta) — ajustar
  ADR-0084 conforme.
- P260.A revela que slot ADR-0084 ou ADR-0085 já está
  ocupado por outro tópico (descoberta crítica análoga P160A
  ADR-0017) — re-numerar.
- Decisão de granularidade ADR: separar ADR-0084 + ADR-0085
  vs ADR única — confirmar precedente P156K.
- Estado dos padrões diverge significativamente do declarado
  (e.g. um dos relatórios P255-P259 não cita o padrão
  explicitamente — N efectivo menor).
- `crystalline-lint` reporta violations não-triviais.

Em qualquer paragem, registar contexto e aguardar instrução.

---

## §8 — Notas estratégicas

### Análogo canónico P156K

P156K é o **template literal** deste passo:

| Aspecto | P156K | P260 |
|---------|-------|------|
| Tipo | Administrativo XS | Administrativo XS |
| ADRs criadas | ADR-0064 + ADR-0065 | ADR-0084 + ADR-0085 |
| Ambos status | `EM VIGOR` | `EM VIGOR` |
| Padrão formalizado N | 6 (Smart→Option) + 5 (Inventariar) | 5 (Auditoria condicional) + 4 (Diagnóstico imutável) |
| Auto-aplicação | Sim | Sim |
| Código alterado | Zero | Zero |
| Tests delta | Zero | Zero |
| Magnitude | XS | XS |

### Subpadrão "passo administrativo XS criar/promover ADR"

Cumulativo:
- N=1 ADR-0062-create (2026-04-27).
- N=2 P156K (criou 0064+0065 simultaneamente; 2026-04-26).
- N=3 P160A (criou ADR-0066 PROPOSTO; 2026-04-30).
- **N=4 P260** (cria ADR-0084+0085 simultaneamente; este passo).

**Patamar N=4 atinge limiar formalização consolidado** (N=3-4
política). Candidato a meta-meta-ADR futuro (improvável e
desnecessário — padrão é auto-documentado em cada aplicação).

### Pós-P260 — sequência recomendada

Audits futuros (Text P261? Layout final P261'?) consumirão
ADR-0084/0085 automaticamente — overhead reduzido. Cada novo
audit cresce N=5→N=6 (Auditoria condicional) e N=4→N=5
(Diagnóstico imutável) sem renegociação metodológica.

---

## §9 — Referências

- `CLAUDE.md` — Protocolo de Nucleação.
- ADR-0034 — Diagnóstico canónico (estendido por ADR-0085).
- ADR-0064, ADR-0065 — Precedente canónico P156K (template
  formato).
- ADR-0083 — Color paridade vanilla (último ADR pré-P260).
- Relatórios das aplicações:
  - P192A — auditoria M7.
  - P255 — DEBT-8 Math (precedente "auditoria condicional"
    N=2).
  - P257 — Color paridade vanilla.
  - P258 — Model fecho conceptual.
  - P259 — Visualize Fase A.
- P156K — precedente "passo administrativo XS criar ADRs
  meta simultaneamente".
- P160A — precedente "passo administrativo XS criar ADR
  PROPOSTO".
- ADR-0062-create — N=1 precedente "passo administrativo XS
  criar ADR".

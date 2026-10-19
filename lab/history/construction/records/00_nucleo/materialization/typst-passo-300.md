# Passo 300 — Retrospectivo metodológico (qualitativamente distinto)

**Frente**: Consolidação metodológica P283-P299 (18 passos).
**Origem**: P299 §9 sugeriu: *"P300 é um marco numérico — pode
ser oportunidade para consolidação dos padrões cumulativos sem
materialização nova (retrospectiva metodológica)."*
**Pré-requisitos**: nenhum (passo consolidação).
**Tipo declarado**: **qualitativamente distinto** — 1ª spec da
sequência P283+ sem materialização de feature concreta.
**Magnitude**: variável (depende de §3 plano de consolidação;
0 features novas).
**Marco**: P300 número redondo, mas critério de fecho **factual**
(P273.17 §0 — número redondo não é critério metodológico).

---

## §1 — Objectivo e enquadramento epistémico

### §1.1 — Honestidade sobre o risco

P300 retrospectivo tem **valor único genuíno** (consolidação de
padrões cumulativos acumulados) **e risco real** (promoção
forçada por número redondo).

**Critério estrito anti-inflação**: P300 promove ADR meta **apenas
se gatilho cumulativo dispara inequivocamente** com base em
evidência factual acumulada P283-P299, **não** porque é número
redondo. **Promoção é caminho condicional, não obrigatório**.

**Alternativas legítimas para P300**:

| Caminho | Descrição | Promoção ADR meta |
|---|---|---|
| **A — Consolidação documental pura** | Actualizar tabelas; registar lições; fechar pendências documentais; sem promoção | 0 promoções |
| **B — Promoção §8.7' robusta** | §8.7' N=7 com 7 magnitudes documentadas dispara genuinamente | 1 promoção (§8.7') |
| **C — Promoção §8.3 acumulada** | §8.3 N=11 candidato cumulativo desde P294 | 1 promoção (§8.3) |
| **D — Múltiplas promoções** | §8.7' + §8.3 simultâneas | **REJEITADA** (P273.17 §0; uma por passo) |
| **E — Auditoria sem decisão** | Listar candidatos com evidência; deferir decisão a passo posterior | 0 promoções |

Default sugerido: **B** se Fase A confirma §8.7' inequívoco;
**A** se Fase A revela que mesmo §8.7' tem ambiguidade residual.
**E** se ambiguidade significativa em ambos.

### §1.2 — Por que P300 (não P298 ou P301)

P300 retrospectivo justifica-se **se e só se** existem condições
factuais cumulativas:

- **§8.7' N=7 acumulado** — 7 reaplicações com magnitudes variadas
  P293-P299; documentação cumulativa robusta.
- **§8.3 N=11 acumulado** — refutações cumulativas P294-P299.
- **Lições metodológicas formalizáveis** — "confirmação esperada"
  (P295 §10; P297 §A.5'), "flutuação saudável" (P299 §10),
  "subdivision decision A.0.0'" (P299 §A.0.0').

**Não é número redondo**. **É evidência cumulativa**.

Confirmação: estado pós-P298 também tinha 3 candidatos adiados
(§8.7' N=6, "cluster math handler" N=3 ambíguo, "cross-variant"
N=1) — **nenhum era inequívoco**. P298 §6.8 honrou anti-padrão
adiando todos. **Mesma análise aplicada a P300**: promove apenas
se inequívoco.

### §1.3 — Razões

1. **Consolidar 8 padrões cumulativos adiados** sem promoção
   forçada.
2. **Avaliar §8.7' N=7** com critério estrito — 1ª oportunidade
   após 6 reaplicações adiadas (P293 inaugural + P294-P299
   adiamentos).
3. **Documentar lições metodológicas** emergentes da série
   P283-P299:
   - "Confirmação esperada" categoria honesta (P295 §10).
   - "Flutuação saudável" magnitude A.0.0 (P299 §10).
   - "Subdivision decision A.0.0'" (P299).
   - "Cross-variant interaction" (P298).
   - "Module namespaced via SSoT" (P299).
4. **Fechar pendências documentais** — Tabelas, hashes L0
   propagados, frentes pendentes catalogadas.
5. **Reset arquitectural** antes de P301+ continuar com
   materializações.

**Não** promover ADR meta como objectivo per se. Promoção é
caminho B/C condicional **se** evidência inequívoca.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; secções adaptadas)

P300 não materializa feature; A.0.0 inspecciona **estado cumulativo
de padrões**, não código. Estrutura adaptada:

### A.0.0 — Inventário literal padrões cumulativos (N=8 cumulativo §8.7')

Para cada padrão pendente, inspecção literal:

| Padrão | N reclamado | Verificações |
|---|---|---|
| §8.7' A.0.0 template | 7 | Listar passos P293-P299 + magnitudes registadas em relatórios |
| §8.3 refutação pragmática | 11 | Listar refutações P294-P299 + magnitudes |
| §8.6 A.5' anti-reflexão | 9 | Listar reaplicações P291-P299 |
| "Variant rico" N=5 | 1 P297 candidato | Verificar P297 §6.4 critério estrutural |
| Sub-padrão "cluster math handler" | 3 P296-P298 | Avaliar qualidade do 3.º caso (P298 trivial) |
| Sub-padrão "cross-variant interaction" | 1 P298 | Inaugural |
| Sub-padrão "module namespaced" | 2 P283+P299 | Verificar paridade arquitectural |
| Sub-padrão "operadores SSoT" | 1 P299 | Inaugural |

**Para cada padrão, A.0.0 produz veredicto factual**:
- **Inequívoco**: evidência cumulativa documentada robusta;
  promoção candidata defensável.
- **Ambíguo**: evidência mista; qualidade questionável.
- **Insuficiente**: N abaixo de limiar; aguardar.

### A.0.0' — Decisão de caminho A/B/C/D/E (paralelo P299 subdivision)

Após A.0.0 produzir veredictos, decidir caminho P300:

| Cenário A.0.0 | Caminho |
|---|---|
| §8.7' inequívoco | **B** — promover §8.7' |
| §8.7' ambíguo + §8.3 inequívoco | **C** — promover §8.3 |
| Ambos ambíguos | **E** — auditoria sem decisão |
| Ambos inequívocos | **D rejeitada** → **escolher um** (preferir §8.7' — mais cumulativa) |
| Nenhum candidato | **A** — consolidação documental pura |

**Decisão genuína condicional**. Spec **aceita não saber** qual
caminho até A.0.0 + A.0.0' concluírem.

### A.0 — Aplicação ADR-0098 (paralelo passos materializadores)

| Verificação | Esperado |
|---|---|
| Hash `export.rs 66cb8ac3` esperado pós-P300 | **Preservado bit-exact** — P300 não toca código de produção |
| Hash `content.rs` esperado | **Preservado** — sem variants novos |
| Hashes L0 esperados | **Preserved** salvo `00_nucleo/adrs/index.md` se promoção (caminho B/C) |

P300 deve preservar hash `export.rs` pelo **17º passo
consecutivo** se for caminho A/E (sem promoção) ou caminho B/C
(promoção sem alterar export).

### A.1 — Inventário documental

Sub-secções:

1. **A.1.1 — Tabelas cobertura actuais** — listar literalmente
   todas as linhas adicionadas/modificadas P283-P299 (~17 entries).
2. **A.1.2 — Hashes L0 actuais** — listar pós-P299.
3. **A.1.3 — ADRs vigentes** — listar (84 + ADR-0098 + ADR-0099
   = 86 pós-P289).
4. **A.1.4 — Frentes pendentes catalogadas** — todas as P*.X
   registadas P283-P299:
   - P295.1 (nota rodapé) + P295.2 (overflow) + P295.X (footnote.entry).
   - P296.X (toggles cancel) + P297.X (UnderoverKind) + P298.X
     (resolvida P299).
   - Auto-lookup math mode (P299 §8).
   - P-curve-scope-methods (P293/P294 §9).
   - P-native-path-svg (P293 §9).
   - P-style-tracking/leading/font (P-style-*-variant resolvidas
     P290-P292).
   - Cosméticos ADR-0054 graded (diversos).
5. **A.1.5 — Bugs latentes descobertos** — NBSP (P287 §A.0.0
   bug colateral via P288).
6. **A.1.6 — Lições metodológicas emergentes** — listadas no §1.3.
7. **A.1.7 — Sequência cumulativa** — gráfico (texto) de
   magnitudes A.0.0 P293-P299.
8. **A.1.8 — Diagrama cumulativo** — produzir genuinamente.

### A.2 — Decisão sobre promoção (per A.0.0 + A.0.0')

Per caminho escolhido:

- **A**: zero promoções; só actualização documental.
- **B**: §8.7' formalizada como **ADR-0100** (numeração propositada
  — coincide marco numérico, mas crítério é mérito factual). Conteúdo:
  - Padrão "Inventário Fase A.0.0 obrigatório antes de
    materialização".
  - 7 aplicações cumulativas P293-P299.
  - Lições "confirmação esperada", "flutuação saudável",
    "subdivision decision".
  - Status `IMPLEMENTADO`.
- **C**: §8.3 formalizada análoga (ADR-0101 ou ADR-0100
  conforme decisão).
- **E**: zero promoções; nota explícita "ambíguo; reavaliar
  P301+".

### A.3 — Integração documental

Per caminho:

- **A/E**: actualizar Tabelas A/B/C cumulativamente; propagar
  hashes L0; **sem** alteração estrutural.
- **B/C**: adicionar ADR nova + entry em `00_nucleo/adrs/index.md`;
  hashes propagados.

### A.4 — Impacto em hashes (ADR-0098 vigente)

| Caminho | export.rs | content.rs | adrs/index.md |
|---|---|---|---|
| A/E (sem promoção) | preserved | preserved | preserved |
| B/C (promoção) | preserved | preserved | **muda** (+1 entry) |

**Hash `export.rs` preservado em ambos os cenários** → 17º passo
consecutivo P282-P300; ADR-0098 N=17 cumulativo.

### A.5 — Detecção de bugs latentes documentais

Cenários a verificar:

- Tabelas A/B/C **consistentes entre si** — e.g. Tabela A.4
  linha 119 pós-P298 lista 3 features `implementado`; Tabela B
  Math variants tem 13 entries; consistente?
- Hashes L0 propagados em **todos** os ficheiros relevantes —
  sem drift?
- Frentes pendentes **sem duplicação** entre relatórios
  P283-P299?
- ADRs documentadas consistentes com nº ADRs vigentes (84 + 0098
  + 0099 = 86 esperado)?

### A.5' — Verificação anti-reflexão (N=10 do padrão §8.6)

**8ª reaplicação A.0.0** consecutiva. P300 testa se template
funciona para **passos não-materializadores** (paradigma novo).

4 verificações:

1. **Comparação literal A.1.6 P288-P300** — paradigma novo?
   - P293-P299 nove paradigmas materializadores distintos.
   - **P300**: paradigma "**retrospectiva sem materialização**"
     — primeira spec não-materializadora desde P283. **Genuinamente
     novo** estructuralmente; não é continuação dos paradigmas
     anteriores.
2. **A.0.0 produzido empiricamente** — magnitude do inventário
   factual dos padrões cumulativos.
3. **Elementos estructuralmente novos identificados**:
   - **Passo não-materializador** — primeiro pós-série P283.
   - **A.0.0 sobre estado cumulativo** vs sobre código.
   - **A.0.0' decisão caminho A/B/C/D/E** — inaugural.
   - **Promoção ADR meta condicional a A.0.0 + A.0.0'** — não
     a priori.
4. **Decisão sobre promoção ADR meta**:
   - **Per A.0.0 + A.0.0'**. Não decidir a priori.
   - **Critério estrito anti-inflação**: promover apenas se
     evidência inequívoca **factualmente**, não por marco
     numérico P300.

---

## §3 — Materialização (per caminho)

### §3.A — Caminho A (consolidação documental pura)

1. Actualizar Tabelas A/B/C com referências cumulativas P283-P299.
2. Propagar hashes L0 onde drift detectado em A.5.
3. Documentar lições metodológicas em
   `00_nucleo/diagnosticos/retrospectivo-p283-p299.md`.
4. Catalogar frentes pendentes em
   `00_nucleo/diagnosticos/frentes-pendentes-pos-p299.md`.
5. **Zero ADRs novas**.

Estimativa: trabalho documental ~1-3h; sem código.

### §3.B — Caminho B (§8.7' promovida)

Tudo de §3.A + adicionalmente:

1. Criar `00_nucleo/adrs/ADR-0100.md` com conteúdo:
   - Título: "A.0.0 inventário Fase A obrigatório antes de
     materialização".
   - Contexto: P273.17 §0 anti-padrão; P293-P299 sequência
     cumulativa.
   - Decisão: A.0.0 obrigatória em todas as Fases A futuras.
   - Consequências: 7 aplicações cumulativas demonstram robustez.
   - Alternativas consideradas: ad-hoc inspecção por passo (mais
     frágil).
   - Status: `IMPLEMENTADO`.
2. Actualizar `00_nucleo/adrs/index.md` (hash muda).
3. Actualizar `00_nucleo/CLAUDE.md` se Protocolo Nucleação
   menciona ADR-0065 — adicionar referência cruzada ADR-0100.

### §3.C — Caminho C (§8.3 promovida)

Análogo §3.B mas para "refutação pragmática":

1. Criar `00_nucleo/adrs/ADR-010X.md`:
   - Título: "Refutação pragmática como padrão epistémico".
   - Contexto: 11 refutações cumulativas P294-P299.
   - Status: `IMPLEMENTADO`.
2. Actualizar index.

### §3.D — Caminho D — REJEITADA

Múltiplas promoções viola P273.17 §0. Não materializável.

### §3.E — Caminho E (auditoria sem decisão)

1. Tudo de §3.A + adicionalmente:
2. Documento explícito em
   `retrospectivo-p283-p299.md` regista veredictos ambíguos
   por padrão; agenda reavaliação P301+.
3. **Zero ADRs novas**.

### §3 — Aplicação ADR-0098

Per caminho:
- A/E: hash `export.rs` preserved → ADR-0098 N=17.
- B/C: hash `export.rs` preserved → ADR-0098 N=17.

**Em todos os cenários**, hash `export.rs` preservado bit-exact
pelo **17º passo consecutivo** P282-P300.

### §3 — Testes

P300 não materializa features → **zero testes novos**.

Testes existentes devem continuar verdes (regression).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P299: 2 862 testes.
  Esperado: **2 862 inalterado** (sem features novas).
- `crystalline-lint` zero violations.
- Hash L0 `content.md` **preserved** (zero variants novos).
- Hash L0 `stdlib.md` **preserved** (zero funcs novas).
- Hash L0 `export.rs` **preserved bit-exact** pelo 17º passo
  consecutivo.
- Hash L0 `adrs/index.md` **condicional**:
  - A/E: preserved.
  - B/C: muda (+1 ADR entry).
- **A.0.0 + A.0.0' decisão registada** em diagnóstico.
- Documento retrospectivo
  `retrospectivo-p283-p299.md` produzido (qualquer caminho).
- **Promoção ADR meta condicional**:
  - **§8.7' N=7 → ADR-010X** se inequívoco (caminho B).
  - **§8.3 N=11 → ADR-010X** se inequívoco (caminho C).
  - **Zero promoções** se A/E (caminho preferencial conservador).
- Frentes pendentes catalogadas em ficheiro dedicado.
- Lições metodológicas documentadas.

---

## §5 — Não-objectivos

- **Não** materializar features novas. P300 é qualitativamente
  distinto.
- **Não** promover ADR meta **por causa do número redondo P300**.
  Critério estrito: evidência cumulativa factual inequívoca.
- **Não** promover múltiplas ADRs meta. Caminho D rejeitado
  per P273.17 §0.
- **Não** documentar lições especulativas. Só lições com
  evidência empírica P283-P299.
- **Não** modificar código de produção (`01_core/`, `03_infra/`).
  Trabalho 100% documental.
- **Não** confundir P300 com "fecho da série P283+". Série
  continua P301+ com frentes pendentes catalogadas. P300 é
  **consolidação intermédia**, não fecho.
- **Não** tentar fechar todas as frentes pendentes em P300.
  Catalogar é diferente de resolver.

---

## §6 — Pendências relacionadas

Resolve (per caminho):
- Catalogação cumulativa de pendências P283-P299.
- Avaliação cumulativa de padrões adiados.
- **Condicional**: promoção §8.7' (B) ou §8.3 (C) se inequívoco.

Não resolve:
- Todas as frentes pendentes materializadoras (P295.1, P296.X,
  P297.X, auto-lookup math, etc.) — catalogadas mas adiadas.
- Padrões cumulativos ambíguos — adiados para passos posteriores
  com mais evidência.
- Cosméticos ADR-0054 graded — fora de scope.

---

## §7 — Risco residual

Risco principal: **promoção forçada por número redondo P300**.
Mitigação: §1.1 + §5 explicitam critério estrito; caminho A/E
sempre disponível se evidência ambígua.

Risco secundário: **A.0.0' caminho B/C escolhido sem evidência
inequívoca**. Mitigação: §A.0.0' critério explícito; default
conservador A/E.

Risco terciário: **retrospectiva degenera em celebração
arquitectural**. Mitigação: §5 não-objectivo "lições especulativas"
proibidas; documentação factual obrigatória.

Risco quaternário: **promoção ADR-0100 §8.7' mal-formulada** —
ADR demasiado vaga ou demasiado prescritiva. Mitigação: usar
linguagem factual P283-P299 directa; 7 aplicações cumulativas
literais como evidência.

Risco quinário: **P300 retrospectivo cria expectativa de
retrospectivos periódicos** (P310, P320, etc.). Mitigação: §5
não-objectivo — P300 não estabelece cadência fixa; futuros
retrospectivos justificam-se por evidência cumulativa real,
não por marcos numéricos.

Risco senário: **fragmentação de frentes pendentes documentadas**.
Mitigação: A.1.4 catalogação completa em ficheiro único.

Risco septenário: **lição metodológica registada como pré-empção
de futuros passos** — e.g. "A.0.0 obrigatório" formaliza algo
que era flexível. Mitigação: ADR proposta inclui "A.0.0
obrigatório **quando feature tem ambiguidade arquitectural
genuína**" (paralelo ADR-0065 critérios); não é rule absoluta.

---

## §8 — Ponteiros

- Lições documentadas previamente: P295 §10 ("confirmação
  esperada"), P297 §A.5' ("variant rico" critério Option),
  P298 §6.8 (4 candidatos avaliados/adiados), P299 §10
  ("flutuação saudável"; "subdivision decision").
- Relatórios P283-P299 (17 documentos cumulativos).
- Diagnósticos Fase A (17 documentos cumulativos).
- Tabelas A/B/C em
  `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- ADRs vigentes: 84 base + ADR-0098 (P288) + ADR-0099 (P289)
  = 86 esperado.
- Hashes L0 actuais — verificar em A.1.2.
- ADR processual: ADR-0065 (inventariar-primeiro).
- ADR cultural: **P273.17 §0** (anti-padrão over-formalização) —
  **fundamental P300**.
- Padrão §8.7' candidato — 7 aplicações documentadas.
- Padrão §8.3 candidato — 11 refutações documentadas.

---

*Spec P300 produzida 2026-05-19 pós-P299 (operadores math 42
pré-definidos via SSoT MathOp). **Qualitativamente distinta**:
1ª spec não-materializadora desde P283; **retrospectivo
metodológico**. **Marco numérico P300 NÃO é critério metodológico**
— critério é evidência cumulativa factual inequívoca P283-P299.
Fase A obrigatória com secções adaptadas: A.0.0 (inventário
padrões cumulativos) + **A.0.0' (decisão caminho A/B/C/D/E,
inaugural)** + A.0-A.5 + A.5' (N=10 padrão §8.6 acumulativo).
**5 caminhos disponíveis**: A consolidação documental pura
(zero promoções); B promoção §8.7' robusta (se inequívoco;
ADR-010X); C promoção §8.3 acumulada (se inequívoco); D
rejeitada (P273.17 §0); E auditoria sem decisão. **Default
conservador A/E** — promoção só se gatilho **inequivocamente**
dispara, não por número redondo. **Hash `export.rs` preservado
bit-exact pelo 17º passo consecutivo** em qualquer cenário.
**Zero testes novos** (sem features). **Sem caps** (P282 §7).
**Honestidade epistémica reforçada**: P300 aceita explicitamente
o risco de inflação artificial; §1.1 + §5 + §7 múltiplos
contingências contra promoção forçada. Após P300, série continua
P301+ com frentes pendentes catalogadas; P300 é **consolidação
intermédia**, não fecho.*

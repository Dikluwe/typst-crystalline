# Relatório — Passo 300 (Retrospectivo metodológico)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-300.md`
**Diagnóstico Fase A**: `00_nucleo/diagnosticos/diagnostico-retrospectivo-passo-300.md`
**Tipo declarado spec**: qualitativamente distinto — 1.ª spec
não-materializadora desde P283.
**Caminho adoptado**: **A (consolidação documental pura)** com nota
explícita E sobre candidatos §8.7' maduro adiado.
**Baseline P299**: 2 862 testes  →  **P300**: 2 862 testes (Δ = 0)
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**17º passo
consecutivo**: P282→P300)
**Hash `content.rs`**: `82d3c47d` inalterado (zero variants novos)
**ADRs meta novas**: 0

---

## §1 — Sumário executivo

P300 é **qualitativamente distinto** dos 17 passos materializadores
P283-P299. Spec definiu 5 caminhos disponíveis (A/B/C/D/E); A.0.0
+ A.0.0' produziram a decisão fundamentada:

**Caminho A — consolidação documental pura**, com auditoria
explícita de 8 padrões cumulativos:

| Padrão | N | Veredicto P300 |
|---|---:|---|
| §8.7' A.0.0 template | 7 | **maduro mas ambíguo** — adiado conservador |
| §8.3 refutação pragmática | 11 | **robusto mas descritivo** — adiado |
| §8.6 A.5' anti-reflexão | 9 | instrumental interno — não promove |
| "Variant rico" Option estrutural | 1 genuíno (P297) | insuficiente |
| "Cluster math handler" | 3 ambíguo | qualidade questionável — adiado |
| "Cross-variant interaction" | 1 inaugural (P298) | insuficiente |
| "Module namespaced" | 2 (P283+P299) | insuficiente |
| "Operadores SSoT" | 1 inaugural (P299) | insuficiente |

**0 promoções ADR meta**. Anti-padrão P273.17 §0 honrado pela
**8.ª vez consecutiva** (P293-P300).

**Resultado documental**: 3 documentos novos produzidos (diagnóstico
Fase A + retrospectivo metodológico cumulativo + catálogo único de
frentes pendentes).

**Resultado epistémico — caminho A não foi fuga**: §8.7' N=7
candidato genuinamente maduro com valor empírico documentado em
P294/P297/P298. Decisão de adiamento é **explicitamente
conservadora**, não dúvida sobre o padrão. Formalização traria
overhead (rigidez vs discrição) sem ganho marginal claro;
preservação intencional da flexibilidade.

**Risco septenário spec realizou-se hipoteticamente e foi evitado**:
ADR "A.0.0 obrigatório" teria formalizado como regra absoluta o
que era prática discricionária per-passo.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=8 reaplica §8.7') | **Inventário** dos 8 padrões cumulativos — categoria nova (não refutação) |
| A.0.0' (decisão caminho) | **A escolhido honestamente** — nenhum padrão dispara inequivocamente |
| A.0 (ADR-0098) | ✅ preservado bit-exact (17º passo) |
| A.1 inventário documental | Tabelas/hashes/ADRs/frentes/bugs/lições/sequência |
| A.2 promoção | **0 ADRs meta** — caminho A confirmado |
| A.3 integração documental | 3 documentos novos; sem alteração código |
| A.4 hashes | **17º passo consecutivo** export.rs preservado |
| A.5 bugs latentes documentais | Tabelas consistentes; sem drift; sem duplicação |
| A.5' anti-reflexão | **N=10 cumulativo** (P291-P300); paradigma "retrospectiva sem materialização" inaugural |

Detalhe completo: `00_nucleo/diagnosticos/diagnostico-retrospectivo-passo-300.md`.

---

## §3 — Materialização (caminho A documental)

### §3.1 — Documentos novos produzidos

**`00_nucleo/diagnosticos/diagnostico-retrospectivo-passo-300.md`**:

Fase A completa com:
- A.0.0 inventário literal dos 8 padrões.
- A.0.0' decisão caminho inaugural (precedente P299 subdivision
  decision).
- Tabelas magnitudes A.0.0 P293-P299 com gráfico texto.
- 11 paradigmas distintos identificados na sequência.

**`00_nucleo/diagnosticos/retrospectivo-p283-p299.md`**:

Documento metodológico cumulativo (~270 linhas) com 8 secções:
1. Sumário cumulativo (17 passos materializadores + P300).
2. Lições metodológicas emergentes (7 lições documentadas).
3. Padrões cumulativos pós-P300 (8 padrões com estado).
4. Bugs latentes descobertos (NBSP P287→P290).
5. ADR-0098 N=17 cumulativo com tabela 17 features.
6. ADR-0099 N=16 cumulativo.
7. Decisão final caminho A com justificação.
8. Próximos passos.

**`00_nucleo/diagnosticos/frentes-pendentes-pos-p299.md`**:

Catálogo único (~250 linhas) com 11 secções:
1. Frentes derivadas (footnote/curve/cluster math).
2. Cosméticos ADR-0054 graded por variant.
3. Frentes Style/Text (resolvidas P288-P292 + refinos pendentes).
4. Frentes em Tabela A.6 (Model).
5. Frentes Visualize.
6. Frentes Math não cobertas P296-P299.
7. Frentes Foundations/Layout.
8. Sub-padrões cumulativos preservados como ferramentas.
9. Bugs latentes monitorizados.
10. Categorias arquitecturais bloqueadas (4 categorias).
11. Total agregado + prioridades sugeridas (~30 frentes; ~10
   principais).

### §3.2 — Zero alterações em código de produção

| Componente | Pós-P300 |
|---|---|
| `01_core/src/entities/content.rs` | **Inalterado** — hash `82d3c47d` preservado |
| `01_core/src/engine/*` | **Inalterado** |
| `03_infra/src/export.rs` | **Inalterado bit-exact** — hash `66cb8ac3` (17º passo) |
| `02_shell/`, `04_wiring/` | **Intactos** |
| L0 markdown | **Inalterados** |

### §3.3 — Zero alterações em ADRs

Caminho A confirmado: **0 promoções**.

Total ADRs vigentes: **86** (84 base + ADR-0098 P288 + ADR-0099
P289). P300 preserva contagem.

---

## §4 — Testes

P300 não materializa features → **zero testes novos**.

Testes existentes preservados:
- 2 862 testes passing (baseline P299 inalterada).
- Δ = 0 — confirma natureza não-materializadora.

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2358 passed; 0 failed; 0 ignored
test result: ok.  457 passed; 0 failed; 6 ignored
test result: ok.   24 passed; 0 failed; 0 ignored
test result: ok.    2 passed; 0 failed; 0 ignored
test result: ok.   21 passed; 0 failed; 0 ignored
                  -----
                  2862 passed total
```

Δ vs P299 = **0** ✓ (consistente com natureza não-materializadora).

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — `crystalline-lint --fix-hashes`

```
Nothing to fix
```

Zero drift — confirma estabilidade pós-P300.

### §5.4 — Hashes pós-P300

| Ficheiro | Antes P300 | Pós P300 |
|---|---|---|
| `entities/content.rs` (`@prompt-hash`) | `82d3c47d` | **`82d3c47d`** inalterado |
| `infra/export.rs` (`@prompt-hash`) | `66cb8ac3` | **`66cb8ac3` preservado bit-exact** (**17º passo consecutivo**) |
| L0 markdown (geometry/content/stdlib/export) | preservados | **todos preservados** |

---

## §6 — Padrões metodológicos

### §6.1 — §8.7' "A.0.0 template" — N=7 maduro adiado por preferência conservadora

**Janela completa P293-P299**:

```
P293: ████████░░ alta     ← H6 não-listada
P294: ██████████ máxima   ← Spec inteira invalidada
P295: ██░░░░░░░░ baixa    ← Linha tabela admin
P296: ██████░░░░ média    ← Classificação inteira inválida
P297: ████████░░ alta     ← Wrapper vanilla inexistente
P298: ████████░░ alta     ← Heurística limits já existia
P299: ███░░░░░░░ baixa    ← Calc module precedente
```

**Valor empírico documentado**:
- **P294**: evitou criação de `QuadraticTo` variant desnecessário.
- **P297**: evitou criação de `UnderoverElem` wrapper inexistente.
- **P298**: estendeu sem substituir heurística pré-existente.
- **P299**: reusou padrão `calc` sem reinvenção.

**Decisão de adiamento** (caminho A):
- 7 aplicações cumulativas — limiar tentativo passado **mas**.
- Funciona organicamente sem formalização desde P293.
- Formalização traz overhead (rigidez vs prática discricionária).
- Marco numérico P300 **não é critério metodológico**.

**Reavaliação P301+**: se §8.7' atingir N=8 com valor empírico
novo inequívoco (e.g. A.0.0 descobrindo bug que outros métodos
falhariam).

### §6.2 — §8.3 "refutação pragmática" (N=11 descritivo adiado)

11 refutações cumulativas P293-P299. Veredicto: **descritivo**
(observação sobre o sistema), não **prescritivo** (princípio
operacional). ADR teria valor marginal. **Adiado**.

### §6.3 — §8.6 "A.5' anti-reflexão" — N=10 cumulativo (P291-P300)

P300 inclui §A.5' apesar de não-materializador. N=10 é instrumental
interno; não promove.

### §6.4 — Sub-padrões cumulativos preservados como ferramentas

| Sub-padrão | N pós-P300 | Estado |
|---|---:|---|
| "Variant rico" Option estrutural | 1 genuíno (P297) | insuficiente — aguarda N≥3 |
| "Cluster math handler" | 3 ambíguo | qualidade questionável — adiado |
| "Cross-variant interaction" | 1 (P298) | insuficiente |
| "Module namespaced" | 2 (P283+P299) | insuficiente — aguarda N=3 |
| "Operadores SSoT" | 1 (P299) | insuficiente |

Todos preservados como **ferramentas discricionárias**; nenhum
formalizado.

### §6.5 — ADR-0098 "single source of truth" (**N=17 cumulativo**)

Hash `export.rs 66cb8ac3` preservado bit-exact pelos **17 passos
consecutivos** P282-P300. Invariante robusta sobre 17 features
distintas (vide tabela completa em §5 do retrospectivo).

P300 é **17.ª preservação** — pelo método trivial de zero
alterações em código (paradigma "retrospectiva sem materialização").

### §6.6 — Anti-padrão P273.17 §0 — 8 passos consecutivos honrados

**0 ADRs meta promovidas P293-P300** num período de **18 passos
sequenciais** (17 materializadores + 1 retrospectivo):

| Passo | Candidatos avaliados | Promovidos |
|---|---:|---:|
| P293 | 1 (§8.7' inaugural) | 0 |
| P294 | 1 (§8.3 N=6) | 0 |
| P295 | 2 (§8.7' N=3 + §8.3 N=7) | 0 |
| P296 | 2 | 0 |
| P297 | 3 (§8.7' + §8.3 + "variant rico" N=5) | 0 |
| P298 | 3 (§8.7' + sub-cluster N=3 + cross-variant N=1) | 0 |
| P299 | 4 | 0 |
| **P300** | **8** (auditoria sistemática) | **0** |

**Padrão**: candidatos amadurecem mas **não há pressa para
formalizar**. Anti-padrão **rigorosamente honrado**.

---

## §7 — Cobertura vanilla vs cristalino

P300 caminho A não altera cobertura material. Tabelas A/B/C
verificadas para consistência em A.5:

| Verificação | Veredicto |
|---|---|
| Tabela A.4 linha 119 (4 features) vs Tabela B Math (14 variants) | ✅ Consistente |
| Hashes L0 propagados | ✅ Sem drift |
| Frentes pendentes duplicação | ✅ Catálogo único |
| ADRs vigentes 84+2=86 | ✅ Verificável |
| P296.2 status pós-P298 | ✅ P298 fechou; P299 fechou P298.X derivada |

**Sem bugs latentes documentais críticos**.

---

## §8 — Frentes pendentes pós-P300

Vide `frentes-pendentes-pos-p299.md` — catálogo único de **~30
frentes** distintas em 11 secções:

**Prioridades sugeridas** (sem ordem fixa):
1. **P295.1 nota corpo no rodapé** — fecha cluster footnote.
2. **Auto-lookup math mode** — integração natural P299.
3. **P296.X toggles cancel** — refino cluster math.
4. **Cosméticos cleanup agregado** — múltiplos XS num passo.

**Decisão de prioridade fica ao operador humano**. P300 não dita
P301.

---

## §9 — Decisão sobre P301

P300 **não estabelece cadência de retrospectivos**. Próximos
retrospectivos só justificáveis por evidência cumulativa real,
não por marcos numéricos (P310, P400, etc.).

**P301 retorna a passos materializadores** com frentes pendentes
catalogadas:

1. Materialização nova (footnote.1, math auto-lookup, cosméticos
   cleanup, etc.).
2. Refino qualitativo de feature existente.
3. Nova frente arquitectural (se evidência sugerir).

Decisão para o operador humano.

---

## §10 — Honestidade epistémica

P300 é **estudo de caso da preservação anti-padrão**. Múltiplas
tensões equilibradas honestamente:

| Tensão | Resolução P300 |
|---|---|
| Marco numérico P300 vs critério factual | Critério factual prevalece |
| §8.7' maduro vs preferência conservadora | Conservador prevalece com justificação explícita |
| Documentação cumulativa vs proibição especulação | Apenas lições com evidência empírica |
| 8 candidatos disponíveis vs P273.17 §0 | 0 promoções honrando anti-padrão |
| Pressão "fechar série" vs P300 = consolidação intermédia | Série continua P301+ |

**Lição metodológica fundamental**:

*Marcos numéricos não são critérios metodológicos. P300 não é
"melhor" momento para promover ADR meta do que P298 ou P301. O
critério é evidência cumulativa factual inequívoca, que pode
emergir em qualquer passo.*

---

## §11 — Fecho

P300 fechado com:

- **+3 documentos novos** (diagnóstico Fase A + retrospectivo
  metodológico + frentes pendentes catálogo).
- **0 alterações em código de produção** (Δ tests = 0).
- **0 violations** no `crystalline-lint`.
- **0 drift** em hashes (`--fix-hashes` "Nothing to fix").
- **0 ADRs meta novas** — 8 candidatos avaliados sistematicamente;
  todos adiados honestamente.
- **Hash `export.rs` preservado pelo 17º passo consecutivo**
  P282-P300.
- **Anti-padrão P273.17 §0 honrado pela 8.ª vez consecutiva**
  (P293-P300).

**MARCO P300**:
- **1.º passo qualitativamente distinto** da série P283-P299 — não
  materializa feature; retrospectivo metodológico.
- **Paradigma "retrospectiva sem materialização"** inaugurado —
  11.º paradigma distinto na sequência cumulativa P288-P300.
- **Auditoria sistemática de 8 padrões cumulativos** — primeiro
  registo explícito de candidatos adiados em ficheiro único.
- **§8.7' N=7 candidato maduro adiado por preferência conservadora
  explícita** — não por evidência ambígua.
- **3 documentos cumulativos produzidos** consolidando 17 passos
  materializadores anteriores.
- **ADR-0098 N=17 cumulativo** — preservação por paradigma trivial
  (zero alterações).
- **Caminho A defensável**: marco numérico P300 NÃO foi pretexto
  para promoção; **critério factual rigorosamente honrado**.
- **Risco septenário spec evitado**: ADR-0100 "A.0.0 obrigatório"
  teria formalizado prática discricionária como regra absoluta —
  preservação intencional da flexibilidade.
- **Reset arquitectural** para P301+ com frentes catalogadas e
  padrões preservados como ferramentas.

**Marco numérico observado, critério metodológico preservado.**

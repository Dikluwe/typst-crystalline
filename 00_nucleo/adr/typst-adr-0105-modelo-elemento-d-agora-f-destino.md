# ⚖️ ADR-0105: Modelo de elemento — D incremental agora, F (PropMap) como destino

**Status**: `EM VIGOR`
**Data**: 2026-06-10
**Passo promotor**: P314 (grava a decisão; a implementação é P316+)
**Diagnóstico**: P313 (`diagnosticos/diagnostico-modelo-elemento-passo-313.md`, §3 matriz + §4 StyleChain≡F + §5 recomendação)
**Categoria**: Arquitectural / Modelo de representação de elemento
**Cross-ref**: ADR-0026 (Content enum fechado — **complementada, não revogada**),
              ADR-0104 (Atomicidade para agentes — a justificativa),
              ADR-0037 (coesão por domínio),
              ADR-0038/0039 (Style/StyleDelta — o que F unifica),
              `00_nucleo/debt-stylechain-nao-materializada.md` (DEBT sucessor 99.E — F resolve-o)

---

## Contexto

O diagnóstico P313 mediu o custo-por-elemento do desenho atual (candidato **B**,
o `enum Content` fechado da ADR-0026) e comparou três candidatos de modelo de
elemento:

- **B** — enum atual: verificação máxima (compilador), mas custo-por-elemento
  cresce com a cobertura (§1–§2 do P313); `content.rs` 5782 linhas / 77
  variantes; elemento locatável é declarado **três vezes** (`Content` +
  `ElementKind` + `ElementPayload`).
- **D** — enum fino com delegação: variante `Nome(Arc<nome::Nome>)`, lógica no
  módulo do elemento, matches do hub viram dispatchers de 1 linha.
  Custo-por-elemento: 1 módulo + 1 linha no enum + 1 linha/dispatcher.
  Exaustividade **intacta**. Migração das 77 variantes = L mecânico.
- **F** — propriedades reificadas: nó genérico `{ kind, props: PropMap }` +
  descritor por elemento + tabela const. O mais barato por elemento, mas troca
  a verificação do compilador por teste/lint (custo L+).

O §4 do P313 verificou **no código** que `Style`/`StyleDelta`
(`entities/style.rs`, `entities/style_chain.rs`) reificam **10 propriedades à
mão** (1 variante de `Style` + 1 campo `Option<T>` por propriedade), sem mapa
genérico `(elemento, campo) → chave resolvível`. O DEBT sucessor da StyleChain
(registado em 99.E, `debt-stylechain-nao-materializada.md`) exige exactamente
esse mapa. **Logo A (materializar a StyleChain) ≡ F (PropMap)**: são a mesma
obra.

---

## Decisão

### 1. Adoptar **D** (enum fino com delegação) de forma incremental

- Cada variante migra para `Content::Nome(Arc<nome::Nome>)`, com campos,
  regras e layout no módulo `01_core/src/entities/elements/nome.rs` via um
  trait `Element`. Os matches gigantes de `content.rs` (`map_content`,
  `map_text`, `eq`, …) encolhem para dispatchers (`Content::Nome(e) => e.…()`).
- A migração é **por lotes** (variante a variante; o enum tolera mistura
  `Nome(Arc<Nome>)` e `Nome { … }` durante a transição). O primeiro lote é
  decidido no arranque do P316.
- **Absorver `ElementPayload` no trait** — fecha a tripla-definição do
  locatável (§1.2 do P313): o trait fornece o payload de introspecção, em vez
  de um segundo enum fechado paralelo.

### 2. Declarar **F** (propriedades reificadas / PropMap) como **destino**

- F é executado **junto com o DEBT da StyleChain (sucessor 99.E)**, porque A ≡ F
  (§4 do P313 verificado no código). A PropMap serve **elemento e estilo de uma
  vez** — não se constroem propriedades duas vezes.

### 3. Trava gravada (cláusula 3 da ADR-0104)

F **não começa** sem repor a verificação mecânica que o compilador deixa de
dar. Mecanismo concreto a escolher no arranque de F: **um teste que varre a
tabela const × os backends** (para cada `(kind, field)` declarado, assertar
handler) **ou** uma **regra nova do `crystalline-lint`** ("todo descritor na
tabela tem handler de layout"). Erro-de-compilação **não** vira
erro-de-runtime silencioso.

### 4. Compatibilidade gravada

A forma do `impl Element` do D deve **nascer compatível** com virar descritor
do F: os módulos de elemento do D são o **continente** que F preenche (§4 do
P313). Sugere-se que `impl Element` exponha desde já algo como
`fn descriptor() -> &'static ElementDescriptor` (estável), para que F
acrescente a tabela const + PropMap sem reescrever os módulos de D.

### 5. Descartes com razão

- **E (geração por `macro_rules`)** — viola a **ADR-0104** (código gerado é
  opaco para agentes; o agente não lê a expansão; resíduo `__ComemoCall`/
  `__ComemoSurface` foi medido no mapa de migração). Rejeitado.
- **F directo agora** — risco e custo **L+** sem a StyleChain agendada; trocaria
  já a verificação do compilador por teste/lint sem o ganho da PropMap-para-
  estilo. Rejeitado como primeiro movimento; permanece o destino.

### 6. Sequência gravada

`P314` (este — ADRs + prompts finos) → `P316+` (D por lotes; primeiro lote
decidido no arranque) → `F` junto com o DEBT 99.E.

---

## Relação com ADR-0026 (explícita para o leitor futuro)

Esta ADR **complementa** a ADR-0026, **não a revoga**. O `enum Content`
fechado **permanece** (a exaustividade do compilador é o que D preserva e o que
a ADR-0104 cláusula 3 protege em F). O que muda é a **morada da lógica por
variante**: do corpo dos matches de `content.rs` (B) para módulos de elemento
(D), e eventualmente para descritores + PropMap (F). ADR-0026 continua a
descrever **porquê um enum** (vs vtable); esta ADR descreve **onde mora a
lógica de cada variante**.

---

## Prompts Afetados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `entities/content.md` | revisão futura (P316) quando o trait `Element` materializar; **não** neste passo |
| (novo, futuro) `entities/elements/_comum.md` + por elemento | a criar com o D (P316+) |

---

## Consequências

**Positivas**: corta o custo-por-elemento (métrica da ADR-0104) já com D;
fecha a tripla-definição do locatável; prepara F sem desperdício (D = continente
de F); unifica elemento e estilo numa PropMap quando F/99.E chegar.

**Negativas**: a migração D é **L** (77 variantes; mecânico mas amplo); F é
**L+** e exige construir a trava de verificação antes de relaxar o compilador.

**Neutras**: mais módulos pequenos (alinhado com ADR-0104 cláusula 1).

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **D agora, F destino** (esta) | corta custo já; verificação intacta; prepara F | migração L; dois movimentos |
| Ficar em B | zero trabalho | custo-por-elemento cresce sem teto (ADR-0104) |
| F directo | o mais barato por elemento no fim | L+; perde compilador sem a StyleChain agendada |
| E (macro) | atomicidade de ficheiro | viola ADR-0104 (opacidade) |

---

## Nota

P314 **grava** esta decisão; **não** implementa o trait `Element` nem nenhum
código do D (isso é P316). Uma linha de referência cruzada foi adicionada ao
DEBT da StyleChain (`debt-stylechain-nao-materializada.md`) apontando para esta
ADR.

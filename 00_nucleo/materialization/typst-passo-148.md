# Passo 148 — Inventário de cobertura: vanilla vs cristalino

**Série**: 148 (passo **L0-puro / administrativo**;
inventário factual de gap de cobertura entre vanilla Typst e
cristalino, antecedente à medição de paridade do Passo 149+).
**Precondição**: Passo 147 encerrado; documentos de paridade
actualizados; 1113 tests; zero violations; 57 ADRs; 10 DEBTs
abertos.

**Numeração**: 148 ocupa o lugar antes reservado para
"materialização de FrameDTO" no §9 dos documentos. **§9
desactualizado** — a sequência muda: inventário de cobertura
primeiro (148), materialização de medição depois (149+).

**Reformulação do objectivo da série**: a pergunta "em que
paridade estamos?" é mal-formulada sem inventário prévio do
que cristalino afirma suportar. Sem isso, "paridade" é número
absoluto sobre tudo o que vanilla faz — quase 0% e sem
informação útil. Com inventário, paridade torna-se "do que
afirmamos cobrir, X% bate observacionalmente". Este passo
produz o inventário; medição vem depois.

**Granularidade decidida**: matriz **dupla** (user-facing +
arquitectural), com vista cruzada ("feature X depende de
tipos A, B, C; A implementado, B parcial, C ausente").

**Formato decidido**: Markdown único em
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
Sem ADR de governança (mantém simplicidade; actualização
ad-hoc nos passos que materializam features).

**Natureza**: passo **L0-puro / administrativo**. **Zero
código**. **Zero testes**. **Zero ADRs criadas, revogadas ou
revisadas**. Único output: documento de inventário + relatório.

**ADRs aplicáveis**:
- **ADR-0026** (Content como enum) — limita a superfície de
  Content variants; relevante para a vista user-facing.
- **ADR-0033** (paridade funcional) — operacionaliza o
  conceito de "implementado".
- **ADR-0034** (diagnóstico obrigatório) — espírito cumprido
  por este passo (inventário antes de medição).
- **ADR-0054** (perfil observacional graded) — guia o
  critério de "implementado" para features com aproximação
  visual (ex: faux-bold, sem subsetting).
- **ADR-0055** (font consumer) — referência directa do
  inventário de fonts.
- **ADR-0057** (lang hyphenation) — referência directa do
  inventário de lang.

---

## Contexto

Documentos de paridade revistos em 147 propunham materialização
imediata de `FrameDTO` + `tests/layout_parity.rs` + relatório
agregado. Diálogo subsequente revelou que essa sequência
**responde mal à pergunta original**: "em que paridade
estamos?".

Argumento: medir paridade sem saber **o que cristalino afirma
cobrir** produz percentagens irrelevantes. Vanilla Typst tem
~150+ features identificáveis; cristalino implementou um
subconjunto progressivo ao longo de 146 passos. Sem inventário
explícito desse subconjunto, qualquer matriz de medição está
a comparar contra denominador errado.

Inventário canoniza o estado em três classes:
- **Implementado** — feature/tipo está em produção; consumer
  activo; testes passam.
- **Parcial** — captura/estrutura existe mas consumer está
  ausente ou aproximado (ex: `weight` faux-bold em vez de
  font Bold dedicada).
- **Ausente** — não capturado, não materializado, não
  referenciado em nenhum ADR como decisão tomada.
- **Scope-out** — explicitamente declarado fora do escopo via
  ADR (ex: shaping rustybuzz; subsetting; gap 8 font dict).

Cada entrada do inventário tem **referência canónica**: passo
de materialização ou ADR que justifica a classificação.

---

## Objectivo

Ao fim do passo:

1. **Documento de inventário** em
   `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
   com:
   - **Vista user-facing** (Tabela A): features do utilizador
     no Typst — markup elements, funções de stdlib, regras
     `#set`/`#show`/`#let`, math, listas, tabelas, etc.
   - **Vista arquitectural** (Tabela B): tipos do Rust —
     variants de `Content`, `Value`, `Style`/`StyleDelta`,
     `FrameItem`, traits principais.
   - **Vista cruzada** (Tabela C): para cada feature
     user-facing **parcial** ou **ausente**, lista de tipos
     arquitecturais bloqueantes.
   - **Resumo agregado**: contagens por classe em cada vista.

2. **Critério de classificação documentado**:
   - "Implementado" exige consumer activo + tests + sem
     ressalvas materiais.
   - "Parcial" cobre 4 sub-casos: aproximado (faux-bold),
     scope-limited (single-font era assim antes de 146;
     hyphenation é escopo restrito), captura-sem-consumer
     pré-DEBT-52, ou consumer-com-divergência (ex:
     subsetting ausente — faz parte de "implementado com
     ressalva").
   - "Ausente" requer verificação dupla: grep no código + sem
     ADR/Passo que materialize.
   - "Scope-out" requer ADR explícita.

3. **Cobertura do inventário**:
   - **User-facing**: ~150 entradas estimadas. Inclusivo:
     todos os elementos `typst-library` listados em
     `lab/typst-original/library/` ou `lab/typst-original/
     reference/` (formato a confirmar em 148.1).
   - **Arquitectural**: enums e structs principais de
     `01_core/src/entities/` + variants do vanilla
     (`lab/typst-original/.../content.rs`, etc.).

4. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-148-relatorio.md`
   com:
   - Sumário das 4 classes (counts por vista).
   - Top divergências surpreendentes (ex: features marcadas
     "ausente" que talvez não devessem estar; "implementado"
     com ressalvas não-óbvias).
   - Próximo passo: 149 — materialização de `FrameDTO` +
     primeiros tests com corpus filtrado pelo subconjunto
     "implementado" + "parcial".

5. **`§9 Próximas acções` dos documentos de paridade
   actualizado**:
   - Item 1 dessa lista (materializar `frame_dto.rs` em P148)
     passa a referir P149+.
   - Esta correcção é uma actualização menor — não justifica
     reabrir o passo 147 como passo separado; entra no
     relatório 148 como sub-acção.

Este passo **não**:

- Implementa `ValueDTO`, `FrameDTO`, `pdf_compare`, ou
  qualquer infraestrutura em `lab/parity/`. **Tudo Passo
  149+**.
- Toca código em L1, L2, L3, L4.
- Toca testes.
- Cria ADRs.
- Modifica `DEBT.md` ou `00_nucleo/adr/README.md`.
- Materializa features ausentes do inventário.
- Decide priorização entre features ausentes (decisão
  humana posterior).
- Resolve a tensão "diagnosticos imutável vs editável" (147
  registou; 148 não corrige).

---

## Decisões já tomadas

1. **Inventário antes de medição**. Reformulação aceite
   pelo utilizador; documentada como decisão da série.
2. **Granularidade dupla** (user-facing + arquitectural +
   cruzada). Resposta a pergunta diagnóstica.
3. **Markdown único, sem ADR**. Burocracia mínima;
   actualização ad-hoc.
4. **4 classes** (implementado / parcial / ausente /
   scope-out). Modelo simples; suficientemente granular.
5. **Localização**: `00_nucleo/diagnosticos/`. Coerente com
   147; tensão imutabilidade/edição registada como dívida.
6. **§9 dos documentos de paridade actualizado**: alteração
   menor (renumeração de passos), entra no relatório 148.
7. **Sem priorização**. Inventário lista; humanos priorizam
   depois.

## Decisões diferidas (resolvidas neste passo)

8. **Fonte canónica do user-facing inventário**: confirmar
   em 148.1 onde vanilla lista as features publicamente.
   Candidatos:
   - `lab/typst-original/library/src/` (módulos Rust por
     categoria: text, math, layout, foundations, etc.).
   - `lab/typst-original/docs/` (se existir; documentação
     de utilizador).
   - `lab/typst-original/reference/` (idem).
   - GitHub typst.app/docs (não preferível — depende de
     fetch externo).
   Decisão default: `lab/typst-original/library/src/`,
   complementado por `docs/`/`reference/` se existirem.

9. **Granularidade exacta de tipos arquitecturais**: um nível
   é variants de enum; outro é fields de struct. Decisão
   default: variants de enum (mais material para inventário);
   fields entram só quando a sua presença/ausência é
   conhecidamente material (ex: `StyleDelta.font` foi
   inerte vs activo é entrada). Confirmar em 148.1.

10. **Tratamento de features que vanilla tem mas não são
    publicamente documentadas** (ex: helpers internos,
    macros de stdlib): excluir do user-facing; podem entrar
    no arquitectural se tiverem tipo dedicado. Critério em
    148.1.

11. **Tratamento de aliases / re-exports** (vanilla pode
    expor `text.bold` e `strong` como caminhos para o
    mesmo efeito): listar como entradas separadas com
    nota cruzando. Inventário fiel ao que utilizador vê.

12. **Critério de "implementado" vs "parcial" para casos
    com ADR-0054 perfil graded**: features que ADR-0054
    declara aceitáveis em forma aproximada são
    "implementado com ressalva" (sub-classe de
    implementado), não "parcial". Razão: ADR-0054 redefine
    o que conta como completo. Notação no inventário:
    "implementado⁺" (com nota explicativa).

13. **Limite prático de tamanho do inventário**: ~150
    entradas user-facing + ~50 arquitecturais = ~200
    linhas de tabela + cabeçalhos. Aceite. Se inventário
    cresce para >300 entradas, **pausar** e considerar
    ramificação 148A (núcleo) + 148B (specialized).

---

## Escopo

**Dentro**:

- Inventário empírico de features vanilla via
  `lab/typst-original/library/src/`.
- Inventário empírico de tipos cristalinos via
  `01_core/src/entities/`.
- Cross-referência com ADRs (0001–0057), Passos (1–146),
  DEBTs (encerrados e abertos).
- Tabelas A, B, C como descrito em §Objectivo.
- Resumo agregado.
- Top divergências surpreendentes.
- Actualização menor de `§9` dos documentos de paridade.
- Relatório do passo.

**Fora**:

- Implementação de qualquer feature ausente.
- Materialização de `lab/parity/` infra.
- Decisão sobre o que vanilla **devia** ter ou não.
- Renumeração de Passos posteriores em geral (apenas
  `§9` dos docs de paridade).
- Validação experimental do inventário ("este input passa /
  falha"). É inventário factual, não medição.
- Subdivisão de "parcial" em sub-classes para além das 4
  documentadas.
- Discussão de prioridade entre ausentes.
- Tradução para EN ou outro idioma.

---

## Sub-passos

### 148.1 — Localização canónica das fontes

**A.1.1 — User-facing (vanilla)**:

```bash
ls -la lab/typst-original/
ls lab/typst-original/library/src/ 2>/dev/null
ls lab/typst-original/docs/ 2>/dev/null
find lab/typst-original/ -type d -maxdepth 3
```

Identificar onde vivem as definições de elementos públicos.
Esperado: `lab/typst-original/library/src/` com submódulos
por categoria (`text/`, `math/`, `layout/`, `foundations/`,
`introspection/`, `model/`, `visualize/`).

Para cada submódulo:

```bash
grep -E "^pub (fn|struct|elem!) " lab/typst-original/library/src/<categoria>/*.rs
```

`elem!` é a macro do vanilla que define elementos; `pub fn`
captura funções nativas; `pub struct` captura tipos públicos.

**A.1.2 — Arquitectural (vanilla)**:

```bash
grep -nE "^pub enum (Value|Content|Style|FrameItem|Item)" lab/typst-original/foundations/src/*.rs
grep -nE "^pub enum (Value|Content|Style|FrameItem|Item)" lab/typst-original/syntax/src/*.rs
```

Para cada enum encontrado, extrair as variants:

```bash
grep -A 40 "^pub enum Content" <ficheiro> | grep -E "^\s+\w+\s*[(,{]"
```

**A.1.3 — Arquitectural (cristalino)**:

Análogo, em `01_core/src/entities/`:

```bash
grep -nE "^pub enum (Value|Content|Style|FrameItem)" 01_core/src/entities/*.rs
```

Para cada enum, extrair variants. Cross-reference com vanilla.

**A.1.4 — User-facing (cristalino)**:

Critério: feature está implementada se um dos seguintes for
verdade:
- Construtor existe em `01_core/src/rules/eval.rs` ou
  `stdlib`.
- Variant capturada em `Content`, `Style`/`StyleDelta`,
  `Value`.
- Test em `01_core/src/rules/` ou `03_infra/src/integration_tests.rs`
  exercita.

Implícito: 146 passos materializaram features
progressivamente. Pista heurística: se uma feature aparece
em algum passo de Materialização (`00_nucleo/materialization/`
ou ADR), é provavelmente implementado/parcial.

### 148.2 — Construir Tabela A (user-facing)

Schema:

```markdown
| Feature | Categoria | Vanilla path | Cristalino estado | Referência canónica | Nota |
|---------|-----------|--------------|--------------------|---------------------|------|
| `text.bold` | text | library/src/text/raw.rs | implementado⁺ | Passo 30; ADR-0054 (perfil) | faux-bold via stroke (Passo 139) quando font Bold dedicada não disponível |
| `text.font` | text | library/src/text/raw.rs | implementado | Passo 140B+141+146 | single, array, multi-font per doc |
| `text.lang` | text | library/src/text/raw.rs | implementado⁺ | Passo 144 (ADR-0057) | hyphenation activo; shaping ausente (DEBT-53) |
| `heading` | model | library/src/model/heading.rs | implementado | Passos 99, 103 | show rules suportadas |
| `figure` | model | library/src/model/figure.rs | ausente | — | — |
| `table` | model | library/src/model/table.rs | ausente | — | — |
| `bibliography` | model | … | ausente | — | escopo grande |
| `math.equation` | math | library/src/math/… | parcial | passos de math | superscript/subscript/fração |
| ... | | | | | |
```

Esperado: ~150 entradas. Categorias prováveis: foundations,
text, layout, model, math, visualize, introspection, others.

**Forma das entradas**:
- "implementado": tudo OK.
- "implementado⁺": consumer activo mas com aproximação
  documentada por ADR-0054 ou ADR específica.
- "parcial": captura ou estrutura existe; consumer ausente,
  divergente, ou opcional.
- "ausente": não capturado.
- "scope-out": ADR declara fora do escopo.

### 148.3 — Construir Tabela B (arquitectural)

Schema:

```markdown
| Tipo | Variant | Vanilla path | Cristalino estado | Referência | Nota |
|------|---------|--------------|--------------------|-------------|------|
| `Value` | `None` | foundations/src/value.rs | implementado | Passo X | |
| `Value` | `Bool` | … | implementado | … | |
| `Value` | `Int` | … | implementado | Passo 30 + 25 (Int==Float ADR-0025) | |
| `Value` | `Float` | … | implementado | … | |
| `Value` | `Length` | … | implementado | Passo 127 | tracking |
| `Value` | `Color` | … | implementado | Passo 102 | fill |
| `Value` | `Datetime` | … | implementado | Passo 21 (ADR-0021) | |
| `Value` | `Func` | … | implementado | … | |
| `Value` | `Type` | … | implementado | … | |
| `Value` | `Module` | … | implementado | … | |
| `Value` | `Args` | … | parcial | … | |
| `Value` | `Bytes` | … | ausente | — | |
| `Value` | `Decimal` | … | ausente | — | |
| `Value` | `Regex` | … | scope-out | ADR-0054bis condicional | gap 8 |
| `Content` | `Empty` | … | implementado | … | |
| `Content` | `Text` | … | implementado | Passo 30 | |
| `Content` | `Sequence` | … | implementado | Passo 30; ADR-0026 + 0026-R1 | Arc<[T]> |
| `Content` | `Styled` | … | implementado | Passo 99–101 | |
| `Content` | `Heading` | … | parcial | Passo 103 | show rules; nem todos os atributos |
| `Content` | `Equation` | … | parcial | math passos | |
| `Content` | `Image` | … | ausente | — | |
| `Content` | `Table` | … | ausente | — | |
| `Style` | `Bold` | … | implementado | Passo 99 | |
| `Style` | `Italic` | … | implementado | Passo 99 | |
| `Style` | `Size` | … | implementado | Passo 99 | |
| `Style` | `Fill` | … | implementado | Passo 99, 102 | |
| `Style` | `HeadingLevel` | … | implementado | Passo 99 | forward-compat até 103 |
| `StyleDelta` | (10 fields) | — | (ver relatório 142 §3) | Passos 30–139 | DEBT-1 fechado |
| `FrameItem` | `Text` | … | implementado | Passos 19–146 | multi-font |
| `FrameItem` | `Group` | … | implementado | … | |
| `FrameItem` | `Glyph` | … | implementado | … | math variants |
| `FrameItem` | `Line` | … | implementado | … | |
| `FrameItem` | `Image` | … | ausente | — | |
| `FrameItem` | `Shape` | … | parcial | … | |
| ... | | | | | |
```

Esperado: ~50 entradas.

### 148.4 — Construir Tabela C (cruzada)

Para cada entrada **parcial** ou **ausente** da Tabela A,
identificar tipos arquitecturais bloqueantes:

```markdown
| Feature user-facing (parcial/ausente) | Tipos arquitecturais bloqueantes | Nota |
|---------------------------------------|-----------------------------------|------|
| `figure` | `Content::Figure` (ausente) | requer `Content` enum extension |
| `table` | `Content::Table` (ausente) + render | requer cell layouting |
| `bibliography` | `Content::Bibliography` (ausente); `Citation`; CSL parsing | escopo XL |
| `math.equation` (parcial) | `FrameItem::Glyph` math variants OK; shaping ausente | rustybuzz DEBT-53 |
| `text.lang` (parcial) | `Lang` OK; shaping ausente | DEBT-53 |
| `text.weight` (parcial) | `StyleDelta.weight` OK; font-file Bold selection ausente | ADR-0055bis candidata |
| `text.font` dict | `regex` ausente em L1 | ADR-0054bis condicional |
| ... | | |
```

### 148.5 — Resumo agregado

Tabela final:

```markdown
| Vista | Implementado | Implementado⁺ | Parcial | Ausente | Scope-out | Total |
|-------|--------------|---------------|---------|---------|-----------|-------|
| User-facing (Tabela A) | A | B | C | D | E | A+B+C+D+E |
| Arquitectural (Tabela B) | F | G | H | I | J | F+G+H+I+J |

**Cobertura user-facing total** (impl + impl⁺): (A+B) / (A+B+C+D+E)
**Cobertura arquitectural total**: análogo
**Itens scope-out**: nomeados com referência ADR
```

Nota: estes valores (A, B, C…) são contagens absolutas;
percentagens calculam-se mas devem ser interpretadas com
cuidado — não são paridade. São **cobertura declarada**.

### 148.6 — Top divergências surpreendentes

Subsecção do relatório com 5–10 entradas onde a classificação
foi **não-óbvia** (sinaliza para humanos focar discussão):

- Features que pareciam implementadas mas inventário revelou
  parcialidade não documentada.
- Features ausentes que talvez não estejam ausentes (verificar
  com grep adicional).
- Tipos arquitecturais com variants em vanilla que cristalino
  não tem mas pode não precisar (ADR-0026 paridade ≠
  paridade total).
- Tipos com variants extra em cristalino vs vanilla (ex:
  cristalino tem `Content::Styled` enquanto vanilla usa
  vtable — divergência intencional ADR-0026).

### 148.7 — Actualização menor de §9 dos documentos de paridade

Em `00_nucleo/diagnosticos/typst-paridade-plano-medicao.md`:

```diff
  ## 9 — Próximas acções concretas
  
  ...
  
- 1. **Passo 148** — Implementar `frame_dto.rs` com
-    `LayoutTolerance` e modo `text_content=true`.
+ 1. **Passo 148** — Inventário de cobertura
+    vanilla vs cristalino (passo de diagnóstico).
+    Reformulação da série: paridade só é mensurável
+    após inventário.
+ 2. **Passo 149** — Implementar `frame_dto.rs` com
+    `LayoutTolerance` e modo `text_content=true`.
+    Corpus filtrado pelo subconjunto "implementado" +
+    "parcial" do inventário 148.
```

(Renumerar 2/3/4 para 3/4/5; ajustar referências.)

### 148.8 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-148-relatorio.md`.

Secções:
1. Sumário executivo.
2. Localizações canónicas confirmadas (148.1).
3. Tabela A (user-facing) — número de entradas + breakdown
   por categoria.
4. Tabela B (arquitectural) — idem.
5. Tabela C (cruzada) — pontes entre as duas vistas.
6. Resumo agregado (contagens).
7. Top divergências surpreendentes.
8. Actualização §9 documentos de paridade.
9. Próximo passo: 149 (materialização de `FrameDTO`).
10. Verificação final.

---

## Verificação

1. ✅ Documento `typst-cobertura-vanilla-vs-cristalino.md`
   criado em `00_nucleo/diagnosticos/`.
2. ✅ Tabela A (user-facing) com ~150 entradas.
3. ✅ Tabela B (arquitectural) com ~50 entradas.
4. ✅ Tabela C (cruzada) com pontes para todas as
   entradas parciais/ausentes da Tabela A.
5. ✅ Resumo agregado com contagens.
6. ✅ Top divergências documentadas.
7. ✅ Cada entrada tem referência canónica (Passo / ADR /
   DEBT) ou explicitamente nenhuma se ausente.
8. ✅ §9 dos documentos de paridade actualizado para
   reflectir Passo 148 = inventário; Passo 149+ =
   materialização.
9. ✅ Nenhum ficheiro em `lab/parity/` tocado.
10. ✅ Nenhum ficheiro em `01_core/`, `02_shell/`,
    `03_infra/`, `04_wiring/` tocado.
11. ✅ Nenhuma ADR criada / revogada / revisada.
12. ✅ `DEBT.md` e `00_nucleo/adr/README.md` intactos.
13. ✅ `cargo test --workspace --lib`: inalterado (1113).
14. ✅ `crystalline-lint .`: zero violations.
15. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. Documento de inventário coerente; cada entrada com
   classificação justificada.
2. Cobertura declarada quantificada (contagens absolutas).
3. Não há entrada sem referência canónica (excepto
   "ausente", que pode legitimamente não ter referência).
4. §9 actualizado.
5. Relatório do passo escrito; próximo passo (149)
   tem âncora documental clara.
6. Sem materialização de medição (escopo de 149+).
7. Sem código tocado.

---

## O que pode sair errado

- **Vanilla expõe features de forma não-trivial** (ex: macros
  geram elementos; `elem!` produz tipos via proc-macro):
  inventário pode ter de inferir features pela expansão de
  macros, não pela leitura linear do código. Documentar a
  metodologia no relatório.

- **Inventário cresce para >300 entradas**: ramificação
  em 148A (núcleo: text/math/layout/foundations) + 148B
  (specialized: model/visualize/introspection). Decidir
  pausar quando se chegar a 200; reavaliar.

- **Classificação ambígua entre "parcial" e "implementado⁺"**:
  exemplos reais — `text.font` com single-font era "parcial"
  pré-146, "implementado" pós-146; `text.weight` é
  "implementado⁺" porque ADR-0054 perfil graded aceita
  faux-bold; `text.lang` é "implementado⁺" (hyphenation OK,
  shaping ausente). Usar critério de §Decisão 12: ADR-0054
  cobre? "implementado⁺". Senão, "parcial".

- **Critério de "ausente"**: tem de ser empírico (grep + sem
  referência em ADR/Passo), não inferencial. Se cristalino
  tem **algo** de uma feature mesmo que mínimo, fica
  "parcial" não "ausente". Documentar.

- **Vanilla tem features que cristalino **descartou
  intencionalmente** mas não há ADR explícita**: ex: ?? — se
  detectado, registar como **divergência intencional não
  formalizada** (sub-classe de scope-out). Candidato para ADR
  futura. Não aberto neste passo.

- **`StyleDelta` tem 10 fields, 9 com consumer; quão granular
  cada field na Tabela B?**: cada field é uma entrada
  separada. Coerente com mapping campo-a-campo do relatório
  142 §3.

- **Fronteira fluida entre user-facing e arquitectural**: ex:
  `text.fill` é user-facing; `Style::Fill` é arquitectural.
  Manter ambas; cruzar na Tabela C.

- **Inventário do vanilla cresce com upgrades**: o
  `lab/typst-original/` está congelado num commit específico
  (vanilla 0.x). Documentar a versão exacta no topo do
  inventário. Atualização futura com novas releases é trabalho
  separado (passo dedicado quando necessário).

- **Tabela cresce e Markdown fica difícil de ler**: aceite
  como custo. Alternativa (CSV ou JSON) sacrificaria
  legibilidade humana. Manter Markdown; usar tabelas
  compactas com colunas curtas.

- **Distinção `implementado⁺` vs `parcial` confunde**:
  considerar nota expandida no documento explicando o
  critério. Se confusão persistir nos passos seguintes,
  refactorar nomenclatura — passo dedicado futuro.

- **Tempo total > esperado**: aceite. É inventário;
  qualidade > velocidade.

---

## Notas operacionais

- **Modelo: passo de inventário análogo a 131A/132A/140A**.
  Diagnóstico-primeiro consistente. Diferença: aplicado a
  cobertura macro do projecto, não a feature específica.

- **Reformulação da série**: pergunta original "em que
  paridade estamos?" foi **transformada** em:
  1. (148) "que features cristalino afirma cobrir?"
  2. (149+) "do que cobre, X% bate com vanilla?"
  
  Esta reformulação é registada no relatório como decisão da
  série paridade — não nova ADR, apenas documento.

- **Sem ADR de governança do inventário**. Decisão por
  simplicidade. Se o inventário ficar desactualizado nos
  passos seguintes (cada nova feature exige actualização ad-
  hoc), considerar ADR + passo de uniformização análogo ao
  P145.

- **Top divergências como ponto de partida** para discussão
  humana sobre prioridades:
  - Quais ausências são bloqueantes? (Tabelas, figuras,
    bibliografia: muito visível para utilizador.)
  - Quais "parcial" são suficientes para uso real? (Math
    com superscript/subscript/fração mas sem matrizes pode
    ser usável para texto académico básico.)
  - Quais scope-out vale a pena reabrir? (Subsetting reduz
    PDFs; rustybuzz dá shaping completo.)

- **Inventário não é roadmap**. Lista o que existe e o que
  não. Decisão de "implementar feature X" é separada,
  humana, posterior.

- **Próximo passo (149)**: materialização de `FrameDTO` +
  primeiros tests com corpus filtrado pelo subconjunto
  "implementado" + "parcial" + "implementado⁺" do inventário.
  Métrica passa a ter denominador honesto.

- **Vanilla freezing**: documentar a versão de
  `lab/typst-original/` (commit hash + tag) no topo do
  inventário. Permite rastrear actualizações futuras.

- **Numeração 149/150 mantém-se indicativa**. Se 149
  ramifica, 149A (corpus + filtro) + 149B (FrameDTO + tests
  + relatório). Padrão paga-se nas 4 aplicações anteriores.

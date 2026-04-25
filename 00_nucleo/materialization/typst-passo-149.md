# Passo 149 — Formalização de `Value::Type` e `Value::Args` (arqueologia + classificação)

**Série**: 149 (passo **L0-puro / administrativo**;
arqueologia + classificação de divergências detectadas no
inventário 148).
**Precondição**: Passo 148 encerrado; inventário de cobertura
em
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`;
54% cobertura user-facing; 70% arquitectural; 1113 tests.

**Numeração**: 149 ocupa o lugar antes reservado para
`frame_dto.rs` + matriz P3. **§9 dos documentos de paridade
desactualizado de novo** — sequência muda: formalização
arqueológica primeiro (149), materialização de medição
depois (150+). Actualização menor de §9 entra no relatório
deste passo.

**Natureza**: passo **L0-puro / administrativo**.
Arqueologia + decisão de classificação, **não** alteração de
código. **Zero código**. **Zero testes**. **Possível criação
de 1-2 ADRs e/ou abertura de 1-2 DEBTs** consoante o resultado
da arqueologia.

**Divergências em escopo** (identificadas no inventário 148
§7):

1. **`Value::Type`** — em cristalino é string-based parcial;
   vanilla tem `Type` como tipo dedicado.
2. **`Value::Args` não-variant** — em cristalino, `Args` é
   tipo separado passado para funções nativas; vanilla tem
   `Value::Args` como variant do enum.

**Outras divergências do inventário 148 §7 não atacadas
neste passo**:

- `Value::Align` (divergência favorável cristalino, DEBT-36
  fechada) — registar como divergência intencional **sem
  formalização nova** se o passo de arqueologia confirmar
  que DEBT-36 ou ADR existente já a cobre.
- `Content::Styled(Box, Styles)` (vanilla vtable vs
  cristalino enum fechado) — coberto por
  ADR-0026/0026-R1/0038. **Sem necessidade** de
  formalização nova.

Foco do passo: as 2 divergências sem ADR/DEBT canónica.

**ADRs aplicáveis**:
- **ADR-0026** (Content como enum) — precedente de
  divergência arquitectural intencional. Modelo para
  classificar `Value::Args` se for divergência consciente.
- **ADR-0033** (paridade funcional) — referência para
  decidir se a divergência afecta paridade observacional
  (provável: não, dado que 1113 tests passam).
- **ADR-0034** (diagnóstico obrigatório) — espírito
  cumprido pelo inventário 148 e por este passo.
- **ADR-0036** (atomização progressiva, ADR-0038
  expandida) — se `Args` é tipo separado por razões de
  atomização, esta ADR é referência directa.

---

## Contexto

Inventário 148 detectou que cristalino diverge de vanilla
em duas formas de representação de `Value`:

**Vanilla**: `Value::Type(Type)` — variant fechada com tipo
dedicado `Type` que carrega informação rica (nome, métodos,
docstring, etc.).

**Cristalino**: `Value::Type(EcoString)` (a confirmar em
149.1) — variant simplificada que carrega apenas o nome
como string.

Análogo:

**Vanilla**: `Value::Args(Args)` — variant do enum.

**Cristalino**: `Args` é tipo separado, **não-variant** de
`Value`. É construído e consumido pelo eval mas não passa
por dentro de `Value` como uma variant.

Ambas as divergências passaram pela **construção do enum
`Value`** num passo histórico que **não foi formalizado por
ADR** e não tem entrada em `DEBT.md`. O inventário 148 §7
sinalizou-as.

A pergunta a responder é dupla:

1. **Arqueologia**: quando foi cada decisão tomada? Que
   passo materializou? Houve discussão registada?
2. **Classificação**: a decisão é **intencional permanente**
   (→ ADR formaliza), **dívida a resolver** (→ DEBT abre),
   ou **intencional temporária** (→ ADR formaliza com plano
   de evolução)?

Resposta depende empiricamente do que arqueologia revele.
Este enunciado **não pré-classifica**.

---

## Objectivo

Ao fim do passo:

1. **Arqueologia documentada** em
   `00_nucleo/diagnosticos/divergencias-value-passo-149.md`:
   - Para `Value::Type`: quando foi materializado (passo);
     forma actual (assinatura + `entities/value.rs` linha);
     forma vanilla (assinatura + path); razões registadas
     em relatório/passo se existirem.
   - Idem para `Value::Args`.
   - Se nenhuma razão foi registada: declarar
     explicitamente "sem registo de razão" — facto também
     vale.

2. **Classificação de cada divergência**:
   - `Value::Type`: ADR? DEBT? ADR com plano? Decisão
     justificada com base na arqueologia.
   - `Value::Args`: idem.

3. **Materialização da classificação**:
   - Se ADR(s) → criar `ADR-0058` e/ou `ADR-0059`
     conforme. Cabeçalhos canónicos (P145).
   - Se DEBT(s) → adicionar a `DEBT.md` Secção 1 com
     número novo (próximo disponível: confirmar em 149.1;
     candidato `DEBT-53` se ainda livre, ou `DEBT-54`).
   - Se ADR com plano → ADR + entrada relacionada em DEBT.

4. **Inventário 148 actualizado** se a classificação
   altera nomenclatura usada na Tabela A/B/C:
   - Ex: `Value::Type` é **parcial** no inventário; se
     decisão for "ADR formaliza como divergência
     intencional permanente", classificação muda para
     **implementado⁺** (ou nova classe a discutir).
   - Inventário ganha referência cruzada para
     ADR-0058/0059 ou DEBT-53/54.

5. **README dos ADRs actualizado** se ADRs novas forem
   criadas:
   - Tabela "Estado por ADR".
   - Distribuição de status.
   - Total.
   - Entrada em "Passos-chave" para Passo 149.

6. **`§9` dos documentos de paridade actualizado** com
   renumeração:
   - Passo 149 = arqueologia + formalização (este).
   - Passo 150 = `frame_dto.rs` + matriz P3 (era 149
     pós-148).
   - Restantes renumerados.

7. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-149-relatorio.md`
   com decisões justificadas.

Este passo **não**:

- Toca código em L1, L2, L3, L4.
- Modifica `Value::Type` ou `Value::Args` em si. Mesmo
  que a classificação seja "DEBT a resolver", a resolução
  é passo separado.
- Materializa `frame_dto.rs` ou medição P3.
- Re-classifica outras divergências do inventário 148 §7.
- Resolve a tensão "diagnosticos imutável vs editável"
  (registada em 147; arrastada para cá).

---

## Decisões já tomadas

1. **Arqueologia antes de classificação**. Sem saber
   quando/porquê, classificação é especulação. Padrão
   consistente com 131A/132A/140A (diagnóstico antes de
   decisão).
2. **Possíveis 4 outputs**: 0 ADRs + 0 DEBTs (ambas
   adequadamente cobertas e o inventário só precisa de
   referência cruzada); 1-2 ADRs; 1-2 DEBTs; mistura.
   **Resultado depende empiricamente**.
3. **Não modificar código**. Mesmo classificação "DEBT" não
   activa correção neste passo.
4. **Cabeçalhos canónicos** (P145) para qualquer ADR
   nova: `**Status**:` com backticks; título com `⚖️`.

## Decisões diferidas (resolvidas neste passo)

5. **Critério para "ADR" vs "DEBT"**:
   - **ADR** se a divergência é **decisão arquitectural
     consciente** (revertê-la exige justificação).
     Permanente ou condicionalmente permanente.
   - **DEBT** se a divergência é **simplificação
     temporária** que sabemos ser inferior à forma vanilla
     mas adiada por priorização. Resolvível em passo
     futuro.
   - **ADR + DEBT** se decisão **acta interim** mas plano
     de evolução é conhecido (ex: ADR registar o estado
     actual como aceitável; DEBT registar o plano
     condicional de migração).
6. **Como atribuir**: arqueologia em 149.1 + leitura de
   passo materializador em 149.2. Se relatório do passo
   contém razão explícita, segue-a. Se não, decisão é
   tomada em 149.3 com justificação no relatório deste
   passo.

7. **Se nenhuma das 2 divergências precisar de ADR/DEBT
   nova**: passo continua válido. Output é apenas
   arqueologia + actualização do inventário 148 com
   referência ao registo arqueológico. Aceitável.

---

## Escopo

**Dentro**:

- Leitura empírica de `01_core/src/entities/value.rs` para
  confirmar forma actual.
- Leitura de
  `lab/typst-original/crates/typst-library/src/foundations/value.rs`
  e ficheiros relacionados (`type.rs`, `args.rs`) para
  confirmar forma vanilla.
- `git log` ou `git blame` em `value.rs` para localizar
  quando cada variant tomou a forma actual. Se git não está
  disponível, usar `grep -rn "Value::Type\|Value::Args" 00_nucleo/`
  para localizar passos.
- Leitura de relatórios de passos materializadores
  candidatos em `00_nucleo/materialization/`.
- Possível criação de 1-2 ADRs em `00_nucleo/adr/`.
- Possível abertura de 1-2 DEBTs em `00_nucleo/DEBT.md`.
- Actualização do inventário 148.
- Actualização do README dos ADRs (se ADRs novas).
- Actualização menor de `§9` dos documentos de paridade.
- Escrita do relatório do passo.

**Fora**:

- Modificação de `01_core/src/entities/value.rs` ou
  qualquer ficheiro de código.
- Modificação de testes.
- Resolução de DEBT, mesmo se aberto neste passo.
- Re-classificação de outras divergências do inventário
  148 §7 (`Value::Align`, `Content::Styled`, etc.). Cada
  uma seria escopo separado.
- Materialização de `frame_dto.rs` (Passo 150 pós-149).

---

## Sub-passos

### 149.1 — Inventário factual da forma actual

**A.1.1 — Cristalino: `Value::Type`**:

```bash
grep -nE "^\s*(Type|Args)\b" 01_core/src/entities/value.rs
view 01_core/src/entities/value.rs   # ver enum Value completo
```

Registar:
- Variant exacta (assinatura: que tipo carrega? `EcoString`?
  outro?).
- Linha exacta no ficheiro.
- Comentário Rust se houver (`/// ...`).

**A.1.2 — Cristalino: `Args`**:

```bash
grep -rn "pub struct Args\|impl Args" 01_core/src/
```

Registar:
- Onde `Args` vive (`entities/args.rs`? `rules/eval.rs`?).
- Se é referenciado por `Value` (`grep "Args" 01_core/src/entities/value.rs`).
- Se é variant ou tipo separado.

**A.1.3 — Vanilla: `Value::Type`**:

```bash
view lab/typst-original/crates/typst-library/src/foundations/value.rs
view lab/typst-original/crates/typst-library/src/foundations/ty.rs   # se existir
```

Registar a variant + tipo carregado.

**A.1.4 — Vanilla: `Value::Args`**:

```bash
view lab/typst-original/crates/typst-library/src/foundations/args.rs
```

Idem.

**A.1.5 — Tabela comparativa**:

```markdown
| Aspecto | Vanilla | Cristalino |
|---------|---------|------------|
| `Value::Type` carrega | `Type` (struct rico) | `EcoString`/outro (a confirmar) |
| `Value::Args` existe? | Sim, variant | Provavelmente não — `Args` é tipo separado |
| ... | | |
```

### 149.2 — Arqueologia: quando + porquê

**A.2.1 — Localizar passo materializador**:

Para cada divergência, identificar **passo** que a
introduziu. Estratégias:

```bash
git log --all --oneline -- 01_core/src/entities/value.rs | head -50
git blame 01_core/src/entities/value.rs > /tmp/blame.txt
# Procurar "Type" e "Args" — primeiro commit que os introduziu
```

Se git não disponível, alternativa:

```bash
grep -rn "Value::Type" 00_nucleo/materialization/ | head -20
grep -rn "Value::Args" 00_nucleo/materialization/ | head -20
grep -rn "Value::Type\|Value::Args" 00_nucleo/adr/ | head
```

Identificar o passo mais antigo que mencione cada uma.
Esperado: passo entre 1 e 30 (eval básico foi materializado
cedo).

**A.2.2 — Ler relatório do passo materializador**:

`view 00_nucleo/materialization/typst-passo-NN-relatorio.md`
para o passo identificado.

Procurar:
- Razão explícita para forma escolhida.
- Discussão de alternativas.
- Referência a ADR ou DEBT.
- "TODO", "FIXME", "fora do escopo" — sinais de dívida
  consciente.

**A.2.3 — Possível razões esperadas (hipóteses)**:

- **`Value::Type` como string**: Vanilla `Type` exigia
  reflexão runtime / docstrings / proc-macros. Cristalino
  pode ter optado por simplificação para evitar superfície
  externa.
- **`Args` como tipo separado**: pode ser por atomização
  (ADR-0036) — `Args` é veículo de input para funções
  nativas, não valor avaliado.

Hipóteses a confirmar empiricamente em 149.2.A.2.

### 149.3 — Decisão de classificação

Para cada divergência, com base nos dados de 149.1 e 149.2:

**Critério explícito**:

| Encontrámos isto | Classificação |
|------------------|---------------|
| Razão explícita "decisão consciente, forma final" | ADR |
| Razão explícita "simplificação a expandir depois" | DEBT |
| Razão explícita "actual + plano" | ADR + DEBT |
| Sem razão registada + comportamento estável + tests OK | ADR (formalizar a decisão de facto) |
| Sem razão registada + comportamento sub-óptimo evidente | DEBT (abrir, plano humano) |
| Razão depende de outra ADR já existente | Referência cruzada; sem ADR/DEBT nova |

**Resultado esperado** (não compromisso):

- `Value::Type`: provável **ADR** ("Tipo simplificado em
  L1") ou **ADR + DEBT** se houver plano de migração
  futura para tipo dedicado.
- `Value::Args`: provável **ADR** ("`Args` como tipo
  separado, não-variant") referenciando ADR-0036
  (atomização). Possivelmente apenas referência cruzada se
  ADR-0036 já cobre suficientemente.

### 149.4 — Materialização da classificação

**A.4.1 — Se ADR**: criar
`00_nucleo/adr/typst-adr-0058-<slug>.md` (ou 0059 se duas).

Cabeçalho canónico (P145):

```markdown
# ⚖️ ADR-0058: <título>

**Status**: `EM VIGOR`   <!-- ou IMPLEMENTADO conforme natureza -->
**Data**: 2026-04-24
**Validado**: Passo 149 — arqueologia + formalização.
```

Estrutura (~80–120 linhas):
- Contexto (resumo de 149.1+149.2).
- Decisão (forma actual codificada).
- Alternativas consideradas (forma vanilla; outras hipóteses).
- Consequências (positivas/negativas/neutras).
- Referências (ADR-0026, 0033, 0036; passos materializadores).

Se classificação for "EM VIGOR" (regra geral) → considerar
adicionar a "Meta-regras em vigor" do README dos ADRs. Se
classificação for "IMPLEMENTADO" (decisão técnica concreta)
→ apenas tabela.

**A.4.2 — Se DEBT**: adicionar a `DEBT.md` Secção 1.

Próximo número: confirmar em 149.1.A.0 ou pelo último DEBT
em uso. Candidato `DEBT-53` (mencionado em relatórios
anteriores como "candidato XL para rustybuzz") — verificar
se já está aberto. Se aberto: usar `DEBT-54`. **Não
re-purposing**.

Estrutura:

```markdown
## DEBT-NN — <título> — EM ABERTO (Passo 149)

**Aberto em**: Passo 149 (2026-04-24) durante arqueologia
do inventário 148.

### Contexto
[resumo da forma actual]

### Diferença face ao vanilla
[o que vanilla tem; o que cristalino tem]

### Razão pela escolha actual
[se conhecida da arqueologia; "sem registo" se não]

### Plano (se aplicável)
[ou "sem plano definido"]

### Critério de fecho
[o que materializaria]
```

**A.4.3 — Se referência cruzada apenas**: adicionar nota ao
inventário 148 + entrada no relatório 149. Sem ADR; sem
DEBT.

### 149.5 — Actualização do inventário 148

Em
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- Tabela B (arquitectural): linhas de `Value::Type` e
  `Value::Args` ganham coluna "Referência" com ADR-0058 /
  ADR-0059 / DEBT-NN.
- Top divergências (§7) — entradas 1 e 2 ganham nota:
  "**Resolvido (ou abrangido) por Passo 149**: ver ADR-NNNN
  / DEBT-NN".

### 149.6 — Actualização do README dos ADRs

(Se ADRs novas foram criadas em 149.4.)

- Tabela "Estado por ADR": entradas novas para
  ADR-0058/0059.
- Distribuição de status: recalcular.
- Total: 57 → 58 ou 59.
- Entrada em "Passos-chave da história dos ADRs" para
  Passo 149.

### 149.7 — Actualização menor de §9 dos documentos de paridade

Em `typst-paridade-plano-medicao.md`:

```diff
- 1. **Passo 149** — Implementar `frame_dto.rs` ...
+ 1. **Passo 149** — Arqueologia + formalização de
+    `Value::Type` e `Value::Args` (passo administrativo
+    pós-inventário 148).
+ 2. **Passo 150** — Implementar `frame_dto.rs` com
+    `LayoutTolerance` e modo `text_content=true`.
+    Corpus filtrado pelo subconjunto declarado no
+    inventário 148 (actualizado pelo Passo 149).
```

(Renumeração de itens 2/3/4/5 para 3/4/5/6.)

### 149.8 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-149-relatorio.md`.

Secções:
1. Sumário executivo.
2. Inventário factual (149.1).
3. Arqueologia: passos materializadores + razões (149.2).
4. Decisão de classificação por divergência (149.3).
5. Outputs criados:
   - ADRs novas (se houver) — referência ao ficheiro.
   - DEBTs novos (se houver) — referência à entrada.
   - Referências cruzadas apenas (se houver).
6. Inventário 148 actualizado (149.5).
7. README dos ADRs actualizado (se aplicável; 149.6).
8. §9 dos documentos de paridade actualizado (149.7).
9. Próximo passo: 150 (`frame_dto.rs`).
10. Verificação final.

---

## Verificação

1. ✅ Inventário factual produzido (149.1) — formas actuais
   de cristalino + vanilla documentadas.
2. ✅ Arqueologia executada (149.2) — passos materializadores
   identificados; razões registadas (ou ausência declarada).
3. ✅ Classificação justificada para cada divergência (149.3).
4. ✅ Outputs materializados (ADR e/ou DEBT e/ou nada,
   conforme classificação).
5. ✅ Inventário 148 actualizado com referências cruzadas.
6. ✅ README dos ADRs actualizado se ADRs criadas.
7. ✅ §9 dos documentos de paridade renumerado (passo 149 =
   arqueologia; passo 150+ = materialização).
8. ✅ Nenhum ficheiro de código tocado em L1/L2/L3/L4.
9. ✅ `cargo test --workspace --lib` inalterado (1113).
10. ✅ `crystalline-lint .` zero violations.
11. ✅ Cabeçalho canónico aplicado a qualquer ADR nova
    (P145).
12. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. Cada divergência (`Value::Type`, `Value::Args`) está
   classificada com justificação registada.
2. Outputs (ADR/DEBT/cruzada) coerentes com classificação.
3. Inventário 148 reflecte os outputs.
4. Próximo passo (150) tem âncora documental clara.
5. Sem materialização de código; sem alteração de
   `Value::*`.
6. Relatório do passo escrito.

---

## O que pode sair errado

- **Git history não disponível**: alternativa via
  `grep` em `00_nucleo/materialization/`. Se mesmo isso falha
  (passo materializador muito antigo, sem relatório
  detalhado), declarar "passo materializador desconhecido"
  como facto. Arqueologia é tentativa, não garantia.

- **Razão arqueológica é "esqueci-me / parecia certo"**:
  válido. Classificação cai em "ADR formaliza decisão de
  facto" — ADR registar a forma como aceitável sem
  reabertura, dado que tests passam (1113) e perfil
  observacional graded (ADR-0054) cobre.

- **`Value::Type` é mais sub-óptimo do que parecia**: ex,
  arqueologia revela que a string-based perde funcionalidade
  útil (reflexão de tipos em runtime). Classificação cai em
  DEBT com plano. Plano fica vago ("expandir quando
  necessário") — aceitável, é DEBT comum.

- **`Value::Args` é trivial**: arqueologia revela que
  ADR-0036 (atomização) já cobre. Resultado: **referência
  cruzada apenas**, sem ADR nova. Aceitável.

- **Ambas as divergências resolvem por referência cruzada**:
  passo termina com 0 ADRs novas, 0 DEBTs novos. Aceitável.
  Output é actualização do inventário 148 + relatório.

- **Arqueologia revela que uma das divergências não é
  divergência**: ex, `Value::Type` é igualmente representado
  em vanilla mas inventário 148 §7 erradamente o classificou.
  Corrigir o inventário; sem ADR/DEBT.

- **Surge terceira divergência durante arqueologia**: ex,
  `Value::Symbol`/`Value::Decimal` ausentes. **Não atacar**
  neste passo; registar como entrada para futuro passo de
  arqueologia análogo (149bis ou 149.5 — decidir mais
  tarde).

- **Conflito de numeração com `DEBT-53`**: relatórios
  anteriores referem `DEBT-53` como "candidato XL para
  rustybuzz". Se ainda não está aberto, **disponível**. Se
  aberto, usar próximo número. Verificar em 149.1.A.0.

- **Critério "ADR" vs "DEBT" ambíguo na prática**: se a
  classificação não bater claramente em nenhuma linha do
  critério, registar a ambiguidade no relatório e escolher
  conservadoramente: **ADR** (formaliza estado actual sem
  obrigação de mudar), com nota "se evidência futura
  surgir, considerar abertura de DEBT relacionada".

- **Tabela B do inventário 148 cresce em complexidade**:
  cada referência cruzada adiciona ruído visual. Mitigação:
  notas de rodapé compactas em vez de coluna nova. Decidir
  em 149.5.

---

## Notas operacionais

- **Modelo: passo arqueológico**. Diferente de
  diagnóstico-primeiro (131A/132A/140A/148): aqui o
  trabalho é **olhar para o passado** para entender uma
  decisão, não inventariar o presente para preparar o
  futuro. Subtipo de passo administrativo.

- **Output incerto à partida**. Pode ser 0 ADRs / 0 DEBTs
  (referência cruzada), 1 ADR / 0 DEBT, 2 ADRs / 0 DEBT, 1
  ADR / 1 DEBT, 0 ADR / 2 DEBTs, etc. **6 cenários
  plausíveis**. Aceite como propriedade do passo.

- **"Sem registo de razão" é resultado válido**.
  Documenta-se como facto. Não é falha do passo.

- **ADR de divergência intencional não-formalizada antes
  é precedente**. Se ADRs forem criadas, contam para o
  total (57 → 58/59). README ADR + tabela actualizam.

- **DEBT-53 candidato**: relatórios 142/144/146 referiram
  "DEBT-53 candidato XL para rustybuzz integration".
  Verificar se já foi aberto. Se aberto, é o número usado;
  se não, este passo pode (ou não) ocupá-lo. Verificação
  em 149.1.A.0.

- **Pós-149**: inventário 148 fica com **referências
  canónicas** completas para todas as divergências de §7.
  Próximo passo (150) materializa medição com confiança no
  denominador.

- **Reformulação da série paridade**:
  - 148 — inventário declarado.
  - **149** — arqueologia + formalização (este).
  - 150 — `frame_dto.rs` + matriz P3.
  - 151+ — restantes (ValueDTO, pdf_compare).
  
  Esta sequência tem 2 reformulações já: original era
  148 = `frame_dto.rs`. Cada reformulação foi justificada
  por descoberta de pré-condição não satisfeita. Padrão
  pode repetir-se em 150 — aceitável.

- **Possibilidade de meta-passo**: se a frequência de
  reformulação aumenta (3+ vezes), considerar passo
  administrativo de "consolidação de roadmap série
  paridade". Não aberto neste passo.

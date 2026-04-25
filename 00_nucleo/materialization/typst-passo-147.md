# Passo 147 — Actualização dos documentos de paridade

**Série**: 147 (passo **L0-puro / administrativo**; consolidação
de documentos `PROPOSTO` desactualizados antes de
materialização em 148).
**Precondição**: Passo 146 encerrado; multi-font materializado;
1113 tests; zero violations; 57 ADRs; 10 DEBTs abertos;
DEBT-1 + DEBT-52 fechados; ADR-0055 anotada
(IMPLEMENTADO preservado); ADR-0057 IMPLEMENTADO.

**Numeração**: 147 (seguinte natural após 146).

**Documentos no escopo**:
1. `00_nucleo/diagnosticos/typst-paridade-definicoes.md`
2. `00_nucleo/diagnosticos/typst-paridade-plano-medicao.md`

(Localização **provisória** — confirmar em 147.1 se vivem em
`diagnosticos/`, `prompts/`, `relatorios/`, ou directório novo.
Decisão guia a actualização das referências cruzadas.)

**Natureza**: passo **L0-puro / administrativo**. **Zero
código**. **Zero testes**. **Zero ADRs criadas, revogadas ou
revisadas**. Apenas reescrita de 2 documentos `PROPOSTO` para
reflectir o estado real do projecto em 2026-04-24 pós-Passo
146.

**ADRs aplicáveis**:
- Nenhuma directamente. O passo executa actualização
  factual; a decisão arquitectural sobre a infra `lab/parity`
  já está implícita nos documentos (continuam `PROPOSTO`
  pós-actualização — materialização em 148).

---

## Contexto

Os documentos foram escritos com contexto desactualizado — o
**Contexto** declarado é "Passos 19-21 (layout, início de
export PDF)". O projecto está em **Passo 146**, com:

- Pipeline end-to-end (parse → eval → layout → export)
  funcional desde Passo 19 (layout) e Passo 22+ (export
  PDF).
- DEBT-1 fechado em 142 com perfil observacional graded
  (ADR-0054).
- DEBT-52 fechado em 142; gaps 7 (hyphenation) e 5/6
  (font wiring + array fallback) materializados (Passos
  140B/141/144).
- Multi-font per document materializado em 146.
- 57 ADRs; 10 DEBTs abertos.

**Discrepâncias materiais** entre documentos e realidade
identificadas no diálogo prévio:

1. "**§Contexto**: Passos 19-21 (layout, início de export
   PDF)" — desactualizado por ~127 passos.
2. "**§2 Falta**: Pipeline end-to-end em L1" — realidade:
   pipeline em **L3** (`03_infra/src/pipeline.rs`); eval e
   layout em L1.
3. "**§7 Ligação com a sequência de passos**": tabela inteira
   refere passos passados (19, 21+) como futuros. Reescrita
   é necessária.
4. "**§2 Existe**: Pipeline end-to-end em L1 (Passo 19):
   parse → eval → layout produz `PagedDocument`" — correcto
   em facto histórico (existia em P19) mas a localização
   "L1" colide com pipeline actual em L3.
5. "**§9 Próximas acções**": refere "depois do Passo 21" —
   completamente desactualizado.

Adicionalmente, alguns aspectos **factuais** dos documentos
permanecem válidos:

- Tipos `CompactNode`, `ValueDTO`, `FrameDTO` ainda não
  existem (verificável; nenhum relatório posterior menciona
  materialização).
- Corpus `lab/parity/corpus/` é provavelmente o do P9 (~11
  ficheiros) — nenhum passo posterior expandiu.
- Quatro níveis (P1/P2/P3/P4) e o vocabulário ("paridade
  funcional vs implementação") são consistentes com ADRs
  vigentes (ADR-0033 paridade funcional; ADR-0026 rejeição de
  paridade de implementação; ADR-0054 perfil observacional
  graded).

Este passo **actualiza** o que é factual e desactualizado;
**preserva** o que é decisão arquitectural ainda válida.

---

## Objectivo

Ao fim do passo:

1. **`typst-paridade-definicoes.md`** revisto:
   - Remover `**Estado**: PROPOSTO` se aplicável (campo
     ausente no documento; verificar) ou actualizar para
     `**Status**: \`PROPOSTO\`` (vocabulário canónico do
     P145).
   - Datar com `2026-04-24` (data deste passo).
   - Secções específicas (P1/P2/P3/P4) preservadas onde
     factualmente válidas.
   - Referências a "Passo 9" (CompactNode), "Passo 14"
     (NaN diagnóstico) e outras passadas: preservadas
     (são histórico imutável).
   - Aviso ao topo declarando "Documento escrito durante a
     fase precoce do projecto; revisto no Passo 147 para
     reflectir estado pós-Passo 146 (DEBT-1 fechado,
     pipeline end-to-end estável)."

2. **`typst-paridade-plano-medicao.md`** revisto:
   - Status canonizado.
   - **§Contexto** reescrito reflectindo estado real (146
     passos, perfil observacional graded, etc.).
   - **§2 "O que já existe e o que falta"** reescrito com
     inventário factual de 2026-04-24:
     - Em "Existe": acrescentar pipeline end-to-end em L3
       (`compile_to_pdf_bytes`); export multi-font (Passo
       146); hyphenation (Passo 144); single-font + array
       fallback (Passos 140B/141).
     - Em "Falta": preservar `ValueDTO`, `FrameDTO`,
       `pdf_compare`, corpus expandido. Remover ou ajustar
       afirmações sobre P1/P3 já materializados.
   - **§3 Estrutura proposta**: preservada
     conceptualmente; ajustar ficheiros listados se a
     realidade do `lab/parity/` actual divergir.
   - **§7 "Ligação com a sequência de passos"**: tabela
     reescrita. Coluna "Quando" passa a referir passos
     **futuros** (148+) e situações **actuais** (não
     passos passados como futuros).
   - **§9 "Próximas acções concretas"**: reescrito
     com base no estado de 2026-04-24. Materialização
     concreta passa para o Passo 148.

3. **Localização canónica** dos dois documentos confirmada
   ou ajustada:
   - Se vivem em `00_nucleo/diagnosticos/`: aceitável (são
     factuais/operacionais).
   - Se vivem em `00_nucleo/prompts/`: revisitar — não são
     prompts L0 de código (não têm hash, não governam
     código L1/L2/L3/L4).
   - Se vivem fora de `00_nucleo/`: confirmar.
   - Decisão default: manter onde estão; mover só se
     análise de 147.1 revelar inconsistência clara.

4. **Sem novo ADR**. Os documentos continuam `PROPOSTO`
   após este passo. A decisão de promoção a `EM VIGOR` /
   `IMPLEMENTADO` fica para o Passo 148 (materialização)
   ou para passo separado de promoção.

5. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-147-relatorio.md`
   com diffs das principais reescritas.

Este passo **não**:

- Implementa `ValueDTO`, `FrameDTO`, `pdf_compare`, ou
  qualquer infraestrutura nova em `lab/parity/`. **Tudo isso
  é Passo 148**.
- Cria ADRs sobre paridade.
- Toca código em L1, L2, L3, L4.
- Toca `lab/parity/` (apenas referencia para inventário).
- Toca testes.
- Toca `DEBT.md` ou `00_nucleo/adr/README.md` (a não ser que
  inventário 147.1 revele inconsistência directa — improvável
  para este escopo).

---

## Decisões já tomadas

1. **Documentos preservam `PROPOSTO`** (ou status equivalente)
   após actualização. A actualização é **factual**, não
   transição de estado.
2. **Sem ADR formal**. ADR-0033 (paridade funcional) e
   ADR-0034 (diagnóstico obrigatório) cobrem a base; uma
   ADR específica sobre "infra de medição de paridade" só
   faz sentido quando a infra é materializada (148) ou se
   houver decisão arquitectural que justifique.
3. **Conteúdo conceptual preservado**. Os 4 níveis (P1-P4),
   os DTOs propostos, os modos de comparação (text_content /
   structural / geometric), a tolerância configurável — tudo
   isto **permanece**. A actualização ataca **discrepâncias
   de contexto temporal**, não estrutura.
4. **Aviso explícito** no topo de cada documento sobre a
   revisão. Análogo ao "Aviso sobre vocabulário em documentos
   históricos" do README dos ADRs — preserva integridade do
   trabalho original sem o iludir.

## Decisões diferidas (resolvidas neste passo)

5. **Localização canónica final** dos documentos: confirmar em
   147.1.
6. **Status canónico exacto** a aplicar: `PROPOSTO` é a
   escolha esperada (a infra ainda não existe). Confirmar
   que o vocabulário do P84.8g + P145 cobre estes documentos
   (não são ADRs em sentido estrito; convenção de status
   pode ser **livre** ou **canónica**).
7. **Localização do corpus expandido**: documentos propõem
   `lab/parity/corpus/{markup,math,code,semantic,visual}/`.
   Confirmar em 147.1 que `lab/parity/corpus/` actual existe
   e identificar o que tem hoje. Materialização do corpus
   expandido é Passo 148.
8. **Reescrita do `§Contexto`**: forma exacta resolvida em
   147.2. Esboço:
   > "O projecto tem 146 passos executados. Pipeline
   > end-to-end (parse → eval → layout → export PDF) está
   > estável desde o Passo 22+; multi-font, hyphenation e
   > consumer integral de StyleDelta (DEBT-52 6/8 gaps
   > materializados) existem. ADR-0033 (paridade funcional) e
   > ADR-0054 (perfil observacional graded) governam o que
   > 'paridade' significa. A pergunta 'em que percentual de
   > paridade estamos?' continua sem resposta numérica
   > porque o projecto continua a medir testes acumulados,
   > não paridade comparativa."

---

## Escopo

**Dentro**:

- Inspecção dos dois documentos em estado actual.
- Reescrita das secções desactualizadas:
  - `§Contexto` em ambos.
  - `§2 "O que já existe e o que falta"` no plano de
    medição.
  - `§7 "Ligação com a sequência de passos"` no plano.
  - `§9 "Próximas acções"` no plano.
  - Outras passagens com referências temporais
    desactualizadas (varrer ambos os documentos).
- Adição de aviso/nota de revisão no topo de cada documento.
- Confirmação ou ajuste de localização canónica.
- Verificação de referências cruzadas entre os 2
  documentos e para outros (ADRs, relatórios anteriores).
- Escrita do relatório do passo.

**Fora**:

- Implementação de `ValueDTO`, `FrameDTO`, `pdf_compare`,
  `report.rs`, `eval_parity.rs`, `layout_parity.rs`,
  `export_parity.rs`. **Tudo Passo 148**.
- Expansão do corpus `lab/parity/corpus/`. **Passo 148**.
- Geração de primeiro relatório `lab/parity/reports/latest.md`.
  **Passo 148**.
- Decisão sobre se o corpus oficial do Typst entra no repo.
  **Adiada para 148 ou ADR separada**.
- Decisão sobre se a métrica entra no `crystalline-lint`.
  **Adiada**.
- Modificação de ADRs existentes.
- Modificação do README dos ADRs.

---

## Sub-passos

### 147.1 — Inventário e localização

**A.1.1 — Localizar os ficheiros**:

```bash
find 00_nucleo -name "typst-paridade-*.md" -type f
find lab/parity -name "*.md" -type f
ls -la 00_nucleo/diagnosticos/ 00_nucleo/prompts/ 00_nucleo/relatorios/ 2>/dev/null | grep -i paridade
```

Registar localização real. Esperado: `00_nucleo/diagnosticos/`
ou `00_nucleo/prompts/` ou raiz de `00_nucleo/`.

**A.1.2 — Inspeccionar `lab/parity/` actual**:

```bash
ls -la lab/parity/
ls lab/parity/src/ lab/parity/tests/ lab/parity/corpus/ lab/parity/reports/ 2>/dev/null
cat lab/parity/Cargo.toml 2>/dev/null | head -30
```

Registar:
- Que ficheiros existem em `lab/parity/src/`. Esperado:
  `main.rs`, `compact.rs`, talvez outros.
- Que tests existem em `lab/parity/tests/`. Esperado
  `parse_parity.rs`. Se há outros: documentar.
- Estrutura de `corpus/`: subdirs e contagem de ficheiros.
- Existe `reports/`? Tem ficheiros?

Esta inspecção alimenta a actualização do `§2 "O que já
existe e o que falta"` do plano.

**A.1.3 — Localização canónica decidida**:

Manter no directório actual a menos que haja inconsistência
clara. Se mover for necessário (ex: estavam em `prompts/`
mas não são prompts L0), documentar a decisão no relatório.

### 147.2 — Reescrita do `§Contexto` em ambos

**Plano de medição**: substituir parágrafo de contexto pelo
esboço de "Decisão diferida 8" acima. Forma final ajustada
para coerência com o resto do documento.

**Definições operacionais**: documento não tem `§Contexto`
explícito; tem `§Inputs`/`§Função sob teste` por nível.
Adicionar nota de revisão no topo (ver 147.5).

### 147.3 — Reescrita do `§2 "O que já existe / falta"`

(Plano de medição apenas — definições não tem secção
equivalente.)

**"Existe"** actualizado:

- `lab/parity/` com `Cargo.toml` separado do workspace
  cristalino — preservar.
- `CompactNode` DTO para comparação estrutural — preservar
  (Passo 9 v2/v3).
- `assert_paridade()`/`_math()`/`_code()` para parsing —
  preservar.
- Corpus mínimo em `lab/parity/corpus/` — actualizar
  contagem se inventário 147.1 revelar > 11.
- Runner interactivo `cargo run --manifest-path
  lab/parity/Cargo.toml -- <input>` — preservar.
- **Adicionar**: pipeline end-to-end (parse → eval → layout
  → export) em L1 + L3 (eval/layout em L1; export em L3
  via `compile_to_pdf_bytes`).
- **Adicionar**: export PDF com multi-font (Passo 146);
  hyphenation (Passo 144); consumer integral de StyleDelta
  para 9/10 campos (DEBT-52 6/8 gaps).

**"Falta"** actualizado:

- `ValueDTO` para P2 — preservar (não materializado).
- `FrameDTO` para P3 — preservar.
- `pdf_compare` para P4 — preservar.
- Corpus expandido para semantic/ e visual/ — preservar.
- Métrica agregada — preservar.
- Painel/relatório — preservar.

### 147.4 — Reescrita do `§7 "Ligação com a sequência de passos"`

Tabela reescrita:

```diff
- | Quando | O que fazer |
- |--------|-------------|
- | Final de cada passo de migração | Correr a suite P1; reportar contagem |
- | Quando `eval()` produz Value real (Passo 17 já foi) | Activar P2; expandir corpus semantic/ |
- | Quando layout() produz Frame real (Passo 19, actual) | Activar P3 com tolerance text_content=true |
- | Quando export_pdf() produz PDF aberto por leitor (Passo 20+) | Activar P4 modo Opção B |
- | Após embedding de fontes (Passo 21+) | Activar P4 modo Opção A |
- | A cada N passos | Gerar relatório histórico |
+ | Estado em 2026-04-24 (Passo 146) | Pipeline end-to-end estável; eval, layout, export PDF (multi-font + hyphenation) materializados. Apenas P1 está medido. |
+ | Passo 148 | Materializar `FrameDTO` + tests P3 (`text_content` mode). Gerar primeiro relatório agregado. |
+ | Passo 149+ (futuro) | Materializar `ValueDTO` + tests P2. |
+ | Passo 150+ (futuro) | Materializar `pdf_compare` + tests P4 (Opção B textual primeiro; Opção A visual quando dependências CI estiverem decididas). |
+ | A cada N passos | Gerar relatório histórico em `lab/parity/reports/history/`. |
```

(Numeração 148/149/150 é **indicativa**, não compromisso. Se
materialização cresce, ramifica em sub-passos.)

### 147.5 — Aviso de revisão no topo

Adicionar no topo de cada documento (após o título principal):

```markdown
> **Revisto no Passo 147 (2026-04-24)**: o documento original
> foi escrito durante a fase precoce do projecto (referências
> a "Passos 19-21" eram contemporâneas). O projecto está
> agora em Passo 146; pipeline end-to-end estável; DEBT-1
> fechado por ADR-0054 (perfil observacional graded).
> Decisões conceptuais do documento (4 níveis P1-P4, DTOs
> propostos, modos de comparação) **permanecem válidas** e
> são alvo de materialização em Passo 148. Discrepâncias
> de contexto temporal foram corrigidas neste passo.
```

### 147.6 — Reescrita do `§9 "Próximas acções"`

```diff
- ## 9 — Próximas acções concretas
-
- Em ordem:
-
- 1. Implementar `frame_dto.rs` com `LayoutTolerance` e modo `text_content=true`
- 2. Adicionar `tests/layout_parity.rs` com o corpus actual (11 ficheiros, todos devem passar em conteúdo)
- 3. Gerar primeiro relatório `lab/parity/reports/latest.md` — número-base
- 4. Decidir corpus: oficial, próprio, ambos. Documentar em ADR separado se a decisão for não-trivial
- 5. Implementar P2 quando eval() suportar mais features (depois do Passo 21)
- 6. Implementar P4 Opção B quando export_pdf() estiver estável
+ ## 9 — Próximas acções concretas
+
+ Pós-Passo 147 (este — actualização). A ordem permanece a do
+ documento original; apenas as condições temporais foram
+ actualizadas para o estado real.
+
+ 1. **Passo 148** — Implementar `frame_dto.rs` com
+    `LayoutTolerance` e modo `text_content=true`. Adicionar
+    `tests/layout_parity.rs` com o corpus actual. Gerar
+    primeiro relatório agregado em `lab/parity/reports/latest.md`.
+ 2. **Passo 149+** — Implementar P2 (`value_dto.rs` +
+    `tests/eval_parity.rs`) quando expansão do corpus
+    semantic/ for priorizada.
+ 3. **Passo 150+** — Implementar P4 Opção B (textual) com
+    `pdf_compare.rs` + `tests/export_parity.rs`. Opção A
+    (visual) quando dependências CI (`pdftoppm`/`mupdf`)
+    forem decididas.
+ 4. **Decisão sobre corpus oficial vs próprio**: documentar
+    em ADR separada se decisão não-trivial. Provável mix
+    (ambos) com filtro por features. Alvo: durante Passo 148
+    ou imediatamente antes.
+
+ A partir do Passo 148, o utilizador passa a ter o número
+ que pediu — em formato de matriz, não de percentual único.
```

### 147.7 — Verificação de referências cruzadas

`grep` em todos os documentos do projecto por menções a:

```bash
grep -rn "typst-paridade-definicoes\|typst-paridade-plano-medicao" 00_nucleo/ lab/parity/
```

Se algum documento referencia estes pelos paths antigos, e
147.1.A.3 decidiu mover, **actualizar**. Se a localização
não muda, nada a fazer.

Adicionalmente, varrer os 2 documentos para referências
internas:

```bash
grep -E "Passo [0-9]+|ADR-[0-9]+|DEBT-[0-9]+" \
  00_nucleo/diagnosticos/typst-paridade-*.md
```

Verificar que cada ADR/Passo/DEBT mencionado é factualmente
válido. Se documento menciona ADR que não existe (ex:
"ADR-0001 ~10 passos"), preservar como facto histórico (ADR
existe; o ~10 passos era estimativa antiga).

### 147.8 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-147-relatorio.md`.

Secções:

1. Sumário executivo.
2. Inventário pré-actualização (resultado de 147.1):
   localização real dos documentos; estado real de
   `lab/parity/` (ficheiros existentes, contagens de corpus).
3. Discrepâncias detectadas (lista canonizada das
   identificadas pré-passo + novas que apareçam).
4. Reescritas aplicadas (diffs por secção; preferir mostrar
   secções inteiras antes/depois quando o diff for material).
5. Aviso de revisão acrescentado.
6. Confirmação de referências cruzadas (em ambos os
   documentos + projecto).
7. Decisão de localização canónica (preservada ou movida).
8. Próximo passo: 148 (materialização).
9. Verificação final.

---

## Verificação

1. ✅ `§Contexto` (ou equivalente) actualizado em ambos os
   documentos.
2. ✅ `§2 "O que já existe / falta"` reescrito no plano de
   medição.
3. ✅ `§7 "Ligação com a sequência de passos"` reescrito.
4. ✅ `§9 "Próximas acções"` reescrito.
5. ✅ Aviso de revisão no topo de cada documento.
6. ✅ Decisões conceptuais (4 níveis, DTOs, modos)
   preservadas literalmente onde são válidas.
7. ✅ Status canónico aplicado (`PROPOSTO` ou equivalente).
8. ✅ Localização canónica confirmada ou ajustada.
9. ✅ Referências cruzadas verificadas.
10. ✅ Relatório do passo escrito.
11. ✅ Nenhum ficheiro em `lab/parity/` tocado.
12. ✅ Nenhum ficheiro em `01_core/`, `02_shell/`,
    `03_infra/`, `04_wiring/` tocado.
13. ✅ `cargo test --workspace --lib`: inalterado (1113).
14. ✅ `crystalline-lint .`: zero violations.
15. ✅ Sem ADR criada; sem DEBT modificado.

---

## Critério de conclusão

1. Documentos coerentes com estado de 2026-04-24.
2. Decisões conceptuais preservadas.
3. Aviso de revisão presente.
4. Próximo passo (148) tem âncoras documentais correctas.
5. Sem materialização de infra (escopo de 148).
6. Relatório do passo escrito.

---

## O que pode sair errado

- **Documentos vivem em local inesperado** (ex: `lab/parity/`
  raiz, ou `00_nucleo/relatorios/`): inventário 147.1
  resolve. Se a localização exigir mudança, registar a
  decisão e o motivo.

- **`lab/parity/` actual é mais avançado do que o
  documento sugere**: ex: alguém materializou `value_dto.rs`
  num passo paralelo que eu não tenho contexto. Se 147.1
  detectar isto, **pausar** e pedir esclarecimento — a
  reescrita teria de cobrir trabalho não documentado.

- **`lab/parity/` actual é menos do que o documento
  declara como "existe"**: ex: documento diz "11 ficheiros
  no corpus" mas só há 5. Actualizar a contagem com a
  realidade. Documentar no relatório como "discrepância
  herdada".

- **Documento referencia features que não existem**: ex:
  "modo `geometric` configurável por ficheiro". Se o
  modo é só conceptual (não materializado), preservar como
  proposta. Se o modo é uma referência a código real que
  diverge: investigar e ajustar.

- **Aviso de revisão entra em conflito com convenção
  do projecto**: convenção do README dos ADRs é "documentos
  imutáveis em `relatorios/`/`materialization/`/`context/`".
  Estes documentos estão em **`diagnosticos/`** (não
  imutável). Edição é permitida. Confirmar que a localização
  permite edição antes de tocar.

- **Discrepância entre status canónico do projecto
  (P84.8g/P145) e o usado pelos documentos**: documento usa
  `**Estado**:` (forma antiga). Migrar para `**Status**:` em
  ambos. Análogo ao P145 mas para documentos não-ADR.

- **Ambos os documentos referenciam `frame_dto.rs` mas
  realidade tem `value_dto.rs` ou outro nome**: padronizar
  pela realidade. Se a realidade não tem nada, preservar a
  proposta literal.

- **Conflito entre `lab/parity/` ser "fora do workspace
  cristalino" e crystalline-lint cobrir o repo todo**:
  documento declara que `lab/parity/` é separado. Confirmar
  com `crystalline.toml` ou config equivalente. Se há
  inconsistência, regista-se mas não corrige aqui (é
  decisão arquitectural separada).

- **Documentos têm secções que não foram listadas no escopo
  mas estão desactualizadas**: varrer integralmente em
  147.7 e adicionar reescritas conforme necessário.
  Crescimento do passo é aceite — natureza administrativa
  não tem teto rígido.

---

## Notas operacionais

- **Modelo: passo administrativo análogo a P143 (correcção
  do README dos ADRs) e P145 (uniformização de cabeçalhos)**.
  Ambos consolidam dívida documental. P147 é o terceiro do
  género.

- **Status `PROPOSTO` preservado**. Os documentos continuam
  propostas; materialização em 148 promove para `IMPLEMENTADO`
  (ou status equivalente) **só** se o conteúdo do documento
  for tornado realidade. P147 não promove status.

- **Sem ADR sobre paridade**. ADR-0033 (paridade funcional) e
  ADR-0054 (perfil observacional graded) cobrem a base
  conceptual. Uma ADR específica sobre "infra de medição
  de paridade" pode aparecer durante ou depois de 148, se
  a decisão arquitectural justificar — não obrigatória.

- **Numeração 148/149/150 é indicativa**. Se materialização
  cresce em complexidade, ramifica-se. P148 pode tornar-se
  P148A (diagnóstico, se aplicável) + P148B
  (materialização). Decisão na altura.

- **Decisões arquitecturais preservadas**:
  - 4 níveis P1-P4 alinhados com pipeline.
  - DTOs neutros (`ValueDTO`, `FrameDTO`) para comparação
    cross-tipo.
  - Modos de comparação configuráveis (`text_content`,
    `structural`, `geometric`).
  - Tolerância numérica configurável (P3 e P4).
  - Corpus categorizado por features.
  - Métrica como matriz, não percentual único.

  Estes elementos são **input directo** para 148 — sem
  redefinição.

- **Conflito potencial com ADR-0054**: ADR-0054 declara
  perfil "observacional graded" com tolerâncias implícitas.
  Os documentos propõem tolerâncias **explícitas e
  configuráveis** (`absolute_pt`, `max_pixel_diff`,
  `max_diff_ratio`). Compatível: ADR-0054 governa o que
  conta como paridade arquitecturalmente; os documentos
  operacionalizam a medição. Sem revisão de ADR-0054
  necessária.

- **Sem mudança no `crystalline-lint`**. Documentos vivem
  fora de `00_nucleo/prompts/`; não têm `@prompt-hash`; não
  activam V5 (`PromptDrift`). Verificação 147 passa por
  `crystalline-lint .` por consistência mas não há ligação
  directa.

- **Pós-147**: documentos coerentes; 148 materializa.
  Caminho directo para a métrica que o utilizador pediu
  ("em que paridade estamos?") concretiza-se em 148.

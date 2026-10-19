# Passo P159B — Diagnóstico expansão série 159 e tecto realista Model

Passo diagnóstico precedendo materialização. **Não materializa
código**. Inventaria **todo o subset de refinos pendentes da
família 159** (hayagriva, form variants, CSL parsing,
numbering schemes, cross-document forward refs) + outros
refinos Model pendentes (supplement por lang em figure-kinds)
para decidir até onde dá para ir com Model **sem entrar noutro
módulo**.

**Quarta aplicação concreta de ADR-0065 critério #5** (scope
determinado por inventário) após P157, P158, P159. Diferenciador
deste diagnóstico vs anteriores: scope mais amplo. P157/P158/P159
inventariaram uma feature concreta cada; P159B inventaria
**toda a expansão pendente da série 159** + outros refinos Model
para definir tecto realista do módulo.

Análoga à análise "fechamos Layout só mantendo o ritmo?"
realizada anteriormente, mas para Model — com base factual
adicional acumulada na sessão.

---

## Estado actual antes de começar

- 63 ADRs após P159A (28 EM VIGOR; ADR-0060 IMPLEMENTADO;
  ADR-0062 reserva sem ficheiro confirmada).
- Layout: 78% (inalterado). Cobertura arquitectural total 82%.
- Cobertura Model agregada: ~50% (impl + impl⁺ inalterada).
  Cobertura ampla cresceu (24 entradas com valor cumulativo).
- Hash actual `entities/content.rs`: `ec58d849` (preservado em
  9 passos consecutivos com interpretação L0-baseada).
- Hash `bib_entry.rs`: `5a2c0ebd` (P159A).
- 1412 tests (lib+integ+diagnostic; workspace 1434); zero
  violations linter.
- 58 variants Content; 48 stdlib funcs.
- Padrões consolidados pós-P159A: granularidade N=14;
  inventariar N=14; Smart→Option Caso A patamar N=5; §análise
  risco N=14; estabilidade hash L0 N=9.

**Direcções abertas pós-P159A** (relatório §8 lista 13
candidatas). P159B foca-se especificamente em Model — pendentes
declaráveis:

Família 159 (Bibliography + Cite refinos):
- Hayagriva integration + CSL parsing (XL; ADR-0062 a promover).
- Form variants Cite (Normal/Prose/Author/Year/etc.) (M).
- Numbering schemes (numérico, autor-ano simples) (M-M+).
- Cross-document forward refs (depende ADR-0017 promovida).

Família 158 (figure-kinds refinos):
- Supplement automático por lang (M).
- Show selectors `figure.where(kind:)` (depende ADR-0041 +
  show rules futuras).
- Refactor `kind: String → Option<String>` per ADR-0064 Caso A
  (XS).

Família 157 (table foundations refinos):
- Algoritmo placement Grid completo (DEBT-34e; depende M+/L
  refactor).
- Refinos table.cell/header/footer (align/stroke/fill/inset/
  breakable diferidos per ADR-0054 graded).

Outros Model:
- Promover ADR-0060 a R1 (administrativo XS).
- Actualizar L0 prompt content.md (administrativo XS).
- ADR meta XS de "ADR-0064 caso completion" (saturação).

**Política "sem novas reservas" estabelecida em P158**
mantida — P159B não cria reservas para passos pós-diagnóstico.

---

## Natureza do passo

**Tamanho**: M- (maior que diagnósticos S+ anteriores P157/
P158/P159 base por causa do scope ampliado).

**Justificação**: Inventário **multi-feature** + análise de
dependências cruzadas + decisão de tecto realista. Trabalho
documental puro. Sem modificação de código, sem ADR nova.

Granularidade preservada: 1 deliverable diagnóstico amplo →
mantém peso M- análogo a P156B (diagnóstico Layout originalmente
mais amplo que diagnósticos posteriores).

**Risco baixo**: passo previne risco em sub-passos seguintes
detectando dependências cruzadas cedo. Particularmente
importante para hayagriva (XL declarado) e numbering (potencial
dependência Introspection).

---

## Decisões já tomadas

- **Identificador P159B**: paridade com sub-passos da série
  159. P159B é **diagnóstico**; identificadores substantivos
  seguintes a decidir per inventário.
- **Natureza diagnóstica**: P159B inventaria, não materializa.
  Passos substantivos seguintes (P159C, P159D, etc., ou
  paragem da série 159 conforme decisão) redigidos após
  validação humana deste diagnóstico.
- **Sem código alterado**: passo puramente documental.
- **Sem ADR nova**: ADRs existentes (0017, 0033, 0054, 0060,
  0061, 0062 reserva) lidas, não criadas.
- **Sem novas reservas**: paridade política P158/A/P159/A.

## Decisões diferidas

- **Subset de Model materializável** dentro de scope 159 sem
  entrar noutro módulo: a decidir em §3.
- **Tecto realista de Model**: a decidir em §4 (análogo à
  análise Layout anterior).
- **Ordem de execução** dos sub-passos materializáveis: a
  decidir em §5.
- **Identificadores concretos** dos sub-passos seguintes
  (P159C, P159D, etc.; ou continuar com P160 fora de Model
  se inventário sugerir): a decidir em §5.

---

## Sub-passos

### .1 Inventário ADRs/DEBTs por família 159

Localizar e ler:
- **Família 159 (Bibliography + Cite)**:
  - DEBT-55 conteúdo completo (mencionado em P156B mas só
    sumarizado em P159).
  - ADR-0062 reserva — confirmar se conteúdo concreto está
    documentado em README ou apenas mencionado.
  - ADR-0017 detalhes sobre cite() cross-document.
  - ADRs sobre autorização de crate externa (0024/0023/0057
    citados em P159).
- **Família 158 (figure-kinds)**:
  - Diagnóstico P158 §3.3 (subset máximo supplement por lang).
  - ADR-0041 sobre show rules (impacto em show selectors).
  - DEBTs relacionados a i18n se existirem.
- **Família 157 (table foundations)**:
  - DEBT-34e detalhes (placement Grid; mencionado mas não
    documentado completo).
  - ADR-0054 detalhes graded para refinos diferidos.

Output: secção §1 do diagnóstico — mapa ADR/DEBT por família.

### .2 Inventário código pendente por família

Inspecção de `01_core/src/`:

Para cada família, identificar:
- Variants existentes que precisam de refino.
- Stdlib funcs existentes que precisam de extensão.
- Helpers que poderiam ser reusados.
- Hashes actuais dos ficheiros relevantes.

Específicos:
- **Família 159**: `BibEntry` actual (P159A); `Bibliography`
  e `Cite` variants; `extract_bib_entries` helper.
- **Família 158**: `Content::Figure` estrutura; `infer_kind_from_body`
  (P158A); padrão de lang em outras stdlib funcs (heading
  numbering tem lang? quote tem lang?).
- **Família 157**: `Content::Table`/`TableCell`/`TableHeader`/
  `TableFooter`; `layout_grid` actual; campos diferidos.

Output: secção §2 do diagnóstico — inventário código pendente.

### .3 Análise de dependências cruzadas

Para cada refino candidato, identificar:

a) **Dependência Introspection runtime (ADR-0017)**?
   - Cross-document forward refs: SIM (relatório P159 §1.3
     já confirmou).
   - Numbering autor-ano cross-references: provavelmente.
   - Supplement por lang: provavelmente NÃO (lang é local).
   - Form variants Cite: provavelmente NÃO.

b) **Dependência refactor multi-region (DEBT-56)**?
   - TableHeader/Footer.repeat real: SIM.
   - Bibliography paginada: provavelmente.

c) **Dependência crate externa**?
   - Hayagriva: SIM (define ADR-0062).
   - Outras crates: pouco provável neste scope.

d) **Dependência ADR pendente de promoção**?
   - ADR-0062 hayagriva.
   - ADR-0017 Introspection runtime.

e) **Dependência outro módulo (Layout/Introspection/Eval)**?
   - Algoritmo placement Grid (DEBT-34e): refactor extenso de
     `layout/grid.rs`.
   - Counters refinados: Introspection.

Output: secção §3 do diagnóstico — matriz de dependências
cruzadas.

### .4 Tecto realista de Model — sem entrar noutro módulo

Síntese de §1-§3 para responder:

1. Que refinos podem ser materializados **dentro de Model
   apenas** (sem tocar Layout/Introspection/Eval além do
   trivial)?
2. Que refinos exigem dependência cruzada hard?
3. Cobertura Model agregada (impl + impl⁺) atingível com
   refinos puramente Model: estimativa numérica.
4. Cobertura ampla (impl + impl⁺ + parcial) atingível.
5. Diferença entre tecto Model puro vs tecto pós-resolver
   dependências.

**Análogo à análise Layout anterior** ("fechamos Layout só
mantendo o ritmo?") mas com base factual: depois de P157/P158/
P159A, cobertura ampla Model está em 24 entradas com valor;
quantas mais são alcançáveis sem cross-módulo?

**Distinção operacional**:
- "Refino qualitativo" — move entrada de `parcial → puro` ou
  adiciona qualidades; não muda agregação.
- "Materialização nova" — adiciona variant/stdlib func novos;
  pode mover agregação se for entrada top-level.
- "Diferimento" — explicitar como ADR-0054 graded e parar.

Output: secção §4 do diagnóstico — análise de tecto Model.

### .5 Sequenciar sub-passos materializáveis

Com base em §4, ordenar refinos materializáveis dentro de Model:

- **Bloco A — Refinos sem dependência cruzada**: hayagriva-
  free, sem Introspection, sem refactor multi-region. Listar
  todos os candidatos M/M+/S+ aplicáveis.
- **Bloco B — Refinos com dependência ADR-0062 (crate
  externa)**: precedidos de promoção ADR-0062.
- **Bloco C — Refinos com dependência cross-módulo**: NÃO
  materializáveis em Model puro; documentar para sessão futura.

Para cada candidato em A:
- Identificador sugerido (P159C, P159D, P158B, P157D, etc.).
- Tamanho estimado.
- Subset minimal preservando granularidade.
- Tests Δ esperado.
- Hash impacto em `content.rs` (variant novo? refino apenas?).
- Aplicação ADR-0064/0065 esperada.

**Não criar reservas** — apenas listar candidatos com informação;
decisão sobre ordem real fica para sessão posterior.

Output: secção §5 do diagnóstico — sequência candidata.

### .6 Decisão sobre próximo passo concreto

Com base em §5 (Bloco A ordenado):
- Recomendar primeiro candidato a executar pós-P159B.
- Identificar passo administrativo XS necessário antes (e.g.
  promover ADR-0062 antes de Bloco B).
- Estimar quantos sub-passos Model são alcançáveis antes de
  saturação (atingir tecto §4).

**Validação humana após este passo**: §6 do diagnóstico é
explicitamente recomendação, não decisão final. Tu validas.

Output: secção §6 do diagnóstico — recomendação de execução.

### .7 Actualizar ADR-0061 §"Aplicações cumulativas"

Anotar P159B como passo diagnóstico amplo. Tabela de slope
cumulativo ganha linha P159B com slope "—" e tests Δ "0".

Padrões metodológicos: inventariar-primeiro N=14 → 15
(ADR-0065 critério #5: scope — quarta aplicação concreta com
diversidade ampliada para multi-feature).

### .8 Actualizar README ADRs

Sem ADR nova; entrada cronológica de P159B adicionada antes
de P159A.

---

## Verificação

Numerada para reporte de conclusão:

1. Diagnóstico
   `00_nucleo/diagnosticos/diagnostico-expansao-159-passo-159b.md`
   produzido com 6 secções (§1 ADRs/DEBTs por família;
   §2 código pendente; §3 dependências cruzadas; §4 tecto
   Model; §5 sequência candidata; §6 recomendação).
2. Mapa ADR/DEBT por família documentado em §1.
3. Inventário código pendente factual em §2 (não inferido).
4. Matriz de dependências cruzadas em §3 com 5 categorias
   (Introspection / multi-region / crate externa / ADR
   pendente / outro módulo).
5. Análise de tecto Model em §4 com estimativas numéricas
   (cobertura agregada e ampla).
6. Sequência candidata em §5 com pelo menos Bloco A (sem
   dependências cruzadas) populado.
7. Recomendação concreta em §6 com primeiro candidato a
   executar.
8. **Sem novas reservas** criadas em P159B (paridade política
   P158).
9. ADR-0061 §"Aplicações cumulativas" actualizada com linha
   P159B.
10. `crystalline-lint`: zero violations (sem código alterado).
11. **Sem alteração de hashes** — `entities/content.rs`
    mantém `ec58d849` (décimo passo consecutivo com
    interpretação L0-baseada).

---

## Critério de conclusão

- Verificações 1-11 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-159b-relatorio.md`
  produzido com:
  - Resumo do diagnóstico (síntese das 6 secções).
  - Recomendação concreta para passo seguinte (de §6).
  - Listagem completa de candidatos em Bloco A com informação
    suficiente para tu decidires.
  - Listagem de candidatos em Bloco B/C com bloqueadores
    explícitos.
  - §análise de risco (padrão N=14 → 15).
  - Confirmação: ADR-0065 critério #5 quarta aplicação
    concreta com diversidade multi-feature.
  - **Decisão crítica**: tecto Model puro vs pós-resolver
    dependências — registar com estimativas.

---

## O que pode sair errado

**Cenários gerais**:
- DEBT-55 ter scope diferente do esperado (e.g. cobrir só
  parsing de hayagriva sem CSL) → ajustar matriz de
  dependências; documentar.
- ADR-0017 detalhes revelarem que mais features Model dependem
  dela do que esperado (e.g. supplement por lang) → matriz §3
  cresce; tecto §4 reduz.
- DEBT-34e detalhes revelarem que algoritmo placement Grid é
  partilhado com outros containers (e.g. layout columns) →
  matriz §3 cresce em direcção a Layout dependência.

**Cenários específicos**:
- Bloco A ficar vazio (todos os refinos exigem dependência
  cruzada) → recomendação §6 muda para "atacar Introspection
  ou outro módulo dependência primeiro"; **caminho válido**.
- Bloco A ter apenas 1-2 candidatos → série 159 está
  praticamente saturada em Model puro; recomendar mudança de
  módulo após esses 1-2 passos.
- Inventário .1 revelar que ADR-0062 tem conteúdo concreto
  documentado algures que não foi visto antes → atalho
  possível: P159 hayagriva pode estar mais próximo do que se
  pensa.
- Múltiplos candidatos com tamanho similar e sem dependência
  → matriz de comparação adicional pode ser útil; documentar.
- Hayagriva integration acabar por não ser realista mesmo com
  ADR-0062 promovida (e.g. crate exige features Rust
  unstables) → P159 no actual contexto fica fechada com P159A
  como subset minimal definitivo.

---

## Notas operacionais

- **P159B é primeiro diagnóstico amplo da sessão**. P156B
  para Layout era amplo (cobria toda a Fase 1+2+3); P157/P158/
  P159 base eram focados (uma feature cada). P159B retoma o
  scope amplo per pedido humano "até onde dá para ir com Model".
- **§análise de risco no relatório**: passo diagnóstico baixo
  risco. Manter §análise de risco preserva precedente N=14 →
  15.
- **Política "sem novas reservas" preservada** — recomendações
  em §6 são para validação humana, não compromissos.
- **Auto-aplicação ADR-0065 critério #5**: quarta aplicação
  concreta com diversidade multi-feature. Patamar consolidado.
- **Recomendação §6**: explicitamente sujeita a validação
  humana. Reservar autonomia humana para a decisão final é
  paridade política P158.
- **Tecto Model análogo a análise Layout**: tu pediste antes
  análise paralela para Layout ("fechamos Layout só mantendo
  o ritmo?"). P159B faz o mesmo para Model com base factual
  adicional acumulada P157/P158/P159 + variants + DEBTs
  pendentes.

---

## Pós-passo

Após conclusão de P159B:

**Layout fica em 78% inalterado**. **Model agregado fica em
~50% inalterado** (passo documental). **Tecto Model puro
documentado**.

**Próxima decisão (validação humana de §6)**:
- Aprovar recomendação §6 → redigir spec do passo concreto
  recomendado.
- Redirigir para outro candidato §5 → redigir spec.
- Mudar de módulo (Introspection ou outro) se §4/§6 mostrar
  que tecto Model puro está saturado → redigir P160 (ou outro)
  como diagnóstico do novo módulo.

ADR-0060 mantém-se IMPLEMENTADO. ADR-0061 mantém-se PROPOSTO.
ADR-0062 estado factual confirmado em §1 (reserva sem ficheiro
documentada).

Padrão granularidade 1-2 features/passo (N=14) NÃO é desafiado
por P159B (passo diagnóstico). Pode ser desafiado por sub-passos
materializáveis seguintes consoante recomendação §6.

**Reservas pendentes** (não criadas neste passo):
- ADR-0062 hayagriva — pré-existente.
- ADR-0017 Introspection runtime adiada — pré-existente.

**Próxima decisão humana**: validação de §4 (tecto Model) e
§6 (recomendação) antes de redigir passo seguinte.

**Princípio operacional confirmado**: P159B documenta
empiricamente o tecto Model — informação útil mesmo se
escolherem mudar de módulo, porque torna a decisão informada
em vez de arbitrária.

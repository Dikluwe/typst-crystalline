# Passo P156K — ADR meta formalizando padrões consolidados

Passo arquitectural meta. Não materializa código. Custo baixo,
benefício alto para sessões futuras (per documento de estado
pós-P156I, direcção 5).

Formaliza dois padrões com patamar empírico forte ao fechar a
série P156C-J:
- Smart<T> → Option<T>/default (**N=6**).
- Inventariar primeiro pré-decisão arquitectural (**N=5**).

---

## Estado actual antes de começar

- 61 ADRs (último: ADR-0061 PROPOSTO; §"Aplicações cumulativas"
  anotada com P156J).
- ADR-0062 reservada hayagriva; ADR-0063 reservada outra crate.
- Próxima numeração disponível para ADR meta: **ADR-0064** (se
  ADR-0063 ficar reservada) ou **ADR-0063** (se for revogada).
- Precedente de ADRs meta: verificar índice canónico
  `00_nucleo/adr/README.md` para precedentes de ADRs com
  natureza metodológica (não técnica).

**Leituras prévias obrigatórias**:
- `00_nucleo/adr/README.md` — confirmar próxima numeração e
  precedentes de ADRs meta.
- Relatórios da série P156C-J (P156D, P156E, P156G, P156H,
  P156I, P156J) para citar evidência empírica de Smart→Option
  (N=6 aplicações).
- Relatórios P156C, P156D, P156G, P156H, P156J para evidência
  de inventariar-primeiro (N=5 aplicações).
- Documento de estado pós-P156I — secção "Padrões metodológicos
  consolidados".
- Relatório P156J §5 — patamar consolidado N=6/N=5/N=8.

---

## Natureza do passo

**Tamanho**: S+.

**Justificação**: Dois ADRs novos. Trabalho documental, sem
modificação de código nem de helpers. Verificação reduz-se a
linter (markdown) e índice canónico.

Granularidade preservada: 2 ADRs separados, não 1 fundido.
Cada padrão tem âmbito distinto e merece ADR próprio para
referência futura precisa.

---

## Decisões já tomadas

- **Dois ADRs separados, não um único fundido**:
  - Smart→Option e inventariar-primeiro têm scopes diferentes.
  - Smart→Option é regra técnica de tradução vanilla→cristalino.
  - Inventariar-primeiro é regra de processo pré-decisão.
- **Status inicial**: ambos `EM VIGOR` (regra/política activa
  validada empiricamente — não `PROPOSTO`).
- **Vinculação a ADR-0034**: inventariar-primeiro estende
  ADR-0034 (diagnóstico obrigatório); ADR meta refere-se a
  ADR-0034 como base e generaliza para decisões arquitecturais
  além de materialização de tipo vanilla.

## Decisões diferidas

- **Subpadrão `extract_length` N=6**: não é formalizado como
  ADR autónoma neste passo. Candidato a §adicional dentro da
  ADR Smart→Option (mencionado como exemplo de helper que
  consolida o padrão), ou a ADR futura "reuso de helpers stdlib"
  se outros helpers atingirem patamar similar.
- **Padrão granularidade 1-2 features/passo (N=8)**: NÃO
  formalizado neste passo. Candidato a ADR meta futura se
  patamar continuar a crescer ou se for desafiado por passo
  M+/L (e.g. columns/colbreak).
- **Padrão §análise de risco (N=5)**: não formalizado neste
  passo. Já parcialmente coberto por convenções de processo
  no documento de estado.
- Numeração exacta dos dois ADRs novos: depende do estado
  actual do índice. Decidida no sub-passo .1.
- Eventual fusão futura em ADR consolidado de "padrões
  metodológicos da série P156" se mais padrões forem promovidos
  — não é scope deste passo.

---

## Sub-passos

### .1 Inventário e numeração

Inspecção de `00_nucleo/adr/README.md`:
1. Confirmar próxima numeração livre.
2. Confirmar status de ADR-0062 e ADR-0063 (reservadas).
3. Identificar precedentes de ADRs meta (status `EM VIGOR` com
   natureza de política/processo, não decisão técnica concreta).
4. Decidir numeração final dos dois ADRs novos (consecutivos
   se possível).

Sub-passo .1 cumpre o próprio padrão a ser formalizado
(inventariar-primeiro) — auto-aplicação tornará a aplicação
N=6 do padrão.

### .2 Redigir ADR Smart→Option/default

Ficheiro: `00_nucleo/adr/typst-adr-<NNNN>-smart-para-option-default.md`.

Conteúdo:
- **Status**: EM VIGOR.
- **Contexto**: vanilla typst usa `Smart<T>` (variant `Auto`/`Custom(T)`)
  para campos com default contextual. Cristalino traduz para
  `Option<T>` com default explícito ao construir, ou para o
  tipo directo `T` quando há `Default` natural.
- **Decisão**: regra de tradução vinculativa.
  - Caso A: `Smart<T>` com semântica "auto = computa do contexto"
    → `Option<T>`; `Auto` → `None`; default contextual resolvido
    em momento de uso (stdlib func ou layout).
  - Caso B: `Smart<T>` com semântica "auto = valor literal
    fixo" → `T` directo se `T: Default`; `Auto` → `T::default()`.
  - Caso C: campo vanilla `T` com default não-`Default` (e.g.
    `Length::zero()`) → `Option<T>`; `None` → default em uso.
  - Caso D: campo vanilla `bool` com default não-`false` (e.g.
    `justify=true` em P156J) → `bool` directo, não Option, com
    documentação explícita do default vanilla.
- **Justificação**: **N=6 aplicações empíricas** consecutivas:
  - **P156D**: `weak: bool` directo (sem Smart).
  - **P156E**: `Smart<Parity>` → `Option<Parity>`.
  - **P156G**: `Smart<Rel<Length>>` → `Option<Length>` (Block.width).
  - **P156H**: idem (Box.width + Box.baseline).
  - **P156I**: `Smart<Length>` → `Option<Length>` (Stack.spacing)
    + `Smart<Dir>` → `Dir` directo com `Default::default() == TTB`.
  - **P156J**: `Length` (default zero vanilla) → `Option<Length>`
    (gap); `bool` directo (justify, default true para paridade).
  Reduz superfície de tipo público; mantém paridade observável
  (ADR-0033). Diagnósticos de erro de tipo mais claros.
- **Excepções**:
  - Campos onde `Auto` tem semântica distinta de "default" e
    requer enum dedicado (não Option).
- **Implicações**:
  - Código mais idiomático Rust.
  - Helpers stdlib (`extract_length`, `extract_parity`,
    `extract_dir`, `extract_weak`) consolidam pattern Option +
    default.
  - Subpadrão emergente: `extract_length` reusado N=6 vezes
    consecutivas (P156C/D/G/H/I/J) — candidato a promoção
    formal a helper público em refactor XS futuro.
- **Relação com outras ADRs**: estende ADR-0034 (diagnóstico
  obrigatório identifica campos Smart antes de tradução).
  Compatível com ADR-0033 (paridade observável preservada).

### .3 Redigir ADR Inventariar-primeiro

Ficheiro: `00_nucleo/adr/typst-adr-<NNNN+1>-inventariar-primeiro.md`.

Conteúdo:
- **Status**: EM VIGOR.
- **Contexto**: ADR-0034 já obriga diagnóstico antes de
  materialização de tipo vanilla. Série P156C-J generalizou:
  qualquer decisão arquitectural não-trivial precede de
  inventário em sub-passo .1.
- **Decisão**: passos com decisão arquitectural não-trivial
  têm sub-passo .1 dedicado a inventário pré-materialização.
  Generaliza ADR-0034 para além de tipo vanilla (e.g. expansão
  de variants existentes, refactor de helpers, ADRs meta).
- **Justificação**: **N=5 aplicações empíricas** consecutivas:
  - **P156C**: diagnóstico pad+hide (vanilla PadElem/HideElem
    inventariado antes de variant).
  - **P156D**: diagnóstico h+v (HSpace/VSpace; Spacing struct
    decidida via inventário).
  - **P156G**: diagnóstico block (campos Block vanilla
    inventariados; decisão sobre inset/stroke/fill diferida).
  - **P156H**: diagnóstico box (naming Box→Boxed decidido via
    inventário do conflito com `std::boxed::Box`).
  - **P156J**: diagnóstico repeat (default `justify=true`
    descoberto via inventário; algoritmo runtime diferido).
  Zero reformulações mid-passo na série N=8 — em contraste
  com precedentes pré-P156C onde inventário em falta levou
  a reformulação (citar exemplos do histórico se passos com
  reformulação tiverem sido marcados; investigar em .1).
- **Critério de "não-trivial"**:
  - Decisão envolve naming (precedente Box→Boxed P156H).
  - Decisão envolve tipo (precedente Arc<[T]> P156I; default
    `bool` P156J).
  - Decisão expande variant existente (precedente P156F skew).
  - Decisão atravessa camadas (L0/L1/L2/L3/L4).
- **Excepções**: passos S triviais sem decisão (e.g.
  propagação de hashes; correcção de typos) não requerem .1.
- **Aplicação a passos meta**: este próprio passo (P156K)
  cumpre o padrão — sub-passo .1 inventaria estado dos ADRs
  antes de redigir os novos. Auto-aplicação valida o padrão.
- **Relação com outras ADRs**: estende ADR-0034. Compatível
  com cadência granular validada N=8.

### .4 Actualizar índice canónico

`00_nucleo/adr/README.md`:
- Adicionar entradas para os dois ADRs novos.
- Marcar status EM VIGOR.
- Linkar para ficheiros.

### .5 Verificação documental

- Linter markdown se existir no projecto.
- Verificar links cruzados (ADR-0034 referenciada correctamente).
- Verificar contagem total ADRs no índice (61 → 63).

---

## Verificação

Numerada para reporte de conclusão:

1. `00_nucleo/adr/README.md` lista 63 ADRs (61 + 2 novos).
2. Status dos dois novos ADRs: ambos EM VIGOR (não PROPOSTO).
3. Numeração consecutiva confirmada (e.g. 0064 + 0065, ou outra
   par decidida em .1).
4. ADR Smart→Option cita **N=6 aplicações** (P156D, P156E,
   P156G, P156H, P156I, P156J).
5. ADR Inventariar-primeiro cita **N=5 aplicações** (P156C,
   P156D, P156G, P156H, P156J).
6. Linter markdown (se aplicável): zero erros.
7. ADR-0034 não modificada (estendida por referência, não
   revogada).
8. `crystalline-lint`: zero violations (sem alteração de código,
   esperado trivial).

---

## Critério de conclusão

- Verificações 1-8 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-156k-relatorio.md`
  produzido com:
  - Resumo dos dois ADRs.
  - Numeração final atribuída.
  - Confirmação de evidência empírica (lista de passos citados:
    6 para Smart→Option; 5 para inventariar-primeiro).
  - §análise de risco (padrão N=5; sexta aplicação se
    preservar precedente; mesmo sendo passo documental).
  - Nota: contagem ADRs total passa a 63; reservas P157/P158/P159
    e ADR-0062/ADR-0063 mantêm-se inalteradas.

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que ADR-0034 já cobre inventariar-primeiro
  com generalidade suficiente → ADR meta de inventariar-primeiro
  passa a ser revisão `-R1` de ADR-0034 em vez de ADR nova.
  Decisão registada em .1.
- Numeração reservada (0062/0063) ocupar slots desejados →
  saltar para 0064/0065.

**Cenários específicos**:
- Smart→Option ter excepções não capturadas (e.g. P156J revelou
  caso de `bool` directo com default não-`false`) → documentar
  como "Caso E" se mais excepções emergirem; por agora cobrir
  com Casos A-D.
- Patamar empírico ainda não merecer EM VIGOR (e.g. utilizador
  preferir status `IDEIA` ou `PROPOSTO` para período de
  observação adicional) → escalar decisão antes de redigir.
- Subpadrão `extract_length` N=6 ser confundido com o padrão
  Smart→Option em si → manter separado: ADR é sobre o padrão
  de tradução, helper é exemplo de implementação consolidada.

---

## Notas operacionais

- Este passo NÃO altera código. Não há hash a propagar.
- Custo estimado baixo (redacção documental).
- Benefício: sessões futuras citam ADR explicitamente em vez
  de re-justificar empiricamente cada vez. Reduz overhead de
  enunciados futuros.
- Auto-aplicação: o próprio P156K segue padrão
  inventariar-primeiro (sub-passo .1 dedicado), tornando-se
  N=6 do padrão se a aplicação contar para o ADR a redigir.
  **Decisão**: contar como N=5 dentro do ADR (passos pré-P156K)
  para evitar circularidade; mencionar auto-aplicação no
  §Aplicação a passos meta.

---

## Pós-passo

Após P156K, ADRs meta passam a base citável para passos
futuros. Próxima decisão depende de outras direcções pendentes
(per documento de estado pós-P156I e relatório P156J §8):
- Continuar Fase 3 — columns + colbreak (DEBT-56 column flow
  L+; quebra granularidade; provavelmente 3-5 passos).
- Mudar para Model Fase 2 P157 (table foundations).
- Footnote area (sub-fase prioritária ADR-0061 Decisão 5).
- Promover ADR-0061 a IMPLEMENTADO (3 caminhos documentados).
- Investigar discrepância DEBTs (9 vs 13) — passo administrativo XS.
- Retomar paridade (DEBT-54/53 suspensos desde P153).
- Atacar Introspection (17% cobertura — mais fraca per
  inventário 148).

ADR-0061 mantém-se PROPOSTO. Padrão granularidade 1-2
features/passo (N=8) NÃO é formalizado neste passo — candidato
a ADR meta futura se patamar continuar a crescer ou se for
desafiado por passo M+/L.

Subpadrão `extract_length` N=6 NÃO é formalizado autonomamente
neste passo — mencionado como §Implicações da ADR Smart→Option.
Promoção a helper público (`pub fn extract_length`) candidato
a refactor XS futuro.

# Passo P156K-meta — ADR meta formalizando padrões consolidados

Passo arquitectural meta. Não materializa código. Custo baixo,
benefício alto para sessões futuras (per documento de estado
pós-P156I, direcção 5).

Formaliza dois padrões com patamar empírico forte ao fechar a
série P156C-I (e P156J se executado primeiro):
- Smart<T> → Option<T>/default (N=5 ou N=6).
- Inventariar primeiro pré-decisão arquitectural (N=4 ou N=5).

---

## Estado actual antes de começar

- 61 ADRs (último: ADR-0061 PROPOSTO).
- ADR-0062 reservada hayagriva; ADR-0063 reservada outra crate.
- Próxima numeração disponível para ADR meta: **ADR-0064** (se
  ADR-0063 ficar reservada) ou **ADR-0063** (se for revogada).
- Precedente de ADRs meta: verificar índice canónico
  `00_nucleo/adr/README.md` para precedentes de ADRs com
  natureza metodológica (não técnica).

**Leituras prévias obrigatórias**:
- `00_nucleo/adr/README.md` — confirmar próxima numeração e
  precedentes de ADRs meta.
- Relatórios da série P156C-I (P156D, P156E, P156G, P156H,
  P156I) para citar evidência empírica de Smart→Option.
- Relatórios P156C, P156D, P156G, P156H (e P156J se já
  executado) para evidência de inventariar-primeiro.
- Documento de estado pós-P156I — secção "Padrões metodológicos
  consolidados".

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

- Numeração exacta: depende do estado actual do índice. Decidida
  no sub-passo .1.
- Eventual fusão futura em ADR consolidado de "padrões
  metodológicos da série P156" se mais padrões forem promovidos
  (e.g. granularidade 1-2 features N=7) — não é scope deste
  passo.

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

### .2 Redigir ADR Smart→Option/default

Ficheiro: `00_nucleo/adr/typst-adr-<NNNN>-smart-para-option-default.md`.

Conteúdo:
- **Status**: EM VIGOR.
- **Contexto**: vanilla typst usa `Smart<T>` (variant `Auto`/`Custom(T)`)
  para campos com default contextual. Cristalino traduz para
  `Option<T>` com default explícito ao construir.
- **Decisão**: regra de tradução vinculativa. `Smart<T>` →
  `Option<T>`; valor `Auto` → `None`; default contextual
  resolvido em momento de uso (stdlib func ou layout).
- **Justificação**: N=5 aplicações empíricas em P156D, P156E,
  P156G, P156H, P156I (citar passos com evidência de helpers
  `extract_*`). Reduz superfície de tipo público; mantém
  paridade observável (ADR-0033).
- **Excepções**: campos onde `Auto` tem semântica distinta de
  "default" (e.g. `Auto` significa "calcula from context" vs
  "usa valor literal definido pela stdlib"). Documenta-se
  caso-a-caso.
- **Implicações**:
  - Código mais idiomático Rust.
  - Diagnósticos de erro de tipo mais claros.
  - Helpers stdlib (`extract_*`) consolidam pattern Option +
    default.
- **Relação com outras ADRs**: estende ADR-0034 (diagnóstico
  obrigatório identifica campos Smart antes de tradução).

### .3 Redigir ADR Inventariar-primeiro

Ficheiro: `00_nucleo/adr/typst-adr-<NNNN+1>-inventariar-primeiro.md`.

Conteúdo:
- **Status**: EM VIGOR.
- **Contexto**: ADR-0034 já obriga diagnóstico antes de
  materialização de tipo vanilla. Série P156C-I generalizou:
  qualquer decisão arquitectural não-trivial precede de
  inventário.
- **Decisão**: passos com decisão arquitectural não-trivial
  têm sub-passo .1 dedicado a inventário pré-materialização.
  Generaliza ADR-0034 para além de tipo vanilla (e.g. expansão
  de variants existentes, refactor de helpers, ADRs meta).
- **Justificação**: N=4 aplicações empíricas em P156C, P156D,
  P156G, P156H (citar passos com evidência de §inventário em
  relatório). Zero reformulações mid-passo na série N=7 — em
  contraste com precedentes pré-P156C (citar exemplos se
  inventário falta levou a reformulação).
- **Critério de "não-trivial"**:
  - Decisão envolve naming (precedente Box→Boxed P156H).
  - Decisão envolve tipo (precedente Arc<[T]> P156I).
  - Decisão expande variant existente (precedente P156F skew).
  - Decisão atravessa camadas (L0/L1/L2/L3/L4).
- **Excepções**: passos S triviais sem decisão (e.g.
  propagação de hashes; correcção de typos) não requerem .1.
- **Relação com outras ADRs**: estende ADR-0034. Compatível
  com cadência granular validada N=7.

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
4. Cada ADR cita ≥3 passos da série P156C-I como evidência
   empírica (Smart→Option: N=5; inventariar-primeiro: N=4).
5. Linter markdown (se aplicável): zero erros.
6. ADR-0034 não modificada (estendida por referência, não
   revogada).
7. `crystalline-lint`: zero violations (sem alteração de código,
   esperado trivial).

---

## Critério de conclusão

- Verificações 1-7 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-156k-meta-relatorio.md`
  produzido com:
  - Resumo dos dois ADRs.
  - Numeração final atribuída.
  - Confirmação de evidência empírica (lista de passos citados).
  - §análise de risco (padrão N=4; mesmo sendo passo documental,
    preservar precedente).
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
- Smart→Option ter excepções não capturadas no inventário →
  documentar excepções no corpo da ADR; não bloquear redacção.
- Patamar empírico ainda não merecer EM VIGOR (e.g. utilizador
  preferir status `IDEIA` ou `PROPOSTO` para período de
  observação adicional) → escalar decisão antes de redigir.

---

## Notas operacionais

- Este passo NÃO altera código. Não há hash a propagar.
- Custo estimado baixo (redacção documental; ~1-2h humano-dia
  equivalente).
- Benefício: sessões futuras citam ADR explicitamente em vez
  de re-justificar empiricamente cada vez. Reduz overhead de
  enunciados futuros.
- Se P156J for executado primeiro, evidência empírica de
  Smart→Option passa de N=5 para N=6 (helper `extract_length`
  reusado). Actualizar contagem em .2.

---

## Pós-passo

Após P156K-meta, ADRs meta passam a base citável para passos
futuros. Próxima decisão depende de outras direcções pendentes:
- Continuar Layout (Fase 3 columns L+ via DEBT-56; ou outra
  Fase).
- Mudar para Model Fase 2 P157 (table foundations).
- Footnote area (sub-fase prioritária ADR-0061 Decisão 5).
- Promover ADR-0061 a IMPLEMENTADO (3 caminhos documentados).

ADR-0061 mantém-se PROPOSTO. Padrão metodológico granularidade
1-2 features/passo (N=7 ou N=8 com P156J) NÃO é formalizado
neste passo — candidato a ADR meta futura se patamar continuar
a crescer.

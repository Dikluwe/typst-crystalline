# Passo P158 — Diagnóstico Model figure-kinds

Passo diagnóstico precede materialização — análogo estrutural a
**P157** (diagnóstico table foundations) e **P156B** (diagnóstico
Layout). **Não materializa código**. Inventaria estado actual
de `Figure` em código cristalino, conteúdo de ADR-0060 sobre
figure-kinds, e define subset concreto para passo substantivo
seguinte (P158A) a redigir após validação humana deste
diagnóstico.

Aplicação directa de **ADR-0065 critério #5** (scope determinado
por inventário). Segunda aplicação concreta deste critério após
P157 — patamar empírico cresce.

---

## Estado actual antes de começar

- 63 ADRs após P157C (28 EM VIGOR; ADR-0060 IMPLEMENTADO; ADR-
  0064 e ADR-0065 EM VIGOR a maturidade empírica per saturação
  cross-domínio cross-caso atingida em P157C).
- Layout: 78% (inalterado). Cobertura arquitectural total 80%.
- Cobertura Model agregada: ~50% (inalterada em P157A/B/C; 4
  variants table foundations adicionadas como expansão estrutural).
- Hash actual `entities/content.rs`: `ec58d849` (preservado
  P156L → P157 → P157A → P157B → P157C — 5 passos consecutivos
  com refactor aditivo).
- 1379 tests (lib+integ+diagnostic; workspace 1401); zero
  violations linter.
- 56 variants Content; 46 stdlib funcs.
- Padrões consolidados pós-P157C: granularidade N=12;
  inventariar N=10; Smart→Option N=9 (saturação cross-domínio
  cross-caso); §análise risco N=10; reuso `Sides<T>` N=2;
  reuso `extract_length` N=7; reuso `extract_tracks` N=2;
  helper privado parametrizado `extract_usize_or_none_min`
  N=4 usos; helper privado parametrizado
  `extract_bool_with_default` N=2 usos; par simétrico em
  pattern-match N=2.

**P158 era reservado** (per documento de estado pós-P156I) para
"Model figure-kinds". Conteúdo concreto desta reserva tem
informação parcial:

- **Relatório P157A §1.2** confirmou: `Figure.kind: "table"`
  slot já existente em código cristalino — preparação directa
  para P158.
- Implica: `Content::Figure` provavelmente já existe em
  cristalino com algum tipo de field `kind`. P158 trabalha
  sobre infraestrutura existente, não cria do zero.
- **Não confirmado**:
  - Estrutura concreta de `Content::Figure` actual.
  - Que kinds (image, table, raw, custom) já estão suportados
    em vanilla.
  - Que kinds (image, table, raw, custom) estão suportados
    em cristalino.
  - Conteúdo de ADR-0060 §"Decisão" relevante a figure-kinds.
  - Estado de `Content::Image` em cristalino (figure-kinds
    image depende de Image).

Resolver estas lacunas é o objectivo de P158.

---

## Natureza do passo

**Tamanho**: S+.

**Justificação**: Trabalho documental puro. Inventário +
diagnóstico + decisão de scope para passo seguinte. Sem
modificação de código, sem ADR nova (ADR-0060 já existe e
governa Model — este passo lê-a, não cria nova).

Granularidade preservada: 1 deliverable diagnóstico → mantém
peso S+ análogo a P157 e P156K (passos meta documentais).

**Risco baixo**: passo é **previne** risco em P158A (passo
substantivo seguinte) detectando lacunas factuais antes de
materialização.

---

## Decisões já tomadas

- **Identificador P158**: mantido per reserva original (decisão
  humana confirmada na sessão pós-P157C).
- **Natureza diagnóstica**: P158 deixa de ser materialização
  e passa a ser diagnóstico precedendo materialização. **P158A**
  (ou outro identificador) será o passo substantivo a redigir
  após validação humana deste diagnóstico.
- **Sem código alterado**: passo puramente documental.
- **Sem ADR nova**: ADR-0060 já governa Model; ADR-0061 governa
  Layout. P158 lê ambas, não cria nova.
- **Sem novas reservas**: P158 NÃO cria reservas para passos
  futuros. Reservas existentes (P159 = bibliography + cite;
  ADR-0062 = hayagriva) mantêm-se documentadas mas não são
  reforçadas neste passo.

## Decisões diferidas

- **Subset concreto de "figure-kinds"**: a decidir no sub-passo
  .3 com base em ADR-0060 e estado factual em código (não a
  priori sem ler).
- **Tamanho de P158A**: a decidir no sub-passo .5 com base em
  scope determinado em .3.
- **Dependência de `Content::Image`**: a decidir no sub-passo
  .2. Se Image não existir em crystalline, P158A pode ter de
  precedêr-se de passo dedicado a Image; ou figure-kinds
  excluir image.

---

## Sub-passos

### .1 Ler e resumir ADR-0060 sobre figure-kinds

Localizar e ler `00_nucleo/adr/typst-adr-0060-*.md` (já
parcialmente lida em P157):
- Procurar §"Decisão" relevante a figure-kinds.
- Procurar referência a kinds suportados (image, table, raw,
  custom, outros).
- Status declarado para figure-kinds (PROPOSTO, IDEIA,
  ADIADO, etc.).
- Dependências documentadas (e.g. requer Image; requer Figure
  pré-existente).

Procurar ADRs adicionais relevantes:
- ADRs sobre Image (se existir).
- ADRs sobre Figure infraestrutura (se diferente de ADR-0060).
- ADRs sobre counters / numbering (figure-kinds depende de
  counters per vanilla).

Output: secção §1 do diagnóstico.

### .2 Inventariar estado de Figure e dependências em código

Inspecção de `01_core/src/`:
- `Content::Figure` actual: estrutura, fields, defaults.
- `Content::Image` actual: existe? estado (puro/parcial/ausente).
- Stdlib funcs `figure`, `image` em `stdlib/structural.rs` ou
  outro módulo.
- Counters relevantes a figure-kinds (paridade vanilla
  `figure(image)` numera diferente de `figure(table)`).

Inspecção de `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:
- Tabela A entrada `figure` — estado.
- Tabela A entrada `image` — estado.
- Outras entradas user-facing relacionadas a figure-kinds
  (raw, listing, custom).

Output: secção §2 do diagnóstico (inventário estruturado de
Figure + dependências).

### .3 Determinar scope de "figure-kinds"

Síntese de §1 e §2 do diagnóstico para responder:

1. Que kinds vanilla constituem "figure-kinds" per ADR-0060?
2. Dessas, quais já estão preparadas em cristalino (`kind`
   slot existente, conforme relatório P157A §1.2)?
3. Que dependências internas existem (e.g. figure(image)
   depende de Image; figure(raw) depende de Raw)?
4. Qual é o subset mínimo materializável que preserva
   granularidade 1-2 features (N=12)?
5. Qual é o subset máximo que ainda cabe num passo M?
6. Subset mínimo vs máximo: recomendação para P158A.

**Avaliar precedente P157**: diagnóstico P157 dividiu "table
foundations" M+ em 3 sub-passos M cada (P157A/B/C). Verificar
se figure-kinds tem estrutura análoga (kinds independentes
materializáveis um por um) ou estrutura diferente que justifique
abordagem distinta.

Output: secção §3 do diagnóstico (decisão de scope).

### .4 Identificar dependências bloqueantes

Para cada item do subset recomendado em §3:
- Existe pré-requisito em outro módulo?
- Existe DEBT aberto que bloqueie?
- Existe ADR pendente de promoção?
- Existe ADR-0017 (Introspection runtime) na cadeia de
  dependências (counters)?

Casos típicos a verificar:
- **figure(image)** exige Image — confirmar se Image está
  implementado em cristalino; se não, expandir P158A para
  incluir Image ou diferir image.
- **figure(table)** já tem slot pós-P157A — confirmar se
  basta activação ou requer trabalho adicional.
- **figure(raw)** exige Raw — verificar estado.
- **figure custom** exige `kind: str` arbitrário — verificar
  se já é suportado.
- **Counters de figure** dependem de runtime introspection
  per ADR-0017 — verificar se aplicável.

Output: secção §4 do diagnóstico (dependências bloqueantes).

### .5 Definir P158A (ou identificador alternativo)

Com base em §1-§4:
- Identificador concreto do passo substantivo seguinte (P158A
  ou outro per precedente P157A/B/C; ou outra estrutura se
  scope justificar).
- Tamanho estimado (S/M/M+/L).
- Subset concreto de features.
- Sub-passos previstos a alto nível.
- Critério de granularidade: preservada (N=13) ou quebrada
  (caso registado).
- **Não criar reservas** para P158B/P158C/futuros — apenas
  esboçar P158A; passos seguintes a decidir sequencialmente
  per evidência empírica.

Output: secção §5 do diagnóstico (esboço de P158A).

### .6 Actualizar ADR-0061 §"Aplicações cumulativas"

Anotar P158 como passo diagnóstico análogo a P156B/P156K/P157
(passo não-materialização). Tabela de slope cumulativo ganha
linha P158 com slope "—" e tests Δ "0".

Padrões metodológicos: inventariar-primeiro N=10 → 11 (ADR-
0065 critério #5: scope — segunda aplicação concreta).

### .7 Actualizar README ADRs

Sem ADR nova; entrada cronológica de P158 adicionada antes de
P157C se a estrutura do README listar passos.

---

## Verificação

Numerada para reporte de conclusão:

1. Diagnóstico
   `00_nucleo/diagnosticos/diagnostico-model-figure-kinds-passo-158.md`
   produzido com 5 secções (§1 ADR-0060 sobre figure-kinds;
   §2 estado código Figure + dependências; §3 scope; §4
   dependências bloqueantes; §5 esboço P158A).
2. ADR-0060 §"Decisão" relevante a figure-kinds lida e resumida
   em §1.
3. Estado de `Content::Figure` em crystalline determinado
   factualmente em §2 (não inferido).
4. Estado de `Content::Image` (e outras dependências) em
   crystalline determinado factualmente em §2.
5. Subset concreto de "figure-kinds" definido em §3 com
   recomendação para P158A.
6. Dependências bloqueantes (DEBTs, ADRs pendentes,
   pré-requisitos) listadas em §4.
7. Esboço de P158A em §5: identificador, tamanho, subset,
   granularidade.
8. **Sem novas reservas criadas** para passos pós-P158A.
9. ADR-0061 §"Aplicações cumulativas" actualizada com linha
   P158 (slope "—"; padrões N actualizados).
10. `crystalline-lint`: zero violations (sem código alterado;
    esperado trivial).
11. **Sem alteração de hashes** (passo documental; nenhum
    código modificado).

---

## Critério de conclusão

- Verificações 1-11 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-158-relatorio.md`
  produzido com:
  - Resumo do diagnóstico (síntese das 5 secções).
  - Decisão final de scope para P158A.
  - Dependências identificadas a tratar antes de P158A.
  - §análise de risco (padrão N=10 → 11; passo diagnóstico
    tem risco baixo, mas registar para preservar precedente).
  - Confirmação: ADR-0065 critério #5 (scope) segunda
    aplicação concreta após P157.

---

## O que pode sair errado

**Cenários gerais**:
- ADR-0060 não cobrir figure-kinds com o detalhe esperado
  (e.g. apenas menção genérica sem subset declarado) → P158
  fica com responsabilidade adicional de definir scope per
  inventário factual; documentar gap em §1.
- `Content::Figure` em crystalline ter estrutura não-trivial
  (e.g. mais fields do que `kind` simples) → §2 documenta
  estrutura completa; §3 ajusta scope.

**Cenários específicos**:
- `Content::Image` não estar implementado em cristalline e
  ser dependência hard de figure(image) → P158A pode expandir
  para incluir Image, ou figure-kinds inicial pode excluir
  image. Decisão registada em §3 e §5.
- Counters de figure dependerem de ADR-0017 Introspection
  runtime (adiada) → figure-kinds pode ficar bloqueado se
  numbering for parte do scope; alternativa: scope-out de
  numbering per ADR-0054 graded. Decisão em §3.
- figure(table) já estar 100% implementado pós-P157A (slot +
  delegation) → P158A pode ter scope reduzido a image/raw/
  custom apenas. Documentar economia.
- Subset "figure-kinds" forçar quebra de granularidade
  (todos os caminhos exigem M+/L) → registar em §5 com
  justificação; granularidade N=12 fica em risco mas decisão
  é informada por inventário, não inferida.

---

## Notas operacionais

- **P158 não materializa código**. Análogo estrutural a P156B
  (diagnóstico Layout), P156K (ADRs meta) e P157 (diagnóstico
  table foundations). Custo baixo; benefício alto para passos
  Model subsequentes.
- **Ordem dos sub-passos importa**: .1 (ADR) precede .2 (código)
  precede .3 (scope) — porque scope depende de saber o que
  ADR define e o que código já cobre.
- **Auto-aplicação de ADR-0065 critério #5**: este próprio
  passo é exemplo de "scope determinado por inventário".
  Segunda aplicação concreta após P157.
- **§análise de risco no relatório**: passo diagnóstico tem
  risco baixo. Manter §análise de risco preserva precedente
  N=10 → 11.
- **Política nova explícita**: P158 NÃO cria reservas para
  passos futuros. Decisão de não-criação de reservas baseia-se
  na crítica humana pós-P157C de que reservas pré-existentes
  (P158 figure-kinds; P159 bibliography; ADR-0062 hayagriva)
  travavam decisões sobre direcção. Reservas pré-existentes
  são respeitadas se ainda válidas, mas não são reforçadas
  nem multiplicadas.

---

## Pós-passo

Após conclusão de P158:
- **P158A** (ou identificador definido em .5) é o passo
  substantivo seguinte com scope concreto baseado em diagnóstico.
- Granularidade pode ser preservada (esperado se subset for
  pequeno) ou quebrada (registado se inventário forçar).
- Se P158A exigir trabalho prévio (e.g. Image antes de
  figure(image)), esse trabalho prévio é redigido antes;
  P158A passa a ser o passo final da cadeia.

ADR-0060 mantém-se IMPLEMENTADO (P158 lê, não modifica).
ADR-0061 mantém-se PROPOSTO (Layout não tocado em P158).

ADR-0017 estado é factualmente confirmado em §4 (não
pré-decidido). Promoção/revogação só é considerada se §4
revelar inconsistência factual com reservas documentadas.

Padrão granularidade 1-2 features/passo (N=12) NÃO é desafiado
por P158 (passo diagnóstico). Pode ser desafiado por P158A
consoante scope decidido em §3.

**Próxima decisão humana**: validação do diagnóstico §3 (scope)
e §5 (esboço P158A) antes de redigir P158A.

**Reservas pendentes** (não criadas neste passo):
- P159 = bibliography + cite (Model XL) — pré-existente.
- ADR-0062 = hayagriva — pré-existente.
- Decisão sobre revogação ou manutenção destas reservas pode
  ser tomada em qualquer momento futuro per crítica de
  reservas travarem decisões.

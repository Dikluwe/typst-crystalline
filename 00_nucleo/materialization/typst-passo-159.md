# Passo P159 — Diagnóstico Bibliography + Cite

Passo diagnóstico precede materialização — análogo estrutural
a **P157** (diagnóstico table foundations) e **P158**
(diagnóstico figure-kinds). **Não materializa código**.
Inventaria estado actual de Bibliography/Cite em código
cristalino, conteúdo de ADR-0062 (reserva hayagriva),
DEBT-55, ADR-0060 sobre Bibliography, e dependências cruzadas
com Introspection (ADR-0017 adiada). Define subset concreto
para passo substantivo seguinte (P159A) a redigir após
validação humana deste diagnóstico.

Aplicação directa de **ADR-0065 critério #5** (scope
determinado por inventário). Terceira aplicação concreta após
P157 e P158 — patamar empírico cresce.

P159 era reservado em ADR-0060 com tamanho declarado **XL** —
maior dos três passos Model Fase 2 reservados. Diagnóstico é
crucial para decidir como dividir (paridade P157 dividiu M+ em
3xM) ou se subset minimal é viável (paridade P158 escolheu
minimal).

---

## Estado actual antes de começar

- 63 ADRs após P158A (28 EM VIGOR; ADR-0060 IMPLEMENTADO).
- Layout: 78% (inalterado). Cobertura arquitectural total 80%.
- Cobertura Model agregada: ~50% (inalterada em P158/P158A;
  refino qualitativo).
- Hash actual `entities/content.rs`: `ec58d849` (preservado em
  7 passos consecutivos P156L → P158A).
- 1385 tests (lib+integ+diagnostic; workspace 1407); zero
  violations linter.
- 56 variants Content; 46 stdlib funcs.
- Padrões consolidados pós-P158A: granularidade N=13;
  inventariar N=12; Smart→Option N=9 (saturação cross-domínio
  cross-caso); §análise risco N=12; estabilidade hash
  `content.rs` N=7 (subpadrão emergente).

**P159 era reservado** (per documento de estado pós-P156I) para
"Model bibliography + cite". Conteúdo declarado:
- Tamanho **XL** em ADR-0060.
- ADR-0062 reservada para `hayagriva` (crate vanilla de
  citações).
- DEBT-55 mencionado em vários relatórios da série como
  bloqueador de Bibliography.

**Não confirmado**:
- Conteúdo concreto de DEBT-55.
- Conteúdo de ADR-0062 (reserva, possivelmente não materializada).
- Estado de `Content::BibliographyEntry`/`Cite`/etc em
  cristalino (parcial vs ausente).
- O que ADR-0060 §"Decisão" diz sobre Bibliography concretamente
  (quais entradas user-facing, qual subset declarado).
- Como `hayagriva` se insere arquitecturalmente — runtime
  dependency vs parsing infrastructure.
- Estado de Introspection (17%) e seu impacto em `cite()`
  (counters cross-document).

**Política "sem novas reservas" estabelecida em P158** mantida
— P159 não cria reservas para passos pós-P159A.

---

## Natureza do passo

**Tamanho**: S+.

**Justificação**: Trabalho documental puro. Inventário +
diagnóstico + decisão de scope para passo seguinte. Sem
modificação de código, sem ADR nova (ADR-0060 já existe;
ADR-0062 é reserva, não materialização — este passo lê,
não cria).

Granularidade preservada: 1 deliverable diagnóstico → mantém
peso S+ análogo a P157 e P158 (passos meta documentais).

**Risco baixo**: passo previne risco em P159A (passo
substantivo seguinte) detectando lacunas factuais antes de
materialização. Particularmente importante porque XL declarado
sugere alta probabilidade de divisão necessária — diagnóstico
é o mecanismo certo para decidir como dividir.

---

## Decisões já tomadas

- **Identificador P159**: mantido per reserva original.
- **Natureza diagnóstica**: P159 deixa de ser materialização
  e passa a ser diagnóstico precedendo materialização. **P159A**
  (ou outro identificador) será o passo substantivo a redigir
  após validação humana deste diagnóstico.
- **Sem código alterado**: passo puramente documental.
- **Sem ADR nova**: ADR-0060/0062 já existem; ADR-0017 também
  já existe (Introspection runtime adiada). P159 lê todas, não
  cria nova.
- **Sem novas reservas**: paridade política P158/P158A. Reservas
  pré-existentes (ADR-0062 hayagriva) mantidas documentadas
  mas não reforçadas.

## Decisões diferidas

- **Subset concreto de "bibliography + cite"**: a decidir no
  sub-passo .3 com base em ADRs lidas em .1 e estado factual
  em .2.
- **Tamanho de P159A**: a decidir no sub-passo .5 com base em
  scope determinado em .3.
- **Dependência de hayagriva**: a decidir no sub-passo .4. Se
  hayagriva for runtime hard, P159A pode ter de ser precedido
  de passo dedicado a integração da crate (com ADR-0062 a
  promover de reserva a IMPLEMENTADO). Se hayagriva for
  scope-out per ADR-0054 graded (parsing diferido para input
  literal cristalino), P159A simplifica.
- **Dependência de Introspection runtime ADR-0017**: a decidir
  no sub-passo .4. `cite()` referencia entries cross-document
  via counters — pode ser bloqueador hard ou contornável per
  ADR-0054 graded.

---

## Sub-passos

### .1 Ler e resumir ADRs relevantes a Bibliography + Cite

Localizar e ler:
- `00_nucleo/adr/typst-adr-0060-*.md` — Model roadmap;
  procurar §"Decisão" relevante a Bibliography (paridade P157
  para table foundations e P158 para figure-kinds).
- `00_nucleo/adr/typst-adr-0062-*.md` — reserva hayagriva.
  Verificar status (PROPOSTO, IDEIA, ADIADO) e scope declarado.
- `00_nucleo/adr/typst-adr-0017-*.md` — Introspection runtime
  adiada. Verificar se cite() referenciada explicitamente como
  caso bloqueado.

Procurar ADRs adicionais relevantes:
- ADRs sobre counters / numbering (cite depende de counters
  cross-document).
- ADRs sobre quarentena vanilla relevantes a hayagriva (crate
  externa).

Output: secção §1 do diagnóstico.

### .2 Inventariar estado de Bibliography + Cite em código

Inspecção de `01_core/src/`:
- Variants `Content::*` relacionadas: `Content::BibliographyEntry`?
  `Content::Cite`? `Content::Reference`? Existência e estado.
- Stdlib funcs `bibliography`, `cite`, `ref` em
  `stdlib/structural.rs` ou outro módulo.
- Tipos auxiliares em `entities/` específicos (e.g.
  `BibStyle`, `CiteForm`).
- Hashes actuais dos ficheiros relevantes.

Inspecção de `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:
- Tabela A entradas `bibliography`, `cite`, `ref` — estado.
- Outras entradas user-facing relacionadas (e.g. `numbering`
  se figure-numbering for relevante a cite-numbering).

Inspecção de `00_nucleo/DEBT.md`:
- DEBT-55 conteúdo completo (mencionado em P156B como bloqueador
  de Bibliography).
- Outros DEBTs relacionados a citação ou parsing externo.

Output: secção §2 do diagnóstico (inventário estruturado).

### .3 Determinar scope de "bibliography + cite"

Síntese de §1 e §2 para responder:

1. Que entradas vanilla constituem "bibliography + cite" per
   ADR-0060?
2. Dessas, quais já estão implementadas em crystalline (puro,
   parcial, ausente)?
3. Que dependências internas e externas existem?
   - Internas: counters, Introspection runtime, FieldAccess
     (paridade P157B `table_cell` flat vs `table.cell`).
   - Externas: hayagriva (parsing) ou alternativa cristalina
     (input literal Vec<BibEntry>).
4. Qual é o subset mínimo materializável que preserva
   granularidade 1-2 features (N=13)?
5. Qual é o subset máximo que ainda cabe num passo M?
6. Avaliar precedente P157 (M+ → 3xM) e P158 (subset minimal):
   - É P159 divisível em sub-passos M análogos a P157A/B/C?
   - Há subset minimal viável análogo a P158A?
   - Outras estruturas se nenhuma destas se aplica?
7. Subset mínimo vs máximo: recomendação para P159A.

**Decisão crítica**: distinguir entre:
- **Bibliography** (lista de entries; struct de entrada) —
  pode ser parsing minimal ou Vec literal.
- **Cite** (referência inline a entry) — pode funcionar com
  string lookup minimal sem counters runtime.
- **Numbering scheme** (numérico, autor-ano, etc.) — pode ser
  scope-out per ADR-0054 graded.

Estes três conceitos são **frequentemente acoplados em vanilla
mas separáveis em cristalino** se ADR-0033 paridade observável
permitir. Inventário .1 e .2 deve confirmar.

Output: secção §3 do diagnóstico (decisão de scope).

### .4 Identificar dependências bloqueantes

Para cada item do subset recomendado em §3:
- Pré-requisito em outro módulo? (Introspection, counters)
- DEBT aberto que bloqueie? (DEBT-55 explicitado)
- ADR pendente de promoção? (ADR-0017 adiada; ADR-0062
  reserva)
- Crate externa requerida? (hayagriva)

Casos típicos a verificar:
- **Bibliography sem hayagriva**: viável se input cristalino
  for `Vec<BibEntry>` literal em vez de parse de `.bib`/`.yaml`?
- **Cite sem Introspection runtime**: viável se cite() resolver
  em walk single-pass (paridade counters figure em P75)?
- **Numbering schemes**: scope-out per ADR-0054 graded
  (apenas numérico em P159A; restantes diferidos)?
- **Cross-document counters**: ADR-0017 bloqueia? Ou contornável
  com counter scope local?

Output: secção §4 do diagnóstico (dependências bloqueantes).

### .5 Definir P159A (ou identificador alternativo)

Com base em §1-§4:
- Identificador concreto do passo substantivo seguinte
  (P159A ou outro per precedente P157A/B/C ou P158A; ou
  outra estrutura se scope justificar).
- Tamanho estimado (S+/M/M+/L/L+).
- Subset concreto de features.
- Sub-passos previstos a alto nível.
- Critério de granularidade: preservada (N=14) ou quebrada
  (caso registado com justificação).
- **Não criar reservas** para P159B/C/futuros — apenas esboçar
  P159A; passos seguintes a decidir sequencialmente per
  evidência empírica.

**Avaliar três estruturas possíveis** (decisão informada):
- **Estrutura A — multi-passo análogo a P157**: P159A bibliography
  base; P159B cite minimal; P159C numbering/styles. Cada um M.
  Preserva granularidade N=14/15/16.
- **Estrutura B — minimal análogo a P158**: P159A subset minimal
  (Bibliography + Cite acoplados num único passo M ou M+).
  Granularidade preservada se M; quebrada se M+.
- **Estrutura C — diferimento**: scope-out total per ADR-0054
  graded até Introspection runtime e/ou hayagriva estarem
  resolvidas. P159A passa a ser passo administrativo XS de
  scope-out documentado.

Output: secção §5 do diagnóstico (esboço de P159A com estrutura
escolhida).

### .6 Actualizar ADR-0061 §"Aplicações cumulativas"

Anotar P159 como passo diagnóstico análogo a P156B/P156K/P157/
P158 (passo não-materialização). Tabela de slope cumulativo
ganha linha P159 com slope "—" e tests Δ "0".

Padrões metodológicos: inventariar-primeiro N=12 → 13 (ADR-0065
critério #5: scope — terceira aplicação concreta após P157 e
P158).

### .7 Actualizar README ADRs

Sem ADR nova; entrada cronológica de P159 adicionada antes de
P158A.

---

## Verificação

Numerada para reporte de conclusão:

1. Diagnóstico
   `00_nucleo/diagnosticos/diagnostico-bibliography-cite-passo-159.md`
   produzido com 5 secções (§1 ADRs relevantes; §2 estado
   código; §3 scope com avaliação das 3 estruturas; §4
   dependências bloqueantes; §5 esboço P159A).
2. ADR-0060 §"Decisão" sobre Bibliography lida e resumida em
   §1.
3. ADR-0062 lida e estado confirmado em §1 (PROPOSTO/IDEIA/
   ADIADO/outro).
4. ADR-0017 lida e impacto em cite() determinado em §1.
5. Estado de `Content::Bibliography*`/`Cite*` em crystalline
   determinado factualmente em §2 (não inferido).
6. DEBT-55 conteúdo completo documentado em §2.
7. Subset concreto definido em §3 com avaliação das 3 estruturas
   (multi-passo / minimal / diferimento) e recomendação para
   P159A.
8. Dependências bloqueantes (DEBTs, ADRs pendentes, crate
   externa hayagriva, Introspection runtime) listadas em §4.
9. Esboço de P159A em §5: identificador, tamanho, subset,
   granularidade.
10. **Sem novas reservas criadas** (paridade política P158).
11. ADR-0061 §"Aplicações cumulativas" actualizada com linha
    P159 (slope "—"; padrões N actualizados).
12. `crystalline-lint`: zero violations (sem código alterado;
    esperado trivial).
13. **Sem alteração de hashes** — `entities/content.rs` mantém
    `ec58d849` (oitavo passo consecutivo se confirmado).

---

## Critério de conclusão

- Verificações 1-13 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-159-relatorio.md`
  produzido com:
  - Resumo do diagnóstico (síntese das 5 secções).
  - Decisão final de scope para P159A com estrutura escolhida
    (A multi-passo / B minimal / C diferimento).
  - Dependências identificadas a tratar antes de P159A.
  - §análise de risco (padrão N=12 → 13).
  - Confirmação: ADR-0065 critério #5 terceira aplicação
    concreta.
  - **Decisão crítica**: avaliação de estrutura escolhida com
    justificação empírica baseada em §1-§4.

---

## O que pode sair errado

**Cenários gerais**:
- ADR-0060 não cobrir Bibliography com detalhe esperado (analogous
  a P158 onde figure-kinds tinha menção genérica) → diagnóstico
  herda responsabilidade adicional de definir scope; documentar
  em §1.
- ADR-0062 não existir como ficheiro (apenas reserva mencionada
  no documento de estado) → confirmar reserva como
  documentação informal; criar ADR-0062 com status PROPOSTO
  pode ser sub-passo administrativo XS futuro mas NÃO neste
  diagnóstico.
- DEBT-55 ter scope diferente do esperado (e.g. cobrir só
  parsing de hayagriva sem cite()) → documentar discrepância;
  ajustar §3 e §4.

**Cenários específicos**:
- Bibliography/Cite estarem **totalmente ausentes** em cristalino
  → §3 estrutura A (multi-passo) torna-se mais provável; cada
  sub-passo M cobre uma camada (entries → cite → numbering).
- Bibliography/Cite estarem **parcialmente implementados** com
  features cristalinas custom (e.g. Vec literal sem hayagriva)
  → §3 estrutura B (minimal) torna-se mais viável; P159A pode
  refinar/expandir o existente.
- ADR-0017 ser bloqueador hard de cite() → §3 estrutura C
  (diferimento) torna-se viável; P159A escopo reduzido a
  Bibliography sem Cite, ou diferimento total registado.
- hayagriva ser dependência runtime obrigatória per ADR-0033
  paridade → ADR-0062 promovida a IMPLEMENTADO requer passo
  dedicado **antes** de P159A; ordem de passos altera-se.
- Subset "bibliography + cite" forçar quebra de granularidade
  em qualquer das 3 estruturas → §5 documenta com justificação;
  granularidade N=13 fica em risco mas decisão é informada por
  inventário, não inferida.

---

## Notas operacionais

- **P159 não materializa código**. Análogo estrutural a P156B,
  P156K, P157 e P158. Custo baixo; benefício alto para passos
  Bibliography/Cite subsequentes — particularmente importante
  porque XL declarado sugere maior risco de inferência errada.
- **Ordem dos sub-passos importa**: .1 (ADRs) precede .2
  (código) precede .3 (scope) precede .4 (dependências) —
  porque scope depende de saber o que ADRs definem e o que
  código já cobre, e dependências dependem de saber o subset
  decidido.
- **Auto-aplicação de ADR-0065 critério #5**: terceira aplicação
  concreta após P157 e P158. Padrão consolidado.
- **§análise de risco no relatório**: passo diagnóstico tem
  risco baixo. Manter §análise de risco preserva precedente
  N=12 → 13.
- **Política "sem novas reservas" preservada** (P158 estabeleceu;
  P158A respeitou; P159 respeita) — refinos pós-P159A
  permanecem candidatos NÃO-reservados.
- **Decisão crítica em §3**: avaliação das 3 estruturas
  possíveis (A multi-passo / B minimal / C diferimento) é
  diferenciador deste diagnóstico vs P157/P158. P159 declarado
  XL torna esta decisão particularmente importante.
- **Hash `entities/content.rs` provavelmente preservado** —
  oitavo passo consecutivo se confirmado. Padrão "estabilidade
  de contrato L0" continua a fortalecer-se.

---

## Pós-passo

Após conclusão de P159:
- **P159A** (ou identificador definido em .5) é o passo
  substantivo seguinte com scope concreto baseado em
  diagnóstico.
- Granularidade pode ser preservada (estrutura A multi-passo
  com sub-passos M cada; estrutura B minimal viável) ou
  quebrada (estrutura A com sub-passos M+; estrutura B
  forçando M+ em vez de M).
- Se P159A exigir trabalho prévio (e.g. ADR-0062 hayagriva
  promovida a IMPLEMENTADO antes de Bibliography materializar),
  esse trabalho prévio é redigido antes; P159A passa a ser
  passo final da cadeia.
- Se estrutura C (diferimento) for escolhida, P159A passa a
  ser passo administrativo XS de scope-out documentado per
  ADR-0054 graded — sem materialização real até Introspection
  runtime e/ou hayagriva estarem resolvidas.

ADR-0060 mantém-se IMPLEMENTADO. ADR-0061 mantém-se PROPOSTO.

ADR-0062 estado é factualmente confirmado em §1 (não
pré-decidido). Promoção a IMPLEMENTADO só é considerada se §1
revelar pré-existência de implementação ou §3 decidir incluir
hayagriva no scope P159A.

ADR-0017 estado é factualmente confirmado em §1. Promoção
fica fora de scope deste diagnóstico (Introspection runtime é
problema arquitectural maior que figure-kinds + bibliography
combinados).

Padrão granularidade 1-2 features/passo (N=13) NÃO é desafiado
por P159 (passo diagnóstico). Pode ser desafiado por P159A
consoante estrutura decidida em §3.

**Próxima decisão humana**: validação do diagnóstico §3 (scope
+ estrutura escolhida) e §5 (esboço P159A) antes de redigir
P159A.

**Reservas pendentes** (não criadas neste passo):
- ADR-0062 hayagriva — pré-existente; estado a confirmar em §1.
- ADR-0017 Introspection runtime adiada — pré-existente.
- Decisão sobre revogação ou manutenção destas reservas pode
  ser tomada em qualquer momento futuro per crítica de
  reservas travarem decisões.

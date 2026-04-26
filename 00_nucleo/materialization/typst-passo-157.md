# Passo P157 — Diagnóstico Model Fase 2 (table foundations)

Passo diagnóstico precede materialização — análogo a P156B para
Layout. **Não materializa código**. Inventaria estado actual de
Model, conteúdo de ADR-0060 (Model roadmap), estado de `grid`,
e define subset concreto de "table foundations" para passo
P157.1 substantivo a redigir após este diagnóstico.

Aplicação directa de **ADR-0065 critério #5** (scope: atributos
a incluir/diferir per ADR-0054 graded). Inventariar antes de
materializar é doutrina vinculativa per ADR-0065 quando há
decisão arquitectural não-trivial — neste caso, definição de
scope de table foundations.

---

## Estado actual antes de começar

- 63 ADRs após P156K (28 EM VIGOR; ADR-0064 Smart→Option e
  ADR-0065 inventariar-primeiro recém-formalizadas).
- Layout: 78% (13 puro + 1 ⁺ = 14/18) após P156L.
- Hash actual `entities/content.rs`: `ec58d849` (preservado em
  P156L).
- 1319 tests; zero violations linter.
- 52 variants Content; 42 stdlib funcs.
- Padrões consolidados: granularidade N=9; inventariar N=6;
  Smart→Option N=7; §análise risco N=6; reuso `Sides<T>` N=2.

**P157 era reservado** (per documento de estado pós-P156I) para
"Model Fase 2 (table foundations) per ADR-0060 renumerada".
**Conteúdo concreto desta reserva não está documentado** nos
ficheiros disponíveis ao redigir este enunciado:

- **ADR-0060** é referenciada mas não foi lida na sessão actual.
- **Cobertura específica de Model** não tem tabela equivalente
  à de Layout no inventário 148.
- **Estado de `grid`** desconhecido (`grid` é dependência
  vanilla de `table`).
- **Conteúdo de Model Fase 1** desconhecido (implícito pela
  existência de Fase 2).

Resolver estas lacunas é o objectivo de P157.

---

## Natureza do passo

**Tamanho**: S+.

**Justificação**: Trabalho documental puro. Inventário +
diagnóstico + decisão de scope para passo seguinte. Sem
modificação de código, sem ADR nova (ADR-0060 já existe e
governa Model — este passo lê-a, não cria nova).

Granularidade preservada: 1 deliverable diagnóstico → mantém
peso S+ análogo a P156K (passo meta documental).

**Risco baixo**: passo é **previne** risco em P157.1 (passo
substantivo seguinte) detectando lacunas factuais antes de
materialização. Auto-aplicação de ADR-0065 sem materialização
posterior bloqueada por este passo — é o passo.

---

## Decisões já tomadas

- **Identificador P157**: mantido per reserva original
  (decisão humana confirmada na sessão).
- **Natureza diagnóstica**: P157 deixa de ser materialização
  e passa a ser diagnóstico precedendo materialização. **P157.1**
  (ou outro identificador) será o passo substantivo a redigir
  após este diagnóstico.
- **Sem código alterado**: passo puramente documental.
- **Sem ADR nova**: ADR-0060 já governa Model; ADR-0061 governa
  Layout. P157 lê ambas, não cria nova.

## Decisões diferidas

- **Subset concreto de "table foundations"**: a decidir no
  sub-passo .3 com base em ADR-0060 (não a priori sem ler ADR).
- **Tamanho de P157.1**: a decidir no sub-passo .5 com base
  em scope determinado em .3. Pode ser M (granular preservado)
  ou L+ (quebra granularidade) consoante dependências de
  `grid`.
- **Dependência de `grid`**: a decidir no sub-passo .2. Se
  `grid` não existir em crystalline, P157.1 pode ter de
  precedêr-se de passo dedicado a `grid`.

---

## Sub-passos

### .1 Ler e resumir ADR-0060

Localizar e ler `00_nucleo/adr/typst-adr-0060-*.md`:
- Título completo da ADR.
- Status (PROPOSTO / EM VIGOR / IMPLEMENTADO / outro).
- Definição de fases de Model:
  - O que é Fase 1 (e seu estado).
  - O que é Fase 2 (escopo declarado).
  - Existem Fase 3+ planeadas?
- Lista de entradas vanilla cobertas por Fase 2.
- Definição concreta de "table foundations" se a ADR a usar.

Output: secção §1 do diagnóstico.

### .2 Inventariar estado de Model em código

Inspecção de `01_core/src/`:
- Variants `Content::*` relacionadas a Model (e.g. existe
  `Content::Table`? `Content::Grid`? `Content::Cell`?).
- Stdlib funcs em `rules/stdlib/model.rs` ou equivalente.
- Tipos auxiliares em `entities/` específicos a Model
  (e.g. `TableCell`, `GridTrack`).
- Hashes actuais dos ficheiros relevantes.

Inspecção de `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:
- Tabela A entrada `table` — estado.
- Tabela A entrada `grid` — estado.
- Tabela A entradas `figure`, `bibliography`, `cite` — estado
  (relevantes a P158/P159 reservados).
- Outras entradas Model identificadas no inventário 148.

Output: secção §2 do diagnóstico (inventário estruturado de
Model em código + cobertura).

### .3 Determinar scope de "table foundations"

Síntese de §1 e §2 do diagnóstico para responder:

1. Que entradas vanilla constituem "table foundations" per
   ADR-0060?
2. Dessas, quais já estão implementadas em crystalline (puro,
   parcial, ausente)?
3. Que dependências internas existem (e.g. `table` depende de
   `grid`)?
4. Qual é o subset mínimo materializável que preserva
   granularidade 1-2 features (N=9)?
5. Qual é o subset máximo que ainda cabe num passo M?
6. Subset mínimo vs máximo: recomendação para P157.1.

Output: secção §3 do diagnóstico (decisão de scope).

### .4 Identificar dependências bloqueantes

Para cada item do subset recomendado em §3:
- Existe pré-requisito em outro módulo?
- Existe DEBT aberto que bloqueie?
- Existe ADR pendente de promoção?
- Existe ADR-0017 (Introspection runtime) na cadeia de
  dependências?

Casos típicos a verificar:
- `table` exige `grid` (vanilla) — confirmar se aplicável a
  crystalline.
- `table.cell` exige modelo de coordenadas (linha, coluna).
- `table.header`/`table.footer` exigem repeat de cells em
  page breaks — possível dependência de Layout repeat (P156J)
  ou de algoritmo multi-region (DEBT-56).

Output: secção §4 do diagnóstico (dependências bloqueantes).

### .5 Definir P157.1 (ou identificador alternativo)

Com base em §1-§4:
- Identificador concreto do passo substantivo seguinte
  (P157.1 ou outro).
- Tamanho estimado (S/M/M+/L).
- Subset concreto de features (1-2 se granularidade preservável;
  mais se forçado por dependências).
- Sub-passos previstos a alto nível.
- Critério de granularidade: preservada (N=10) ou quebrada
  (caso registado).

Output: secção §5 do diagnóstico (esboço de P157.1).

### .6 Actualizar ADR-0061 §"Aplicações cumulativas"

Anotar P157 como passo diagnóstico análogo a P156B/P156K (passo
não-materialização). Tabela de slope cumulativo ganha linha
P157 com slope "—" e tests Δ "0".

Padrões metodológicos: inventariar-primeiro N=6 → 7 (ADR-0065
critério #5: scope).

### .7 Actualizar README ADRs (se aplicável)

Sem ADR nova; entrada cronológica de P157 adicionada antes de
P156L se a estrutura do README listar passos. Verificar
estrutura em .1 ou .7 conforme relevante.

---

## Verificação

Numerada para reporte de conclusão:

1. Diagnóstico
   `00_nucleo/diagnosticos/diagnostico-model-fase-2-passo-157.md`
   produzido com 5 secções (§1 ADR-0060; §2 estado código;
   §3 scope; §4 dependências; §5 esboço P157.1).
2. ADR-0060 lida e resumida em §1 (status confirmado;
   definição de fases citada literalmente).
3. Estado de `grid` em crystalline determinado factualmente
   (não inferido) em §2.
4. Estado de `table` em crystalline determinado factualmente
   em §2.
5. Subset concreto de "table foundations" definido em §3 com
   recomendação para P157.1.
6. Dependências bloqueantes (DEBTs, ADRs pendentes,
   pré-requisitos) listadas em §4.
7. Esboço de P157.1 em §5: identificador, tamanho, subset,
   granularidade.
8. ADR-0061 §"Aplicações cumulativas" actualizada com linha
   P157 (slope "—"; padrões N actualizados).
9. `crystalline-lint`: zero violations (sem código alterado;
   esperado trivial).
10. **Sem alteração de hashes** (passo documental; nenhum
    código modificado).

---

## Critério de conclusão

- Verificações 1-10 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-157-relatorio.md`
  produzido com:
  - Resumo do diagnóstico (síntese das 5 secções).
  - Decisão final de scope para P157.1.
  - Dependências identificadas a tratar antes de P157.1.
  - §análise de risco (padrão N=6 → 7; passo diagnóstico tem
    risco baixo, mas registar para preservar precedente).
  - Confirmação: ADR-0065 critério #5 (scope) aplicado pela
    primeira vez (auto-validação de ADR meta P156K).

---

## O que pode sair errado

**Cenários gerais**:
- ADR-0060 não existe ou tem outro número → grep em `00_nucleo/adr/`
  por palavras-chave "Model", "table", "Fase". Documentar
  ADR real em §1.
- ADR-0060 tem scope diferente de "Model roadmap" (e.g. é
  sobre outro tópico) → reserva P157 em documento de estado
  estava errada; documentar discrepância e propor renumeração
  ou nova reserva.
- Tabela de cobertura não tem secção dedicada a Model
  (apenas tabela A user-facing) → fazer inventário de novo,
  marcando o gap como achado P157.

**Cenários específicos**:
- `grid` não estar implementado e ser dependência hard de
  `table` → P157.1 expande para incluir `grid`, ou P157.1
  precede-se de passo dedicado a `grid`. Decisão registada
  em §3 e §5.
- `table` em crystalline ter decisão arquitectural diferente
  de vanilla (e.g. não depender de grid) → documentar em §3
  como divergência consciente; verificar se há ADR a justificar
  ou se é gap arquitectural a registar como DEBT novo.
- Subset "table foundations" forçar quebra de granularidade
  (todos os caminhos exigem M+/L) → registar em §5 com
  justificação; granularidade N=9 fica em risco mas decisão
  é informada por inventário, não inferida.

---

## Notas operacionais

- **P157 não materializa código**. Análogo estrutural a P156B
  (diagnóstico Layout) e P156K (ADRs meta). Custo baixo;
  benefício alto para passos Model subsequentes.
- **Ordem dos sub-passos importa**: .1 (ADR) precede .2 (código)
  precede .3 (scope) — porque scope depende de saber o que
  ADR define e o que código já cobre.
- **Auto-aplicação de ADR-0065 critério #5**: este próprio
  passo é exemplo de "scope determinado por inventário". A
  decisão de scope (sub-passo .3) é exactamente o critério #5
  em acção.
- **§análise de risco no relatório**: passo diagnóstico tem
  risco baixo. Manter §análise de risco preserva precedente
  N=6 → 7. Sexta aplicação consecutiva (P156F/G/H/I/J/K/L).
- **Reservas de numeração**: P158 (figure-kinds) e P159
  (bibliography + cite) mantêm-se. P157.1 é nome provisório
  para passo substantivo — pode ser P157A se a sessão preferir
  numeração com letras (precedente P156C-L), ou outro
  identificador a decidir em .5.

---

## Pós-passo

Após conclusão de P157:
- **P157.1 / P157A** (ou identificador definido em .5) é o
  passo substantivo seguinte com scope concreto baseado em
  diagnóstico.
- Granularidade pode ser preservada (esperado se subset for
  pequeno) ou quebrada (registado se inventário forçar).
- Se P157.1 exigir trabalho prévio (e.g. `grid` antes de
  `table`), esse trabalho prévio é redigido antes; P157.1
  passa a ser o passo final da cadeia.

ADR-0061 mantém-se PROPOSTO (Layout não tocado em P157).

ADR-0060 estado é factualmente confirmado em §1 (não pré-decidido).
Promoção/revogação de ADR-0060 só é considerada se §1 revelar
inconsistência factual com reservas documentadas.

Padrão granularidade 1-2 features/passo (N=9) NÃO é desafiado
por P157 (passo diagnóstico, não materialização). Pode ser
desafiado por P157.1 consoante scope decidido em §3.

**Próxima decisão humana**: validação do diagnóstico §3 (scope)
antes de redigir P157.1.

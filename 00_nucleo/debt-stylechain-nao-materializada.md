# DEBT-XX — StyleChain não materializada (divergência cristalino vs vanilla)

**Estado**: aberto
**Data de abertura**: 2026-05-02
**Origem**: P182A (diagnóstico lacuna #4 `numbering_active`).
**Magnitude**: L (mecanismo arquitectural; afeta múltiplas
features actuais e futuras).
**Pré-condição de fecho**: nenhuma técnica imediata; depende de
prioridade humana.

---

## Contexto

Vanilla resolve propriedades hierárquicas com `StyleChain` —
estrutura de escopo léxico encadeado onde cada elemento pergunta
"qual é o meu valor de X neste ponto do documento?" e a estrutura
responde com herança e override por contexto. Set rules dentro de
um container ficam escopadas ao container; elementos fora não
herdam o override.

Cristalino não materializou StyleChain. Features que precisariam
de scoping léxico foram resolvidas com mecanismos globais por
chave (`HashMap<String, T>` em `CounterStateLegacy`,
`StateRegistry` P171). Comportamento: set rule é global a partir
do ponto onde aparece; não respeita escopo léxico de container.

---

## Motivo do adiamento

StyleChain é sub-sistema substancial em vanilla (resolução em
árvore, herança, scoping, integração com tipos de propriedade
heterogéneos). Materializar em cristalino é trabalho L+ (estimativa
ordens de grandeza acima dos passos típicos S-M).

Em cada feature individual, o custo de evitar StyleChain (usar
state global) foi pequeno e a maior parte dos documentos não exerce
o caso de scoping hierárquico. As features acumulam-se mas cada
decisão pontual foi defensável.

P182A identificou explicitamente a divergência ao migrar
`numbering_active` (lacuna #4) para `StateRegistry` em vez de
StyleChain. Decisão consciente de manter forma cristalina.

---

## Features actualmente afetadas

Identificadas durante P182A inventário:

1. **`numbering_active` (heading + equation)** — `HashMap<String, bool>`
   global em `CounterStateLegacy`; migra para `StateRegistry` via
   P182. Vanilla resolve via `Option<Numbering>` em
   `HeadingElem.numbering` + `EquationElem.numbering` com StyleChain.

2. **Set rules de styling** (`Bold`, `Italic`, etc.) — cristalino
   resolve via `Content::Styled` wrapper explícito; vanilla resolve
   via `set rule` + StyleChain. Cristalino exige wrapping manual
   onde vanilla permite set global. Documentado em ADR-0026.

3. **Outras features que dependem de set rules hierárquicos**: a
   identificar quando forem materializadas. Auditoria fresh F2
   (`Content` 59 variants) sugere que mais features podem cair
   nesta categoria à medida que cristalino aproxima cobertura
   vanilla.

Lista cresce com auditorias futuras. P182A é primeira instância
explicitamente registada.

---

## Consequências da divergência

**Documentos típicos**: output observable idêntico a vanilla. Set
rule no início do documento, ou toggle linear, funciona em ambos.

**Documentos com containers paralelos** que querem propriedades
diferentes (ex.: figura com numeração ligada dentro de capítulo
sem numeração): vanilla suporta nativamente; cristalino exige
toggles manuais antes/depois de cada container. É possível mas é
fricção visível.

**Modelo mental do utilizador**: quem aprende cristalino lendo
documentação vanilla pode tentar set rule local esperando scoping
e o comportamento difere silenciosamente. Sem mensagem de erro.

**Auditoria de cobertura**: divergência permanente até DEBT
fechar. Auditor que mede paridade observable encontra excepção
nesta área — não é bug, é divergência registada.

---

## Critério de fecho

DEBT fecha quando todas estas condições forem verdadeiras:

1. Estrutura `StyleChain` (ou equivalente funcional com nome
   diferente) materializada em `01_core/src/entities/`.
2. Resolução hierárquica de propriedades com escopo léxico
   funciona — testes E2E confirmam set rule local não vaza para
   irmãos no mesmo nível.
3. Cada feature listada em "Features actualmente afetadas"
   migrada para StyleChain ou explicitamente registada como
   "fica em state global por design" com ADR.
4. Auditoria de cobertura vanilla-vs-cristalino na área de
   styling/numbering deixa de listar a divergência como ponto
   diferenciado.

Fecho parcial não é fecho. Materializar StyleChain sem migrar
features afetadas mantém o DEBT aberto até as migrações
concluírem.

---

## Caminhos possíveis (sem ordem de preferência)

- **Caminho A** — passo dedicado L+ que materializa StyleChain
  do zero, com posterior série de migrações feature-a-feature.
  Magnitude grande mas previsível.
- **Caminho B** — materialização incremental: cada nova feature
  que precisaria de scoping força a decisão (StyleChain agora ou
  state global temporariamente). DEBT cresce até pressão
  acumulada justificar Caminho A.
- **Caminho C** — divergência permanente: aceitar que cristalino
  não vai ter StyleChain, fechar DEBT como "wontfix" via ADR. Só
  defensável se ficheiros futuros confirmarem que nenhuma feature
  prioritária precisa de scoping.

Decisão sobre caminho fica para passo futuro quando alguma das
três condições aparecer:
- nova feature exige scoping e não pode ser feita sem (forçando A
  ou B);
- pressão acumulada de DEBTs relacionados torna A mais barato que
  continuar B;
- auditoria confirma que C é seguro.

---

## Referências

- P182A — `diagnostico-numbering-active-passo-182a.md` §3
  (cláusula 1, decisão M1) — primeira instância explicitamente
  registada.
- P171 — StateRegistry como infraestrutura usada em vez de
  StyleChain.
- ADR-0026 — `Content::Styled` wrapper como decisão paralela
  para set rules de styling.
- Auditoria fresh 2026-04-29 — F2 (Content 59 variants) sugere
  superfície grande potencialmente afetada.
- `m1-lacunas-captura.md` — lacuna #4 (numbering_active) que
  expôs o DEBT.
- Vanilla `lab/typst-original/crates/typst-library/src/foundations/styles.rs`
  (ou equivalente actual) — implementação StyleChain de
  referência.

---

## Notas operacionais

- DEBT não bloqueia P182B–F. Implementação directa avança com
  decisão registada em P182A.
- Cada feature futura que toque scoping deve referenciar este
  DEBT no relatório do passo.
- Auditorias periódicas de cobertura (instrumento
  `auditoria-cobertura-instrucao-claude-code.md`) devem listar
  features afetadas como divergência intencional registada,
  não como trabalho por fazer.

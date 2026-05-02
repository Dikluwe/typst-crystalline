# ⚖️ ADR-0067: Attribute-grammar pattern para scoping de propriedades

**Status**: `PROPOSTO`
**Validado**: pendente — não vinculativo até primeira materialização.
**Data**: 2026-05-02
**Diagnóstico prévio**:
- `00_nucleo/diagnosticos/diagnostico-numbering-active-passo-182a.md` (P182A).
- DEBT-XX (StyleChain não materializada).

---

## Contexto

Vanilla typst resolve scoping de propriedades com `StyleChain` —
estrutura de cons-list encadeada por referência, construída
durante a fase de "realização" do documento, passada como
parâmetro pervasivo aos consumers (eval, realize, layout,
foundations).

Cristalino divergiu desta forma desde o início. Features
afectadas (`numbering_active`, set rules de styling) foram
materializadas com state global por chave (`HashMap<String, T>`
em `CounterStateLegacy`, `StateRegistry` P171). Set rules são
globais a partir do ponto onde aparecem; não respeitam escopo
léxico de container.

A divergência foi identificada explicitamente em P182A
(diagnóstico lacuna #4) e registada em DEBT-XX. Discussão
posterior à abertura do DEBT identificou um caminho
arquitectural não considerado nos 3 caminhos originais do
DEBT: **attribute-grammar pattern**.

Esta ADR regista a direcção arquitectural preferida sem
fechar prematuramente as alternativas.

---

## Decisão

Cristalino adopta **attribute-grammar pattern** como mecanismo
preferido para scoping léxico de propriedades, em vez de
replicar `StyleChain` cons-list (vanilla) ou manter state global
indefinidamente.

Princípio: cada propriedade scopável **declara junto da sua
definição** duas funções:

- **herança** — como o valor passa do pai para os filhos durante
  o walk;
- **combinação** (folding) — como múltiplos valores no mesmo
  nível se combinam, quando aplicável (override last-wins é o
  default; outras propriedades podem declarar fold custom).

Estas declarações vivem perto da definição da propriedade, não
num módulo central de scoping.

Status `PROPOSTO`: a ADR regista a direcção. Validação ocorre
quando a primeira propriedade for materializada com este
pattern. Se a materialização revelar problemas estruturais não
antecipados, ADR transita para `REJEITADO` ou é revista; senão
transita para `IMPLEMENTADO`.

---

## Esboço técnico

### Forma da declaração de propriedade

Cada propriedade scopável tem uma declaração com forma
aproximada:

```rust
pub struct ScopedProp<T> {
    /// Valor inicial quando nenhum scope define a propriedade.
    pub initial: T,
    /// Como o valor passa do pai para os filhos.
    /// Default: identity (filho herda directamente).
    pub inherit: fn(parent: &T) -> T,
    /// Como múltiplos valores no mesmo scope se combinam.
    /// Default: last-wins (substituição directa).
    pub combine: fn(prev: &T, new: &T) -> T,
}
```

A forma exacta da assinatura é decisão da primeira
materialização (P_alvo). Pode ser trait, pode ser struct com
function pointers, pode ser macro. ADR não fixa a forma;
fixa o princípio.

### Integração com walk

Walk recebe um parâmetro adicional `inherited: &InheritedAttrs`
contendo os valores actuais de cada propriedade scopável.
Quando walk desce numa subárvore que define um set rule, o
parâmetro passa-se modificado para a sub-chamada — sem mutar o
parâmetro original.

Walk continua puro (P163 invariante preservada): atributos
herdados são argumento de função, não state mutável.

### Integração com `Content::Styled` (ADR-0026)

`Content::Styled([StyleDelta::...], body)` já é o mecanismo de
"set rule inline" estabelecido em ADR-0026. A integração é
directa: walk arm `Content::Styled` aplica os deltas ao
`InheritedAttrs` antes de descer no body, e restaura ao
sair.

`Content::Styled` deixa de ser apenas wrapper de styling para
ser a forma canónica de "abrir scope" para qualquer propriedade
scopável. Set rule de qualquer propriedade torna-se um delta
em `Content::Styled`.

### Integração com `Introspector`

Propriedades scopáveis ficam **fora** de `Introspector`. O
attribute-grammar pattern resolve scoping durante o walk; o
valor já está calculado quando o consumer precisa dele.

`Introspector` continua a expor state global e queries
location-aware para o que **não** é scopável (numeração de
elementos, contagens, bibliografia, etc.).

A separação é intencional:
- **Scopável** (depende do contexto léxico) → attribute-grammar.
- **Global** (depende do documento como um todo) → Introspector
  + StateRegistry.

Algumas propriedades existem em ambos os domínios. Decisão por
propriedade.

### Paridade com vanilla

Para propriedades override-simples (a maioria), paridade
observable é trivial.

Para propriedades fold-style (vanilla `WeightDelta` é o exemplo
documentado), paridade exige declaração explícita da função
`combine`. Não vem "de graça".

Para propriedades com resolução tardia (ex.: `1em` resolvido
relativamente ao pai), paridade exige decisão sobre quando
resolver. Não vem "de graça".

A ADR aceita que paridade é alcançável mas não automática.
Cada propriedade individual decide o seu nível de paridade
durante materialização.

---

## Propriedades alvo identificadas

Lista das propriedades para as quais este pattern é candidato.
Não exaustiva — cresce com auditorias futuras.

### Identificadas em P182A

1. **`numbering_active`** (heading, equation) — boolean
   override-simples. Caso mais simples possível. Candidato a
   primeira materialização (validação rápida do pattern).

### Identificadas em ADR-0026

2. **Set rules de styling** — `Bold`, `Italic`, peso de fonte,
   tamanho, família, cor, alinhamento, etc. Cristalino actual
   resolve via `Content::Styled` wrapper explícito. Migração
   para attribute-grammar mantém `Content::Styled` mas
   automatiza propagação por scope léxico.

### Categoria aberta

3. **Outras propriedades scopáveis a identificar** —
   auditorias periódicas (instrumento
   `auditoria-cobertura-instrucao-claude-code.md`) podem
   identificar mais features que beneficiariam do pattern.
   Adições à lista são incrementais, não exigem revisão
   desta ADR.

---

## Alternativas consideradas

| Alternativa | Forma | Decisão |
|-------------|-------|---------|
| **Caminho 1** — manter state global por chave | `HashMap<String, T>` global; `StateRegistry` location-aware | rejeitado como direcção permanente — não resolve scoping léxico |
| **Caminho 2** — replicar StyleChain cons-list (vanilla) | Estrutura central pervasiva passada por todo o código | rejeitado como direcção preferida — viola princípio de isolamento por feature (ADR-0036, ADR-0037); paridade fácil mas custo de manutenção alto |
| **Caminho 3** — attribute-grammar pattern | Declaração por propriedade, atributos herdados como argumento de walk | **escolhido** — alinha com isolamento por feature já estabelecido em sub-stores P165/P169/P171/P177/P181 |
| **Caminho 4** — persistent hash trie | Forma alternativa de Caminho 2 com lookup O(log n) | rejeitado como direcção primária; fica como optimização futura possível se Caminho 3 mostrar gargalo |

Critério decisivo: alinhamento com princípios de isolamento já
estabelecidos no projecto (ADR-0036 atomização, ADR-0037 coesão
por domínio, padrão sub-store).

---

## Consequências

### Positivas

- **Localidade**: cada propriedade vive perto da sua definição.
  Para entender comportamento de uma propriedade, lê-se um
  sítio.
- **Testabilidade**: testes unitários por propriedade, sem
  setup global.
- **Escalabilidade**: adicionar nova propriedade scopável =
  declaração isolada. Não interfere com outras propriedades.
- **Compatibilidade com walk puro**: atributos herdados são
  argumento de função, não state. P163 invariante preservada.
- **Coerência arquitectural**: alinha com sub-stores
  (P165/P169/P171/P177/P181) e ADR-0026 (`Content::Styled`).
- **Refactor distribuído**: dívida técnica fica localizada na
  declaração da propriedade afectada, não acumula no centro.

### Negativas

- **Paridade com vanilla não vem "de graça"**: cada propriedade
  fold-style ou com resolução tardia exige decisão explícita.
- **Custo intelectual maior por propriedade**: declarar herança
  e combinação exige pensar caso-a-caso.
- **Propriedades que interagem entre si** (ex.: `font-size`
  afecta `line-height`) exigem coordenação explícita ou ordem
  de avaliação declarada.
- **Performance de lookup** pode exigir abstração comum por
  baixo se cada propriedade calcular do zero. Optimização
  futura.

### Neutras

- Vocabulário diferente do vanilla. Quem aprende cristalino
  lendo documentação vanilla pode ter de re-mapear conceitos
  ("StyleChain" não tem equivalente directo; "attribute
  herdado" tem).
- ADR-0026 (`Content::Styled`) ganha papel mais central.
  Decisão dessa ADR continua válida; o uso aumenta.

---

## Plano de validação

ADR-0067 transita de `PROPOSTO` para `IMPLEMENTADO` quando todas
estas condições forem verdadeiras:

1. Pelo menos uma propriedade alvo (provavelmente
   `numbering_active`) materializada com o pattern.
2. Walk recebe o parâmetro de atributos herdados sem violar
   P163.
3. Tests E2E confirmam scoping léxico funciona — set rule num
   `Content::Styled` não vaza para irmãos.
4. Comparação de magnitude com Caminho 2 hipotético confirma
   que Caminho 3 é praticável (não substancialmente mais caro
   sem benefício).

ADR transita para `REJEITADO` se durante materialização for
descoberto problema estrutural não antecipado (ex.: integração
com walk impossível sem violar pureza, performance
inaceitável, complexidade de coordenação entre propriedades
torna o pattern impraticável).

Se ADR for rejeitada, DEBT-XX permanece aberto e os caminhos
alternativos voltam à mesa.

---

## Plano de materialização

Esta ADR não fixa o passo concreto de materialização. Sugestões
não vinculativas:

- **Passo dedicado** para o mecanismo (struct/trait
  `ScopedProp` + integração walk + framework de declaração).
  Magnitude provavelmente M.
- **Passo subsequente** materializa primeira propriedade
  (`numbering_active`) com o pattern. Magnitude S, valida o
  mecanismo.
- **Passos futuros** migram propriedades existentes uma a uma,
  ou adicionam novas. Cada um magnitude S por propriedade.

Ordem inversa também é defensável (`numbering_active` primeiro
sem framework, depois extrair pattern reutilizável). Decisão
fica para passo de inventário antes da materialização.

---

## Referências

- ADR-0026 — `Content::Styled` wrapper (mecanismo "set rule
  inline" que esta ADR generaliza).
- ADR-0033 — convenções gerais de eval/walk.
- ADR-0036 — atomização (cada feature tem consumer explícito).
- ADR-0037 — coesão por domínio (features vivem perto da sua
  definição).
- P163 — walk puro como invariante.
- P165, P169, P171, P177, P181 — padrão sub-store que esta ADR
  estende para a categoria de propriedades scopáveis.
- P182A — diagnóstico lacuna #4 onde a divergência foi
  primeiro registada explicitamente.
- DEBT-XX — StyleChain não materializada; ADR-0067 refina os
  caminhos do DEBT nomeando Caminho 3.
- Vanilla `lab/typst-original/crates/typst-library/src/foundations/styles.rs`
  — implementação StyleChain de referência (Caminho 2
  rejeitado).
- Wikipedia "Attribute grammar" — fundamento teórico (atributos
  herdados, Knuth + Wegner).
- Haskell Reader monad — equivalente funcional ao pattern em
  outra linguagem.

---

## Notas operacionais

- Esta ADR não bloqueia P182B–F. Migração actual de
  `numbering_active` para `StateRegistry` continua. Quando o
  pattern for materializado, `numbering_active` pode ser
  re-migrado.
- DEBT-XX deve ser actualizado para nomear Caminho 3 e
  referenciar ADR-0067. Critério de fecho do DEBT pode ser
  refinado para "ADR-0067 implementada + propriedades
  identificadas migradas".
- Cada passo futuro que materialize propriedade scopável deve
  declarar: usa attribute-grammar (esta ADR) ou diverge
  conscientemente.

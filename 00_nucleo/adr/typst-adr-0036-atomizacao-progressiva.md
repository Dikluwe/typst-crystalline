# ⚖️ ADR-0036: Atomização progressiva — estado partilhado como dívida

**Status**: `EM VIGOR`
**Data**: 2026-04-22

---

## Contexto

A série de passos 85–90 revelou que o modelo `EvalContext`
actual do cristalino tem características que se afastam do ideal
arquitectural declarado para o projecto. Concretamente:

1. **Funções `eval_*` recebem `&mut EvalContext`** com vinte ou
   mais campos mutáveis. A assinatura da função declara uma
   dependência abstracta ("precisa de contexto") mas esconde as
   dependências reais (quais campos específicos lê ou muta).

2. **Modificação de uma função pode afectar outras** via campos
   partilhados sem que a assinatura indique. Exemplo: função
   nova que muta `ctx.styles` afecta funções chamadas depois
   que lêem esse campo, sem nenhuma pista sintáctica da
   dependência.

3. **Testabilidade é aparente, não real**. Cada teste de função
   `eval_*` requer construir `EvalContext` inteiro, mesmo que
   a função só leia 2 dos 20+ campos. Isto encoraja testes
   frágeis ligados a estrutura interna.

4. **`Route<'a>` no Passo 90 confirmou o padrão correcto**: o
   vanilla propaga `Route<'a>` por valor entre frames, não
   mantém estado partilhado num contexto mutável. O cristalino
   materializou `Route<'a>` mas por decisão pragmática manteve
   `Vec<FileId>` dentro do `EvalContext` (documentado em
   DEBT-44). A integração estrutural fica como trabalho futuro.

Esta ADR formaliza o princípio que guia a redução progressiva
desta dívida, começando pela integração do `Route<'a>` e
estendendo-se aos outros campos do `EvalContext` em passos
subsequentes.

---

## Decisão

**Funções de L1 declaram explicitamente todas as dependências
na assinatura.** Estado partilhado mutável é dívida
arquitectural a reduzir progressivamente.

Regras operacionais:

### Regra 1 — Declaração explícita

Uma função que depende de estado X recebe X (ou uma referência
a X) como parâmetro. Não adquire X via `&mut self` de um
contexto que contém X entre outros campos.

Exemplo conforme:

```rust
fn eval_expr(&mut self, route: &Route<'_>, expr: &Expr) -> Value
```

Exemplo não-conforme (estado escondido):

```rust
fn eval_expr(&mut self, expr: &Expr) -> Value {
    if self.import_stack.contains(&id) { ... }  // dependência não declarada
}
```

### Regra 2 — Agrupamento deliberado

Quando múltiplas dependências relacionadas aparecem juntas em
muitas funções, agrupá-las num tipo dedicado é preferível a
passá-las individualmente. O vanilla faz isto com `Engine<'a>`.

**A diferença face ao `EvalContext` monolítico não é o
agrupamento — é que o agrupamento vanilla é:**
- **Construído por escopo** (novo `Engine<'a>` para cada
  sub-computação recursiva).
- **Imutável onde possível** (campos como `Route<'a>` são
  `&`, não `&mut`).
- **Mínimo** (cada campo tem razão arquitectural documentada).

### Regra 3 — Redução progressiva, não refactor monolítico

O `EvalContext` actual não é desfeito de uma vez. Cada passo
futuro pode extrair um campo (ou grupo coerente de campos) e
convertê-lo em parâmetro declarado. A ordem respeita:

- **Pontos de recursão** primeiro (estado que muda entre frames
  recursivos é o mais problemático).
- **Estado imutável** em seguida (pode ser extraído para
  referência compartilhada).
- **Estado genuinamente global à avaliação** por último (ou
  permanecer no contexto se não houver alternativa melhor).

### Regra 4 — Excepções permitidas

Alguns campos do `EvalContext` podem legitimamente permanecer
lá:

- **Colectores de resultados** (diagnósticos, introspecção):
  estado acumulativo da avaliação inteira, não estado
  semântico.
- **Limites de recursos** (profundidade máxima, tempo máximo):
  cross-cutting concerns que atravessam todas as funções.
- **Inicialização estática** (biblioteca padrão, configuração):
  imutável após construção.

Estas excepções são registadas explicitamente. Um campo que
não cabe em nenhuma das categorias acima é candidato a
extracção.

### Regra 5 — Divergência do vanilla permitida mas registada

Se uma decisão arquitectural do cristalino divergir do vanilla
(ex: não criar `Engine<'a>` equivalente, agrupar campos de
forma diferente), a divergência é registada em ADR ou no
comentário do tipo. A ADR-0033 cobre este caso — divergência
estrutural é aceitável, divergência em comportamento observável
não é.

---

## Alternativas Consideradas

| Alternativa | Razão rejeitada |
|-------------|-----------------|
| Refactor monolítico do `EvalContext` num único passo | Escopo demasiado grande; alto risco de regressão. |
| Manter `EvalContext` como está (não atomizar) | Contradiz o objectivo declarado do projecto (peças substituíveis); acumulação contínua de dívida. |
| Copiar `Engine<'a>` do vanilla literalmente | Pressupõe que todos os campos do `Engine` são necessários; algumas decisões vanilla são herança histórica. |
| Atomização por injecção de dependência (traits) | Complexidade alta; ofusca estado real; padrão rejeitado no Passo 86. |

---

## Consequências

### Positivas

- Funções ganham contratos legíveis. Ler assinatura basta para
  saber de que depende a função.
- Testes isolados tornam-se práticos. Construir `Route<'_>`
  ou `StyleChain<'_>` para teste é mais leve que construir
  `EvalContext` completo.
- Refactoring localizado. Mudar a implementação de uma função
  não exige entender o contexto global.
- Convergência incremental com o vanilla sem refactor
  traumático.

### Negativas

- Cada passo de atomização é trabalho mecânico. A série
  completa pode levar 5–15 passos dependendo dos agrupamentos.
- Algumas assinaturas ficam mais longas antes do agrupamento
  ser feito. Estado intermédio menos legível que destino final.
- Risco de extrair campos na ordem errada e ter de reverter.
  Mitigação: começar pelos campos claramente recursivos
  (`Route`, `StyleChain`).

### Neutras

- A política "sub-passos para DEBTs" aplica-se — cada extracção
  paga parte de um DEBT agregador (ver DEBT-46 se aberto) ou
  DEBT específico.
- Funções não-L1 (L2 shell, L3 infra, L4 wiring) não são
  directamente abrangidas. A ADR aplica-se a L1. L2-L4 podem
  usar estado partilhado quando encapsula I/O ou orquestração.

---

## Plano de redução progressiva

Ordem sugerida de campos candidatos a extracção do `EvalContext`
(não vinculativa — cada passo avalia viabilidade):

1. **`Route<'a>`** (DEBT-44) — estado fundamentalmente recursivo;
   ganho estrutural claro.
2. **`StyleChain<'_>`** — propagação entre frames ao entrar/sair
   de blocos; estrutura similar ao `Route`.
3. **`Scopes<'a>`** — já é `Scopes<'a>` mas pode ter interacção
   com o contexto que merece revisão.
4. **`show_rules`** — DEBT-22 já converteu para `Arc<[ShowRule]>`;
   propagação como parâmetro pode ser próxima.
5. **Outros campos** — avaliar caso a caso.

A atomização de cada campo corresponde a um passo dedicado ou
a um sub-passo integrado em outra materialização.

---

## Referências

- ADR-0029 — pureza física de L1 (contexto relacionado:
  L1 é puro mas pode ter estado; este ADR clarifica que
  estado preferencialmente é declarado).
- ADR-0030 — performance é domínio de L1.
- ADR-0032 — política de `unsafe` em L1 (contexto: decisões
  similares de "reduzir progressivamente").
- ADR-0033 — paridade funcional com vanilla (contexto:
  divergências estruturais são aceitáveis; este ADR orienta
  quando devem reduzir-se).
- Passo 86 — diagnóstico do padrão vanilla.
- Passo 90 — primeira aplicação parcial (materialização do
  `Route<'a>` sem integração).
- DEBT-44 — integração estrutural do `Route<'a>` (primeiro
  pagamento concreto deste ADR).
- DEBT-45 — `check_*_depth` não chamados (relacionado mas
  ortogonal).

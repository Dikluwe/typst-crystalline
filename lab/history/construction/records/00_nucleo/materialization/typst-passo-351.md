# Passo 351 — F-realização, fatia 1: transporte `StyledElem`-scoped + `#show` léxico (caso 4)

> **O que faz.** Primeira fatia da **F-realização** (o próximo da fila F). Constrói o
> **transporte multi-passe `StyledElem`-scoped** (a fundação) e, sobre ele, implementa o
> **escopo léxico do `#show`** — o **caso 4** do spike-2: bloco de conteúdo `[]` **vaza**
> para o irmão; bloco de código `{}` **confina**; `#set` já está escopado. Os casos 1
> (composição) e 3 (show-set) ficam para P352/P353. O caso 2 (recursão) **já está fechado**
> pela linha P342–P350c — este lote **não o toca**. **NÃO content-preserving**: o escopo
> léxico é comportamento observável que passa a casar com o vanilla; as asserções que
> mudarem são só as ligadas ao novo escopo, cada uma justificada contra a harness de
> paridade (o oráculo). Decisão de camada e contagem de sítios **produzidas pela Fase A**
> (ADR-0108 regra 1), não assumidas.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P351 (confirmar livre — o passo de releitura do plano que ocuparia o
P351 não foi necessário: o `f-plano-lotes-passo-333.md` foi fornecido diretamente, então
nenhum passo de código foi consumido antes deste).
**Pré-condição**: P350c fechado — a linha da recursão encerrada (igualdade morfológica
P345, terminação por ponto-fixo, classificação cíclico/não-convergente sob a flag, mensagem
base byte-idêntica ao vanilla); a flag de erro completo com capacidade interna implementada;
a CLI registrada como **DEBT-59**. HEAD pós-P350c, árvore limpa, lint 0/0, suíte verde no
número que o P350c deixou (confirmar contra HEAD; **não** assumir contagem). Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar o que diverge.
**Tipo**: F-realização (realização do `#show` na camada `rules/`) — **NÃO
content-preserving** (adiciona o caminho do `#show` léxico + o transporte). A exceção ao
critério 1 do plano é declarada: o caso 4 muda escopo observável; só as asserções de escopo
mudam, e mudam **para casar com o vanilla**, justificadas contra a harness.

---

## Leituras da Fase A (a fonte, não a memória)

Antes de qualquer código, ler no repo e registrar com `file:line`:

1. **L0 da realização** — `entities/f_fronteira_e1.md` (a seção da realização / `#show` sob
   a fronteira E1). É a fonte do contrato; sincronizar o hash antes do código (Trava
   arquitetural, CLAUDE.md; critério 5 do plano).
2. **Requisitos do spike-2** — `f-spike2-show-passo-333.md`: os S* relevantes ao caso 4
   (escopo). Confirmar quais S* esta fatia consome e quais ficam para P352/P353.
3. **Recon dimensionado** — `f-recon-passo-337.md`: a contagem real dos sítios show-state
   e a quebra dos 20 testes de `#show` (o plano diz "1 vira, 18 revisão, 1 permanece" para a
   F-realização **inteira** — a Fase A separa quantos caem **nesta fatia** vs nas seguintes).
4. **A semântica do escopo no vanilla** — leitura autorizada da quarentena, nunca importar:
   o transporte multi-passe `StyledElem`-scoped em `lab/.../content/mod.rs:744-752`; e o
   ponto do vazamento `[]` vs confinamento `{}` que o plano aponta em `eval/mod.rs:460`
   (confirmar se a linha ainda bate; registrar a real).

Se alguma leitura contradisser o plano (ex.: o transporte sozinho já enche a faixa
validada), registrar como achado e aplicar a **válvula** (abaixo) — não inchar o lote.

---

## Limites duros

- **O caso 2 (recursão) não é tocado.** A terminação por ponto-fixo, a igualdade
  morfológica (`morph_canon`, P345) e o `PartialEq` do Rust ficam **intactos**. A
  realização do escopo **usa** o `==` morfológico para comparar nós; **não** o altera.
- **A flag de diagnóstico (P350c) não é tocada**, e o **DEBT-59** (exposição na CLI)
  **continua débito** — este lote não expõe nada na CLI.
- **A mensagem base de erro fica byte-idêntica ao vanilla** (ADR-0033) — esta fatia não
  mexe em erro; se algum caminho de erro for atravessado, a base não muda.
- **Os 65 nativos permanecem monomórficos.** O `#show` léxico toca o eager dos nativos pelo
  caminho que o F-3 inc-2 já abriu (`Selector::DynKind`, guard por `RuleId`); confirmar que
  a **Trava ADR-0105 cláusula 3** já existe (construída em F-1/F-2/F-3) **antes** de relaxar
  o compilador no caminho dinâmico. Se não existir para este caminho, construí-la neste lote
  é pré-requisito (não relaxar sem a trava).
- **`#set` não é re-escopado** — o plano confirma que `#set` já está escopado; esta fatia
  só faz o `#show` ganhar o mesmo escopo léxico.
- **Marco G não é tocado.** `edges(content → elements::*)` deve continuar **66** (a fila F
  não corta o acoplamento; isso é o Marco G, pós-F-6, spec própria). Confirmar inalterado
  pela lente.

---

## Estágios

### Estágio 0 — fundação: o transporte `StyledElem`-scoped
Construir o transporte multi-passe escopado por `StyledElem` (a forma confirmada na Fase A
contra `lab/.../content/mod.rs:744-752`). É a peça que o caso 4 monta em cima e que os casos
1 e 3 (P352/P353) também vão usar. Se o transporte sozinho atingir a faixa validada
(~110–150 sites, ou o que a Fase A medir como teto), **parar aqui** e o caso 4 vira P352 —
ver válvula.

### Estágio 1 — caso 4: o escopo léxico do `#show`
Sobre o transporte: bloco de conteúdo `[]` deixa a regra `#show` **vazar** para o irmão;
bloco de código `{}` **confina** a regra. O ponto é o que a Fase A confirmar em
`eval/mod.rs:460`. `#set` já escopado — não mexer.

### Estágio Teste
- Os testes de `#show` que esta fatia altera (subconjunto dos 20, separado na Fase A):
  cada asserção que mudar é de **escopo** e muda **para casar com o vanilla** — justificar
  uma a uma contra a harness de paridade. Os que não são de escopo **não mudam**.
- Um teste prova `[]` vaza para o irmão; um prova `{}` confina; um prova `#set` continua
  escopado (não regrediu).
- Confirmar por construção/teste que o caminho da recursão (caso 2) e a flag (P350c) não
  foram alterados.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)

```
build: limpo a cada estágio (release buildável por estágio — o par de perf mede o que diz
  medir; lição do P338 §4).
suíte (RUST_MIN_STACK=33554432): o número que o P350c deixou ± as asserções de escopo
  alteradas (só essas; justificadas; reportar quais e por quê).
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável, contra a harness de paridade — o oráculo, não os testes internos):
  - `[]` com regra `#show` dentro: a regra vaza para o irmão, igual ao vanilla.
  - `{}` com regra `#show` dentro: a regra confina, igual ao vanilla.
  - `#set` dentro de bloco: escopo inalterado (não regrediu).

morph == / morph_canon / PartialEq (P345): INTACTOS. A realização lê via `==` morfológico,
  não o altera. Confirmar.
recursão (caso 2) + flag de diagnóstico (P350c) + DEBT-59 (CLI): INTACTOS. Confirmar.
Trava ADR-0105 cl.3: presente para o caminho dinâmico de show antes de relaxar o compilador.

lente (critério 3 do plano):
  - edges(content → elements::*) = 66 INALTERADO (não é Marco G).
  - edges(elemento → elemento) = 0 (métrica-gate do F).
  - par --comparar antes/depois: registrar o delta de aresta na camada que a Fase A apontar.
perf (critério 4): antes = 0.6518 s ± 0.0057 (P330); reportar o depois (≥10 execuções).

L0 (critério 5): `f_fronteira_e1.md` auditado/atualizado e hash sincronizado ANTES do código.
```

---

## Válvula declarada

Se a Fase A medir que **transporte + caso 4** passa a faixa validada, fatiar:
- **P351** = só o transporte `StyledElem`-scoped (a fundação; aditivo, verificável por um
  teste que o exercita sem o escopo ainda ligado).
- **P352** = o caso 4 (escopo léxico) sobre o transporte.
Os casos 1 e 3 deslocam para P353/P354. Registrar a fatia escolhida e o número medido.

---

## O que NÃO fazer

- **Não tocar o caso 2 (recursão), o `morph_canon`/`==`, a flag de diagnóstico (P350c) nem
  o DEBT-59.** Tudo isso é estado fechado/registrado; mexer aqui é fora de escopo.
- **Não implementar os casos 1 (composição) e 3 (show-set) neste lote.** São P352+.
- **Não tocar a CLI nem o Marco G.** A CLI é DEBT-59; o corte `content→elements→0` é o
  Marco G, pós-F-6, com spec própria.
- **Não assumir os sítios nem a contagem de testes.** A Fase A os mede no L0/S*/recon; se a
  fonte contradisser o plano, a fonte vence e a divergência é reportada.
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Relatório (`typst-passo-351-relatorio.md` + resumo no chat)

As leituras da Fase A com `file:line` (L0, S*, recon, os pontos do vanilla); a fatia escolhida
(transporte+caso 4, ou a válvula); as asserções de escopo alteradas com a justificativa de
paridade; a prova de que caso 2 / flag / DEBT-59 / morph `==` ficaram intactos; os números da
lente (edges 66 inalterado, par `--comparar`) e da perf (antes/depois); produto fora de `lab/`
e docs com `git status` limpo por estágio; lint 0/0; o caveat de stack.

## Fora de escopo (confirmado)

Casos 1 e 3 da F-realização (P352+); F-5 (de-bake); F-6 (3 folhas); Marco G (desacoplamento
dos nativos); exposição da flag na CLI (DEBT-59); qualquer mudança no caso 2 / na flag de
diagnóstico / no `==` morfológico.

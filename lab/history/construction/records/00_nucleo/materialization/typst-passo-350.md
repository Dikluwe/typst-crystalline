# Passo 350 — a flag de erro completo: capacidade interna (CLI = débito)

> **O que faz.** Implementa a **capacidade interna** da flag de erro completo: quando o
> teto de recursão dispara **e** a flag está ligada, o erro de `#show` recursivo ganha um
> **hint separado** classificando **cíclico / divergente / converge-fundo** (por histórico
> de morfologias do caminho), com a **mensagem base intacta** (byte-idêntica ao vanilla). A
> **exposição na CLI fica como débito registrado** — não é feita aqui. **Primeira
> aplicação prática da ADR-0108**: a Fase A **mede** o canal de config L4→L1 da fonte e a
> medição **produz** a decisão de onde o campo mora — **não** se assume "L2". **NÃO**
> content-preserving (adiciona um caminho).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P350 (confirmar livre).
**Pré-condição**: P349 fechado (ADR-0108 EM VIGOR; a pilha P347c/d + P348 commitada,
árvore limpa). HEAD pós-P349; suíte **2726** (`typst-core --lib`) / **3245** (workspace),
lint 0/0, árvore limpa. Se não bater, parar.
**Tipo**: flag de erro completo (capacidade interna) — **NÃO content-preserving**:
adiciona o caminho da flag + a classificação. Regra do P340 + ADR-0107 + **ADR-0108**: a
decisão de camada é **produzida por medição** (Fase A), não assumida; a aceitação é
observável (a mensagem base intacta; o hint correto sob a flag) — a mecânica da
classificação é interna.
**Limites duros (ADR-0108 aplicada)**:
- **a camada do campo é decidida pela Fase A, não pré-assumida** — o P348 e o dono disseram
  "L2" por raciocínio (L1 não lê env); a regra 1 da ADR-0108 exige que a medição **produza**
  a classificação. Se a medição contradisser "L2", a forma muda.
- **a mensagem base não muda** — a flag escreve **só** no canal de hint (a base é
  comportamento observável, byte-idêntica ao vanilla, ADR-0033);
- **a classificação não corre no caminho quente** — só quando o teto dispara **e** a flag
  está ligada;
- **L1 não lê env** (pureza) — L1 recebe um booleano já resolvido; quem lê a config é a
  camada que a Fase A apontar;
- **a CLI NÃO é tocada** — a exposição é débito registrado, não trabalho deste lote.
**Objetivo**: a capacidade interna existir e ser **testável** (um teste liga a flag via o
campo de config e verifica o hint classificado; com a flag desligada, a mensagem base é
byte-idêntica ao vanilla). A porta de usuário (CLI) fica nomeada como débito.
**Fontes**: relatório P348 (o loop de revisitação em `apply_show_rules`/`apply_all`; o teto
backstop em `world_types.rs:273`; o `§3a.7-bis` onde a flag foi adiada; a classificação
desenhada — histórico de morfologias), P345 (`morph_canon` — a base da detecção de ciclo vs
divergência), **ADR-0108** (medir antes de decidir; aceitação no nível do observável),
`f_fronteira_e1.md §3a.7-bis`. A fonte do `lab/` **não** é necessária aqui (a flag é
melhoria do cristalino, não paridade de mecanismo) — exceto a mensagem base, já confirmada
byte-idêntica no P348.
**Commits** (isoláveis): "Passo 350 — caronas" · "Passo 350 — Fase A (canal de config
L4→L1)" · "Passo 350 — L0 (campo de config + classificação) se superfície" · "Passo 350 —
capacidade da flag + classificação" · "Passo 350 — testes" · "Passo 350 — débito CLI
registrado".

---

## Caronas (commit próprio)
- **C0 — base exata**: confirmar 2726 / 3245 no HEAD pós-P349; árvore limpa (a pilha foi
  commitada).
- **C1 — o `§3a.7-bis`**: confirmar que ele registra a flag como adiada (P348); este lote a
  realiza (capacidade) e move o registro de "adiada" para "capacidade interna feita; CLI =
  débito".

---

## Fase A — a medição que produz a decisão de camada (ADR-0108 regra 1)

A pergunta, da fonte, com `file:line`: **como uma config de execução chega hoje da
fronteira (L4/CLI/`SystemWorld`) até o ponto onde o erro de recursão é emitido (o loop em
`apply_show_rules`/`apply_all`, L1)?**

### M-canal — existe plumbing de config L4→L1?
Rastrear o caminho de uma opção de compilação existente (ex.: alguma flag/feature que já
desça da fronteira ao eval): há um struct de opções de compilação (em L2/L4) que é passado
ao eval e alcança o ponto do erro? Ou nada de config de usuário chega a L1 hoje?
- **canal existe** → a flag é um **campo a mais** nele; a Fase A diz **onde** o campo é
  definido/default (a camada real, medida) e por onde desce. Lote pequeno.
- **canal NÃO existe** → a flag teria de **abrir** o caminho (superfície nova, multi-camada)
  → **fronteira de decisão do dono** (a flag é melhoria sobre paridade; abrir um canal
  inteiro por ela pode não valer agora — parar na trava).

### M-classif — a classificação cabe no ponto do erro?
Confirmar que, no ponto onde o teto dispara (o loop), o **histórico de morfologias do
caminho** está disponível (ou é reconstruível barato ali) para distinguir:
- **cíclico** — uma morfologia do caminho **repete** (via `morph_canon`/`==` do P345);
- **divergente** — cresce sem repetir nem convergir;
- **converge-fundo** — estabilizaria, mas passou do teto (raro; recursão legítima profunda).
Se o histórico não existe no loop atual, medir o custo de mantê-lo **só quando a flag está
ligada** (não no caminho quente).

### M-msg — a mensagem base não muda
Confirmar (do P348) que a mensagem base + os dois hints do vanilla são emitidos em
`world_types.rs:273` e que adicionar um hint de classificação **não altera** a base nem os
dois hints existentes — só **acrescenta** um terceiro hint sob a flag.

**Saída da Fase A**: M-canal (a camada real do campo, medida — confirma ou corrige "L2"),
M-classif (o histórico disponível ou o custo de mantê-lo sob flag), M-msg (a base intacta).
Sem código antes disto.

---

## TRAVA — checkpoint
**Parar** se:
- **M-canal = canal não existe** → abrir o caminho é superfície nova multi-camada; o dono
  decide se a capacidade da flag vale isso agora ou se ela toda fica como débito (não só a
  CLI). (Fronteira de decisão — a flag é melhoria, não paridade.)
- a camada medida **contradiz** "L2" de um jeito que muda o desenho → reportar (a ADR-0108
  pediu a medição justamente para isto).
- o L0 ganhar superfície nova (o campo de config no contrato) → escrever, `--fix-hashes`,
  parar para selar.
**Seguir sem parar** se M-canal = canal existe, M-classif = histórico disponível/barato sob
flag, M-msg = base intacta.

---

## Fase B — execução (após a trava; só se o canal existe)

### Estágio L0 — o campo de config (se M criou superfície)
Atualizar o L0 (`§3a.7-bis` + o L0 da camada que M-canal apontou): o campo de "erro
completo" no canal de config, a classificação, e o registro do débito CLI. `--fix-hashes`;
parar para selar se superfície nova.

### Estágio Campo — o campo de config plumbed
Adicionar o campo (booleano "erro completo") na camada que M-canal mediu; plumbá-lo até o
ponto do erro em L1, que o **recebe resolvido** (L1 não lê env). Default: desligado (o
comportamento padrão = vanilla, sem o hint extra).

### Estágio Classif — a classificação (só no erro + flag)
No ponto onde o teto dispara: **se** a flag está ligada, reconstruir/consultar o histórico
de morfologias do caminho e classificar (cíclico / divergente / converge-fundo); emitir a
classificação como um **terceiro hint** no `SourceDiagnostic`. **Se** a flag está desligada
(default), nada disso corre — a mensagem base + os dois hints do vanilla, intactos.

### Estágio Teste
- Teste: flag **ligada** → o erro de `#show` recursivo (ciclo) tem o hint "cíclico"; um caso
  divergente tem "divergente"; (se construível) um converge-fundo tem o seu hint. A
  **mensagem base** e os dois hints originais permanecem.
- Teste: flag **desligada** (default) → a mensagem é **byte-idêntica** ao vanilla (base +
  dois hints), **sem** o terceiro. (Confirma que a flag não muda o padrão.)
- Teste: o caminho quente (recursão que **não** erra, ou nenhuma recursão) não paga a
  classificação — confirmar por construção (a classificação está atrás do `if teto && flag`).

### Estágio Débito — a CLI registrada
Registrar, no L0 e no DEBT/plano, o **débito**: "a flag de erro completo tem capacidade
interna (P350); falta a **exposição na CLI** (a porta de usuário em L4) — débito nomeado,
não esquecido". Não tocar a CLI.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)

```
build: limpo a cada estágio.
suíte (RUST_MIN_STACK=33554432): C0 (2726) ± N. Asserções alteradas só as relativas ao
  novo caminho (justificadas). Reportar novas.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável):
  - flag desligada (default): a mensagem de erro de recursão é BYTE-IDÊNTICA ao vanilla
    (base + 2 hints) — o comportamento padrão não muda. (A prova de que a flag é aditiva.)
  - flag ligada: o hint de classificação correto (cíclico/divergente/converge-fundo) aparece
    COMO TERCEIRO hint, sem alterar a base nem os 2 originais.
  - a classificação só corre no erro + flag (caminho quente intacto) — por construção.

camada do campo: a medida pela Fase A (M-canal), não "L2" assumido. Registrar a camada real
  e o caminho de plumbing.

== / morph_canon intactos (P345): a classificação USA o morph_canon para detectar ciclo,
  mas não muda o == da linguagem nem o PartialEq do Rust. Confirmar.

débito CLI: registrado no L0 + DEBT/plano — a exposição de usuário fica nomeada, não feita.

lente (--comparar antes/depois): a flag adiciona um campo de config + um ramo no erro —
  delta de aresta possível na camada do campo; content→elements 66; elem→elem 0. Registrar.

perf: caminho quente ~nulo (classificação atrás do if teto && flag). Reportar.
```

---

## O que NÃO fazer
- **Não assumir a camada do campo** — a Fase A a mede (ADR-0108 regra 1); "L2" é hipótese
  até a medição confirmar.
- **Não tocar a CLI** — a exposição é débito registrado, não trabalho deste lote.
- **Não mudar a mensagem base** — a flag só acrescenta um hint; a base é byte-idêntica ao
  vanilla.
- **Não rodar a classificação no caminho quente** — só no erro + flag.
- **Não fazer L1 ler env** — L1 recebe o booleano resolvido; a config é lida acima.
- **Não tocar o `==` morfológico nem o `PartialEq`** — a classificação usa o `morph_canon`
  para ler, não o altera.
- **Não medir aceitação pelo booleano** — a prova é observável (a mensagem com/sem o hint).
- **Não estimar com "~"**.

---

## Relatório (`typst-passo-350-relatorio.md`)
- Fase A: M-canal (a camada real do campo, medida — confirma ou corrige "L2", com
  `file:line` do plumbing), M-classif (o histórico disponível/custo sob flag), M-msg (base
  intacta).
- Fase B: o diff por estágio; o campo de config e o caminho até L1; a classificação no erro
  + flag.
- Testes: flag ligada (os hints de classificação), flag desligada (byte-idêntica ao
  vanilla), caminho quente intacto.
- Aceitação observável: a mensagem com/sem a flag; a camada medida do campo.
- Débito registrado: a exposição na CLI, nomeada no L0 + DEBT/plano.
- Verificação: suíte, lint, base-intacta, ==-intacto, lente, perf.
- **Mapa de filtro (campo):** o lugar lógico — "a flag de erro completo é melhoria sobre
  paridade (não paridade): a mensagem base permanece a do vanilla, e o cristalino acrescenta,
  sob pedido, a classificação que o vanilla não dá — primeira aplicação prática da ADR-0108
  (a camada foi medida, não assumida)" — com o rastro (P348 desenhou e adiou a flag; o dono
  pediu capacidade interna, CLI como débito; P350 mede o canal e implementa a capacidade).
- Item: `content→elements` aponta para o Marco G (P346).
```

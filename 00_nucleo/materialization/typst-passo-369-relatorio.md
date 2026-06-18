# Relatório P369 — público typst: completo pela fronteira da linguagem (VEREDITO B, medido)

> **Desfecho.** A medição decisiva (Fase A, read-only no oráculo vanilla) deu **Veredito B**:
> definir um elemento **com layout próprio** puramente em `.typ` **não existe no vanilla** — exige
> código (a macro Rust `#[elem]`). ∴ o público typst do cristalino — composição via `#let`,
> `#set`/`#show` sobre elementos registrados (P368), definição-com-layout em Rust — **casa
> exatamente** o que o `.typ` puro do vanilla permite. **O público typst está completo pela
> fronteira da linguagem.** P369 **fecha sem código**. **Com ele, o F está completo pelos
> princípios — exceto o F-5b** (DEBT-61). **Nenhum `.rs` funcional tocado.**

**HEAD**: pós-P368 (b2b2bc996). **Branch**: Tekt. Oráculo = vanilla 0.14.2 (`lab/`).

---

## Fase A — a pergunta decisiva, medida no vanilla (`file:line`)

> *No vanilla, um usuário define um elemento novo com **layout próprio** puramente em `.typ`
> (sem Rust/plugin)?*

**Medido [medido], em `lab/`:**
- **Elementos definem-se só pela macro Rust `#[elem]`** (`foundations/content/mod.rs:707`,
  `text/mod.rs:94`, …). **Não há primitivo `.typ`** de definição de elemento.
- **`#let` produz um `Closure`** (`foundations/func.rs:154` `FuncInner::Closure`), **não** um
  elemento: `func.element()` → `None` para closures; ops de elemento rejeitam closures
  (`where()` "can only be called on element functions", `func.rs:410`). ∴ `#set`/`#show` sobre
  uma função de usuário **não** é um caminho de elemento (o set rule casta a `Func` e chama
  `.set`, que é de elemento — `typst-eval/rules.rs:11-23`).
- **A única extensibilidade `.typ` com comportamento novo é WASM plugin** (`plugin("…wasm")`,
  `foundations/plugin.rs`) — **código** (WASM), e produz **computação** (funções que devolvem
  content/bytes), **não** elementos de **layout**.

**Mapeamento contra o cristalino:** o cristalino já tem em `.typ` puro — **composição via `#let`**
(closures no eval), **`#set`/`#show` sobre elementos registrados** (P368 + `#show` anterior), e
**definição-com-layout em Rust** (o registry + o `trait Element`, espelho do `#[elem]`). Isso é
**exatamente** o conjunto que o `.typ` puro do vanilla permite. Nada falta para casar o vanilla.

---

## Veredito B — sem código

A fronteira "**definir elemento com layout = código**" é da **própria linguagem** (vanilla **e**
cristalino), confirmando o P362-C. O público typst do cristalino está **completo**. P369 fecha
**read-only** (probes mentais sobre a fonte vanilla; nenhuma alteração de `.rs`).

**Registrado** no L0 `f_fronteira_e1.md §3a.11` (bloco "Público typst — a outra metade (P369,
VEREDITO B medido)") com os `file:line`.

---

## Estado / gates

```
Fase A: read-only (leitura do vanilla em lab/; nenhuma probe de código; árvore limpa).
build/suíte/lint: INALTERADOS — nenhum .rs funcional. Suíte 2742, lint 0/0.
ACEITAÇÃO (oráculo = vanilla): o público typst do cristalino (composição via #let + #set/#show
  sobre registrados) casa o .typ puro do vanilla; definição-com-layout = código nos dois.
INTACTOS: TextStyle/F-5b (DEBT-61); α/caso 2, morph ==/morph_canon, caso 4, flag P350c, Marco G;
  os 3 numbering; o #set de props de usuário (P368). (Nenhum tocado — sem código.)
lente: n/a (sem código). perf: n/a.
L0: f_fronteira_e1.md §3a.11 (fronteira + confirmação do F completo) + hash sincronizado.
commit: só L0 + relatório (padrão veredito-sem-código).
```

**Tocados:** `f_fronteira_e1.md` (§3a.11, Veredito B) + 9 backings (hash-sync); este relatório.
Nenhum `.rs` funcional.

---

## Estado do F após o P369

**O F está completo pelos princípios:**
- **Extensibilidade** — `#set` de props de usuário (P368) + a definição-com-layout = Rust
  (fronteira da linguagem medida, P369). Os **dois públicos** (Rust-define + compor/`#set`/`#show`
  em `.typ`) estão completos no que a linguagem permite.
- **Atomização (fonte única)** — os 3 numbering (P364/P365); F-6 confirmado sem de-bake (P367).
- **Exceto o F-5b** — o `TextStyle` do `Content::Text` permanece assado; lote **arquitetural
  dedicado** (modelo strong/emph/styled + α-fixpoint), **DEBT-61**.

Débitos **não-F** nomeados: **DEBT-59** (flag CLI), **DEBT-60** (contador), **Marco G**
(`content→elements→0`, decisão de modelo α/β, fora da fila F).

**Termino aqui — não emendo o seguinte (Trava 5).**

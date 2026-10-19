# Passo 350c — flag de erro completo: capacidade interna (C-com-origem intermédio)

> **O que faz.** Implementa a **capacidade interna** da flag de erro completo, na forma
> **C-com-origem intermédio** (confirmada pelo dono, P350b): `full_error` em `RunIntent` →
> caminho **interno** de L3 → `eval()` → `EvalContext` lê no ponto do erro de recursão. Sob
> a flag, o erro ganha um **terceiro hint** com a classificação em **dois rótulos sólidos**
> — **cíclico** (uma morfologia do caminho **repetiu** — fato medido pelo `==` do P345) e
> **não-convergente** (bateu no teto **sem** repetir). O terceiro rótulo ("converge-fundo")
> foi **cortado** por decisão do dono ("a mais correta"): distinguir divergente de
> converge-fundo exigiria adivinhar o futuro pós-corte — confiança falsa. A **mensagem base
> é byte-idêntica ao vanilla**; a assinatura **pública** de L3 **não muda**. CLI = débito.
> **NÃO** content-preserving.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P350c (confirmar livre).
**Pré-condição**: P350b fechado (forma C-com-origem intermédio confirmada). HEAD pós-P350b;
suíte **2726** (`typst-core --lib`) / **3245** (workspace), lint 0/0, árvore limpa. Se não
bater, parar.
**Tipo**: capacidade interna da flag — **NÃO content-preserving** (adiciona o caminho da
flag + a classificação). Regra do P340 + ADR-0107 + **ADR-0108**: a aceitação é
**observável** (mensagem base intacta; hint correto sob a flag); a classificação afirma só
o **medido** (regra 4 da ADR-0108 — daí dois rótulos, não três).
**Limites duros**:
- **a assinatura PÚBLICA de L3 (`compile_to_pdf_bytes`) NÃO muda** — o param entra pelo
  caminho **interno** (`eval_to_module_with_sink`/`eval_to_module`) com default `false`;
- **a mensagem base + os 2 hints do vanilla não mudam** — a classificação é um **3º hint**,
  só com a flag ligada;
- **o caminho quente não paga nada com a flag desligada** — o `Vec` do histórico é alocado
  **só** quando `full_error` está ligada (atrás do `if`);
- **L1 não lê env** — `EvalContext.full_error` recebe o booleano já resolvido;
- **a classificação tem DOIS rótulos** — cíclico (fato) e não-convergente (teto sem
  repetição); **não** inventar um terceiro que adivinhe o futuro pós-corte;
- **não tocar o `==` morfológico / `morph_canon` (P345)** — a detecção de ciclo o **usa**
  para ler, não o altera;
- **a CLI não é tocada** — parsing = débito.
**Objetivo**: a capacidade existir e ser **testável** via o param de `eval` (flag-on →
o 3º hint com o rótulo certo; flag-off → mensagem byte-idêntica ao vanilla), com o canal
construído de `RunIntent` até o `EvalContext` pelo interior de L3, sem expor na API pública.
**Fontes**: relatório P350b (a forma exata: `RunIntent` `cli.rs:103`; `EvalContext` ponto de
leitura `mod.rs:211`/`:454`; o erro em `apply_all` `rules.rs:196-201`; o loop sem histórico
`rules.rs:110-205`; a API L3 `pipeline.rs` sem options), P348 (o loop de revisitação, o teto
backstop, a mensagem base em `world_types.rs:273`), P345 (`morph_canon` — a base da detecção
de ciclo), **ADR-0108** (afirmar só o medido). A fonte do `lab/` **não** é necessária (a flag
é melhoria do cristalino, não paridade) — exceto a mensagem base, já confirmada idêntica.
**Commits** (isoláveis): "Passo 350c — caronas" · "Passo 350c — `full_error` em RunIntent +
canal interno L3→eval→EvalContext" · "Passo 350c — histórico + classificação (2 rótulos) sob
flag" · "Passo 350c — testes" · "Passo 350c — L0 + débito CLI registrado".

---

## Caronas (commit próprio)
- **C0 — base exata**: confirmar 2726 / 3245 no HEAD pós-P350b; árvore limpa.
- **C1 — o `§3a.7-bis`**: mover o registro da flag de "adiada" (P348) → "capacidade interna
  feita (P350c); parsing CLI = débito".

---

## Fase de código — por estágio isolável

### Estágio Canal — `full_error` da origem ao ponto de leitura (sem tocar a API pública)
1. **Origem (L2):** `full_error: bool` em `RunIntent` (`cli.rs:103`), ao lado de `colored`,
   default `false`. (O parsing `--full-error`→`RunIntent` é **débito**; aqui o campo só
   passa a existir, default `false`.)
2. **Canal interno (L3):** o caminho **interno** (`eval_to_module_with_sink`/
   `eval_to_module`) passa o booleano a `eval()`; a **assinatura pública**
   `compile_to_pdf_bytes` **não muda** (recebe o default `false` internamente). `eval()`
   (L1) ganha o param `full_error`.
3. **Leitura (L1):** `EvalContext.full_error` (campo novo, default `false`), preenchido pelo
   param de `eval`, lido no ponto do erro (`apply_all`).
Confirmar: a API pública de L3 tem a mesma assinatura de antes (grep/diff da assinatura).

### Estágio Histórico+Classificação — sob a flag, dois rótulos
1. No loop de revisitação (`rules.rs:110-205`): **se** `ctx.full_error` está ligada, manter
   um `Vec` das morfologias do caminho (`morph_canon` de cada passo). **Se desligada (default),
   o `Vec` não é alocado** — o caminho quente não muda.
2. Detecção de **ciclo** (a única classificação sólida): a cada passo, se `morph_canon(work)`
   **já está** no histórico, é **cíclico** → erro classificado. (Usa o `==` do P345 para
   comparar; não o altera.) O ciclo pode ser detectado **antes** do teto (mais cedo que o
   vanilla), mas a **mensagem base é a mesma**.
3. Ao emitir o erro do teto/ciclo, **se** a flag está ligada, acrescentar o **3º hint**:
   - **cíclico** — "a regra de exibição entrou em ciclo (uma forma se repete)"; (opcional, se
     barato) a profundidade onde o ciclo fechou.
   - **não-convergente** — "a recursão passou do limite sem estabilizar nem repetir; verifique
     se a regra termina, ou se é recursão legítima profunda" (cobre divergente **e** o que
     estabilizaria mais fundo — **sem** fingir distinguir, ADR-0108).
   A **mensagem base** ("maximum show rule depth exceeded") + os **2 hints** do vanilla
   permanecem byte-idênticos; o 3º hint só aparece com a flag.
4. **Sem** a flag: nada disso corre; a mensagem é a do vanilla (base + 2 hints), byte-idêntica.

### Estágio Teste
- **flag-on, ciclo:** `==[a]→[= b]; else [= a]` · `= a` → erro com 3º hint **"cíclico"**;
  base + 2 hints intactos.
- **flag-on, não-convergente:** uma regra que cresce sem repetir → erro com 3º hint
  **"não-convergente"**.
- **flag-off (default):** os mesmos casos → mensagem **byte-idêntica ao vanilla** (base + 2
  hints, **sem** 3º). (A prova de que a flag é aditiva.)
- **caminho quente:** recursão convergente (`m1`) ou sem recursão → o `Vec` **não** é
  alocado com a flag off (confirmar por construção — o `if ctx.full_error`).
- **assinatura pública L3 intacta:** um teste/asserção de que `compile_to_pdf_bytes` tem a
  mesma assinatura (ou: nenhum chamador público mudou).

### Estágio L0 + débito
- L0 (`§3a.7-bis` + o L0 da camada do canal): a capacidade, os **dois rótulos** (e por que
  não três — afirmar só o medido), o canal interno, a flag lida no `EvalContext`.
- **Débito nomeado** (no L0 + DEBT/plano): (1) o **parsing da CLI** (`--full-error` em `Args`
  → `RunIntent.full_error`); (2) o **fio `RunIntent`→ponto interno de L3** (hoje o default
  `false` entra no interior; ligar o `RunIntent` real a ele é débito junto com a CLI). A
  assinatura pública de L3 **fica intacta** — não é débito, é decisão (não expor).

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)
```
build: limpo a cada estágio.
suíte (RUST_MIN_STACK=33554432): C0 (2726) ± N. Asserções alteradas só as do novo caminho
  (justificadas). Reportar novas.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável):
  - flag-off (default): mensagem de erro de recursão BYTE-IDÊNTICA ao vanilla (base + 2
    hints). O padrão não muda.
  - flag-on: 3º hint com o rótulo certo — "cíclico" (morfologia repetiu) ou "não-convergente"
    (teto sem repetição); base + 2 hints intactos.
  - DOIS rótulos, não três: nenhum rótulo afirma o futuro pós-corte (sem "converge-fundo").
  - caminho quente: Vec do histórico NÃO alocado com flag off (por construção).

assinatura pública L3 intacta: compile_to_pdf_bytes com a mesma assinatura — confirmar.
== / morph_canon intactos (P345): a detecção de ciclo os USA, não os altera.
débito: parsing CLI + fio RunIntent→L3-interno registrados no L0 + DEBT/plano.
lente (--comparar antes/depois): a flag adiciona um campo em RunIntent/EvalContext + um ramo
  no erro; content→elements 66; elem→elem 0. Registrar.
perf: caminho quente ~nulo (Vec atrás do if full_error); a detecção de ciclo só sob flag.
  Reportar.
```

---

## O que NÃO fazer
- **Não mudar a assinatura pública de L3** (`compile_to_pdf_bytes`) — o param entra pelo
  interior; a pública fica.
- **Não inventar um terceiro rótulo** — dois sólidos (cíclico = fato; não-convergente = teto
  sem repetição); o que adivinharia o futuro pós-corte não entra (ADR-0108: afirmar só o
  medido).
- **Não mudar a mensagem base** — a classificação é o 3º hint, só sob a flag.
- **Não alocar o histórico com a flag desligada** — atrás do `if ctx.full_error`.
- **Não fazer L1 ler env** — recebe o booleano resolvido.
- **Não tocar o `==`/`morph_canon`** — a detecção de ciclo os usa para ler.
- **Não tocar a CLI** — parsing = débito.
- **Não medir aceitação pelo booleano** — a prova é a mensagem com/sem o hint.
- **Não estimar com "~"**.

---

## Relatório (`typst-passo-350c-relatorio.md`)
- O diff por estágio: o canal (`RunIntent`→interior L3→`eval`→`EvalContext`), com a prova de
  que a assinatura pública de L3 não mudou.
- A classificação: os **dois** rótulos, e o registro de por que não três (afirmar só o
  medido — o corte do "converge-fundo" por decisão do dono).
- Testes: flag-on (cíclico, não-convergente), flag-off (byte-idêntico ao vanilla), caminho
  quente (Vec não alocado), assinatura pública intacta.
- Aceitação observável: a mensagem com/sem a flag.
- Débito: parsing CLI + fio `RunIntent`→L3-interno, no L0 + DEBT/plano.
- Verificação: suíte, lint, base-intacta, assinatura-pública-intacta, ==-intacto, lente, perf.
- **Mapa de filtro (campo):** o lugar lógico — "a flag de erro completo afirma só o que mede:
  cíclico é fato (a morfologia repetiu), não-convergente é fato (o teto sem repetição); o
  rótulo que adivinharia o futuro pós-corte foi cortado — a ADR-0108 (afirmar só o medido)
  aplicada ao próprio diagnóstico; e o canal foi aberto até onde a origem real exige (L2/
  interior de L3), não até a assinatura pública (sem demanda)" — com o rastro (P348 adiou a
  flag; P350 mediu sem canal L4→L1; P350b mediu a origem `RunIntent` + o ponto de leitura
  `EvalContext`; o dono confirmou o intermédio e cortou o 3º rótulo; P350c implementa).
- Item: `content→elements` aponta para o Marco G (P346).
```

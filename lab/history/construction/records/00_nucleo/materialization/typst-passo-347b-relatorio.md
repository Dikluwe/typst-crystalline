# Passo 347b — relatório: intenção da terminação de recursão de `#show` — **VEREDITO: GEROU**

> **Veredito (com a inferência marcada): GEROU.** A distinção `it`-guardado-termina /
> fresco-recursa-até-o-teto é **consequência de um mecanismo anti-loop por-instância**, não
> uma promessa declarada da linguagem. Evidência: o guard é doc'd como "*disable* a show
> rule recipe" (bit de `lifecycle` por-instância); a mensagem/hint são um **limite
> protetor** ("*maybe* a show rule matches its own output"); os testes do vanilla afirmam
> **só o erro/limite** (não a distinção como desejada); e — decisivo — **I4** mostra que a
> terminação depende de **identidade de objeto exata** (`it => it` termina; reconstruir um
> heading idêntico, `it => heading(level: it.level, it.body)`, **erra**). Nenhuma semântica
> de linguagem prometeria "re-emitir um elemento idêntico é infinito mas devolver o mesmo
> objeto não é". **Implica** (sem escolher): **α** vira defensável/preferível (ADR-0107: a
> mecânica do vanilla não é a linguagem); a confirmação definitiva da intenção é **lacuna
> I6** (doc/PR fora de alcance). Content-preserving: zero `.rs`, lint 0/0, suíte 2723/3242
> intacta; `lab` só lido/executado.

## Pré-condição
P347 Fase A parada na TRAVA (M3 = multi-passe). HEAD `04e02f93a` (pós-P346), P345
commitado, suíte 2723/3242, lint 0/0, árvore limpa (o relatório P347 é untracked, não
modificação rastreada). Bate.

## A distinção investigada
O P347 mediu o **comportamento**: o vanilla termina por **identidade de instância**
(devolver `it` guardado → termina; devolver `[= Z]` fresco → recursa → teto → erro). Este
passo mede a **intenção**: a linguagem **quis** isso (promessa) ou a implementação
**gerou** (mecânica sem promessa)?

## I1 — o guard e o teto na fonte (intenção no código)
- **Guard por-instância** (`foundations/content/mod.rs:147-156`):
  - `is_guarded`: *"Check whether a show rule recipe is **disabled**."*
  - `guarded`: *"**Disable** a show rule recipe."*
  - Armazenado em `self.0.meta_mut().lifecycle.insert(index.0)` — um **bitset de
    `lifecycle` por-instância** do nó de `Content`, **agrupado** com `is_prepared`/
    `mark_prepared` (bit 0). É a maquinaria de **ciclo-de-vida da realização**, não uma
    propriedade semântica do conteúdo.
- **Realização** (`typst-realize/src/lib.rs:1-5`): *"Realization is the process of
  **recursively applying** styling and, in particular, show rules…"* — framing de
  **processo/mecanismo recursivo**; o guard é o dispositivo anti-loop dentro dele.
- **Teto** (`typst-library/src/engine.rs:335,346-356`): `MAX_SHOW_RULE_DEPTH = 64`;
  `check_show_depth` agrupado com `check_layout_depth`/`check_html_depth`/`check_call_depth`
  — uma família de **limites de profundidade protetores** (`MAX_*_DEPTH`), genérica, não
  específica de show.
- **O guard segue a INSTÂNCIA**, não o conteúdo (o bit vive no `meta` do `Arc`). → reforça
  que `it`-vs-fresco é estrutural do mecanismo de identidade.
- **Indício: MECÂNICA.** Os nomes/comentários descrevem "disable recipe" + "depth limit",
  proteção — **nenhum** comentário declara a distinção `it`-vs-fresco como semântica.

## I2 — a mensagem de erro (aviso ou semântica)
`engine.rs:349-353`:
```
bail!("maximum show rule depth exceeded";
      hint: "maybe a show rule matches its own output";
      hint: "maybe there are too deeply nested elements");
```
O texto trata como **erro do usuário a evitar**: "maximum … exceeded" (um limite
estourado) + hints especulativos ("**maybe** you…"). É um **aviso de proteção** ("talvez
você fez algo por engano"), **não** uma semântica declarada ("para parar a recursão,
devolva `it`"). → **Indício: MECÂNICA.**

## I3 — os testes do vanilla (intenção revelada por teste)
`tests/suite/scripting/recursion.typ:44-64` — os testes de recursão de `#show` afirmam
**só o erro/limite**:
- `#show math.equation: $x$` → `Error: maximum show rule depth exceeded` (+ os 2 hints).
- `#show heading: it => heading[it]` → mesmo erro.
- `#layout(_ => include "recursion.typ")` → mesmo erro.

**Nenhum** teste afirma "devolver `it` termina **e** devolver fresco recursa" como
comportamento **desejado**. Os testes **protegem o limite** (que o auto-matching erra),
não a distinção. → **Indício: MECÂNICA** (protege-se o limite, não a distinção).

## I4 — casos de borda no binário (o critério real de terminação) — DECISIVO
Vanilla (`typst 0.14.2`, binário do P341), `= a`:

| `.typ` | saída | revela |
|---|---|---|
| `#show heading: it => it` | **a** (termina) | a instância exata `it` carrega o guard |
| `#show heading: it => heading(it.body)` | **erro** `maximum show rule depth exceeded` | reconstruir = fresco = sem guard → recursa |
| `#show heading: it => heading(level: it.level, it.body)` | **erro** (idem) | mesmo morfologicamente idêntico, fresco → recursa |

O critério é **identidade de objeto**, não forma: reconstruir um heading **morfologicamente
idêntico** ainda **erra**; só o literal `it` termina. Nenhuma semântica de linguagem
prometeria que "re-emitir um elemento idêntico (`heading(level: it.level, it.body)`) é
recursão infinita, mas devolver o mesmo objeto (`it`) não é". Isto é o **guard-por-instância
vazando** como se fosse regra de terminação. → **Indício forte: GEROU.**

## I5 — situar contra outros sistemas
Nenhuma referência cross-system na fonte (o `lifecycle`/guard é conceito **interno** da
realização; sem design-notes apontando para outros sistemas). Sem evidência de que
`it`-vs-fresco seja escolha de domínio conhecida.

## I6 — a lacuna explícita (o que exige doc/histórico fora de alcance)
- **`lab/typst-original` não tem histórico git próprio** (`git -C lab log` devolve o log do
  repo cristalino, não o do vanilla) → o **PR/commit que introduziu o guard/teto** e a
  **mensagem do autor** (a intenção mais direta) **não são acessíveis** aqui.
- A **doc oficial do Typst** sobre show rules + recursão (promete terminação? descreve
  `it`-vs-fresco?) **não é alcançável** neste ambiente (sem web confirmada).
- **Perguntas exatas para buscar fora** (dono/agente externo):
  1. A doc oficial declara como parar uma show rule recursiva (devolver `it`?) ou só avisa
     do teto?
  2. O PR que adicionou `check_show_depth`/`guarded` descreve a distinção `it`-vs-fresco
     como design, ou como backstop anti-loop?

## VEREDITO — **GEROU** (inferência marcada)
A distinção `it`-vs-fresco é **consequência do mecanismo anti-loop por-instância**
(guard de `lifecycle` + teto de profundidade), **não** uma promessa declarada da linguagem.

**Base (evidência da fonte acessível):** I1 (guard = "disable recipe" por-instância; teto =
limite protetor genérico), I2 (mensagem/hint = aviso "maybe você se auto-casou"), I3
(testes afirmam só o erro/limite), I4 (terminação por identidade de objeto exata —
reconstruir idêntico erra).

**Marca de inferência (exigida pelo passo):** GEROU é **inferido** da (a) moldura mecânica
do guard/teto/hint, (b) **ausência** de qualquer semântica declarada de `it`-vs-fresco em
comentários/testes, e (c) a dependência de identidade pura medida em I4. É inferência da
**fonte acessível**; a intenção **definitiva** do autor (doc/PR) é a **lacuna I6**, não
confirmada aqui. Nenhum indício de QUIS foi encontrado; não é AMBÍGUO no sentido de
"nada aponta" — a fonte aponta consistentemente para mecânica, só não há a confirmação
externa do autor.

**Implicação (sem escolher γ/β/α):** sendo GEROU, reproduzir o vanilla **byte a byte** é
copiar um **acidente de identidade de instância**. Pela ADR-0107 (a mecânica do vanilla
não é a linguagem), **α** — uma terminação mais limpa (ponto-fixo/morfologia, que **vê** a
convergência que o vanilla não vê) — torna-se **defensável e possivelmente preferível**,
com a divergência (o_inf → "Z" gracioso vs erro do vanilla) registrada como **consciente e
justificada**: o vanilla erra ali só porque o seu anti-loop não enxerga o ponto-fixo. A
escolha **γ vs β vs α** continua sendo do **dono**, agora **com o veredito na mão** — e o
veredito **abre** α (que o P347 tinha como "divergência"; agora é "fidelidade à intenção").

**Parei. Não escolhi γ/β/α. Nenhum código.**

## Verificação (gates)
```
content-preserving: zero código/teste/.rs/.toml. Suíte 2723/3242 intacta (não re-rodada).
  lab só lido/executado.
lint: crystalline-lint . = 0/0.
evidência: I1-I4 com file:line (fonte) e .typ + saída do vanilla (binário). I4 decisivo.
  Inferência do veredito marcada como tal. Zero "~".
lacuna: I6 lista o que exige doc/PR online (lab sem git próprio; sem web).
fronteira: o passo NÃO escolheu γ/β/α — entregou o veredito e parou.
```

## Mapa de filtro (campo)
**Lugar lógico:** antes de reproduzir a terminação, o projeto separou a **intenção da
linguagem** do **acidente da implementação** — é a **ADR-0107 aplicada à terminação** (a
paridade é com o que a linguagem **promete**, não com o que a implementação **calhou de
fazer**). **Rastro:** P347 Fase A mediu o comportamento (por instância); **P347b mede a
intenção e conclui GEROU**; a escolha γ/β/α vem depois, com o veredito (que **abre α** como
fidelidade à intenção, não mais como mera divergência).

## Item carregado
`content→elements` aponta para o **Marco G** (P346) — não volta como órfão.
```

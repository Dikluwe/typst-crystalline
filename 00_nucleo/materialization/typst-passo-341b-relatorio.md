# Passo 341b — relatório: recursão limitada + semântica do guard do vanilla (medição)

> **Veredito:** o guard do cristalino **trunca no nível 1** (M1 = "b"). Cruzado
> com a semântica do vanilla (guard **por-conteúdo** + teto-64 intencional),
> igualar o vanilla é **CARO** (mini-fixpoint estreito) e a arbitragem proposta
> pelo dono é **INFIEL como proposta** (o árbitro teria de simular a recursão do
> vanilla). **E** surgiu um segundo achado que **agrava**: a igualdade de conteúdo
> `it.body == [a]` diverge (crist NOMATCH, vanilla MATCH). Spike content-preserving:
> zero produto tocado, suíte 2719/3238, lint 0/0.

## Setup (reusado do P341)

Vanilla `typst 0.14.2` (binário sobrevivente em `lab/.../target/debug/typst`;
`lab/` **não** foi editado nesta sessão — só executado). Cristalino
`target/debug/typst`. Comparação por `pdftotext` da saída observável.

## Medição 1 — recursão limitada

### `.typ` (a→b→c, 3 aplicações, para no `else`)
```typ
#show heading: it => {
  if it.body == [a] { [= b] } else if it.body == [b] { [= c] } else { it }
}
= a
```
| | vanilla | cristalino |
|---|---|---|
| `m1.typ` (a→b→c) | **c** | **a** |
| `m1b.typ` (a→stop, 2 passos) | **stop** | **a** |

**O crist deu "a", não "b" nem "c".** Causa medida: a condição `it.body == [a]`
**falha** no cristalino → cai no `else { it }` → identidade → "a". **Não** é o
guard; é um **segundo divergência** (igualdade de conteúdo — ver §Achado 2).

### Confirmação limpa do guard (sem a condição confundente)

`one.typ` — marcador fixo incondicional `#show heading: it => [= Z]` · `= a`:
| | vanilla | cristalino |
|---|---|---|
| `one.typ` | **ERRO** `maximum show rule depth exceeded` | **Z** |

`stop.typ` — `if it.body==[STOP] { it } else { [= STOP] }` · `= a`:
| | vanilla | cristalino |
|---|---|---|
| `stop.typ` | **STOP** (2 aplicações, para via content-eq) | **STOP** (1 aplicação — **sem erro** ⇒ truncou; se recursasse, a content-eq quebrada → loop → erro) |

**Veredito M1 = "b" (trunca no nível 1).** O cristalino aplica uma regra recursiva
**exatamente uma vez** por nó original (`one.typ`: "Z", uma aplicação, vs vanilla
que recursa até o teto e erra). Confirmado também pela fonte (§M2). Para a
recursão limitada a→b→c, o crist aplicaria **uma vez** (a→b) e pararia ("b"), o
vanilla roda todos os passos ("c") — mas a content-eq quebrada mascara isto como
"a".

## Medição 2 — semântica do guard/teto do vanilla (da fonte, `file:line`)

1. **Teto = 64, intencional.** `MAX_SHOW_RULE_DEPTH = 64`
   (`crates/typst-library/src/engine.rs:335`); `check_show_depth`
   (`:347-353`) erra com `"maximum show rule depth exceeded"` + **hint
   `"maybe a show rule matches its own output"`**. O hint **prevê** a auto-recursão
   — é desenho, não débito; o vanilla **espera** que regras casem o próprio output
   e bound­eia em 64.

2. **Guard por-CONTEÚDO (não por-regra).** `crates/typst-realize/src/lib.rs`:
   - `output.into_owned().guarded(guard)` (`:366`) — após aplicar um recipe, o
     **output** é marcado `.guarded(index)`; o guard viaja **com o elemento**.
   - `let index = RecipeIndex(*depth - r); if elem.is_guarded(index)` (`:471-473`)
     — um recipe é saltado só se **aquele elemento** já está guardado para ele.
   - **Conteúdo NOVO** produzido por um recipe nasce **fresco** (não herda o guard)
     → pode ser re-processado → recursão até o teto-64 ou terminação natural.
   - Há ainda `Style::Revocation` (`:1232`) — revogação de show rule como feature.

   **Contraste com o cristalino:** o guard é por-`RuleId` (`rules.rs:104`:
   `if engine.active_guards.contains(&rule.id) { continue }`; push/pop em
   `:141-143`). Enquanto a regra roda, **qualquer** conteúdo que ela produz é
   saltado por ela → **trunca no nível 1**. Esta é a raiz medida da divergência:
   **por-conteúdo (vanilla) vs por-regra (cristalino).**

## Classificação (o entregável)

**M1 = "b" (trunca no nível 1)** → segundo a grade do P341b:

- **Igualar o vanilla = CARO.** A divergência atinge recursão **legítima**
  (limitada), não só a infinita. Exige replicar o guard **por-conteúdo** do vanilla
  (marcar o output com recipe-index, não bloquear a regra globalmente) **+**
  re-realizar o output até fixpoint/teto — um **mini-fixpoint estreito** (só o caso
  recursivo; a cascata A→B já é fiel, p3b do P341).

- **Arbitragem do dono = INFIEL como proposta.** A proposta (motor gracioso +
  segunda função que detecta a borda e corta para imitar o vanilla) **não** é fina:
  o guard do crist corta no nível 1 ("b"/identidade) num documento que o vanilla
  **compila** para "c". Para "detectar a borda do vanilla", o árbitro teria de
  **simular a recursão por-conteúdo do vanilla** — deixa de arbitrar sobre o crist
  e vira **reimplementação do motor**. Não é fino nem fiel.

- **Nota S5b (morto-alimentado).** Se o árbitro sempre corta para imitar o vanilla,
  o caminho gracioso do motor **nunca** é observável em modo fiel = comportamento
  morto-alimentado. "Motor gracioso + árbitro" só se paga se um requisito futuro
  expuser o modo gracioso; senão, **igualar o vanilla direto é menos superfície**
  pelo mesmo resultado observável.

### Achado 2 (agrava) — igualdade de conteúdo `it.body == [a]`

`#show heading: it => if it.body == [a] { [MATCH] } else { [NOMATCH] }` · `= a`:
**vanilla MATCH, cristalino NOMATCH.** (`[a] == [a]` isolado dá "EQ" no crist; o
problema é `it.body` (corpo do heading) **vs** `[a]` (bloco fresco) não serem
estruturalmente iguais no crist.) Consequência: **regras condicionais sobre o
corpo** — a forma natural de escrever recursão auto-limitada — **divergem
independentemente** do guard. Logo, mesmo um mini-fixpoint do guard **não** faria
`m1` bater sem **também** consertar a content-eq. A lacuna de recursão é
**composta** pela lacuna do modelo de conteúdo.

## Recomendação ao dono — o conserto mais estreito que o resultado sustenta

O resultado **não** sustenta a arbitragem fina. Duas saídas honestas:

- **(A) Aceitar a truncação como divergência intencional documentada.** O crist
  corta a auto-recursão no nível 1 (nunca erra por profundidade; mais conservador).
  Registrar na §3a.7-bis como **escolha** (guard por-regra), não débito. Custo:
  zero código; uma nota. Risco: documentos que dependem de recursão de show rule
  divergem (raros; e a content-eq já os quebraria de outra forma).
- **(B) Lote dedicado de re-realização estreita** (se a fidelidade de recursão for
  requisito): guard **por-conteúdo** (recipe-index no output) + loop até teto-64
  **e** conserto da content-eq de `it.body`. Medido contra o vanilla compilado,
  testes do balde (ii) evoluídos um a um. **Não** é a arbitragem fina; é o
  mini-fixpoint + content-eq. Maior superfície.

**Não recomendado:** o "motor gracioso + árbitro que imita o vanilla" — medido como
infiel (exigiria simular o vanilla) e morto-alimentado (S5b).

## Verificação (gates)

```
content-preserving: zero código de produção, zero teste alterado, zero ficheiro
  de produto tocado (lab não editado nesta sessão). Suíte 2719 / 3238.
lint: crystalline-lint . = 0 violations, 0 warnings.
lente: não medida (nada de produto mudou) — inalterada vs P340 (219|676|[90,4]|66|0).
medição reprodutível: binários do P341; comandos acima; vanilla 0.14.2.
```

Nenhum commit de código; entregável = veredito + classificação, para decisão.

# Passo 341b — Medição da recursão limitada + leitura do `lab/` (decide barato-vs-caro e a viabilidade da arbitragem)

> **Adendo ao P341.** O spike P341 mediu recursão **infinita** (p4/p7) e deixou
> sem medir a recursão **limitada** (uma regra que dispara N vezes e para). Esse
> caso decide se igualar o vanilla é barato ou caro, **e** se a arbitragem
> proposta pelo dono (motor gracioso + segunda função que detecta a borda e corta
> para imitar o vanilla) é **fina e fiel** ou **grossa e infiel**. Medição
> content-preserving: zero produto tocado.

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P341b (confirmar livre).
**Pré-condição**: relatório P341 entregue; árvore do produto limpa; `lab/`
revertido ao estado de quarentena. Se não, parar.
**Tipo**: **medição** — content-preserving estrito: **zero código de produção,
zero teste alterado, zero ficheiro de produto tocado** (`lab/` revertido ao fim,
como no P341). Termina com **veredito + classificação**, para decisão do dono.
**Não** construir conserto, **não** decidir a arbitragem.
**Objetivo**: medir se o guard do cristalino **trunca** a recursão limitada de
show rule no nível 1, ou se **recursa** como o vanilla até o teto; e confirmar da
fonte (`lab/`) se o teto e o guard do vanilla são intencionais e qual a sua
semântica. O resultado decide (a) barato-vs-caro de igualar o vanilla e (b) se a
arbitragem do dono é viável como proposta.
**Fontes**: `lab/typst-original/` (compilar + ler), relatório P341 (setup
reprodutível, o mapa p1–p8), o guard `RuleId` do cristalino (`rules.rs:97-167`,
`world_types.rs:249`, `lib.rs:401-402`).

---

## Setup (reusar o do P341, registrado)

`cp Cargo.toml.original Cargo.toml` (members `crates/*`), o fix compile-only de
`E0282` em `crates/typst-library/src/foundations/str.rs:711`, `cargo build -p
typst-cli` → vanilla `typst 0.14.2`. **Reverter ambas as mudanças de `lab/` ao
fim** (árvore do produto limpa). Comparação: `typst compile X.typ` (vanilla) vs
`typst X.typ` (cristalino) → `pdftotext` → texto normalizado. Saída **observável**.

---

## Medição 1 — recursão limitada (o discriminador central)

Uma regra que dispara mais de uma vez e **termina sozinha** antes de qualquer
teto, transformando a cada passo (para a saída distinguir o número de aplicações):

```typ
#show heading: it => {
  if it.body == [a] { [= b] }
  else if it.body == [b] { [= c] }
  else { it }
}
= a
```

- **Vanilla (esperado):** heading "c" — a regra aplica a→b→c (3 aplicações) e
  para no ramo `else`. **Confirmar compilando.**
- **Cristalino:** rodar e observar a saída.
- **Veredito:**
  - saída = **"c"** → o cristalino **recursa como o vanilla**; diverge **só** na
    recursão infinita (p4/p7).
  - saída = **"b"** (uma aplicação) → o guard `RuleId` **trunca no nível 1**;
    diverge também em recursão **legítima** (limitada).

**Caso-âncora de 2 passos** (bracketar): mesma forma, mas `a → stop` (2
aplicações). Vanilla esperado "stop" via 2 aplicações; observar o cristalino.

Registrar as `.typ`, os comandos, e as saídas dos dois lados lado a lado.

---

## Medição 2 — leitura do `lab/` (intencionalidade e semântica do guard, da fonte)

Sem compilar; leitura com `file:line`:

1. **O teto** `MAX_SHOW_RULE_DEPTH` no vanilla (o valor, e qualquer comentário de
   intenção em volta). Confirma "desenho, não débito".
2. **O guard/revogação do vanilla** na realização: o recipe é marcado após
   aplicar? O guard é **por-conteúdo** (o output é conteúdo novo, não herda o
   guard → a regra reaplica ao output, recursando até o teto) ou **por-regra**
   (a regra não reaplica)? `file:line`. Isto explica **por que** o vanilla
   recursa até 64 e o cristalino corta antes — e confirma ou refuta a inferência
   de que o guard do cristalino é por-regra/nível-1.

(Nota externa, da web, a confirmar pela fonte: o erro de profundidade do vanilla
é mantido ativamente — há PR melhorando a dica do erro — e o typst trata o
aninhamento de show rules como área com feature planeada, "Revocation". Ou seja:
intencional, mas em evolução. A leitura do `lab/` confirma o lado da fonte.)

---

## Classificação (o entregável — para o checkpoint do dono)

Cruzar Medição 1 com a semântica da Medição 2:

- **Se M1 = "c"** (recursa como o vanilla): a divergência é **só** a recursão
  infinita.
  - Igualar o vanilla = **BARATO**: emitir o erro no ponto onde o guard já corta.
  - Arbitragem do dono = **FINA E FIEL**: o motor recursa como o vanilla, a borda
    do infinito é a mesma, e a segunda função erra ali — observável igual ao
    vanilla.

- **Se M1 = "b"** (trunca no nível 1): a divergência atinge recursão **legítima**.
  - Igualar o vanilla = **CARO**: exige replicar a recursão-até-teto do vanilla
    (semântica de guard por-conteúdo) — um mini-fixpoint **estreito** (só o caso
    recursivo; a composição A→B já é fiel, p3b).
  - Arbitragem do dono = **INFIEL como proposta**: o árbitro cortando no ponto do
    guard erraria num documento que o vanilla **compila**; "detectar a borda do
    vanilla" exigiria **simular a recursão do vanilla**, não arbitrar sobre o
    cristalino — o árbitro deixa de ser fino e vira reimplementação do motor.

**Nota sobre a arbitragem (registrar em qualquer dos casos):** se o árbitro
sempre corta para imitar o vanilla, o caminho gracioso do motor nunca é
observável em modo fiel = comportamento morto-alimentado (lição S5b). Manter
"motor gracioso + árbitro" só se paga se um requisito futuro expuser o modo
gracioso; senão, igualar o vanilla direto é menos superfície pelo mesmo resultado
observável.

---

## Verificação (gates)

```
content-preserving: zero código de produção, zero teste alterado, zero ficheiro
  de produto tocado (lab revertido). Suíte inalterada: 2719 / 3238.
lint: crystalline-lint . = 0 violations, 0 warnings.
lente: não medida (nada de produto mudou) — declarada inalterada.
medição reprodutível: comandos e fixes de build registrados; vanilla 0.14.2.
```

## O que NÃO fazer

- **Não construir conserto** (nem ordenação, nem erro de recursão, nem fixpoint).
- **Não decidir a arbitragem** — o veredito + a classificação vão ao dono.
- **Não alterar nenhum teste nem ficheiro de produto.**
- **Não estimar com "~"** — as saídas são medidas.

## Relatório (`typst-passo-341b-relatorio.md`)

- Medição 1: as `.typ`, os comandos, as saídas vanilla vs cristalino, o veredito
  ("c" recursa / "b" trunca) e o caso-âncora de 2 passos.
- Medição 2: o teto e a semântica do guard do vanilla, com `file:line`.
- Classificação: barato-vs-caro **e** arbitragem fina-fiel-vs-grossa-infiel, com a
  nota S5b.
- Recomendação ao dono: o conserto mais estreito que o resultado sustenta.

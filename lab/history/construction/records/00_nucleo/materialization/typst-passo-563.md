---
# P563 — Medir o custo real da reordenação bidi + quantificar a distorção em texto misto

> **Passo:** 563
> **Data:** 2026-07-04
> **Foco:** P562 fechou as três camadas de RTL sem correr o benchmark que o próprio passo pedia, e descreveu a distorção de espaçamento em texto misto como "pequena" sem número. Este passo mede as duas coisas antes de aceitar o fecho como definitivo.
> **Tipo:** Verificação directa.
> **Tamanho:** S–M.
> **ADR-0108 EM VIGOR.** A mesma regra de sempre — uma afirmação sobre desempenho ou sobre grandeza de um problema precisa de um número, não de um adjectivo.
> **Dependências:** P562 (onde as duas afirmações sem número apareceram).

---

## Parte 1 — Custo real da passagem de reordenação bidi

```bash
python3 tools/perf/benchmark-p507.py
```

Se o script demorar muito por correr o corpus inteiro, correr pelo menos os casos de documento grande (o mesmo `macro-10x` já usado em P544/P546/P548), comparando antes e depois de P562:

```bash
git log --oneline | grep -i "P561\|P562" | tail -5
# checkout do commit antes de P562, rebuild, medir
```

```bash
cat > /tmp/p563-latin-large.typ <<'EOF'
#lorem(1200)
EOF
time ./target/release/typst /tmp/p563-latin-large.typ /tmp/p563.pdf
```

Medir com o commit antes de P562 e com o commit depois, o mesmo documento grande, sem nenhum RTL — isto confirma ou refuta a afirmação de que a passagem "sai logo" para texto puramente LTR, sem custo perceptível.

### Critério de fecho da Parte 1

- [ ] Benchmark oficial corrido, não substituído por explicação teórica.
- [ ] Documento grande sem RTL medido antes e depois de P562.
- [ ] Se houver custo mensurável: decidir se é aceitável, com número ao lado da decisão, não só a palavra "barata".

---

## Parte 2 — Quantificar a distorção de espaçamento em texto misto

```bash
cat > /tmp/p563-mixed.typ <<'EOF'
#set text(lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p563-mixed.typ /tmp/p563-mixed.pdf
lab/typst-original/target/release/typst compile /tmp/p563-mixed.typ /tmp/p563-mixed-vanilla.pdf
```

Medir as posições reais dos glifos nos dois PDFs, o mesmo método já usado em P549 (`mutool show`, ou extracção directa das posições x de cada trecho):

```bash
mutool show /tmp/p563-mixed.pdf 4 | grep -A2 "Td\|TJ"
mutool show /tmp/p563-mixed-vanilla.pdf 4 | grep -A2 "Td\|TJ"
```

Calcular a diferença em pontos entre a posição esperada (vanilla) e a posição real (cristalino) para cada palavra da linha.

### Critério de fecho da Parte 2

- [ ] Distância medida em pontos, não descrita como "pequena".
- [ ] Decidido se a distância medida é aceitável (com um limiar razoável, por exemplo menos de 1 pt não é visível a olho nu a tamanhos de texto normais) ou se precisa de correcção.

---

## Decisão

Se o custo de desempenho for real e mensurável em documentos grandes, ou se a distorção de espaçamento for maior do que um limiar razoável: não fica fechado. Volta a aberto, com o número a explicar porquê, e um passo de correcção próprio.

Se as duas medições confirmarem que o custo é de facto desprezável e a distorção de facto pequena (com número a apoiar): o fecho de P562 fica confirmado, não só assumido.

---

## Critério de fecho do passo

- [ ] Parte 1: benchmark corrido, custo real medido.
- [ ] Parte 2: distorção medida em pontos.
- [ ] Decisão registada com números, não com adjectivos.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p563.md`.
- [ ] Inventário actualizado, se o estado de RTL mudar de "fechado" para "fechado com ressalva" ou "aberto".

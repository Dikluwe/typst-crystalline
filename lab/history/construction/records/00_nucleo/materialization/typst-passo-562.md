---
# P562 — Ordem visual RTL das palavras na linha

> **Passo:** 562
> **Data:** 2026-07-04
> **Foco:** P561 confirmou, com imagem, que a ordem das palavras numa linha de texto árabe sai da esquerda para a direita no cristalino, quando devia sair da direita para a esquerda. O shaping de cada palavra (formas das letras, ligações) já está correcto desde P484/P521. Falta a camada de layout de parágrafo — decidir onde cada palavra fica posicionada na linha, não só como cada palavra é desenhada por dentro.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** L.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — sonda antes de qualquer código, dado o tamanho e a proximidade com áreas já sensíveis (P482 shaping, P544 métricas de layout).
> **Dependências:** P484 (`bidi_runs`, usado hoje só para shaping), P521 (fronteira de cluster RTL), P560 (fonte embutida corrigida, revelou este problema), P561 (confirmação com imagem).

---

## Contexto

O texto de cada palavra árabe já sai correcto — as letras ligam-se certas dentro da palavra. O que falta é a linha inteira saber que, quando o texto é árabe, a primeira palavra da frase fica à direita, não à esquerda.

`bidi_runs` já existe (P484), mas é usado só dentro do shaper (L3), para decidir a direcção de shaping de cada trecho. O Layouter (L1), que decide onde cada palavra fica posicionada horizontalmente numa linha, trata o texto sempre como uma sequência da esquerda para a direita, independentemente da direcção real do conteúdo.

---

## Sonda

### Confirmar onde o Layouter posiciona palavras

```bash
grep -n "layout_word\|cursor_x\|advance.*word" 01_core/src/engine/layout/cursor.rs | head -20
```

Perguntas, com `file:line`:

1. O Layouter avança `cursor_x` sempre para a direita, palavra a palavra, sem nenhuma noção de direcção?
2. `bidi_runs` está disponível em L1 (Layouter), ou só em L3 (shaper), como P544 e P546 descreveram para o problema de métricas de fonte?
3. Existe alguma informação de nível de bidi (par ou ímpar, indicando LTR ou RTL) que sobrevive da análise de texto até ao momento em que o Layouter decide posições?

### Confirmar o padrão já usado para problemas parecidos

P482 estabeleceu o padrão de "passagem depois do layout" para o shaping — o Layouter trata o texto de forma simples, e uma passagem posterior (`shape_document`, em L3) resolve a fonte e o desenho real de cada glifo. Confirmar se esse mesmo padrão pode servir aqui: o Layouter continua a posicionar palavras da forma simples que já faz, e uma passagem posterior reordena visualmente os grupos de palavras RTL dentro de cada linha, trocando as suas posições horizontais, sem mudar a lógica de quebra de linha em si.

### Critério de fecho da sonda

- [ ] Confirmado como o Layouter posiciona palavras hoje, sem noção de direcção.
- [ ] Confirmado se `bidi_runs` está acessível no momento certo, ou precisa de ser calculado de novo nesta camada.
- [ ] Decisão de desenho: reordenação como passagem posterior (mesmo padrão de P482), ou mudança directa na lógica de posicionamento do Layouter. Registar a escolha com razão, não implementar sem decidir primeiro.

---

## Implementação

Depende da sonda. Se o padrão de passagem posterior for escolhido: depois de o Layouter posicionar as palavras de uma linha (da forma simples, sempre da esquerda para a direita), uma passagem nova identifica os grupos de palavras que pertencem a um trecho RTL (usando a mesma informação de `bidi_runs` já calculada) e troca as suas posições horizontais dentro da linha, preservando a posição do trecho como bloco, mas invertendo a ordem interna das palavras desse trecho.

### Critério de fecho da implementação

- [ ] O documento de teste de P561 (`الكتاب على الطاولة`) mostra a primeira palavra da frase à direita da linha, a última à esquerda.
- [ ] O shaping interno de cada palavra (já correcto) não regride.
- [ ] Testado com uma linha que mistura latim e árabe (por exemplo, um número ou palavra em inglês no meio de uma frase árabe) — confirmar que só o trecho árabe inverte, o trecho latino mantém a ordem normal.
- [ ] Texto latino puro, sem nenhum trecho RTL, sem regressão nenhuma de posição.

---

## Validação

```bash
./target/release/typst /tmp/p561-rtl-order.typ /tmp/p562-cristalino.pdf
mutool draw -o /tmp/p562-cristalino.png -r 150 /tmp/p562-cristalino.pdf
```

Comparar com a imagem do vanilla já obtida em P561.

```bash
cat > /tmp/p562-mixed.typ <<'EOF'
#set text(lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p562-mixed.typ /tmp/p562-mixed.pdf
mutool draw -o /tmp/p562-mixed.png -r 150 /tmp/p562-mixed.pdf
lab/typst-original/target/release/typst compile /tmp/p562-mixed.typ /tmp/p562-mixed-vanilla.pdf
mutool draw -o /tmp/p562-mixed-vanilla.png -r 150 /tmp/p562-mixed-vanilla.pdf
```

```bash
cargo test --workspace
crystalline-lint .
python3 tools/perf/benchmark-p507.py
```

O benchmark importa aqui — uma passagem adicional de reordenação, mesmo que só para trechos RTL, tem custo. Confirmar que documentos sem RTL nenhum não pagam esse custo (a passagem deve identificar rapidamente que não há nada a reordenar e sair, não processar a linha inteira à procura de algo que não existe).

---

## Critério de fecho do passo

- [ ] Sonda completa, decisão de desenho registada com razão.
- [ ] Ordem visual das palavras árabes corrigida, confirmada com imagem contra o vanilla.
- [ ] Caso misto (latim + árabe na mesma linha) testado e correcto.
- [ ] Shaping interno das palavras sem regressão.
- [ ] Texto sem RTL sem regressão de posição nem de desempenho.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p562.md`.
- [ ] Inventário actualizado: item de layout bidireccional fechado, distinto do shaping já fechado antes.

---

## Estado da sequência de RTL

| Camada | Passo | Estado |
|--------|-------|--------|
| Shaping (forma das letras dentro da palavra) | P484, P521 | Fechado |
| Fonte embutida no PDF (o ficheiro abre em leitores comuns) | P560 | Fechado |
| Ordem visual das palavras na linha | Este passo | Em preparação |

Só depois deste passo é que um documento árabe ou hebraico fica completo — legível, não só visível.

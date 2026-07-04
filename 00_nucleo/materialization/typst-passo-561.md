---
# P561 — Ordem visual do texto RTL: confirmar contradição entre P484 e P560

> **Passo:** 561
> **Data:** 2026-07-04
> **Foco:** P560, secção 8, diz que a ordem visual do texto árabe sai da esquerda para a direita, porque o layout bidireccional "ainda não está implementado". Isto contradiz P484 (que introduziu `bidi_runs` e a crate `unicode-bidi`) e P521 (que corrigiu um erro na fronteira de cluster de texto RTL — corrigir algo que não existe não faz sentido). Este passo confirma, com imagem, qual das duas descrições é a certa, antes de qualquer decisão sobre o que falta fazer.
> **Tipo:** Sonda directa. Sem código antes de a contradição ser resolvida.
> **Tamanho:** S–M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P484 (bidi_runs, shaping RTL), P521 (correcção de fronteira de cluster RTL), P560 (onde a afirmação "não implementado" apareceu).

---

## Contexto

Duas descrições que não podem estar as duas certas ao mesmo tempo:

- P484/P521: existe um mecanismo de `bidi_runs` que divide o texto em trechos por direcção, e um bug nesse mecanismo (fronteira de cluster) foi corrigido.
- P560: "a ordem visual dos caracteres árabes... aparece da esquerda para a direita porque o bidi layout ainda não está implementado."

A explicação mais provável, a confirmar: P484/P521 trataram do **shaping** — a forma de cada letra árabe dentro do seu trecho, incluindo a ligação correcta entre letras consecutivas (o árabe muda a forma da letra conforme a posição na palavra). Isso é diferente de decidir **onde cada trecho fica na linha** — se o texto árabe começa à direita da página e a linha se lê da direita para a esquerda como um todo. Este segundo mecanismo pode nunca ter sido ligado ao layout de parágrafo, mesmo com o shaping de cada trecho a funcionar.

Se for isto, um documento árabe pode ter cada palavra desenhada correctamente por dentro, com as letras ligadas certas, mas com as palavras na ordem errada ao longo da linha — o que é ilegível para quem lê árabe, mesmo que "haja texto visível na imagem".

---

## Verificação directa

```bash
cat > /tmp/p561-rtl-order.typ <<'EOF'
#set text(lang: "ar", size: 40pt)
الكتاب على الطاولة
EOF
```

Esta frase tem quatro palavras separadas por espaços — "O livro está na mesa" em árabe. Numa linha correcta, a primeira palavra lida ("الكتاب", "o livro") aparece à **direita** da linha, não à esquerda.

```bash
./target/release/typst /tmp/p561-rtl-order.typ /tmp/p561-cristalino.pdf
mutool draw -o /tmp/p561-cristalino.png -r 150 /tmp/p561-cristalino.pdf

lab/typst-original/target/release/typst compile /tmp/p561-rtl-order.typ /tmp/p561-vanilla.pdf
mutool draw -o /tmp/p561-vanilla.png -r 150 /tmp/p561-vanilla.pdf
```

Comparar as duas imagens lado a lado. Confirmar:

1. No vanilla, a primeira palavra da frase aparece à direita da linha.
2. No cristalino, a primeira palavra aparece à direita (correcto) ou à esquerda (errado, confirma o que P560 disse).
3. Dentro de cada palavra, as letras árabes aparecem ligadas correctamente nos dois casos (isto testa o shaping, que P484/P521 trataram).

### Critério de fecho

- [ ] Imagem comparada directamente, palavra por palavra, não só "há texto visível".
- [ ] Confirmado se a ordem das palavras na linha está certa ou trocada.
- [ ] Confirmado se as letras dentro de cada palavra estão ligadas correctamente (shaping) independentemente da ordem das palavras (layout).
- [ ] As duas descrições (P484/P521 vs P560) reconciliadas — qual delas estava a descrever o quê.

---

## Decisão

Se a ordem das palavras estiver de facto trocada (P560 tinha razão): isto é um item novo, maior do que os anteriores desta sequência — falta ligar a informação de direcção ao layout de parágrafo, não só ao shaping de cada trecho. Regista-se como item próprio, distinto do shaping já corrigido, com prioridade alta — um documento árabe legível é o objectivo, não só um documento árabe que não dá erro ao abrir.

Se a ordem estiver certa (P560 estava enganado, ou a descrever só a falta da propriedade explícita `dir: rtl`, não o mecanismo de facto): corrigir a nota de P560 para não deixar a afirmação errada registada, e confirmar que o aviso de "`dir: rtl` não suportado" é sobre a sintaxe explícita, não sobre o comportamento real do documento.

---

## Relatório de execução

`00_nucleo/diagnosticos/paridade-producao-p561.md` — as duas imagens incluídas ou descritas em detalhe suficiente para não depender de memória. A conclusão final diz, sem ambiguidade, qual das duas descrições anteriores estava certa.

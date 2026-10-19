---
# P614 — Sonda de escrita vertical CJK

> **Passo:** 614
> **Data:** 2026-07-05
> **Foco:** Escrita vertical (chinês, japonês: de cima para baixo, colunas da direita para a esquerda) está confirmada como ausente desde P531, nunca tentada. Não é uma extensão do trabalho de RTL já feito — troca qual eixo é "linha" e qual é "quebra de linha", o que significa que a cascata inteira (letra→palavra→linha→parágrafo→coluna→página) precisa de ser pensada de novo para o eixo vertical, não adaptada rapidamente da horizontal. Este passo é só sonda: mapear o que o vanilla faz, o que o cristalino teria de mudar, e o tamanho real do trabalho, antes de qualquer código.
> **Tipo:** Sonda directa. Sem código nenhum neste passo.
> **Tamanho:** M para a sonda em si; a implementação, se avançar, é provavelmente L ou XL — a confirmar aqui.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — esta é uma área nova, nunca tocada; sonda obrigatória antes de qualquer estimativa de tamanho de implementação.
> **Dependências:** P593 (cascata de largura horizontal, para servir de referência do que precisa de equivalente vertical), toda a sequência RTL (P484-P592) como exemplo do tipo de camadas envolvidas, mas não como código reutilizável directamente.

---

## Sonda

### Confirmar o comportamento do vanilla

```bash
cat > /tmp/p614-vertical.typ <<'EOF'
#set text(lang: "ja")
これは縦書きのテストです。日本語の文章は上から下に読みます。
EOF
lab/typst-original/target/release/typst compile /tmp/p614-vertical.typ /tmp/p614-vanilla.pdf
mutool draw -o /tmp/p614-vanilla.png -r 150 /tmp/p614-vanilla.pdf
```

Confirmar, com a imagem: o vanilla já produz escrita vertical automaticamente para japonês, ou precisa de uma propriedade explícita (algo como `dir: ttb` ou equivalente)?

```bash
cat > /tmp/p614-vertical-explicito.typ <<'EOF'
#set text(lang: "ja", dir: ttb)
これは縦書きのテストです。
EOF
lab/typst-original/target/release/typst compile /tmp/p614-vertical-explicito.typ /tmp/p614-vanilla-explicito.pdf
mutool draw -o /tmp/p614-vanilla-explicito.png -r 150 /tmp/p614-vanilla-explicito.pdf
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p614-vertical.typ /tmp/p614-cristalino.pdf
mutool draw -o /tmp/p614-cristalino.png -r 150 /tmp/p614-cristalino.pdf
```

Confirmar: erro, aviso, ou silenciosamente tratado como horizontal?

### Mapear a cascata para o eixo vertical, nível a nível

Usando o documento já escrito (`mapa-motor-composicao-texto.md`) como referência, confirmar para cada nível o que muda:

| Nível | Pergunta horizontal (já resolvida) | Pergunta vertical equivalente (a confirmar) |
|---|---|---|
| Letra/glifo | Largura do glifo | Altura do glifo — vem da mesma tabela `hmtx`/`vmtx`? Fontes CJK têm métricas verticais próprias (`vmtx`, `vhea`) separadas das horizontais. |
| Palavra | Largura da palavra, com forma de escrita aplicada | CJK não tem "palavras" no mesmo sentido — cada carácter é uma unidade própria; não há ligação de formas como no árabe. |
| Linha | Onde a linha quebra, largura disponível | "Linha" passa a ser uma coluna vertical; a quebra é por altura disponível, não largura. |
| Parágrafo | Alinhamento à esquerda ou direita | Onde a primeira coluna começa (topo direito da página, tipicamente) |
| Coluna/página | Largura útil, fluxo entre colunas | As "colunas" verticais fluem da direita para a esquerda, não de cima para baixo — o eixo de fluxo entre colunas também muda. |

```bash
grep -rn "vmtx\|vhea\|vertical" 03_infra/src/font_metrics.rs 03_infra/src/shaper.rs 2>/dev/null
```

Confirmar se o código já lê alguma tabela de métricas verticais da fonte (`vmtx`), ou se só lê métricas horizontais (`hmtx`) em todo o lado.

### Confirmar pontuação e caracteres latinos misturados em texto vertical

Texto vertical japonês frequentemente mistura números e palavras latinas, que continuam horizontais dentro do fluxo vertical (rotadas ou não, consoante convenções tipográficas). Confirmar como o vanilla trata isto:

```bash
cat > /tmp/p614-misto.typ <<'EOF'
#set text(lang: "ja", dir: ttb)
これはテスト123です。ABC も含みます。
EOF
lab/typst-original/target/release/typst compile /tmp/p614-misto.typ /tmp/p614-misto-vanilla.pdf
mutool draw -o /tmp/p614-misto-vanilla.png -r 150 /tmp/p614-misto-vanilla.pdf
```

### Critério de fecho da sonda

- [ ] Confirmado se o vanilla activa escrita vertical automaticamente ou por propriedade explícita.
- [ ] Confirmado se o cristalino hoje falha, avisa, ou ignora silenciosamente um pedido de escrita vertical.
- [ ] Cascata mapeada nível a nível, com as perguntas específicas do eixo vertical identificadas (tabela acima preenchida com respostas, não só perguntas).
- [ ] Confirmado se o código já lê `vmtx`/`vhea`, ou só métricas horizontais.
- [ ] Confirmado o comportamento esperado para números/latim misturados em texto vertical.
- [ ] Estimativa de tamanho da implementação revista com base no que a sonda encontrar — pode ser maior do que "L", a confirmar aqui, não assumir.

---

## Decisão

Este passo não implementa nada. No fim, produz uma lista clara do que precisa de ser construído, nível a nível, e uma estimativa honesta de tamanho — se for maior do que um único passo consegue cobrir, dizer isso directamente, e propor como dividir o trabalho em passos menores (por exemplo: primeiro só a métrica vertical de glifo e a quebra de coluna, sem misturar latim; depois a mistura de scripts; depois casos avançados).

---

## Critério de fecho do passo

- [ ] Sonda completa, todas as perguntas da tabela respondidas com evidência, não suposição.
- [ ] Comportamento do cristalino hoje confirmado (erro/aviso/silêncio).
- [ ] Estimativa de tamanho revista e honesta.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p614.md`, com as imagens ou descrição suficiente para não depender de memória.
- [ ] Proposta de divisão em passos menores, se o tamanho total for maior do que um passo só.

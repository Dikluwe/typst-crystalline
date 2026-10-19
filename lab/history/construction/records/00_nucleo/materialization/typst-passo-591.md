---
# P591 — Usar largura com forma de escrita aplicada na decisão de quebra de linha

> **Passo:** 591
> **Data:** 2026-07-05
> **Foco:** P590 confirmou, com números, que o cristalino decide onde quebrar a linha somando a largura de cada letra árabe isolada, mas desenha o texto com as letras ligadas — muito mais estreitas. Uma palavra medida em 159,57 pontos sai desenhada com 107,32. Este passo corrige a decisão de quebra para usar a largura real, com a forma de escrita já aplicada, pelo menos para scripts onde isto faz diferença grande (árabe confirmado; a confirmar outros).
> **Tipo:** Sonda + Implementação.
> **Tamanho:** L. Toca a mesma área que P544/P546/P548 já tinham corrigido para outra causa (largura de palavra na decisão de layout).
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — esta área já teve várias causas diferentes antes (P544 métricas erradas, P587 espaço a mais); confirmar a causa exacta de novo antes de código, não assumir que é só isto.
> **Dependências:** P590 (causa confirmada com números), P482 (desenho original: medir sem forma de escrita em L1, aplicar a forma depois em L3), P544/P546/P548 (correcção anterior na mesma área, por outra razão).

---

## Contexto

O desenho actual do projecto separa duas fases: o Layouter (L1) decide onde cada linha quebra, usando larguras medidas sem aplicar as regras de forma de escrita (isto evita que L1 precise de acesso a ficheiros de fonte, mantendo a camada pura). A forma de escrita (letras ligadas, formas iniciais/médias/finais do árabe) só é aplicada depois, em L3, no desenho final.

Para latim, a diferença entre "letra isolada" e "letra em contexto" é pequena — quase só kerning, já corrigido em P544/P546/P548. Para árabe, a diferença é grande, porque a escrita árabe muda a forma de cada letra consoante a posição na palavra, e uma palavra "toda ligada" ocupa muito menos espaço do que a soma das letras separadas.

---

## Sonda

### Confirmar se rustybuzz consegue devolver a largura já com a forma aplicada, sem desenhar

```bash
grep -n "shape\|hb_shape\|rustybuzz" 03_infra/src/shaper.rs | head -20
```

Perguntas, com `file:line`:

1. O mecanismo que já faz shaping em L3 (usado para o desenho final) pode ser chamado antes da decisão de quebra de linha, só para obter a largura, sem gerar o resultado final duas vezes?
2. Se sim: qual é o custo de chamar isto durante o layout, não só no fim? P544/P546 já mostraram que chamadas repetidas de shaping por carácter podem ter custo alto sem cache — confirmar se esse problema volta a aparecer aqui.
3. Existe forma de detectar, antes de gastar tempo a fazer shaping completo, se um trecho de texto precisa mesmo disto (por exemplo, script árabe, hebraico, ou outros scripts com formas contextuais) e só chamar o mecanismo mais caro nesses casos, deixando o resto do texto como está?

### Confirmar noutros scripts, não só árabe

```bash
cat > /tmp/p591-hebraico.typ <<'EOF'
#set text(dir: rtl, lang: "he", size: 40pt, font: "DejaVu Sans")
שלום עולם מה שלומך היום
EOF
./target/release/typst /tmp/p591-hebraico.typ /tmp/p591-heb.pdf
pdfinfo /tmp/p591-heb.pdf | grep Pages
lab/typst-original/target/release/typst compile /tmp/p591-hebraico.typ /tmp/p591-heb-vanilla.pdf
pdfinfo /tmp/p591-heb-vanilla.pdf | grep Pages
```

O hebraico não liga letras da mesma forma que o árabe — confirmar se o mesmo problema aparece, ou se é específico de scripts com ligação obrigatória (árabe, e talvez alguns scripts indianos).

### Critério de fecho da sonda

- [ ] Confirmado se o mecanismo de shaping pode devolver só a largura, sem custo de desenhar duas vezes.
- [ ] Custo medido, não assumido — reaproveitar o método de benchmark já usado em P546/P548.
- [ ] Confirmado se o problema é específico de scripts com ligação obrigatória, ou mais geral.

---

## Implementação

Depende da sonda. Provavelmente: quando o Layouter encontra um trecho de texto num script que exige forma contextual (detectado por análise Unicode do texto, sem precisar de fonte), chamar o mecanismo de shaping nesse ponto só para obter a largura real, em vez de somar larguras isoladas. Para scripts sem esta necessidade (latim, a maioria), manter o método actual, mais barato.

### Critério de fecho da implementação

- [ ] Decisão de quebra de linha usa largura real para scripts que precisam (árabe confirmado).
- [ ] Documento de referência da sequência RTL (`الكتاب 42 على الطاولة`, fonte neutra) cabe numa linha só, como o vanilla.
- [ ] Custo de desempenho para texto latino sem regressão — medido, com o método já estabelecido em P546/P548, não assumido.
- [ ] Hebraico testado, com decisão sobre se precisa da mesma correcção ou não.

---

## Validação

Repetir exactamente a medição de P590, com fonte neutra:

```bash
cat > /tmp/p591-rtl.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt, font: "DejaVu Sans")
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p591-rtl.typ /tmp/p591.pdf
pdftotext -tsv /tmp/p591.pdf -
```

Confirmar que as quatro palavras cabem numa linha só, com posições próximas do vanilla.

```bash
cargo test --workspace
crystalline-lint .
python3 tools/perf/benchmark-p507.py
```

O benchmark aqui é importante — chamar shaping durante a decisão de layout, não só no fim, pode ter custo real em documentos grandes com muito texto árabe. Medir antes de aceitar.

---

## Critério de fecho do passo

- [ ] Sonda completa, causa confirmada, custo medido antes de decidir a abordagem.
- [ ] Implementação corrige o documento de referência da sequência RTL.
- [ ] Hebraico testado e decidido.
- [ ] Texto latino sem regressão de desempenho, medida, não assumida.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p591.md`, com hash do commit.
- [ ] Tabela de estado da sequência RTL actualizada — só fecha de vez se este passo confirmar a correcção com números.

---

## Nota

Este é agora o quarto passo desta sequência a encontrar uma causa diferente para o mesmo sintoma superficial ("a linha quebra onde não devia"): primeiro pareceu ser RTL em si (P566, refutado), depois `finish()` sem avançar (P578, parcialmente errado), depois `font_size_pt` estático (P579/580/582, real mas não suficiente), depois espaço inicial (P587/588, real mas não suficiente), agora shaping não aplicado na medição (P590/591). Cada causa encontrada era real — não eram invenções — mas o sintoma tinha mais do que uma causa a somar-se. Este pode ser o último, ou pode não ser; só a medição final o dirá.

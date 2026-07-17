---
# P623 — Devanágari denso em conjuntas: `advance_shaped` é mesmo desnecessário?

> **Passo:** 623
> **Data:** 2026-07-05
> **Foco:** P622 decidiu não incluir devanágari em `needs_shaped_width`, com a razão "provavelmente as ligaduras não reduzem a largura da mesma forma" — uma suposição, não uma medição. O sânscrito tem muitas consoantes conjuntas, mais do que o texto usado no teste de P622. Este passo testa com um documento desenhado especificamente para ter muitas conjuntas, antes de aceitar a decisão de P622 como confirmada.
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** "Provavelmente" não é uma razão suficiente para não corrigir algo — é um convite a medir.

---

## Verificação

### Documento denso em consoantes conjuntas

Sânscrito clássico tem conjuntas frequentes e complexas — usar um texto conhecido por isso, por exemplo um verso ou frase com várias conjuntas seguidas:

```bash
cat > /tmp/p623-conjuntas.typ <<'EOF'
#set text(lang: "sa", font: "Noto Sans Devanagari", size: 40pt)
धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय
EOF
./target/release/typst /tmp/p623-conjuntas.typ /tmp/p623-cristalino.pdf
pdfinfo /tmp/p623-cristalino.pdf | grep Pages
mutool draw -o /tmp/p623-cristalino.png -r 150 /tmp/p623-cristalino.pdf

lab/typst-original/target/release/typst compile /tmp/p623-conjuntas.typ /tmp/p623-vanilla.pdf
pdfinfo /tmp/p623-vanilla.pdf | grep Pages
mutool draw -o /tmp/p623-vanilla.png -r 150 /tmp/p623-vanilla.pdf
```

Este verso (abertura do Bhagavad Gita) tem várias conjuntas complexas: `र्म` (rma), `क्ष` (ksha), `र्व` (rva), `ण्ड` (nda), `ञ्ज` (nja) — mais densidade de conjuntas do que o texto usado em P622.

### Medir largura real vs largura sem forma de escrita, directamente

```bash
python3 -c "
from fontTools.ttLib import TTFont
import unicodedata

f = TTFont('/usr/share/fonts/.../NotoSansDevanagari-Regular.ttf')  # ajustar caminho
# somar advances de cada codepoint isoladamente vs o que o shaper produziria
# (comparação semelhante à já feita em P590 para árabe)
"
```

Repetir o mesmo tipo de comparação que P590 fez para árabe (largura não-shaped vs largura shaped), agora para este texto devanágari, para ter um número concreto, não só "página igual ou diferente".

### Critério de fecho

- [ ] Documento denso em conjuntas testado, não um texto genérico.
- [ ] Número de páginas comparado entre cristalino e vanilla.
- [ ] Se possível, medição directa de largura shaped vs não-shaped (o mesmo método de P590), para ter um número, não só "parece igual".
- [ ] Testado também com uma margem mais estreita (o mesmo truque usado em P586/587 para forçar a fronteira de quebra de linha a aparecer, tornando qualquer diferença de largura mais visível).

```bash
cat > /tmp/p623-margem-estreita.typ <<'EOF'
#set page(margin: 3cm)
#set text(lang: "sa", font: "Noto Sans Devanagari", size: 40pt)
धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः मामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय
EOF
./target/release/typst /tmp/p623-margem-estreita.typ /tmp/p623-estreito.pdf
pdfinfo /tmp/p623-estreito.pdf | grep Pages
lab/typst-original/target/release/typst compile /tmp/p623-margem-estreita.typ /tmp/p623-estreito-vanilla.pdf
pdfinfo /tmp/p623-estreito-vanilla.pdf | grep Pages
```

---

## Decisão

Se, mesmo com texto denso em conjuntas e margem estreita (forçando a fronteira de quebra a ficar sensível a pequenas diferenças de largura), não houver divergência: a decisão de P622 fica confirmada com prova, não só suposição — actualizar a razão registada de "provavelmente" para o número medido.

Se houver divergência: adicionar devanágari a `needs_shaped_width`, seguindo o mesmo padrão já usado para árabe em P591.

---

## Critério de fecho do passo

- [ ] Teste com texto denso em conjuntas, não genérico.
- [ ] Teste com margem estreita, para maximizar a sensibilidade à diferença de largura.
- [ ] Decisão final registada com número, substituindo a palavra "provavelmente" do relatório de P622.
- [ ] Se confirmado que precisa de correcção: implementada, seguindo o padrão de P591.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p623.md`, com hash do commit.

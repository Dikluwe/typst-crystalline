---
# P622 — Quebras de parágrafo não respeitadas em RTL

> **Passo:** 622
> **Data:** 2026-07-05
> **Foco:** P621 encontrou, por acidente, que duas frases RTL separadas por linha em branco (dois parágrafos) aparecem concatenadas numa única linha visual — a quebra de parágrafo não é respeitada. Isto foi tratado como "divergência pré-existente, não bloqueia o fecho" dentro de um passo sobre tracking, mas é um problema maior, potencialmente presente desde o início da sequência RTL (P560 em diante), nunca testado directamente porque quase todos os testes usaram um parágrafo só. Este passo investiga a fundo, e também responde à pergunta de P621 deixada em aberto sobre `advance_shaped` em devanágari.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M, a confirmar.
> **ADR-0108 EM VIGOR.** Um achado "não bloqueia o fecho" dentro de outro passo não significa "não importa" — significa só que não era o assunto desse passo.

---

## Parte 1 — Quebras de parágrafo em RTL

### Confirmar o sintoma isolado, sem tracking

```bash
cat > /tmp/p622-dois-paragrafos.typ <<'EOF'
#set text(lang: "ar", font: "DejaVu Sans", size: 24pt)
مرحبا بالعالم

هذا فقرة ثانية منفصلة
EOF
./target/release/typst /tmp/p622-dois-paragrafos.typ /tmp/p622-cristalino.pdf
mutool draw -o /tmp/p622-cristalino.png -r 150 /tmp/p622-cristalino.pdf
pdftotext -tsv /tmp/p622-cristalino.pdf -

lab/typst-original/target/release/typst compile /tmp/p622-dois-paragrafos.typ /tmp/p622-vanilla.pdf
mutool draw -o /tmp/p622-vanilla.png -r 150 /tmp/p622-vanilla.pdf
pdftotext -tsv /tmp/p622-vanilla.pdf -
```

Confirmar, com a tabela de posições (já usada ao longo desta sequência): o vanilla produz duas linhas com `top` diferente (dois parágrafos); o cristalino produz uma linha só, ou duas?

### Confirmar se o sintoma é específico de RTL, ou também latim

```bash
cat > /tmp/p622-latim-dois-paragrafos.typ <<'EOF'
Primeiro parágrafo latino aqui.

Segundo parágrafo latino, separado por linha em branco.
EOF
./target/release/typst /tmp/p622-latim-dois-paragrafos.typ /tmp/p622-latim.pdf
pdftotext -tsv /tmp/p622-latim.pdf -
```

Se o latim quebrar parágrafos correctamente e o árabe não, confirma que é específico de RTL. Se o latim também falhar, é um problema mais geral, ainda maior do que RTL.

### Localizar a causa no código

```bash
grep -n "Parbreak\|SyntaxKind::Parbreak\|flush_line.*parbreak" 01_core/src/rules/eval/mod.rs 01_core/src/rules/layout/mod.rs 01_core/src/rules/layout/cursor.rs 03_infra/src/layout_bidi.rs | head -30
```

Dado que P587 já encontrou que uma quebra de linha simples depois de `#set` era tratada como `Content::Space`, confirmar se `Parbreak` (linha em branco, diferente de uma quebra simples) está a ser tratado da mesma forma incorrecta, ou se é um caminho de código completamente diferente, talvez dentro de `layout_bidi.rs` (a passagem de reordenação RTL, que pode estar a juntar coisas que não devia).

### Critério de fecho da Parte 1

- [ ] Sintoma confirmado com números (posições, não só imagem).
- [ ] Confirmado se é específico de RTL ou também afecta latim.
- [ ] Causa localizada com `file:line`.

---

## Parte 2 — `advance_shaped` em devanágari, sem tracking

Pergunta deixada em aberto por P621, nunca respondida.

```bash
grep -n "needs_shaped_width\|Script::Devanagari" 01_core/src/rules/layout/metrics.rs 03_infra/src/font_metrics.rs 03_infra/src/shaper.rs
```

Confirmar se `needs_shaped_width` (criada em P591 para árabe/siríaco/mongol/etc.) inclui devanágari na lista de scripts que activam a largura com forma de escrita aplicada, ou se devanágari usa sempre `advance` simples, sem a correcção de P591.

```bash
cat > /tmp/p622-devanagari-largura.typ <<'EOF'
#set text(lang: "sa", font: "Noto Sans Devanagari", size: 40pt)
नमस्ते संसार यह एक लंबा वाक्य है जो पंक्ति के अंत तक पहुंच सकता है
EOF
./target/release/typst /tmp/p622-devanagari-largura.typ /tmp/p622-deva.pdf
pdfinfo /tmp/p622-deva.pdf | grep Pages
lab/typst-original/target/release/typst compile /tmp/p622-devanagari-largura.typ /tmp/p622-deva-vanilla.pdf
pdfinfo /tmp/p622-deva-vanilla.pdf | grep Pages
```

Se o número de páginas divergir de forma parecida ao que aconteceu com árabe antes de P590/591 (quebra prematura por largura sem forma de escrita), confirma que devanágari tem o mesmo gap, ainda por corrigir.

### Critério de fecho da Parte 2

- [ ] Confirmado se `needs_shaped_width` inclui devanágari.
- [ ] Se não incluir: testado se isso causa o mesmo tipo de quebra prematura já visto para árabe antes de P591.
- [ ] Decisão registada — incluir devanágari na lista, ou confirmar que não é necessário (por exemplo, se devanágari não reduzir tanto de largura ao aplicar forma de escrita como o árabe reduz).

---

## Implementação

Depende das duas sondas. Não implementar sem confirmar as causas primeiro.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Repetir os testes de P621 (tracking) para confirmar que continuam correctos depois de qualquer correcção feita aqui.

---

## Critério de fecho do passo

- [ ] Parte 1: causa das quebras de parágrafo RTL localizada, corrigida ou registada com plano claro.
- [ ] Parte 2: devanágari e `advance_shaped` resolvido, com decisão registada.
- [ ] Sem regressão nos testes de P621.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p622.md`, com hash do commit.
- [ ] Se a quebra de parágrafo RTL afectar passos anteriores da sequência (P560-P621): nota adicionada a cada um, não silenciosamente ignorada.

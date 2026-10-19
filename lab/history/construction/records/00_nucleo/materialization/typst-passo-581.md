---
# P581 — A cadeia de fontes por defeito: Liberation Serif é estável, ou é outro remendo à espera de falhar?

> **Passo:** 581
> **Data:** 2026-07-05
> **Foco:** A fonte por defeito mudou três vezes — Helvetica (nunca funcionou, P538e), FreeSerif (P554, resolveu paginação, quebrou acentos), Liberation Serif (P558, resolveu acentos). Cada mudança só apareceu depois do problema anterior já estar em produção. Este passo testa Liberation Serif de forma mais ampla do que qualquer passo anterior testou as duas fontes que vieram antes, para saber se é estável antes de confiar nela, não depois de mais um problema aparecer.
> **Tipo:** Verificação directa, ampla.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P554, P558 (as duas mudanças anteriores, cada uma testada de forma mais estreita do que devia).

---

## Contexto

Cada uma das duas mudanças anteriores foi validada com um conjunto pequeno de testes, focados no problema que motivou a mudança — paginação para FreeSerif, acentos para Liberation Serif. Nenhuma das duas foi testada de forma ampla contra o tipo de coisas que podem falhar numa fonte: cobertura de caracteres, comportamento com números, pontuação, símbolos comuns, e línguas diferentes do português.

---

## Verificação ampla

### Cobertura de caracteres comuns em documentos reais

```bash
cat > /tmp/p580-cobertura.typ <<'EOF'
#set text(size: 16pt)
Letras: abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ
Números: 0123456789
Pontuação: . , ; : ! ? ( ) [ ] { } " ' - – — / \ @ # $ % & * + = < >
Acentos portugueses: á à â ã é ê í ó ô õ ú ü ç Á À Â Ã É Ê Í Ó Ô Õ Ú Ü Ç
Outras línguas latinas: ñ Ñ ø Ø å Å æ Æ ß
Símbolos matemáticos: + − × ÷ = ≠ ≤ ≥ ∞ √ π
Moeda: € £ ¥ $ ¢
EOF
./target/release/typst /tmp/p580-cobertura.typ /tmp/p580.pdf
pdftotext /tmp/p580.pdf -
mutool draw -o /tmp/p580.png -r 150 /tmp/p580.pdf
```

Confirmar visualmente (não só por extracção) se algum carácter aparece como `.notdef` (caixa vazia ou símbolo de erro), ou trocado.

### Confirmar que a mudança de P558 não regrediu o que P554 tinha corrigido

Repetir exactamente o teste de paginação de P553/P554/P558 (`#lorem(1200)` em duas colunas), confirmando que continua em 2 páginas, não voltou a 3 ou 5.

```bash
cat > /tmp/p580-colunas.typ <<'EOF'
#set page(columns: 2)
#lorem(1200)
EOF
./target/release/typst /tmp/p580-colunas.typ /tmp/p580-col.pdf
pdfinfo /tmp/p580-col.pdf | grep Pages
```

### Testar com uma língua que P554/P558 nunca testaram

```bash
cat > /tmp/p580-outras-linguas.typ <<'EOF'
#set text(size: 16pt, lang: "fr")
Voilà, ça sera intéressant. Où êtes-vous allé?

#set text(lang: "de")
Über die Straße gehen, die Größe.

#set text(lang: "pl")
Łódź, żółw, świnka.
EOF
./target/release/typst /tmp/p580-outras-linguas.typ /tmp/p580-linguas.pdf
pdftotext /tmp/p580-linguas.pdf -
```

Polaco (`ł`, `ż`, `ó`, `ś`, `ę`) é um bom teste porque tem acentos e caracteres que fogem do alfabeto latino básico — se Liberation Serif tiver os mesmos problemas de decomposição de acentos que FreeSerif tinha (P558), isto pode revelar sintomas parecidos noutros caracteres.

### Critério de fecho

- [ ] Nenhum carácter comum aparece como `.notdef` ou trocado, confirmado visualmente.
- [ ] Paginação de P553/P554/P558 continua correcta.
- [ ] Outras línguas latinas testadas, sem sintomas parecidos com o bug de P558.
- [ ] Decisão registada: Liberation Serif confirmada como estável para este conjunto ampliado de testes, ou encontrado um novo problema, com número e razão, não deixado para descobrir depois em produção.

---

## Relatório de execução

`00_nucleo/diagnosticos/paridade-producao-p581.md`, com o hash do commit usado, seguindo a regra de proveniência. Se for encontrado um novo problema: não corrigir por adivinhação — sondar a causa da mesma forma que os passos recentes da sequência RTL fizeram, antes de mudar a fonte outra vez.

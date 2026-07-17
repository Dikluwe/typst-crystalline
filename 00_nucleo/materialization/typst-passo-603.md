---
# P603 — Re-verificar `/Count` de bookmarks isolando a árvore de `/Outlines`

> **Passo:** 603
> **Data:** 2026-07-05
> **Foco:** P602 mediu `/Count` com uma expressão regular que apanha tanto os bookmarks (`/Outlines`) como o catálogo de páginas (`/Pages`), sem distinguir os dois — o próprio relatório reparou nisto, mas não corrigiu o método. Neste caso, o resultado bateu certo por o documento ter uma página só, o que esconde a falta de rigor do método. Este passo repete a verificação com um método que isola só a árvore de `/Outlines`, e testa com um documento de mais do que uma página, onde a mistura teria dado um resultado diferente.
> **Tipo:** Verificação directa.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.** Um método que só bate certo por coincidência não é um método confirmado.

---

## Verificação

### Isolar a árvore de `/Outlines` antes de procurar `/Count`

```bash
python3 -c "
import re
data = open('/tmp/p602-cristalino-depois.pdf', 'rb').read()
# localizar o objecto /Type /Outlines e os objectos referenciados a partir dele,
# não procurar /Count no ficheiro inteiro sem contexto
outlines_start = data.find(b'/Type /Outlines')
print('Posição de /Type /Outlines:', outlines_start)
# mostrar o dicionário à volta desse ponto
print(data[max(0,outlines_start-100):outlines_start+200])
"
```

Construir a verificação de forma a percorrer só os objectos que fazem parte da árvore de bookmarks (a partir de `/Outlines`, seguindo `/First`, `/Next`), não uma busca cega por `/Count` em todo o ficheiro.

### Testar com documento de várias páginas

```bash
cat > /tmp/p603-multipagina.typ <<'EOF'
= Primeira Secção
== Subsecção A
=== Sub-sub A1
#lorem(400)
== Subsecção B
#lorem(400)
= Segunda Secção
#lorem(400)
EOF
./target/release/typst /tmp/p603-multipagina.typ /tmp/p603-cristalino.pdf
pdfinfo /tmp/p603-cristalino.pdf | grep Pages
lab/typst-original/target/release/typst compile /tmp/p603-multipagina.typ /tmp/p603-vanilla.pdf
pdfinfo /tmp/p603-vanilla.pdf | grep Pages
```

Com um documento de várias páginas, o `/Count` do `/Pages` vai ser diferente de qualquer `/Count` de bookmark, tornando a mistura óbvia se o método antigo fosse usado, e confirmando se o método novo (isolado) continua a dar a resposta certa.

### Critério de fecho

- [ ] Método de verificação isola a árvore de `/Outlines`, sem misturar com `/Pages`.
- [ ] Testado com documento de várias páginas, onde a mistura seria visível se ainda existisse.
- [ ] Confirmado que a conclusão de P602 (sinais e valores de `/Count` correctos) se mantém com o método limpo.

---

## Critério de fecho do passo

- [ ] Método de verificação corrigido e reutilizável para passos futuros.
- [ ] Conclusão de P602 confirmada de novo, com o método certo, não só reafirmada.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p603.md`, com hash do commit.

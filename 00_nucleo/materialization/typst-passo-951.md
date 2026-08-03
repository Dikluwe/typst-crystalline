# Passo 951 — `compare.py`: distinguir "regra de layout diferente" de "bug pontual" (variância, não só mediana)

**Precede este passo**: achado metodológico do dono — comparando as 44 equações numeradas do
documento de teste, não há pico isolado (assinatura de bug pontual), mas um **padrão repetido**:
intervalos de equação simples (uma linha) ficam perto de zero; intervalos de equação multi-linha
(fração/integral/matriz) ficam consistentemente entre +7 e +17pt. Isso é uma **regra de
espaçamento vertical diferente entre os dois motores**, não um defeito localizado — mas a mediana
sozinha, como a ferramenta usa hoje, reportaria isto como "mediana alta e estável" sem distinguir
de um caso onde a mesma mediana viesse de um bug pontual grande arrastando a estatística.

**Confirmação adicional do dono, já validada na prática**: emparelhamento por posição (não só
conteúdo) já provou o próprio valor — o token `(1)` do rótulo de equação e o `(1)` dentro da frase
"Referindo-se às equações (1) e (2)" têm o mesmo conteúdo mas colunas x diferentes; só a posição
desambigua. Isto já está coberto pelo desenho actual da ferramenta — não precisa de mudança, só
registo de que funcionou como pretendido.

---

## Fase A — desenhar a extensão de método

1. Confirmar a métrica certa para distinguir os dois casos: variância/desvio padrão dos deltas
   locais dentro de uma secção (ou do documento inteiro), não só mediana e máximo. Um caso "regra
   diferente, sistemática" deveria mostrar deltas agrupados em torno de um valor não-zero, com
   variância baixa dentro de cada grupo (linha única vs multi-linha); um caso "bug pontual" deveria
   mostrar a maioria dos deltas perto de zero, com um ou poucos outliers muito acima do resto.
2. Desenhar a heurística de classificação: por exemplo, se o desvio padrão dos deltas for baixo em
   relação à média (coeficiente de variação baixo), é mais provável ser regra sistemática; se
   houver um ou poucos pontos muito além de N desvios padrão da média do resto, é mais provável
   ser bug pontual. Não é preciso ser perfeito — é para triagem, não veredicto automático.
3. Confirmar se faz sentido agrupar automaticamente por "tipo de construção" (equação de uma linha
   vs multi-linha, por exemplo, contando quebras de linha dentro de cada equação) antes de calcular
   variância — o achado do dono já fez essa distinção manualmente; ver se dá para automatizar sem
   over-engineering.

## Fase B — Implementação

1. Adicionar ao `compare.py`: cálculo de desvio padrão/coeficiente de variação por secção (ou por
   grupo, se a Fase A decidir agrupar), e uma classificação de saída (`sistemático`/`pontual`/
   `indeterminado`) ao lado da mediana já existente.
2. Testar contra os dois casos já conhecidos: o par que motivou P948 (bug pontual, delimitador
   duplicado) deve classificar como `pontual`; o par actual (espaçamento multi-linha) deve
   classificar como `sistemático`.
3. Actualizar `engine/compare.md` (ou onde a spec da ferramenta vive) com a nova secção — mediana
   e máximo continuam existindo, variância/classificação é adicional, não substitui.

## Resultado esperado

- `compare.py` reporta, além da mediana, uma classificação de padrão (sistemático vs pontual vs
  indeterminado) por secção.
- Os dois casos de referência (P948 bug pontual; achado actual sistemático) classificados
  corretamente pela nova lógica.
- Spec da ferramenta actualizada, registando a lição desta investigação.

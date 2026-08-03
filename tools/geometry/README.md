# tools/geometry — comparação geométrica de glifos entre PDFs (P948)

Ferramenta de verificação (não é código do compilador — não passa pelo gate de
Nucleação L0, mesmo padrão de `tools/perf/`).

## O que faz

Dado o mesmo `.typ` compilado por dois binários (cristalino e vanilla), extrai de
cada PDF — caminhando directamente o content stream com `pikepdf` (operadores
`Tj`/`TJ`/`cm`/`Tm`/`Td`/`Tf`, larguras `/W`, `ToUnicode`) — a **posição absoluta
(x, y), o texto e a fonte/tamanho de cada glifo desenhado**, e compara-os:

1. **Secções**: divide o documento pelos cabeçalhos numerados `N.` na margem
   esquerda (funciona com runs únicos à vanilla e com dígitos glifo-a-glifo à
   cristalino). Sem cabeçalhos, compara o documento inteiro como secção 0.
2. **Emparelhamento**: alinha os glifos de cada secção por ordem de leitura
   (linhas por y, x dentro da linha) com `difflib` sobre os codepoints —
   robusto a ordens de emissão diferentes no stream (ex.: peças de assembly).
3. **Deltas por construção**: cada par é medido relativamente à origem do seu
   cluster de linha (a "construção"), não à página — imune a diferenças
   legítimas de largura/altura de página (ex.: centragem com `width: auto`,
   que desloca equações inteiras ~metade da diferença de largura da página).
4. **Relatório**: tabela por secção (medianas e máximos de |Δx|/|Δy|, glifos
   acima do limiar) ordenada pelo pior desvio + JSON completo com o detalhe por
   glifo.

## Uso

```bash
../../lab/.venv/bin/python compare.py A.pdf B.pdf [--limiar 0.5] [--json out.json] [--secoes 4,5,21]
```

Dependências: `pikepdf` (já presente em `lab/.venv`).

## Auto-testes (correr antes de confiar num relatório)

- **Delta zero**: `compare.py test_crystalline.pdf test_crystalline.pdf` →
  30 secções, todos os deltas 0.0, 0 glifos acima do limiar.
- **Divergência conhecida**: `compare.py temp/p944/out-p943.pdf test_vanilla.pdf`
  (render P943, com os defeitos de P944 ainda presentes) → secções defeituosas
  com desvios grandes (ex.: max|dx| > 90pt em 21/23/29/30), secções sãs ~0.

## Notas e limitações conhecidas

- Glifos sem `ToUnicode` (extensores de assembly no vanilla) entram como `∅` —
  emparelham por ordem, não por codepoint.
- A orientação de y de cada PDF (top-origin vs bottom-origin) é normalizada por
  heurística sobre a MediaBox; os dois compiladores actuais ficam consistentes.
- O emparelhamento é por sequência (não por significado): em construções muito
  divergentes (conteúdo diferente), alguns pares podem ser espúrios — usar a
  mediana por secção para triagem e o `detalhe` do JSON para confirmar casos
  concretos antes de abrir um passo.

## P951 — variância e classificação de padrão por secção

Além de mediana/máximo, cada secção reporta `padrao` (+ `cv`, coeficiente de
variação dos |delta|), para distinguir **regra de layout diferente** de **bug
pontual** (achado metodológico do dono: a mediana sozinha não distingue os
dois):

- **`sistemático`**: cluster apertado (desvio interno < 1pt) de deltas em torno
  de um centro ≥2pt, cobrindo ≥8% dos glifos da secção, em qualquer eixo —
  assinatura de regra diferente entre os motores (ex.: espaçamento vertical de
  equações multi-linha; conteúdo extra deslocando a secção inteira).
- **`pontual`**: maioria ~zero com poucos outliers (mediana + 6×MAD) —
  assinatura de bug localizado.
- **`indeterminado`**: limpa, ou misto sem assinatura clara.

É heurística de triagem, não veredicto — confirmar no `detalhe` do JSON antes
de abrir passo. Validação de referência: caso sintético de outlier isolado →
`pontual`; cluster apertado ~10.5pt (o achado do dono que motivou P952) →
`sistemático`; auto-comparação → tudo `indeterminado`.

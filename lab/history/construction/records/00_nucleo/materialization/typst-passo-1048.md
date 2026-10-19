# Passo 1048 — Investigar os 0.109pt na ponta da peça de montagem do delimitador elástico

**Tipo**: Investigar só — não corrigir neste passo. Objectivo: distinguir se a diferença
é **inerente à fonte** (dados diferentes entre o ficheiro de fonte usado pelos dois
binários — mesma classe de "não-achado" já confirmada no P1026 para outro caso) ou se é
**um passo de cálculo em falta** no cristalino (ex.: overlap/kerning entre peças de
`MathStretchyAssembly` não aplicado, ou aplicado com valor ligeiramente diferente).
**Achado de origem**: P1047 — ponta inferior de `⎩` (chave elástica montada por peças) a
$y_{\max}=48.446$pt no cristalino vs $48.337$pt no vanilla, diferença de 0.109pt,
suficiente para cruzar um limiar de raster a 150dpi.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1047.

---

## Fase A — Confirmar se é a mesma fonte nos dois binários (mesmo procedimento do P1026)

Antes de olhar para o mecanismo de montagem, eliminar a hipótese mais barata primeiro:

1. Confirmar que os dois binários (cristalino e vanilla de referência) estão a usar
   **exactamente o mesmo ficheiro de fonte matemática** neste teste — mesmo md5, não só
   "a mesma fonte por nome".
2. Se os ficheiros de fonte diferirem: repetir o procedimento do P1026 (verificar tabela
   `MATH` byte-a-byte, contornos e avanços das peças `⎧`/`⎨`/`⎩` especificamente,
   `cmap`). Se a tabela `MATH` e os contornos das peças de `⎩` forem idênticos entre as
   duas fontes usadas, a diferença **não é a fonte**, avançar para a Fase B. Se
   divergirem, **é a fonte** — não-achado, registar e não perseguir (mesmo tratamento do
   P1026).

## Fase B — Se a fonte é idêntica: localizar o mecanismo de montagem

1. Ler `layout_stretchy_delimiter`/`layout_assembly` (ou equivalente, confirmar nome
   actual pós-fatiamentos) no cristalino — como calcula a posição/overlap entre as peças
   `top`/`extension`/`middle`/`bottom` de uma `MathStretchyAssembly`.
2. Ler o mecanismo equivalente no vanilla (`glyph.rs`/`assemble`, já citado no P1026 —
   reaproveitar essa localização, não procurar do zero).
3. Comparar especificamente: o vanilla aplica algum overlap/kerning entre peças
   consecutivas (campo OpenType MATH `MathGlyphPartRecord.startConnectorLength`/
   `endConnectorLength`, ou equivalente) que o cristalino não aplica, ou aplica com valor
   diferente?
4. Medir directamente: para a peça `bottom` (`⎩`) especificamente, o vanilla usa a altura
   nativa do glifo, ou ajusta por algum campo adicional antes de posicionar a base final?

## Fase C — Resultado

Duas respostas possíveis, sem presumir qual antes de medir:

1. **É a fonte** (Fase A já resolve) — registar como não-achado no mesmo diagnóstico de
   delimitadores do P1026, não perseguir mais.
2. **É um passo de cálculo em falta** (overlap/kerning não aplicado, ou aplicado
   incorrectamente) — isto é um achado de precisão sub-pixel, catalogar com `file:line`
   de onde o vanilla lê o campo em falta. **Não corrigir neste passo** — dado que é
   sub-visual (0.109pt, só cruza limiar de raster por coincidência de tamanho), decidir
   com o dono se vale passo de correcção próprio ou fica registado como débito de baixa
   prioridade.

---

## O que este passo NÃO faz

- Não corrige nenhum código.
- Não decide sozinho se vale a pena corrigir — só localiza a causa e classifica
  fonte-vs-mecanismo, deixando a decisão de prioridade para depois.

## Resultado esperado

Resposta fechada: a diferença de 0.109pt é efeito de fonte (não-achado) ou de um campo de
overlap/kerning não aplicado no cálculo de montagem do cristalino (achado, com
`file:line` de onde falta ler o valor certo).

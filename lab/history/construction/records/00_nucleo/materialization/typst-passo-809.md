# Prompt — typst-passo-809: itálico matemático não estilizado (`x` vs `𝑥`)

**Origem**: observação de P786 §7, confirmada como causa distinta e ainda aberta em P799 e P800 (achados #13 e #7 de P798)
**Estado**: aguardando execução

---

## Achado (como já foi observado, texto consolidado)

Conteúdo de modo matemático (variáveis como `x`, `alpha`, `beta`) extrai/renderiza como texto plano (`x`, `αβ`) em vez de com o estilo itálico matemático Unicode que o vanilla usa (`𝑥`, `𝛼𝛽` — caracteres do bloco "Mathematical Alphanumeric Symbols"). Observado em P786 §7, testado e confirmado como causa **diferente** do que P799 corrigiu (posicionamento de sub/superscript) e do que P800 corrigiu (baseline da equação) — nenhum dos dois mexeu em selecção de fonte/estilo dos glifos.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Cada afirmação do relatório tem de vir com comando exacto + saída literal, comparando vanilla (`lab/typst-original/target/release/typst`) e cristalino (`./target/release/typst`). A contagem de testes da suíte `typst-core` tem de aparecer e bater com os testes novos declarados.

---

## Passo 1 — Sonda (obrigatória antes de qualquer alteração de código)

1. Compilar `$x$`, `$alpha$`, `$alpha beta$` com os dois binários e extrair o texto do PDF resultante (`pdftotext` ou `mutool draw -F txt`, o mesmo método já usado em P799/P800/P805a — texto extraído, não só renderização visual, porque a substituição Unicode afecta especificamente a extracção/cópia de texto, não necessariamente o desenho do glifo). Registar a saída literal.
2. Confirmar se o problema é de **extracção** (glifo certo desenhado, ToUnicode mapeando para o carácter errado — mesma categoria do bug de P805a) ou de **selecção de fonte/glifo** (o glifo desenhado já não é o itálico matemático). Usar `mutool trace` para inspeccionar qual glifo é efectivamente pedido no content stream, tal como em P799.
3. Localizar no código-fonte do vanilla (`lab/typst-original/`) o mecanismo que faz a conversão de letras latinas/gregas para os codepoints do bloco "Mathematical Alphanumeric Symbols" em modo matemático — normalmente uma tabela de mapeamento aplicada na fase de shaping/layout matemático, condicionada ao estilo (itálico é o default em modo math para variáveis de letra única, mas `#text()` ou funções como `upright()`/`bold()` mudam isso).
4. Localizar no código do cristalino o ponto equivalente (ou a ausência dele) — é provável que o `math::layout` monte o texto literal sem passar pela tabela de conversão, ou que a tabela exista mas não esteja a ser aplicada no caminho usado pelos casos testados no Passo 1.
5. Registar os pontos exactos (vanilla e cristalino) no relatório antes de tocar em código.

## Passo 2 — Implementação

Aplicar a tabela de conversão de estilo matemático (itálico por defeito para variáveis de letra única, com os modificadores relevantes já existentes no projecto — `math.class()`, `upright`, `bold`, mencionados no handoff `p798` como já tratados noutros achados) no caminho de layout/render de conteúdo matemático do cristalino. Cobrir pelo menos letras latinas minúsculas e o alfabeto grego usado nos testes do Passo 1; não é necessário cobrir a tabela inteira do vanilla (que tem centenas de codepoints) — cobrir o que os casos de teste exigem e registar o resto como scope-out explícito, seguindo o padrão já usado no projecto para tabelas grandes (ex.: `Lang::ENGLISH` único inicialmente, constantes adicionadas on-demand).

## Passo 3 — Validação

1. Recompilar o cristalino.
2. Repetir os comandos do Passo 1 e mostrar a saída literal, agora igual à do vanilla para os casos cobertos.
3. Adicionar casos de teste cobrindo pelo menos uma letra latina, uma letra grega, e um caso com modificador de estilo (`upright`/`bold`, se já existirem consumers para esses estilos em modo math) para confirmar que a conversão não ignora estilo explícito.
4. Correr a suíte `typst-core` completa e mostrar o comando e a contagem de testes antes/depois.

## Passo 4 — Relatório

Produzir `00_nucleo/materialization/typst-passo-809-relatorio.md` com:
- Comando + saída literal do Passo 1 (antes da correcção), incluindo a distinção extracção vs selecção de glifo.
- Trecho do código vanilla e do código cristalino identificados no Passo 1.
- Alcance exacto da tabela de conversão implementada e o que ficou fora (scope-out explícito, se aplicável).
- Diff da correcção.
- Comando + saída literal do Passo 3 (depois da correcção).
- Contagem de testes antes/depois.
- Actualizar o handoff mais recente (`handoff-novo-chat-p807.md` ou o que estiver activo nessa altura) removendo este item da lista de abertos.

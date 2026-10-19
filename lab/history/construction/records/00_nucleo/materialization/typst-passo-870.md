# Prompt — typst-passo-870: implementar PNG e SVG como formatos de saída reais (resto do achado de P861/P866, decisão do dono em P868)

**Origem**: P866 implementou a detecção de formato e a rejeição explícita de PNG/SVG (em vez de gerar PDF disfarçado); a decisão de escopo foi levada ao dono em P868, que confirmou: implementar de verdade, num passo futuro. Este é esse passo.
**Estado**: aguardando execução — implementação real, autorizada a portar do vanilla onde fizer sentido.

---

## Autorização explícita de porte do vanilla

Diferente da maioria dos achados deste projeto (onde a regra é medir o observável e reimplementar do zero na arquitetura própria do cristalino), este passo tem autorização direta para **copiar/portar lógica do vanilla** onde isso for sensato — rasterização de gráficos vetoriais e exportação SVG são domínios com muita complexidade acumulada (anti-aliasing, hinting, regras de composição de path) que não têm motivo tipográfico para reinventar do zero. Ainda assim, usar julgamento: portar o algoritmo não é o mesmo que importar a crate inteira sem entender o que ela faz — confirmar que qualquer coisa portada é compreendida o suficiente para manter depois.

---

## O que já está confirmado, não precisa resondar

- Formatos do vanilla: `pdf`, `png`, `svg`, `html`, `bundle` (`typst-cli/src/args.rs:591-603`, medido por P866). Este passo cobre `png` e `svg` — os dois que P866 já preparou a detecção/CLI para aceitar. `html`/`bundle` ficam fora, não fazem parte deste achado.
- O vanilla separa isso em duas crates: `typst-render` (rasterização PNG) e `typst-svg` (exportação SVG) — ambas de peso significativo, mencionadas por P866.
- Dependências candidatas identificadas por P866 como ausentes no cristalino: `tiny-skia`, `resvg`, `pixglyph`, `bytemuck`.
- O CLI já detecta o formato pedido (`OutputFormat` em `02_shell/src/cli.rs`, `resolve_output_format_with`) e hoje recusa com erro claro em `04_wiring/src/main.rs` — este passo troca a rejeição pela implementação real.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino) para PNG e para SVG. Contagem de testes discriminada por crate.

---

## Passo 1 — Avaliar o peso real antes de decidir a estratégia

1. Ler `typst-render` e `typst-svg` do vanilla (`lab/typst-original/crates/`) para entender a estrutura: o que cada crate recebe como entrada (provavelmente o mesmo `Frame`/documento paginado que o exportador PDF já consome) e o que produz.
2. Confirmar se as dependências candidatas (`tiny-skia`, `resvg`, `pixglyph`, `bytemuck`) já estão na whitelist `l1_allowed_external` de `crystalline.toml`, ou se precisam ser adicionadas — e se isso é uma decisão que ainda precisa voltar ao dono (mesmo padrão de SVG-como-imagem/P834), ou se a autorização já dada neste prompt cobre isso.
3. Estimar o esforço real (não a estimativa de P866, que era preliminar) para cada formato separadamente — PNG e SVG podem ter pesos bem diferentes; não tratar como um pacote único se um for muito mais barato que o outro.

## Passo 2 — Implementar PNG

1. Se o vanilla usa `tiny-skia` para rasterizar o `Frame` final (path fills, strokes, texto via glifos), portar essa lógica para uma função nova de L3 (`03_infra/src/export/` ou um módulo novo `03_infra/src/render/`, seguindo a convenção de organização já usada no projeto).
2. Conectar ao CLI: quando `OutputFormat::Png`, chamar o rasterizador em vez do exportador PDF, sobre o mesmo `Frame`/documento já produzido pelo pipeline (não duplicar o layout).
3. Confirmar resolução/DPI — o vanilla tem uma flag `--ppi` (ou equivalente); replicar se fizer parte do observável esperado pelo usuário do CLI.

## Passo 3 — Implementar SVG

1. Se o vanilla usa `resvg`/mecanismo próprio para gerar o SVG a partir do `Frame`, portar de forma equivalente — atenção especial a texto (glifos como `<path>` vs `<text>`, confirmar o que o vanilla faz) e a como gradientes/imagens embutidas (já suportados no exportador PDF) são traduzidos para SVG.
2. Conectar ao CLI da mesma forma que PNG.

## Passo 4 — Validação

1. Para um conjunto de documentos de teste já usados em achados anteriores (texto simples, imagem, gradiente, forma com `radius`, equação) — gerar PNG e SVG nos dois binários e comparar: PNG por comparação visual (RMSE, mesmo método já usado em P855); SVG por estrutura (parseável, elementos correspondentes) já que comparação byte-a-byte de SVG é frágil demais para ser útil.
2. Confirmar que PDF (o caminho já funcionando) não regride.
3. Confirmar que `--format` e a extensão do `-o` continuam funcionando como P866 implementou, agora efetivamente produzindo o arquivo certo em vez de recusar.
4. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-870-relatorio.md` com: a avaliação de peso real (Passo 1), o que foi portado do vanilla e o que foi reimplementado (com justificativa para cada escolha), a validação visual/estrutural de PNG e SVG contra o vanilla, e as contagens de teste discriminadas por crate. Se PNG e SVG tiverem pesos muito diferentes e um deles não for viável neste passo, fechar o que for viável e deixar o outro registrado como pendência explícita — não forçar os dois a qualquer custo.

# Prompt — typst-passo-871: converter texto em path no exportador SVG (resto do achado de P870)

**Origem**: P870 implementou PNG e SVG como formatos de saída reais. PNG usa contornos de glifo reais via `pixglyph` e está correto (RMSE ≈2.5 contra o vanilla, confirmado em duas versões vanilla diferentes). SVG só escreve `<text font-family="...">`, delegando a resolução da fonte ao visualizador — o que quebra assim que a fonte não está instalada no sistema de quem abre o arquivo (medido pelo dono: fallback para Noto Sans em vez de Libertinus Serif).
**Estado**: aguardando execução

---

## Por que fazer isto

Um SVG que só referencia o nome da fonte não é um documento portável — ele funciona só no ambiente de quem o gerou. O vanilla resolve isso convertendo cada glifo em contorno (`<symbol>`/`<path>`), tornando o SVG uma imagem vetorial autocontida, do mesmo jeito que o PDF é autocontido (fontes embutidas/subset). Este passo faz o cristalino fazer o mesmo.

## O que já existe e deve ser reaproveitado, não recriado

O projeto já extrai contornos de glifo em pelo menos dois lugares:
1. `render.rs` (P870, PNG) — usa `pixglyph` para rasterizar glifos a partir da fonte resolvida.
2. O exportador PDF (`03_infra/src/export/`) — já lida com subsetting e embedding de fontes, o que exige acesso aos contornos/outlines dos glifos em algum nível.

Antes de escrever qualquer código novo, confirmar qual desses dois caminhos já expõe os contornos vetoriais (curvas Bézier) do glifo em formato reutilizável — `pixglyph` provavelmente rasteriza direto para bitmap, o que não ajuda aqui (SVG precisa do path, não do pixmap); o caminho do PDF é mais provável de já ter algo próximo, mesmo que precise de adaptação.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-infra` a bater com os testes novos.

## Passo 1 — Localizar a extração de contorno reutilizável

1. Ler o código de extração de glifo usado no exportador PDF (provavelmente via `ttf-parser`, já que é a crate usada no resto do projeto para acessar dados de fonte) — confirmar se ele já produz uma representação de path (lista de comandos moveto/lineto/curveto) ou só bytes crus da fonte para embedding.
2. Se a representação de path já existir em algum lugar (mesmo que para outro propósito), extrair/expor como função reutilizável em L3, sem duplicar lógica entre PDF e SVG.
3. Se não existir (o caminho do PDF só lida com bytes crus para embedding, sem decompor em path), usar `ttf-parser::Face::outline_glyph` (ou equivalente) diretamente no exportador SVG — essa API já expõe os comandos de contorno por glifo, é o caminho mais direto.

## Passo 2 — Implementação

1. Em `03_infra/src/export/svg.rs`, trocar a emissão de `<text>` (caminho `_with_fonts`, que já tem acesso às fontes resolvidas) por: para cada glifo do texto shaped, extrair o contorno e emitir como `<path>` (ou `<symbol>` + `<use>`, replicando a estrutura do vanilla — `<symbol>` reutilizável por glifo repetido é mais compacto que repetir o path inteiro cada vez que a letra aparece, e é o padrão que o vanilla já usa segundo a comparação estrutural de P870).
2. Posicionar cada glifo com a mesma transformação (`matrix(...)`) já usada no `<text>` atual — a posição já está correta (P870 confirmou isso), só a forma de desenhar o glifo muda.
3. Manter a variante sem fontes (`export_svg`, sem `_with_fonts`) com o comportamento atual (texto omitido) — essa variante existe para quando não há fonte resolvida disponível, e não tem outorno para converter.

## Passo 3 — Validação

1. Repetir a comparação estrutural do Passo 7.2 de P870 (o mesmo documento "Hello World"), confirmando que o cristalino agora usa `<path>`/`<symbol>` em vez de `<text>`, com posição e forma do glifo batendo com o vanilla.
2. Testar num ambiente **sem** a fonte do documento instalada (removendo ou ignorando a fonte do sistema, ou testando numa VM/container limpo, se disponível) — confirmar que o SVG cristalino agora renderiza corretamente mesmo sem a fonte instalada, fechando o problema que motivou este passo.
3. Comparar o tamanho do arquivo antes/depois — confirmar que o crescimento é da mesma ordem de grandeza do vanilla (P870 mediu vanilla ~7-10KB vs cristalino ~500B para o caso mínimo; depois desta mudança os dois devem ficar próximos).
4. Confirmar que PNG (já correto) não é afetado por esta mudança.
5. Suíte completa, comando + contagem antes/depois, discriminada por crate. Usar o binário vanilla de referência do projeto (`lab/typst-original/target/release/typst`, 0.15.0) para a comparação — não um binário do sistema, retomando a consistência que P870 quebrou.

## Relatório

`00_nucleo/diagnosticos/typst-passo-871-relatorio.md` com: onde a extração de contorno foi encontrada/reaproveitada ou implementada (Passo 1), o diff, a comparação estrutural batendo com o vanilla, a confirmação explícita de que o texto renderiza sem depender de fonte instalada no visualizador (o teste que mais importa aqui), e as contagens de teste discriminadas por crate.

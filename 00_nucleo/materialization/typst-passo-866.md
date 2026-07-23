# Prompt — typst-passo-866: CLI ignora a extensão do arquivo de saída (achado de P861, item 4)

**Origem**: nota registrada desde P831, confirmada ainda ativa por P861 — `typst simple.typ -o simple.png` — vanilla gera PNG; cristalino gera PDF (ignora a extensão pedida)
**Estado**: aguardando execução

---

## Achado (medição de P861)

`typst simple.typ -o simple.png` — vanilla gera um PNG de fato (renderização rasterizada); cristalino escreve PDF no arquivo, apesar da extensão `.png` pedida.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-shell`/`typst-wiring` a bater com os testes novos (este achado é de CLI, não de `typst-core`).

## Passo 1 — Sonda

1. Confirmar o caso com `.png` e testar também `.svg` se o vanilla suportar exportação para SVG via CLI — mapear a lista completa de formatos de saída que o vanilla aceita pela extensão (ou por uma flag explícita, tipo `--format`).
2. Localizar no cristalino (`02_shell/` ou `04_wiring/`, onde a CLI decide o formato de saída) como a extensão do argumento `-o` é tratada hoje — se é ignorada por completo, ou se há algum código morto/parcial para isso.
3. Confirmar se o cristalino já tem capacidade de gerar PNG internamente (pode já existir para outro propósito, como os testes de renderização usados em vários achados anteriores com `mutool draw`/`pdftoppm` — mas isso é ferramenta externa, não necessariamente algo que o binário `typst` sabe fazer sozinho; confirmar se há alguma crate de rasterização já disponível em L3/L4).

## Passo 2 — Implementação

Adicionar a detecção de formato pela extensão do argumento `-o` (ou pela flag explícita, replicando a interface do vanilla), com rasterização real para PNG (e SVG, se aplicável) usando a infraestrutura já disponível no projeto. Se rasterização PNG exigir uma dependência nova de peso significativo, isso é decisão de escopo — apresentar ao dono antes de implementar, mesmo padrão de SVG/PDF-como-imagem já usado no projeto.

## Passo 3 — Validação

1. `-o simple.png` gerando um PNG de verdade, comparável ao vanilla (mesmo método de comparação visual já usado em achados de geometria — RMSE, ou equivalente).
2. Confirmar que `-o simple.pdf` (o caso comum, já funcionando) continua sem regressão.
3. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-866-relatorio.md` com medição antes (lista completa de formatos suportados pelo vanilla), código identificado, diff (ou decisão de escopo formal se depender de dependência pesada), medição depois, contagem de testes.

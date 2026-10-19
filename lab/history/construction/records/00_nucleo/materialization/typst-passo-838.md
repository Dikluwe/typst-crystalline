# Prompt — typst-passo-838: `text::font::book` — fallback CJK sem similarity scoring (achado #24)

**Origem**: achado #24 de P831 (lote 5)
**Estado**: aguardando execução

---

## Achado (medição de P831)

Fallback para caracteres CJK sem fonte explícita: vanilla embute `NotoSansCJKjp-Regular` escolhida por scoring de similaridade (`book.rs:94-115,139-185`); cristalino escolhe a primeira fonte por ordem de índice que cobre o caractere (uma fonte fallback qualquer, ex. DroidSansFallbackFull), resultando em glifos visivelmente diferentes (9.6pt/glifo vs 16pt/glifo do vanilla) e quebra de linha diferente. Cristalino: `03_infra/src/shaper.rs:604-626`, `font_metrics.rs:708-733`, `fallback_fonts.rs:28-33` (sem entradas CJK nas listas).

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Testar texto CJK (japonês, chinês, coreano) sem `font:` explícito nos dois binários — confirmar qual fonte cada um escolhe (via metadados do PDF ou log de debug) e medir o tamanho de glifo/quebra de linha resultante.
2. Localizar no vanilla o algoritmo de scoring de similaridade (`book.rs:94-185`) — entender os critérios (provavelmente peso, largura, presença de cobertura Unicode ampla) usados para escolher entre múltiplas fontes fallback candidatas.
3. Confirmar a lista de fontes fallback disponíveis no ambiente do cristalino (`fallback_fonts.rs`) — se não há nenhuma fonte CJK de qualidade equivalente ao Noto Sans CJK disponível no ambiente de build/teste, isso pode limitar o que dá para medir (a fonte específica pode não estar instalada).

## Passo 2 — Implementação

Implementar o scoring de similaridade no fallback do cristalino, replicando os critérios do vanilla. Se a fonte de referência do vanilla (Noto Sans CJK) não estiver disponível no ambiente, adaptar o teste para comparar contra a fonte que **estiver** disponível nos dois lados, focando em confirmar que o **critério de escolha** bate, não que o binário exato escolhido seja idêntico (isso pode depender do inventário de fontes do ambiente, que é mecânica, não língua — mesma distinção já usada em P816 para o caso de fonte embutida).

## Passo 3 — Validação

1. Recompilar. Repetir o teste CJK, confirmar que a fonte escolhida (ou o critério de escolha, se o ambiente não tiver a mesma fonte) bate com o vanilla.
2. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-838-relatorio.md` com medição antes, código identificado, diff, medição depois, contagem de testes. Se a limitação de ambiente (fonte CJK específica ausente) impedir uma comparação byte-idêntica, documentar isso explicitamente em vez de forçar uma conclusão.

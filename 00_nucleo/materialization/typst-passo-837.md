# Prompt — typst-passo-837: `text::font::metrics` — `top-edge`/`bottom-edge` inválido aceito em silêncio (#22) e `Length` descartado (#23)

**Origem**: achados #22 e #23 de P831 (lote 5)
**Estado**: aguardando execução

---

## Achado #22

`#set text(top-edge: "middle")` — vanilla `error: expected "ascender", "cap-height", "x-height", "baseline", "bounds", or length` (exit 1); cristalino exit 0 silencioso, cai no default. Cristalino: `01_core/src/engine/eval/rules.rs:1624-1629`, `03_infra/src/font_metrics.rs:175-178`. Vanilla: `text/mod.rs:1169-1177`.

### Sonda
Testar `top-edge:`/`bottom-edge:` com string inválida, string válida das enumeradas, e um `Length` explícito, nos dois binários.

### Implementação
Adicionar validação: string fora do domínio enumerado → erro com a mensagem exata do vanilla (lista completa das opções válidas).

---

## Achado #23

`#set text(top-edge: 18pt, bottom-edge: -4pt)` — vanilla `yMin=10.120000 yMax=32.920000` (a métrica muda com o valor); cristalino `5.280000/28.080000` (idêntico ao default — os valores de `Length` são descartados). Cristalino: `rules.rs:1624-1635` (`if let Value::Str` — o braço `Value::Length` nunca existe, então cai no fallback).

### Sonda
Testar `top-edge`/`bottom-edge` com `Length` explícito e medir a bbox resultante (`pdftotext -bbox` ou `pdfinfo`, mesmo método de P831) nos dois binários — confirmar que o cristalino ignora o valor.

### Implementação
Adicionar o braço `Value::Length` em `rules.rs`, propagando o valor até `03_infra/src/font_metrics.rs` (que precisa aceitar edges explícitos em unidade absoluta, não só os nomes enumerados).

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino) para cada achado. Contagem de testes de `typst-core` a bater com os testes novos.

## Validação (comum aos dois achados)
1. Recompilar. Repetir os casos de sonda — erro para string inválida, bbox correta para `Length` explícito.
2. Confirmar que os nomes enumerados válidos (`"ascender"`, `"cap-height"`, etc.) continuam funcionando sem regressão.
3. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-837-relatorio.md`, uma seção por achado (#22, #23).

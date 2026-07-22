# Prompt — typst-passo-823: `loading::cbor_` — mensagem de erro CBOR usa `Debug` interno da crate (achado #10 de P810)

**Origem**: achado #10 da tabela de P810
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> mensagem de erro CBOR: Debug do ciborium vs texto amigável + ficheiro + span

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. Construir um ficheiro CBOR malformado (bytes inválidos) e compilar um documento que o carregue via `#cbor(...)` (ou a função equivalente) com os dois binários — registar a mensagem de erro literal de cada um. O cristalino deve estar a expor o `Debug` da struct de erro interna da crate `ciborium` (mensagem técnica, sem contexto de ficheiro/posição); o vanilla deve mostrar uma mensagem amigável com o caminho do ficheiro e, se aplicável, posição do byte problemático.
2. Localizar no vanilla (`lab/typst-original/`) a rotina de carregamento de CBOR (`crates/typst-library/src/loading/cbor.rs` ou equivalente) — como envolve o erro da crate numa mensagem amigável.
3. Localizar `native_cbor`/loading equivalente no cristalino e confirmar que propaga o erro da crate directamente (`format!("{:?}", err)` ou similar).
4. Registar os pontos antes de tocar em código.

## Passo 2 — Implementação

Envolver o erro da crate `ciborium` numa mensagem amigável, com caminho do ficheiro e, se o vanilla expuser, posição do byte problemático — replicando o formato de mensagem do vanilla.

## Passo 3 — Validação

1. Recompilar. Repetir o comando do Passo 1, mensagem literal batendo com o vanilla.
2. Confirmar que CBOR válido continua a carregar sem regressão.
3. Teste novo cobrindo pelo menos um caso de CBOR malformado.
4. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-823-relatorio.md` com: medição antes, código vanilla/cristalino identificado, diff, medição depois, contagem de testes.

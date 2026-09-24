# Passo 1491 — completar `measure().baseline`

**Estado:** PLANEADO. Dependência: P1489; independente de P1490.

## Medição anterior à decisão

O vanilla pinado define `baseline` como distância desde o topo da medição (`lab/typst-original/crates/typst-library/src/layout/measure.rs:44-47,105-110`); a sonda `P1489-L-001` mede `rect(width: 10pt, height: 20pt)` → `20pt`. O cristalino cria apenas `width`/`height` em `01_core/src/compiler/eval/call_dispatch.rs:1684-1705` e `measure_content_real` devolve um par em `01_core/src/compiler/layout/mod.rs:3198-3248`. Proveniência do RED: relatório P1489, pin `9f2b6e87`, HEAD e hashes ali registrados; reexecutar e registrar novo estado antes da decisão.

Inferência a testar: a baseline do retângulo coincide com a borda inferior, mas **não** se pode generalizar `baseline = height`. Refutação: corpo com texto, baseline explícita, conteúdo vazio ou target distinto.

## Contrato e sequência

1. Medir bilateralmente texto, caixa com baseline controlada, corpo vazio/composto e target HTML onde a API for válida. Separar semântica da linguagem da mecânica interna de `Frame`; classificar erros ou casos não suportados sem inventar valores.
2. Auditar e atualizar, antes de L1, os L0 dos consumers afetados: `00_nucleo/prompts/compiler/stdlib/layout.md`, `00_nucleo/prompts/compiler/eval/call_dispatch.md` e `00_nucleo/prompts/compiler/layout.md`. Manter ownership 1:1 (ADR-0129); compartilhar apenas claims realmente comuns via Núcleo Tekt pinado.
3. Escrever testes RED para a chave, o valor real de baseline e controles de `width`/`height`, sombra de `measure` e diagnóstico; preservar a rejeição atual de `measure(width:)`, cuja aceitação é outro escopo/gate. Implementar a extração geométrica causal, não um literal `height`.
4. Confirmar GREEN de `P1489-L-001` e dos controles nos perfis habilitados; só então mudar seu `expected_state` de `VIOLATED` para `MATCH` com rebaseline/proveniência; testar módulo, build seriado, lint e diff-check. Registrar os hashes efetivos após resselo.

**Gate ADR-0127:** a nova chave de linguagem é correção de paridade; não criar carrier público desnecessário. Se o desenho alterar assinatura pública de `measure_content_real`, campo de entidade, trait ou fase, redigir o L0 correspondente e parar para confirmação antes do código. Em dúvida sobre a classe, parar.

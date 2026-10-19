# Prompt L0 — export HTML: p1360_ab_tests
Hash do Código: 855426c5

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/html/p1360_ab_tests.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

No snapshot anterior `03_infra/src/export/html.rs:78-139 contém fixture e teste A/B dos escapes e pretty.`
A localização é mecânica L3 (ADR-0109); preservar a morfologia HTML e
mensagens de erro observáveis (ADR-0107), sem inferir intenção do produto.

## Responsabilidade e invariantes vigentes

Possui o módulo cfg(test) export::html::p1360_ab_tests e preserva integralmente nomes, fixture, ordem de atributos e a prova de que title/phrasing/quoted > não ganham whitespace indevido. É owner separado dos demais testes pela ADR-0129.

Módulo privado descendente de export::html; helpers usados entre irmãos
empregam pub(super), sem pub(crate), API nova ou mudança de fase. Mantém as
assinaturas, algoritmos e diagnósticos do snapshot medido, incluindo
adaptação local existente a Content::Styled(styled). A extração é interna
e segue fluxo contínuo ADR-0127; cada consumer possui owner 1:1 ADR-0129.

## Aceitação

Testes funcionais existentes e novas caracterizações passam antes/depois;
comparação de assinatura/corpo detecta deriva na movimentação. Não afirmar
ganho de tempo nem paridade nova a partir desta reorganização.

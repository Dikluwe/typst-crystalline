# Prompt L0 — export HTML: tests
Hash do Código: b327f103

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/html/tests.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

No snapshot anterior `03_infra/src/export/html.rs:920-1355 contém 17 testes funcionais de modos, DOM, atributos, void, whitespace, ruby, raiz explícita, raw/title e origem do espaço.`
A localização é mecânica L3 (ADR-0109); preservar a morfologia HTML e
mensagens de erro observáveis (ADR-0107), sem inferir intenção do produto.

## Responsabilidade e invariantes vigentes

Possui o módulo cfg(test) export::html::tests, mantendo nomes e conteúdo dos testes existentes. Antes da extração acrescentar caracterizações de styled/raw/title, frame não resolvido/resolvido, tabelas com spans e pretty protegido. Asserções mecânicas da serialização são regressões deste encoder, não prova de paridade universal com vanilla. Sem rede, layout ou I/O adicional.

Módulo privado descendente de export::html; helpers usados entre irmãos
empregam pub(super), sem pub(crate), API nova ou mudança de fase. Mantém as
assinaturas, algoritmos e diagnósticos do snapshot medido, incluindo
adaptação local existente a Content::Styled(styled). A extração é interna
e segue fluxo contínuo ADR-0127; cada consumer possui owner 1:1 ADR-0129.

## Aceitação

Testes funcionais existentes e novas caracterizações passam antes/depois;
comparação de assinatura/corpo detecta deriva na movimentação. Não afirmar
ganho de tempo nem paridade nova a partir desta reorganização.

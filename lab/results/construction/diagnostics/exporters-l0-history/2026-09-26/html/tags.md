# Prompt L0 — export HTML: tags
Hash do Código: aaf9a4d6

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/html/tags.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

No snapshot anterior `03_infra/src/export/html.rs:14-68 contém tabelas void, raw, phrasing, block e replaced; :514-516 consulta agrupamento.`
A localização é mecânica L3 (ADR-0109); preservar a morfologia HTML e
mensagens de erro observáveis (ADR-0107), sem inferir intenção do produto.

## Responsabilidade e invariantes vigentes

Possui as cinco tabelas estáticas e should_group_into_paragraph. Manter exatamente os conjuntos atuais, incluindo summary block, script/style raw e custom tags não-groupable por default. A classificação é partilhada por consumidores internos sem duplicar tabelas, CSS/display ou estado global. Nenhuma alteração de classificação ou parser HTML neste lote.

Módulo privado descendente de export::html; helpers usados entre irmãos
empregam pub(super), sem pub(crate), API nova ou mudança de fase. Mantém as
assinaturas, algoritmos e diagnósticos do snapshot medido, incluindo
adaptação local existente a Content::Styled(styled). A extração é interna
e segue fluxo contínuo ADR-0127; cada consumer possui owner 1:1 ADR-0129.

## Aceitação

Testes funcionais existentes e novas caracterizações passam antes/depois;
comparação de assinatura/corpo detecta deriva na movimentação. Não afirmar
ganho de tempo nem paridade nova a partir desta reorganização.

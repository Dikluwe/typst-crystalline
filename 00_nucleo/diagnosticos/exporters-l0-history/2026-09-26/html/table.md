# Prompt L0 — export HTML: table
Hash do Código: e0d03431

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/html/table.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

No snapshot anterior `03_infra/src/export/html.rs:631-704 achata células e emite thead/tbody sem passar por PagedDocument.`
A localização é mecânica L3 (ADR-0109); preservar a morfologia HTML e
mensagens de erro observáveis (ADR-0107), sem inferir intenção do produto.

## Responsabilidade e invariantes vigentes

Possui html_table e seus helpers locais flatten/rows. Columns usa len().max(1); header presente emite thead com th; children emitem tbody com td. Sequence/TableHeader/TableFooter achatam seus bodies; células usam colspan/rowspan com default 1 e só emitem attrs acima de 1. Conservar agrupamento por chunks, inclusive a limitação atual de não corrigir ocupação de spans neste lote. Toda célula delega ao inline com o mesmo modo/resolvedor/StyleChain. Não inventar footer separado ou suporte novo de tabelas.

Módulo privado descendente de export::html; helpers usados entre irmãos
empregam pub(super), sem pub(crate), API nova ou mudança de fase. Mantém as
assinaturas, algoritmos e diagnósticos do snapshot medido, incluindo
adaptação local existente a Content::Styled(styled). A extração é interna
e segue fluxo contínuo ADR-0127; cada consumer possui owner 1:1 ADR-0129.

## Aceitação

Testes funcionais existentes e novas caracterizações passam antes/depois;
comparação de assinatura/corpo detecta deriva na movimentação. Não afirmar
ganho de tempo nem paridade nova a partir desta reorganização.

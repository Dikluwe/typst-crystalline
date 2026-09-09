# P1319 — revisão do delta local e do RED

Revisor `/root/p1319_review`, A/B sem atestação técnica de isolamento; não
editou produto, L0, testes locais ou oráculos julgados.

## Entradas confirmadas

- Manifesto `p1319-test-delta.json`, SHA-256
  `5856efb9c7f04ccf099095a72f266d8535090abb371f8216188f88e3659a5545`.
- Fonte RED, SHA-256
  `9c647415dac61f26ac5a729bb9f5cfa80867bb85de83b76288d40e87f3a9152c`.
- L0 após resselo, SHA-256 raw
  `5c33854b87f1af32a6bee7f62de0aa27f73ee6496f27e5662d253db2349aff75`;
  apenas metadata alterada desde a revisão normativa anterior.
- Recibo `p1319-unit-red.json`, SHA-256
  `adbfe60839f87ac4087d15848755d8a2dc4de3da375a4b9dd1500032fe2c5e7a`.

Todos os hashes acima foram recalculados. Inspeção direta do diff confirma
que os dois testes antigos alteram somente a expectativa da mensagem Path
inválida, conservando controles puros/UTF-8 válido e spans detached. Os dois
testes novos exercitam causa, caminho normalizado, Project/Package, primeiro
positional, origem explícita/detached, excesso, resolução/leitura única e
opção inválida sem leitura. Nenhuma asserção anterior foi removida.

Comparação independente via `git show HEAD:01_core/src/compiler/stdlib/loading.rs`
e leitura da fonte atual confirmou prefixo antes de `#[cfg(test)]` idêntico
ao HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, salvo `@prompt-hash`.
O manifesto do autor não foi a única evidência desse ponto.

## RED observado

Comando no recibo: `cargo test -p typst-core --release
compiler::stdlib::loading::tests --lib`, target
`/tmp/p1319-target.VqXtmj`, cwd do repositório. O recibo conserva horário,
HEAD, diff/stat e hashes de fonte/L0 antes/depois, iguais aos acima.

A compilação foi concluída e os testes executaram: exit 101, 65 passaram,
4 falharam, nenhum ignorado. As quatro falhas são os dois testes alterados e
os dois novos. Cada uma compara a mensagem baseline sem caminho/posição à
mensagem P1319 com `in caminho:L:C`; não há falha de build, fixture ausente
ou erro de outro estrato apresentado como RED. Os controles anteriores
passaram na mesma execução.

O teste em loop interrompe no primeiro assert falho: RED comprova falta do
comportamento, não afirma que todas as combinações Package/origem/posição
tenham sido executadas até o final. GREEN deverá percorrer o conjunto todo.

Limitação de instrumentação comunicada: `forbid_sources` veta World::source,
mas include_path usa o default que retorna Err. Uma chamada include_path
ignorada não seria capturada por esse flag; a proibição explícita no L0
deve também ser verificada no diff do candidato. Isso não invalida o RED.

Veredito desta fase: delta e RED adequados. Parecer pré-patch definitivo
ainda depende do freeze A/B final e de sua revisão. Sem selo ou promessa
de paridade geral.

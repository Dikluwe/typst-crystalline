# P1324 — inspeção independente do candidato

Em 2026-09-09T00:23:01.207Z, working tree de HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, recibo
`p1324-review-candidate-audit.json` conserva diff/stat, hashes, manifesto e
lista de recibos. O corpo L0 congelado permanece idêntico; os seis arquivos
dirty prévios e o Núcleo também. Header efetivo `740a39f5` é correto.
Recíproco calculado independentemente: `c4567498`; ainda constava
`5a601982` naquele instante e o root foi informado para correção.

O delta produtivo observado cabe no owner: helper privado de nome aceita
ambas as categorias nativas com ou sem namespace, atravessa With e conserva
projeção pública; seletor AST usa field.span para nativas; lookup Some
presente retorna o valor clonado, ausência usa mensagem nominal. Closure,
Plugin e Element continuam excluídos por variante, sem identificação por nome.
Os demais braços permanecem iguais. Nenhuma entidade, API, núcleo, default,
fase ou outro consumer foi alterado. Não há achado funcional no delta.

O snippet independente bruto está preservado. Em 00:24:19.267Z executei
`rustfmt --edition 2021 --emit stdout 00_nucleo/diagnosticos/p1324-ab-unit-snippet.rs`
e comparei a saída (excluído só o cabeçalho de caminho emitido pelo rustfmt)
com o owner: conteúdo formatado inteiro presente, SHA-256
`bd351888c15a75cd043b92edd4a6517badde68264024933fc3ef468864798a60`.
Portanto a mudança no bloco de testes é formatação reprodutível, não nova
expectativa; o módulo bruto permanece pinado por seu SHA original. Uma
tentativa anterior de formatar stdin pelo subprocesso Node ficou pendente e
foi interrompida (exit 130); não alterou arquivos nem embasa este resultado.

O unit-green anterior à formatação executou os cinco testes com exit 0;
fechamento aguarda os gates sobre o source formatado, A/B V3 e metadado
recíproco corrigido. Limitação de isolamento e ordem temporal V3 permanecem
as documentadas no parecer de reabertura.

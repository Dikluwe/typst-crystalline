# P1326 — auditoria anterior ao candidato

Revisor `/root/p1326_review`, contexto fresh. Regime A/B, executado sem
atestação técnica de isolamento e sem selo de refinamento. Escrita autorizada
somente em `00_nucleo/diagnosticos/p1326-review-*`; produto, L0 e testes são
somente leitura. SKILL Tekt e ambas referências lidas; CLAUDE raiz/core e
ADR-0127 lidos. Nenhum acesso a materialization/context.

## Entradas e medição anterior à classificação

Baseline `p1326-baseline.json`, SHA-256
`3198d97b15c02085af158e88e44c4025e4f76b29654e7309c2ca21fdb2a41f79`,
UTC `2026-09-09T01:23:32.345952+00:00`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093` e working tree não commitado;
status/diff/stat integrais no recibo. Binário baseline
`3511b08aa89d088e908dd239d8942f1eeea1978d31140ef9510e81de06023dab`,
vanilla ratificado `a51e02804`, binário
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

L0 field_access lido integralmente: SHA-256
`71b7492efffd851fbd404daabecef38dd2a3eaac8e23195b129368af2816354d`;
consumer `2d32b6e5ab691b19dde2213fd8df32cc9e6cfc1adae4477fe96d7cec976cda6a`.
L0 Func lido integralmente:
`909cd5a85e4968d42a381b33d343e50a7a37e875817d581a6a2ebe9076057a1a`;
fonte Func `9bcd2e2b0c79cae5c2478b55d9d902f78983f9119d634a04a846d93210307bde`.
L0 eval/tests lido integralmente:
`8205c6fc1f5ae0a5546b8ea0754913343134cf02fbe31565c6122324b1cb58e8`;
consumer `4d64677e7a0963e5c45b8ed4e284ef985549650b71dbd2f48e365fa1dafb7aeb`.
Núcleos content-snapshot e eval/core lidos. Hashes acima identificam leitura,
não provam isolamento nem substituem gates V15/V26/revalidação de linhagem.

`lab/typst-original/crates/typst-library/src/foundations/func.rs:280–308`
mostra Closure sem scope e erro `cannot access fields on user-defined functions`;
With delega scope ao original. `typst-eval/src/code.rs:347–366` fornece
field.span(). `01_core/src/entities/func.rs:24–43,273–276,355–365` já expõe
internamente categoria Closure/With e namespace. O consumer
`field_access.rs:502–511,651–663` seleciona span total de closures e texto
genérico. O baseline mediu a diferença em closure nomeada/anônima, alias,
With, multilinha e callee com argumento benigno.

## Suficiência e classe

Owner suficiente: exclusivamente `compiler/eval/bindings/field_access.md`
para `01_core/src/compiler/eval/bindings/field_access.rs`, incluindo os testes
embutidos. Helper privado de categoria atravessando With pode usar somente
os dados existentes. Mensagem/range são observáveis da língua. Classificação
proposta aceita: `ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY`, limitada a Closure
subjacente, sem contrato público, default, fase ou compatibilidade novos.
Necessidade de outro carrier, assinatura ou reordenação reabre a classe.

Plugin e Element de usuário conservam baseline; scope ausente ou nome
presente não bastam para classificá-los como Closure. Tratar o diagnóstico
vanilla de plugin como autorização por analogia ampliaria o escopo sem
medição. Guard nominal `text.size/lang`, warnings math/sym e ordem de
argumentos de callee ficam fora. O caso com panic nos argumentos é residual
explícito, não paridade fechada nem autorização para modificar call_dispatch.

## Sucessões necessárias antes de C

- L0 field_access P1311, linhas 230–234, protege Closure; P1324, linhas
  305–309, repete a proteção. Suceder somente Closure e With de Closure.
- Suceder pontualmente as proteções genéricas de outros targets de P1293,
  P1301, P1303 e P1306, além da preservação dos demais tipos de P1325.
- `p1324_named_closure_and_user_element_are_not_native`, linhas 183–207,
  agrupa Closure e Element sob mensagem antiga. Separar expectativas,
  preservando Element, profundidades With e âncoras fornecidas.
- `p1311_named_non_native_categories_preserved`, linhas 359–407, agrupa
  Closure, Element e Plugin. Atualizar somente Closure simples/With;
  conservar controles Element/Plugin.

Busca literal `cannot access fields on type function|user-defined functions`
em todo `01_core/src` encontrou expectativas apenas nesses grupos e no
controle separado Plugin P1324. Busca de closure/campos e leitura do L0
eval/tests não encontrou expectativa de closure missing-field naquele owner.
Testes P1325 protegem Dict/Content/Float/LocatedContent, sem sucessão necessária.
Não há fundamento para editar o owner test-only eval/tests neste recorte.

Esta auditoria antecede novo L0 e candidato; ainda não emite GREEN, aprovação
de gates funcionais, atestação de protocolo completo ou paridade geral.

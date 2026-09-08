# P1314 — revisão de fronteiras antes do contrato

Revisor `/root/p1314_review`, 2026-09-08T12:25:43Z. Regime A/B proporcional,
executado sem atestação de isolamento técnico. Esta revisão recebeu o escopo
candidato e fontes anteriores; não recebeu implementação P1314. A capacidade
de escrita do papel está limitada por instrução a `p1314-review-*` em
diagnósticos. O filesystem é compartilhado. Não é selo de refinamento.

## Medição anterior à classificação

HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree não commitado.
`git diff HEAD --stat` observado nesta leitura:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  62 ++-
 00_nucleo/prompts/compiler/stdlib/loading.md       | 209 ++++++++-
 01_core/src/compiler/eval/bindings/field_access.rs | 175 ++++++-
 01_core/src/compiler/stdlib/loading.rs             | 515 ++++++++++++++++++++-
 4 files changed, 940 insertions(+), 21 deletions(-)
```

Índice sem diff. SHA-256 do owner anterior:
`a8ef0db2cd9531d6f86aa5c346c3860495a002a8244b4e9594cc9fd6f15b3468`;
L0 `00_nucleo/prompts/compiler/stdlib/loading.md` lido integralmente:
`103a2035a2c4c534e3e905bf168ec50d06dc6c2fc484cfc362a74b22f7bf9c5c`.

Na fonte ratificada em `lab/typst-original/crates/typst-library/src/`:

- `loading/csv.rs:27–45` declara fonte, delimiter e row-type nessa ordem;
  `csv.rs:103–110` exige delimitador char ASCII;
  `csv.rs:120–134` admite somente os tipos array/dictionary para row-type.
- `foundations/args.rs:218–235` percorre todas as ocorrências do named,
  converte cada valor com `at(value.span)` e retém a última somente depois
  de conversões bem-sucedidas. O comentário de `222–223` declara a intenção
  de remover todas e usar a última; rejeição de conversão anterior vem do
  `?` em `230`, não de uma hipótese sobre o mapa cristalino.
- `foundations/str.rs:828–837` converte Str a char exigindo exatamente um
  caractere; `foundations/value.rs:632–637` admite Symbol no cast Str.
  Logo Symbol não pode ser apresentado como igualdade geral vanilla se a
  implementação continuar rejeitando a classe, como no baseline.

Checagem extra da fonte para evitar o enquadramento confortável “basta seguir
a ordem da chamada”: `lab/typst-original/crates/typst-macros/src/func.rs:376–393`
gera parsers na ordem dos parâmetros, e só depois chama `finish`;
`func.rs:463–480` usa `args.named` para named. Assim delimiter precede row-type
mesmo se row-type foi escrito primeiro. Named desconhecido vence antes no
cristalino; é dívida preservada, não regra paritária vanilla.

No owner cristalino anterior, `loading.rs:1226–1231` rejeita named desconhecido
antes do cast da fonte. `1233–1271` valida apenas a última view `args.named`,
emitindo `err` detached; delimiter já precede row-type. `1273–1277` obtém os
bytes somente depois dessas validações. `entities/args.rs:53–80` conserva a
sequência causal e fornece fallback sintético detached; não falta campo de
domínio para o recorte proposto.

O L0 anterior P1313 preserva opções inválidas/duplicadas e demais validações;
o relatório e a revisão final P1313 identificam isso como limite deliberado.
Essa preservação não é autorização causal para o próximo código: deve ser
substituída expressamente no recorte P1314 antes de materializar.

## Classificação e fronteiras

**`CONTINUOUS_SCOPED`**, condicionado ao contrato congelado abaixo e à
medição bilateral independente. ADR-0127 §2, fluxo contínuo item 3, cobre
convergência do diagnóstico e validação de opções para a referência, desde
que não haja novo tipo admitido, assinatura pública, entidade, trait, fase,
modo ou efeito de I/O. A mudança de erro observável é superfície de linguagem
diagnóstica (ADR-0107/0108); não se exige copiar estruturas ou algoritmo Rust.

O contrato precisa delimitar explicitamente:

- Todas as ocorrências de cada opção são validadas em ordem causal; a última
  válida vence somente se nenhuma anterior falhar. Delimiter inteiro precede
  row-type inteiro, conservando a precedência já existente entre opções.
- Erro de opção aponta ao value_span da ocorrência inválida, com origem
  detached preservada, inclusive fallback Args sintético; não usar chamada,
  named inteiro ou último span para encobrir perda de origem.
- Casts admissíveis atuais permanecem: delimiter Str e row-type Type de
  array/dictionary. Symbol exige controle normativo separado; não habilitar
  coerção nova como consequência incidental de helper.
- P1313 Source, named desconhecido primeiro, missing/excesso, parsing, leitura,
  outros loaders/encoders e outros owners permanecem protegidos. Opções
  inválidas devem continuar falhando antes de leitura de Path/Str ou decode.

É inferência que o owner e carrier atuais bastam. Evidência de perda causal
anterior ao consumer, necessidade de editar Args/dispatch ou mudança de cast
admissível a refuta e exige reabrir o escopo/gate. Nenhum contrato ou teste
P1314 é aprovado por este parecer preliminar; o freeze e os resultados ainda
precisam de revisão. Não há achado que exija parar antecipadamente no escopo
estrito descrito, nem autorização para ampliar esse escopo.

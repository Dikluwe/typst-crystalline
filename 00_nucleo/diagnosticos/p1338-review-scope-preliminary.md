# P1338 — revisão preliminar, sem autorização de C

Contexto de revisão P1337 retido explicitamente por limite de agentes. Revisor não é implementador nem autor de testes/ataques. Skill `tekt-materializacao-segregada` e ambas referências lidas integralmente; CLAUDE raiz/core, ADRs 0107/0108/0127/0129/0130 e L0 inteiro `compiler/eval/bindings/field_access.md` relidos. Nenhum acesso a materialization/context. Regime A/B de autoria separada, sem atestação técnica de isolamento e sem selo de refinamento.

## Evidência anterior ao parecer

`p1338-review-baseline.json` e seu auditor read-only verificam baseline 6d85aece… contra closure 04d5c3cc… e postclosure ce6873d6… de P1337. Inventário produtivo do baseline coincide integralmente com closure; snapshots source/L0 são os bytes anteriores. Manifesto inicial 8c51d637… conserva o corpo produtivo e todos os testes, mudando somente header. Foram rehashados 5203 históricos, 499 temporários do baseline e os conjuntos antecedentes do fechamento, sem divergência. Os 17 casos/34 processos de `p1338-measurement.json` têm fontes, canais UTF-8/base64 e binários pinados íntegros. Nenhum gate de produto repetido nesta revisão.

Fonte: `01_core/src/compiler/eval/bindings/field_access.rs:468` devolve cedo o resultado do pré-despacho de coleções; `:532` seleciona âncora e `:671` possui o ramo Array. Lookup puro len/first/last devolve comprimento/clone/None, enquanto AST pode parar antes com erro de array vazio. A sentinela P1337 em `:1739` protege justamente o diagnóstico a ser sucedido. Vanilla `lab/typst-original/crates/typst-eval/src/code.rs:347`, `typst-library/src/foundations/value.rs:157` e `foundations/fields.rs:14` fundamentam mensagem sem nome arbitrário e âncora field-only para o erro ausente. A medição separa explicitamente as divergências de pré-despacho, Type::Array e Length.

## Parecer de classe e fronteira

Correção interna de paridade diagnóstica ADR0127 em fluxo contínuo é sustentada pelo recorte. Não muda API pública, disponibilidade de campos/métodos, default ou fase. L0 P1338 sucede expressamente apenas o erro Array ausente que alcança o lookup; a preservação indiscriminada de Array nas cláusulas antigas não anula essa sucessão específica. Lookup puro conserva span recebido; AST conserva pré-despacho e usa origem do identificador no caminho contratado. Bool/None/Auto, Int/Str, Content/LocatedContent, funções, módulos, gates/features e dívida de outros tipos permanecem protegidos.

Testes antecedentes só podem migrar nome e expectativas da sentinela Array expressamente autorizada, preservando fonte do caso e comparadores. Uma correção honesta do comentário descritivo dessa sentinela, caso necessária, deve ser explicitamente justificada no sucessor; nenhum outro teste antigo pode mudar. Testes/ataques precisam discriminar mensagem, âncora, span puro e fronteiras funcionais, com RED de assertions e mutantes compilados realmente executados. Falha anterior ao lookup não vale como convergência nem como witness desse erro.

Ponto documental antes dos freezes independentes: a referência herdada `bindings/access.md` no Contexto do L0 não é canônica sob ADR0130. Recomendada correção exclusiva para `compiler/eval/bindings/access.md` em sucessor explícito do manifesto. As outras referências Markdown do owner não têm o mesmo defeito. Esta revisão não altera inputs nem autoriza expansão normativa.

Status: **sem GO para candidato** até sucessão documental estabilizada, freeze dos testes e ataques e RED genuíno independente. Não há alegação de paridade geral Array.

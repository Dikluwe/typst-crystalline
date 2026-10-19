# FG1 e RM1 — passos pendentes

Estado: incompleto, investigação; não são Prompts L0 e não autorizam código.
Proveniência inicial: before.json.gz, 130 pares em before-expanded-measurement.json.gz,
contract-dependencies-before.json (busca lexical, não escopo semântico completo).

FG1: concluir a identidade de figure kind e sua síntese. O contrato público atual
Option<String> não distingue função de elemento de string homônima; a inferência
é parcial e anterior à síntese. Ler todos os owners do inventário, medir render,
contadores, seletores, reflexão e erro antes de decidir carrier/fase. Produzir
L0s candidatos, lista exata de migrações, testes e manifesto revisável; pedir gate
ADR-0127 antes de implementar mudanças públicas/ordem. O V16 de inferência fica
aberto. Não converter funções para nomes nem classificar containers como folhas
neutras para silenciar a regra. A validação interna desta etapa não fecha FG1.

RM1: concluir transporte relativo e política de cast em h/v/box e outros consumers
que usam extract_length. Spacing e dimensões de Box têm carriers públicos de
Length, e lados/cantos/outros adapters possuem contratos próprios. Ler os owners,
medir tamanho/posição e erros em cada família antes de selecionar migrações;
preparar L0s/testes/manifesto e gate de API. Não truncar como semântica final nem
substituir os percentuais aceitos por erro. Preservar o contrato relativo já
materializado de Block. Dois V16 de cobertura fechada corrigidos nesta etapa não
provam RM1 completo. Priorizar transporte de linguagem, não igualdade Rust/bytes.

Nenhuma API será alterada com este diagnóstico. RG1 continua independente e a
aprovação TS1 não autoriza estes contratos novos.

SY1: completar o carrier SymbolElem e sua reflexão/morfologia, sem trocar a
preservação textual desta etapa por alegação de paridade completa. Ler os owners
de Content/travessia/render antes de proposta pública; depende do gate ADR-0127.

FG1 inclui presença de caption: o carrier None colapsa omissão e none explícito;
o getter desta etapa expõe somente caption Some existente. Não inventar presença.

RM1 deve distinguir perda de informação de extensão intencional: Float/Int em pt
constam de contratos históricos, inclusive Block ratificado. Divergir do pin não
prova defeito; medir, consultar a decisão vigente e obter nova decisão antes de
remover compatibilidade. Truncar percentual é uma limitação de transporte já
medida, que não se resolve trocando o valor por erro.

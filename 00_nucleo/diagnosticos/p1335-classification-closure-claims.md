# P1335 — alegações históricas a revalidar

Entradas pinadas em `p1335-classification-historical-inputs.json`; todos os
relatórios P1323–P1334 foram lidos integralmente. Esta tabela descreve seus
recortes históricos, sem convertê-los em medições/fechamentos P1335. A fonte
de cada linha é `pNNNN-final-report.md`; a identidade temporal é seu closure.

| Antecedente | Efeito alegado | Testemunhas atuais necessárias | Fronteira que não recebe crédito |
| --- | --- | --- | --- |
| P1323 | Warning HTML completo com três hints e parágrafo | Transversal compile/legacy HTML, sucesso/erro; feature ligada/desligada; canais integrais | DOM igual não fecha warnings; serialização exclusiva cristalina não é igualdade vanilla; render/PDF global fora |
| P1324 | Nativa Some ausente nomeia função e ancora field | Suite p1324: json/yaml/toml/cbor/assert/table/grid; alias/With/traces/Unicode; sucesso de campos | json.nope(panic(...)) continua ordem de argumento distinta; Content após table.cell é outra superfície |
| P1325 | Dict, Content raw e Float diretos usam field-only | Suite p1325: direct/alias/Unicode/multiline; conteúdo strong e erro Float | [x].text continua ausente; método/callee e LocatedContent fora; mudar só âncora não é fechar lookup |
| P1326 | Closure/With recebe mensagem específica e field-only | Suite p1326: named/anonymous/with/nested-with/callee; positivos de chamada | f.nope(panic(...)) é residual anterior; Element/Plugin não cobertos por CLI |
| P1327 | Bare Ident import avisa; eval publica erros antes de warnings | Suite p1327 e p1327r2 completa, com ordinary/math/warning.typ, alias, erro posterior, warnings múltiplos | Rename redundante, import type/trace e raw serializer permanecem dívidas; target HTML não é profile eval |
| P1328 | Rejeição Content/LocatedContent por abs publica união e origem do valor | Corpus cumulativo P1334: content direto/math/With/spread/arguments | Conteúdo math não vira número; isolamento de LocatedContent sintético exige teste nativo |
| P1329 | abs aceita length/angle/ratio/fraction; rejeita length misto no valor | P1334: grandezas absoluta/em/angular/ratio/fraction, misto mesmo/oposto sinal, With/spread | NaN dimensional e Float×Fraction são construção anterior; origin detached sintético não prova produção pública |
| P1330 | abs do i64 mínimo construído por subtração erra em vez de saturar | P1334: overflow e vizinhos; Int/Float/Decimal; With/Args | Literal direto mínimo tem dívida de parser anterior a abs |
| P1331 | Outros tipos rejeitados por abs recebem união, nome longo e origem | P1334: string/bool/symbol/path válido e demais tipos públicos | path() sem argumento falha antes; location tipo não é instância; produção de Location não comprovada pelo fallback |
| P1332 | Nome intrínseco abs corrige traces; identidade entre rotas preservada | P1334: With/Args, alias/import, igualdade/distinção/repr; efeitos gradient/show/where | Mudar nome em diagnóstico não fecha texto/âncora/operação externa |
| P1333 | Primeiro valor inválido vence sobras de abs | P1334: rejected/overflow/mixed + sobras/named/With/spread | Panic ao avaliar expressão de argumento acontece antes da nativa; não atribuir a guards |
| P1334 | Missing/value named+hint/primeira sobra na ordem conjunta com origem | Corpus público P1334 e suplemento import entre arquivos; ausência/With/Args/alias/valores válidos | Resolver abs importado em math é causa anterior; Some incoerente sintético não legitima fallback nem fecha paridade |

## Atualizações normativas que impedem comparar cegamente saídas históricas

P1325 sucedeu apenas spans de Dict/raw/Float; P1326 sucedeu Closure;
P1327 acrescentou warnings de bare imports e a ordem de eval;
P1329/P1330/P1331/P1332/P1333/P1334 sucederam em sequência observáveis abs.
Assim, preservação literal contra P1324/P1325 isoladamente pode falhar por
ganho autorizado posterior. A transição deve nomear o sucessor efetivo,
comparar vanilla atual e distinguir ganho integral de alteração parcial.
Outras classes preservadas por esses contratos continuam abertas até medição.

## Dívida certificatória

Os relatórios descrevem A/B sem atestação técnica de isolamento e sem selo
de refinamento. Testes e ataques em cópias do auditor não são mutantes do
produto. O ledger histórico de certificação P1322 será pinado/referenciado
separadamente, com cada família e escopo, sem transformar testes funcionais
P1323–P1334 em quitação adversarial não executada. Nenhuma contagem histórica
de famílias será apresentada como medição nova sem conferir suas entradas.

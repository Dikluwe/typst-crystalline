# P1331 — revisão pré-C do congelamento A/B

Revisor `/root/p1331_review`. A/B sem atestação técnica de isolamento,
sem refinamento. Li os testes somente após receber o freeze final; não
editei nenhum artefato julgado. C ainda não foi recebido.

Entradas: norma L0 `ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a`,
manifesto `837bb832ce13f0686a438f672c6fe001224738a15b1a369efcc6f7533b7db141`,
freeze `207ff2040a913c2da429325b566e0923c98215d292993c30f95e51020591bd8a`,
integração `1b2f2b4263aed9dfeff7de426e0a515b4e15100d78e9ef77ea88c79aef374bdb`.
Esses documentos identificam os hashes dos snippets e oráculos e o estado
completo da working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`.

`node 00_nucleo/diagnosticos/p1331-review-pre-c-audit.cjs`, executado em
`2026-09-09T13:50:11.187Z`, terminou exit 0. Reconstruí o owner esperado a
partir do baseline com substituição exclusiva do sucessor P1328 e inclusão
do snippet novo. A igualdade integral, retirando somente metadata de
linhagem, confirmou runtime baseline exato, P1329/P1330 intactos e hashes
congelados/históricos preservados. Os inventários integrais before/after
da integração são a proveniência dessa medição.

O diff do sucessor muda somente str/symbol: mensagem longa, span do argumento
e retirada do trace redundante direto. A ramificação de sqrt retém suas
asserções anteriores. Nenhuma aceitação numérica/dimensional ou overflow
foi removida. A auditoria CLI independente confirma 332 células históricas,
oito migradas exclusivamente string/symbol, 324 preservadas literalmente e
172 novas, total 504. As expressões e perfis estão nos JSONs congelados;
os números não são inferidos de resumos de teste.

Os sete testes novos avaliam famílias rejeitadas, tipos construídos e
diagnósticos completos. Há distinção explícita de Location nativa e Type
location; Relative zero e Path têm sondas concretas anteriores a C.
Primeira ocorrência posicional, metadados named, ocorrência posterior
enganosa, origem ausente/vazia/named-only/detached são discriminados. Rotas
alias/With/spread/arguments, UTF-8, linhas e warning são cobertas. Guards
named/aridade permanecem sentinelas de dívida; os tipos já tratados têm
controles de espécie/valor e erros.

A correção pré-C da hipótese de string em math é sustentada pela captura:
string explícita alcança o fallback como string. Os controles math históricos
de conteúdo permanecem. O nome inicial do caso no JSON não muda seu valor
observado nem é usado como oráculo. Traces externos calc.abs/abs têm literais
congelados antes C; o comparador não normaliza saídas candidatas.

Scripts de integração/linhagem/corpus foram revistos: a integração exige
preimagem completa, e corpus confere células, não só contagens. O fechamento
automatizado exige saídas CLI completas e inventários estáveis, mas usa
presença textual de PASS nos pareceres; por isso o veredito final deste
revisor deve conferir também a substância e os hashes das evidências.

Veredito: freeze A/B e integração aprovados para avaliar RED. Este documento
não autoriza C antes do RED compilado e não fecha P1331.

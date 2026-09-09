# P1337 — segundo achado pré-C: formatação mecânica do módulo

Destinatário: autor independente dos testes; operador encaminha sem ler.
Não editar Rust durante o RED-r1 corrente. Nenhum candidato existe.

O check read-only `rustfmt --check --edition 2021
01_core/src/compiler/eval/bindings/field_access.rs` retornou exit 1, restrito ao
módulo novo: quebra da lista de fields e chamada de lookup, lista de pares
literal/nome, pares AST multilinha e chamada de eval dos positivos. Nenhuma
diferença no produto ou em testes anteriores foi pedida pelo formatter.

Para impedir formatação pós-C de testes congelados, publicar sucessor R2
mecanicamente formatado antes de C, mantendo todos os valores literais,
assertions, ordem e testes R1. Usar a configuração rustfmt do repositório;
provar que a diferença R1→R2 é somente whitespace externo aos literais.
Preservar R0/R1, todos os oráculos CLI e a sentinela adicionada. Avisar o
adversário para atualizar somente o vínculo de módulo/freeze. Não ampliar
famílias, norma ou critério; não relaxar o gate fmt.

Este achado é distinto da lacuna de discriminação já sanada no R1. A melhoria
R1 foi real e não constitui duas revisões sem ganho na mesma causa. R2 é
correção mecânica pré-C; novo recibo RED sobre os bytes efetivos ainda é
necessário, com os RED anteriores preservados como históricos genuínos.

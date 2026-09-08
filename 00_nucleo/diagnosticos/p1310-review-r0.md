# P1310 — revisão independente do candidato e do oracle R1

Revisor: `/root/p1310_review`; regime A/B, executado sem atestação de
isolamento técnico. O revisor não editou solução nem testes. Baseline
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, recibo P1310 SHA-256
`3a19c3b842c5bdcc2d4e6df3ea8778a407a9843b2fc5ec183f3b07620c25d65f`.

## Achado bloqueante no oracle R1

`p1310-ab-freeze-r1.py:26-39` exige paridade vanilla para `named-prefix`,
`with-named-prefix` e `args-named-prefix`, embora o L0 P1310 exclua alteração
da ordem de validação de named. A medição prévia conservada em
`p1310-ab-freeze-r1-measurement.json` demonstra, por exemplo,
`json(nope: true, 42)` no perfil default: vanilla acusa o cast de `42`, mas
o baseline acusa `argumento nomeado inesperado em json(): 'nope'` detached,
com trace da chamada. `loading.rs:1024-1029` rejeita named antes do
`resolve_data`; a macro e `native_cbor` conservam essa ordem. O candidato
altera somente o ramo inválido de `resolve_data`, portanto não pode cumprir
essa expectativa sem violar o L0 vigente.

O problema pertence ao oracle, não autoriza corrigir a ordem do produto.
Essas testemunhas devem ser classificadas conforme a exclusão já normativa,
preservando a comparação bilateral original e o candidato já existente.
Qualquer revisão posterior ao candidato deve ser registrada como tal, com
invalidação explícita da expectativa anterior; não pode receber a alegação
de congelamento integral anterior à implementação. `first-invalid`, que não
tem named, não apresenta essa incompatibilidade.

## Revisão do diff

O diff funcional observado está limitado ao ramo `Some(other)` de
`resolve_data`, mais testes locais e metadata de linhagem. Usa o formatter
canônico total de Value e a primeira ocorrência posicional, conservando
`value_span` detached. Não muda assinatura, tipo admitido, parser, I/O nem
ordem de validação. Sem achado funcional adicional confirmado nesta revisão.

Symbol foi explicitamente excluído da alegação de paridade antes do candidato:
vanilla converte Symbol em Str em `foundations/value.rs:632-637`; o produto
conserva rejeição com o diagnóstico comum especificado. Essa fronteira
documentada não constitui falha deste recorte.

Veredito provisório: **bloqueado por expectativa R1 incompatível com L0**.
Gates finais ainda pendentes; nenhum fechamento ou equivalência geral atestados.

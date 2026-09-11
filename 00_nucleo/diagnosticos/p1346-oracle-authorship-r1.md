# P1346 — autoria independente do oráculo de transporte R1

- Autoridade: `/root/p1346_oracle`.
- Regime: `executado sem atestacao de isolamento`.
- Entradas: passo, manifesto, freeze, baseline, topologia, relocação e contrato P1346 nos hashes do recibo.
- Saídas causais já fechadas: corpus canônico e checker; nenhum candidato produtivo foi lido ou executado.
- Transporte: preflight manual antecede `argparse`; o probe é compilado explicitamente do `.rs.txt`, copiado para `memfd`, selado e executado exclusivamente por `/proc/self/fd/N`.
- DAG: este documento e o recibo de autoria não consomem caller nem delivery receipt. O caller posterior fixa checker, recibo e authoring root; o delivery posterior fixa o caller.
- Orçamento: `full=0`; somente a execução focal é permitida antes do adversário.
- Estado: artefato de autoria, não verificado e não selado.

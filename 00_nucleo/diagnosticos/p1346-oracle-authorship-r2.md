# P1346 — revisão focal final do oráculo R2

Hipótese única: substituir resultados herdados copiados por execução real em subprocesso, executar T08–T11 em subprocessos próprios e fechar o probe sobre o mesmo descritor `memfd` verificado no filho imediatamente antes de `execve`.

- Autoridade: `/root/p1346_oracle`.
- Regime: `executado sem atestacao de isolamento`.
- Revisão focal restante após R2: zero.
- R1 e adversarial R1 permanecem imutáveis e pinados.
- O worker compatível executa os 122 casos P1345; o checker substitui somente A21/R2A17 pelo preflight P1346.
- `MFD_ALLOW_SEALING` e `F_SEAL_WRITE|F_SEAL_GROW|F_SEAL_SHRINK|F_SEAL_SEAL` antecedem o fork. O filho rehasha e confirma a identidade do mesmo FD antes de executar `/proc/self/fd/N`.
- Falhas de criação, sealing, fork, verificação ou exec são `Violated/PROBE_AUTHORITY`.
- `full=0`; nenhum candidato, L0, passo, contrato ou teste foi lido ou alterado.
- Este documento não sela nem consome caller/delivery.

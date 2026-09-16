# P1425 R1 — relatório independente

**Veredito: PASS.** Executado sem atestação de isolamento.

O candidato preserva a cadeia R1 e fecha o fragmento contratado de `math.limits`. Os hashes do manifesto (`4545e3b9…`), contrato (`1cc4feb…`), oráculos (`e2158b8d…`), mutantes (`aac01398…`), auditoria de allowlist (`4de22230…`), selo (`4a2c4ad5…`) e oito artefactos selados de teste coincidem com as entradas congeladas. Os 15 caminhos candidatos estão dentro da allowlist: sete consumers produtivos, sete owners L0 e o reparo mecânico autorizado dos seis callsites P992. Os owners mudaram somente em `Hash do Código`; `crystalline-lint .` validou as sete relações 1:1 e manteve V5, V15 e V26 livres.

As suítes seladas passaram nas três ordens, sempre em processos frescos, com vetor idêntico: 10/10 testes L1 e 3/3 integrações L3 em `forward`, `forward_repeat` e ordem inversa. Assim, P01–P12 e N01–N05 são `Preserved`. O binding qualificado é função distinta de `scripts`; os três estados causais de `inline` são preservados; a chamada bare e qualificada convergem; os cinco diagnósticos públicos permanecem; walkers, igualdade, `ObservationEq`, `repr`, reconstrução por default e geometria estabelecida foram observados diretamente. Os quatro controles opacos continuam `Unknown`, recebem crédito zero e nenhum caso obrigatório foi promovido a sucesso.

O refinamento R1 cobre explicitamente o sétimo consumer. P12 compara dois `MathLimitsOverrideElem` completos: valores idênticos produzem `Same`, enquanto omissão e `inline:true` explícito, diferindo só em `inline_explicit`, produzem `Different`. Logo M21, que omite, infere, ignora ou torna opaco esse campo em `ObservationEq`, é `Violated` e não sobrevive.

Resultado adversarial: 21/21 mutantes válidos `Violated`, zero sobreviventes, mutation score `1.0`, igual ao mínimo exigido. Os vetores permaneceram idênticos nas três ordens. `Unknown` ocorreu somente nos quatro controles opacos congelados.

Os gates finais passaram: `cargo fmt --all -- --check`, `cargo build`, `crystalline-lint .` e `git diff --check HEAD`. As regressões direcionadas passaram com 22/22 testes no filtro P992 e 17/17 no filtro `scripts`.

O PASS atesta somente binding e assinatura de `math.limits`, presença causal de `inline`, convergência bilateral, carrier e walkers, relação observacional, `repr`, layout já vigente e preservação de `scripts` no fragmento R1. Não atesta paridade matemática geral, as outras 21 rotas residuais, nova geometria de attachments, igualdade estrutural Rust ou igualdade de bytes de render.

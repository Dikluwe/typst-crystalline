# Context — fechamento do lote textual

HEAD b7e7c83223e5daa67bbdf124c36b5b7f93200731, branch Tekt, working tree não
commitado. Freeze text-final-sources.json.gz e text-final.patch.gz; UTC, diffstat,
argv e recursos em validation-text-final-resources.json. Encerramento da execução:
2026-10-18T16:48:12.319321+00:00. Sem commit, push, merge ou release.

## Resultado

Corrigidas as duas lacunas de herança textual: global A→R e local A→L.
Captura conserva baseline/delta local; a inserção sela as regras efetivas sem
executar a closure. RuleIds herdados são reservados antes de criar regras locais
no corpo contextual. Retirados os filtros de captura exclusivos Equation/Table
e seus dois testes de whitelist, substituídos por dez testes funcionais.
Styled comum mantém seu contrato; não se alterou API pública ou fase.

## Evidência

- Baseline recompilado das fontes atuais; before.json.gz reproduz as quatro
  divergências herdadas e controles. Vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3,
  SHA-256 9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
- RED textual: nove falhas, um controle aprovado. Candidato1 corrigiu nove mas
  regrediu o controle local; candidato2 resolveu a colisão de RuleId. Logs e
  recibos red-text/green-text1/green-text2 preservados, sem renomear falhas como GREEN.
- Dez testes de Context em PDF e HTML aprovados; dez sondas textuais CLI PDF
  coincidem com o vanilla, incluindo nesting/regex/produto/pai ocultando filho.
- Suíte final: **9119 aprovados, zero falhas, sete ignorados**, incluindo cinco
  doctests. Conta anterior 9111 + dez testes funcionais − dois testes do filtro
  retirado. suite-summary-text-final.json preserva cada executável.
- Build, Clippy all-targets estrito, Rustdoc estrito, fmt, linhagem direta/reversa
  e diff-check passaram. Ownership/núcleos íntegros, selos bidirecionais refeitos.
- SARIF: **105 V16:error**, 1140 V19:note, 879 V20:note. Nenhum novo erro/warning.
  O lookup do snapshot foi extraído sem mudar o despacho; audit.json registra
  essa movimentação na comparação. V16 global continua vermelho.
- Tekt final SHA-256 `f3dc15354db6b3b27829ef0f0da9fcd20b2be29ec44e7a1dd6aa613cc1d2b341`. A fonte baseline
  coincide em 2098 arquivos protegidos com o inventário anterior (baseline-equivalence.json).

## Recursos e limites

Execução sequencial, Cargo jobs=1, debug=0, incremental=0, cgroup Max12GiB,
High8GiB e swap zero. Pico desta validação: **4.676 GiB**.
Eventos: `low 0; high 0; max 0; oom 0; oom_kill 0; oom_group_kill 0`.
Não é benchmark de produção ou declaração de paridade universal.

As duas lacunas de identidade permanecem nesta revisão: duas equações Q1/Q2,
prefixo Heading Q0/Q1; it.location dá 00/01. after.json.gz também contém
alias, conteúdo gerado e nesting ainda divergentes. As sondas grow/shrink desta
primeira coleta usam Heading sem numbering e não provam mudança de ramo;
serão complementadas por controles numerados antes da arena.

O humano aprovou a proposta de arena/frame sob
00_nucleo/materialization/context-occurrences-2026-10-18/review.md, hash
500e01cf0754a166deb7f493ff734f040ef38e0374cbf5aa0ba879a37d19f388.
Recibo no mesmo diretório. A materialização seguinte tem freeze e validação
próprios; estes resultados pertencem exclusivamente à revisão textual.

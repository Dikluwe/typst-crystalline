# P1322 — contrato de revisão D, congelado antes do checker

Congelamento: 2026-09-08T22:11:25Z. HEAD observado: `d31047d7b8af7837c84adae4ded3d2ff50c62093`; a identidade efetiva de produto será o manifesto do operador, não HEAD sozinho. Este arquivo é plano de verificação, não resultado.

Executor: agente `/root/p1322_review`, papel D revisor/verificador. Entradas permitidas: passo P1322 explicitamente liberado, skill e referências abaixo, ADRs, fontes/L0 em leitura, diagnósticos históricos pertinentes e artefatos P1322 do operador/inventarista/classificador. Escritas: somente `00_nucleo/diagnosticos/p1322-review-*` e temporário exclusivo, se necessário. Não escreve catálogo, inventários, matrizes, ledger, produto ou L0 julgados. Não herda resultados P1309 como resultados atuais. Contexto recebido: instruções gerais do repositório e delegação P1322, sem resultados novos da operação bilateral. Ferramentas e filesystem compartilhados permitem tecnicamente mais que esta allowlist: execução **sem atestação técnica de isolamento**; separação é de autoria declarada e recibos, não selo atestado.

Entradas lidas integralmente e hashes SHA-256:

| Entrada | SHA-256 |
| --- | --- |
| `00_nucleo/materialization/typst-passo-1322.md` | `40dc97700ea3184aaf3db914f6251bbd7eb4dfb33fc0563a8a480a2f1ab443d4` |
| `/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md` | `66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48` |
| skill `references/papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| skill `references/artefatos-e-gates.md` | `16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d` |
| ADR-0107 | `e680d22bbf4486cf93f5bfb4ec85f4ae965e6db788c2a18c48f3be000029d49d` |
| ADR-0108 | `31daec5ae9e84cb5bbdcb806e9a2b6cb9160b7e90519df53e6e0bd8809076405` |
| ADR-0127 | `5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9` |
| ADR-0129 | `64756b81ce58ca62e1a166b3776303759bc7af507a1c97a4e3ad91a8dc5b906e` |

Regime: revisão independente de auditoria somente leitura, com contrato discriminatório sobre cópias de dados do auditor. Nenhuma mutação do produto. Busca em `00_nucleo/adr/` por segregação/isolamento não identificou ADR local dedicada a materialização segregada. As ADRs acima e P1322 governam o recorte.

## Observáveis obrigatórios do checker

1. Reconciliação de todos os IDs históricos e união das rotas vanilla/cristalinas recém-observadas, incluindo ancestors bloqueados. Checar unicidade e razões de added/removed/renamed/split/merged.
2. Cartesiano catálogo × quatro perfis × normal/repeat/reverse, recomposto por chave. Conferir identidades de binário, fixtures, argv/perfil e obrigatoriedade de stdout, stderr e exit; missing/timeout/crash/parser opaco produzem Unknown.
3. Recalcular igualdade de valores e diagnóstico completo a partir das saídas preservadas; comparar classes publicadas. Repetições não acrescentam features. Reordenar registros não altera resultado.
4. Verificar denominador de inventário, Unknown separado e suplemento funcional fora desse denominador. Igualdade ajustada exige fundamento normativo atual para cada extensão subtraída.
5. Identidade funcional não se reduz a nome/repr. Presença de membro não comprova funcionalidade; ausência bilateral de encoders não é lacuna. Fechamentos exigem sentinelas do comportamento alegado, incluindo válidos.
6. MATCH histórico que virou diferença/Unknown deve aparecer nas transições; atribuição ao produto só após isolar adapter/fixture/ambiente. Ledger causal deve trazer testemunha bilateral, fonte, perfil, file:line, owner/L0, hipótese e refutação.
7. Seleção por prioridade explícita P1322/P1309 e causa demonstrada. Desempates: número real de owners necessários, paths comprovadamente da mesma causa, superfície de regressão conhecida, ID lexical. Risco desconhecido não é baixo. Unknown obrigatório/instabilidade bloqueia seleção, preservando diagnóstico válido parcial.
8. Gates finais e byte-identidade produto/L0 são confrontados com baseline efetivo. Falha preexistente é reportada; não desaparece pelo rótulo histórico.

## Ataques congelados e controles correspondentes

Todo ataque será aplicado a uma cópia do estado genuíno pertinente (com inputs e hashes registrados), sem sobrescrever o original. Ataque sem precondição real demonstrada será explicitamente `NOT_EXECUTED`, não contará no score. `Violated` só conta quando contém testemunha da regra quebrada. Dados deliberadamente opacos devem ser `Unknown`, nunca `Preserved`.

| ID | Transformação negativa em cópia | Controle genuíno | Resultado esperado |
| --- | --- | --- | --- |
| D01 | Omitir um ID histórico da reconciliação/catálogo | Mesmo catálogo completo | Violated: historical ID omitted |
| D02 | Omitir rota nova descoberta de um lado | União íntegra dos dois inventários | Violated: discovered member omitted |
| D03 | Trocar identidade de binário de uma execução mantendo resultado | Identidades observadas no manifesto | Violated: binary identity mismatch |
| D04 | Trocar perfil/argv em uma célula mantendo label | Perfis/argv genuínos, incluindo html+a11y | Violated: profile mismatch |
| D05 | Apagar stderr não vazio ou seu campo e conservar MATCH | Diagnóstico bruto integral | Violated: diagnostic evidence altered; falta sem alegação de MATCH é Unknown |
| D06 | Converter execução Unknown em MATCH | Unknown honestamente preservado | Violated: Unknown promoted; controle opaco é Unknown |
| D07 | Declarar fechamento funcional baseado apenas em presença/kind/repr | Presença medida com limite declarado | Violated: presence-only closure |
| D08 | Trocar observável de identidade funcional por comparação nominal de nome/repr | Sentinela direta/alias/With e carrier observado | Violated: nominal identity capture |
| D09 | Remover transição de MATCH histórico para diferença atual | Todas as transições reconstruídas | Violated: hidden regression candidate |
| D10 | Somar suplemento ao denominador do inventário | Denominador catálogo × perfis, repetições separadas | Violated: inflated denominator |
| D11 | Tratar extensão como autorizada sem L0 atual | Extensão comprovada ou diferença não ajustada | Violated: undocumented extension |
| D12 | Escolher coorte menos prioritária ignorando causa/ordem | Ranking explícito com testemunhas causais | Violated: invalid ranking |
| D13 | Retirar owner necessário ao transporte/observável completo | Owners comprovados por rota da testemunha | Violated: owner undercount |

Controles adicionais: permutação da ordem de registros mantém o veredito; repetição conserva classificações; ausência bilateral declarada como ausência não gera dívida; Unknown verdadeiro bloqueia fechamento/seleção e não vira falha de língua. Ataques D07/D08/D11–D13 podem exigir revisão semântica/manual além de cálculo; a execução deverá distinguir a parte automatizada da parte inspecionada, sem alegar que um predicado textual prova intenção arquitetural.

## Revisões e limite de calibração

Budget inicial de calibração: até duas revisões focais por causa antes de rever método/observabilidade; até uma execução completa de controles após cada correção focal aprovada, sem repetir a matriz bilateral por mudança do checker. Registrar UTC, duração, custo, vetor de classes e delta por revisão. Duas revisões consecutivas sem ganho na mesma causa suspendem a calibragem dessa causa. Se faltar entrada obrigatória, publicar revisão parcial Unknown, preservar ataques não executados e não emitir selo/seleção.

Ordem causal: congelar este plano → receber dados genuínos → criar/adaptar leitor de schema sem mudar regras → executar controles e ataques → revisão focal se necessária → conferir ledger/seleção/gates finais → publicar recibo independente com hashes. O checker não escreve os dados julgados.

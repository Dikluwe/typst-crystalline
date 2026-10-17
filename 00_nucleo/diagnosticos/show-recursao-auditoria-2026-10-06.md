# Show: identidade da ocorrência e encerramento por morfologia

A auditoria confirmou quatro diferenças de aceitação em oito controles. Alias
it, duas inserções de it e gerações seletivas finitas compilam nos dois. Quatro
receitas que criam novos headings abrangidos pela própria regra compilam no
Tekt e são rejeitadas pelo vanilla por maximum show rule depth exceeded.
O comportamento não é explicado por um simples limite numérico ausente.

## Proveniência

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC `2026-10-06T13:26:04.786505+00:00`.
Binário Tekt anterior às correções de formas/HTML nesta rodada SHA-256
`c4d483841bf40830599b6977cc2cbd1d0a5f3ec7dd8e0f6eb212e0e600e93914`; o código de show permanece inalterado.
Oracle pinado 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. measurement.json.gz conserva fonte/L0 dos
três módulos de realização/terminação, stat/status, comandos, texto/PDF,
consultas, hashes e saídas. SOURCE_DATE_EPOCH=0 e RAYON_NUM_THREADS=1 na
compilação. Não houve benchmark.

| Controle | Exit Tekt | Exit vanilla |
|---|---:|---:|
| alias | 0 | 0 |
| alias_twice | 0 | 0 |
| bounded_generated | 0 | 0 |
| fresh_same_body | 0 | 1 |
| fresh_z | 0 | 1 |
| alias_plus_generated | 0 | 1 |
| text_and_generated | 0 | 1 |
| bounded_replacement | 0 | 0 |

## Medição antes da classificação

`eval/rules/apply.rs:75-92` descreve P348 como divergência consciente: fresh
heading Z convergiria pela igualdade de morfologia. `eval/tests.rs:1259-1273`
exige esse resultado. `eval/show_rule_termination.rs:61-67` encerra quando
a forma se repete; seu L0 determina o mesmo. O rótulo consciente no código
não comprova aprovação específica do usuário.

`realize/mod.rs:149-176` aplica cada snapshot via map_content; `entities/content/
traversal.rs:11-21` transforma após os filhos, sem revisitar o produto devolvido.
`eval/rules/apply.rs:109-135` usa uma guarda RuleId enquanto executa o callback.
Esses mecanismos em conjunto também deixam produtos em Sequence fora de nova
aplicação da mesma regra; a ausência de recursão não prova ponto-fixo correto.

No oracle pinado, `typst-realize/src/lib.rs:462-495` guarda a regra por elemento
realizado, não por igualdade do corpo. A sonda separa alias e elemento recém-
construído com corpo igual. Compilar ou rejeitar, número de efeitos e produtos
visíveis são observáveis da linguagem; não são mera diferença de PartialEq Rust.
Não copiar a representação de guards do upstream nem inferir intenção.

## Proposta concreta para o owner apply

Na realização produtiva de elementos localizáveis, usar a identidade Located
existente para proteger (ocorrência, regra) e distinguir alias de nova construção.
Alias de it mantém proteção; produtos recém-criados voltam ao matching no mesmo
ambiente lexical, com budget de profundidade da linguagem. Nenhuma identidade
é deduplicada por corpo/forma. Preservar Realized.original, estilos, regras
where/show-set, namespace de contextos, ordem causal, índices e contadores.
Não reavaliar o arquivo nem introduzir estado global/unsafe/API ou nova fase.
O helper legado de terminação continua proprietário de seu contrato atual.

Aceitação proposta: os quatro controles inválidos passam a rejeitar por limite
de show; os quatro controles finitos conservam conteúdo/consultas e número de
ocorrências. As receitas com efeitos/contextos, múltiplas regras, where e alias
são refutadores obrigatórios antes de fechar, não sucessos implícitos.

Esta revisão retira uma política explicitamente documentada e altera programas
hoje aceitos. A classificação de compatibilidade é uma interpretação cautelosa;
CLAUDE.md:31-39 exige parada em caso de dúvida. A proposta L0 canônica é
`00_nucleo/prompts/compiler/eval/rules/apply.md`; não materializar esse pacote
antes da confirmação. Reparos locais de formas e HTML seguem em fluxo contínuo.

## Atualização — show por ocorrência materializado em 2026-10-06

O gate foi confirmado e implementado. Ver `00_nucleo/diagnosticos/show-ocorrencia-reparo-2026-10-06.md` e seu receipt repair/execution.json.gz: 6.551 testes L1, 1.183 L3 e 105 CLI passaram; 228/228 fixtures de layout não regrediram. As quatro gerações ilimitadas medidas agora rejeitam, com aliases e gerações finitas preservados. A matriz macro mantém 31 MATCH/5 DIFF crus; todos os cinco DIFF atuais se limitam ao caminho da fonte, com mensagem/range/hints iguais após projeção explícita. A proposta de apply não está mais aguardando confirmação. Persistem origem de Content constante, caminhos crus, duas fixtures de matriz e seletor table.cell; não se afirma ganho de tempo.

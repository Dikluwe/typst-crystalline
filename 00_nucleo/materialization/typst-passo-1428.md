# Passo 1428 — recalibração observável do certificado `math.frac`

## Estado medido antes da decisão

P1427 implementou a convergência completa de `math.frac`, mas o verificador
segregado encerrou o passo como `BLOCKED`. A implementação aprovou 15/15
oráculos L1 nas três ordens, enquanto a suíte pública L3 repetiu 4/6. As duas
falhas têm causa no observador, não uma testemunha atribuível de divergência do
produto:

1. o HTML contém `style: &quot;vertical&quot;`, que representa exatamente o texto
   `style: "vertical"`, mas o teste procura aspas cruas na serialização;
2. `compile_to_svg_string` devolve o mesmo grupo vazio para as quatro fixtures
   de parênteses, logo o SVG não transporta informação suficiente para decidir
   P13, P14, P15 ou P21.

Medição executada em `2026-09-16T19:06:59-03:00`, sobre HEAD limpo
`22d7795d48786e590e12c1d6c02280673c657a8c`. Entradas congeladas:

| artefato | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1427-report.md` | `70086f4d7ae3247b700e863e9cb4c2f95ff833367433071f5b7587df96b9b44e` |
| `00_nucleo/diagnosticos/p1427-verification.json` | `6e806664fd8ceca5718f1b3f91dd70f5d1eef19a4e25b0d9dd1a1b7fa742764d` |
| `03_infra/tests/p1427_math_frac.rs` | `708a39bc4da195ecfab55bdac0c780a9161a353fb1889375cf1b6ef950626ff7` |
| `00_nucleo/prompts/infra/tests/p1427-tests.md` | `fecdbce0a62a01d85343e54ff244b6bb50d5874a03bd69e5fe333b77bb92a8c6` |

## Classificação

Este passo não muda contrato público, comportamento por defeito, fase do
pipeline nem compatibilidade. Ele substitui dois observadores inadequados por
observadores que expõem a mesma obrigação já aprovada. Portanto segue o fluxo
contínuo do ADR-0127, com L0 de teste escrito antes do novo teste, sem paragem de
confirmação intermediária.

Como entradas protegidas do protocolo P1427 precisam ser substituídas, o selo
P1427 não será editado. P1428 cria manifesto, contrato, oráculos, mutantes,
testes e selo próprios, referenciando P1427 como predecessor causal.

## Decisão

1. manter todo código produtivo do commit P1427 byte-idêntico durante P1428;
2. criar um teste L3 novo e proprietário, sem editar a suíte selada P1427;
3. observar `repr` como texto semântico do HTML: a serialização é decodificada
   antes da comparação, e entidades desconhecidas ou markup ambíguo produzem
   `Unknown`;
4. abandonar o SVG vazio apenas como observador destes quatro casos, não como
   alegação sobre o backend SVG;
5. avaliar as quatro fontes reais pela API pública `eval_to_module_with_sink` e
   extrair do `Module` o único `MathFracElem` alcançável. Os bits
   `num_deparenthesized` e `denom_deparenthesized` devem formar exatamente
   `00`, `10`, `01`, `11`; zero ou múltiplos carriers são `Unknown`;
6. compor essa testemunha sintática/semântica com o oracle de layout P1427 já
   congelado, que distingue a morfologia horizontal. Nenhum dos dois sozinho é
   promovido a equivalência geral do pipeline;
7. preservar os controles de rotas, diagnósticos, constantes OpenType e
   `binom`; a nova suíte deve demonstrar que a correção dos observadores não
   afrouxou os casos que já passavam;
8. exigir `mutation_score = 1.0` contra mutações do observador: comparar HTML
   cru, decodificar entidades em ordem insegura, aceitar markup ausente,
   reutilizar o SVG vazio, aceitar carrier ausente/múltiplo, inverter/forçar os
   bits ou conceder crédito a `Unknown`.

## Limites

P1428 não altera `math.frac`, layout, parser, exportador HTML/SVG, formato de
serialização nem as entradas seladas P1427. Não afirma que SVG matemático vazio
é correto; apenas prova que essa saída é opaca para o recorte. Não reduz as
obrigações P13/P14/P15/P21: troca a fonte de evidência causal e exige a mesma
classificação nas três ordens.

## Regime e gates

Aplicar o protocolo completo `tekt-materializacao-segregada`, executado sem
atestação de isolamento no workspace partilhado:

1. autor independente cria manifesto, contrato e oráculos sem editar testes;
2. adversário cria mutantes e o teste independente P1428;
3. contrato é selado somente após RED atribuível e score 1.0;
4. a implementação limita-se ao L0 proprietário e ao teste/observador P1428;
5. verificador final executa três ordens, confere que os 12 consumers
   produtivos P1427 estão byte-idênticos ao predecessor e emite certificado;
6. `cargo fmt --all -- --check`, testes focais, `cargo build`,
   `crystalline-lint .` e `git diff --check` devem passar.

`Unknown` continua fail-closed. Duas revisões sem ganho discriminatório ou uma
nova ausência de informação pública interrompem a calibração e reabrem o
desenho, sem ajustar predicados ao candidato.

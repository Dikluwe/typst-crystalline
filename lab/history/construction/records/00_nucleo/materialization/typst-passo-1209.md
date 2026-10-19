# P1209 — triagem semântica dos avisos V16–V21

**Estado:** EXECUTADO — GREEN POR TRIAGEM (2026-08-26)
**Dependência:** P1208 GREEN, com V5/V7/V15/V26=0 e `fix-hashes` idempotente.
**Princípio:** reduzir risco e aumentar clareza; não perseguir zero por expansão
mecânica de `match`, extração artificial de helpers ou comentários vazios.

## Baseline

Execução fresca após P1208:

| Regra | Total | Produção | Testes | Classe inicial |
|---|---:|---:|---:|---|
| V16 | 210 | 136 | 74 | acionável por auditoria |
| V17 | 36 | 26 | 10 | acionável seletivo |
| V18 | 2 | 2 | 0 | classificação de fronteira |
| V19 | 349 | 308 | 41 | métrica informativa |
| V20 | 600 | 552 | 48 | métrica informativa |
| V21 | 24 | 9 | 15 | proveniência acionável |

O linter termina com exit 0. Nenhuma dessas contagens é prova isolada de
defeito; cada decisão exige leitura do owner L0 e medição anterior.

## 1. Congelar inventário reproduzível

Registrar HEAD, hora, `git diff HEAD --stat`, digest do diff e índice. Gerar um
TSV em `00_nucleo/diagnosticos/` com regra, nível, ficheiro, linha, trecho,
produção/teste e owner L0. Não ler nem listar `00_nucleo/context/` ou
`00_nucleo/materialization/`.

Classificar cada ocorrência com um dos resultados:

- `REFACTOR`: há perda semântica, decisão misturada ou constante arbitrária;
- `CITE`: fórmula correta, mas falta fonte/rationale verificável;
- `ACCEPT-BOUNDARY`: parsing, serialização ou enum deliberadamente aberto;
- `ACCEPT-EQUIVALENT`: alternativas condensadas têm corpo semanticamente igual;
- `TEST-FIXTURE`: literal deriva diretamente da construção da fixture;
- `BACKLOG-L0`: correção requer decisão normativa ainda ausente;
- `GATE-ADR0127`: mudaria contrato, default, fase ou compatibilidade.

## 2. Lote A — V21 produtivo primeiro

Auditar integralmente os 9 V21 produtivos, começando por:

- `compiler/layout/boxed.rs`: duas fórmulas `font / 11.0`;
- `compiler/layout/stack.rs`: `1.2 × size`;
- `compiler/math/layout/cancel.rs`: `0.05 × size`;
- `compiler/math/layout/matrix.rs`: gaps `0.5/0.2 × font` e stroke `0.05 × font`;
- `compiler/math/layout/mod.rs`: `0.25 × size`;
- `compiler/math/layout/underover.rs`: ocorrência restante.

Para cada fórmula:

1. ler integralmente o Prompt L0 proprietário vigente e seus núcleos pinados;
2. medir o vanilla ratificado `a51e02804` em `file:line` e, quando observável,
   por sonda de linguagem;
3. distinguir constante normativa, métrica de fonte, aproximação cristalina e
   mero detalhe mecânico;
4. atualizar primeiro o L0 quando a fórmula correta já estiver materializada;
5. usar `// ref:`, `// spec:` ou `// rationale:` somente com conteúdo que permita
   reproduzir a decisão — nunca para silenciar o linter;
6. se a constante estiver errada, escrever teste RED e corrigir em fluxo
   contínuo apenas quando for paridade interna. Parar no ADR-0127 se a mudança
   alterar comportamento por defeito sem ser correção inequívoca de paridade.

Nos 15 V21 de testes, citar somente valores derivados de comportamento
normativo; classificar como `TEST-FIXTURE` valores algébricos evidentes e
propor ajuste do detector/configuração em passo separado se ele exigir
comentários redundantes.

## 3. Lote B — V16 por risco, não por volume

Priorizar as 136 ocorrências produtivas cujo wildcard devolve `None`, `false`,
`{}`, `vec![]` ou outro default neutro. Para cada uma:

- identificar o enum real e listar suas variantes vigentes;
- provar se o wildcard significa “todas as variantes restantes têm a mesma
  semântica” ou se apaga informação;
- usar braços explícitos quando a enumeração fecha uma obrigação de domínio e
  faz novas variantes falharem em compilação;
- delegar a função nomeada quando existe uma fronteira total legível;
- manter e classificar `ACCEPT-BOUNDARY` quando o enum é externo/não exaustivo
  ou a fronteira realmente aceita qualquer caso futuro;
- não expandir dezenas de variantes para corpos duplicados sem benefício.

Auditar primeiro código produtivo; ocorrências em testes entram somente quando
o wildcard poderia fazer uma asserção passar silenciosamente.

## 4. Lote C — V17 seletivo

Nos 36 guards compostos, desdobrar apenas quando os operadores lógicos unem
casos com razões, resultados ou diagnósticos diferentes. Manter condições
coesas quando o predicado possui uma única intenção e um nome extra não
melhoraria a leitura.

Cada `REFACTOR` deve preservar ordem de avaliação, curto-circuito, spans e
mensagens observáveis. Adicionar teste focal quando o desdobramento separar
comportamentos antes não cobertos.

## 5. V18 — classificar os dois ranges

Auditar:

- `03_infra/src/export/builder.rs`: `0x00..=0x1f | 0x7f`;
- `03_infra/src/export/oracle.rs`: `b'0'..=b'9' | b'-'`.

Se forem classes de bytes definidas por PDF/formato, registrar
`ACCEPT-BOUNDARY` com referência normativa e manter o range. Só refatorar se o
match estiver modelando domínio fechado e o range ocultar casos semanticamente
distintos. Não enumerar bytes individualmente para zerar V18.

## 6. V19/V20 — observação e ratchet

Não abrir 949 refactors. Produzir histogramas por ficheiro e destacar apenas:

- V19 com muitos padrões semanticamente diferentes num único braço;
- V20 nos maiores níveis de profundidade ou repetidos em fronteiras críticas;
- casos que coexistam com V16/V17 no mesmo bloco.

Refatorar somente os outliers cuja leitura, teste ou diagnóstico melhore.
Registrar os demais como `ACCEPT-EQUIVALENT`/`ACCEPT-BOUNDARY` e estabelecer
baseline ratcheted: novas ocorrências não entram sem classificação, enquanto o
estoque legítimo permanece visível como métrica. Não configurar V19/V20 como
meta de zero.

## 7. Nucleação, execução e gates

Agrupar mudanças por owner 1:1. Antes de qualquer código:

1. ler e auditar o L0 proprietário;
2. editar L0 primeiro quando a decisão mudar ou ganhar proveniência normativa;
3. confirmar V15/V26 após qualquer pin/núcleo;
4. testes RED quando houver comportamento corrigido;
5. implementação mínima e resselo focal;
6. ao fim de cada lote, `crystalline-lint --fix-hashes --dry-run .` deve ficar
   vazio; executar fix real somente se os hashes mudaram e o preflight passou.

Gates finais:

- V5/V7/V15/V26=0;
- nenhum `REFACTOR`, `CITE`, `BACKLOG-L0` ou `GATE-ADR0127` sem destino;
- V21 produtivo=0 por correção ou proveniência substantiva;
- V16/V17 acionáveis=0, ainda que ocorrências classificadas permaneçam;
- V18=0 acionável, aceitando ranges normativos;
- manifesto/histograma ratcheted para V19/V20;
- testes focais, `cargo build`, `git diff --check` e índice vazio.

## 8. Fechamento

Criar `00_nucleo/diagnosticos/typst-p1209-triagem-v16-v21.md` com inventário,
classificação, medições, owners tocados, gates ADR-0127, contagens antes/depois,
baseline V19/V20 e proveniência temporal.

P1209 não promete zerar todos os avisos. Fecha quando todo sinal acionável foi
resolvido ou encaminhado e o restante possui classificação defensável sem
degradar a expressão semântica do código. Fecho: V16 permanece visível por
decisão categórica do linter (ADR-0017); V17–V20 ficam ratcheted; V21 produtivo
foi zerado e os 15 sinais de teste foram classificados como fixtures.

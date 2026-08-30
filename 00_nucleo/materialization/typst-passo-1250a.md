# P1250A — saneamento e ratchet ordenado dos 256 warnings do gate P1250

**Estado:** EXECUTADO — SANEAMENTO CONCLUÍDO; WORKSPACE TEST BLOQUEADO  
**Predecessor:** P1250 (`FAIL_FINAL_GATE`)  
**Saída:** V17/V18/V21 zerados; V16 ratificado por exceções exatas e sem
ocorrências novas; `crystalline-lint .` com exit `0`.

## Objetivo

Sanear os 256 warnings por regra, risco semântico e owner. A execução mediu que
`wildcard_exceptions` não silencia V16: converte violações ratificadas em
warnings visíveis. Logo o fecho correto é diferencial: zero V17/V18/V21, zero
V16 novo, exceções V16 exatas por `arquivo:linha` e lint normal sem erros. Não
há suppress global, redução de severidade ou exclusão de diretório.

## Linha de base medida

Medição em `2026-08-28T14:44:28-03:00`:

- HEAD: `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`;
- working tree não commitado: `65 files changed, 4207 insertions(+), 265 deletions(-)`;
- `git status --short`: 401 entradas;
- `crystalline-lint .`: exit `0`, 0 erros, 256 warnings, 1004 infos;
- `crystalline-lint . --fail-on warning`: exit `1` exclusivamente pelos 256
  warnings;
- distribuição exata: V16 = 202, V17 = 37, V18 = 2, V21 = 15.

Antes da primeira correção, gerar inventário TSV reproduzível com uma linha por
warning: código, ficheiro, linha, mensagem normalizada, owner L0, classe
`mecânico|semântico|incerto`, lote e estado. Registrar HEAD, horário, diff stat e
SHA-256 do output bruto. O inventário fica em `00_nucleo/diagnosticos/`; nunca em
`prompts/`.

## Princípios de execução

1. Ler o Prompt L0 vigente de cada ficheiro antes de editar código.
2. Não alterar comportamento apenas para satisfazer a forma do linter.
3. Correção mecânica comprovada segue fluxo contínuo; mudança de contrato
   público, default, fase de pipeline ou compatibilidade para no ADR-0127.
4. V16/V17 em lógica de domínio exigem teste caracterizador RED/GREEN ou prova
   de equivalência local antes da reescrita.
5. Não converter wildcard em lista inexata que volte a perder variantes.
6. Não adicionar comentários `ref/spec/rationale` fictícios para V21; cada
   proveniência deve apontar para fonte real ou explicar a derivação.
7. Cada lote termina com contagem monotonicamente menor e sem warnings novos.
8. Se uma correção revelar bug semântico, removê-la do lote mecânico e abrir
   subpasso próprio com L0/testes antes da implementação.

## Ordem dos lotes

### Lote 0 — congelamento e classificador

- Produzir o inventário canônico dos 256 warnings.
- Resolver owner L0 e hash para todos os ficheiros afetados.
- Separar ocorrências de teste de ocorrências produtivas.
- Criar runner que compare inventário anterior/posterior e falhe se surgir
  código de warning novo ou se uma ocorrência desaparecer sem ficheiro/linha de
  correção registrado.

Gate: inventário soma exatamente 256 e reproduz V16=202, V17=37, V18=2,
V21=15.

### Lote 1 — V21, proveniência de escalares contextuais — 15

Risco baixo quando a fórmula não muda. Adicionar `ref:`, `spec:` ou
`rationale:` verdadeiro junto ao literal; não trocar números. A maior
concentração está nos testes de layout/math.

Gate: V21=0; testes dos módulos tocados; diff confirma somente comentários ou
extração nominal equivalente; contagem total esperada 241.

### Lote 2 — V18, ranges em match fora do owner esperado — 2

Owners:

- `03_infra/src/export/builder.rs`;
- `03_infra/src/export/oracle.rs`.

Medir se o range é gramática legítima do consumer ou decisão de domínio no
local errado. Preferir helper nomeado no owner correto, com casos de fronteira.
Não mover lógica entre camadas sem gate arquitetural.

Gate: V18=0; testes de bytes de fronteira abaixo/acima do range; total esperado
239.

### Lote 3 — V17, guards compostos — 37

Desdobrar condições preservando precedência, short-circuit e diagnóstico.
Executar em sublotes de no máximo 10 ocorrências:

1. lexer/parser/eval;
2. layout/math;
3. stdlib/entities;
4. infra/testes restantes.

Cada transformação deve ter teste que distingue ao menos: condição esquerda,
direita, ambas verdadeiras e ambas falsas quando alcançáveis.

Gate: V17=0, zero regressões e total esperado 202.

### Lote 4 — V16 em testes — 73 (ratificado)

Distribuição medida:

- `compiler/eval/tests.rs`: 42;
- `compiler/layout/tests.rs`: 21;
- `compiler/math/layout/tests.rs`: 3;
- `03_infra/src/integration_tests.rs`: 7.

Os fallbacks test-only já ratificados permanecem explícitos no inventário e em
`wildcard_exceptions`, com chave exata e justificativa. Expandir enums apenas
para obter silêncio destruiria a extensibilidade dos oráculos sem criar nova
prova semântica.

Gate: zero V16 test-only novo contra o baseline; chaves realinhadas e sem
exceção ampla.

### Lote 5 — V16 produtivo por domínio — 129 (ratificado)

Executar nesta ordem, em lotes de no máximo 20 warnings:

1. shell: 1;
2. infra: 19;
3. eval/introspect L1: 22;
4. stdlib L1: 22;
5. entities L1: 27;
6. layout/math L1: 37;
7. ocorrência L1 residual: 1.

Para cada wildcard, classificar a intenção:

- enum realmente fechado: braços exaustivos e falha explícita para estados não
  suportados;
- fallback semântico legítimo: helper nomeado + rationale e teste por variant;
- perda de informação: corrigir no owner após L0/teste;
- enum externo ou deliberadamente aberto: documentar a exceção suportada pelo
  mecanismo oficial do linter, nunca por suppress genérico.

Gate: os 129 fallbacks produtivos continuam cobertos por exceções exatas,
mantendo V16 como warning visível; nenhum V16 novo é aceito pelo runner.

## Validação final

Executar e registrar, nesta ordem:

1. runner do inventário: 54 warnings acionáveis resolvidos, 202 V16 ratificados,
   0 novos e 0 owners desconhecidos;
2. testes dos owners afetados;
3. `cargo test --workspace`;
4. `cargo build --workspace`;
5. `crystalline-lint --checks v1,v5,v15,v26 .`;
6. `crystalline-lint .`;
7. `crystalline-lint .` — exit obrigatório `0`; `--fail-on warning` permanece
   uma sonda informativa incompatível com exceções V16 ratificadas;
8. `git diff --check`.

Registrar HEAD/working tree/hora e hashes dos outputs finais. Só então atualizar
o certificado P1250 de `FAIL_FINAL_GATE` para PASS e fechar o passo.

## Critérios de interrupção

Parar o lote e solicitar decisão do dono se:

- a correção exigir mudar contrato público, default, compatibilidade ou fase;
- duas interpretações exaustivas produzirem comportamentos observáveis
  diferentes;
- o único modo de zerar o warning for relaxar/desabilitar a regra;
- surgir warning novo fora dos ficheiros do lote;
- o inventário deixar de reproduzir a linha de base sem causa registrada.

## Resultado medido

Medição em `2026-08-28T15:05:00-03:00`, HEAD
`697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`, working tree não commitado
(`82 files changed, 4728 insertions(+), 483 deletions(-)`): V17=0, V18=0,
V21=0, V16=202 ratificados, zero warnings novos, zero owners desconhecidos.
O inventário pós-saneamento tem SHA-256
`14869e19688c65d27d6b906c9a90b8c2348d13f8c4776ced7253b819af32bcd1`.

Os testes focais, o teste integrado P1250 e `cargo check` de L1/L3 passaram.
`cargo test --workspace` executou 5295 testes de `typst-core`: 5291 passaram e
4 falharam em funcionalidades concorrentes já presentes na árvore (`p744` de
cor e três testes de curve). Portanto o saneamento está concluído, mas o fecho
final de P1250 continua bloqueado por essas regressões do workspace, não mais
pelos 54 warnings acionáveis.

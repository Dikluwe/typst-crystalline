# Passo 1139 — Paridade dos diagnósticos públicos

**Data:** 2026-08-23  
**Origem:** baseline P1138, casos `P1138-S-001..003`  
**Vanilla ratificado:** `upstream/main a51e02804`  
**Classe:** `PUBLIC_DIAGNOSTIC` — a mecânica textual é o observável público  
**Gate:** mudança de comportamento por defeito e de API pública; ADR-0127

## 1. Objetivo

Fechar os três diagnósticos medidos como `DIFF`:

| ID | Caso | Resíduo medido |
|---|---|---|
| `P1138-S-001` | erro sintático | cristalino emite uma linha gcc; vanilla emite cabeçalho, localização, source e caret |
| `P1138-S-002` | nome desconhecido | quoting e forma da mensagem divergem; bloco fonte/caret ausente |
| `P1138-S-003` | erro em include | além das diferenças acima, falta `while including` com localização e snippet do chamador |

O resultado deve igualar a linguagem pública do diagnóstico vanilla: severidade,
mensagem, ficheiro, linha, coluna, trecho, marcador, hints e trace cross-file.
Não se exige igualdade de ANSI quando `--color=never`; o modo colorido é medido
separadamente e preserva a política de ativação já existente.

## 2. Medição anterior à decisão

Proveniência da baseline:

- medição `2026-08-23T21:29:41.436043+00:00`;
- HEAD `781b207b4a5de9c2bfbe5819918a193d1d9293e5`;
- working tree não commitada: `271 files changed, 12130 insertions(+),
  4697 deletions(-)`, além de ficheiros não rastreados;
- reprodução: `python3 lab/parity/matrix/runner.py --case <ID>`.

### 2.1 Memória da ADR-0045: quando e por que foi escrita

A ADR-0045 foi escrita em **2026-04-23**, no **Passo 111**, e promovida a
`EM VIGOR` em 111.E. O contexto daquele momento deve ser preservado antes de
qualquer revisão:

- o canal Sink → L3 recém-aberto pelo Passo 106 imprimia literalmente
  `warning: Span(8796093022226) ...`;
- `Span(N)` era um identificador interno do nó da AST, sem significado para o
  utilizador;
- o output não continha ficheiro, linha, coluna nem os hints que já existiam
  em `SourceDiagnostic`;
- ainda não havia CLI real — ela só seria materializada no Passo 113;
- o modelo era single-source e não conseguia resolver spans cross-file;
- migrar o formatter vanilla implicaria, naquele enquadramento, trazer o
  `codespan-reporting` e um conjunto maior de mecânica ainda ausente.

A decisão gcc/clang resolveu essa lacuna com escopo deliberadamente pequeno:

1. L1 passou a resolver `Span → (linha, coluna)` sem introduzir apresentação;
2. a camada de apresentação passou a mostrar path, posição, severidade,
   mensagem e todos os hints;
3. o formato escolhido era reconhecível por humanos e parseável por editores;
4. não foram inventadas posições para spans detached;
5. trace, multi-source e paridade visual vanilla ficaram explicitamente fora
   do escopo.

Portanto, ADR-0045 não foi um acidente nem uma preferência estética arbitrária.
Ela foi uma solução incremental para substituir output opaco e preservar
integração com ferramentas, sob a doutrina então vigente da ADR-0033:
**paridade funcional, não visual**. ADR-0046 reutilizou essa escolha na primeira
CLI; ADR-0048 adicionou cores sem mudar o texto; ADR-0049/0050 corrigiram apenas
a camada dona do formatter, mantendo o comportamento.

A questão de P1139 não é apagar essa decisão retroativamente. É medir se, após
ADR-0107 e o baseline ratificado `a51e02804`, a apresentação do diagnóstico
público passou a ser parte do observável da linguagem e, em caso afirmativo,
como preservar a motivação original de integração com editores ao mudar o
default. A revisão deve registar tanto a razão histórica quanto a razão nova.

### 2.2 Formato principal

Vanilla, entrada `#let x = (`:

```text
error: unclosed delimiter
  ┌─ lab/parity/matrix/fixtures/invalid.typ:1:9
  │
1 │ #let x = (
  │          ^
```

Cristalino:

```text
/repos/Antigravity/typst-crystalline/lab/parity/matrix/fixtures/invalid.typ:1:9: error: unclosed delimiter
```

Fonte vigente:

- `00_nucleo/prompts/shell/diagnostic.md` exige explicitamente formato
  gcc/clang e a assinatura pública atual;
- `02_shell/src/diagnostic.rs:39-82` materializa a linha única;
- o formato vanilla está em
  `lab/typst-original/crates/typst-kit/src/diagnostics.rs`.

Conclusão: não é bug interno acidental. É uma possível mudança de critério entre
uma decisão incremental válida no seu contexto (ADR-0045/L0) e o alvo atual de
paridade pública. Alterar sem demonstrar essa mudança, preservar a motivação
original e revisar a decisão seria deriva arquitetural.

### 2.3 Mensagem e hint

Para `#unknown-p1138`, ambos apontam `1:1` e oferecem o mesmo conselho
semântico sobre espaços na subtração, mas:

- vanilla: `unknown variable \`unknown-p1138\``;
- cristalino: `unknown variable: unknown-p1138`;
- vanilla apresenta `= hint:` dentro do bloco visual;
- cristalino apresenta `  hint:` depois da linha gcc.

Quoting, pontuação e apresentação são observáveis do diagnóstico. A origem da
mensagem deve ser corrigida no produtor L1 somente se a medição `file:line`
confirmar que o formatter não consegue realizar a transformação sem conhecer a
classe da mensagem. Não aplicar substituição textual geral em L2.

### 2.4 Trace cross-file

O domínio já contém `Tracepoint::Include(String)` em
`01_core/src/entities/source_result.rs`. O formatter já sabe escrever
`while including`, mas recebe somente um `&Source`.

Em `P1138-S-003`:

- o diagnóstico principal pertence a `broken.typ`;
- o span do trace pertence a `import-error.typ`;
- `drain_to_stderr` escolhe o source do diagnóstico principal;
- `format_diagnostic` tenta resolver também o trace nesse mesmo source e o
  omite quando `span_to_line_col` falha.

Conclusão: o trace não está ausente do modelo; a fronteira L4→L2 perde a
capacidade de resolver vários `FileId`. Corrigir por resolver cada span no
`SystemWorld`, sem mover I/O ou conhecimento de world para L2.

## 3. Decisão arquitetural a redigir

### P1139-A — revisão ADR/L0

Antes dos testes de produção:

1. redigir adendo à ADR-0045 que primeiro preserve o contexto de 2026-04-23 e
   depois demonstre por que o critério atual exige — ou não — substituir o
   formato gcc/clang como default pelo formato humano vanilla-espelhado;
2. atualizar `00_nucleo/prompts/shell/diagnostic.md` com o contrato completo;
3. atualizar `00_nucleo/prompts/wiring.md` para a resolução cross-file;
4. atualizar `00_nucleo/prompts/entities/source-result.md` apenas se a medição
   provar necessidade de mudar mensagem/trace no domínio;
5. medir consumidores e integrações que dependam do formato gcc/clang e
   registrar explicitamente se ele é removido, preservado por compatibilidade
   ou reservado para futura flag — não inventar flag neste passo sem decisão
   própria;
6. ressellar hashes somente depois da confirmação humana.

**Executado em P1139-A:** auditoria registada em
`00_nucleo/diagnosticos/auditoria-diagnosticos-p1139.md`; adendo acrescentado à
ADR-0045; L0 de `shell/diagnostic`, `wiring` e o índice `shell` atualizados.
`entities/source-result` foi medido e não muda: hints permanecem sem span e o
modelo de trace já contém os dados necessários. A proposta usa
`codespan-reporting 0.11.1`, igual ao vanilla ratificado. Gate aguarda
confirmação humana antes do RED e do resselo dos headers de código.

Este ponto é **paragem obrigatória** da ADR-0127: muda o output por defeito e a
assinatura pública usada por L4. O humano confirma ADR + L0 antes de RED.

### P1139-B — fronteira de fontes do formatter

Após aprovação, L2 deve receber dados suficientes para resolver:

- o span principal;
- cada span de hint, quando o modelo passar a possuí-los;
- cada tracepoint, mesmo quando pertence a outro `FileId`.

A solução mantém as responsabilidades:

- L1: severidade, mensagem, hints, spans e tracepoints puros;
- L2: estrutura visual, palavras, cores e snippets;
- L3: filesystem e `SystemWorld`;
- L4: composição e resolução `FileId → Source + path`.

L2 não importa `SystemWorld`, não lê ficheiros e não recebe callback que faça
I/O oculto. L4 materializa previamente as fontes necessárias e entrega uma
vista read-only ao formatter. A API antiga pode permanecer como wrapper apenas
se houver consumidor real e teste; não manter compatibilidade hipotética.

### P1139-C — render humano vanilla-espelhado

O formatter sem cores produz, nesta ordem:

1. `<severity>: <message>`;
2. localização com ficheiro, linha e coluna;
3. linha de separação `│`;
4. número e texto da linha fonte;
5. marcador `^`/faixa alinhado por coluna visual;
6. hints na ordem original;
7. tracepoints na ordem original, com localização e snippet da fonte dona.

Requisitos:

- tabs e Unicode não podem desalinhar o marcador; medir coluna visual do
  vanilla antes de escolher fórmula;
- spans multi-linha seguem a política medida do vanilla, não truncamento
  inventado;
- span detached possui fallback explícito e testado;
- paths relativos/absolutos seguem exatamente o valor entregue por L4;
- nenhum ANSI em `--color=never`;
- `--color=always` preserva texto e estrutura quando ANSI é removido.

### P1139-D — produtores de mensagens

Depois de o formatter estar correto, remedir `P1138-S-002`. Se a única
divergência restante for `unknown variable: name` versus
`unknown variable \`name\``, localizar o produtor exato e atualizar primeiro o
L0 do módulo dono. Esta correção segue fluxo contínuo apenas se for fórmula
interna de uma mensagem já contratada; se introduzir classificação pública ou
alterar contrato, parar novamente.

Não usar regex no formatter para reescrever mensagens arbitrárias.

## 4. Sequência RED → GREEN

1. Congelar snapshots sem cor dos três casos P1138.
2. RED unitário L2 para bloco fonte/caret.
3. RED de integração para sintaxe inválida.
4. Implementar o renderer humano em L2.
5. RED cross-file demonstrando que o trace span pertence ao chamador.
6. Implementar a vista multi-source L4→L2.
7. Remedir nome desconhecido; corrigir o produtor somente se necessário.
8. Adicionar casos de tab, Unicode, span multi-linha, detached, hint múltiplo
   e `--color=always` com stripping ANSI.
9. Rodar a matriz P1138 integral; nenhuma tolerância nova.

## 5. Testes obrigatórios

### L2 unitários

- erro de uma linha com caret na coluna correta;
- warning e error usam cabeçalhos distintos;
- dois hints preservam ordem;
- tab antes do span;
- texto Unicode antes do span;
- span multi-linha;
- span detached;
- trace `Call`, `Show`, `Import` e `Include`;
- trace cuja fonte difere da fonte principal;
- colorido, após remover ANSI, preserva o texto sem cor.

### Integração CLI

- `P1138-S-001` — sintaxe inválida;
- `P1138-S-002` — nome desconhecido e hint;
- `P1138-S-003` — include cross-file com `while including`;
- erro no main continua apontando o main;
- erro no import aponta o ficheiro importado;
- stdout permanece vazio em erro;
- exit code permanece 1 para erro de compilação.

## 6. Critérios de aceitação

- [x] ADR-0045 foi reconciliada com o novo default, sem decisão contraditória.
- [x] O adendo preserva data, problema original, restrições e benefícios da
      ADR-0045; não a reclassifica retroativamente como erro.
- [x] O impacto sobre consumidores do formato gcc/clang foi medido antes da
      decisão de removê-lo ou mantê-lo.
- [x] L0 de diagnostic e wiring foi aprovado e ressellado antes do RED.
- [x] L2 continua puro e sem conhecimento de `SystemWorld`.
- [x] L4 resolve todos os sources antes de formatar.
- [x] Mensagens não são reescritas por regex genérica.
- [x] Bloco, localização, source, caret, hints e traces coincidem nos casos.
- [x] `P1138-S-001`, `P1138-S-002` e `P1138-S-003` passam a `MATCH`.
- [x] A matriz integral não regride nenhum dos 12 MATCH anteriores.
- [x] Testes L2 e CLI passam.
- [x] `cargo build` passa.
- [x] `cargo test --manifest-path lab/parity/Cargo.toml` passa.
- [x] `crystalline-lint .` não possui violações bloqueantes.
- [x] `git diff --check` passa.
- [x] Relatório registra HEAD, working tree, horário e lista integral não-MATCH.

## 7. Limites

- Não corrige SVG, geometria, raster ou PDF (`P1141..P1144`).
- Não implementa JSON/SARIF nem nova flag de formato diagnóstico.
- Não altera semântica da linguagem para imitar apenas a aparência de erro.
- Não lê `00_nucleo/context/` ou `00_nucleo/materialization/` sem path completo
  fornecido pelo dono.

## 8. Encerramento

P1139 foi executado após a confirmação humana do gate ADR-0127. O relatório
reproduzível está em
`00_nucleo/diagnosticos/typst-passo-1139-relatorio.md`. As próximas frentes
permanecem SVG, layout, raster e PDF, sem serem ampliadas por este passo.

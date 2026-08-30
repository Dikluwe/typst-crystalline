# P1181 — sanear a linhagem L0 para prompts 1:1 e Núcleos Tekt 1:N

**Data:** 2026-08-25  
**Estado:** `FASES 0–1 CONCLUÍDAS — ADR-0129 APROVADA`  
**Dependências:** P1179 decisão C; P1180; correção do `tekt-linter`  
**Baseline de escrita:** HEAD `00f402e875956304aa435f749a251f359287e2ba`  
**Classe ADR-0127:** mudança do contrato arquitetural de linhagem; gate humano
obrigatório antes de migrar L0s ou código

## Autoridade arquitetural de entrada

Adotar no `typst-crystalline` a decisão da ADR-0004 do Tekt:

```text
/repos/Antigravity/Tekt/00_nucleo/adr/0004-nucleos-tekt-compartilhados.md
SHA-256: 2ae6e859396442a3e46405987429c16393539e0c0357c968d023ced1503405c2
```

Invariantes normativos:

```text
Prompt L0 materializável 1:1 consumer produtivo
Núcleo Tekt 1:N prompts proprietários
Código → Prompt proprietário → zero ou mais Núcleos Tekt
Código nunca → Núcleo Tekt diretamente
```

Cada Núcleo Tekt deve ter ao menos um prompt consumidor. Núcleo órfão é V26.
Compartilhamento normativo ocorre exclusivamente por Núcleos Tekt declarativos
TOML sob `00_nucleo/prompts/_nuclei/**/*.toml`.

## Objetivo

Sanear o corpus L0 do `typst-crystalline` para que:

1. cada consumer produtivo tenha exatamente um prompt proprietário;
2. cada prompt materializável tenha exatamente um consumer produtivo;
3. invariantes genuinamente compartilhados sejam extraídos para Núcleos Tekt;
4. cada prompt proprietário permaneça completo e legível sozinho;
5. metadata canônica e pins sejam íntegros antes de qualquer resselo global;
6. `crystalline-lint` prove V5, V15 e V26 sem falsos fechamentos.

Não alterar comportamento da linguagem Typst, contrato público do compilador,
fase de pipeline ou mecânica funcional. Este é saneamento de causalidade L0.

## Evidência congelada

Usar como inventário inicial:

```text
00_nucleo/diagnosticos/typst-p1179-auditoria-migracao-hashes-linter.md
00_nucleo/diagnosticos/typst-p1179-manifest-hashes-linter.tsv
```

Manifesto P1179, SHA-256
`67f3ec296c9e8bf54891b1c1a32cd323d8509c6957767a3f87e0135622315a6a`:

- 421 consumers com drift segundo o binário então instalado;
- 336 prompts;
- 22 prompts compartilhados por 107 consumers;
- 23 prompts sem metadata canônica, afetando 78 consumers;
- 7 prompts combinam compartilhamento e metadata ausente, abrangendo 62
  consumers.

Esses números são baseline histórico, não autorização para correção mecânica.
Reproduzir o inventário com o linter corrigido antes de decidir cada path.

## Restrições

- não acessar/listar `00_nucleo/context/` ou `00_nucleo/materialization/`;
- preservar toda alteração preexistente da working tree;
- não executar `--fix-hashes` mutante enquanto houver V15 ou V26;
- não transformar automaticamente um prompt compartilhado em Núcleo;
- não copiar integralmente o mesmo prompt para vários owners;
- não apontar código diretamente para `_nuclei/*.toml`;
- não colocar `Hash do Código`, consumer list ou ownership num Núcleo;
- não usar múltiplos `Hash do Código` para representar `1:N`;
- não misturar saneamento documental com alteração funcional;
- não aplicar os 421 hashes num único lote;
- não stagear ou commitar sem autorização explícita.

## Fase 0 — proveniência e capacidade da ferramenta

Registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
which crystalline-lint
stat /home/dikluwe/.cargo/bin/crystalline-lint
sha256sum /home/dikluwe/.cargo/bin/crystalline-lint
```

Confirmar na versão instalada, por testes ou fonte pinada, que:

- cardinalidade Prompt→consumer diferente de 1:1 é diagnosticada antes de
  writes;
- V26 reconhece Núcleos TOML, pins, DAG, órfãos e uso direto por código;
- `--fix-hashes` bloqueia diante de V15/V26 estruturais;
- metadata ausente ou inválida não causa escrita parcial;
- plano rejeitado termina com exit code não zero;
- reparo de pins e hashes é transacional ou possui rollback comprovado.

Se qualquer item falhar, parar e retornar ao P1180. Não usar a árvore do
produto como fixture mutante.

## Fase 1 — adoção local e gate ADR-0127

Redigir `00_nucleo/adr/typst-adr-0129-nucleos-tekt-l0-compartilhado.md`.
A ADR local deve:

- registrar a proveniência e o SHA-256 da ADR-0004 do Tekt;
- declarar Prompt L0 ↔ consumer produtivo como relação exatamente 1:1;
- declarar Núcleo Tekt → prompts como relação 1:N, sem ownership de código;
- adotar namespace, TOML 1.0, `tekt = 1`, `kind = "nucleus"`, claims
  `must`/`must-not`/`may`, DAG e SHA-256 completo;
- exigir ao menos um prompt consumidor por Núcleo;
- distinguir Núcleo, ADR, diagnóstico, passo e Prompt L0;
- decidir V15/V26 como gates bloqueantes antes de reparo;
- declarar que prompts proprietários continuam completos no seu escopo e não
  podem terceirizar identidade, ownership ou comportamento específico;
- definir rollback da adoção caso o linter pinado não reproduza o contrato.

Atualizar `AGENTS.md` e somente a documentação arquitetural necessária para
incorporar a nova categoria. Não migrar ainda nenhum dos 22 prompts.

**PARAR.** Apresentar ADR-0129, paths modificados e hashes ao humano. Só
prosseguir após aprovação explícita e resselo aplicável. Esta paragem é
obrigatória porque muda o contrato de linhagem do projeto.

## Fase 2 — inventário reprodutível posterior ao gate

Após aprovação, executar dry-run duas vezes e produzir um novo diagnóstico e
manifesto, sem sobrescrever P1179:

```text
00_nucleo/diagnosticos/typst-p1181-inventario-bijecao-l0.md
00_nucleo/diagnosticos/typst-p1181-inventario-bijecao-l0.tsv
```

Colunas mínimas do TSV:

```text
prompt | consumer | consumer_count | prompt_metadata_count |
conteudo_especifico | claims_compartilhadas | classe | destino | decisão
```

Reconciliar explicitamente os 22 prompts históricos:

| Consumers | Prompt compartilhado |
|---:|---|
| 28 | `compiler/atomizacao_elementos.md` |
| 16 | `compiler/layout.md` |
| 10 | `compiler/eval.md` |
| 7 | `compiler/parse.md` |
| 5 | `compiler/lang.md` |
| 5 | `entities/f_fronteira_e1.md` |
| 4 | `compiler/lexer/mod.md` |
| 4 | `compiler/stdlib/primitives-constructors.md` |
| 2 | `compiler/math/layout/_comum.md` |
| 2 | `compiler/stdlib/context.md` |
| 2 | `compiler/stdlib/state.md` |
| 2 | `compiler/stdlib_audit_methodology.md` |
| 2 | `entities/page_canvas.md` |
| 2 | `entities/page_running.md` |
| 2 | `infra.md` |
| 2 | `infra/export/builder.md` |
| 2 | `infra/package_downloader.md` |
| 2 | `infra/shaper.md` |
| 2 | `shell/cli.md` |
| 2 | `shell/info.md` |
| 2 | `testing/math_oracle.md` |
| 2 | `wiring.md` |

Para cada prompt, ler o documento vigente e todos os consumers antes de
classificar. A medição `file:line` deve preceder a decisão.

## Fase 3 — classificação semântica obrigatória

Cada relação compartilhada recebe exatamente uma classe:

### A — falso compartilhamento

O documento contém contratos específicos de vários consumers misturados.

**Destino:** criar um Prompt L0 proprietário por consumer, distribuindo as
cláusulas específicas. Não criar Núcleo apenas para reduzir duplicação textual.

### B — prompt-owner mais claims comuns

Existe um consumer que é realmente o owner principal, mas parte das cláusulas
é normativa para outros prompts.

**Destino:** manter um prompt proprietário para o owner, criar prompts próprios
para os demais consumers e extrair somente as claims genuinamente comuns para
um Núcleo Tekt consumido por dois ou mais prompts.

### C — documento inteiramente transversal

O documento não materializa diretamente consumer algum; contém apenas
invariantes compartilhados.

**Destino:** converter seu núcleo normativo em Núcleo Tekt e criar prompts
proprietários completos para todos os consumers atuais. Remover o `.md`
compartilhado somente depois de zero referências e prova de conteúdo
preservado.

### D — associação incorreta

O consumer aponta para documento metodológico, teste, diagnóstico, agregador
ou outra categoria que não o legitima.

**Destino:** criar ou localizar o Prompt L0 proprietário correto. Migrar o
documento antigo para sua categoria própria quando necessário; não convertê-lo
automaticamente em Núcleo.

Toda inferência deve declarar o que a refutaria. Em dúvida entre owner e
invariante compartilhado, parar para decisão humana.

## Fase 4 — desenho dos Núcleos

Antes de criar qualquer TOML, demonstrar no inventário:

- ao menos dois prompts proprietários consumidores reais;
- conjunto atômico de claims compartilhadas;
- inexistência de ownership, algoritmo específico ou inventário de consumers
  no conteúdo extraído;
- benefício sobre manter a obrigação em apenas um prompt;
- path lógico e `id` estáveis;
- ausência de ciclo no DAG proposto.

Cada Núcleo deve usar schema fechado e claims pequenas. Exemplo estrutural:

```toml
tekt = 1
kind = "nucleus"
id = "<id-estavel>"
title = "<título>"

[[claims]]
id = "<claim-estavel>"
level = "must"
statement = "<obrigação verificável>"
```

Não criar Núcleo para texto explicativo, histórico, métricas, decisões já
capturadas por ADR ou detalhes pertencentes a um único consumer.

## Fase 5 — materialização em lotes pequenos

Executar lotes por família, nunca todos os 107 consumers juntos. Ordem
recomendada, do menor para o maior:

1. prompts com 2 consumers;
2. prompts com 4–5 consumers;
3. `parse.md` e `eval.md`;
4. `layout.md`;
5. `atomizacao_elementos.md` por último.

Para cada lote:

1. congelar paths, hashes e estado da árvore;
2. escrever/atualizar primeiro os Prompts L0 proprietários;
3. criar Núcleo somente se aprovado pela classificação B/C;
4. inserir pins efetivos completos nos prompts consumidores;
5. parar no gate ADR-0127 se surgir qualquer mudança de contrato público,
   default, fase ou compatibilidade não prevista;
6. atualizar headers `@prompt` para estabelecer bijeção;
7. ressellar somente os paths do lote com manifest explícito;
8. executar V5/V15/V26 e testes focais;
9. provar zero alteração funcional fora de metadata/estrutura L0;
10. emitir diagnóstico do lote antes de iniciar o seguinte.

Se um consumer exigir mudança funcional porque o prompt anterior era
insuficiente, retirar esse consumer do saneamento e abrir passo de nucleação
próprio com RED→GREEN.

## Fase 6 — metadata canônica

Depois de estabelecer 1:1, reconciliar os 23 prompts históricos sem metadata.

- prompts substituídos/divididos recebem metadata apenas no novo owner 1:1;
- Núcleos não recebem `Hash do Código`;
- prompts singleton preservados recebem exatamente uma linha canônica na
  posição definida pelo linter;
- duplicata ou formato malformado bloqueia o lote;
- comparar independentemente o hash de cada consumer ignorando sua própria
  linha `@prompt-hash`.

Não corrigir metadata ausente antes de decidir ownership: isso ressellaria uma
relação inválida e produziria falsa legitimidade.

## Fase 7 — validação final e migração de hashes

Somente quando V15=0 e V26=0 em toda a árvore:

1. executar `--fix-hashes --dry-run` duas vezes;
2. exigir logs byte-idênticos e manifest sem duplicatas;
3. verificar que cada ação toca exclusivamente um par Prompt/consumer 1:1 ou
   pin de Núcleo previsto;
4. executar reparo em lotes manifestados, nunca por descoberta irrestrita;
5. reexecutar dry-run e exigir `Nothing to fix`;
6. executar lint com `--fail-on warning` para V5/V15/V26;
7. validar build e testes proporcionais aos paths cujos headers mudaram;
8. executar `git diff --check` e provar índice vazio.

Gates finais mínimos:

```text
cargo build
cargo test
crystalline-lint --checks v5,v15,v26 --fail-on warning .
crystalline-lint --fix-hashes --dry-run .
git diff --check
git diff --cached --quiet
```

Se o CLI instalado não suportar V26 ou transação comprovada, parar; não usar
uma sequência manual de `sed`, scripts ou writes parciais como substituto.

## Entregáveis

- ADR-0129 de adoção local, aprovada antes da migração;
- atualização mínima de `AGENTS.md` e documentação arquitetural;
- inventário P1181 com classificação dos 22 prompts e 107 consumers;
- Prompts L0 proprietários 1:1;
- zero ou mais Núcleos Tekt justificados, nunca artificiais;
- diagnósticos por lote com proveniência e hashes;
- reconciliação dos 23 prompts sem metadata;
- manifesto final de reparo;
- relatório final com V5/V15/V26, testes, build e preservação da árvore.

## Critérios de aceitação

- ADR-0004 pinada e adotada localmente por ADR-0129;
- gate ADR-0127 respeitado antes de mudar o corpus L0;
- 22/22 prompts compartilhados classificados com evidência;
- 107/107 consumers têm destino explícito;
- relação Prompt L0 ↔ consumer produtivo exatamente 1:1;
- nenhum código aponta para Núcleo;
- todo Núcleo tem ao menos um prompt consumidor e DAG válido;
- claims compartilhadas não foram copiadas entre prompts;
- nenhum Núcleo contém ownership ou `Hash do Código`;
- metadata canônica exatamente uma por prompt materializável;
- V5=0, V15=0 e V26=0 após reparo;
- segunda passagem realmente vazia;
- nenhuma alteração funcional incorporada silenciosamente;
- working tree preexistente preservada e índice vazio.

## Paragens obrigatórias

1. Após redigir ADR-0129 e atualizar a arquitetura local.
2. Após classificar os 22 prompts, antes de criar Núcleos ou dividir L0s.
3. Diante de qualquer dúvida entre claim transversal e contrato proprietário.
4. Diante de qualquer mudança funcional descoberta durante o saneamento.
5. Antes do primeiro reparo mutante global de hashes.

## Próximo passo condicionado

Executar somente a Fase 0 e a Fase 1. Após aprovação da ADR-0129, escrever um
passo de auditoria dedicado à Fase 2–3; não iniciar imediatamente a migração
dos 107 consumers.

## Resultado parcial

Fase 0 executada em 2026-08-25. O binário corrigido foi construído da fonte do
`tekt-linter` em `cc357d9c3bfa26a4ce1bd71c72bc9cba5b3b027c`, cuja suíte terminou
com exit 0, e reinstalado com SHA-256
`b1521dbdc85d0218dee60a9710bb1b1de011c6039fa2c6adf525b1f30e78d3cf`.
No HEAD atual, V15/V26 encontrou 24 colisões V15; o dry-run de reparo terminou
com exit 2 antes de qualquer write.

Fase 1 redigiu a ADR-0129 como `PROPOSTA`, atualizou `AGENTS.md` e registrou a
proveniência em
`00_nucleo/diagnosticos/typst-p1181-fase0-capacidade-linter.md`. Execução
interrompida no gate ADR-0127. Nenhum Prompt L0 proprietário, Núcleo, header,
hash, configuração do linter ou código produtivo foi alterado.

O dono aprovou a ADR-0129 em 2026-08-25. A ADR passou a `EM VIGOR`, as
cláusulas de `AGENTS.md` foram ativadas e V15/V26 foram registradas como
`error` em `crystalline.toml`. A aprovação não iniciou as Fases 2–7: os 24
conflitos V15 atuais permanecem dívida inventariada, e nenhum prompt, Núcleo,
header ou código produtivo foi migrado.

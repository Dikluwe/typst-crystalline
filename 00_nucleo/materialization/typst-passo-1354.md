# Passo 1354 — retificação do L0 e retorno ao produto

## Problema

Desde P1266–P1275, com inflexão em P1293, parte dos Prompts L0 passou a
incorporar mecânica de execução: agentes, preseal, recibos, mutation score,
vereditos de sessão, autorização e sucessão de passos. Isso mistura entidades
que a arquitetura separa:

```text
Prompt L0   = obrigação durável do consumer
Skill       = método proporcional de execução
Diagnóstico = medição de um estado
Passo       = coordenação temporária
```

## Objetivo

Restaurar `00_nucleo/prompts/` à função arquitetural sem mudar o produto:

1. manter semântica, interfaces, ownership, Núcleos Tekt e testes substantivos;
2. retirar cerimônia, estado histórico e evidência de sessão;
3. resselar somente os consumers cujos L0 mudarem;
4. fechar a limpeza neste passo e retomar lacunas observáveis de paridade.

## Estado medido

Em `2026-09-12`, no HEAD `ed4da535a3a2d554c981cf20af81fec41d2a410a`,
uma busca lexical encontrou 123 prompts candidatos. O número não é meta de
redução: palavras como `oráculo` e `mutante` também têm usos legítimos.

P1293, commit `67b5feae6811ba8bc07df7e7d7ccb4e9324d4c1c`, é a principal
inflexão observada: 32 prompts tocados, 3.387 linhas adicionadas em L0 e 37.763
linhas diagnósticas. A auditoria cobre primeiro esse conjunto e depois o
namespace completo.

## Regime

Esta é correção documental e mecânica de linhagem. Não aplicar o protocolo
completo da skill Tekt, criar papéis segregados, executar mutation testing ou
exigir matriz bilateral. ADR-0127 permite fluxo contínuo porque não mudam API,
default, compatibilidade ou fase do pipeline.

## Classificação por cláusula

### A — obrigação durável: manter

Semântica, sintaxe, morfologia, interface, ownership, pureza, topologia,
diagnóstico observável e propriedades reais do consumer.

### B — contrato durável de teste: manter e deshistoricizar

Entradas, observáveis, controles e condições de sucesso podem permanecer.
Remover autoria, ordem entre agentes, recibos, selos e resultados passados
convertidos em obrigação futura.

### C — metaprocesso: retirar do L0

Preseal, certificado, veredito de materialização, agentes, isolamento de
contexto, mutation score universal, autorização de sessão e ordem ritual de
execução pertencem à skill ou ao passo que a invoca.

### D — história: retirar sem transcrever em massa

Datas, commits antigos, resultados de candidatos, nomes de passos e artefatos
temporários pertencem ao Git ou a diagnóstico já existente. Só permanece a
proveniência indispensável para interpretar baseline pinado ou constante.

## Execução

1. Gerar em `/dev/shm/` inventário lexical e histórico dos prompts candidatos.
2. Ler cada ocorrência no contexto do consumer; não editar por palavra-chave.
3. Converter descobertas válidas em obrigações presentes e concisas.
4. Remover cláusulas C/D e prompts de teste que descrevam cerimônia em vez do
   comportamento efetivamente verificado.
5. Não alterar Rust produtivo; no código só podem mudar headers de linhagem.
6. Validar V15/V26 e executar `crystalline-lint --fix-hashes .`.
7. Executar `cargo check --workspace --locked`, `crystalline-lint .` e
   `git diff --check`.

## Evidência permitida

Criar somente `00_nucleo/diagnosticos/p1354-retificacao-l0.md`, com no máximo
400 linhas: proveniência, prompts alterados por classe, candidatos mantidos,
gates finais e próxima lacuna do produto. Não criar JSONs, recibos, manifests,
certificados ou cópias de diff.

## Encerramento

O passo termina quando o L0 descreve o que cada consumer garante sem depender
de P1293 ou de uma sessão anterior; semântica pública permanece inalterada;
nenhum código muda além da linhagem; V5/V15/V26 ficam a zero; e o próximo
trabalho volta a nascer de lacuna observável do produto.

Uma ocorrência lexical legítima não gera sucessor. Uma mudança pública real
descoberta durante a auditoria é retirada do escopo sem bloquear o restante.

## Retorno ao prumo

```text
medição da linguagem → L0 conciso → RED → implementação → GREEN
```

Segregação, ataques e mutação são ferramentas proporcionais ao risco, não
fases obrigatórias da arquitetura. Diagnósticos comprovam decisões atuais;
não legitimam código nem geram trabalho por existirem.

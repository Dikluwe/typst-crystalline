# P1180 — corrigir a cardinalidade e a atomicidade de `fix-hashes`

**Data:** 2026-08-25  
**Estado:** `ESCRITO — NÃO EXECUTADO`  
**Origem:** P1179, decisão C  
**Repositório de execução:** `/repos/Antigravity/tekt-linter`  
**Classe ADR-0127:** contrato público de linhagem e comportamento de comando;
gate L0 obrigatório antes de código

## Decisão do dono

A relação arquitetural correta é estritamente:

```text
1 Prompt L0 ↔ 1 consumer materializado
```

Um documento necessário a vários consumers (`1:N`) deixa de ser Prompt L0.
Esse conteúdo exige uma categoria documental diferente, com nome, diretório,
contrato e tratamento pelo linter definidos explicitamente antes da migração.
Não representar `1:N` escolhendo um consumer, acumulando vários
`Hash do Código` no prompt ou tornando a metadata ambígua.

## Objetivo

Corrigir o contrato e a implementação do linter para que:

1. cardinalidade de Prompt L0 diferente de `1:1` seja detectada antes de
   qualquer escrita;
2. `--fix-hashes` seja transacional no nível lógico do lote: ou todo o plano é
   válido antes da primeira mutação, ou nenhum ficheiro é alterado;
3. metadata canônica ausente, duplicada ou malformada nunca produza
   `Partial write` invisível numa segunda passagem;
4. documentos transversais `1:N` não sejam tratados como prompts nem recebam
   `Hash do Código` escalar;
5. a análise posterior valide os dois sentidos da linhagem, não apenas a
   ausência de V5 no header do consumer.

Este passo corrige primeiro o linter. Não migrar os 421 consumers do
`typst-crystalline` e não normalizar os 22 prompts compartilhados neste passo.

## Evidência de entrada

Ler antes de decidir:

```text
/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/typst-p1179-auditoria-migracao-hashes-linter.md
/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/typst-p1179-manifest-hashes-linter.tsv
/repos/Antigravity/typst-crystalline/typst-passo-1179.md
```

Baseline P1179, medido sobre `typst-crystalline` HEAD
`ce49041de76eb64c011e990e9774a0339f979211`:

- 421 consumers com V5;
- 336 prompts referenciados;
- 22 prompts compartilhados por 107 consumers;
- todos os 22 têm `hash-b` distinto por consumer;
- 23 prompts sem metadata canônica, afetando 78 consumers;
- 7 prompts combinam as duas condições, abrangendo 62 consumers;
- numa fixture, `fix-hashes` escreveu primeiro o source, falhou depois no
  prompt e terminou com exit 0;
- no caso compartilhado, duas escritas sucessivas deixaram o `hash-b` do
  último consumer;
- a segunda passagem respondeu `Nothing to fix` e o lint com
  `--fail-on warning` terminou em zero apesar dos estados inválidos.

## 1. Proveniência e preservação

No repositório do linter, registrar antes de alterações:

```text
date --iso-8601=seconds
git rev-parse HEAD
git branch --show-current
git status --short
git diff HEAD --stat
git diff --cached --stat
sha256sum /home/dikluwe/.cargo/bin/crystalline-lint
```

Não assumir árvore limpa. Preservar alterações preexistentes, não stagear
ficheiros alheios e não usar `reset`, `checkout`, `restore` ou limpeza global.
Todo número decisório deve carregar HEAD, estado da árvore e hora quando
necessário.

## 2. Auditoria L0 obrigatória

Ler integralmente os L0s e ADRs vigentes que governam:

- `fix-hashes` e V5;
- parser e escrita de metadata canônica;
- descoberta de prompts e consumers;
- formato de `Hash do Código` e `@prompt-hash`;
- comportamento e exit code do CLI;
- atomicidade/rollback, se já houver contrato.

Medir na fonte, com `file:line`:

- onde o plano é construído por violação;
- onde o source é escrito antes do prompt;
- onde metadata ausente/duplicada é rejeitada;
- onde a reanálise conclui “zero drift” olhando apenas V5;
- se já existe regra de cardinalidade ou categoria documental transversal;
- todos os testes que hoje legitimam o comportamento observado.

Classificar explicitamente intenção de contrato versus comportamento acidental.
Não inferir que o comportamento atual é correto porque existe ou porque termina
com exit 0.

## 3. Redigir L0 e parar no gate ADR-0127

Atualizar primeiro o Prompt L0 correspondente, ou criar os L0s necessários,
para especificar no mínimo:

### 3.1 Cardinalidade

- Prompt L0 materializado possui exatamente um consumer;
- zero consumers é prompt órfão, salvo exceção explícita já prevista pelo
  contrato;
- mais de um consumer é violação de cardinalidade;
- a violação deve listar o prompt e todos os consumers em ordem
  determinística;
- `fix-hashes` não corrige nem escolhe automaticamente um consumer em `1:N`.

### 3.2 Categoria documental `1:N`

Definir, sem deixar implícito:

- nome canônico da nova categoria;
- diretório permitido;
- finalidade: norma, referência ou contrato transversal não materializado
  diretamente por um único source;
- se pode ou não ser citado por headers e, se puder, por qual campo distinto;
- como o walker distingue essa categoria de Prompt L0;
- quais regras de órfão, stale e drift se aplicam;
- proibição de `Hash do Código` escalar nessa categoria;
- estratégia futura para migrar documentos atualmente compartilhados.

Não reutilizar `@prompt` para dois significados. Se a categoria não legitima
diretamente código, ela não participa da linhagem Prompt L0 ↔ consumer.

### 3.3 Preflight e atomicidade

Antes da primeira escrita, o plano completo deve validar:

- headers legíveis;
- prompts existentes e confinados;
- cardinalidade exatamente `1:1`;
- metadata canônica com multiplicidade válida;
- hashes computáveis;
- ausência de dois writes concorrentes para o mesmo prompt;
- todos os destinos graváveis segundo o contrato.

Qualquer falha de preflight aborta o lote sem mutação. Especificar também a
garantia para falha de I/O ocorrida depois do preflight: escrita atômica do
conjunto ou rollback verificável. Não chamar a operação de atômica se a
garantia cobrir apenas cada ficheiro individualmente.

### 3.4 Metadata ausente

Decidir no L0, sem deixar ao código:

- ou metadata ausente é erro de preflight e exige correção explícita;
- ou `--fix-hashes` está autorizado a inserir exatamente uma linha canônica
  numa posição definida.

Em ambos os casos, duplicata/malformação deve abortar antes de writes. O texto
da porta (“inject”) e a implementação devem concordar.

### 3.5 Resultado e exit code

- lote rejeitado não pode terminar como sucesso silencioso;
- `Partial write` é estado de falha, nunca resultado aceitável;
- a verificação posterior deve comparar header do consumer, hash do L0 e
  metadata de código do prompt;
- `Nothing to fix` só é válido quando a relação bidirecional completa está
  consistente;
- formato de diagnóstico e exit code devem ser testados como observáveis.

Como estas cláusulas alteram contrato público e comportamento do comando,
**parar após redigir e ressellar os L0s**. Registrar paths e hashes novos e
aguardar aprovação humana antes de escrever testes ou implementação.

## 4. RED após aprovação do gate

Somente depois da aprovação explícita, criar fixtures segregadas para:

1. um prompt e um consumer válidos;
2. um prompt referenciado por dois consumers;
3. dois prompts tentando legitimar o mesmo consumer, se o formato permitir a
   entrada;
4. prompt sem metadata canônica;
5. prompt com metadata duplicada;
6. prompt com metadata malformada;
7. prompt órfão com e sem exceção declarada;
8. documento da nova categoria `1:N` corretamente localizado;
9. documento `1:N` usado incorretamente como `@prompt`;
10. falha simulada no segundo write do lote;
11. plano com erro após uma entrada válida;
12. segunda passagem sobre estado integralmente válido.

Confirmar RED substantivo. Os testes devem provar:

- zero bytes alterados quando o preflight falha;
- nenhum “último consumer vence”;
- diagnóstico determinístico contém todos os consumers conflitantes;
- exit code não zero na rejeição;
- ausência de falso `Nothing to fix`;
- idempotência real no caso válido;
- nenhuma regressão nos formatos de output não alterados pelo novo contrato.

## 5. Implementação mínima

Implementar apenas o necessário para os REDs aprovados:

1. construir um índice determinístico `prompt → consumers` antes de executar;
2. validar cardinalidade e metadata no planejamento;
3. separar `PlanValid` de `PlanRejected`, impedindo que entradas rejeitadas
   alcancem qualquer writer;
4. garantir transação ou rollback conforme o L0 aprovado;
5. revalidar a linhagem bidirecional depois da escrita;
6. reconhecer a nova categoria documental sem tratá-la como Prompt L0;
7. remover ou tornar impossível o estado de sucesso com `PartialWrite`.

Não resolver `1:N` com arrays de hashes, múltiplas linhas `Hash do Código`,
ordem de traversal ou escolha de consumer representativo.

## 6. Validação do linter

Executar a suíte focal e depois os gates completos definidos pelo próprio
repositório. Incluir obrigatoriamente:

```text
cargo test
cargo build
crystalline-lint .
git diff --check
git diff --cached --quiet
```

Se o linter não puder autoanalisar-se durante a alteração, registrar o motivo,
o comando exato e o gate substituto; não declarar GREEN apenas pela suíte
focal.

Instalar/republicar o binário somente conforme o processo versionado do
`tekt-linter`, registrando versão, commit e SHA-256 do executável resultante.

## 7. Revalidação externa sem migração

Com o novo binário, voltar a `/repos/Antigravity/typst-crystalline` e executar
somente:

```text
crystalline-lint --fix-hashes --dry-run .
```

Exigir:

- working tree idêntica antes/depois;
- diagnóstico explícito dos 22 prompts `1:N`, até o corpus ser normalizado;
- diagnóstico explícito da metadata ausente conforme o contrato escolhido;
- nenhuma proposta que escolha um dos hashes de consumer para prompt
  compartilhado;
- exit code coerente com o L0 atualizado;
- lista determinística e reproduzível em duas passagens.

Ainda não aplicar os 421 hashes. A normalização do corpus e a migração global
serão passos posteriores e separados.

## Entregáveis no `tekt-linter`

- L0(s) atualizado(s) e hashes ressellados;
- diagnóstico de medição anterior à decisão;
- fixtures RED e testes GREEN;
- implementação da cardinalidade/preflight/atomicidade;
- especificação da nova categoria documental `1:N`;
- relatório de validação e proveniência do binário;
- reprodução externa dry-run no `typst-crystalline`.

## Critérios de aceitação

- invariável `Prompt L0 ↔ consumer = 1:1` explícita no L0;
- categoria `1:N` definida sem reutilizar a semântica de prompt;
- gate ADR-0127 respeitado antes de código;
- conflito `1:N` rejeitado antes de qualquer write;
- metadata inválida nunca produz mutação parcial;
- falha de lote tem exit code não zero;
- verificação posterior é bidirecional;
- fixtures cobrem falha após entrada válida e comprovam zero mutação;
- segunda passagem válida é realmente idempotente;
- binário tem proveniência reproduzível;
- dry-run externo não altera o `typst-crystalline`;
- nenhuma migração global ou normalização do corpus é incorporada neste passo.

## Próximos passos condicionados

1. Após o linter corrigido: repetir P1179 com o novo binário.
2. Escrever um passo separado para classificar os 22 prompts compartilhados
   entre divisão `1:1` e migração para a nova categoria `1:N`.
3. Somente depois de corpus e ferramenta estarem consistentes, escrever o
   passo de aplicação controlada dos hashes restantes.

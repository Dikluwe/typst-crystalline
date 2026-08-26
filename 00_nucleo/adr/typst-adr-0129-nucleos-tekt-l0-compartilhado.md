# ADR-0129 — Prompts L0 proprietários e Núcleos Tekt compartilhados

**Estado:** EM VIGOR — APROVADA NO GATE ADR-0127  
**Data:** 2026-08-25  
**Aprovação do dono:** 2026-08-25  
**Origem:** P1179–P1181

## Autoridade e proveniência

Esta ADR adota localmente a decisão normativa:

```text
/repos/Antigravity/Tekt/00_nucleo/adr/0004-nucleos-tekt-compartilhados.md
SHA-256: 2ae6e859396442a3e46405987429c16393539e0c0357c968d023ced1503405c2
Tekt HEAD observado: fc8449f748c66adf084fdcad3a969343807f7bbe
```

O pin acima identifica os bytes efetivamente auditados. A incorporação desta
ADR ao `typst-crystalline` não importa mecanicamente documentos do repositório
Tekt nem transfere autoridade para alterações futuras não pinadas.

## Contexto medido

P1179 demonstrou que a metadata escalar `Hash do Código` não representa um
prompt usado por vários consumers. O reparador antigo escrevia sucessivamente
o mesmo prompt, conservava o hash do último consumer e podia deixar source e
prompt parcialmente atualizados enquanto a segunda passagem fechava em zero.

O manifesto P1179 mediu, sobre HEAD histórico
`ce49041de76eb64c011e990e9774a0339f979211`, 22 prompts compartilhados por 107
consumers e 23 prompts sem metadata canônica, afetando 78 consumers.

Em 2026-08-25T21:02:41-03:00, sobre HEAD
`00f402e875956304aa435f749a251f359287e2ba`, o binário corrigido do Tekt Linter
mediu 24 colisões globais de ownership. O aumento face aos 22 casos históricos
reflete o estado atual do corpus e a análise global corrigida; o inventário
P1179 permanece evidência histórica, não lista fechada.

O executável corrigido tinha SHA-256
`b1521dbdc85d0218dee60a9710bb1b1de011c6039fa2c6adf525b1f30e78d3cf`,
construído da fonte `/repos/Antigravity/tekt-linter` no commit
`cc357d9c3bfa26a4ce1bd71c72bc9cba5b3b027c`. A suíte completa dessa fonte
terminou com exit 0. No produto:

- `--checks v15,v26 --fail-on warning` terminou com exit 1 e listou as 24
  colisões V15; não encontrou V26 porque ainda não existem Núcleos no corpus;
- `--fix-hashes --dry-run .` terminou com exit 2, bloqueado pelas 24 colisões
  antes de qualquer write;
- o estado da working tree permaneceu inalterado pelos dois comandos.

## Decisão

### 1. Ownership produtivo é biunívoco

Para todo código produtivo em L1–L4:

```text
1 Prompt L0 materializável ↔ 1 consumer produtivo
```

- cada consumer produtivo declara exatamente um `@prompt` proprietário;
- cada Prompt L0 materializável legitima exatamente um consumer produtivo;
- Prompt→Código `1:N` é violação de ownership, não formato alternativo;
- zero consumers é prompt órfão, salvo exceção explícita e não materializável
  já declarada na configuração;
- V15 bloqueia reparo de hashes enquanto a bijeção não estiver íntegra.

Um arquivo de teste produtivo, build script ou entrypoint também precisa de
owner próprio quando estiver nos diretórios estritos; não pode compartilhar o
prompt do módulo apenas por proximidade funcional.

### 2. Compartilhamento normativo usa Núcleo Tekt

Adotar **Núcleo Tekt** como artefato L0 declarativo que compartilha obrigação,
nunca ownership:

- localização: `00_nucleo/prompts/_nuclei/**/*.toml`;
- formato: TOML 1.0 estrito;
- discriminadores obrigatórios: `tekt = 1` e `kind = "nucleus"`;
- conteúdo: claims atômicas `must`, `must-not` ou `may`;
- composição: dependências entre Núcleos formam exclusivamente DAG;
- consumo: um ou mais prompts proprietários referenciam o Núcleo por path
  lógico e SHA-256 efetivo completo;
- cardinalidade: `1:N` prompts; zero consumidores é Núcleo órfão e produz V26;
- código nunca usa Núcleo como `@prompt` nem aponta diretamente para ele;
- Núcleo não contém `Hash do Código`, owner produtivo ou lista de consumers;
- alteração de bytes ou dependências invalida deterministicamente pins e
  hashes efetivos dos prompts consumidores transitivos.

Um prompt pode consumir zero ou mais Núcleos, mas deve permanecer completo e
legível no seu escopo proprietário. Identidade, contrato específico, algoritmo
do consumer e critérios próprios não podem ser terceirizados ao Núcleo.

### 3. Fronteiras documentais

- **Prompt L0:** causa ex-ante e owner de exatamente um consumer;
- **Núcleo Tekt:** obrigação declarativa compartilhada por prompts;
- **ADR:** decisão arquitetural e sua motivação histórica;
- **diagnóstico:** evidência e estado ex-post;
- **passo:** coordenação operacional temporária;
- **relatório:** recibo mínimo do que foi executado.

ADRs, diagnósticos, passos e relatórios não podem ser referenciados como
Núcleos. TOML fora de `_nuclei` não adquire identidade de Núcleo. A extensão
`.tekt` não é suportada.

### 4. Gates da ferramenta

- V15 valida ownership global antes de qualquer reparo;
- V26 valida schema, inventário, pins, DAG, dependências, órfãos e proibição de
  código→Núcleo;
- V15 e V26 são bloqueantes para `--fix-hashes`;
- preflight deve concluir integralmente antes da primeira mutação;
- falha de aplicação exige rollback verificável e exit code não zero;
- `Nothing to fix` exige coerência bidirecional completa, não apenas ausência
  de drift direto no header.

O saneamento ocorre em lotes manifestados. É proibido aplicar reparo global
enquanto houver colisão de ownership ou erro de Núcleo.

## Migração do corpus existente

Cada prompt atualmente compartilhado deve ser medido junto a todos os seus
consumers e classificado antes de alterações:

1. contratos específicos misturados → dividir em prompts proprietários;
2. owner real com claims comuns → individualizar owners e extrair apenas as
   claims comuns para Núcleo;
3. documento inteiramente transversal → converter o núcleo normativo em
   Núcleo e criar prompts proprietários para todos os consumers;
4. associação documental incorreta → criar/localizar o owner correto e mover o
   documento antigo para sua categoria própria.

Não há conversão automática Markdown→TOML. Um Núcleo só é criado quando pelo
menos dois prompts proprietários consomem claims genuinamente comuns.

Metadata canônica ausente só é reparada depois de resolver ownership. Resselar
antes disso legitimaria uma relação inválida.

## Consequências

### Positivas

- `Hash do Código` volta a ter significado único;
- mudanças num consumer não escolhem nem invalidam owners alheios;
- invariantes compartilhados deixam de ser copiados ou escondidos em prompts
  monolíticos;
- V15 prova ownership e V26 prova o grafo declarativo compartilhado;
- reparo de hashes pode ser transacional e reprodutível.

### Custos

- os prompts compartilhados existentes precisam de auditoria semântica;
- o corpus ganhará prompts proprietários adicionais;
- Núcleos exigem schema, pins completos, DAG e atualização transitiva;
- alterações num Núcleo repercutem deliberadamente em todos os prompts
  consumidores.

## Alternativas rejeitadas

- múltiplos `Hash do Código` num prompt;
- hash agregado de vários consumers;
- consumer representativo ou comportamento “último vence”;
- duplicação integral do mesmo contrato entre prompts;
- código apontando diretamente para Núcleo;
- ADR como dependência normativa executável;
- migração automática de todo prompt compartilhado para Núcleo.

## Rollback da adoção

Se o binário pinado deixar de provar V15/V26, preflight integral ou rollback,
o saneamento deve parar antes de novos writes. Prompts já individualizados não
voltam a ser compartilhados: preserva-se a bijeção e remove-se temporariamente
apenas o consumo de Núcleos afetado, restaurando dos manifests os bytes dos
prompts/pins do último lote válido. Uma nova versão da ferramenta exige
proveniência, dry-run duplo e decisão explícita antes de retomar.

## Gate ADR-0127

Esta decisão altera o contrato arquitetural público de linhagem do projeto.
O dono aprovou explicitamente a decisão em 2026-08-25. A partir da aprovação:

- esta ADR está `EM VIGOR`;
- V15 e V26 integram explicitamente a configuração do projeto;
- novos owners e Núcleos devem obedecer às cardinalidades decididas;
- o corpus legado é saneado somente pelos lotes auditados do P1181;
- aprovação da ADR não autoriza por si só reparo global, divisão automática de
  prompts ou alteração funcional.

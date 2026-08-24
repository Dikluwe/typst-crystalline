# Passo 1140 — Inventário verificável da superfície da linguagem

**Data:** 2026-08-23  
**Origem:** backlog produzido por P1138 e encerramento de P1139  
**Vanilla ratificado:** `upstream/main a51e02804`  
**Classe:** diagnóstico de cobertura; não implementa funcionalidades  
**Gate:** nenhuma mudança L1–L4 neste passo

## 1. Objetivo

Construir um inventário reproduzível da superfície pública do Typst ratificado
e compará-lo com o cristalino, cobrindo:

- bindings globais;
- módulos e seus membros;
- funções e construtores chamáveis;
- namespaces anexados a funções;
- parâmetros posicionais, nomeados, variádicos e settable;
- valores por omissão, quando observáveis ou disponíveis na fonte ratificada;
- tipo público do binding e condição mínima de chamada.

O inventário não fecha lacunas nem adiciona reflexão à linguagem. Seu produto
é uma fila classificada e verificável para passos posteriores, evitando que a
cobertura seja inferida apenas por nomes encontrados no código.

## 2. Proveniência da medição inicial

Medição realizada em `2026-08-23T20:56:14-03:00`:

- HEAD cristalino: `fcbc9763f8925d5c27b3670e35597b9adc0412a0`;
- working tree não commitada: `26 files changed, 515 insertions(+), 398
  deletions(-)`, além de três ficheiros P1139 não rastreados;
- fonte vanilla: árvore em `lab/typst-original`, revisão ratificada
  `a51e02804`;
- os números desta secção não são usados para decidir cobertura: registram
  somente o estado exato em que o passo foi redigido.

Reprodução do estado:

```sh
git rev-parse HEAD
git diff HEAD --stat
git status --short
```

## 3. Medição antes da decisão

### 3.1 O vanilla possui um catálogo Rust, não reflexão geral da linguagem

Fonte ratificada:

- `lab/typst-original/crates/typst-library/src/foundations/scope.rs:141-255`:
  `Scope` preserva bindings e oferece `iter()`;
- `lab/typst-original/crates/typst-library/src/lib.rs:357-414`: a biblioteca
  constrói o scope global por grupos e módulos;
- `lab/typst-original/crates/typst-library/src/foundations/func.rs:220-243`:
  `Func::params()` e `Func::param()` expõem metadados internamente em Rust;
- `lab/typst-original/crates/typst-library/src/foundations/func.rs:529-616`:
  `ParamInfo` distingue nome, default, positional, named, variadic, required e
  settable;
- `lab/typst-original/crates/typst-library/src/foundations/func.rs:689+`:
  `NativeParamInfo` é a origem das assinaturas nativas.

Essas APIs não demonstram que um documento Typst possa chamar `params()`.
A ausência dessa reflexão pública impede usar apenas runtime para enumerar
todas as assinaturas. Inferência: o catálogo vanilla deve ser extraído da
fonte/API Rust ratificada e validado por observáveis da linguagem. Refutação:
um probe no binário ratificado que enumere integralmente parâmetros, defaults
e flags usando somente linguagem Typst.

### 3.2 O cristalino enumera nomes, mas não modela assinatura nativa completa

Fonte cristalina vigente:

- `01_core/src/entities/scope.rs:63-96`: `Scope::iter()` permite enumerar os
  bindings existentes e conserva sua ordem;
- `01_core/src/entities/module.rs:29-82`: `Module` expõe nome e scope;
- `01_core/src/entities/func.rs:260-335`: `Func` expõe nome e namespace;
- `01_core/src/entities/func.rs:84-105`: closures carregam parâmetros e default;
- `01_core/src/entities/func.rs:112-139`: funções nativas carregam nome, ponteiro
  de chamada e namespace, mas não `ParamInfo` equivalente;
- `01_core/src/compiler/eval/mod.rs:1555+`: a stdlib cristalina é materializada
  por definições explícitas no scope.

Conclusão: globais, módulos e namespaces podem ser extraídos mecanicamente do
estado construído. Assinaturas nativas não podem ser declaradas completas a
partir de `Func` sem medir os seus consumidores e sem uma futura decisão L0.
Não se deve preencher campos ausentes por adivinhação a partir da implementação
dos corpos.

### 3.3 L0 vigente

Foram lidos antes desta proposta:

- `00_nucleo/prompts/entities/scope.md`;
- `00_nucleo/prompts/entities/module.md`;
- `00_nucleo/prompts/entities/func.md`;
- `00_nucleo/prompts/entities/args.md`;
- `00_nucleo/prompts/compiler/scopes.md`.

Os contratos atuais legitimam iteração de scopes, nomes e namespaces, mas não
legitimam acrescentar metadados às funções nativas. P1140 não altera esses L0.
Qualquer proposta posterior de `ParamInfo`, novo campo público ou reflexão deve
começar por L0 e parar no gate ADR-0127.

## 4. Produtos obrigatórios

P1140 produz somente diagnósticos e fixtures de medição:

1. `00_nucleo/diagnosticos/superficie-linguagem-p1140.json` — catálogo
   canônico, próprio para diff;
2. `00_nucleo/diagnosticos/superficie-linguagem-p1140.md` — síntese humana,
   método, totais com proveniência e fila priorizada;
3. fixtures/probes em `lab/parity/` apenas quando necessários para validar um
   observável público;
4. atualização de `00_nucleo/diagnosticos/paridade-p1138-backlog.md` com os
   passos derivados, sem apagar o estado histórico.

O JSON é dado diagnóstico, não API do produto. Seu schema mínimo por entrada:

```json
{
  "path": "calc.round",
  "kind": "function",
  "vanilla": {
    "present": true,
    "type": "function",
    "params": [],
    "source": "file:line"
  },
  "crystalline": {
    "present": true,
    "type": "function",
    "params": null,
    "source": "file:line"
  },
  "probe": null,
  "classification": "UNVERIFIED_METADATA",
  "inference": null,
  "refutation": null
}
```

`null` significa “não medido ou não representado”; nunca significa lista
vazia. Toda contagem do relatório deve referenciar o JSON, HEAD, working tree e
horário que a geraram.

## 5. Método

### Fase A — congelar o universo vanilla

1. Construir o catálogo a partir do `Library` ratificado e de seus scopes.
2. Percorrer recursivamente módulos e namespaces de função, com detecção de
   identidade/ciclo.
3. Para cada callable, registrar os metadados fornecidos por `Func::params()` e
   a linha da fonte ratificada que os origina.
4. Separar aliases por path: dois paths podem apontar para a mesma função e
   continuam sendo duas entradas públicas, ligadas por identidade apenas como
   metadado mecânico.
5. Não incluir símbolos internos que não estejam alcançáveis pelo scope
   público construído.

### Fase B — extrair a superfície cristalina existente

1. Instanciar a mesma stdlib usada pelo avaliador, sem duplicar uma lista à
   mão.
2. Percorrer `Scope::iter()`, `Module::scope()` e `Func::namespace()`.
3. Registrar `Value::type_name()` ou equivalente como tipo público.
4. Para closures, registrar apenas parâmetros realmente modelados.
5. Para nativas, usar `params: null`; não deduzir assinatura por branches de
   `args.named.get`, índices posicionais ou mensagens de erro.

Se a extração exigir código novo nas camadas L1–L4, parar: P1140 deverá primeiro
redigir o L0 correspondente e obter confirmação humana. Ferramentação isolada
em `lab/` pode consumir APIs públicas existentes, mas não recebe exceção para
importar internals ou alterar visibilidade de produção.

### Fase C — validar por probes observáveis

Para cada divergência ou metadado crítico, selecionar probes mínimos:

- `type(binding)` confirma a classe pública;
- field access confirma membro de módulo/namespace;
- chamada mínima confirma callability;
- chamadas omitindo, nomeando ou posicionando argumentos confirmam required,
  named e positional;
- spread confirma variadic;
- `set`/`show` confirma settable apenas quando o binding é elemento;
- mensagem e exit code distinguem “não existe” de “existe, mas chamada
  inválida”.

O probe valida linguagem, não a estrutura Rust. Um resultado de erro não prova
sozinho a assinatura completa; registra apenas a propriedade que o caso isola.

### Fase D — classificar sem implementar

Cada entrada recebe exatamente uma classe:

- `MATCH` — presença, tipo e propriedades medidas coincidem;
- `MISSING_BINDING` — path público vanilla ausente;
- `WRONG_KIND` — path existe com tipo público diferente;
- `MISSING_MEMBER` — módulo/namespace existe, membro não;
- `EXTRA_BINDING` — path público cristalino sem correspondente no universo
  vanilla ratificado;
- `CALL_MISMATCH` — callable existe, probe isolado diverge;
- `METADATA_ONLY` — linguagem medida coincide, representação interna de
  metadados diverge;
- `UNVERIFIED_METADATA` — fonte sugere propriedade que ainda não foi isolada
  por probe;
- `OUT_OF_SCOPE_RATIFIED` — decisão vigente e citada por ADR/L0;
- `BLOCKED_BY_GATE` — correção exigiria contrato/default/pipeline ou quebra de
  compatibilidade ainda não aprovada.

Não usar `MATCH` apenas porque o nome existe. Não usar `MISSING_BINDING` para
um item deliberadamente excluído pelo target ratificado. Não transformar
`METADATA_ONLY` em dívida de linguagem.

## 6. Ordem e prioridade

O relatório agrupa primeiro por impacto observável:

1. globais ausentes ou com kind errado;
2. módulos/namespaces e membros ausentes;
3. callables presentes com chamadas públicas divergentes;
4. parâmetros required/named/positional/variadic/settable;
5. defaults;
6. metadados apenas internos.

Dentro de cada grupo, ordenar por path e registrar o menor probe que reproduz
a classificação. Prioridade não pode ser derivada somente do número bruto de
lacunas: um binding global ausente pode ter superfície maior que dezenas de
defaults não observados.

## 7. Testes e controles do próprio inventário

- schema JSON válido e ordenação determinística;
- paths únicos;
- todo `source` segue `file:line` e existe no estado medido;
- nenhum `MATCH` contém probe divergente;
- nenhum `null` é contado como zero;
- aliases não são colapsados como se fossem o mesmo path;
- módulos e namespaces cíclicos terminam sem perder paths;
- uma amostra estratificada é reproduzida nos dois binários;
- o inventário executado duas vezes no mesmo estado produz o mesmo JSON;
- fixtures temporárias não contaminam globals nem package paths;
- `git diff --check` passa;
- `crystalline-lint .` continua sem violações bloqueantes.

## 8. Critérios de aceitação

- [x] Catálogo vanilla completo para globais, módulos, callables e parâmetros.
- [x] Catálogo cristalino extraído da construção real da stdlib.
- [x] Todos os registros possuem fonte `file:line` ou justificativa explícita.
- [x] Probes distinguem observável de linguagem de metadado Rust.
- [x] `null`, vazio, ausente e não verificado não são confundidos.
- [x] Classes da Fase D são aplicadas sem tolerância inventada.
- [x] Relatório registra HEAD, working tree, horário, comandos e totais.
- [x] Toda lacuna priorizada aponta para o L0 dono vigente.
- [x] Nenhum código L1–L4 é alterado neste passo.
- [x] Nenhum contrato público novo é proposto como implementação automática.
- [x] Próximos passos são atomizados por módulo/contrato, sem “corrigir a
      stdlib inteira” num único lote.
- [x] Backlog P1141–P1145 permanece separado das descobertas de P1140.

## 9. Limites

- Não implementa `params()` na linguagem nem API pública equivalente.
- Não adiciona campos a `Func`, `NativeFunc`, `Binding`, `Scope` ou `Module`.
- Não altera assinatura, default ou comportamento de callable.
- Não corrige SVG, layout, PNG, PDF ou HTML (`P1141..P1145`).
- Não usa documentação web como substituta do hash ratificado; docs podem ser
  índice auxiliar, nunca oráculo final.
- Não lê `00_nucleo/context/` nem `00_nucleo/materialization/` sem path completo
  fornecido pelo dono.

## 10. Paragens obrigatórias

Parar e pedir confirmação humana se a execução revelar necessidade de:

1. novo campo em entidade ou método público;
2. alterar a construção default da stdlib;
3. expor internals apenas para facilitar o inventário;
4. acrescentar reflexão observável à linguagem;
5. mudar fase do pipeline ou compatibilidade.

Correções futuras de tabela ou fórmula interna só poderão seguir fluxo contínuo
depois de terem passo próprio, L0 atualizado primeiro e teste RED reproduzível,
conforme ADR-0127.

## 11. Encerramento

Executado sem mudanças em L1–L4. Catálogo, probes e conclusões estão em
`00_nucleo/diagnosticos/superficie-linguagem-p1140.json`,
`superficie-linguagem-p1140-probes.json` e
`superficie-linguagem-p1140.md`. As correções descobertas exigem passos
próprios e leitura/atualização do L0 dono antes de código.

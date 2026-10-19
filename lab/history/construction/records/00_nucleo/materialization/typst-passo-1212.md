# P1212 — reconciliação de paridade assistida por mapa vanilla ↔ cristalino

**Estado:** EXECUTADO — MAP PILOT CLOSED; RESIDUAL CLASSIFIED  
**Data da redação:** 2026-08-26  
**Dependências:** P1210 (rebaseline funcional RED), P1211 (método de comparação
segregada) e lente DSM com `--mapa-correspondencia` materializada no working tree
de `tekt-cargo-dsm`.  
**Vanilla autoritativo:** `a51e02804`.  
**Mapa vivo:**
`00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml`.

## 1. Objetivo fechado

Transformar o resíduo mecânico da lente em um inventário acionável de paridade,
distinguindo:

1. item apenas movido ou renomeado;
2. responsabilidade dividida `1:N`;
3. responsabilidades consolidadas `N:1`;
4. grupo `N:M` que precisa ser julgado como unidade;
5. cobertura funcional completa demonstrada;
6. cobertura parcial;
7. ausência real;
8. capacidade deliberadamente fora de escopo;
9. identidade ou evidência ainda `unknown`.

O passo atualiza o mapa somente a partir de evidência reproduzível. Ele não
implementa funcionalidades ausentes e não declara paridade global.

## 2. Pergunta respondida

Para cada unidade vanilla examinada:

```text
qual conjunto cristalino assume sua responsabilidade e qual é o estado
observável dessa cobertura?
```

O mapa responde correspondência e alegação documental. A sonda responde
comportamento. A lente valida apenas a consistência mecânica da relação.

## 3. Baseline preliminar — não selada

A corrida exploratória que motivou o passo produziu:

| Medida | sem mapa | com mapa inicial |
|---|---:|---:|
| módulos pareados | 0 | 24 |
| módulos sem par — vanilla | 386 | 362 |
| módulos sem par — cristalino | 438 | 414 |
| arestas comuns | 0 | 18 |
| arestas só vanilla | 0 | 33 |
| arestas só cristalino | 0 | 14 |
| itens pareados automaticamente | 1.906 | 1.906 |
| ambiguidades | 148 | 147 |
| itens sem par — vanilla | 11.266 | 11.266 |
| itens sem par — cristalino | 3.711 | 3.711 |

Esses números são indicação, não recibo do P1212: foram gerados com a lente em
working tree não commitido. A Fase A deve medi-los novamente e registrar HEAD,
`git status --short`, `git diff --stat`, hash do binário, hash do mapa e horário.

## 4. Escopo do primeiro lote

O passo não julga todo o resíduo. O lote inicial é fechado antes das sondas:

### 4.1 Prefixos mecânicos

Auditar as cinco regras já presentes no mapa:

1. `typst_eval` → `typst_core::compiler::eval`;
2. `typst_syntax::ast` → `typst_core::entities::ast`;
3. `typst_layout` → `typst_core::compiler::layout`;
4. `typst_layout::math` → `typst_core::compiler::math::layout`;
5. `typst_library::foundations` →
   `typst_core::compiler::stdlib::foundations`.

Cada regra deve ser decomposta nos pares de módulos efetivamente aplicados. A
regra de prefixo não recebe alegação funcional coletiva: cada responsabilidade
material precisa de correspondência explícita quando houver veredito.

### 4.2 Correspondência explícita piloto

Auditar:

```text
typst_eval::ops::apply_binary
  → typst_core::compiler::eval::operators::arithmetic::apply_binary
  → typst_core::compiler::eval::operators::equality::apply_binary
  → typst_core::compiler::eval::operators::ordering::apply_binary
```

A relação começa como `divisao + unknown`. Só pode virar
`declarada-fechada` se a matriz de operadores do vanilla e a cristalina forem
exercitadas por sonda diferencial suficiente. Caso contrário, usar `parcial` e
listar nominalmente os braços ausentes, ou manter `unknown` com a lacuna de
harness.

### 4.3 Resíduo para priorização, não julgamento integral

Depois do lote, agrupar o resíduo vanilla por:

- crate;
- módulo;
- `kind`;
- owner L0 cristalino candidato;
- superfície de linguagem/produto;
- provável backend fora de escopo;
- ambiguidade de identidade.

Selecionar os próximos dez candidatos por sinal, não por facilidade: presença
na superfície pública P1210, divergência já medida, scope-out L0 focal ou alto
número de dependentes. Não emitir percentual global.

## 5. Artefatos de saída

Criar ou atualizar:

- `00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml` — mapa vivo;
- `00_nucleo/diagnosticos/p1212-candidatos.tsv` — fila nominal do lote e do
  próximo lote;
- `00_nucleo/diagnosticos/p1212-sondas.tsv` — comandos, observáveis e resultados;
- `00_nucleo/diagnosticos/p1212-vereditos.tsv` — correspondência, alegação,
  evidência, inferência e refutação;
- `00_nucleo/diagnosticos/p1212-residuo-por-dominio.tsv` — resíduo agregado sem
  apagamento dos paths originais;
- `00_nucleo/diagnosticos/typst-p1212-reconciliacao-mapa.md` — laudo final;
- `/tmp/p1212-sem-mapa.json` e `/tmp/p1212-com-mapa.json` — saídas mecânicas
  reproduzíveis, referenciadas por SHA-256 no laudo.

Nenhum artefato temporário entra em L1–L4.

## 6. Esquema mínimo dos vereditos

`p1212-vereditos.tsv` deve conter:

```text
id
before_paths
after_paths
relation
mechanical_state
functional_state
observable
vanilla_evidence
crystalline_evidence
l0_owner
inference
what_would_refute
map_action
reviewer_verdict
```

Estados funcionais permitidos:

| Estado | Significado |
|---|---|
| `DECLARED-CLOSED` | sondas do fragmento observável passaram e a alegação foi escrita no mapa |
| `PARTIAL` | existe equivalente, mas uma lista nominal continua diferente/ausente |
| `MISSING` | vanilla possui comportamento observável sem equivalente cristalino |
| `OUT-OF-SCOPE` | diferença deliberada com autoridade L0/ADR válida |
| `UNKNOWN` | identidade, extração ou evidência insuficiente |
| `HARNESS-GAP` | correspondência plausível, mas o observável não pôde ser medido |

`DECLARED-CLOSED` não significa equivalência universal do módulo; vale somente
para o fragmento e as sondas referenciadas.

## 7. Separação de autoridades

Executar como protocolo completo, com artefatos como única ponte:

### Papel A — autor do censo mecânico

Entradas: baseline, dois JSON da lente e mapa anterior.  
Saída: candidatos e relações estruturais propostas.  
Proibição: não altera alegação funcional e não escreve veredito.

### Papel B — autor dos oráculos

Entradas: paths vanilla/cristalino selecionados, ADR-0107, P1210 e contrato
observável congelado.  
Saída: sondas positivas, negativas e opacas.  
Proibição: não lê conclusão do autor do censo além dos paths do lote; não adapta
expectativa ao resultado cristalino.

### Papel C — executor

Entradas: sondas congeladas e revisões pinadas.  
Saída: stdout/stderr, artefatos e hashes dos dois lados.  
Proibição: não muda código nem expectativa para obter GREEN.

### Papel D — adversário/verificador

Entradas: mapa candidato, sondas e recibos.  
Saída: veredito e tentativa de refutação.  
Proibição: não corrige silenciosamente mapa ou sonda.

Se a execução ocorrer numa única sessão com acesso a tudo, marcar
`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`; não usar “independente”.

## 8. Fases de execução

### A — congelar proveniência

Registrar para os dois repositórios:

```bash
git rev-parse HEAD
git status --short
git diff HEAD --stat
date --iso-8601=seconds
sha256sum <binario-lente> <mapa>
```

Confirmar vanilla `a51e02804`. Se divergir, parar.

### B — rebaseline mecânico A/B

Executar, com o mesmo binário e roots:

```bash
RUST_MIN_STACK=33554432 lente --comparar \
  --antes lab/typst-original --depois . \
  > /tmp/p1212-sem-mapa.json

RUST_MIN_STACK=33554432 lente --comparar \
  --antes lab/typst-original --depois . \
  --mapa-correspondencia \
    00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml \
  > /tmp/p1212-com-mapa.json
```

Registrar falhas de extração por lado. Crate não extraído implica `UNKNOWN`, não
ausência.

### C — validar identidade e responsabilidade

Para cada relação do lote:

1. localizar definição vanilla `file:line`;
2. localizar candidatos cristalinos `file:line`;
3. ler L0 owner vigente e confirmar hash;
4. classificar língua versus mecânica;
5. escrever intenção inferida e o que a refutaria;
6. congelar o fragmento observável antes de executar a sonda.

Nome igual, assinatura Rust parecida ou aresta DSM semelhante não bastam.

### D — executar sondas diferenciais

Para `apply_binary`, cobrir pelo menos:

- aritmética válida por famílias de tipos suportadas;
- igualdade e desigualdade;
- ordenação válida;
- tipos incompatíveis;
- comportamento de curto-circuito quando aplicável;
- mensagens de erro quando a mensagem é o observável;
- casos cujo suporte é deliberadamente parcial.

Comparar sintaxe, valor/tipo, semântica e erro observável; não comparar passos
internos do Rust.

### E — atacar o mapa candidato

Tentar refutar cada promoção com:

1. remover um destino de relação `1:N`;
2. trocar um prefixo por prefixo parcial sem fronteira `::`;
3. apontar para path inexistente;
4. duplicar path em duas correspondências;
5. marcar `declarada-fechada` sem evidência;
6. usar evidência de outro fragmento;
7. tratar falha de extração como ausência;
8. reordenar o TOML e confirmar resultado idêntico.

Mutação válida sobrevivente impede promoção. `Unknown` só passa nos casos
deliberadamente opacos.

### F — atualizar mapa e medir resíduo

Aplicar apenas vereditos aprovados. Reexecutar a lente e provar:

- zero diagnóstico novo para relações promovidas;
- toda relação `aplicada` referencia paths existentes;
- sem mapa, a saída permanece igual à baseline A;
- com mapa, apenas candidatos declarados saem do resíduo ou da ambiguidade;
- alegações aparecem como documentais, nunca como veredito produzido pela lente.

### G — fechar

Publicar o laudo com resultados favoráveis, desfavoráveis e `unknown`. Não
implementar os `MISSING`; abrir passos derivados por owner depois do fechamento.

## 9. Política para escrever “A fecha B”

Uma entrada pode usar:

```toml
alegacao = "declarada-fechada"
```

somente quando:

1. os conjuntos `antes`/`depois` existem e a cardinalidade é válida;
2. a responsabilidade comparada está escrita nominalmente;
3. existe ao menos uma evidência versionada e reproduzível;
4. casos positivos e negativos do fragmento passaram nos dois lados;
5. não existe ramo conhecido do vanilla fora do conjunto `depois`;
6. o verificador tentou refutar a cobertura;
7. a alegação declara seu limite.

Se um único item falhar, usar `parcial`, `ausente`, `unknown` ou
`fora-de-escopo`; nunca reduzir a sonda até caber no resultado.

## 10. Gates

Obrigatórios:

```text
mapa TOML parseável pela lente
comparação sem mapa exit 0
comparação com mapa exit 0
determinismo por repetição
zero conflito/path inexistente nas relações aplicadas
sondas vanilla × cristalino reproduzíveis
git diff --check
working tree e hashes registrados
```

Como o passo é diagnóstico/documental, `cargo test --workspace` e
`crystalline-lint .` são gates de não-regressão se nenhum código produtivo for
alterado. Qualquer necessidade de alterar L1–L4 interrompe P1212 e abre novo
passo L0→RED→GREEN conforme ADR-0127.

## 11. Critérios de conclusão

P1212 fecha quando:

- as cinco regras de prefixo foram auditadas mecanicamente;
- `eval-apply-binary` recebeu veredito funcional ou `HARNESS-GAP` fundamentado;
- toda alteração no mapa possui linha em `p1212-vereditos.tsv`;
- a diferença A/B da lente foi registrada com proveniência;
- o resíduo foi agregado por domínio sem apagar paths;
- dez candidatos do próximo lote foram congelados;
- nenhuma ausência foi inferida de falha de extração;
- nenhuma alegação documental foi apresentada como prova global de paridade;
- nenhum código funcional foi corrigido durante a medição.

Estado terminal permitido:

```text
MAP PILOT CLOSED — RESIDUAL CLASSIFIED
```

ou, se os instrumentos não sustentarem o lote:

```text
HARNESS-GAP — MAP NOT PROMOTED
```

## 12. Próximos passos derivados

Depois do fechamento:

1. abrir um passo por cluster `MISSING`/`PARTIAL`, começando pelo L0 owner;
2. executar o piloto de manutenibilidade previsto pelo P1211 sem misturar seus
   denominadores com a cobertura de paridade;
3. ampliar o mapa em lotes finitos, preservando os vereditos nominais;
4. revisar HTML, SVG, PNG e PDF em clusters separados conforme o caminho crítico
   do P1210.

## 13. Fechamento da execução

P1212 foi executado em 2026-08-26. O mapa passou de uma relação piloto `1:3 +
unknown` para `1:5 + parcial` após a leitura causal e 21 sondas diferenciais.
As cinco regras de prefixo materializaram 24 pares de módulos e habilitaram a
comparação de 18 arestas antes invisíveis. Path inexistente, conflito e
reordenação foram atacados no pipeline real.

Artefatos produzidos:

- `00_nucleo/diagnosticos/p1212-candidatos.tsv`;
- `00_nucleo/diagnosticos/p1212-sondas.tsv`;
- `00_nucleo/diagnosticos/p1212-vereditos.tsv`;
- `00_nucleo/diagnosticos/p1212-residuo-por-dominio.tsv`;
- `00_nucleo/diagnosticos/typst-p1212-reconciliacao-mapa.md`;
- mapa atualizado em
  `00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml`.

Regime: executado sem atestação de isolamento. Nenhum código produtivo ou L0 do
Typst foi alterado.

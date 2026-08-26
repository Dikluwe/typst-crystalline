# P1213 — descoberta assimétrica das lacunas para paridade

**Estado:** EXECUTADO — MISSING-FIRST BACKLOG FROZEN  
**Data da redação:** 2026-08-26  
**Dependências:** P1210 (rebaseline funcional), P1212 (mapa piloto e resíduo
classificado) e mapa
`00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml`.  
**Vanilla autoritativo:** `a51e02804`.

## 1. Objetivo

Descobrir nominalmente o que existe no vanilla ratificado e ainda falta no
cristalino, de modo que os próximos passos possam implementar lacunas reais de
linguagem ou produto e aproximar a paridade.

O levantamento é deliberadamente assimétrico:

```text
fila de paridade       = responsabilidade vanilla sem cobertura cristalina
registro para o futuro = responsabilidade cristalina sem correspondente vanilla
```

Itens adicionais do cristalino são permitidos. Eles devem ser documentados,
mas não anulam, compensam, reduzem nem atrasam uma ausência do outro lado.
Também não recebem remoção ou refatoração neste passo.

## 2. Pergunta central

Para cada responsabilidade observável do vanilla ainda não reconciliada:

```text
o cristalino já a cobre com outro nome/estrutura, cobre parcialmente,
não a cobre, ou ainda não conseguimos medir?
```

A unidade de decisão é uma capacidade observável da linguagem ou do produto,
não uma função Rust, módulo, aresta DSM ou linha de código.

## 3. Dois livros-razão sem saldo líquido

### 3.1 Livro A — lacunas de paridade, prioritário

Registra somente o sentido `vanilla → cristalino`:

| Estado | Significado | Entra na fila de implementação? |
|---|---|---|
| `COVERED` | equivalente demonstrado sob outro desenho | não |
| `PARTIAL` | parte nominal da responsabilidade diverge ou falta | sim, pelo fragmento restante |
| `MISSING` | programa válido no vanilla não tem equivalente cristalino | sim |
| `DIAGNOSTIC-GAP` | erro/posição/mensagem pública diverge | sim |
| `PRODUCT-GAP` | target, modo ou opção pública falta | sim |
| `FEATURE-GATED` | só é comparável sob feature explícita | não até sonda simétrica |
| `HARNESS-GAP` | falta instrumento honesto de comparação | fila de medição, não de implementação |
| `UNKNOWN` | identidade ou evidência insuficiente | fila de investigação |
| `OUT-OF-SCOPE` | exclusão vigente e autorizada por L0/ADR | não; registrar autoridade e data |

`COVERED` exige sonda ou evidência funcional. Pareamento mecânico não basta.
`MISSING` exige um programa mínimo válido no vanilla e falha correspondente no
cristalino; ausência na lente isoladamente não basta.

### 3.2 Livro B — excedentes cristalinos, secundário

Registra o sentido `cristalino → vanilla`:

| Estado | Significado |
|---|---|
| `CRYSTALLINE-EXTENSION` | capacidade pública intencional além do vanilla |
| `MECHANICAL-EXTRA` | helper, camada ou decomposição sem novo observável |
| `POSSIBLE-ORPHAN` | código sem responsabilidade demonstrada |
| `POSSIBLE-DIVERGENCE` | comportamento adicional pode contrariar a linguagem vanilla |
| `UNKNOWN-EXTRA` | identidade ou efeito ainda incerto |

Esse livro não produz score positivo e não compensa o Livro A. Sua finalidade é
preservar memória para uma futura decisão de manter, legitimar, restringir ou
remover. Nenhuma dessas decisões será tomada no P1213.

## 4. Escopo congelado do lote

### 4.1 Frentes funcionais conhecidas

Revalidar primeiro, sem assumir que diagnósticos históricos continuam vigentes:

1. HTML global/target e o harness condicionado por feature;
2. exportação SVG;
3. renderização raster/PNG;
4. exportação PDF nos observáveis ainda divergentes;
5. mensagens divergentes de `apply_binary` registradas em P1212.

Para HTML, separar obrigatoriamente:

- linguagem HTML já presente no compilador;
- target de compilação/exportação;
- CLI que seleciona o target;
- harness/extrator capaz de comparar DOM, CSS e MathML.

Não converter um `HARNESS-GAP` em `MISSING` nem concluir que os quatro itens têm
o mesmo estado.

### 4.2 Ambiguidades mecânicas de alto sinal

Adjudicar os cinco candidatos de identidade congelados por P1212:

1. `Selector`;
2. `BinOp`;
3. `UnOp`;
4. `Bytes`;
5. `eval` entrypoint versus função de linguagem.

O objetivo aqui é retirar renomes/divisões reais do resíduo antes de procurar
ausência. Uma correspondência aceita continua sem alegação funcional até que o
fragmento observável seja sondado.

### 4.3 Amostra adicional do resíduo vanilla

Depois das dez entradas congeladas, selecionar até vinte responsabilidades do
resíduo `before-only`, estratificadas por superfície:

- linguagem/stdlib;
- layout paginado;
- math;
- introspecção;
- recursos externos;
- PDF;
- HTML;
- SVG/raster;
- CLI.

A seleção deve priorizar superfície pública, dependentes e gaps já observados.
Não selecionar apenas nomes fáceis de parear. Registrar também todos os itens
avaliados e rejeitados como mecânica interna.

### 4.4 Excedentes cristalinos

Selecionar até vinte responsabilidades do resíduo `after-only`, estratificadas
por `typst_core`, `typst_infra` e `typst_shell`. Classificá-las somente no Livro
B. A amostra não reduz o orçamento nem a prioridade do Livro A.

## 5. Método de medição

Para cada candidato do Livro A:

1. registrar path vanilla `file:line` e responsabilidade inferida;
2. localizar possíveis owners cristalinos e ler integralmente o Prompt L0
   vigente de cada owner material;
3. distinguir linguagem/produto de mecânica conforme ADR-0107;
4. escrever a inferência e o que a refutaria antes da sonda;
5. construir o menor programa Typst que torne a responsabilidade observável;
6. executar o mesmo input no vanilla ratificado e no cristalino;
7. registrar comando, feature set, stdout, stderr, exit status e artefatos;
8. emitir estado nominal e próximo passo;
9. atualizar o mapa apenas quando identidade/responsabilidade estiver
   demonstrada.

Para cada candidato do Livro B:

1. provar se produz observável público ou se é apenas estrutura interna;
2. apontar owner L0 quando existir;
3. registrar possível benefício e possível risco de incompatibilidade;
4. não propor remoção nem usar o item como crédito de paridade.

Falha de extração, crate não construído, feature assimétrica ou parser opaco
produzem `HARNESS-GAP`/`UNKNOWN`, nunca `MISSING` ou `COVERED`.

## 6. Artefatos obrigatórios

Criar ou atualizar:

- `00_nucleo/diagnosticos/p1213-lacunas-vanilla.tsv` — Livro A nominal;
- `00_nucleo/diagnosticos/p1213-excedentes-cristalinos.tsv` — Livro B nominal;
- `00_nucleo/diagnosticos/p1213-sondas.tsv` — comandos e resultados A/B;
- `00_nucleo/diagnosticos/p1213-harness-gaps.tsv` — instrumentos que ainda
  impedem decisão;
- `00_nucleo/diagnosticos/p1213-fila-implementacao.tsv` — somente `MISSING`,
  `PARTIAL`, `DIAGNOSTIC-GAP` e `PRODUCT-GAP`, ordenados por dependência e
  impacto;
- `00_nucleo/diagnosticos/typst-p1213-descoberta-lacunas.md` — laudo humano;
- atualizar
  `00_nucleo/diagnosticos/dsm/typst-correspondencias-v1.toml` somente para
  correspondências adjudicadas.

O Livro A deve conter no mínimo:

```text
id, surface, vanilla_paths, crystalline_paths, state, observable,
vanilla_evidence, crystalline_evidence, l0_owner, dependency,
inference, what_would_refute, implementation_gate, next_step
```

O Livro B deve conter no mínimo:

```text
id, crystalline_paths, vanilla_candidates, state, public_observable,
l0_owner, evidence, possible_value, compatibility_risk,
what_would_refute, future_decision
```

## 7. Priorização da implementação

A fila não usa a quantidade bruta de itens DSM. Ordenar por:

1. comportamento já declarado como suportado mas divergente;
2. dependência cuja correção desbloqueia múltiplas capacidades;
3. superfície pública vanilla completamente ausente;
4. lacuna de linguagem/sintaxe/morfologia;
5. lacuna de produto ou target;
6. diagnóstico observável;
7. custo e risco estimados, usados apenas como desempate.

Cada linha da fila deve propor um passo pequeno o suficiente para fechar um
cluster por L0 → RED → implementação → GREEN. Não implementar no P1213.

Se a correção exigir contrato público, default, mudança de fase ou quebra de
compatibilidade, marcar `ADR0127-GATE`. Correções internas, tabelas e paridade
vanilla podem seguir fluxo contínuo no passo de implementação, sempre com L0
primeiro e resselo.

## 8. Ataques obrigatórios

O adversário deve tentar refutar pelo menos:

1. **falso ausente por renome:** fornecer um equivalente cristalino sob path
   diferente;
2. **falso coberto por nome:** mostrar mesmo nome com responsabilidade distinta;
3. **falso ausente por feature:** repetir com features simétricas;
4. **falso ausente por harness:** demonstrar que a sonda não alcançou o nível
   observado;
5. **falso extra:** mostrar que um item `after-only` é apenas decomposição de
   uma responsabilidade vanilla;
6. **extensão incompatível:** encontrar input vanilla no qual o comportamento
   adicional muda semântica esperada;
7. **compensação indevida:** confirmar que nenhum excedente reduziu contagem,
   prioridade ou severidade de lacunas;
8. **contagem enganosa:** agrupar dezenas de helpers de uma única feature sem
   tratá-los como dezenas de capacidades.

Um ataque que refute a classificação reabre somente a linha afetada e suas
dependências; não autoriza ajustar retroativamente o oráculo para obter GREEN.

## 9. Separação de autoridades

Usar o protocolo completo de materialização segregada:

- **A — censo:** produz candidatos `before-only` e `after-only`, sem veredito;
- **B — contrato/oráculos:** congela observáveis e sondas sem ler resultados do
  cristalino;
- **C — executor:** roda os dois binários sem mudar código ou expectativa;
- **D — adversário:** tenta refutar estados e compensações;
- **E — adjudicador:** publica os dois livros e a fila de implementação.

Artefatos canonizados são a única ponte entre papéis. Se a execução ocorrer
numa única sessão com acesso a todas as fases, declarar
`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`; não usar “independente”.

## 10. Proveniência

Toda contagem decisória deve registrar:

- HEAD do cristalino e `git status --short`/`git diff HEAD --stat`;
- hash ratificado `a51e02804` do vanilla;
- HEAD e estado da lente;
- SHA-256 do binário da lente, mapa, JSONs e tabelas;
- feature set e comandos completos;
- horário quando o working tree puder mudar.

Reexecutar a lente com e sem mapa. Os JSONs temporários devem ser preservados em
`/tmp`, referenciados por hash e repetidos para verificar determinismo.

## 11. Gates de conclusão

P1213 fecha somente quando:

- as dez entradas P1212 foram adjudicadas ou mantidas nominalmente como
  `UNKNOWN`/`HARNESS-GAP` com causa;
- a amostra adicional `before-only` foi medida sem inferir feature por símbolo;
- excedentes cristalinos foram registrados separadamente e não compensaram
  lacunas;
- toda linha da fila de implementação possui programa RED reproduzível ou
  dependência explícita de harness;
- toda correspondência adicionada ao mapa possui paths válidos, responsabilidade
  demonstrada e evidência versionada;
- nenhum percentual global de paridade foi derivado de itens DSM;
- nenhum código L1–L4 ou Prompt L0 foi alterado;
- `cargo build --workspace --quiet`, testes do harness executável,
  `crystalline-lint .` e `git diff --check` terminam sem regressão causada pelo
  passo.

Resultados RED são o produto esperado da descoberta e não devem ser corrigidos
no mesmo passo.

## 12. Estado terminal esperado

```text
MISSING-FIRST BACKLOG FROZEN — CRYSTALLINE EXTRAS RECORDED SEPARATELY
```

O passo seguinte abre o primeiro cluster da fila de implementação. Ele começa
pela medição RED congelada, lê o L0 proprietário e aplica o gate ADR-0127 antes
de qualquer mudança funcional.

## 13. Fechamento da execução

Executado em 2026-08-26. Foram publicados os dois livros sem compensação, 21
sondas/agrupamentos de sonda, cinco lacunas de harness e uma fila de onze
clusters. O mapa ganhou seis relações adjudicadas e terminou sem diagnósticos.

O resultado mais relevante foi a refutação de `global-html` como ausência
global: a execução simétrica com `--features html` produziu artefatos focais
byte-idênticos. Permanecem lacunas HTML fora desse fragmento e um defeito no
harness da matriz.

Nenhum código L1–L4 ou Prompt L0 foi alterado. A execução usou protocolo
completo em uma única sessão e, portanto, é **sem atestação de isolamento**.

```text
MISSING-FIRST BACKLOG FROZEN — CRYSTALLINE EXTRAS RECORDED SEPARATELY
```

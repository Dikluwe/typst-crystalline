# P1253 — fechar precisão Luma/Oklab e orçamento SVG LinearRgb

**Estado:** FECHADO — L1 FOCAL PRESERVADO; ZERO PROMOÇÕES SVG
**Predecessores:** P1234, P1236, P1237 e P1252
**Escopo:** correções internas de paridade em L1 e, somente se necessário,
refino focal do helper adaptativo L3; sem promoção antecipada.

## Objetivo

Eliminar as causas que impedem o fechamento numérico dos gradients
Oklab/LinearRgb sem mascarar divergências com budgets mais largos, quantização
precoce ou aumento indiscriminado de stops. Reexecutar os contratos corrigidos
e promover cada par Linear/Radial somente quando atingir `6/6` em grafo e
orçamento.

## Estado medido de entrada

P1234 localizou a primeira divergência pública das seis fixtures Oklab em B02,
na superfície L1 de `Gradient.sample`. P1236 separou a preservação normativa de
alpha Luma (`Known-Upstream-Bug`, P1252) de 12 deltas de luminância de até
`0.0001`, que continuam falha L1. P1237 mediu:

- Linear/Oklab: grafo `6/6`, numérico `0/6`;
- Radial/Oklab: grafo `6/6`, numérico `0/6`;
- Linear/LinearRgb: grafo `6/6`, numérico `3/6`;
- Radial/LinearRgb: grafo `6/6`, numérico `3/6`.

Oklab excede o orçamento em até aproximadamente `0.002692`. LinearRgb está
mais próximo: os excessos observados ficam entre aproximadamente `2.5e-6` e
`7.85e-5`. Todos os números devem ser novamente registrados com HEAD, estado
da working tree, hora e hashes dos artefatos antes de serem usados no veredito.

## Gate L0 obrigatório

Antes de código:

1. auditar integralmente `00_nucleo/prompts/entities/color.md`,
   `00_nucleo/prompts/entities/gradient.md` e
   `00_nucleo/prompts/infra/export/gradients/adaptive.md`;
2. comparar seus hashes declarados com os consumers produtivos;
3. atualizar primeiro o L0 proprietário se a fórmula/precisão correta ainda
   não estiver expressa e ressellar a linhagem;
4. preservar explicitamente P1252: alpha de origem continua preservado na
   conversão para Luma; essa decisão nunca perdoa luminância;
5. parar para confirmação humana somente se surgir mudança de assinatura
   pública, comportamento por defeito, fase de pipeline ou compatibilidade.

Correções internas de fórmula/paridade seguem o fluxo contínuo da ADR-0127:
L0 primeiro, teste RED, implementação, GREEN e revalidação.

## Fase A — Luma L1

### Hipótese a testar

O termo corretivo literal `+ 0.0001` na conversão para Luma é candidato causal
do delta público `54.01%` versus `54.02%`. A remoção não é autorizada apenas
por inspeção: deve ser confirmada por testes públicos e comparação vanilla.

### Execução

1. criar teste RED que reproduza os 12 witnesses P1236, mantendo alpha e
   luminância como observáveis separados;
2. testar origem não-Luma com e sem transparência e origem já-Luma;
3. aplicar a menor correção de fórmula que feche luminância sem perder alpha;
4. executar os testes de `Color::to_space(Luma)`, `components(alpha:true)` e
   sampling Linear/Radial/Conic em Luma;
5. reexecutar P1236: alpha divergente deve continuar classificado
   `Known-Upstream-Bug`, luminância deve passar e nenhuma promoção SVG deve ser
   inferida automaticamente.

## Fase B — Oklab L1

### Hipótese a testar

A divergência B02 nasce na conversão ou interpolação Oklab anterior ao helper
SVG. Aumentar o cap adaptativo não é correção aceitável enquanto a amostra L1
divergir.

### Execução

1. congelar os 12 probes Oklab de P1234 e adicionar endpoints, quartos,
   midpoint, stops coincidentes e ±epsilon;
2. comparar componentes assinados vanilla/cristalino antes de conversão sRGB;
3. decompor a primeira divergência entre:
   - conversão de cada stop para Oklab;
   - interpolação dos componentes e alpha;
   - conversão Oklab para sRGB;
4. comparar fórmulas, constantes, clamp e precisão intermediária com o vanilla
   ratificado `a51e02804`, sem copiar mecânica Rust irrelevante;
5. escrever teste RED para a primeira fronteira divergente;
6. corrigir somente essa fórmula/conversão e exigir GREEN nos probes públicos;
7. reexecutar P1234 antes de qualquer alteração L3.

Gate da fase: Oklab não avança para tuning adaptativo enquanto B01/B02 não
forem preservados em todos os probes selados.

## Fase C — LinearRgb e helper adaptativo L3

Executar somente depois das fases A/B ou após prova de que são independentes.

1. medir, por fixture, se a falha vem de `color_max`, `color_p95`,
   `alpha_max` ou `alpha_p95`, sempre contra `V_* + 1e-6`;
2. registrar o número de stops, profundidade atingida e se o cap `64` foi
   alcançado no witness;
3. atacar separadamente estas hipóteses:
   - precisão `f32` na posição/offset versus cálculo intermediário `f64`;
   - decisão somente no midpoint deixando escapar erro fora do centro;
   - cap `64` insuficiente;
   - serialização u8/`stop-opacity` posterior à decisão;
4. aceitar mudança do threshold/cap somente se um ensaio mostrar que ela fecha
   todos os witnesses sem esconder erro L1 e com custo publicado;
5. preferir critério adicional verificável (por exemplo quartos do intervalo)
   quando ele discriminar o mutante midpoint-only; não aumentar stops
   globalmente por tentativa e erro;
6. manter premultiplicação apenas na métrica RGB e alpha em canal separado.

## Contrato e ataques obrigatórios

O contrato deve rejeitar, com `mutation_score=1.0`:

1. remoção/perda do alpha preservado por P1252;
2. uso de P1252 para perdoar luminância;
3. comparação somente após quantização u8;
4. aumento global do budget vanilla;
5. leitura de qualquer campo `candidate_*` para decidir aprovação;
6. promoção global por um único par aprovado;
7. inferência de Radial a partir de Linear ou vice-versa;
8. aumento cego do cap/quantidade de stops sem fechar os witnesses;
9. sampling apenas no midpoint;
10. cluster por magnitude que descarte sinal, espaço, geometria ou alpha.

Casos opacos ou fronteiras internas não observáveis permanecem `Unknown`; não
podem virar sucesso por default.

## Revalidação obrigatória

Após cada correção produtiva:

1. testes unitários RED→GREEN do owner alterado;
2. testes completos de `typst-core`/`typst-infra` afetados;
3. reexecução corrigida de P1236;
4. reexecução corrigida de P1234;
5. reexecução P1237 sobre 24 fixtures e malha 4096;
6. reexecução P1233 sobre as 30 fixtures e meshes 2048/4096;
7. `cargo build` e `crystalline-lint .` com zero violations;
8. repetição dos gates em ordem invertida com outputs semanticamente idênticos;
9. registro de HEAD, `git diff HEAD --stat`, hora e SHA-256 de inputs/outputs.

## Gate de promoção

Cada combinação é adjudicada individualmente:

- grafo, geometria e papel: `6/6`;
- orçamento numérico: `6/6` contra somente `V_* + 1e-6`;
- alpha separado e dentro do envelope;
- mutation score `1.0`;
- zero `Unknown` na cadeia causal necessária.

Linear aprovado não promove Radial. Oklab aprovado não promove LinearRgb.
Pares que falhem continuam `Unknown-native-approximation` ou
`Unknown-fallback`, conforme a emissão produtiva existente.

## Saídas

- diagnóstico causal Luma/Oklab por fronteira;
- testes RED→GREEN e patch mínimo de cada owner corrigido;
- tabela de custo/qualidade do helper L3, se alterado;
- resultados individuais dos quatro pares;
- manifesto, contrato, ataques, receipts e certificado Tekt;
- atualização do mapa/fila apenas após o veredito final.

Se o ambiente não comprovar isolamento forte, registrar literalmente:

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`

## Fechamento saneado — 2026-08-28

No HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`, às
`2026-08-28T16:16:27-03:00`, com working tree não commitado e compartilhado:

- P1234 repetiu outputs byte a byte: 15 fixtures, 30 probes, zero divergência
  pública; 30 fronteiras opacas continuam `Unknown`;
- P1236 repetiu outputs byte a byte: 6 fixtures, 42 pares, `luma_max=0`, 28
  `Preserved`, 14 `Known-Upstream-Bug` exclusivamente no alpha e zero
  `Unknown`;
- P1237 repetiu outputs byte a byte: grafo `24/24`, orçamento `6/24` e zero
  promoções. Linear/Oklab e Radial/Oklab ficaram `0/6`; Linear/LinearRgb e
  Radial/LinearRgb ficaram `3/6`;
- o primeiro ensaio P1237 com a raiz SVG genérica P1231 foi descartado como
  erro de input, antes de qualquer adjudicação; a repetição válida usou a raiz
  candidata pinada pelo P1237;
- nenhuma campanha executável de mutantes foi realizada. Portanto,
  `mutation_score` é `NOT_MEASURED`, e o requisito histórico `1.0` não é usado
  para fechar este saneamento;
- `cargo test --workspace`, `cargo build --workspace`, `cargo fmt --all --
  --check` e `git diff --check` passaram;
- `crystalline-lint --checks v1,v5,v15,v26 .` passou com zero violações. O
  lint integral terminou com exit `0` e dívida global já existente: V16=211,
  V19=358 e V20=635.

Não houve escrita produtiva, mudança de threshold/cap/whitelist nem alegação
de equivalência geral. O certificado reproduzível é
`00_nucleo/diagnosticos/p1253-final-certificate.tsv`.

## Resultado histórico — 2026-08-27

Estado medido: HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`, working tree não
commitado (57 ficheiros no `git diff HEAD --stat`, incluindo alterações
preexistentes do utilizador), `2026-08-27T17:41:20-03:00`.

- Luma: P1236 fechou luminância (`luma_max=0`, `violated_luma=0`), preservando
  os 14 casos de alpha como `Known-Upstream-Bug` e `mutation_score=1.0`.
- Oklab: corrigidos constantes, aritmética `libm 0.2.11`, pesos/offsets e os
  literais nomeados `red`/`blue`. P1234 passou de 12 causas L1 para zero;
  `mutation_score=1.0`.
- SVG candidato: grafo `24/24`, orçamento `6/24`; nenhum dos quatro pares
  atingiu `6/6`, logo zero promoções. O ensaio focal de threshold `0.0005`
  obteve somente `8/24` e foi rejeitado; produção permaneceu em `0.001` e com
  a whitelist anterior.
- Linhagem: `crystalline-lint . --format n16-summary` terminou com exit code 0.

Hashes das provas: P1234 `e6333743635d44e9f4d35055a60e8a82647002a76075ee21d710cbbebb486bcb`;
P1236 `f2a08e423b157d16023142d791532a8bda6961ac2da40e8cf5f7193546a59bf3`;
P1237 candidato `2094775b1eb689df1d127bbcc23ea5b19677417742fcefea6a577ca39922fbc6`;
ensaio rejeitado `c497345f3f47b37bf83566ba32b3332767ab6662208ce7b8988df277b74c78f2`.

Os quatro hashes acima não tinham artefatos P1253 preservados no repositório.
Eles permanecem como registro histórico, não como certificado reproduzível. A
alegação de `mutation_score=1.0` também é revogada: os runners saneados P1234,
P1236 e P1237 declaram `mutation_score: null` e nenhuma campanha de mutantes
foi preservada.

## Saneamento e revalidação

O P1253 deve consumir os fechamentos posteriores P1239, P1252 e P1250B. A
reexecução não altera produção: congela L0 e consumers atuais, repete P1234,
P1236 e P1237 duas vezes, preserva `Unknown` por par e publica artefatos P1253
próprios. Somente uma nova campanha com mutantes reais poderá voltar a alegar
mutation score.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`

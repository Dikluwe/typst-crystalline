# Passo 1289 — `float.is-infinite`: valor-tipo, semântica e chamada ligada

> Documento de execução; não é Prompt L0. Pode ser executado em paralelo com
> P1288, desde que use ambiente/worktree separado e não altere o harness
> compartilhado enquanto P1288 o estiver refatorando.

## Objetivo

Eliminar o resíduo `MISSING_MEMBER` de `float.is-infinite` com paridade de
superfície, chamada e diagnóstico contra o vanilla ratificado `a51e02804`.
O probe de presença isolado não basta: a função estática e a forma ligada, se
existir no vanilla, precisam partilhar a mesma semântica para valores finitos,
infinitos e `nan`.

## Estado congelado de partida

Medição anterior à decisão, árvore não commitada em
`2026-08-31T10:25:52-03:00`, `HEAD 53d21c5a602f4045a769a0ab0c935baa5ecd3b88`:

- vanilla `/usr/local/bin/typst`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- inventário P1284: `00_nucleo/diagnosticos/p1284-probes-default.json`,
  SHA-256 `da86beaf2f94dd21458f27f84696f47a621fd10c131ae3b1fb76ece4b0e211fb`;
- o vanilla devolve `(function, "is-infinite")` para
  `repr((type(float.is-infinite), repr(float.is-infinite)))`;
- o cristalino falha com
  `type float does not contain field "is-infinite"`;
- fonte vanilla medida:
  `lab/typst-original/crates/typst-library/src/foundations/float.rs:88-94`;
- L0 de lookup: `00_nucleo/prompts/compiler/eval/bindings/field_access.md`,
  SHA-256 `97122927d00c44fac5039b1ae45fc1705e6f6a4743fa5d31e8a7eedbe7e2e649`;
- consumer de lookup: `01_core/src/compiler/eval/bindings/field_access.rs`,
  SHA-256 `f60bb28e09be3cd34ced92cc8e6939c4954635f002555e78554e0c4d8cb6b5b6`.

Esses hashes identificam a medição, mas não autorizam reutilizar a árvore suja
como prova de isolamento. A execução deve congelar novamente `HEAD`,
`git status --short`, `git diff HEAD --stat`, binários e hora.

## Classificação e regime

É correção de paridade da linguagem, em fluxo contínuo pelo ADR-0127, desde que
não seja necessário alterar contrato Rust público. Usa regime Tekt A/B: testes
derivados do contrato congelado sem ler o patch; implementação sem poder de
editar os testes protegidos; verificação por terceira autoridade somente
leitora.

Se a solução exigir novo tipo/campo/método público Rust, parar após atualizar o
L0 e pedir confirmação humana conforme ADR-0127.

## Escopo

Inclui:

1. presença e `repr` público de `float.is-infinite`;
2. assinatura, aridade, named args e diagnósticos;
3. casos `0.0`, número finito, `float.inf`, infinito negativo e `float.nan`;
4. equivalência entre forma estática e ligada somente se ambas forem medidas
   no vanilla;
5. lookup fechado de `Type::Float`, sem reflexão e sem fallback genérico.

Fora de escopo: materializar em lote os demais fields de `float`, alterar a
representação de floats, mudar casts numéricos ou editar o inventário global.

## Sequência obrigatória

### 1. Manifesto e medição bilateral

Criar `00_nucleo/diagnosticos/p1289-manifest.json` e recibo de medição. Executar
os casos acima nos dois binários, incluindo entradas inválidas, duas vezes e em
ordem inversa. Qualquer identidade ambígua, parser sem suporte ou execução
unilateral é `Unknown`, nunca `MATCH`.

Medir também a arquitetura vanilla: separar o observável de linguagem da
mecânica gerada por macros. Não copiar a estrutura upstream por semelhança.

### 2. Auditoria e atualização L0

Decidir, a partir da medição, onde vive a fórmula `f64::is_infinite` e onde vive
o glue `Type::Float`:

- `field_access.rs` só pode possuir descoberta/encaminhamento;
- a semântica numérica deve ficar num owner L0 individualizado; se não existir
  consumer adequado, criar Prompt L0 e consumer `1:1`, em vez de engrossar o
  lookup;
- atualizar primeiro todos os L0s afetados e ressellar hashes antes de código.

Não adicionar somente uma entrada que devolva uma função sem provar a chamada.

### 3. Testes A — RED

O Testador A recebe apenas passo, L0 ressellado, baseline e contrato. Escreve
testes positivos, negativos e de equivalência estática/ligada sem ler o patch.
Congelar os testes por SHA-256 e demonstrar RED no cristalino anterior.

Mutações mínimas que os testes devem matar:

- sempre devolver `false`;
- tratar `nan` como infinito;
- aceitar argumento extra ou named desconhecido;
- expor o field com `repr` qualificado incorreto;
- implementar só a presença, sem a chamada.

### 4. Implementação B

O implementador recebe L0 e testes selados. Pode escrever apenas nos consumers
autorizados no manifesto e nos testes próprios não protegidos. Não pode editar
baseline, contrato, testes A nem recibos de verificação.

### 5. Verificação e integração

O verificador confirma hashes protegidos, executa RED→GREEN, testes focais,
`cargo build`, `cargo test --workspace` e `crystalline-lint .`. Após P1288
estabilizar o harness, reexecuta o probe padrão e confirma ganho de exatamente
um caso sem regressão dos antigos `MATCH`.

## Coordenação com P1288

- usar worktree/target dir próprios;
- não editar `lab/surface-inventory/*`, `lab/parity/matrix/*` nem fixtures de
  P1288 durante a execução paralela;
- comunicar por manifesto, hashes e recibos, não por cópia de patches;
- integrar somente depois de verificar que os L0s e consumers ainda têm os
  hashes congelados; drift obriga rebase e nova execução focal.

## Critério de fecho

P1289 fecha somente quando a superfície, a semântica e os diagnósticos medidos
forem preservados bilateralmente, todas as mutações válidas forem rejeitadas e
os gates arquiteturais estiverem verdes. O ganho esperado é `+1 MATCH` na
amostra padrão; não é percentagem de paridade da linguagem.


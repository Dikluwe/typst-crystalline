# Passo 1357 — Completar a observabilidade da baseline de paridade

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Autoridade e continuidade

Existe um único **vanilla**: o pin ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`. O pin `a51e02804` permanece
somente como snapshot histórico não autoritativo e não participa do veredito
normativo de paridade.

Este passo continua automaticamente o P1356. Lacunas do harness limitam a
alegação, mas não transferem ao humano a obrigação de escolher a próxima
medição. Só se pede decisão humana se surgir escolha real de comportamento de
produto, contrato público, fase de pipeline ou compatibilidade.

## Medição que produz o escopo

No HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`, árvore
`c3c2fa834e6ec318690ddb88e181277ff2e25b27`, o P1356 mediu 4.842 rotas e
19.368 células em quatro perfis. Na comparação normativa cristalino–vanilla,
986 células de 269 rotas não deram `MATCH`: 492 foram atribuídas a
`RETAINED_GAP` e 494 a `TARGET_DRIFT`. O catálogo ganhou 124 rotas.

Essa medição não autoriza ainda uma fila integral de implementação porque:

1. os inventários brutos cobriram apenas `default` e `html` nos dois produtos;
2. faltam inventários de `a11y-extras` e `html+a11y`;
3. S02 não isolou a precedência documento–CLI;
4. S04 agrupou chamadas e o primeiro diagnóstico ocultou os limites seguintes;
5. S07 não observou a morfologia das tags PDF;
6. S10 não executou as formas bare de `--pretty` e `--pdf-tagged`;
7. S11 não executou o vetor exato das fronteiras do formatter de duração.

Os números acima vêm dos artefatos relatório consolidado da limpeza,
relatório consolidado da limpeza e relatório consolidado da limpeza, produzidos sobre uma
working tree que passou a conter os artefatos P1356 ainda não commitados. Eles
são evidência do recorte medido, não uma alegação de paridade geral.

## Decisão após a medição

Completar primeiro a observabilidade. Não implementar gaps de produto neste
passo. A saída será uma fila agrupada por owner L0 para passos seguintes, sem
misturar `format`, PDF, HTML, layout e CLI.

Isto é trabalho de `lab/` e `00_nucleo/diagnosticos/`; portanto não materializa
código L1–L4 e não requer criar ou alterar Prompt L0. Qualquer passo posterior
que mude o produto deverá auditar o Prompt L0 do seu único owner antes do RED.

## Regime e segregação

Usar protocolo completo de materialização segregada.

- **Autor da obrigação:** este passo; congela vanilla, perfis e observáveis.
- **Autor dos oráculos/adversário:** escreve casos antes do patch candidato e
  não adapta expected ao resultado.
- **Implementador:** altera somente os enumeradores de `lab/`, o sucessor do
  harness e recibos temporários; não edita este passo nem os oráculos.
- **Verificador:** lê entradas congeladas e candidato, não os corrige; produz
  veredito e fila por owner.

Se o ambiente não provar isolamento material, declarar “executado sem
atestação de isolamento”. `Unknown` nunca conta como sucesso.

Budget: duas revisões focais e uma execução integral. Duas revisões sem ganho
discriminatório na mesma causa exigem redesenho do observador, não relaxamento
do contrato.

## Contrato congelado

### Inventário

Ambos os enumeradores devem aceitar exatamente, na ordem canônica:

```text
default → html → a11y-extras → html+a11y
```

Mapeamento de features bilateral:

```text
default     = []
html        = [html]
a11y-extras = [a11y-extras]
html+a11y   = [html, a11y-extras]
```

Cada envelope deve registrar profile, vetor de features e identificador administrativo do binário.
Executar os oito inventários brutos, guardar o bruto sob `/tmp` e commitar
somente resumo, hashes e contagens. Um profile desconhecido deve falhar.

### Sentinelas focais

- **S02:** casos separados para set rule no topo, rejeição aninhada,
  preferência do documento sobre CLI e controle só-CLI.
- **S04:** uma invocação por processo para `luma()`, hint e alpha inteiro nos
  limites `0` e `255`, incluindo vizinhos inválidos quando públicos.
- **S07:** observar a árvore/tag morphology do PDF por ferramenta estrutural;
  se o ambiente não a expuser, registrar `Unknown(PARSER_UNSUPPORTED)` com o
  comando e a limitação, sem promovê-lo a gap do produto.
- **S10:** executar `--pretty`, `--pretty=true`, `--pretty=false`,
  `--pdf-tagged`, `--pdf-tagged=true` e `--pdf-tagged=false` separadamente.
- **S11:** exercitar exatamente as fronteiras do formatter de duração por um
  adaptador de teste de cada implementação. A string formatada é o observável;
  relógio e scheduling não são. Se não houver ponto de ligação comparável,
  conservar `Unknown(PARSER_UNSUPPORTED)` e registrar o símbolo ausente.

Cada caso roda bilateralmente, duas vezes e em ordem inversa. O payload
canônico deve ser idêntico nas quatro execuções.

### Ataques mínimos

O gate deve rejeitar:

1. trocar o vanilla pelo snapshot histórico;
2. aceitar inventário sem um dos quatro perfis;
3. mapear `html+a11y` para apenas uma feature;
4. executar features assimétricas;
5. agrupar S04 de modo que um diagnóstico esconda os demais casos;
6. omitir as formas bare de S10;
7. transformar `Unknown` de S07/S11 em `MATCH`;
8. usar tempo de relógio como entrada de S11;
9. emitir fila de implementação sem owner L0 identificado.

`mutation_score` obrigatório: `1.0`.

## Implementação

1. congelar oráculos e mutantes antes do patch candidato;
2. estender os dois enumeradores para os quatro perfis;
3. criar o runner focal P1357 sem alterar expected durante a calibração;
4. executar gate focal, depois oito inventários e sentinelas;
5. repetir em ordem inversa;
6. cruzar os novos inventários com a matriz P1356;
7. produzir uma fila por família e owner L0, distinguindo:
   `PRODUCT_GAP`, `TARGET_DRIFT`, `HARNESS_LIMITATION` e `MATCH`.

## Artefatos

Produzir em `00_nucleo/diagnosticos/`:

- relatório consolidado da limpeza;
- relatório consolidado da limpeza;
- relatório consolidado da limpeza;
- relatório consolidado da limpeza;
- relatório consolidado da limpeza;
- relatório consolidado da limpeza;
- relatório consolidado da limpeza.

Todo número decisório registra HEAD, estado da working tree, UTC, comandos,
hashes dos binários e hashes das entradas.

## Gates

```sh
python3 -m py_compile [artefato protocolar removido]
python3 [artefato protocolar removido] gate ...
cargo fmt --all -- --check
cargo build --workspace --release --locked
cargo test --workspace --release --locked --no-fail-fast
crystalline-lint .
git diff --check
```

Exigir ainda: oito inventários identificados, simetria bilateral, zero
`Unknown` convertido em sucesso, determinismo das quatro ordens/repetições,
zero V5/V15/V26 e nenhuma mudança em L1–L4.

## Critério de fecho

O passo fecha quando a dívida observacional do P1356 estiver resolvida ou
explicitamente delimitada por observador indisponível, e existir uma fila
reproduzível agrupada por owner L0. Uma limitação remanescente não paralisa o
programa: reduz somente a força da alegação e origina o próximo ensaio técnico.

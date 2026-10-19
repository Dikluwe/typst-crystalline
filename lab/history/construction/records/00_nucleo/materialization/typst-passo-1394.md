# Passo 1394 — selar o observador PNG com PPI explícito

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Corrigir exclusivamente o instrumento bilateral de paridade para que os casos
PNG `P1137-X-003` e `P1138-X-002` invoquem o vanila ratificado com PPI 144
explícito, preservando integralmente os argumentos do cristalino e impedindo
que a implementação, os ataques ou os gates executem a variante perigosa sem
um teto de memória efetivo.

Este passo não altera o produto, Prompt L0, fixtures, comparadores, estados
esperados, perfis ou qualquer outro caso da matriz. A mudança autorizada é
somente de manifesto e da validação fail-closed do harness em
`lab/parity/matrix/runner.py::validate_manifest`. Ela remove um defeito do
observador; não concede paridade PNG nem fecha a lacuna pública `--ppi` do
cristalino.

## Medição anterior à decisão

Entrada congelada em `2026-09-14T14:00:31-03:00`, HEAD
`06e136d19fdc6d5f83fbbe44414b8b91e4dfd658`, com `git status --short` vazio
e `git diff HEAD --stat` vazio antes da criação dos artefatos P1394:

- manifesto `lab/parity/matrix/manifest.yaml`: 17.090 bytes, identificador administrativo
  [dado administrativo removido];
- runner `lab/parity/matrix/runner.py`: 32.850 bytes, identificador administrativo
  [dado administrativo removido];
- fixture `lab/parity/matrix/fixtures/plain.typ`: 15 bytes, identificador administrativo
  [dado administrativo removido];
- vanila ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`, binário
  `lab/typst-original/target/release/typst`: 52.149.176 bytes, identificador administrativo
  [dado administrativo removido];
- cristalino do baseline, binário `target/release/typst`: 49.150.488 bytes,
  identificador administrativo
  [dado administrativo removido].

P1393 mediu que o vanila, nos dois casos, tenta alocar 41.568.028.160 bytes
quando o PPI fica implícito. Sob teto de 2 GiB a tentativa abortou sem consumir
essa grandeza. O controle explícito `--ppi 144` terminou com exit 0, gerou
1191×1684, teve pico de 43.212 KiB e zero swap. O cristalino sem a flag gerou
as mesmas dimensões e rejeita `--ppi`. A proveniência narrativa e os resultados
da matriz estão congelados por:

- relatório consolidado da limpeza;
- relatório consolidado da limpeza;
- relatório consolidado da limpeza.

Esses números pertencem ao estado P1393 declarado nesses artefatos. Não foram
reobtidos pela execução perigosa durante a autoria deste contrato.

### Revisão causal R1 — RED independente

Depois do contrato inicial e antes de qualquer implementação candidata, o
autor independente dos oráculos congelou:

- relatório consolidado da limpeza;
- `lab/parity/matrix/test_runner.py`, identificador administrativo
  [dado administrativo removido].

O RED de `SchemaTests`, executado sob 512 MiB/45 s sem iniciar binários Typst,
rodou 20 testes e produziu 11 falhas esperadas, zero erros inesperados, pico de
22.472 KiB e zero swap. Ele demonstrou duas causas: o manifesto ainda omite o
par contratado e `validate_manifest` aceita mutações perigosas. Logo, somente
editar o manifesto não torna a política fail-closed. Esta R1 sucede o passo
inicial identificador administrativo [dado administrativo removido]
e o contrato inicial identificador administrativo
[dado administrativo removido].

## Classificação e gate L0

A fonte mostra um defeito de baseline/harness: o default declarado do vanila
precisa ser materializado no comando do oráculo para evitar a alocação anômala.
O observável desta mudança é o vetor público de argumentos que o runner envia a
cada lado e a execução limitada desses dois casos. Não é mudança de semântica,
sintaxe ou morfologia do produto, nem de contrato público, default do produto
ou fase de pipeline. Portanto, nenhum Prompt L0 é criado ou atualizado.

Essa classificação seria refutada se a solução exigisse aceitar `--ppi` no
cristalino, mudar o rasterizador, o comparador, a fixture, o comportamento por
defeito ou a interface pública. Nesse caso P1394 deve parar e abrir passo
produtivo com owner L0 próprio.

## Obrigação congelada

Partindo do manifesto congelado, a única transformação permitida é:

1. em `P1137-X-003`, substituir somente `oraculo.args` por
   `["--color=never", "compile", "{source}", "{output}.png", "--format", "png", "--ppi", "144"]`;
2. em `P1138-X-002`, fazer a mesma substituição exata em `oraculo.args`;
3. manter byte-a-byte os dois `cristalino.args` atuais:
   - `P1137-X-003`: `["{source}", "{output}.png", "--format", "png", "--color=never"]`;
   - `P1138-X-002`: `["compile", "{source}", "{output}.png", "--format", "png", "--color=never"]`;
4. preservar semanticamente todos os demais campos, casos e ordem do manifesto;
5. alterar no runner somente o corpo de `validate_manifest`, acrescentando a
   política estrutural fail-closed P1394: identificar exatamente os dois casos
   PNG contratados; exigir em cada `oraculo.args` exatamente um par de tokens
   separados `--ppi`, `144`; proibir `--ppi` e qualquer token iniciado por
   `--ppi=` nos `cristalino.args`; e proibir ambas as grafias em qualquer lado
   dos demais casos;
6. fazer omissão, duplicação, valor diferente, grafia combinada
   `--ppi=144`, lado cristalino, caso não-PNG ou conjunto de casos PNG diferente
   produzirem um ou mais erros de validação antes de qualquer execução;
7. preservar todo o restante de `runner.py`, a fixture e ambos os binários
   congelados.

`144` explicita no lado vanila o default já declarado e medido. Ele não é
copiado para o cristalino, não é injetado globalmente e não autoriza outros
valores, formas equivalentes ou aplicação a outros casos.

## Observáveis e política de `Unknown`

O contrato decide somente:

- o delta estrutural exato do manifesto;
- o comando expandido de cada lado nos dois casos;
- a validação estrutural da matriz;
- a rejeição fail-closed das seis classes de mutação congeladas;
- a execução focal direta e reversa sob teto, incluindo exit, artefato PNG,
  dimensões e classificação já definida por cada comparador.

Manifesto que viole a política PPI falha como erro de schema antes de executar;
essa malformação não é aceita como `Unknown`. Para entradas estruturalmente
válidas, o resultado permanece `Unknown` e não satisfaz aceitação se houver
identidade divergente, ausência de teto comprovável, timeout, OOM, swap,
artefato ausente ou ilegível, erro de harness, desacordo direto/reverso ou
informação insuficiente. `Unknown` nunca vira `MATCH`/`DIFFERENCE` por default.
O estado final de cada caso é produzido pelo comparador vigente: P1394 não o
predetermina e não transforma sucesso de compilação em equivalência geral.

## Oráculos, ataques e gate discriminatório

O autor independente dos oráculos já congelou, sem patch candidato existente,
os testes e o recibo identificados na revisão causal R1. Eles verificam:

- positivo: exatamente os dois argv vanila terminam em `--ppi`, `144`;
- positivo: os argv cristalinos e todos os campos fora dos dois deltas são
  idênticos ao baseline congelado;
- positivo: `runner.py --validate-only` aceita o manifesto candidato;
- opaco: identidade divergente, teto ausente ou execução interrompida resulta
  em `Unknown`, nunca em sucesso;
- negativos: omitir o par em um ou ambos os casos; usar valor diferente;
  duplicar a opção; aplicá-la ao cristalino, a outro caso ou globalmente;
  usar a forma combinada `--ppi=144`; aceitar somente presença de PNG sem
  confirmar que o comando bilateral observado é o contratado.

Mutantes sem PPI são rejeitados por inspeção estrutural e **não são
executados**. O gate exige `mutation_score = 1.0`, repetição e reordenação com o
mesmo vetor. Uma mutação válida sobrevivente impede o selo.

## Disciplina obrigatória de memória

Nenhum comando que possa alcançar o vanila nos casos PNG é autorizado sem
teto externo efetivo. A forma canônica de execução é serial, com 2 GiB virtuais
e timeout externo de 60 s por comando focal:

```bash
bash -lc 'ulimit -v 2097152; exec timeout 60s /usr/bin/time -v python3 lab/parity/matrix/runner.py --case P1137-X-003 --output /tmp/p1394-p1137-x-003.json'
bash -lc 'ulimit -v 2097152; exec timeout 60s /usr/bin/time -v python3 lab/parity/matrix/runner.py --case P1138-X-002 --output /tmp/p1394-p1138-x-002.json'
```

Cada comando focal roda duas invocações por lado porque o runner verifica ordem
direta/reversa. Não há paralelismo. O executor deve registrar comando, HEAD,
working tree, timestamp, exit, duração, RSS máximo e swap. Se não puder provar
que `ulimit -v 2097152` foi aplicado, não executa e classifica `Unknown` com
`MEMORY_CEILING_UNPROVEN`. RSS acima de 85% do teto, crescimento sem estabilizar
ou qualquer swap interrompe a execução. Duas tentativas com a mesma causa e
sem informação nova encerram a calibração e reabrem o desenho.

Validação estrutural, testes Python focais e `crystalline-lint .` podem rodar
serialmente sob 2 GiB/60 s. Não há autorização para rebuild, teste workspace ou
execução do corpus completo neste passo.

## Regime Tekt e capacidades

Aplica-se o protocolo completo porque uma alteração pequena no observador pode
mudar a classificação de paridade e precisa rejeitar assimetrias perigosas sem
adaptar o contrato à solução.

- autor do contrato: lê AGENTS, skill e referências, P1393, manifesto e runner
  baseline; escreve somente este passo e relatório consolidado da limpeza; não lê a
  implementação candidata, não escreve manifesto/testes/código e não emite
  veredito;
- autor dos oráculos: lê este contrato e entradas congeladas; escreve somente
  testes/recibos designados; não lê o patch candidato antes de congelá-los;
- adversário: lê contrato, baseline e oráculos selados; produz mutações em área
  descartável; não corrige a solução e nunca executa mutante sem PPI;
- implementador: recebe contrato/oráculos selados; pode editar somente os dois
  arrays autorizados do manifesto e `validate_manifest` no runner lab-only;
  não edita contrato, baseline histórico, fixtures, comparadores ou oráculos;
- verificador: lê artefatos selados e candidato; escreve somente recibo/selo;
  não corrige o que julga.

A autoria deste contrato é segregada por capacidade, mas ainda não constitui
selo nem atestação da implementação. Contexto conversacional herdado: pedido do
passo P1394 e resultados P1393; nenhuma implementação candidata foi recebida.

## Critérios de aceitação

P1394 só pode ser selado por autoridade independente quando:

1. o contrato e seus oráculos foram congelados antes do patch candidato;
2. o manifesto parte do identificador administrativo congelado e difere somente nos dois
   `oraculo.args` exatos;
3. ambos os comandos vanila contêm exatamente um `--ppi 144`, e nenhum comando
   cristalino ou outro caso contém a adição;
4. o runner parte do hash baseline congelado e difere somente dentro de
   `validate_manifest` para materializar a política exata; fixture, binários,
   evidências P1393, testes e recibo de oráculos mantêm seus hashes;
5. validação estrutural passa;
6. os dois casos focais são executados serialmente sob 2 GiB/60 s, em ordem
   direta e reversa, sem OOM nem swap, e produzem PNGs legíveis;
7. cada execução registra proveniência e dimensões; resultados do comparador
   são relatados sem promover `Unknown` nem alegar paridade geral;
8. os nove testes P1394 congelados passam, todas as mutações negativas são
   rejeitadas antes da execução, `mutation_score = 1.0`, e opacos permanecem
   `Unknown`;
9. `crystalline-lint .` termina com zero violações sob teto;
10. nenhum produto, L0, fixture, comparador ou comportamento esperado mudou.

Este documento planeia a materialização; não é Prompt L0, selo, certificado ou
veredito final.

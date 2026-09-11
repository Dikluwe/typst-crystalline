# Passo 1343 — tornar o diff verificável por cápsulas determinísticas e concluir o binding P1342

## Objetivo

Desbloquear exclusivamente a implementação focal autorizada no Passo 1342,
substituindo o modelo ambíguo de união de ranges de diff por uma transformação
determinística e reversível de **cápsulas de substituição**. Depois de selar esse
novo modelo, reemitir o RED e implementar os hooks reais H00D/H00S/H01–H16.

Este passo não reabre nem fecha a matriz ampla lifecycle/profile P1340,
NT01–NT06, retenção, descarte, invalidação ou política terminal. Também não
autoriza mudanças de API pública, comportamento por defeito, compatibilidade ou
fase do pipeline.

Este documento é um passo de execução, não Prompt L0. O código continua
legitimado exclusivamente pelos dez owners já congelados em
`00_nucleo/diagnosticos/p1342-l0-freeze-r1.json`.

Regime: **executado sem atestacao de isolamento**. O workspace é compartilhado;
a segregação é de autoridade, entradas, ordem e artefatos.

## Autorização e baseline

O humano autorizou explicitamente em `2026-09-10` escrever e executar este
sucessor depois da condição de parada P1342 R5.

Medição em `2026-09-10T20:33:22-03:00`:

- HEAD: `2f42d64253547734564513a1159ee6b584c1c4b4`;
- working tree não commitada;
- SHA-256 de `git status --porcelain=v1 -z`:
  `263fed0ec287468cfbee57b1ebfdcbc265a5424c05cbf23196355ee5cd7f567c`;
- SHA-256 de `git diff --binary HEAD`:
  `f73da54bdbeb1bd086caeff259b9517bdbf0f1648efcb8119d36699f5bcf0be5`;
- stat: 72 arquivos, 10889 inserções e 830 remoções.

Entradas causais:

- `p1342-implementation-blocker-r1.json`, SHA-256
  `9a46cf931ea8f90f884b0b22cc93df574e17537bae807051cc26870f2b28a4f2`:
  o checker antigo exigia que o candidato permanecesse byte-idêntico ao
  baseline que precisava alterar;
- `p1342-contract-spec-r3.json`, SHA-256
  `07689c204f8741cdcfcc23bef9fa4af969dba6d991caaa93afaf117eb7ac73e7`;
- `p1342-contract-binding-r3.json`, SHA-256
  `8630a376350d374f854345c37282ac8fbb657f2f9a8506b7de47bda7b8276bf5`;
- `p1342-oracle-authorship-r5.md`, SHA-256
  `ca40925c678c7cfe7dfa19d93c6fb41b59c798c089f30b9457b7b8d741f3dad2`;
- `p1342-oracle-authorship-receipt-r5.json`, SHA-256
  `43eb0716901da8d49835bfef72508900dffb2461e3f5d7cb0796e6eaed8de713`,
  status `FOCAL_STOP_NO_FULL_EXECUTION`.

Os dez L0 e a fixture P1342 permanecem inputs válidos somente se seus hashes
continuarem iguais ao freeze e à fixture pinada. Nenhum candidato produtivo
P1342 existia na medição: o primeiro implementador parou antes de editar.

## Causa medida antes da decisão

O R5 rejeitou corretamente Y02 e Z01/Z02/Z06/Z07/Z12, mas rejeitou também o
controle positivo Y01 duas vezes pela mesma causa pública: o algoritmo genérico
de diff não conseguiu demonstrar que a união dos ranges declarados era
exatamente o diff real. Ajustar marcadores de fim não produziu delta e acionou a
condição de parada.

A informação necessária não está ausente do problema; está perdida pela
representação escolhida. Toda mudança P1342 pode ser expressa como inserção ou
substituição local sob cfg. Logo, não é necessário inferir um alinhamento global
entre textos repetitivos: basta exigir uma transformação inversa única cujo
resultado tenha o hash integral candidate-free já congelado.

## Decisão — cápsula de substituição P1343

Cada hunk candidato autorizado fica dentro de uma cápsula lexical única:

```text
// P1343-CAPSULE-BEGIN <capsule-id>
... corpo candidato fechado ...
// P1343-CAPSULE-END <capsule-id>
```

O manifesto pré-candidato define para cada `capsule-id`:

- caminho e símbolo owner;
- rows H00D/H00S/H01–H16 cobertos, ou papel `test-facade`;
- espécie `insert` ou `replace`;
- bytes exatos de reposição candidate-free, em codificação canônica, e seu
  SHA-256 — vazio para inserção;
- cfg obrigatório;
- âncoras anterior/posterior candidate-free e cardinalidade;
- restrições de escrita e callsite.

Marcadores são reconhecidos somente como comentários de linha completos no
nível lexical Rust, nunca dentro de string, raw string, char, comentário de
bloco ou macro token reconstruído. IDs são únicos; cápsulas não se aninham, não
se sobrepõem e aparecem somente nos arquivos allowlisted.

### Normalização inversa

O verificador de source, escrito e selado antes do candidato, lê os bytes live e
executa uma única operação determinística por arquivo:

1. valida UTF-8, pares BEGIN/END, IDs, ordem, não sobreposição e manifesto;
2. substitui cada cápsula inteira pelos bytes candidate-free pinados para seu
   ID, em ordem decrescente de offset;
3. calcula SHA-256 do arquivo normalizado;
4. exige igualdade com `baseline_file_sha256` já atestado.

Assim, qualquer byte fora das cápsulas, cápsula omitida/adicional, reposição
mentirosa ou alteração escondida torna o hash normalizado diferente. Não há
`SequenceMatcher`, união heurística, range autodeclarado nem escolha de
alinhamento. O controle Y01 precisa normalizar exatamente ao baseline e ser
`Preserved`; Y02 continua `Violated` por ausência de hook/coverage, mesmo que o
arquivo live seja igual ao baseline.

### Forma cfg e equivalência normal

- Uma cápsula `insert` contém somente itens/statements/fields protegidos por
  `#[cfg(p1339_observation)]` ou `#[cfg(all(test, p1339_observation))]`.
- Uma cápsula `replace` contém um ramo `not(p1339_observation)` cujos tokens
  efetivos correspondem aos bytes de reposição pinados e um ramo observado
  separado; não pode alterar o caminho normal.
- `cargo build` sem cfg, rustfmt/check sintático e a normalização integral são
  gates distintos e todos obrigatórios.
- Cabeçalhos de linhagem já ressellados por P1342 não são cápsulas candidatas;
  são baseline vigente. Nenhum L0 é alterado neste passo salvo se uma auditoria
  refutar concretamente a suficiência das cláusulas já congeladas — nesse caso,
  parar por ADR-0127 quando aplicável.

## Verificador de source pré-autorado

Antes do selo deve existir um verificador determinístico independente, com
script e fixtures pinados, que:

- compõe os 19 rows e 18 hooks aplicáveis por célula;
- faz a normalização inversa diretamente dos bytes, sem confiar em flags,
  hashes de símbolo/âncora ou ranges fornecidos pelo DTO/candidato;
- deriva os hashes de arquivo, cápsula, símbolo e âncora dos bytes lidos;
- confirma âncora e cápsula dentro do símbolo/branch declarado por scanner Rust
  focal, rejeitando decoys `cfg(test)` e texto em strings/comentários;
- exige exatamente um writer append-only e projeção read-only nos símbolos
  allowlisted;
- recebe baseline, candidate root e manifesto por argumentos controlados pelo
  verificador final; o DTO e a fachada não escolhem path, hash ou resultado;
- emite evidência fechada reproduzível, mas o checker reexecuta a normalização
  sobre os mesmos bytes em vez de confiar em booleanos do envelope.

A prova estrutural é focal, não um parser Rust geral. Sintaxe Rust completa é
gateada por `cargo test --no-run`, `cargo build` normal e rustfmt. Se o scanner
focal não conseguir distinguir uma cápsula real de um decoy, o resultado é
`Violated`, não `Unknown` nem relaxamento.

## Reemissão segregada

1. **Autor do contrato:** recebe este passo, baseline e diagnósticos P1342; cria
   contrato/manifesto P1343 candidate-free, sem ler patch futuro.
2. **Autor do verificador/oráculos:** cria normalizador, fixtures positivas,
   negativas e opacas. Deve preservar todos A/X/Y/Z relevantes e acrescentar
   ataques a marcador em string/comentário, cápsula aninhada, ID duplicado,
   reposição falsa, alteração fora da cápsula, cfg removido e branch normal
   alterado.
3. **Adversário:** muta bytes e manifesto sem corrigir o normalizador.
4. **Pré-verificador:** executa uma única vez o corpus completo após o focal
   estar verde, exige score `1.0` e emite `SEALED_FOR_IMPLEMENTATION` ou para.
5. **Testador A/B:** após selo, reemite o teste RED para o contrato P1343; não lê
   o patch candidato.
6. **Implementador:** recebe apenas L0, contrato/oráculos selados e RED; envolve
   cada mudança nas cápsulas autorizadas, implementa ledger/hook/fachada real e
   não edita inputs protegidos.
7. **Verificador final:** normaliza o candidato contra o baseline, produz a
   evidência externa, executa o checker e os gates finais sem modificar o que
   julga.

Troca de informação ocorre somente por artefatos canônicos e hashes. A frase de
regime permanece `executado sem atestacao de isolamento`.

## RED e implementação retomada

O teste R1 P1342 é histórico e inválido para julgar este contrato. O novo teste
deve:

- continuar usando a fixture de 178 bytes e três World/Source/challenges reais;
- exigir `p1342_run_fixture_for_test` ou sucessor nominal fixado no contrato;
- receber raw pré-projeção e DTO runtime da fachada;
- executar o checker selado com paths/hashes do baseline, manifesto, source
  verifier e candidate root fornecidos pelo testador, não pela fachada;
- falhar candidate-free por ausência específica da fachada/hook, sem executar o
  corpus completo.

Depois do RED, a implementação retoma as obrigações originais P1342: carrier
privado no `Func`, handle no `EvalContext`, emissão real H00D/H00S/H01–H16,
CounterUpdate/Location/snapshots reais, dispatch With/inner, SyntaxNode body,
Dict IndexMap tipado, raw freeze antes da projeção e normal/repeat/reverse
realmente frescos. Não usar side table global, callback extra, busca por nome ou
output, ledger pós-hoc ou fixture P01 embutida.

## Aceitação

- L0 freeze e fixture P1342 intactos;
- contrato/manifesto/verificador P1343 escritos antes do candidato;
- Y01 `Preserved`, Y02 e todos A/X/Y/Z negativos `Violated`, opaco somente
  `Unknown` após predicados não-payload;
- ataques novos de cápsula todos `Violated`, mutation score `1.0`;
- normal/repeat/reverse concordantes;
- pré-selo válido e RED reemitido;
- candidato normaliza byte a byte aos dez hashes baseline após remover/substituir
  cápsulas;
- hooks e coverage vêm dos callsites produtivos reais;
- teste focal GREEN repetido; build normal, rustfmt, `git diff --check`,
  V5/V15/V26 e `crystalline-lint .` sem violações novas;
- certificado independente `ACCEPTED` restrito ao binding focal.

O certificado não fecha P1340/P1339, NT01–NT06, retenção geral ou política
terminal.

## Budget e condições de parada

- uma versão inicial de contrato e no máximo 1 revisão contratual;
- no máximo 1 revisão focal do normalizador por nova classe de falha;
- calibração autoral executa somente recortes focais; **a única execução completa
  antes do selo pertence ao pré-verificador independente**;
- uma execução completa na verificação final;
- no máximo 2 ciclos de correção do candidato, cada qual focal e seguido primeiro
  pelo teste afetado;
- parar após duas tentativas com o mesmo vetor e causa;
- parar se uma mudança necessária não puder ser expressa por cápsula cuja
  normalização recupere exatamente o hash baseline;
- parar e pedir decisão humana diante de mudança ADR-0127, score menor que 1.0,
  input protegido alterado ou necessidade de afrouxar um negativo.

Não conceder novo waiver implícito, não reclassificar sobrevivente como inválido
por conveniência e não executar corpus completo durante autoria/calibração.

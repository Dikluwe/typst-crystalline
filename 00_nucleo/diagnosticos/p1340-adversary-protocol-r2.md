# P1340 — ataques focais R2 e protocolo de handoff

Executor `/root/p1340_adversary`, sem leitura do candidato P1340. Executado
sem atestação de isolamento. Mantidas as capacidades do manifesto inicial;
nenhuma produção, intenção, contrato, oráculo ou veredito foi escrito pelo
adversário. Os arquivos desta família são entradas adversariais, transportes
e recibos crus. O verificador decide elegibilidade e rejeição.

## Entradas e predecessor causal

- Baseline P1340: `0869202e774bd7b76278365aac7292d45a1c930be42cf83270845c985cb5000a`.
- Aprovação terminal: `b47f3ba08307a2bc01c720c8ac218da2091d8965e417d2b34ed77936f2f714c6`.
- L0 child aprovado: `3094bd8cda8d1971e765cf91cbb110d376ff713e4348ae8f4bf8214e33b45b9b`.
- Contrato R2 recebido após congelar os inputs adversariais, antes de
  qualquer leitura de candidato: `9ea5ecc71e26e1ee707fe4eecdf6d5cd2768f2e2be8cd6e6d0605b1ef38ae434`.
- `p1340-adversary-focal-inputs-r2.json`:
  `d4c5dfc90fdef655fc14c873c502acfeb85e5d89506751fdc04d1c4cb2fbcbca`.
- `p1340-adversary-focal-harness-r2.rs`:
  `40560192cf55bbde4dacd20f49c2d13f4ed51a75993ad52c3bde5553402525a1`.
- Harness mecânico reutilizado P1339:
  `09c467410cb0f4e37d11dca47c15f4913271f74e7b1400457513d76cd9c373c0`.

O contrato novo usa Func::element em Metadata/query como input opaco; a sonda
L1 adversarial usa o DynElement real Callout em State. São carriers distintos
com ramos privados Unproven distintos. Não afirmar que esta execução já
exercitou o input terminal T01/T02 do contrato: isso permanece F, com o
constructor opaco real e os predicados R2 congelados.

## Cópia e ligação usadas

Baseline L1 copiado para `/tmp/p1340-adversary-baseline.IqcW7q` depois de
verificar os hashes de seus arquivos modificados contra `before.modified`.
O único novo fonte L1, array.rs, foi confirmado separadamente contra
`before.new_sources`:
`fc90dd20da8098416d90c3ea1293c457bde1c147826258a9e7c28534c12ce038`.
O baseline registra HEAD `2f42d64253547734564513a1159ee6b584c1c4b4` mais
working tree, diff/stat integral e fontes novas. Não houve cópia de candidato
L3 nem leitura de materialization/context.

Na cópia, workspace limitado a 01_core; Cargo.toml ficou
`c86e34ba21fe67021b4b49bc6b913ae96f0eaff6e9a730e687133b2bf9aa2c40` e
Cargo.lock
`7b2ee08c3718b37573d7ac5d1bb5d4d88a990d06f9ed80d82fa5d61b52ff33f7`.
O lock foi resolvido offline ao retirar os outros membros; subsequentes
execuções usaram `--locked --offline`. Target exclusivo:
`/tmp/p1340-adversary-target.DvNoJ7`. A ligação insere, antes do primeiro use
de eval/mod.rs, somente:

```rust
#[cfg(test)]
mod p1340_adversary_focal_binding {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../00_nucleo/diagnosticos/p1340-adversary-focal-harness-r2.rs"));
}
```

Fonte controle com essa ligação:
`1fc8ceacb962b492a4cce180e54137d8f231461df5df1b9bb4c95f402108b6e5`.
O harness chama as APIs reais e observa o discriminante do comparador real
usado por elas. Não há mini-coordenador, decisão de exportação ou predicado
semântico paralelo. Todos os fixtures permanecem idênticos entre modos.

## Mutantes reais e testemunhas crus

NM01 altera exclusivamente o retorno do impl de ObservationEq para DynElement:
`ObservationRelation::Unproven` vira
`/* P1340_NM01 */ ObservationRelation::Same`.
Fonte: `462302cd685e37ac3017d87cb8bca58cde7015ebcf23386040512fcb575a7bab`.

NM02 parte do controle e, imediatamente após
`ctx.introspector = candidate.clone()` no replay, acrescenta:

```rust
if matches!(read.request, ContextReadRequest::LocationPosition { .. }) {
    /* P1340_NM02 */ ctx.introspector = self.introspector.clone();
}
```

Fonte: `f7bfd73288e5c97ec91727a0c945ed61a1448acb731b733157882929aa2fe2f4`.
Nunca executar ambos mutantes simultaneamente. A cópia ficou em NM02 após
a última medição; o target contém o executável NM02 correspondente.

| Input | Controle real | Delta NM01 | Delta NM02 |
|---|---|---|---|
| mixed counter antes/depois opaco | self=false, candidato=false, detalhe counter presente, folha Unproven | self=true e folha Same | inalterado |
| opaco sem mudança fechada | self=false, candidato=false, detalhe vazio | ambos true | inalterado |
| posição muda | self=true, candidato=false, detalhe vazio, relação Different | inalterado | candidato=true e relação Same |
| closed Same e closure retida | ambos true, relações Same | inalterado | inalterado |
| closed Different | self=true, candidato=false, detalhe presente | inalterado | inalterado |
| erro original estável | ambos true, corpo Err com diagnóstico original | inalterado | inalterado |

Os valores são suportados pelos recibos diretos abaixo, com timestamps,
comandos, fonte e binário, não por uma execução simulada de pipeline.

| Recibo | SHA-256 | SHA-256 do testbin |
|---|---|---|
| p1340-adversary-direct-control-r2.json | 638c043767cf07b67ce357c667ca447fb61a6de4cdd2e4b742e7adec7067843d | d4334a142ee88ca4a3d1ed4d4d9d02c8d8b3e373a447274bb490cccf6e66f17e |
| p1340-adversary-direct-nm01-r2.json | f34c5ddeac44326a4b9ed4256f45711ff8df70892eef76c31e377430fdeae93c | 33ace889805cd9f1adac049e373b86714e378bf9adfa747f210225bd2faee3c2 |
| p1340-adversary-direct-nm02-r2.json | d2147eef69ca49701762ac01025cf9a96f5b45727521f2ff218f1ff87cd27288 | 4f7b942f4cbc666a95e06eccbe98b823a1628585e18797b99b9566b154e8c43f |

Cada confirmação direta terminou exit 0. Os rows são integralmente iguais
aos respectivos recibos Node anteriores. Os rows controle e control-repeat
também são idênticos. A inversão da posição da leitura opaca está congelada
nos próprios inputs A01; não se afirma execução de uma ordem global reverse.

## Incidente de transporte e custo

O runner Node registrou simultaneamente status 0, stdout completo com teste
concluído, e `error: spawnSync cargo EPERM`. Os arquivos `focal-*-raw-r2.json`
foram preservados. A causa do metadado contraditório não foi determinada.
O adversário não apagou o error nem o converteu em sucesso por inferência.

Os receipts diretos vêm da ferramenta shell, sem child_process Node. A
confirmação foi necessária porque o runner inicial não guardava hash do
testbin por modo, e o target já tinha substituído o executável anterior.
As novas execuções confirmam as mesmas observações com proveniência própria;
não consertam retroativamente o recibo anterior.

Houve custo administrativo além do budget inicial: smoke de compilação,
repetição para persistir stdout NM01 que inicialmente só foi enviado ao canal
da ferramenta, e primeira coleta direta controle descartada por erro de índice
do parser JSON. Esse parser foi corrigido de índice constante para tamanho do
prefixo; nenhuma entrada nem predicado semântico mudou. Ao todo foram dez
invocações focais (mesmos oito inputs), incluindo essas execuções sem crédito.
São duas causas negativas e nenhuma revisão semântica; nenhum corpus global.
O custo extra foi reprodução/transporte, não calibração repetida para mudar
vetor. Não executar novas rodadas L1: a testemunha discriminatória já existe.

## Protocolo mínimo para revisão e selo sem circularidade

1. Verificador confere os pins do L0 aprovado, contrato R2, harness terminal,
   baseline e estes inputs. Confirma que a nova política é a autorizada e
   que não houve candidato nas entradas dos autores.
2. Avalia os raws L1 diretos e as duas mutações compiláveis contra a fonte
   aprovada. O discriminante privado permanece dado observado; bool false
   nunca é recodificado como Unknown. O adversário não atribui mutation score.
3. Reuso C histórico fica restrito às famílias inalteradas e à decisão
   explícita do verificador. Esta fase não afirma ter testado a coordenação
   terminal inexistente. Não escrever solução temporária para fabricar C.
4. Congelar NT01–NT06 e A03–A08, os casos T01–T06 do contrato e os fixtures
   lifecycle históricos. Se aceitos, a ligação tardia recebe somente a
   capacidade de projetar entradas/estados reais do candidato. Nomes privados
   podem ser ligados depois; causas, expectativas e observáveis não mudam.
5. Na fase F, executar os ataques no MESMO algoritmo produtivo: NT01→T02,
   NT02→T03, NT03→T04, NT04/NT05→T01, NT06→T01/T02/T04; A03 exige os dois
   filhos válidos, não o erro T05. T05 exige fonte do ramo de impossibilidade
   mais operação terminal real. Mensagem, span, trace/hints, precedência,
   sinks, número de tentativas e export_calls são campos independentes.
6. Mutante sobrevivente, callback opaca executada, falta de binding real,
   caso obrigatório Unknown ou reconstrução de resultado no adapter bloqueia
   F. Não importar o score de L1 como score da política L3. Verificador
   emite os vereditos e o selo; adversário apenas entrega raws e mutações.

Esta entrega conclui o preparo focal anterior ao candidato. Os ataques contra
a coordenação produtiva continuam pendentes, sem crédito C fictício.

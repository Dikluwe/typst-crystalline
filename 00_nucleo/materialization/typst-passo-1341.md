# Passo 1341 — separar expectativas semânticas de identidades runtime

## Objetivo

Desbloquear e concluir as obrigações lifecycle que o Passo 1340 encerrou como
parciais. O problema a resolver é protocolar: R6b misturou expectativas ex ante
com identidades que só existem durante cada execução e, por isso, não pôde ser
selado nem legitimou implementação adicional.

Este documento é um passo de execução, não Prompt L0. Código continua
legitimado somente pelos owners em `00_nucleo/prompts/`.

## Baseline e causa medida

Baseline: HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não
commitada. O estado exato é congelado pelo manifesto P1341 antes de qualquer
contrato ou edição.

O veredito independente `p1340-verifier-final-r6.json`, SHA-256
`519d4791626c7a3bfe325d5b4d2bba302b57b75b5a13627dee8838df2f0975a0`,
é `NO_SEAL_STOP_REQUIRES_PROTOCOL_REDESIGN`. O adversário R6b matou 19 de
41 mutantes e deixou 22 sobreviventes em oito classes. O autor das
expectativas demonstrou que `callsite_witness_id`, `value_id`, `snapshot_id`,
`location` e `span` dinâmicos não podem ser congelados uma vez por caso para
144 execuções com Worlds e registros frescos.

## Decisão de protocolo

O sucessor separa três planos:

1. **Expectativa semântica estável:** por caso e perfil/ordem quando material,
   usa coordenadas deriváveis da fonte e da intenção: índice/offset do
   `#context`, papel de callback, fase, chave/ação de counter, papel lexical do
   span, multiplicidade por tentativa e caminho tipado de Dict. Nunca contém
   ponteiro, Location, snapshot ou id runtime esperado.
2. **Ledger runtime real:** a instrumentação `cfg(test)` emite identidades
   dinâmicas, Locations, snapshots, spans e relações de retenção. O checker
   prova bijeções e continuidade dentro da própria célula, sem comparar esses
   valores com tokens fabricados pela fixture.
3. **Manifesto estático de binding:** o verificador pina owners/callsites reais
   e associa cada papel semântico a pelo menos um hook alcançável. Esse
   manifesto prova completude de ligação; não é fornecido pelo DTO candidato.

`Unknown` só é permitido para payload deliberadamente opaco após estrutura,
cobertura externa e consistência causal terem sido verificadas. Ausência,
omissão coerente, ledger malformado ou cobertura autodeclarada é `Violated`.

## Fronteira de implementação

- Atualizar primeiro os Prompt L0 dos owners realmente tocados, descrevendo
  exclusivamente instrumentação `cfg(p1339_observation)` sem efeito no build
  normal. Resselar os hashes derivados e validar V15/V26.
- Completar hooks privados do lifecycle em L3: início/fim de execução,
  descarte/invalidação com causa, decisão de validação, retenção, snapshots,
  sinks e retorno.
- Nos callsites L1 reais, observar callback, Func, fase, span e contexto sem
  executar callback adicional nem inferir origem por nome/igualdade.
- Transportar origem lexical somente em carriers `cfg(test)` que preservem
  clone/mapeamento quando necessário; não adicionar campo/API normal.
- Projetar Dict como variante tagueada mais lista ordenada de pares.
- Integrar harness externo sem mini-orquestrador e executar a matriz real
  somente depois do selo.

## Gates

1. Contrato, expectativas estáveis e manifesto de binding independentes.
2. Gate discriminatório sintético: todos os 41 mutantes R6b mais mutantes das
   oito classes sobreviventes; score obrigatório `1.0`; normal/repeat/reverse.
3. Selo independente antes de código.
4. L0 antes da implementação; RED real do binding; GREEN focal.
5. Matriz lifecycle/profile real, NT01–NT06, controles públicos e mutações
   produtivas aplicáveis.
6. Repetição, ordem inversa, build normal, rustfmt, `git diff --check`, V15/V26
   e `crystalline-lint .`.
7. Certificado final independente. Nenhum `Unknown` obrigatório, mutante
   sobrevivente ou input protegido alterado permite fechar P1341/P1340.

## ADR-0127 e condição de parada

A intenção atual é apenas observabilidade privada `cfg(test)` e execução dos
gates já aprovados; não muda API, entidade normal, default, compatibilidade ou
fase. Se o binding exigir qualquer uma dessas classes, parar antes do código e
pedir novo gate humano. Duas revisões sem ganho causal, input não autorável ou
novo mutante coerente após o budget obrigam novo diagnóstico, não relaxamento.

Regime: **executado sem atestação de isolamento**. O workspace é compartilhado;
capacidades, entradas e escritas de cada papel são registradas, mas isolamento
técnico não é alegado.

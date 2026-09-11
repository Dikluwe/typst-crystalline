# Prompt L0 — conversão limitada `array(bytes)`
Hash do Código: 11f935ba

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stdlib/primitive-calls.toml sha256:761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736

**Camada:** L1
**Ficheiro proprietário:** `01_core/src/compiler/stdlib/primitives_constructors/array.rs`
**Incompleto deliberado:** somente Bytes; conversões de Array e Version e
demais formas vanilla ficam fora do P1339, sem passo de completação autorizado.

## Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitada,
com proveniência em `diagnosticos/p1339-array-prerequisite-measure-r2.json`:
`compiler/eval/call_dispatch.rs:1791-1830` não possui Array; chamadas com bytes
chegam ao diagnóstico de construtor ausente. Vanilla ratificado a51e02804,
`foundations/array.rs:164-169,1176-1179`, converte cada byte para Int sem sinal,
conservando ordem e comprimento. A medição confirma vazio, extremos, alias,
spread e rejeição de sobras. O parecer independente
`p1339-verifier-array-prerequisite-gap-r1.json` identifica esse pré-requisito
nos testes congelados de float.to-bytes.

## Decisão autorizada

Autorização: `diagnosticos/p1339-array-authorization.md`. Implementar função
interna `native_array_bytes` no ABI de constructor do núcleo compartilhado.
Recebe Args já avaliados, sem reavaliar expressões, callbacks ou I/O. A rota
pública é selecionada somente quando o primeiro positional é Value::Bytes.

Consumir exatamente esse positional. Rejeitar a primeira ocorrência restante
na ordem causal de Args: `unexpected argument` para positional,
`unexpected argument: <nome>` para named. Usar span da ocorrência; fallback
para Args.span somente quando detached. Não descartar sobras nem mudar
prioridades de avaliação do dispatcher.

Produzir Value::Array com um Value::Int por byte no domínio 0..255, sem
inversão, coerção assinada, truncamento, transformação de endian ou texto.
Bytes vazio produz Array vazio. Bytes permanece imutável. A unidade pode
validar defensivamente a precondição e devolver
`type array does not have a constructor` em Args.span sem Bytes; isso não
amplia a rota pública.

Não descobrir métodos de Array, alterar Type/Value/Args ou implementar
`array(version)`, `array(array)`, missing ou named-only públicos. Essas
formas permanecem legadas pelo dispatcher. Não expor API Rust pública nem
tornar Type um Func para habilitar With/callbacks novos.

## Aceitação

RED→GREEN: vazio, todos os bytes em ordem, alias, spread, sobras, efeitos
avaliados uma vez e controles não Bytes. Manter programas/oráculos originais
de float.to-bytes. Ordem/sinal/comprimento incorretos, descarte de sobras ou
aceitação de Version refutam o recorte. Exigir sucessão discriminatória antes
do código, ownership 1:1, V15/V26, build/testes e lint. Não afirmar paridade
geral do construtor array ou conclusão do P1339 a partir deste pré-requisito.

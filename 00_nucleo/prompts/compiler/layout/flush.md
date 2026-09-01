# Prompt L0 — `compiler/layout/flush` — realização de `place.flush`

**Estado:** CONTRATO P1292 AGUARDA SELO ADR-0127 — sem consumer e sem
`Hash do Código` até a materialização posterior ao gate humano.

**Camada:** L1
**Alvo planejado:** `01_core/src/compiler/layout/flush.rs`
**Entidade:** `entities/elements/flush.md`
**Vanilla ratificado:** `a51e02804`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## Medição anterior à decisão

O layouter cristalino já mantém `floats_pending` em ordem e possui
`flush_pending_floats`, usado nos limites de página/finalização. O recibo
P1292 mediu um bottom float anterior ao marcador indo para a página 2, o
conteúdo seguinte iniciando na página 3, e um segundo float criado depois do
marcador permanecendo posterior e sendo emitido na página 3. Sem o marcador,
o conteúdo seguinte permaneceu na página 1.

### Amendment-8 — hipótese de drain direto refutada

A implementação literal deste contrato como
`flush::layout -> flush_pending_floats()` foi medida no binário candidato e
refutada: no caso prefix/suffix produziu uma página, enquanto o vanilla
ratificado e o oracle selado exigem três. O drain direto emite todo o buffer na
página corrente sem terminar/avançar a região; logo não consegue colocar o
prefixo antes do flow posterior com a paginação causal exigida.

No vanilla ratificado, o collector conserva `Flush` como child próprio do
flow. O distribuidor, ao encontrá-lo com floats pendentes, devolve
`Stop::Finish(false)`: termina a região sem consumir visualmente a sentinela e
continua somente depois de o mecanismo paginado realizar os floats. Isto é
semântica observável de fluxo; o tipo `Stop` e o algoritmo upstream são apenas
evidência mecânica, não obrigação de cópia.

## Decisão — forma B e ponto exato do fluxo

Este consumer contém uma free function descendente da camada de render:

```text
flush::layout(layouter, &FlushElem)
```

Ao encontrar `Content::Flush`, o dispatcher estático e exaustivo delega a
esta função. Ela captura a fronteira de ocorrência do prefixo já pendente e
chama somente o hook interno `finish_float_prefix_at_marker` do owner paginado
`compiler/layout/cursor.md`. É proibido chamar diretamente
`flush_pending_floats` neste consumer ou duplicar aqui seleção, posicionamento,
clearance, top/bottom ou paginação.

Se não há float pendente, o hook é no-op exato. Se há, a sentinela permanece
logicamente no ponto do fluxo enquanto o owner paginado termina/avança as
regiões necessárias e realiza o prefixo pelo distribuidor normal. O conteúdo
seguinte só começa quando esse prefixo deixou de estar pendente, na primeira
região corrente que o mecanismo normal permitir. A fronteira capturada é
imutável durante a operação: floats criados depois do retorno não recebem
crédito retroativo e continuam para a realização posterior.

### Amendment-9 — retorno somente sobre região estabilizada

“Prefixo deixou de estar pendente” não basta se a sua inserção ainda não foi
incorporada às reservas da região. O hook só retorna depois de o owner
`compiler/layout/cursor.md` estabilizar a composição contra as inserções
top/bottom realizadas por `compiler/layout/place.md`. A sentinela então é
consumida e o próximo child vê a região efetiva já reduzida, sem que Flush ou
Cursor inspecionem o seu tipo ou altura.

Flush não decide se o próximo child cabe e não força uma página após o
marker. `clearance: 0pt` pode permitir continuação na mesma região; o default
vigente `1.5em` pode esgotá-la e causar progresso pelas regras normais. Toda
reserva, alinhamento, fitting e posição permanecem no owner Place; toda
recomposição/checkpoint/avanço permanece no owner Cursor.

A sentinela não chama `flush_line`, não cria item, não avança cursor por si,
não cria quebra de página artificial e não reordena top/bottom. Paginação,
clearance, alinhamento e ordem são os do mecanismo paginado existente; qualquer
avanço ou nova página é consequência exclusiva de o prefixo ainda estar
pendente. Em sub-frame, atua somente sobre o buffer e o distribuidor do
Layouter/sub-layout ativo e não pode terminar região nem drenar estado de um
owner exterior.

Toda a decisão de efeito da sentinela vive aqui. `compiler/layout/mod.rs`
mantém apenas declaração do módulo e arm de uma linha. Não há `dyn`, vtable,
registry, importo `entities→compiler` ou alteração dos owners de `place`.

## Aceitação

Casos RED devem provar: prefixo anterior drenado no ponto; posterior não
antecipado; nenhum item/cursor próprio; ordem e clearance preservados; e
comportamento seguro em fluxo principal, sub-frame e fronteira de página.
O caso prefix/suffix conserva exatamente três páginas; no-flush conserva o
flow posterior na página 1; top+bottom conserva duas páginas e sua ordem;
no-floats é idêntico ao controle sem marcador. Direct drain, quebra
incondicional, retorno antes da recomposição estabilizada ou algoritmo de
distribuição local neste arquivo violam o contrato.

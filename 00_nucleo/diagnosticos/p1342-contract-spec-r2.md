# P1342 — revisão R2 do contrato de binding real

Regime: **executado sem atestacao de isolamento**. Veredito de autoria:
**CONTRACT_REVISED_NOT_SEALED**. R1 permanece intacto; esta revisão responde ao
parecer adversarial R1 sem ler candidato e sem autorizar implementação.

## Fecho dos bloqueios

- B01: o manifesto R2 inteiro é pinado por hash, tem metadados e enums
  closed-world, âncoras resolvidas por símbolo e branch Rust, exclusão explícita
  de `#[cfg(test)]` decoy e cobertura runtime por recibo. Uma ocorrência textual
  isolada não prova binding.
- B02: `session_carrier` percorre toda a célula; `occurrence_carrier` é idêntico
  da criação até replay-exit, incluindo ambos os dispatches, With, closure e
  `syntax-body`. Location de H11 é a mesma em H12A/H12B. Snapshot pre repete
  identidade/valor; snapshot post vem do estado validado imediatamente antes do
  history append.
- B03: papéis, eventos, refs, dados e tipos são totalmente fechados. Cada append
  tem recibo encadeado; o ledger cru é congelado e inspecionado antes da
  projeção, que é read-only. JSON final coerente sozinho nunca prova origem.
- B04: cada execução recebe challenge fresco de 32 bytes fornecido pelo
  verificador. IDs são tipados e namespaced pelo commitment do challenge;
  normal/repeat/reverse têm challenges e conjuntos de IDs disjuntos.
- B05: hash/binding, prova runtime, schemas e toda continuidade causal são
  verificados antes da opacidade. Só o payload witness deliberadamente opaco
  pode resultar em `Unknown`; qualquer falha anterior é `Violated`.

Os ataques A06 e A07 continuam negativos válidos. A06 só é rejeitado quando os
IDs fixos falham o namespace challenge-bound entre runs. A07 só é rejeitado
quando uma rechain do DTO não corresponde ao snapshot cru/recibos congelados;
se o candidato reconstruir também o raw antes do freeze, a inspeção obrigatória
de source e dos hooks precisa demonstrar a fabricação. Não se descarta nenhum
dos dois por conveniência classificatória.

## Challenge, append e preprojeção

O driver independente fornece o challenge antes de construir o ledger. Em cada
hook, o único writer apenda atomicamente o evento e um recibo SHA-256 que inclui
challenge, sequência real, recibo anterior, hook e digest canônico do evento.
Em H16, depois de impedir novos appends, o driver lê diretamente o
`RawLedgerSnapshot`, grava seu digest e só então permite a projeção. Após a
projeção, o mesmo raw deve conservar o digest.

Essa cadeia é prova necessária, não suficiente: o challenge fica conhecido
durante o run. Por isso o verificador candidato também inspeciona o source,
confirma um único writer, projeção read-only, calls nos branches produtivos,
coverage da fixture e ausência de construção/rechain post-hoc. O checker nunca
promove um DTO final a prova de origem por si só.

## Attempt-kind e R

`attempt-kind` deriva exclusivamente do recibo H00D (`Session::discover`) ou
H00S (`Session::realize`) imediatamente anterior a H01. Discovery tem H11=0 e
R=0 no fragmento focal. Selected tem H11=1 quando seu resultado é incorporado,
ou zero com resultado fechado não incorporado.

Para uma célula de origem, R é a quantidade de recibos H12A que selecionam o
carrier daquela ocorrência. R não é campo do DTO nem é contado de eventos
autodeclarados. Replay posterior pode apendar à célula de origem durante outra
célula consumidora; por isso H03 registra resultado, mas só H16 fecha células.
H12B=R, H13=2R na ordem With/Closure, H14=R, body enter/exit=R e witness=R.

## Divisão de responsabilidade

O checker valida hashes selados externos, manifesto exato, schemas fechados,
âncoras estruturais, cadeia de recibos, raw/projection digest, challenge e
identidades entre runs, attempt-kind/R, cardinalidades, carriers, endpoints,
Location, snapshots, spans, Dicts tipados, causalidade e precedência de
`Unknown`.

O verificador candidato fornece challenges fora do controle do candidato,
inspeciona o ledger em memória antes da projeção, confirma o mesmo digest
depois, audita o único writer e todos os callsites no source, verifica coverage
real da fixture, segue os valores reais de Location/snapshots/carrier e confirma
que observação não executa callback extra. Hash chain sem essa inspeção não é
aceite como origem produtiva.

## Autoridade e limite

Não é necessária mudança L0: os dez owners congelados já autorizam exatamente
o ledger append-only, handle cfg-only, identidades locais e projeção sem
fabricação; a R2 especifica a evidência test-only dessas obrigações.

Estado permanece `NOT_SEALED`. Um autor independente ainda deve revisar
oracle/checker/corpus e um pré-verificador deve obter score 1.0. Nada aqui fecha
lifecycle/profile P1340, NT01–NT06, retenção/descarte/invalidação geral,
política terminal, paridade geral ou mudança pública/default/fase.

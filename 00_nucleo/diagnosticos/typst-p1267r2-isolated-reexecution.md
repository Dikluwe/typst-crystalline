# P1267-R2 — reexecução segregada e comparação com R1

**Estado:** EXECUTADO — `R2-CANDIDATE-UNSEALED-REFINEMENT-REQUIRED`

**Produto/L0:** nenhuma escrita e nenhuma promoção

## Resultado

A reexecução independente não mudou o núcleo da readjudicação P1267. Autor e
analista, ambos iniciados com `fork_turns=none` e sem acesso autorizado aos
artefatos R1, voltaram a concluir que:

- G04 deve separar a morfologia dos stops originais da mecânica dos stops
  adaptativos, preservando ordem, suporte da curva e envelopes de cor/alpha;
- G10 é limitado por intervalo, não por um cap global de 64;
- S20 é stroke degenerado com região pintada positiva a preservar;
- S21 é fill sem área e deve continuar sem paint positivo;
- P1266 permanece histórico, `Unknown` não sucede e P1268 continua obrigatório.

Portanto a mudança conceitual é baixa e a mudança de produto é nula. A
segregação, porém, produziu uma melhoria material de precisão contratual.

## Achados que R1 não explicitou

O analista derivou duas classes de trabalho no algoritmo adaptativo. Uma árvore
máxima por intervalo possui 127 decisões de midpoint e ainda 126 amostras da
próxima fronteira, totalizando no máximo 253 invocações do sampler exato. O
contrato R2 do autor publicou apenas as 127 decisões; o verificador exige
publicar também `q_i <= 253`, sem chamar 127 de custo total.

O verificador também exigiu definir `M` como a sequência pública efetiva que
entra no adaptador (`effective_public_stops`/G04A), e não como a cardinalidade
lógica de cores da matriz. Essa distinção revelou uma falha localizada na
evidência independente: ela publicou `m=904` e limite agregado `51808`, obtidos
de `logical_stops`. A recontagem verificada é `M=1320`, `E=11188` e limite
conservador agregado `78432`. O número de violações do limite derivado continua
zero, então a refutação do falso cap global sobrevive; o recibo errado não pode
ser usado como prova até ser sucedido por um artefato novo.

Por fim, G04 precisa distinguir o valor estrutural no ponto coincidente do
comportamento epsilon à direita. A expressão genérica `right-continuity` não
congela adequadamente os dois observáveis.

## Papéis, ordem e gates

1. `/root` congelou passo, baseline `a51e02804`, P1266, L0s, ADRs, fontes e
   capacidades no manifesto de entrada.
2. O autor do contrato escreveu somente seis artefatos em
   `/tmp/p1267r2/author`, sem contexto herdado e sem outputs R1 autorizados.
3. Em paralelo, o analista escreveu somente derivação e recibo em
   `/tmp/p1267r2/evidence`, sem ver o contrato candidato.
4. Após freeze por hash, o verificador recebeu os dois conjuntos, rechecou
   fontes e escreveu somente veredito/recibo em `/tmp/p1267r2/verifier`.
5. Os dez artefatos foram materializados byte a byte em
   `00_nucleo/diagnosticos/`; os hashes permanecem iguais aos temporários.

Passaram os gates de identidade das entradas, freeze dos oito artefatos
julgados, estrutura TSV, G04 core, limites G10 centrais, S20, S21, política
`Unknown`, imutabilidade P1266 e ausência de conversão em sucesso. O verificador
marcou refinamentos em coincidência, custo total do sampler e identidade de
`M`, e falha localizada no agregado histórico do analista. Não houve oráculos,
ataques, `mutation_score`, selo ou certificado, pois pertencem ao P1268.

## Limite da atestação

Há atestação verificável de **não-herança conversacional**: os três papéis
independentes foram criados com `fork_turns=none`, e suas escritas ficaram em
escopos disjuntos. Não há atestação completa de isolamento de capacidades: os
agentes compartilham o filesystem e as read-allowlists foram processuais, não
impostas por sandbox. O veredito proporcional é:

**SEGREGADO COM NÃO-HERANÇA CONVERSACIONAL ATESTADA; ISOLAMENTO DE CAPACIDADES
DO FILESYSTEM NÃO ATESTADO.**

Logo esta repetição é mais forte que R1, mas não satisfaz uma alegação de
isolamento total. Antes do P1268, um sucessor contratual deve resolver V05,
V07, V08 e V09 sem editar os artefatos congelados desta execução.

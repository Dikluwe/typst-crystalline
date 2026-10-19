# Identidade por ocorrência em clusters bidi

Os três casos pendentes de LRM/PDI agora preservam os controles na cópia do
PDF. O mapa global dos glifos continua contendo apenas a identidade visual,
e outras ocorrências da mesma letra permanecem sem controles.

## Medição e proveniência

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC `2026-10-06T11:32:53.823943+00:00`.
[before.json](bidi-identidade-ocorrencia-2026-10-06/before.json) conserva fontes
L0/código anteriores, stat completo, status, binários e a sonda rustybuzz
executável/source/SHA. [execution.json.gz](bidi-identidade-ocorrencia-2026-10-06/execution.json.gz)
conserva código/L0 final, logs, estado completo e SHA dos binários.
Fontes DejaVu e Noto versionada estão em font-provenance.json, reproduzindo
os mesmos arquivos do lote anterior. Owners 1:1 e resselo em seals.json.

Oracle vanilla: pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA binário
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt anterior SHA `ecd69b3c3c045d0f7860e2f98f0e71b526204e5bc7844363369a09bfc6592fa9`;
final SHA `94c18ba207426bc5e81259b381b7842717009189f179768e00183e0a76de8f01`.
CLI com SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1, PDF sem tags e pretty=false.
Cada fixture, comando, texto bruto e XML/bbox está em
[verification.json.gz](bidi-identidade-ocorrencia-2026-10-06/verification.json.gz).
Compacto tem lote independente, comandos e estado em compact-verification.json.gz.

## Causa antes da decisão

Com REMOVE_DEFAULT_IGNORABLES, LRM+ABC e PDI+ABC produzem A no cluster byte 0,
B no byte 4 e C no byte 5. O range 0..4 inclui o controle e A. Esse range é
uma ocorrência mista válida, diferente de um controle isolado removido sem
glifo. O vanilla pinado fornece ranges completos à exportação em inline/
shaping.rs:1052-1099. O Tekt já conservava o texto integral no TextShaped,
mas fonts.rs removia controles do mapa global e text_identity só protegia SHY.

Conservar o controle no mapa global por GID recolocaria a identidade em todos
os usos daquela letra. O problema é falta de identidade por ocorrência, não
seleção de fonte ou largura. Hipótese refutada se a correção duplicasse o
controle, contaminasse letras repetidas ou alterasse a tinta.

## Correção materializada

O helper privado protect_authored_identity mantém o comportamento SHY e
certifica bidi pelos ranges UTF-8 entre clusters únicos emitidos e text.len().
Um range precisa conter controle bidi e outro codepoint. Sem glifo, range
inválido ou controle fora dos ranges, não há certificado. Não decidir por
GID, avanço, posição, aparência ou mero texto contendo um controle.

Com certificado, TextReplacement recebe exatamente o text daquele TextShaped,
e o item visual inteiro permanece como único filho. Fonte efetiva, bidi,
glifos/avanços, posição e cluster offsets são preservados. O walker delega
somente fora de envelope existente; não faz nova medição, shape ou layout.
ToUnicode global não mudou. O conjunto bidi continua o pin já existente;
ZWJ/ZWNJ/VS não ganham nova política. O certificado cobre a identidade integral
do sub-run, sem cortar ligaduras para construir envelopes por caractere.

[Streams de prova](bidi-identidade-ocorrencia-2026-10-06/pdf-identity-proof.json)
mostram uma única ocorrência ActualText `200E004100420043` ou
`2069004100420043` no primeiro ABC. CMap não contém 200E/2069 e os ABC seguintes
usam a identidade comum. identity-counts.json compara contagens brutas com o
oracle em todas as sondas novas, incluindo repetições, meio/fim, ligaduras,
estilos, Block e tabela.

Correção interna ADR-0127: contrato TextReplacement já aprovado; nenhum campo,
assinatura pública, fase, modo, dependência ou unsafe novo. Alterados três
consumers com owners exclusivos, não o exportador nem as entidades L1.

## Resultado e validação

| Lote | Antes | Depois |
|---|---:|---:|
| 112 controles herdados | 108 | 111 |
| 17 novos controles de ocorrência | 1 | 17 |
| Total verboso | 109/129 | 128/129 |
| Compacto direcionado | 1/20 | 20/20 |

Nenhum caso antes positivo regrediu. A comparação conserva LRM/PDI; remove
somente espaços e marcadores U+202A–U+202E para neutralizar formatação do
extrator, mantendo também o texto bruto como evidência. Caixas agrupadas têm
tolerância 0,05pt; maior delta aceito 0,016206pt. Paridade segue identidade
Unicode e morfologia observável, não GIDs, estrutura Rust ou bytes completos.

Dois testes novos falharam antes da implementação: certificado causal e
integração com fonte obrigatória. Depois, 1.177 testes L3 passaram. Integração
prova miss/hit, glifos/fonte/posição conservados, letra comum sem controle e
envelope existente sem aninhamento. Build debug/release e diff-check passaram.
L1 não mudou e sua suíte não foi repetida. SOURCE_DATE_EPOCH=0 preserva a
condição estável dos dois testes CMYK temporalmente frágeis, dívida já registrada.

Lint: zero erros, 531 warnings e 1.578 notes anteriores. Sem novos diagnósticos
por regra/mensagem/arquivo, ignorando deslocamento de linha; V15/V26 íntegros
e V5 ausente nos três consumers. Não afirmar zero violations global.

Quatro renders a 600dpi, threshold 180: caixas de tinta iguais ao oracle e
zero pixels escuros sem correspondente dentro de 1px nas duas direções.
Inspeção visual e dados em raster.json; não certificar pixels/bytes exatos.

## Limites e próxima frente

Os casos medidos de identidade bidi agora passam. Isso não certifica todos os
controles, fontes, clusters complexos, extratores, PDFs tagueados ou todos os
contextos RTL. Fallback sem fonte e política de invisíveis além do conjunto
continuam fora do certificado. Não houve medição de tempo nem ganho de CLI
alegado nesta rodada.

O único controle divergente do lote ampliado é `literal-terminal-space`.
A próxima frente deve separar espaço autoral terminal de espaçamento fraco ou
avançamento derivado e conservar sua identidade sem ampliar a geometria.
Coverage::from_ranges continua proposta independente, sem materialização.

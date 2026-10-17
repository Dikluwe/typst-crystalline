# Controles bidi autorais: placeholders, fallback e identidade PDF

Correção interna concluída para os dois controles herdados da frente Block/Align.
O lote anterior passou de 85/88 para 87/88 controles positivos; somente
`literal-terminal-space` permanece divergente nesse lote. Nos 24 controles
novos passaram 21, totalizando 108/112. Nenhum caso antes positivo regrediu.
Isso não fecha toda a identidade dos controles bidi na cópia do PDF.

## Proveniência e critério

Medição em working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`.
UTC `2026-10-06T11:09:19.896670+00:00`. [before.json](controles-bidi-pdf-2026-10-06/before.json)
conserva código/L0 anteriores, stat completo, status e SHA do binário copiado
antes da alteração. [execution.json.gz](controles-bidi-pdf-2026-10-06/execution.json.gz)
conserva estado final dos cinco consumers e owners, logs, stat/status e binários.
Fontes DejaVu Sans e Noto versionada têm SHA em font-provenance.json; os demais
controles repetem fontes/fixtures do lote Block/Align, cuja proveniência não muda.

Oracle vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, SHA binário
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt anterior `1a4c5c101caceb4f87dd7b706961a0f43054702817de125ff3eb22daa6131df2`;
final `ecd69b3c3c045d0f7860e2f98f0e71b526204e5bc7844363369a09bfc6592fa9`.
Fonte literal, comandos, extração bruta e XML/bbox de cada engine constam em
[verification.json.gz](controles-bidi-pdf-2026-10-06/verification.json.gz).
CLI com SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1, PDF sem tags, pretty=false.

Aceitação combina texto sem espaços/marcadores U+202A–U+202E inseridos pelo
extrator e caixas agrupadas com tolerância 0,05pt. O texto bruto também fica
preservado; não afirmar igualdade da string ou que todo marcador extraído é
autoral. LRM e PDI não são descartados na comparação: suas divergências ficam
visíveis. Máxima diferença das caixas aceitas: 0.016206pt. Pares completos
estão em summary.json; identidade Unicode e morfologia são observáveis,
GIDs e bytes de PDF não constituem o critério de paridade.

## Causa e correção

1. O rustybuzz conservava placeholders invisíveis com GID compartilhado por
   espaço. O primeiro mapeamento global do GID contaminava todos os espaços:
   CID 0001→202C no embedding e CID 0001→200F no RLM. Os CMaps antes/depois
   arquivados mostram agora CID 0001→0020 nos dois casos.
2. O pipeline confundia shaping válido sem glifos com falha e retornava Text
   ao exportador. Corrigidos o término do shaping e a saída antecipada sem
   runs: textos exclusivamente bidi com fonte legível são resultado vazio
   válido. Fontes/contextos não resolvidos conservam fallback anterior.
3. A seleção de fontes reconhecia só U+2060/U+202B/U+202C como formato.
   Isolados podiam pedir cobertura/fallback, desenhar notdef sobre A ou
   escolher outra face. O conjunto bidi explícito agora dispensa cobertura,
   conservando caracteres autorais na entrada do shaping.

Configuração de REMOVE_DEFAULT_IGNORABLES antes de shape para remover
placeholders depois do shaping, nos sub-runs com SHY ou controles bidi explícitos; entrada e resolução bidi permanecem completas.
ToUnicode visual exclui SHY e esses controles do mapeamento compartilhado
por GID. SHY mantém seu envelope ActualText existente. Não criar envelope
bidi genérico nesta rodada: a identidade por ocorrência precisa de certificado
causal próprio para distinguir controle isolado de cluster visível.

O conjunto inclui U+061C, U+200E/U+200F, U+202A–U+202E e U+2066–U+2069.
ZWJ, ZWNJ, variation selectors e outros invisíveis não ganham política geral.
A flag pode remover outros Default_Ignorable coexistentes no mesmo sub-run,
conforme a API rustybuzz; não se certificam todos os casos mistos por esta medição.
Invariantes compartilhadas estão no núcleo bidi-visual-identity com pin efetivo
SHA-256 nos três owners de shaping/mapa e no owner de seleção de fontes.
Não há campo/API pública, fase, flag, dependência ou unsafe novo.

## Verificação e refutações

- RED: os dois testes novos de identidade/flags falharam; o teste de resultado
  vazio falhou; completar o teste de classificação de formato também falhou.
- GREEN: 1.175 testes L3 passaram com SOURCE_DATE_EPOCH=0. As duas falhas CMYK
  temporais conhecidas da frente anterior não reapareceram nessa condição.
  L1 não foi alterado; sua suíte não foi repetida nesta rodada.
- `cargo build`, release e `git diff --check` passaram.
- Lint: zero erros, 531 warnings e 1.578 notes preexistentes, sem novos
  diagnósticos por regra/mensagem/arquivo (deslocamento de linha ignorado).
  V15/V26 íntegros e nenhum V5 nos cinco consumers. Resselo localizado.
- Quatro controles visuais a 600dpi, threshold 180: caixas de tinta iguais e
  zero pixels escuros sem correspondente dentro de 1px em ambas as direções.
  Comparação e inspeção visual em raster.json; não afirmar bytes/pixels exatos.

first-verification.json.gz refutou fechar apenas a colisão: RLM voltava via
fallback. second-verification.json.gz e first-raster.json refutaram fechar
somente o retorno final: LRI voltava pela saída antecipada e desenhava notdef;
PDI consultava cobertura e trocava face. As provas intermediárias foram
preservadas. Erros de compilação durante a edição constam nos logs; não são
contados como testes RED de comportamento.

## Limites e próxima unidade

Restam `control-ltr-lrm`, `control-rtl-lrm` e `control-ltr-pdi`: o vanilla
extrai respectivamente U+200E ou U+2069 junto a A; o Tekt extrai A. A troca
indevida de fonte do PDI foi corrigida, mas a cópia continua divergente.
Não inferir que o comportamento do oracle é defeito nem certificar apagamento
universal de controles autorais. A próxima unidade precisa preservar a identidade
**por ocorrência** dos clusters que unem controle e letra, sem voltar a
contaminar todos os usos do GID daquela letra ou do espaço.

Espaço literal terminal permanece aberto; seleção/render sem fonte, texto
exclusivamente invisível fora do conjunto, todos os extratores e todos os
contextos bidi não foram certificados. Coverage::from_ranges permanece
proposta separada, sem materialização. Tempos de CLI não foram medidos nesta
rodada: nenhum ganho de desempenho é alegado.

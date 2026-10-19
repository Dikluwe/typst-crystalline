# Paint de contorno e dicionários uniformes

Corrigidos o descarte de dicionário uniforme em rect, a rejeição de Gradient e
Tiling no paint nomeado de stroke, a rejeição de paints diretos nos shapes e
o contorno preto indevido com dicionário vazio. Paint auto permanece não
especificado; Color, Gradient e Tiling conservam o tipo e os dados.
Isso não encerra contornos por lado nem a implementação de tiling no PDF.

## Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before/results.json.gz
conserva estado, diffstat completo, fontes/owners, UTC, comandos e binários.
Coleta inicial de 2026-10-14T08:53:04.290592+00:00 a 2026-10-14T08:53:06.261610+00:00, Tekt SHA-256
d01f3f505dce39ef5a9b734380365999695035e0bbf5a3529646ca3f2415eb4e.
O facade layout foi acrescentado ao escopo com snapshot próprio em
facade-before.json.gz antes da alteração de seu L0/código.

Referência vanilla: pin 9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489, SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
A fonte pinada stroke.rs:236-243 aceita Paint smart e sides.rs:238-258 distingue
dicionário vazio, por lado e uniforme. Em Tekt, layout/stroke.rs:53-63 aceitava
apenas Color no named; shapes/paint.rs:45-64 delegava novos paints ao caster
legado restrito; rect.rs:40-43 descartava todos os Dict. São causas distintas,
medidas antes da decisão, não uma autorização para copiar a estrutura upstream.

## L0, implementação e RED→GREEN

Owners atualizados primeiro. O núcleo stroke-paint-inputs compartilha a
preservação de tipo entre os owners de cast e adaptação de shapes; ownership
produtivo continua 1:1. O helper privado de Paint no owner layout/stroke é usado
por named e pelo campo paint do dicionário posicional. None/integer recebem a
mensagem medida no pin. Demais campos named e a extensão overhang mantêm sua
validação anterior; não se reivindica paridade completa de stroke().

Shapes encaminham Gradient/Tiling/Dict uniforme ao cast posicional existente,
sem reduzir para primeira cor. O facade layout reexporta esse cast apenas
pub(super), sem nova API externa. Rect distingue chaves por lado de dicionário
uniforme; rect e square tratam (:), assim como none, como ausência explícita
para o fallback. Dimensões, size e defaults de square não foram alterados.

Os oito testes novos falharam funcionalmente e passaram após o reparo. Eles
verificam Paint completo, thickness/cap, specified, overhang, ausência e erros.
red-fixture-import.log é erro inicial do fixture (repr_value não importado),
não RED funcional. implementation-private-module.log registra a tentativa de
acessar módulo privado; foi corrigida por reexportação interna após L0 próprio.
red.log, green.log, red-state e candidate-final preservam as etapas sem
apresentar erros de compilação como provas de defeito comportamental.

## Recoleta e controles de render

Recoleta de 2026-10-14T09:01:33.810501+00:00 a 2026-10-14T09:01:36.266690+00:00, binário SHA-256
882a0c2fdbdf61775425948ad5cd707e2305a5198c3a12a31bf04600e38cc308. Fontes e binários permaneceram estáveis.
Os controles registram os mesmos comandos/expressões de antes, inclusive
falhas que continuam abertas. Contagens abaixo são pixels pintados do Tekt
em canvas inteiro, Poppler a 144 dpi; não equivalem a paridade integral.

| Caso | Antes | Depois |
|---|---:|---:|
| rect-dict | 768 | 9216 |
| rect-pair | 9216 | 9216 |
| rect-named | erro | 9216 |
| rect-empty | 768 | 0 |
| rect-none | 0 | 0 |
| circle-direct | erro | 530 |
| circle-named | erro | 530 |

Os pixels Tekt de rect-dict, rect-named e rect-pair são idênticos. Rect-empty
é idêntico a rect-none e não pinta; circle-direct e circle-named também são
idênticos. O recibo render-comparison conserva hashes, diferenças bilaterais
integrais e checks qpdf, sem recortes para aprovar o resultado. Os nove snapshots
permaneceram byte a byte iguais; não houve regeneração neste lote.

A recoleta de eval confirma aceitação de named Gradient/Tiling/Auto e circle
Gradient; None/integer continuam rejeitados, agora com mensagem medida. Dict
uniforme inválido deixou de ser silenciosamente aceito. A mensagem para chave
uniforme desconhecida ainda difere do vanilla. Repr de Shape e Gradient permanece
incompleto; não se apresenta coincidência de status como igualdade de linguagem.

extra acrescenta controles com largura explícita de square: dict vazio não
pinta e dict uniforme chega ao render. O caso original square(size:...) e o
constructor sem largura continuam falhando no Tekt por diferenças de dimensão;
não são falhas do cast corrigido e seguem abertos.

## Limitações confirmadas

- Dict por lado e mistura de chaves por-lado/uniformes continuam incorretos:
  a entidade uniforme e o scope-out anterior não foram substituídos. A correção
  atual não converte seus valores em um stroke uniforme nem certifica esse ramo.
- Tiling é preservado pelo modelo, mas o exportador PDF usa paint.to_color no
  fallback em stream/shape.rs:201-206. O controle named/posicional é idêntico
  no Tekt, mas ambos desenham uma faixa preta; o vanilla desenha o motivo.
  extra e open-export-evidence conservam essa falha, fonte e owner. Aceitar o
  valor não é implementar o padrão PDF.
- O corpus ainda apresenta diferenças de render nos contornos e gradientes,
  além das lacunas de morfologia e diagnósticos acima. Não foram escondidas por
  tolerância nova, snapshots atualizados ou igualação mecânica ao upstream.

## Validação final

Suíte completa de 2026-10-14T09:00:41.786594+00:00 a 2026-10-14T09:06:44.064396+00:00: 8476 passaram,
0 falharam, 7 ignorados. Fontes estáveis durante execução;
ambiente /dev/shm, vanilla pinado e comandos conservados em workspace-pre/result.
Build, check all-targets, rustdoc -D warnings, fmt, diffcheck, gates estritos
V1/V5/V15/V26 e reverso passaram. Clippy foi inventário cap-lints warn:
181 antes, 181 depois. Arquitetura:
0 erros, 382 warnings,
1626 notas. Não certifica Clippy -D warnings nem
zero violations. scope-check confirma alterações Rust restritas aos seis
consumers deste lote e os snapshots intactos.

O objetivo global permanece aberto: contornos por lado, tiling PDF, CMYK,
Raw/presença e seu gate público, demais frentes funcionais e CI/release remoto.
Nenhum commit, push ou publicação. evidence.json.gz conserva receitas,
fontes/owners/núcleo, logs e artefatos com base64 e SHA-256 verificados após releitura.

# Posicionamento físico de gradientes cônicos no PDF

O Pattern cônico agora recebe escala, posição e transformação da ocorrência
Shape em todos os espaços de cor. Grupos afins aninhados, referência parent,
bleed e diferentes alturas de página participam da matriz. A correção não
encerra a geometria nem a amostragem cônica: cantos em branco, centro ignorado
e offsets CMYK ignorados continuam reproduzidos e abertos.

## Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
placement-before.json.gz registra UTC, status, git diff HEAD --stat e fontes/
owners antes de editar L0. O corpus inicial foi executado de 2026-10-14T07:40:45.053635+00:00 a
2026-10-14T07:40:48.380385+00:00, binário Tekt SHA-256 142f4fb5a6b44d844a7227a51d36145fc051ce2ab45f3019e3af26f9fb5842bd.
São controles Tekt contra Tekt, sem usar a aparência do vanilla como requisito.

`gradients/mod.rs:224-264` no snapshot anterior perdia a caixa própria dos
processos e deduplicava por Arc/bbox. `builder/resources.rs:564-574` só
posicionava Conic spot; RGB/CMYK usavam identidade. `stream/draw.rs` aplicava
transformações posteriores ao Pattern; `stream/page.rs:45-49` acrescentava
bleed. Pattern está no espaço inicial do stream, como já documentado no
relatório conic-cobertura-2026-10-13.md. A inspeção dos três builders confirma
que descoberta, recursos e pintura usam o mesmo PagedDocument emprestado.
FrameItem possui filhos Vec próprios: nenhum clone ou mutação entre passes.

## L0 e correção

Os owners foram atualizados antes dos cinco testes novos e da implementação.
Todos os cinco testes falharam no código anterior e passaram com a correção.
Eles verificam matriz física, mesmo Arc em várias ocorrências/páginas,
parent explícito/local/página, grupos afins aninhados, bleed, referências
resolvidas e fill/stroke, incluindo Compact/Verbose. RED funcional em red.log;
placement-red.json.gz conserva a fonte de teste e os L0 nessa etapa.

ConicPlacement privado compõe TransformMatrix existente. O coletor guarda a
matriz e associa recursos por identidade de Shape e Arc durante esta exportação;
a mesma ocorrência pode compartilhar fill/stroke. O endereço não é serializado,
não define igualdade da linguagem e não é reutilizado entre documentos. Isso
conserva cada colocação sem depender de arredondamento da bbox. Não há promessa
de deduplicação máxima entre ocorrências geometricamente iguais.

O builder emite a matriz preparada; draw passa o Shape e PageContext existentes
aos helpers privados de paint. Linear/radial conservam sua política anterior.
Sem mudança de API externa, entidade pública, modo, fase ou pin do linter.
As invariantes comuns pertencem ao núcleo pdf-conic-placement, pinado pelos
quatro owners responsáveis. Um primeiro resselo foi bloqueado pela ordenação
das referências de núcleo; a ordenação foi corrigida antes do resselo final.
Imports privados sem uso nos dois facades foram removidos, com L0 e snapshots
próprios; não se introduziu supressão. Dois braços Shape inalcançáveis saíram
dos matches que já tratavam Shape no primeiro braço.

## Recoleta visual e limites

Corpus após posicionamento: 2026-10-14T07:51:24.012907+00:00 a 2026-10-14T07:51:28.068697+00:00;
binário SHA-256 491cb7186819701299e76cc4d1aed7c48afee89f1b22ac6cd4979c577e740c3c. Fontes e binário estáveis durante a
coleta. O binário antecede apenas a retirada do import privado final e a
manutenção de fixture; os cinco fontes produtivos da sonda permanecem idênticos
aos finais. Não se apresenta esse executável como build dos bytes finais do facade.

Raster pdftoppm a 144 dpi, canvas inteiro sem alinhamento ou recorte. O mesmo
retângulo recebe red sólido ou conic(red,red); critérios originais preservados.

| Caso | Pixels pintados no sólido | Brancos do conic antes | Brancos depois |
|---|---:|---:|---:|
| page | 24576 | 24571 | 5004 |
| offset | 7072 | 7072 | 1378 |
| center | 24576 | 24571 | 5004 |
| rotate | 4104 | 4104 | 857 |
| parent | 4000 | 4000 | 703 |

A área preenchida voltou à posição/dimensão da forma. A imagem de página mostra
uma elipse inscrita, conservando cantos brancos: não é cobertura correta de um
retângulo. As contagens integrais de diferenças estão nos dois probe.json.gz;
a tabela não substitui a comparação completa nem certifica a geometria do
controle sólido em todo documento possível.

As sondas refuters de 2026-10-14T07:52:46.428127+00:00 a 2026-10-14T07:52:47.897900+00:00 confirmam:
- alterar center de (50%,50%) para (30%,70%) mantém os pixels e a malha RGB;
- deslocar a parada CMYK interna de 25% para 75% mantém pixels e malha;
- com duas cores CMYK, sample(50%) é [50%,50%,0%,0%], mas o limite angular
  correspondente da malha contém magenta puro; a volta final retorna à primeira
  cor, em desacordo com sample(100%).

offset-language-controls.json confirma, com o binário final identificado por
SHA-256 e estado registrado, que sample(50%) responde à alteração de offsets;
a perda está no exportador.

qpdf aceita os arquivos desses controles: validade sintática não comprova
correção visual. A etapa seguinte conic-geometria-amostragem deve corrigir
centro, cobertura e amostragem incluindo offsets/descontinuidades, sem copiar
o fallback uniforme do vanilla ou exigir igualdade de tesselação. Stroke e
formas além da caixa exigem controles de cobertura próprios. A paridade geral
e a precisão de todos os contextos parent produzidos em L1 não estão encerradas.

## Snapshot controlado

A primeira suíte encontrou apenas o snapshot p307b_06_gradient_conic falhando
no corpus e depois terminou por SIGTERM (session 19817, exit 143); não gerou
resultado final. A verificação de processos confirmou cargo/rustc/testes já
terminados. Causa desconhecida, registro em first-validation/termination.json.
Não contar essa execução como suíte completa aprovada.

O L0 do corpus autorizou a manutenção específica após a prova causal. A referência
06 passou de 5327 para 5350 bytes. Duas gerações pelo
harness Compact/Disabled/epoch fixo foram idênticas. qpdf aprovou ambas; os outros
oito PDFs e todas as fontes ficaram intactos. A comparação JSON independente
aponta somente o objeto Pattern 110: Matrix [1,0,0,1,0,0] passa a
[100,0,0,100,70.866,671.024]. Malha, stream de pintura e todos os demais objetos
permanecem iguais. A referência anterior rasterizava em branco; a nova mostra
o círculo. Os hashes, raster e comandos estão em snapshot-comparison.json e
snapshot-update-results.json. MANIFEST atualizado, asserts do harness preservados.

## Validação final

Suíte completa de 2026-10-14T07:56:13.089234+00:00 a 2026-10-14T08:01:53.066716+00:00:
8460 testes passados, 0 falhas, 7 ignorados.
workspace-pre/result conservam comandos, hashes, estado e ambiente /dev/shm;
fontes inalteradas durante a suíte. Os nove snapshots passaram sem UPDATE.
O vanilla exigido pelos testes foi o pin 9f2b6e8715237cb086899a42873660fe744622e8,
árvore 6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489, binário SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

Build, check all-targets, rustdoc -D warnings, fmt, diffcheck, linhagem estrita
V1/V5/V15/V26 e reversa passaram. Clippy é inventário cap-lints warn:
184 antes e 182 depois, delta {'unreachable_patterns': -2}.
Arquitetura: 0 erros,
382 warnings e 1631
notas. Isso não aprova Clippy -D warnings nem o objetivo zero violations.

A proposta pública Raw segue inativa; CI/release remoto e demais frentes
funcionais continuam pendentes. Nenhum commit, push ou publicação. O objetivo
integral permanece aberto. evidence.json.gz conserva fontes/owners, núcleo,
receitas, logs, resultados, PDFs, PNGs e referências anteriores com base64 e
SHA-256 verificados após releitura.

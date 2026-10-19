# Relatório — Passo 976 (determinismo do vanilla entre execuções)

**Data:** 2026-08-05 · Passo de medição — **nada implementado**.
**Proveniência**: HEAD = `2b97feed4` (P975). Vanilla: `/usr/local/bin/
typst` 0.15.1 (9dfd3a08). Documento: `.typ/typst-math-comprehensive-test.typ`
(30 secções). Ficheiros em `temp/p976/`.

## Fase A — medições

**3 execuções seguidas, sha256** (`v1..v3` vanilla, `c1..c3` cristalino):

- Vanilla: v1 ≠ v2, **v2 = v3** (v2/v3 caíram no mesmo segundo).
- Cristalino: c1 ≠ c2 ≠ c3 (cada execução noutro segundo).

**Localização exacta das diferenças** (`cmp -l` + inspecção das regiões):

- Vanilla: 92 bytes divergentes, todos no bloco XMP —
  `xmp:CreateDate`/`xmp:ModifyDate` (timestamp ao segundo) e
  `xmpMM:DocumentID`/`InstanceID` (derivados do timestamp — prova: duas
  execuções no mesmo segundo produzem IDs iguais e PDFs byte-idênticos).
- Cristalino: 48 bytes, mesma classe — timestamp XMP + IDs (sem
  `CRYSTALLINE_PDF_FIXED_EPOCH` definida).

**Prefixos de subset de fonte** (a hipótese de P950):

| execução | prefixos |
|---|---|
| vanilla v1 | `FHILWO+NewCMMath-Book`, `HGWOSO+NewCM10-Bold`, `IHRKXL+LibertinusSerif-Regular`, `LTLHQI+NewCM10-Regular` |
| vanilla v2 | **idênticos a v1** |
| cristalino c1/c2 | `AAAAAA+…` (determinístico, decisão P950) |

**A premissa de P950 fica refutada para o vanilla 0.15.1**: os prefixos
são estáveis entre processos para o mesmo documento (derivados de
conteúdo/ordem, não aleatórios por execução). A única não-determinismo do
vanilla é o timestamp (e os IDs dele derivados).

**Epoch fixada** (duas execuções cada):

- `SOURCE_DATE_EPOCH=1700000000` vanilla: **byte-idênticos** ✓ (o vanilla
  honra SOURCE_DATE_EPOCH).
- `CRYSTALLINE_PDF_FIXED_EPOCH=1700000000` cristalino: **byte-idênticos** ✓
  (o mecanismo de P615/P617 funciona como documentado).

## Conclusão da medição

Os DOIS compiladores são determinísticos quando a epoch é fixada; sem
epoch fixada, nenhum é (o vanilla por timestamp+IDs, o cristalino idem).
"Byte-idêntico ao vanilla" **não é um alvo instável** — a objecção
registada no passo (prefixo aleatório) não se confirma nesta versão do
vanilla.

O que separa hoje os dois PDFs com epoch fixada (tamanhos 150 694 vs
154 524 bytes, bytes completamente diferentes): ordem/numeração de
objectos PDF, serialização de CMaps/widths/ToUnicode, compressão dos
streams, prefixos de subset (`AAAAAA+` vs hash do vanilla), metadados
XMP/document info, e o próprio conteúdo dos streams (operadores — a
frente P956 já espelha o envelope verbose). Byte-paridade plena exigiria
portar o escritor do vanilla (krilla) quase verbatim — objecto a objecto,
não só fórmula a fórmula.

## Fase B — opções para o dono decidir (informadas pela medição)

1. **Byte-idêntico com epoch fixada, portando o escritor do vanilla**
   (krilla): tecnicamente estável (a medição prova), mas é um projecto de
   meses — replicação de ordem de objectos, serializações e compressão,
   não de geometria. Subproduto útil: obrigaria a paridade total de
   streams (já quase coberta por P956).
2. **Byte-idêntico módulo fontes de não-determinismo conhecidas**:
   alinhar o que é semanticamente visível (mesmos objectos de fonte com
   os mesmos prefixos — adoptar a derivação do vanilla; mesma ordem de
   objectos; mesma compressão) sem perseguir a serialização exacta do
   krilla. Sub-opção: adoptar os prefixos por hash de conteúdo do vanilla
   em vez de `AAAAAA+` (reverte a decisão de P950 — que foi tomada sob a
   premissa, hoje refutada, de que o vanilla era aleatório).
3. **Paridade de sequência de operadores e valores, não de bytes**
   (a formulação anterior à correcção do passo): o que a frente já
   persegue — continua.

Recomendação técnica (não é decisão minha): a opção 2 tem o melhor
retorno — os bytes diferentes restantes são quase todos serialização sem
efeito visual; a opção 1 gasta esforço desproporcionado em byte-exactidão
de container. Mas se o objectivo do dono é *provar* paridade total via
hash, só a opção 1 o dá.

**Fico a aguardar a decisão.** Sem implementação neste passo, por desenho
do próprio passo.

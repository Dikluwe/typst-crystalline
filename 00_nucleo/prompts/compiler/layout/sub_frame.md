# Prompt L0 — sublayout isolado
Hash do Código: 5c9ec7b0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/sub_frame.rs`

## Contrato e aceitação

Executar conteúdo numa SubLayoutRegion salvando/restaurando cursor, região,
linha, collectors e pending state em LIFO. Devolver items, altura, decorações e
fixups no mesmo referencial local. Variante inline isola somente items do body
e deixa o caller decidir emissão.

## P1391 — isolamento do arm de linha overlong

### Medição anterior à decisão

O arm one-shot P1391 pertence à linha exterior certificada. Herdá-lo por um
sub-frame permitiria que um flush interno o consumisse ou que o estado interno
vazasse de volta ao caller.

### Decisão e aceitação

Ao entrar no sub-layout, salvar e limpar o arm; ao sair, restaurar exatamente
o valor exterior. O corpo local nunca consome o fechamento pendente do pai.
Esta regra é estado LIFO privado: não reconhece Content, não altera geometria
do sub-frame e não cria API, fase, alocação proporcional ou recursão.

## P1392 — preservação do mesmo isolamento

O certificado P1392 reutiliza o arm one-shot sem alterar sua vida útil. O
sub-frame continua salvando, limpando e restaurando o estado exterior em LIFO;
nenhum flush interno pode consumir um arm P1391 ou P1392 do pai. Não há código
ou geometria adicional neste owner além da linhagem atualizada.

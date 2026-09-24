# Prompt L0 — `boxed`
Hash do Código: c6dc5c19

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/boxed.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## P1408 — largura fracionária transportada

Quando `BoxedElem.width_fraction` existe, o layout registra no mecanismo
`pending_fr` uma fronteira imediatamente depois dos itens do body, para que os
itens seguintes consumam o restante proporcional da linha. O caminho de
largura absoluta e todos os cosméticos permanecem intactos. A origem interna
de P1404 e a origem pública proposta em P1408 têm a mesma semântica de layout.
Junto à fronteira, registra no Layouter a largura exterior natural já ocupada
pelo body e pelos lados da caixa, para que Cursor substitua essa largura pela
parcela em vez de somá-la à parcela. Não analisar nem repetir conteúdo
ilimitadamente.

## Medição e contrato

Compor Boxed inline, isolando extensão do body, largura/altura explícitas, overflow, clip, radius e coordenadas locais do Group.

### Proveniência das calibrações P1119/P1120

Os fatores `28.81670pt` e `7.633997pt` na base de 11pt não são defaults
normativos do vanilla nem métricas copiadas de sua implementação: são
calibrações empíricas do documento canônico registradas em
`diagnosticos/typst-passo-1119-relatorio.md` e reavaliadas em
`diagnosticos/typst-passo-1120-relatorio.md`. A divisão por `11.0` apenas
normaliza a medição para `em`, preservando escala com o tamanho da fonte.
Substituí-las exige nova medição da morfologia da linha; não inferir métrica
tipográfica inexistente apenas para remover um literal.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.


## P1491 — baseline explícita participa da extensão inline

### Medição anterior à decisão

O consumer anterior lia `baseline` e o descartava (`let _ = baseline`). No
vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, uma caixa de 20pt
com `baseline: 3pt` tem baseline a 17pt do topo; combinada com texto, a linha
mantém altura 20pt e baseline 17pt. O valor é shift relativo à baseline natural
do frame, não altura literal nem posição absoluta.

### Decisão proprietária

Resolver o shift contra o font-size e aplicá-lo à baseline natural do frame da
caixa. Para body com baseline textual, usar a extensão já medida do body; para
frame sem baseline natural, a baseline natural é sua borda inferior, como no
vanilla. Registrar no Layouter o par causal `(ascent, descent)` após inset e
altura final; valores fora da caixa continuam permitidos e as extensões da
linha usam máximos não negativos.

O layout do body é fisicamente isolado da `current_line` do pai: retirar a
linha antes da descida e restaurá-la antes de anexar a cauda produzida. Assim um
body de bloco que execute `flush_line` não descarrega nem invalida o prefixo
inline anterior à caixa. Quando o body materializa apenas itens de frame, esses
itens são rebaseados para a origem inline do body e a largura natural já medida
recompõe o avanço da caixa.

O campo, constructor, assinatura pública, render e fase não mudam. Não criar
variant, carrier público ou caso especial de `Content::Boxed` em
`measure_content_real`.

# Prompt L0 — campanha integrada SVG P1250
Hash do Código: 479c68aa

**Camada:** L3 — apoio de teste, sem consumer produtivo  
**Ficheiro alvo:** `03_infra/tests/p1250_svg_integrated.rs`  
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Medição antes da decisão

P1245–P1249 possuem owners e certificados próprios. A integração entre tiling,
clip, destinos, imagens e glifos precisa ser exercitada sem adicionar lógica ao
owner produtivo `export/svg.rs` e sem alterar os seus hashes certificados.

## Contrato do harness

O teste executa seis corpus integrados. Cada corpus combina ao menos três
famílias funcionais e observa o grafo de referências locais, geometria e
transformações, conteúdo visível, alpha e classificações `Unknown`.

Deve cobrir:

- tiling, clip transformado e destino interno same-page;
- paint/alpha, imagem orientada e cover clip;
- glifo direto, identidade completa da fonte, link e transform aninhado;
- SVG autossuficiente, tiling e stroke complexo;
- grafos page-local separados e cross-page sem rota como `Unknown`;
- coexistência sem captura entre IDs de glyph, paint, pattern, clip e destino.

O harness rejeita mutações negativas sobre saídas reais e repete o corpus para
demonstrar determinismo. Construções opacas ou sem carrier não são promovidas a
sucesso. Nomes concretos de IDs, ordem equivalente de `defs`, bytes/base64 e
ortografia XML são mecânica fora do gate quando o grafo e a morfologia
permanecem equivalentes.

Este L0 legitima somente o ficheiro de teste indicado. Não autoriza novo owner
produtivo, mudança de API ou alegação de equivalência geral SVG/Typst.

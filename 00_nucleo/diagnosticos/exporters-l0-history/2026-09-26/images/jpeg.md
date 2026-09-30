# Prompt L0 — infra/export/images/jpeg
Hash do Código: fc298d64

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/images/jpeg.rs`
**ADRs:** ADR-0029, ADR-0095, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: esta família está misturada em 03_infra/src/export/images.rs
nos intervalos medidos em infra/export/images.md. A atomização desloca os
corpos, não redefine o comportamento. Nenhuma conclusão de paridade nova
é inferida das medições históricas do L0 anterior.

## Decisão e contrato

Owner de jpeg_components, jpeg_color_space e jpeg_is_rgb. Varre o
cabeçalho a partir de SOI, respeita comprimento dos segmentos e interrompe
em SOS, comprimento inválido ou limite do buffer. SOF0/SOF2 fornece canais;
1 vira DeviceGray, 4 vira DeviceCMYK, demais ColorSpace usam DeviceRGB.
Sem componente legível, preserva fallback 3; jpeg_is_rgb conserva teste
exato de 3 componentes. Não decodifica JPEG nem altera seus bytes.
Construção do XObject pertence a infra/export/images/xobjects.md;
validação integral JPEG permanece no owner infra/export/images/scan.md.
Aceitação: cabeçalhos baseline/progressive, grayscale, CMYK, fallback e
truncados conservam a seleção e não ultrapassam o buffer.

## Linhagem e validação

Owner único deste consumer, módulo privado ao hub. Helpers reexportados
conservam alcance de visibilidade anterior. Testes são locais e não expõem
novas APIs. Executar testes de images, suíte E2E export::tests, cargo build
e crystalline-lint .; não alterar políticas durante a movimentação.

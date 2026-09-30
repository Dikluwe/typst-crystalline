# Prompt L0 — infra/export/images/xobjects
Hash do Código: 9e8fdb0b

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/images/xobjects.rs`
**ADRs:** ADR-0029, ADR-0095, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: esta família está misturada em 03_infra/src/export/images.rs
nos intervalos medidos em infra/export/images.md. A atomização desloca os
corpos, não redefine o comportamento. Nenhuma conclusão de paridade nova
é inferida das medições históricas do L0 anterior.

## Decisão e contrato

Owner de build_jpeg_xobject, build_png_smask_xobject e
build_png_rgb_xobject. JPEG conserva bytes crus em /DCTDecode, width/height
recebidos, BitsPerComponent 8 e ColorSpace fornecido ou ICCBased pelo ID.
SMask conserva DeviceGray/FlateDecode; objeto main usa ColorSpace e bytes
comprimidos do payload e referencia /SMask somente quando recebe ID.
Cada builder conserva /Length do payload armazenado, delimitadores stream e
endstream e ordem/formatação atual. Não decodifica, recomprime ou converte
bytes binários em UTF-8. ICC stream pertence a infra/export/images/icc.md.
Aceitação: payload binário é preservado e /Length, ICCBased, DeviceGray e
referência opcional SMask são coerentes com os argumentos existentes.

## Linhagem e validação

Owner único deste consumer, módulo privado ao hub. Helpers reexportados
conservam alcance de visibilidade anterior. Testes são locais e não expõem
novas APIs. Executar testes de images, suíte E2E export::tests, cargo build
e crystalline-lint .; não alterar políticas durante a movimentação.

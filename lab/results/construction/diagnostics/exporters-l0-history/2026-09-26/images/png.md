# Prompt L0 — infra/export/images/png
Hash do Código: af82eaf9

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/images/png.rs`
**ADRs:** ADR-0029, ADR-0095, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: esta família está misturada em 03_infra/src/export/images.rs
nos intervalos medidos em infra/export/images.md. A atomização desloca os
corpos, não redefine o comportamento. Nenhuma conclusão de paridade nova
é inferida das medições históricas do L0 anterior.

## Decisão e contrato

Owner de process_png_for_pdf, função pública com nome histórico:
decodifica PNG/GIF/WebP por image::load_from_memory, preservando também o
comportamento genérico atual do decoder. GIF conserva o primeiro frame.
Retorna o PdfImagePayload definido em infra/export/images.md, com largura,
altura, DeviceRGB e RGB8 comprimido via helper do hub. Imagem com alpha
converte RGBA8, separa RGB/A e só comprime alpha se algum valor não é 255;
imagem opaca omite SMask. Preserva detalhe cru do erro do decoder, sem
envelope adicional aqui. Não memoiza por ponteiro; esse cache pertence a
infra/export/images/scan.md. Não muda formato, API, alocação ou compressão.
Aceitação: RGB e alpha descomprimidos preservam pixel e transparência;
PNG totalmente opaco omite máscara e dados corrompidos retornam erro.

## Linhagem e validação

Owner único deste consumer, módulo privado ao hub. Helpers reexportados
conservam alcance de visibilidade anterior. Testes são locais e não expõem
novas APIs. Executar testes de images, suíte E2E export::tests, cargo build
e crystalline-lint .; não alterar políticas durante a movimentação.

# Prompt L0 — infra/export/images/scan
Hash do Código: 9a791a3c

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/images/scan.rs`
**ADRs:** ADR-0029, ADR-0095, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: esta família está misturada em 03_infra/src/export/images.rs
nos intervalos medidos em infra/export/images.md. A atomização desloca os
corpos, não redefine o comportamento. Nenhuma conclusão de paridade nova
é inferida das medições históricas do L0 anterior.

## Decisão e contrato

Owner de validate_document_images, validate_image_data,
scan_all_images, process_image_item, xobject_resources_for_page e PAYLOAD_CACHE.
Validação percorre pages/groups pelo FrameVisitor, descodifica JPEG integral e
outros rasters pelo owner infra/export/images/png.md, retornando primeiro erro
no envelope failed to decode image (...); Unknown conserva Ok(()) aqui.
Medição do código :280-320 refuta a frase histórica que atribui memoização à
validação: ela chama diretamente o decoder. A extração não altera essa rota.
Scan conserva dedup por Arc::as_ptr, ordem de encontro e nomes Im1, Im2, etc.
Arcs vivos do documento estabilizam identidade durante esse export. Mantém
thread-local de payload por ponteiro, escopo/lifetime e política existentes;
não amplia garantia de cache para depois do documento nem muda sua limpeza.
JPEG RGB usa ID ICC fornecido, grayscale/CMYK não; dimensões JPEG vêm do item.
Raster aloca SMask antes de main ID quando alpha está presente; decode inválido
omite imagem com aviso, sem registrar ponteiro. Unknown também é omitido.
Resource dict percorre Groups, emite uma referência por imagem usada na página
e retorna string vazia quando não há imagens registradas. Não muda IDs,
estrutura de tipos nem comportamento do caller que pula validação.
Aceitação: clones do mesmo Arc em grupos têm um objeto/recurso; bytes iguais
em Arcs diferentes continuam distintos; máscara antecede objeto RGB.

## Linhagem e validação

Owner único deste consumer, módulo privado ao hub. Helpers reexportados
conservam alcance de visibilidade anterior. Testes são locais e não expõem
novas APIs. Executar testes de images, suíte E2E export::tests, cargo build
e crystalline-lint .; não alterar políticas durante a movimentação.

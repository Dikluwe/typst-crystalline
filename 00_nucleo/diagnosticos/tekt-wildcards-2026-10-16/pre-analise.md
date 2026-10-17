# Próximo recorte V16 — pré-análise

Leitura durante a validação V17; nenhuma fonte ou L0 alterada. read-context.json.gz conserva a origem SARIF, UTC, HEAD/estado, fonte e L0 vigentes. Esta é pré-análise, não L0 nem decisão final de implementação.

MathStyleKind possui onze variantes atuais. Quatro fallbacks em map_glyph_vs, letter_base, digit_base e bmp_exception usam binding _other sem reincorporação; comentários N16[β] de neutralidade não satisfazem exaustividade do enum fechado. Os caracteres e flags são domínios diferentes e não devem virar listas finitas artificiais: nominalizar o eixo MathStyleKind, conservando fallback de char/flags e sem alterar mappings.

O L0 contém fragmento público histórico com nove variantes e texto Roundhand pré-P812 já refutado por suas próprias seções posteriores. Antes de código, atualizar a descrição vigente sem criar API: explicitar Display/Inline já existentes e precedência da normalização Roundhand e das medições posteriores; verificar os quatro fallbacks diretamente. Não alegar atualização de pin upstream ou paridade nova a partir desse saneamento.

Candidato a prova estrutural: numa cópia isolada do módulo, acrescentar uma variante sentinela e verificar se o compilador exige decisão nos quatro matches após o reparo; não editar enum produtivo. Manter controles de glyphs/VS/Greek/digits/size antes/depois. Essa prova precisa ser prescrita no L0 antes de execução, com baseline e RED do V16. Não implementar antes do encerramento e recibo final V17; o snapshot definitivo do novo lote deve partir do final-state V17.

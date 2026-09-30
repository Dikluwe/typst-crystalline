# Prompt L0 — infra/export/images/icc
Hash do Código: 4ec97dcd

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/images/icc.rs`
**ADRs:** ADR-0029, ADR-0095, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitado,
2026-09-26: esta família está misturada em 03_infra/src/export/images.rs
nos intervalos medidos em infra/export/images.md. A atomização desloca os
corpos, não redefine o comportamento. Nenhuma conclusão de paridade nova
é inferida das medições históricas do L0 anterior.

## Decisão e contrato

Owner dos bytes vendorizados dos perfis sRGB v4 e v2, dos getters
srgb_icc_profile_bytes/srgb_v2_icc_profile_bytes, do seletor por PdfVersion e de
build_icc_profile_stream. Preserva os bytes e a licença/proveniência histórica.
Match exaustivo: V1_4/V1_5/V1_6 usam v2; V1_7/V2_0 usam v4. Perfil v2 tem o
SHA-256 af4efe28f6d311799865f325ba39184a2e978b113ca74124d60af7de22b105f4.
Stream ICC mantém /N 3, /Range [0 1 0 1 0 1], /FlateDecode e /Length dos bytes
armazenados. Compressão usa o helper do hub e preserva o fallback existente
ao dado cru em erro, sem decidir nova política. Nenhum perfil grayscale novo.
Aceitação: versões escolhem a versão ICC correspondente; perfil v2 mantém
identidade e o stream comprimido decodifica para os bytes do perfil.

## Linhagem e validação

Owner único deste consumer, módulo privado ao hub. Helpers reexportados
conservam alcance de visibilidade anterior. Testes são locais e não expõem
novas APIs. Executar testes de images, suíte E2E export::tests, cargo build
e crystalline-lint .; não alterar políticas durante a movimentação.

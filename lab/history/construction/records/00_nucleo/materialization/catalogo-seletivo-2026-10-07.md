# Prioridade 3 — pacote completo preparado

Estado atual: pacote confirmado com “confirmo” e materializado. Preparação original abaixo conservada como histórico. Relatório final: `00_nucleo/diagnosticos/catalogo-seletivo-execucao-2026-10-07.md`.

Contrato completo: `00_nucleo/materialization/catalogo-seletivo-2026-10-07/contract.md`. Dois candidatos L0 completos, owners exclusivos1:1, base fingerprints e manifesto. Manifesto SHA-256 `045ef18d7a16143e640669cf5923db1a213eb677b6c278d4ee4a0bf542973c2e`. Sem nova API pública; helper L3 privado. SharedFile preserva bytes retidos; Source::File regular/Unix tenta leitura limitada, demais casos usam with_face_data original. Buffer parcial nunca serve a FontSlot/shaping/export. Caps e estabilidade explícitos, RED→GREEN/integração/PDF/tempo/RSS exigidos.

Auditoria vigente: `00_nucleo/diagnosticos/catalogo-seletivo-preparacao-2026-10-07/audit.json` e before.json.gz. 1.140 faces elegíveis nos caps,20 registros e259.927 bytes máximos;1.086 assets históricos conservados, metadata body idêntico. Coverage atual já usa intervalos; ganho conjunto ainda não medido. Release e44043d9471a6d2eec25da7562dd5b0a071d4e14ab0f307011ca978e46e49109 preservado.

Gate: `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md` §2.2: “troca de caminho por defeito do compilador”; §5: “em caso de dúvida sobre a classe ... parar”. Classificação registrada: novo caminho padrão de aquisição substitui cláusula atual de Database::with_face_data. A ratificação autoriza materialização do pacote completo, seus testes/fallback/selos e medições sem nova paragem por detalhes privados. A confirmação anterior autorizou exclusivamente prioridade2 e não este caminho novo.

Verificação da preparação: preservation.json confirma todos os bytes produtivos/L0/binário preservados; candidatos/manifesto consistentes; linter sem erros (535 warnings históricos). Nenhum teste/build de código ainda não ratificado.


Execução: dois L0/consumers, RED→GREEN, fallback/SharedFile/identidade/caps verificados,7.954 testes e builds/lint aprovados; catálogo/PDF/CLI/RSS medidos em fontes finais. Relatório preserva medições iniciais e finais e diferenças vanilla. Sem commit.

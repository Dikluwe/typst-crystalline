# Manutenção prevista de duas referências

O L0 vigente `00_nucleo/prompts/infra/export-fixtures.md` autoriza regeneração
seletiva após mudança observável intencional, com justificação e verificação.
A correção de fronteira física está especificada no owner de Shape e tem
quatro regressões RED→GREEN, 32 controles bilaterais e prova das coordenadas.
A suíte em execução registra somente dois snapshots com bytes diferentes:
04-shapes e 07-multi-feature. Não se altera o harness, seu L0, fontes ou asserts.

A inspeção anterior à manutenção de 04-shapes via qpdf mostra o círculo
transladado 7.7pt para baixo, removendo a subtração indevida de cap-height;
retângulo, cor, raio e demais operadores são preservados. O CLI Compact/Disabled
produz 2326 bytes, igual ao tamanho candidato do harness (a identidade será
verificada, não inferida pelo tamanho). A fórmula do pin põe o topo do círculo
em 70.86614+50+13.2=134.06614pt. O novo PDF coloca 841.8898−707.824=134.0658pt,
com a precisão decimal vigente do stream. PDF binário vanilla não é requisito.

07 usa fontes internas no harness e fontes descobertas no CLI; não comparar
seus bytes como se as configurações fossem iguais. Inspecionar a referência
gerada pelo próprio harness, preservando fontes/gradiente/texto/destinos e
limitando mudanças às posições verticais decorrentes do fim físico da forma.
Se houver mudança não explicada, não certificar por atualização de snapshot.

Antes de qualquer escrita, before.json.gz preserva PDFs/fontes/L0/harness e
manifesto com SHA-256/base64, HEAD/working tree/status/diffstat/UTC. Regerar
somente 04/07 duas vezes, validar determinismo, os outros sete PDFs intactos,
qpdf, texto/streams/raster e repetir nove casos sem UPDATE. Atualizar MANIFEST.

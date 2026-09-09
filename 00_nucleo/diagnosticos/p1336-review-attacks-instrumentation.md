# P1336 — observação independente sobre cache adversarial

Manifesto `54864bfd67d50197560011d5aae055a0303fcc9cadc4230221dfd4f7a6daea88`. O plano e os seis patches produtivos foram inspecionados e correspondem às famílias congeladas; seus testes permanecem byte-idênticos. A validade dinâmica ainda não foi concedida.

Na revisão do runner `p1336-attacks-mutate.py`, após preparação das fontes em `2026-09-09T18:18:43.360920+00:00`, observou-se que o loop instala cada snapshot por `shutil.copy2` no mesmo caminho de workspace. Isso conserva mtimes anteriores às compilações anteriores. É um risco de reutilização indevida do artefato Cargo: o hash registrado da fonte prova os bytes presentes, mas sozinho não demonstra que foram compilados. O critério inicial `Finished release` e `running 5 tests` também pode ser satisfeito por cache.

O reviewer solicitou ao adversário instrumentar timestamp fresco no source da cópia exclusiva após cada instalação, ou demonstrar recompilação e identidade efetiva do artefato para cada mutante. Nenhuma mudança de intenção, fonte congelada, suíte ou expectativa foi solicitada. Se algum ensaio antigo estiver sob dúvida, deve permanecer registrado como evidência instrumental insuficiente, sem crédito de eliminação, até reprodução adequada.

Esta é revisão de capacidade de observar o mutante, não falha do candidato. Não autoriza mudança de código principal, teste privado ou plano semântico. O veredicto final exigirá candidato positivo, fontes produtivas válidas, recompilação demonstrada e falha discriminatória correspondente.

# P1336 — revisão independente do candidato

Veredicto estático: candidato conforme ao escopo e à intenção congelada; fechamento permanece pendente dos gates dinâmicos.

Manifesto `54864bfd67d50197560011d5aae055a0303fcc9cadc4230221dfd4f7a6daea88`; candidato `p1336-candidate.json`, SHA `d8295978c3ec103730a82e5286c75eda7c87038bd929783ad045c6a595819e94`. Proveniência da árvore e hash observado pelo reviewer em `p1336-review-candidate-audit.json`; HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada. Somente o par L0/consumer difere do baseline sujo, nenhum novo arquivo produtivo, 5022 artefatos históricos preservados, corpo normativo congelado intacto.

Comparação contra `manifest.pre_candidate_source`, após retirar unicamente o módulo local independente, encontrou dois deltas produtivos: `Value::Int/Str` passam a integrar a seleção `access.field().span()`; um braço dedicado de erro Int/Str usa o helper existente `vanilla_type_name`. O fallback das demais variantes conserva literalmente `other.type_name()` e o span recebido. O braço puro Int/Str consome o span recebido sem reconstruí-lo. Não há nomes de fixtures, condicionais por valor ou blacklist de identificadores.

Não há alteração de assinatura, imports, entidade, API, feature/default, ordem de avaliação ou fase. O match fechado distingue instâncias de valores-tipo; métodos/pré-despacho, `field_callee_error`, gates antecipados e contexto text.size/lang são byte-idênticos ao antecedente. Os ramos Module, Dict, Content/LocatedContent, Float, Native Some/None, Closure/With e warnings também permanecem iguais. Essa observação sustenta preservação estática; não é alegação de execução bilateral de text contextual.

O módulo de testes congelado não foi alterado para acomodar C. A correção implementa a regra geral do L0 para quaisquer valores/campos alcançando o lookup. A única dívida Str.len interceptada antes desse lookup continua fora do reparo, como decidido e classificado antes de C.

Ainda necessários: GREEN real, build e testes workspace, teste legado LocatedContent, matriz A/B quatro perfis/três ordens com canais integrais, mutantes compiláveis rejeitados pelo sintoma correspondente, resselo bidirecional V5/V15/V26 e preservação final. Não há veredicto de equivalência funcional geral.

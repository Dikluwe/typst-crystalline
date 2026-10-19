# Gate concreto — completar os dois resíduos de query

Status atual: confirmado pelo humano (confirmo) e materializado.
Recibo: query-compilada-aprovacao-2026-10-06.json. Candidatos/manifesto
originais abaixo são histórico; L0s proprietários vigentes foram aplicados.

Medição: 00_nucleo/diagnosticos/query-residuos-2026-10-06/before.json.gz.
O terceiro resíduo (arquivo inexistente) foi corrigido em fluxo contínuo.

A CLI usa avaliação inicial e parsing limitado; vanilla compila primeiro.
Reutilizar compilação paginada existente corrige execução contextual, consulta
sobre conteúdo gerado e precedência de erro antes do selector. Avaliar selector
com o avaliador da linguagem corrige .where e outras composições, em scope stdlib
sem bindings do documento. A enum explícita distingue compilation/retrieval,
conservando a ordem de diagnóstico e a ausência de depreciação no early error.

Novas APIs propostas, com assinaturas integrais nos candidatos:

- pipeline::compile_query_context(world, source) → introspector final + warnings.
- query_helpers::CompiledQueryError::{Compilation, Retrieval}.
- query_helpers::query_compiled_elements(world, source, selector) → valores/erro tipado + warnings.

Cinco L0s completos e hashes base/candidato estão em
query-compilada-2026-10-06/manifest.json. O contrato legado de query_elements e
query_to_summary permanece para consumers existentes; nenhum novo arquivo
produtivo, nenhum owner compartilhado. Paged já existe e permanece responsável
por avaliação única, realização, callbacks e orçamento de estabilização.

ADR-0127 §2 exige confirmação para assinatura pública nova e mudança de fase/
ordem do pipeline. Confirmar autoriza aplicar os L0s candidatos, escrever testes
RED, materializar estas fachadas e compor query, ressellar somente owners afetados,
rodar build/lint e repetir controles bilaterais. Não autoriza targets/flags novas,
API de watch, unsafe, math phases ou propostas PDF/bundle pendentes.

Entrega e checks em ../diagnosticos/query-compilada-execucao-2026-10-06.md;
os três controles originais estão fechados. O manifesto original permanece
inalterado para proveniência da aprovação, não para comparar o L0 materializado.

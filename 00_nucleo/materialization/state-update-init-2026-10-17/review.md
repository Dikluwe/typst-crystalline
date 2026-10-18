# Revisão A1-S1 — transportar o init de State no update

## Medição antes da proposta

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c; árvore inicial e UTC em
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
Vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3.
A coleta anterior (paridade-novo-vanilla-2026-10-17/supplement.json.gz,
callback-isolation) confirma update funcional ignorado e final sem valor.
value_methods/state.rs:25 trata Func como Set. StateUpdateElem tem só key/update;
ElementPayload::StateUpdate também. apply_state_funcs não executa sem init no registry.
O init do objeto não chega à ocorrência quando o State fica num let.
Embutir um Content::State extra em Sequence mudaria a morfologia do update;
registrar init globalmente em eval não preserva causalidade documental.

## Contrato público proposto, sujeito à aprovação ADR-0127 §2.1/§2.4

Adicionar `pub init: Option<Box<Value>>` a StateUpdateElem e
`init: Option<Box<Value>>` à variante ElementPayload::StateUpdate.
Some transporta o init do receiver, incluindo Some(Box::new(Value::None));
None conserva a ausência de receiver das nativas legadas. Não confundir ambos.
Content::state_update mantém assinatura atual, criando init None; o método
moderno anexa Some(state.init.clone()) à ocorrência já construída.
Element::to_payload preserva init; map/clone/hash/observação preservam o campo.
Consumidores Rust externos por literal/pattern exaustivo precisam migrar.
Custo de payload: um ponteiro opcional no elemento e na variante; o efeito no
layout do enum e no pico total será medido, sem prometer redução global.
Sem nova variante, vtable, estado global, API de registry pública ou novo modo.

## Implementação interna

Func é update funcional; outros Values são Set. O replay no post-pass já
existente processa State/Set/Func na ordem dos tags, evitando que Set futuro
preceda callback e preservando ordem documental (não ordem numérica de Location).
Inicializar a primeira ocorrência moderna pelo seu init quando necessário;
legado sem init conserva a regra vigente. Não inserir elemento extra visível.
Erros de callback propagam; callbacks não são executados na construção.
Nenhuma mudança de fase eval/layout, assinatura de callback, here/get/final
ou política global de convergência está autorizada.

## L0s donos a atualizar antes da materialização

- 00_nucleo/prompts/entities/elements/state_update.md: novo campo e to_payload.
- 00_nucleo/prompts/entities/element_payload.md: novo campo da variante.
- 00_nucleo/prompts/entities/content/construction.md: compatibilidade do helper.
- 00_nucleo/prompts/compiler/eval/observation/elements.md: comparar init causal.
- 00_nucleo/prompts/compiler/eval/bindings/value_methods/state.md: Func/Set e receiver init.
- 00_nucleo/prompts/compiler/introspect/populate.md: init no caminho sem Engine.
- 00_nucleo/prompts/compiler/introspect/from_tags/state_updates.md: replay ordenado.
- 00_nucleo/prompts/compiler/introspect/from_tags.md: migrar fixtures existentes.

## Aceitação S1–S8

S1 update função sobre init sem emitir State; S2 init None válido;
S3 sequência Func/Set/Func e localização não numérica; S4 erros/captura da closure;
S5 Set simples e dois estados independentes; S6 método não avalia callback ao
construir; S7 legado sem init e fixtures existentes; S8 preservação de campo
em clone/payload/observação e medição de size_of.
RED funcional antes de implementação; revalidação bilateral no novo pin,
workspace/build/Clippy/Rustdoc/linhagem estrita/reversa. Jobs sequenciais,
MemoryMax12GiB, High8GiB, swap0, jobs1 e debug0/incremental0 nesta validação.
Não inclui commit, push, merge, publicação ou release.

# Proposta retirada — A1-S1

A proposta review.md (SHA-256 c72a7d91557b42fbe80e82bdef3caf4ea7a6df095c4bbb56b0e852f896505635)
foi retirada antes de qualquer alteração de StateUpdateElem/ElementPayload.
Uma eventual aprovação dessa revisão antiga não autoriza materializá-la.

Contraprova normativa: upstream pin bd5ebec285, introspection/state.rs:226–245
explica que leitores da mesma key conservam seus próprios init; :477–520
começa o fold com state.init e aplica os updates. Portanto init por update
não fornece semântica correta para leitores com init distinto. Callbacks
sem leitura também não devem ser executados por mera presença no documento.
A revisão correta precisa tratar resolução por leitor e a fronteira de fase,
com novos controles bilaterais e gate ADR-0127 quando a solução estiver pronta.
Nenhum campo público dessa proposta foi adicionado.

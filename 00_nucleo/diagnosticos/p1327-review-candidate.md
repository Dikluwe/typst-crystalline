# P1327 — revisão estática do candidato

Regime A/B sem atestação de isolamento. Manifesto
`cca33a6a95d40efa74d6dc8f7f910a711968a4c7bb771393ffc975bc9e515ea8`;
baseline e RED R1 conforme `p1327-review-red.md`. Esta revisão não modifica
produto, L0, testes ou oráculos. Gates finais e resselo recíproco pendentes.

Fonte modules.rs candidata inspecionada, SHA-256
`f094fb15083d1375fcd973b8c28be4fb541a66ea1d25d677b8d327af54c78666`.
O diff funcional contém somente a condição no braço `None` de imports:
ausência de `new_name` e `Expr::Ident`, seguida de
`engine.sink.warn_note(source_span, "this import has no effect", "")`.

A resolução do Module e a validação de `bare_name` precedem esse bloco e
seus erros retornam antes do warning. O literal-file resolve em braço
distinto da fonte e não satisfaz Ident; fields/dinâmicos não satisfazem Ident;
rename falha no guard; items/wildcard nem entram nesse braço. Módulos de
qualquer origem, alias ou nome público seguem a condição sintática.

O mesmo binding lexical permanece definido imediatamente após o warning;
não há remoção, alteração de lookup/valor, early return novo ou avaliação
extra da fonte. Warning já emitido fica no sink para erro posterior.
Uso da API tracked existente conserva a política de deduplicação e a rota
dos módulos importados. Confirmei `world_types.rs:511-524`: string hint vazia
produz Warning sem hint e delega a `record`, sem criar trace. Não há motivo
estático para estender o owner set.

Recalculei os hashes normativos de ambos os L0 e continuam iguais ao
manifesto congelado. O arquivo completo de testes permanece byte-idêntico
ao R1 independente, SHA-256
`dd017795b6b9e9f8977d2ba2a2b3dac65979b6d7708f19bba747db4fa9e026a8`.
Nenhuma adaptação de oracle ao candidato.

Veredito estático: PASS no fragmento contratado. Não substitui GREEN,
transcripts CLI finais, preservação global, build, lint ou linhagem final.

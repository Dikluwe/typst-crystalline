# Gate de perfil não é execução

O leitor final-ledger R0 parou com KeyError `channel_state` ao atingir um gate
de perfil: linhas DISABLED_BY_PROFILE não executadas não possuem esse campo,
e o TSV conserva célula vazia. Nenhum resultado R0 foi emitido. O sucessor R1
aceita somente a representação vazia desse campo ausente, continuando a exigir
projection_state idêntico e cobertura integral. Isso não transforma gate em MATCH.

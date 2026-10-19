# Proposta — Display e FromIterator

Quatro contratos Rust: Display para Decimal/Duration/Version e FromIterator<Style> para Styles. Os três métodos inerentes to_string saem e as chamadas usuais passam ao ToString padrão; Styles::from_iter continua como adapter com a mesma assinatura. Os parsers Option e os next de domínio permanecem. A proposta preserva os textos atuais (incluindo Duration MIN corrigido), dobra estilos na mesma ordem e não altera defaults, campos, versão, repr Typst ou fase.

Os quatro L0 completos estão em proposed/ e o delta em proposal.diff; base.json.gz preserva os owners/fontes/hashes e estado da árvore. Testes T1–T5 estão prescritos em cada owner proposto. Não inclui mudar enums públicos, FromStr, publicar release ou alterar as 48 exceções de aridade.

A ADR-0127 §2 exige confirmação porque são novos contratos públicos e há migração de métodos: “alterar assinatura pública de função L1–L4” e “qualquer mudança que, se revertida depois, quebraria código já escrito contra ela (binário/API)”. Chamadas Rust usuais .to_string() continuam disponíveis pelo prelude; usuários que dependam especificamente da definição inerente precisam considerar a migração para ToString. Não aplicar os L0 ou código antes da ratificação.

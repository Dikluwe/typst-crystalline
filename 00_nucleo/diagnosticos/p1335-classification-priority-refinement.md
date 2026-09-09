# P1335 — revisão causal de prioridade antes da seleção

## Medição e fonte

O normal fresco `p1335-sentinels-normal-r3.json` reproduz, nos quatro perfis:

- `p1325.integer-boundary`, `(1).nope`: vanilla `cannot access fields on type integer`, cristalino `cannot access fields on type int`; field-only versus acesso inteiro.
- `p1325.string-boundary`, `"abc".nope`: vanilla `cannot access fields on type string`, cristalino `cannot access fields on type str`; a mesma divergência de âncora.

Não são mudanças de estado entre fases: o candidato é o binário P1335 congelado, a árvore é a do manifesto P1335 e ambos os canais integrais estão no recibo. Repetição/inversão ainda são gates independentes da decisão final.

Em `00_nucleo/prompts/compiler/eval/bindings/field_access.md:320`, o critério vigente é explicitamente `#(1).foo → Err (nomeia "integer")`. As preservações estreitas de P1324/P1325/P1326 limitam aqueles reparos, mas não revogam esse critério. A sugestão inicial de prioridade 3 em `p1335-classification-causal-specs-r2.json` ignorava esta cláusula. A revisão independente encontrou o erro; não é válida a justificação de que todo diagnóstico restante seria apenas dívida sem contradição L0.

Na fonte atual, `01_core/src/compiler/eval/bindings/field_access.rs:533` exclui Int/Str da seleção field-only e `:836` usa `other.type_name()` no mesmo fallback para ambos. O helper `vanilla_type_name` já é importado em `:28` e devolve `integer`/`string` em `01_core/src/compiler/eval/operators/error_formatting.rs:58,60`. A função já é acessível sem novo reexport/API/consumer. Nenhuma causa é inferida de um nome de teste: o match de variantes e o uso do nome curto são os mesmos para qualquer Int/Str que chegue a esse fallback.

## Decisão refinada, não seleção

`primitive-instance-field-diagnostic` passa a prioridade **2**, por contradição canônica específica no caso Int. Int e Str continuam na mesma coorte causal: duas variantes no mesmo fallback de mensagem e no mesmo seletor de origem; não duas prioridades fundidas apenas para aumentar paths. Há duas rotas reais, `integer.instance.missing-field` e `string.instance.missing-field`; aliases, valores e nomes de field não multiplicam o peso.

Um owner completo permanece a hipótese: `compiler/eval/bindings/field_access.md` → `01_core/src/compiler/eval/bindings/field_access.rs`. Superfície demonstrada de regressão: rank 1, alteração local apenas nos erros de acesso direto de instância Int/Str, mantendo métodos existentes, Type::Int/Type::Str, funções, chamadas com argumentos, outros valores e todos os fechamentos P1324–P1326. O recorte não altera operadores/error_formatting, que só fornece helper já existente.

O inteiro é uma contradição L0; a âncora e a extensão à string são correção diagnóstica da mesma causa completa. Caso se demonstrasse que um deles exige outro ramo/owner/semântica, seria obrigatório separar as coortes e recomputar o ranking — nunca manter um agrupamento conveniente.

Refutam esta hipótese: paridade atual no mesmo literal/perfil, aplicação a métodos bem-sucedidos, necessidade de outro consumer, alteração da representação pública `type()/repr()` para obter o nome longo, ou mudança no dispatch/callee. A proteção dos métodos e de outros valores deverá ser atestada no passo de materialização, não é certificada por esta auditoria.

O candidato de resolução math continua separado e prioridade 2. Sua hipótese de risco 2 alcança o dispatch de chamadas, enquanto este recorte é de mensagem/origem local. Nenhum vencedor é declarado aqui: a comparação final depende de todos os gates e testemunhas atuais, seguida de veredito de outro papel.

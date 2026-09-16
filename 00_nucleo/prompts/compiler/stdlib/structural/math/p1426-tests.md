# Testes P1426 — `math.root` qualificado, carrier, `repr` e walkers

## Autoridade

Prompt test-only derivado do contrato congelado P1426. Legitima exclusivamente
`01_core/src/compiler/stdlib/structural/math/p1426_tests.rs`; não legitima código
produtivo nem escolhe a implementação.

## Obrigações

1. provar que `math.root` existe como função nativa pública com nome `root`;
2. provar a assinatura posicional medida: índice e radicando obrigatórios para
   sucesso, casts de `Content | none` e os diagnósticos N01–N07;
3. observar diretamente os carriers `None/false`, `None/true` e
   `Some(content)/true`, incluindo a coerção de radicando `none` para vazio;
4. exigir as três morfologias canônicas de `repr`, sem inferir presença a
   partir de `Option`;
5. provar que clone, igualdade, hash, `map_content`, `map_text`, `plain_text`
   e `is_empty` preservam a semântica existente e o bit causal.

Falha de identidade, compilação não atribuível ao contrato, observação opaca,
timeout ou saída ilegível é `Unknown` fail-closed. O teste não pode editar a
solução para obter GREEN.


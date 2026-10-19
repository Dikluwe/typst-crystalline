# P1177.1 — materialização da família HTML ruby

**Data final:** 2026-08-25T15:54:43-03:00  
**HEAD:** `ce49041de76eb64c011e990e9774a0339f979211`  
**Estado medido:** working tree não commitada; índice vazio

## Entrega

- `html.ruby`, `html.rp` e `html.rt` foram acrescentados ao dispatcher
  estático global-only, totalizando 58 tags tipadas;
- cada função reutiliza os 76 atributos globais e body Content opcional;
- `ruby` permanece agrupável pela tabela L3 existente; `rp` e `rt` isolados
  permanecem boundaries;
- no body de `ruby`, `Content::Space` entre dois filhos imediatos `rp`/`rt` é
  descartado antes da proteção de espaço;
- espaços após texto-base e em torno de inline comum permanecem literais;
- nenhuma entidade, validação parental, default ou fase mudou.

## RED → GREEN

O RED L1 falhou por ausência de `html.ruby`; o RED L3 falhou porque o exporter
emitia espaço entre `rp`/`rt`. Após a implementação, ambos passaram, assim
como a integração CLI de repr, grouping e boundaries.

A fixture tipada compartilhada foi compilada pelos dois binários. Vanilla e
cristalino produziram o mesmo SHA-256 HTML:
`75d997ac7181e7d6282fb05943ddff7e8ef3a4d4ff78d4d98bc296d77d50e5a1`.
O binário cristalino final medido tinha SHA-256
`9d73429527721d4bf6cda1a8ad0049c3c77f5dbfbf3cd2670cb14fe8e2ede325`.

## Validação final

- `typst-core`: 5249 passed; 3 doctests ignored;
- `typst-infra`: 855 passed;
- `typst-shell`: 55 passed;
- `typst-wiring`: 69 testes CLI + 4 auxiliares passed;
- `cargo build`: exit 0;
- `cargo fmt --all -- --check`: exit 0;
- `crystalline-lint .`: exit 0;
- `git diff --check`: exit 0;
- índice: vazio.

Os avisos de compilação e informações V19/V20 do linter são preexistentes e
não são violações. Nenhum staging ou commit foi realizado.

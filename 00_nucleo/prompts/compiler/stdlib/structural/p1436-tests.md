# Prompt L0 — testes independentes de `math.lr` P1436

**Camada**: L1 test-only  
**Alvo**: `01_core/src/compiler/stdlib/structural/p1436_tests.rs`

## Entradas congeladas

Passo P1436 aprovado, manifesto, contrato e oráculos P1436. Estes testes não
derivam expectativas do patch produtivo.

## Obrigações

- o módulo `math` contém função `lr`;
- zero, um e muitos posicionais constroem `Content::MathLr` e body canônico;
- múltiplos posicionais recebem vírgula matemática sem perder ordem;
- `size` relativo conserva valor e presença, inclusive `100%` explícito;
- `repr` usa `lr(size?, body: ...)` e cascata não fabrica campo;
- inteiro como body, tipo inválido de size, named desconhecido, `body:` e
  duplicado falham fechado;
- igualdade e walker distinguem/preservam body, size e presença;
- `MathDelimited` mecânico continua variante distinta.

Os controles positivos devem permanecer verdes; ausência de identidade,
falha de build, timeout ou observação opaca é `Unknown`, nunca sucesso.

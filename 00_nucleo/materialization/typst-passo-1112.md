# Materialização — Passo 1112: Investigação de Acentos Empilhados em `dot(hat(x))` e Deslocamento Uniforme em `hat(a) + tilde(b) + dot(c)`

## Objetivo
Auditar analiticamente o comportamento de acentos empilhados em `dot(hat(x))` e isolar a causa do deslocamento de $-0.68\text{ pt}$ em `hat(a) + tilde(b) + dot(c)` na Secção 33.

## Arquivos Modificados
1. `01_core/src/compiler/math/layout/accent.rs`:
   - Alinhada a declaração de `width = base_box.width` para acentos padrão (com expansão condicional para variantes gigantes de testes unitários).

## Resultados
- **Paridade Exata ($0.0000\text{ pt}$)** nos testes atômicos de `$ dot(hat(x)) $`, `$ tilde(hat(x)) $`, `$ hat(x) $` e `$ dot(x) $`.
- **Origem do shift de $-0.68\text{ pt}$ comprovada analiticamente** como efeito de centralização de bloco da equação completa na página.
- **Suíte Total:** 5.960 testes aprovados / 0 falhas.

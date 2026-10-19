# Relatório do Passo 1140.3 — fechamento de `WRONG_KIND` em `math`

**Data:** 2026-08-23  
**Vanilla ratificado:** `a51e02804`  
**Estado medido:** HEAD `a8959bd184871d72f470ee4dd06d829e6ff0e483`,
working tree não commitada, `2026-08-23T22:45:32-03:00`.

## Resultado

As duas divergências de kind foram atomizadas e corrigidas:

- `math.sqrt`: `symbol → function`; produz `MathRoot`, compartilha a unidade
  construtora com `$sqrt(x)$`, e o binding extra `sym.sqrt` foi removido;
- `math.equation`: `none → function`; a fase P1140.3-B aceita `body: Content`
  obrigatório e `block: bool = false`, compartilhando a construção de
  `Content::Equation` com `$...$`.

Os named args válidos no vanilla `numbering`, `number-align`, `supplement` e
`alt` foram medidos e atribuídos explicitamente ao P1140.4. P1140.3-B os
rejeita; nenhum argumento é ignorado silenciosamente.

## Probes e inventário

Os probes focados reproduzem:

```text
repr(type(math.sqrt))                    → "function"
repr(math.sqrt([x]))                     → "root(radicand: [x])"
repr(type(math.equation))                → "function"
repr(math.equation([x]))                 → "equation(body: [x])"
repr(math.equation(block: true, [x]))    → "equation(block: true, body: [x])"
```

O inventário final (`/tmp/p11403b-merged.json`, gerado a partir de release
rebuild) contém **0 `WRONG_KIND`**. A distribuição relevante mudou de
`WRONG_KIND: 2` antes do passo para `WRONG_KIND: 0`; `math.equation` passou a
`UNVERIFIED_METADATA`, coerente com a fase de campos declarada para P1140.4.

## Verificação

- testes P1140.3 focados: **5 passed, 0 failed**;
- suíte `typst-core --lib`: **5126 passed, 2 failed**;
- as duas falhas remanescentes são preexistentes e fora deste diff:
  `p862_repr_plain_text_splits_on_space` e
  `p862_content_tree_splits_plain_text_on_space`;
- o teste histórico P731 foi atualizado do antigo placeholder `none` para a
  função pública e passou;
- build release do binário `typst`: passou;
- `cargo fmt --check`: passou;
- `git diff --check`: passou;
- `crystalline-lint .`: exit 0, sem violations de trava; permanecem avisos e
  informações já existentes do linter ampliado.

Na medição final, `git diff HEAD --stat` registrou **25 ficheiros**, **331
inserções** e **43 remoções**; ficheiros novos ainda não rastreados não entram
nessa contagem.

## Estado posterior

P1140.3 está fechado. O próximo contrato já nomeado é P1140.4, destinado aos
quatro campos públicos restantes de `math.equation`; `math.root` continua como
`MISSING_MEMBER` fora deste passo.

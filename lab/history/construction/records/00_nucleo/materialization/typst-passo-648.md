---
# P648 — Propagar erros de sintaxe pelo eval, sem repetir as regressões de P634

> **Passo:** 648
> **Data:** 2026-07-09
> **Foco:** Dois casos ficaram por resolver ao longo desta sequência, ambos pela mesma causa: o parser/lexer já detecta o erro correctamente e produz um nó `SyntaxKind::Error`, mas o eval ignora esse nó — `#let x = 0xZZ` vira `x = none` (P634), e escape unicode inválido em markup compila sem erro apesar do lexer já ter gerado a mensagem certa (P643). P634 tentou propagar todos os erros de parser de uma vez e encontrou dez regressões (smart quotes, `#set` dentro de blocos, escapes válidos). Este passo faz o trabalho com mais cuidado — sonda primeiro, entender cada regressão antes de tentar de novo.
> **Tipo:** Sonda + Implementação. O item mais delicado desta linha de trabalho.
> **Tamanho:** L.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — já houve uma tentativa falhada nesta área exacta; sonda obrigatória, sem excepção, antes de qualquer código.

---

## Contexto

Confirmado por P634 e P643, em locais diferentes do código, o mesmo padrão: o parser produz um nó de erro correctamente, mas em algum ponto entre o parser e o resultado final, esse erro é descartado, não propagado. A tentativa anterior de propagar "todos os erros de parser" de uma vez, no entrypoint (`eval_with_full_error`), quebrou dez testes existentes.

---

## Sonda

### Reconstituir exactamente as dez regressões de P634

```bash
git log --all --oneline | grep -i "P634" 
git show <commit-da-tentativa-revertida> --stat 2>/dev/null
```

Se a tentativa revertida ainda estiver acessível no histórico (mesmo que não tenha sido mantida), examinar exactamente quais dos dez testes falharam e porquê. Se não estiver acessível, reconstituir a tentativa (propagar erros no entrypoint) num branch/working tree separado, só para ver as falhas, sem committar.

### Classificar as regressões por causa

Para cada uma das dez falhas, confirmar se a causa é:

1. **Um nó de erro genuíno a meio de uma construção válida** — por exemplo, o parser marca uma secção como erro por um motivo que não devia impedir o resto da construção de ser avaliada (falso positivo do parser, não do eval).
2. **Smart quotes especificamente** — já mencionado por P634; confirmar se smart quotes produzem nós de erro em situações que são, na verdade, válidas (uso legítimo de aspas dentro de código), e o parser está a assinalar isso por engano.
3. **`#set` dentro de blocos** — confirmar se há um nó de erro estrutural que aparece sempre nesta construção, mesmo quando o código é válido, sugerindo um bug do parser nesta área específica, não um erro genuíno a propagar.

### Critério de fecho da sonda

- [ ] As dez regressões reconstituídas e confirmadas, uma a uma.
- [ ] Cada uma classificada: é um nó de erro genuíno que devia mesmo propagar-se, ou é o parser a marcar algo como erro quando não devia (bug do parser, não falta de propagação no eval)?
- [ ] Se houver bugs do parser a marcar coisas válidas como erro: esses são passos à parte, anteriores a este.
- [ ] Só os nós de erro genuinamente correctos (`0xZZ`, escape unicode inválido, e semelhantes) ficam no âmbito da propagação deste passo.

---

## Implementação

Depende inteiramente da sonda. A direcção mais provável: propagar erros de forma mais selectiva do que a tentativa anterior — por tipo de nó de erro, não "todos de uma vez" — começando pelos dois casos já confirmados (`0xZZ`, escape unicode em markup), e só estendendo a mais tipos de erro depois de cada um ser confirmado como seguro.

### Critério de fecho da implementação

- [ ] `#let x = 0xZZ` produz erro.
- [ ] Escape unicode inválido em markup produz erro (o teste `p643_invalid_unicode_escape_markup_still_silent` de P643 passa a confirmar erro, não silêncio).
- [ ] As dez regressões anteriores não se repetem — testado uma a uma, não só "a suite passa".
- [ ] Se algum bug de parser for encontrado pelo caminho (marcando código válido como erro): registado como passo à parte, não misturado com esta correcção.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Correr especificamente os dez testes que falharam na tentativa anterior, confirmando que continuam a passar:

```bash
# lista a preencher depois da sonda reconstituir quais eram
cargo test -p typst-core <nome_do_teste_1>
cargo test -p typst-core <nome_do_teste_2>
# ...
```

```bash
cat > /tmp/p648-0xzz.typ <<'EOF'
#let x = 0xZZ
EOF
./target/release/typst /tmp/p648-0xzz.typ /tmp/p648.pdf
echo "Exit code: $?"
```

```bash
cat > /tmp/p648-escape-markup.typ <<'EOF'
[\u{FFFFFFFF}]
EOF
./target/release/typst /tmp/p648-escape-markup.typ /tmp/p648-2.pdf
echo "Exit code: $?"
```

---

## Critério de fecho do passo

- [ ] Sonda completa, dez regressões reconstituídas e classificadas.
- [ ] `0xZZ` e escape unicode em markup produzem erro.
- [ ] Nenhuma das dez regressões anteriores repetida.
- [ ] Bugs de parser encontrados pelo caminho, se algum, registados à parte.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p648.md`, com hash do commit.

---

## Estado da sequência de falhas silenciosas

Com este passo, fecha-se por completo a linha de trabalho iniciada em P633: 293 ocorrências varridas, 23 confirmadas, todas corrigidas ou reclassificadas com razão. Se este passo não conseguir avançar (por exemplo, se a sonda confirmar que a propagação segura não é possível sem mais trabalho no parser em si), o estado fica registado com honestidade — "tentado, âmbito maior do que o previsto, passo seguinte necessário" — não escondido como se estivesse resolvido.

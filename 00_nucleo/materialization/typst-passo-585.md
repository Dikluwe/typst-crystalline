---
# P585 — Resolver a contradição entre as duas versões de P583, com o ficheiro em si

> **Passo:** 585
> **Data:** 2026-07-05
> **Foco:** Existem duas versões do relatório de P583, ambas a citar o mesmo commit (`f61461c03`). Uma diz que o teste `p581_cobertura_de_escape_e_shorthand_em_layout` não existe. A outra diz que existe, em `01_core/src/rules/layout/tests.rs`, linhas 3488-3503. As duas não podem estar certas para o mesmo commit. Este passo não aceita mais nenhuma das duas frases — pede o conteúdo do ficheiro, directamente.
> **Tipo:** Verificação directa, sem relato — cópia literal do ficheiro.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.** Uma afirmação sobre a existência de código não se resolve com outra afirmação. Resolve-se mostrando o código.

---

## O que fazer

```bash
git rev-parse HEAD
git status --short
wc -l 01_core/src/rules/layout/tests.rs
sed -n '3480,3510p' 01_core/src/rules/layout/tests.rs
```

Colar a saída exacta destes quatro comandos no relatório. Não resumir, não descrever — colar o texto tal como sai do terminal.

Se o ficheiro tiver menos de 3488 linhas, isso já responde à pergunta sozinho. Se tiver mais, o conteúdo das linhas 3480 a 3510 mostra directamente se o teste está lá ou não, e com que nome exacto.

```bash
grep -n "p581_cobertura_de_escape_e_shorthand_em_layout\|fn.*escape.*shorthand\|fn.*shorthand.*escape" 01_core/src/rules/layout/tests.rs
```

Se esta busca não devolver nada, o teste não existe, independentemente do que qualquer relatório anterior tenha dito.

Correr o teste directamente, pelo nome, e mostrar o resultado exacto:

```bash
cargo test -p typst-core p581_cobertura_de_escape_e_shorthand_em_layout -- --exact 2>&1
```

Se o nome do teste não existir, este comando falha com uma mensagem clara ("test not found" ou 0 testes corridos) — essa mensagem, colada aqui, é a prova final.

## Critério de fecho

- [ ] Saída dos quatro comandos colada, sem resumo.
- [ ] Resultado de `cargo test` pelo nome exacto do teste, colado.
- [ ] Uma frase final, sem ambiguidade: "o teste existe, no ficheiro X, linha Y" ou "o teste não existe" — nada entre os dois.

## Depois disto

Se o teste não existir: retomar o P584 já escrito (criar o teste). Se existir: confirmar que corre e passa, e corrigir a versão do relatório de P583 que dizia o contrário, apagando essa versão, não deixando as duas em paralelo.

# Prompt — typst-passo-827: `typst_syntax::package` — mensagem de "pacote não encontrado" diverge para não-preview (achado #15 de P810)

**Origem**: achado #15 da tabela de P810
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> erro "pacote não encontrado" não-preview diverge (parsing em paridade verbatim, 5 casos)

Nota: o parsing da sintaxe de pacote em si já está confirmado em paridade (5 casos verbatim) — o problema é só na mensagem de erro quando o pacote não é encontrado, especificamente para pacotes que não são do namespace `@preview`.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. Compilar um documento com `#import "@algumnamespace/pacote:1.0.0"` para um namespace que não seja `@preview` (ex.: `@local` ou outro namespace configurado, ou um namespace inválido/inexistente — conferir no relatório de materialização de P810 o caso exacto usado) com os dois binários. Registar a mensagem de erro literal de cada um.
2. Comparar também o caso `@preview` já em paridade (controlo, para confirmar que só o não-preview diverge).
3. Localizar no vanilla (`lab/typst-original/`, `crates/typst-syntax/src/package.rs` ou equivalente) a rotina que gera a mensagem de "pacote não encontrado" e como ela varia por namespace.
4. Localizar o equivalente no cristalino e identificar a divergência.
5. Registar os pontos antes de tocar em código.

## Passo 2 — Implementação

Corrigir a mensagem de erro para pacotes não-preview não encontrados, replicando o texto e a lógica (se o vanilla varia a mensagem por tipo de namespace) do vanilla.

## Passo 3 — Validação

1. Recompilar. Repetir os comandos do Passo 1, mensagem literal batendo com o vanilla.
2. Confirmar que o caso `@preview` continua em paridade (controlo de regressão).
3. Teste novo cobrindo pacote não encontrado em namespace não-preview.
4. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-827-relatorio.md` com: medição antes, código vanilla/cristalino identificado, diff, medição depois, contagem de testes.

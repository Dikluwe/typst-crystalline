---
# P664 — Verificar directamente, sem depender de auto-rotulagem

> **Passo:** 664
> **Data:** 2026-07-09
> **Foco:** P663 procurou por divergências de linguagem usando palavras já escritas nos relatórios anteriores ("extensão", "capacidade nova"). Isso só encontra casos onde quem implementou já tinha percebido que estava a divergir — como aconteceu, por sorte, com `variant`. Não encontraria um caso onde ninguém percebeu. Este passo testa directamente contra o vanilla, sem depender de nenhum relatório anterior ter usado a palavra certa.
> **Tipo:** Sonda directa, ampla.
> **Tamanho:** L.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P663 (onde o método por palavras-chave foi usado, com esta limitação).

---

## Método

Em vez de procurar por auto-rotulagem, listar todos os argumentos nomeados que o cristalino aceita em `#set`/funções nativas tocadas ao longo desta conversa, e testar cada um directamente contra o vanilla — aceite ou não, com o mesmo comportamento.

### Parte 1 — Listar todos os argumentos nomeados reconhecidos

```bash
grep -n "\"[a-z_]*\" =>" 01_core/src/rules/eval/rules.rs | grep -oP '"\K[a-z_]+(?=")' | sort -u
```

Complementar com uma leitura directa dos ficheiros de regras (`rules.rs`, `bindings.rs`, `structural.rs`, `collections.rs`) para não depender só do padrão de grep, que pode não apanhar tudo.

### Parte 2 — Para cada regra/função, listar os argumentos nomeados que ela aceita, e testar cada um

Para cada função nativa ou regra `#set` já tocada nesta conversa (`page`, `text`, `document`, `figure`, `table`, `math.equation`, `heading`, `grid`, `counter.display`, `array.sorted`, e outras), construir um documento de teste com cada argumento nomeado, um de cada vez, e confirmar:

```bash
# Padrão do teste, repetido para cada combinação função×argumento:
cat > /tmp/p664-teste.typ <<EOF
#set <funcao>(<argumento>: <valor-de-tipo-certo>)
EOF
lab/typst-original/target/release/typst compile /tmp/p664-teste.typ /tmp/p664-vanilla.pdf
echo "Vanilla exit: $?"
./target/release/typst /tmp/p664-teste.typ /tmp/p664-cristalino.pdf
echo "Cristalino exit: $?"
```

Confirmar que os dois aceitam ou rejeitam da mesma forma. Prestar atenção especial a argumentos introduzidos ou tocados nos seguintes passos, por serem os que mais mexeram em sintaxe de argumentos:

- P536, P600, P601 (metadados de documento)
- P605, P606 (`outlined`/`bookmarked` de heading)
- P617 (confirmado, CLI apenas — não repetir)
- P636 (nove regras `#set`)
- P639, P661 (`table.numbering`/`caption`)
- P653 (`array.sorted(key:)`)

### Parte 3 — Testar também a ausência de argumentos que o vanilla tem e o cristalino não

O caso inverso também conta como divergência de linguagem: um documento que funciona no vanilla mas falha no cristalino, por um argumento que o vanilla aceita e o cristalino rejeita como "desconhecido". Confirmar, para cada função já tocada, se há argumentos do vanilla que o cristalino ainda não reconhece.

### Critério de fecho da sonda

- [ ] Lista completa de argumentos nomeados reconhecidos pelo cristalino, por função.
- [ ] Cada argumento testado directamente contra o vanilla, aceitação e comportamento.
- [ ] Testado também o caso inverso — argumentos do vanilla ausentes no cristalino.
- [ ] Qualquer divergência nova encontrada, classificada como P662-P663 já estabeleceram: extensão consciente, ou erro a corrigir.

---

## Decisão

Mesma estrutura das rondas anteriores. Cada divergência nova confirmada vira o seu próprio passo pequeno de decisão e, se for erro, reversão — seguindo exactamente o padrão já estabelecido por P662.

---

## Critério de fecho do passo

- [ ] Todos os argumentos nomeados já tocados nesta conversa, listados e testados directamente.
- [ ] Caso inverso (argumentos do vanilla ausentes) também testado.
- [ ] Divergências novas, se houver, classificadas e com passo de correcção próprio.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p664.md`, com a lista completa de testes e resultados, não só o resumo.

---
# P608 — A fusão de blocos `BT...ET` ainda se aplica?

> **Passo:** 608
> **Data:** 2026-07-05
> **Foco:** Registado em P534 como scope-out ("optimização de export, não correcção de fallback") e nunca mais verificado desde então. A lista de disparidades marca isto com uma nota a pedir confirmação. Muito código mudou desde P534 na área de export de texto (P543, P548, P558, P591, P593). Este passo confirma se o sintoma original ainda existe, ou se foi resolvido por acidente por algum dos passos posteriores.
> **Tipo:** Sonda directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P534 (onde o item foi registado), P543/P548/P558/P591/P593 (trabalho posterior na mesma área de export).

---

## Contexto

`BT...ET` são os operadores PDF que delimitam um bloco de texto. P534 encontrou que o cristalino gera vários blocos `BT...ET` separados onde o vanilla gera um só (ou menos), quando há fallback de fonte a meio de uma linha — cada troca de fonte fecha um bloco e abre outro. Isto não é um erro visual (o texto continua a aparecer certo), mas gera ficheiros PDF maiores do que precisavam de ser.

---

## Sonda

### Confirmar o sintoma directamente

```bash
cat > /tmp/p608-fallback.typ <<'EOF'
Hello 你好 مرحبا world, more latin text after the fallback scripts.
EOF
./target/release/typst /tmp/p608-fallback.typ /tmp/p608.pdf
grep -c "^BT$" <(strings /tmp/p608.pdf) 2>/dev/null || python3 -c "
data = open('/tmp/p608.pdf', 'rb').read()
print('Contagem de BT:', data.count(b'BT'))
print('Contagem de ET:', data.count(b'ET'))
"
```

```bash
lab/typst-original/target/release/typst compile /tmp/p608-fallback.typ /tmp/p608-vanilla.pdf
python3 -c "
data = open('/tmp/p608-vanilla.pdf', 'rb').read()
print('Contagem de BT:', data.count(b'BT'))
print('Contagem de ET:', data.count(b'ET'))
"
```

### Comparar tamanho de ficheiro

```bash
ls -la /tmp/p608.pdf /tmp/p608-vanilla.pdf
```

### Critério de fecho da sonda

- [x] Contagem de blocos `BT...ET` confirmada nos dois lados, para o mesmo documento com fallback de fonte.
- [ ] Se a contagem for igual ou próxima: o sintoma desapareceu, decisão a registar.
- [x] Se a contagem ainda divergir: confirmar se a diferença é do mesmo tamanho que em P534, ou mudou (para melhor ou pior) com o trabalho posterior.

---

## Decisão

Se o sintoma já não existir (por exemplo, porque a consolidação de P593 juntou a forma como os segmentos de texto são medidos e possivelmente também como são emitidos): actualizar a lista de disparidades, remover a nota de scope-out, marcar como corrigido incidentalmente, com o passo que provavelmente causou isso.

Se ainda existir: decidir se vale a pena corrigir agora (o ficheiro fica maior, mas funciona) ou manter como scope-out, desta vez com razão actualizada e reconfirmada, não só herdada de P534.

---

## Critério de fecho do passo

- [x] Sintoma confirmado directamente, com números, não com a memória de P534.
- [x] Decisão registada — mantém-se scope-out, com razão actualizada.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p608.md`, com hash do commit.
- [x] Lista de disparidades actualizada, tirando a nota de "confirmar se ainda se aplica".

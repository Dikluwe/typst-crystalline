---
# P613 — O vanilla mantém `DocumentID` estável, ou muda a cada compilação?

> **Passo:** 613
> **Data:** 2026-07-05
> **Foco:** P612 corrigiu o `DocumentID` para vir de um hash do conteúdo do documento, ficando estável entre compilações do mesmo conteúdo. Isto não foi confirmado contra o vanilla — só se testou que dois documentos diferentes produzem IDs diferentes, não se o mesmo documento, compilado duas vezes, produz o mesmo ID nos dois lados. Se o vanilla usar um valor aleatório a cada compilação, a semântica do cristalino (estável por conteúdo) é diferente, não igual.
> **Tipo:** Verificação directa.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P612 (onde a estratégia de hash de conteúdo foi decidida sem esta confirmação).

---

## Verificação

### Compilar o mesmo documento duas vezes no vanilla

```bash
cat > /tmp/p613-mesmo.typ <<'EOF'
Documento igual, compilado duas vezes.
EOF
lab/typst-original/target/release/typst compile /tmp/p613-mesmo.typ /tmp/p613-vanilla-1.pdf
sleep 2
lab/typst-original/target/release/typst compile /tmp/p613-mesmo.typ /tmp/p613-vanilla-2.pdf

for f in /tmp/p613-vanilla-1.pdf /tmp/p613-vanilla-2.pdf; do
  python3 -c "
import re
data = open('$f', 'rb').read()
m = re.search(rb'DocumentID>([^<]+)<', data)
print('$f DocumentID:', m.group(1) if m else 'não encontrado')
"
done
```

### Compilar o mesmo documento duas vezes no cristalino

```bash
./target/release/typst /tmp/p613-mesmo.typ /tmp/p613-cristalino-1.pdf
sleep 2
./target/release/typst /tmp/p613-mesmo.typ /tmp/p613-cristalino-2.pdf

for f in /tmp/p613-cristalino-1.pdf /tmp/p613-cristalino-2.pdf; do
  python3 -c "
import re
data = open('$f', 'rb').read()
m = re.search(rb'DocumentID>([^<]+)<', data)
print('$f DocumentID:', m.group(1) if m else 'não encontrado')
"
done
```

### Critério de fecho

- [ ] Confirmado se o vanilla produz o mesmo `DocumentID` nas duas compilações do mesmo documento, ou um diferente de cada vez.
- [ ] Confirmado o comportamento equivalente no cristalino (já esperado: igual, por vir de hash de conteúdo).
- [ ] Se as semânticas divergirem: decidir se o cristalino deve mudar para igualar o vanilla, ou se a decisão de P612 (estável por conteúdo) fica registada como divergência intencional, com razão nova.

---

## Decisão

Se o vanilla gerar um valor diferente a cada compilação (aleatório, não baseado em conteúdo): a escolha de P612 diverge da semântica real do vanilla. Não é necessariamente errado — um `DocumentID` estável por conteúdo tem o seu próprio mérito (permite reconhecer duas cópias do mesmo documento como "o mesmo", o que o vanilla não permitiria) — mas precisa de ficar registado como decisão consciente, com a razão escrita, não como "paridade com o vanilla" quando na verdade é outra coisa.

---

## Critério de fecho do passo

- [ ] Comportamento do vanilla confirmado com teste directo (duas compilações do mesmo documento).
- [ ] Comportamento do cristalino confirmado.
- [ ] Se divergirem: decisão registada com razão, não deixada como estava.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p613.md`, com hash do commit.

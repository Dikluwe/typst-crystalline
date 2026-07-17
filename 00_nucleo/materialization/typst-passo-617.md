---
# P617 — Bandeira opcional para `DocumentID` fornecido externamente

> **Passo:** 617
> **Data:** 2026-07-05
> **Foco:** Depois de P615 reverter para IDs aleatórios (correcto, como comportamento por defeito), fica a pergunta de quem precisa de rastrear identidade de documento entre compilações (fluxos parecidos com InDesign). A resposta certa não é tornar o compilador interactivo — é aceitar, por uma bandeira explícita, um valor de `DocumentID` fornecido de fora, deixado por defeito desligado. Isto é uma capacidade nova, não uma correcção de paridade — o vanilla não tem nada equivalente.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S–M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P615 (comportamento por defeito, aleatório, correcto e a manter), a conversa sobre a ideia do pré-compilador interactivo (rejeitada) e a alternativa de bandeira (aceite como direcção).

---

## Contexto

Um utilizador ou ferramenta externa que precise de manter o mesmo `DocumentID` entre compilações sucessivas do mesmo documento fonte (por exemplo, um sistema de gestão documental que já sabe qual era o ID da versão anterior) não tem hoje forma de o dizer ao compilador. A única opção é aceitar um ID novo a cada compilação.

---

## Sonda

### Confirmar se existe algum mecanismo parecido no vanilla ou noutros compiladores

```bash
lab/typst-original/target/release/typst compile --help | grep -i "id\|metadata"
```

Confirmar que não há nada equivalente — isto é mesmo uma capacidade nova do cristalino, não algo a replicar.

### Decidir a forma de entrada

Duas opções, a decidir:

1. **Bandeira de linha de comandos:** `typst compile --document-id <uuid> ficheiro.typ saida.pdf`.
2. **Variável de ambiente:** `CRYSTALLINE_DOCUMENT_ID=<uuid> typst compile ficheiro.typ saida.pdf`.

A bandeira de linha de comandos é mais explícita e mais fácil de descobrir (aparece no `--help`); a variável de ambiente é mais fácil de usar em scripts que já chamam o compilador sem querer mudar a lista de argumentos. Não são mutuamente exclusivas — pode haver as duas, com a bandeira a ter prioridade se ambas estiverem definidas.

### Critério de fecho da sonda

- [ ] Confirmado que não há mecanismo equivalente no vanilla.
- [ ] Decidida a forma de entrada (bandeira, variável de ambiente, ou as duas).

---

## Implementação

- Adicionar o parâmetro (bandeira e/ou variável de ambiente) ao ponto de entrada do CLI (`04_wiring`).
- Quando fornecido: validar que é um UUID bem formado (ou o formato que o vanilla usa para os campos base64 de 16 bytes, a confirmar contra o que P611 já mediu); se inválido, erro claro, não aceitação silenciosa.
- Quando fornecido: usar esse valor para `DocumentID`; `InstanceID` continua sempre aleatório (a identidade do documento pode ser fixa, mas cada compilação continua a ser uma instância diferente).
- Quando não fornecido (o caso por defeito): comportamento inalterado de P615 — aleatório.

### Critério de fecho da implementação

- [ ] Bandeira/variável reconhecida, documentada no `--help`.
- [ ] `DocumentID` fixo quando fornecido, `InstanceID` sempre aleatório.
- [ ] Valor inválido produz erro claro.
- [ ] Comportamento por defeito (sem a bandeira) inalterado — continua aleatório, como P615.

---

## Validação

```bash
./target/release/typst compile --document-id "f81d4fae-7dec-11d0-a765-00a0c91e6bf6" /tmp/p617-teste.typ /tmp/p617-1.pdf
./target/release/typst compile --document-id "f81d4fae-7dec-11d0-a765-00a0c91e6bf6" /tmp/p617-teste.typ /tmp/p617-2.pdf
```

Confirmar que os dois PDFs têm o mesmo `DocumentID` (fornecido) e `InstanceID` diferente (sempre aleatório).

```bash
./target/release/typst compile /tmp/p617-teste.typ /tmp/p617-3.pdf
```

Confirmar que, sem a bandeira, o comportamento continua igual a P615 (aleatório).

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, forma de entrada decidida.
- [ ] Bandeira/variável implementada, documentada.
- [ ] Comportamento por defeito inalterado.
- [ ] `InstanceID` continua sempre aleatório, mesmo com `DocumentID` fixo.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p617.md`, com hash do commit.
- [ ] Registado explicitamente como capacidade nova do cristalino, não como paridade com o vanilla.

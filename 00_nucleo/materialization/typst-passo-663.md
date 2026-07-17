---
# P663 — Auditoria por divergências de linguagem introduzidas sem confirmação

> **Passo:** 663
> **Data:** 2026-07-09
> **Foco:** P660/P662 confirmaram que uma extensão de sintaxe (`variant: (eixo: valor)`) foi introduzida sem confirmar primeiro se existia no vanilla — uma diferença de linguagem, categoria que este projecto não aceita, distinta de diferenças de implementação (aceitáveis). Este passo audita os mais de 660 passos desta conversa à procura de outros casos do mesmo tipo: sintaxe ou semântica que o cristalino aceita e o vanilla rejeita, ou vice-versa, introduzida sem confirmação directa na altura.
> **Tipo:** Sonda directa, ampla.
> **Tamanho:** L.
> **ADR-0108 EM VIGOR.** A distinção entre diferença de implementação (aceitável) e diferença de linguagem (inaceitável) passa a ser explícita a partir de agora — nenhum passo futuro deve introduzir a segunda sem que fique registada como decisão consciente, nunca por omissão.

---

## Método

Diferente das auditorias de falhas silenciosas (P633, P650, P655), que procuravam padrões de código. Esta auditoria procura, para cada extensão ou funcionalidade nova introduzida ao longo desta conversa, uma pergunta simples: **foi confirmado, com o binário vanilla de referência, que a sintaxe/semântica usada existe lá?** Se não foi confirmado, ou se foi confirmado que não existe e ficou mesmo assim, é candidato a revisão.

### Parte 1 — Listar todas as extensões e funcionalidades novas já introduzidas

Percorrer os relatórios de diagnóstico já escritos, procurando por marcações como "extensão", "capacidade nova", "além do vanilla", "não é paridade":

```bash
grep -rl "extensão\|capacidade nova\|além do vanilla\|não é paridade\|melhoria além" 00_nucleo/diagnosticos/*.md
```

Para cada ficheiro encontrado, extrair o que foi introduzido e se a confirmação contra o vanilla foi feita antes ou depois de implementar.

### Parte 2 — Casos já conhecidos, a re-confirmar com esta lente específica

| Item | Passo | Confirmado contra vanilla antes de implementar? |
|---|---|---|
| `table.numbering`/`caption` | P459 (original), P639 (confirmação) | A confirmar — P639 testou depois, não antes |
| `variant: (eixo: valor)` | P660 | Não — confirmado depois, e a falhar (P662 em curso) |
| `--document-id` (CLI) | P617 | Sim — P617 confirmou explicitamente que não há equivalente, decidiu por bandeira de CLI (não entra no documento, categoria diferente) |
| `DocumentID` estável por conteúdo | P612, revertido por P615 | Já revertido, caso fechado |

Continuar a lista com qualquer outra funcionalidade introduzida ao longo da conversa que não conste ainda desta tabela — rever sistematicamente desde P459 (onde `table.numbering` apareceu pela primeira vez) até ao passo mais recente.

### Parte 3 — Para cada item não confirmado, testar agora

Para cada linha da tabela onde a confirmação não tiver sido feita antes de implementar, testar directamente contra o vanilla agora, tal como P660 fez para `variant`.

### Critério de fecho da sonda

- [ ] Todos os relatórios de diagnóstico revistos, extensões/capacidades novas listadas.
- [ ] Tabela completa, com confirmação contra vanilla feita ou não, para cada item.
- [ ] Itens não confirmados, testados agora.
- [ ] Cada item classificado: diferença de implementação (aceitável, sem acção), diferença de linguagem confirmada e aceite conscientemente (como `table.numbering`, com razão escrita), ou diferença de linguagem por erro (como `variant`, candidata a reversão).

---

## Decisão

Para cada diferença de linguagem confirmada por erro (não decisão consciente): propor reversão, seguindo o mesmo tratamento de P662.

Para cada diferença de linguagem já confirmada como decisão consciente (como `table.numbering`): manter, mas confirmar que a documentação já deixa isso claro para quem vier a usar o cristalino, avisando que não é portável para o Typst real.

---

## Critério de fecho do passo

- [ ] Auditoria completa de P459 até ao passo mais recente.
- [ ] Tabela final com todos os itens classificados.
- [ ] Reversões propostas para os casos de erro, cada uma como o seu próprio passo pequeno, seguindo o padrão de P662.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p663.md`, com a lista completa.

---

## Nota

Esta distinção — implementação vs. linguagem — devia ter sido explícita desde o início desta conversa. Não esteve. Este passo é, em parte, uma correcção de um vazio na disciplina de verificação já estabelecida (regra de decisão nova, disciplina de verificação, proveniência de medição, paridade de defeitos, checklist de sub-layouts), que nunca distinguiu estas duas categorias com este grau de precisão. Vale a pena considerar escrever esta distinção como uma regra formal nova, ao lado das já existentes.

# Passo 97 — Auditoria de visibilidade `pub(super)` (DEBT-47)

**Série**: 97 (único passo de construção; sub-passos apenas de
verificação).
**Precondição**: Passo 96.10 encerrado, DEBT-46 fechado, 764 L1
+ 174 L3 testes a passar, zero violations no `crystalline-lint`.
**ADRs aplicáveis**: ADR-0036 (atomização), ADR-0037 (coesão
por domínio, com nota de visibilidade da Regra 3 introduzida no
Passo 96.6).

---

## Objectivo

Pagar DEBT-47. Auditar todos os usos de `pub(super)` introduzidos
na série 96.1–96.9 e aplicar a escada de visibilidade da nota da
Regra 3 da ADR-0037:

```
privado
  → método `pub(super)`
  → `pub(in path)`
  → campo `pub(super)` (justificado)
  → `pub(crate)`
  → `pub`
```

Cada item `pub(super)` existente deve ficar num dos seguintes
estados no fim do passo:

1. **Reduzido** para visibilidade mais restrita (`pub(in path)`
   ou privado atrás de método).
2. **Convertido** de campo para método (quando o acesso externo
   quebra invariantes do struct que o contém).
3. **Mantido** com justificação registada num comentário
   `// Regra 3 (ADR-0037): <razão>` na declaração.

Não é permitido fechar o passo com um `pub(super)` sem uma das
três marcas acima.

---

## Escopo

Auditoria sobre os ficheiros tocados nos Passos 96.1–96.9, por
ordem cronológica inversa (piores métricas primeiro, conforme
rácio método/campo do relatório de continuidade):

| Passo | Módulo | Rácio m:c | Prioridade |
|-------|--------|-----------|-----------|
| 96.1  | `rules/eval/`        | desconhecido | alta |
| 96.2  | `rules/eval/` (armos)| desconhecido | alta |
| 96.4  | `rules/parse/`       | desconhecido | alta |
| 96.5  | `rules/stdlib/`      | desconhecido | média |
| 96.7  | `rules/layout/`      | 2.1:1        | média |
| 96.8  | `rules/math/layout/` | 2.6:1        | baixa |
| 96.9  | `rules/lexer/`       | 4.0:1        | baixa |

"Desconhecido" significa que a métrica não foi registada na
altura (bulk replace de `pub(super)` aplicado antes da nota da
Regra 3 entrar em vigor no Passo 96.6). Medir no sub-passo 97.A.

Passos 96, 96.3, 96.6 e 96.10 são de governança e não
introduziram visibilidades; estão fora do escopo.

---

## Sub-passos de verificação

### 97.A — Inventário empírico

1. Enumerar todos os `pub(super)` em `01_core/src/rules/`.
   Formato por linha: `<ficheiro>:<linha>:<tipo>` onde `<tipo>`
   é `fn`, `struct_field`, `enum_variant`, `const`, `type`, ou
   `mod`.
2. Calcular o rácio métodos/campos por módulo (Passos 96.1,
   96.2, 96.4, 96.5 ainda sem medição).
3. Escrever inventário em
   `00_nucleo/diagnosticos/inventario-pub-super-passo-97.md`.

Critério de saída: total de `pub(super)` conhecido e
categorizado por módulo.

### 97.B — Classificação por ficheiro

Para cada item do inventário, classificar em uma das quatro
categorias:

- **R1 (reduzir)**: acesso vem de caminho conhecido e estreito;
  aplicar `pub(in path)`.
- **R2 (método)**: item é campo, acedido apenas para leitura ou
  mutação controlada; converter em método.
- **R3 (privado)**: item não é acedido fora do módulo; remover
  `pub(super)`.
- **R4 (manter)**: é campo genuinamente estrutural (dados
  passivos) ou o custo de métodos excede o ganho de invariante;
  registar justificação.

Critério de saída: cada item do inventário tem uma letra
`R1–R4` anotada.

### 97.C — Execução por módulo

Aplicar as mudanças por ordem inversa de rácio (pior primeiro:
96.1, 96.2, 96.4, 96.5). Um commit por módulo.

Regras de execução:

- Depois de cada ficheiro modificado: `cargo test -p typst-core`
  do sub-módulo tocado. Não continuar com testes a falhar.
- Depois de cada módulo completo: `cargo test --workspace` e
  `crystalline-lint`. Zero regressões em ambos.
- Para cada **R2** (conversão campo → método), verificar que
  não há `&mut` escondido no método que reintroduza o acoplamento
  removido pela ADR-0036.
- Para cada **R4** (manter), o comentário justificativo vai
  imediatamente antes da declaração, não no fim da linha.

Critério de saída por módulo: zero `pub(super)` sem uma das
marcas `R1–R4` aplicada.

### 97.D — Verificação final

1. Grep final por `pub(super)` em `01_core/src/rules/`. Confirmar
   que cada ocorrência tem comentário `// Regra 3 (ADR-0037): ...`
   na linha imediatamente acima ou é `fn` (métodos não precisam
   de comentário; o próprio método **é** a aplicação da nota).
2. Contagem final de testes: deve ser igual ou superior à
   linha de base (764 L1 + 174 L3). Testes V2 de smoke podem
   aumentar o total; nunca diminuir.
3. `crystalline-lint` com zero violations.
4. Rácio método:campo final por módulo, registado no
   relatório de encerramento.

### 97.E — Encerramento

Escrever `typst-passo-97-relatorio.md` com:

- Número de itens por categoria (R1/R2/R3/R4) por módulo.
- Rácios finais método:campo por módulo.
- Lista de `pub(super)` mantidos (R4) com justificação resumida.
- Decisão sobre DEBT-47: **ENCERRADO** se todos os itens
  classificados e executados; **PARCIALMENTE RESOLVIDO** se
  algum módulo ficou por auditar (com razão documentada).

---

## Critério de conclusão do passo

Todas as condições em conjunto:

1. Inventário 97.A completo e escrito em ficheiro.
2. Cada item do inventário com classificação R1–R4.
3. Cada módulo 96.1–96.9 processado (ou adiado com justificação
   escrita).
4. `cargo test --workspace` a passar.
5. `crystalline-lint` com zero violations.
6. Nenhum `pub(super)` não-função sem comentário de justificação.
7. Relatório 97.E escrito.

---

## Notas operacionais

- Não introduzir reestruturação de ficheiros neste passo. É
  auditoria de visibilidade, não de organização. Se surgir
  tentação de mover código, registar como novo DEBT e deixar
  para passo futuro.
- Se a auditoria revelar que um método `pub(super)` viola Regra 1
  da ADR-0036 (dependências implícitas), registar como DEBT
  separado e **não** corrigir neste passo.
- `pub(super)` em testes (`#[cfg(test)]`) fica fora do escopo.
- Os campos públicos dentro de structs marcados
  `#[non_exhaustive]` ou com builder pattern explícito contam
  como R4 automaticamente; basta citar o padrão na justificação.

---

## O que pode sair errado

- **Inventário muito maior que o esperado**. Se 97.A revelar
  mais de ~200 `pub(super)`, parar e abrir sub-decisão sobre
  faseamento. Não tentar processar tudo num único commit.
- **R2 a reintroduzir acoplamento**. Se converter campo em
  método exigir `&mut self` e esse `&mut` for transitivo para
  outros campos, reverter para R4 com justificação.
- **Circularidade com ADR-0036**. Se algum método novo pedir
  mais parâmetros na assinatura do que o campo original expunha,
  o método piora a coesão. Nesse caso, manter campo e marcar R4.

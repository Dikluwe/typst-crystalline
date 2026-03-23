# Prompt: [Nome do Componente]

**Camada**: L[n] — [Núcleo | Casca | Infra | Fiação]
**Criado em**: YYYY-MM-DD
**Arquivos gerados**: [lista dos arquivos que este prompt originou]

---

## Contexto

O que o sistema já tem que é relevante para este prompt.
Qual problema esta geração resolve dentro do domínio.
Quais componentes existentes este novo componente vai interagir.

---

## Restrições Estruturais

- Camada alvo e o que isso implica (ex: se L₁, zero I/O)
- Interfaces que deve implementar (referenciar arquivos existentes)
- O que este componente **não deve** fazer
- Dependências proibidas para esta camada

---

## Instrução

O prompt em si — preciso o suficiente para que uma nova execução
produza resultado estruturalmente equivalente.

---

## Critérios de Verificação

O que deve ser gerado junto com o código para confirmar que a instrução
foi satisfeita. O agente gera código e testes simultaneamente a partir
deste campo.

```
Dado [pré-condição precisa]
Quando [ação determinística]
Então [estado resultante esperado]
```

Incluir casos de borda relevantes e comportamentos de erro esperados.

---

## Resultado Esperado

- Quais arquivos foram criados (implementação e testes)
- O que cada arquivo expõe (funções, classes, interfaces)
- Como verificar que as restrições estruturais foram respeitadas

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| YYYY-MM-DD | Criação inicial | [arquivo] |

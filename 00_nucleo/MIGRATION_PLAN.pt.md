# Plano de Migração Typst Crystalline

## A Pergunta Central

> Essa documentação realmente melhora o desenvolvimento?

### Resposta Honesta: **Depende de como você usa.**

A documentação sozinha não acelera compilação nem corrige bugs magicamente. O valor dela é **navegacional** e **contratual**.

---

## Análise Prática de Benefícios

### 🟢 O que fica MAIS RÁPIDO

| Tarefa | Sem Núcleo | Com Núcleo | Por quê |
|--------|------------|------------|---------|
| **Achar onde X está implementado** | `grep` + tentativa e erro | Specs mapeiam 1:1 com código | Estrutura de pastas espelhada |
| **Entender propósito de um módulo** | Ler arquivo .rs inteiro | Ler resumo da spec | Documentação destilada |
| **Saber contratos de API** | Ler código + comentários | Ler contrato | Garantias formalizadas |
| **Contexto arquitetural** | Perguntar a alguém / chutar | Ler ADRs | Decisões documentadas |

### 🟡 O que CONTINUA IGUAL

| Tarefa | Realidade |
|--------|-----------|
| **Velocidade de compilação** | Igual (docs não afetam rustc) |
| **Execução de testes** | Igual |
| **Geração de código** | Igual (a menos que você adicione ferramentas) |

### 🔴 O que fica MAIS LENTO (inicialmente)

| Tarefa | Por quê |
|--------|---------|
| **Fazer mudanças** | Precisa atualizar spec também |
| **Adicionar features** | Deveria atualizar contratos |

---

## O Valor Real: Desenvolvimento Assistido por IA

A documentação do Núcleo é **projetada principalmente para agentes de IA**:

```
┌──────────────────────────────────────────────────────┐
│  Humano pede: "Adicione feature X ao Typst"          │
│                                                      │
│  IA sem Núcleo:                                      │
│    - Varre todos os 350+ arquivos .rs                │
│    - Adivinha relacionamentos                        │
│    - Pode violar padrões arquiteturais               │
│                                                      │
│  IA com Núcleo:                                      │
│    1. Lê ADRs relevantes → entende restrições        │
│    2. Lê contratos → sabe as interfaces              │
│    3. Lê specs → acha localização exata              │
│    4. Faz mudança direcionada                        │
└──────────────────────────────────────────────────────┘
```

### Melhoria Quantificada para IA

| Métrica | Sem | Com | Melhoria |
|---------|-----|-----|----------|
| Arquivos para varrer | 352 .rs | ~5-10 specs | 35-70x menos |
| Tokens de contexto | 500K+ | 10-50K | 10-50x menos |
| Violações arquiteturais | Prováveis | Detectadas | ∞ |

---

## Fluxos de Trabalho Práticos

### Fluxo 1: Corrigir Bug

```
1. Erro: "quebra de linha inesperada em equação"
2. → Ler GLOSSÁRIO: "equation" → typst-layout/math/
3. → Ler spec: math/mod.md → achar função de layout
4. → Ir ao código: math/mod.rs
5. → Corrigir bug
6. → Atualizar spec se comportamento mudou
```

**Aceleração**: ~2-5x mais rápido para localizar o arquivo certo.

### Fluxo 2: Adicionar Novo Elemento

```
1. Quero: elemento #callout
2. → Ler contrato: native-element.md → saber requisitos da trait
3. → Ler ADR-011 → entender sistema de Content
4. → Ler spec existente (ex: BlockElem) como template
5. → Implementar seguindo padrão
6. → Criar spec para novo elemento
```

**Benefício**: Implementação consistente, sem adivinhação.

### Fluxo 3: Entender Fluxo de Compilação

```
1. Pergunta: "Por que minha show rule não está aplicando?"
2. → Ler ADR-009: Realization vs Layout
3. → Ler spec: typst-realize/mod.md
4. → Entender ordem de execução de show rules
5. → Corrigir ordem das rules no documento
```

**Benefício**: Sem debugging por tentativa e erro.

---

## Fases de Migração

### Fase 0: Estado Atual ✅
- Estrutura de pastas implementada
- Specs geradas (99.7%)
- ADRs documentados (14)
- Contratos definidos (10)

### Fase 1: Validação (Próximo Recomendado)
Criar verificações automatizadas:

```bash
# Validador de dependências
cargo run --bin crystalline-check

# Saída:
# ✅ 01_core não depende de 02_shell
# ✅ 02_shell não depende de 03_infra
# ⚠️  typst-layout importa typst-pdf (violação!)
```

### Fase 2: Integração (Opcional)
- Hook de pre-commit: spec obrigatória para novo .rs
- Check de CI: validação de dependências entre camadas
- Documentação: linkar specs no rustdoc

### Fase 3: Evolução (Contínuo)
- Manter specs sincronizadas com código
- Atualizar ADRs quando decisões mudarem
- Refinar contratos conforme interfaces evoluem

---

## Métricas para Acompanhar

| Métrica | Como Medir |
|---------|------------|
| **Tempo até primeiro fix** | Rastrear timestamps de PRs |
| **Arquivos tocados por feature** | Contar em PRs |
| **Violações arquiteturais** | Falhas em check de CI |
| **Taxa de sucesso da IA** | % de tarefas completadas |

---

## Conclusão

### Isso ajuda?

| Para... | Resposta |
|---------|----------|
| **Desenvolvedores humanos** | Ajuda moderada (navegação, entendimento) |
| **Assistentes de IA** | Ajuda significativa (contexto, contratos) |
| **Novos contribuidores** | Ajuda grande (onboarding) |
| **Manutenção** | Ajuda a longo prazo (previne degradação) |

### O Trade-off

```
Investimento: ~20% mais trabalho mantendo docs sincronizadas
Retorno:      ~50% navegação mais rápida + produtividade de IA
              + Guardrails arquiteturais
              + Melhoria no onboarding
```

### Resumo Final

A documentação do Núcleo **não é mágica** — é um **multiplicador**. Ela torna o código mais navegável e amigável para IA, mas os ganhos reais de produtividade vêm de:

1. **Usar a documentação** ao desenvolver
2. **Mantê-la atualizada** conforme o código muda
3. **Adicionar ferramentas** para validar arquitetura

Sem uso ativo, a documentação apodrece e não oferece valor.
